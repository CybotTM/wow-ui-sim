#!/usr/bin/env python3
"""Exercise the real harness generator across two roots sharing one Cargo target.

Run under the lane's admitted async build unit, with CARGO_TARGET_DIR set.
An optional build-script input permits reproducing the old generator first.
"""

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time


def admit_build():
    capacity = Path('/home/osso/.worktrees/logs/capacity.sh')
    while True:
        result = subprocess.run(
            [str(capacity)], capture_output=True, text=True, check=True,
            env={**os.environ, 'CAPACITY_ID': 'wrath-addon-load'},
        )
        print(result.stdout, flush=True)
        if '| ADD:' in result.stdout:
            return
        time.sleep(120)


def write_fixture(root, repo, build_script):
    helper = root / 'crates/test-harness'
    shutil.copytree(repo / 'crates/test-harness', helper)
    shutil.copy2(build_script, helper / 'build.rs')
    (root / 'Cargo.toml').write_text('''[package]
name = "wow-ui-sim-harness-relocation-fixture"
version = "0.1.0"
edition = "2024"
autotests = false
[workspace]
members = ["crates/test-harness"]
resolver = "3"
[features]
gui = []
[dev-dependencies]
wow-ui-sim-test-harness = { path = "crates/test-harness" }
[[test]]
name = "integration"
path = "tests/integration.rs"
''')
    tests = root / 'tests'
    (tests / 'common').mkdir(parents=True)
    (tests / 'c_item_api').mkdir()
    for path in ['common/mod.rs', 'common/token_ui_fixtures.rs', 'c_item_api/support.rs']:
        (tests / path).touch()
    (tests / 'integration.rs').write_text('wow_ui_sim_test_harness::integration_tests!();\n')
    (tests / 'relocation.rs').write_text('''#[test]
fn harness_uses_current_worktree() {
    assert_eq!(file!(), concat!(env!("CARGO_MANIFEST_DIR"), "/tests/relocation.rs"));
}
''')


def build_and_run(root, evidence):
    admit_build()
    command = ['cargo', 'test', '--manifest-path', str(root / 'Cargo.toml'),
               '--test', 'integration', '--no-run', '-j1', '--message-format=json']
    build = subprocess.run(command, cwd=root, capture_output=True, text=True)
    (evidence / (root.name + '-build.stdout')).write_text(build.stdout)
    (evidence / (root.name + '-build.stderr')).write_text(build.stderr)
    build.check_returncode()
    records = [json.loads(line) for line in build.stdout.splitlines() if line.startswith('{')]
    binary = next(record['executable'] for record in records
                  if record.get('reason') == 'compiler-artifact'
                  and record.get('target', {}).get('name') == 'integration'
                  and record.get('executable'))
    selected = 'relocation::harness_uses_current_worktree'
    test = subprocess.run([binary, selected, '--exact', '--nocapture'],
                          capture_output=True, text=True, timeout=90)
    (evidence / (root.name + '-runtime.stdout')).write_text(test.stdout)
    (evidence / (root.name + '-runtime.stderr')).write_text(test.stderr)
    print(test.stdout, test.stderr, flush=True)
    if '1 passed; 0 failed' not in test.stdout:
        raise AssertionError(f'{root.name}: expected one passing relocation test')
    test.check_returncode()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--evidence', type=Path, required=True)
    parser.add_argument('--build-script', type=Path)
    args = parser.parse_args()
    if 'CARGO_TARGET_DIR' not in os.environ:
        raise RuntimeError('CARGO_TARGET_DIR must name the admitted shared target slot')
    os.umask(0o077)
    repo = Path(__file__).resolve().parent.parent
    args.evidence.mkdir(parents=True, exist_ok=True)
    source = args.build_script or repo / 'crates/test-harness/build.rs'
    with tempfile.TemporaryDirectory(prefix='relocation-', dir=args.evidence) as directory:
        first = Path(directory) / 'first'
        second = Path(directory) / 'second'
        write_fixture(first, repo, source)
        build_and_run(first, args.evidence)
        # Preserve every source timestamp: relocation alone must invalidate generation.
        shutil.copytree(first, second, copy_function=shutil.copy2)
        build_and_run(second, args.evidence)


if __name__ == '__main__':
    main()
