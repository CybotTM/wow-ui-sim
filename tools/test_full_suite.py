"""Bounded subprocess coverage of the detached full-suite worker."""

import fcntl
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest


RUNNER = Path(__file__).with_name("full_suite.py")
SCOPES = ("integration", "prefork_full_ui", "lib")
PROFILES = ("retail", "ptr", "wrath", "mists", "wowforever", "era", "anniversary")


def write_executable(path, body):
    path.write_text(f"#!{sys.executable}\n" + body)
    path.chmod(0o755)


def wait_for(path, process):
    deadline = time.monotonic() + 5
    while not path.exists():
        if process.poll() is not None:
            raise AssertionError(
                f"Worker exited before {path.name}: {process.communicate()}"
            )
        if time.monotonic() >= deadline:
            raise AssertionError(f"Timed out waiting for {path.name}")
        time.sleep(0.01)


class CertifiedSuiteTests(unittest.TestCase):
    @staticmethod
    def stop_worker(process):
        if process.poll() is None:
            os.killpg(process.pid, signal.SIGKILL)
        process.communicate(timeout=5)

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.home = Path(self.temp.name)
        self.bin = self.home / 'bin'
        self.bin.mkdir()
        self.repo = self.home / 'Projects/wow/wow-ui-sim'
        self.repo.mkdir(parents=True)
        self.slot = self.home / '.cache/wow-ui-sim-targets/slot-2'
        self.slot.mkdir(parents=True)
        capacity = self.home / '.worktrees/logs/capacity.sh'
        capacity.parent.mkdir(parents=True)
        write_executable(capacity, "print('agents | ADD: available')\n")
        cvars = self.home / '.local/share/wow-sim/cvars.json'
        cvars.parent.mkdir(parents=True)
        cvars.write_text('{}')
        self.env = dict(os.environ, HOME=str(self.home),
                        PATH=f"{self.bin}{os.pathsep}{os.environ['PATH']}",
                        GIT_CONFIG_NOSYSTEM='1')
        def git(*args):
            return subprocess.run(['git', *args], cwd=self.repo, env=self.env,
                                  check=True, capture_output=True, text=True).stdout.strip()
        git('init', '-q')
        git('config', 'user.name', 'Test')
        git('config', 'user.email', 'test@example.invalid')
        (self.repo / 'source').write_text('frozen source\n')
        git('add', 'source')
        git('commit', '-qm', 'fixture')
        self.sha = git('rev-parse', 'HEAD')
        git('remote', 'add', 'origin', str(self.repo))
        write_executable(self.bin / 'cargo',
                         "import sys\nsys.exit('unexpected cargo invocation')\n")
        write_executable(self.bin / 'systemctl', "print('LoadState=not-found')\n")
        write_executable(self.bin / 'slot-build.py',
                         "import json, os, pathlib, sys\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "(root / 'slot-argv.json').write_text(json.dumps(sys.argv[1:]))\n"
                         "print('guard rejected: owner mismatch', file=sys.stderr)\n"
                         "sys.exit(1)\n")

    def test_failed_launcher_with_unknown_unit_retains_owner(self):
        write_executable(self.bin / 'systemctl', "import sys\nsys.exit(1)\n")
        process = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                  '--profile', 'ptr', '--target-dir', str(self.slot)],
                                 env=self.env, capture_output=True, text=True, timeout=10)
        self.assertNotEqual(process.returncode, 0)
        self.assertTrue((self.slot / '.owner').exists())
        argv = json.loads((self.home / 'slot-argv.json').read_text())
        unit = argv[argv.index('--unit') + 1]
        record = json.loads((self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json').read_text())
        self.assertEqual(record['status'], 'FAILED')
        self.assertIn(f'slot 2 retained: unit {unit}', record['error'])

    def test_failed_test_launcher_with_unknown_unit_retains_owner(self):
        write_executable(self.bin / 'slot-build.py',
                         "import json, os, pathlib, sys\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "unit = sys.argv[sys.argv.index('--unit') + 1]\n"
                         "p = root / '.cache/wow-ui-sim-targets/slot-2/receipts' / (unit + '.json')\n"
                         "p.parent.mkdir(exist_ok=True)\n"
                         "p.write_text(json.dumps({'result': 'success', 'returncode': 0, 'reported_artifacts': [{'executable': '/unused', 'profile': {'test': True}, 'target': {'name': 'integration', 'kind': ['test']}}]}))\n")
        write_executable(self.bin / 'systemd-run', "import sys\nsys.exit(1)\n")
        write_executable(self.bin / 'systemctl',
                         "import sys\n"
                         "sys.exit(1) if 'full-suite-test-' in ' '.join(sys.argv) else print('ActiveState=inactive\\nResult=success')\n")
        process = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                  '--profile', 'ptr', '--target-dir', str(self.slot)],
                                 env=self.env, capture_output=True, text=True, timeout=10)
        self.assertNotEqual(process.returncode, 0)
        self.assertTrue((self.slot / '.owner').exists())
        record = json.loads((self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json').read_text())
        self.assertEqual(record['status'], 'FAILED')
        self.assertIn('slot 2 retained: unit full-suite-test-', record['error'])

    def test_failed_launcher_stops_active_unit_before_releasing_owner(self):
        write_executable(self.bin / 'systemctl',
                         "import os, pathlib, sys\n"
                         "p = pathlib.Path(os.environ['HOME']) / 'stopped'\n"
                         "if 'stop' in sys.argv: p.touch()\n"
                         "elif p.exists(): print('ActiveState=inactive')\n"
                         "else: print('ActiveState=active')\n")
        process = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                  '--profile', 'ptr', '--target-dir', str(self.slot)],
                                 env=self.env, capture_output=True, text=True, timeout=10)
        self.assertEqual(process.returncode, 0, process.stderr)
        self.assertTrue((self.home / 'stopped').exists())
        self.assertFalse((self.slot / '.owner').exists())

    def test_guard_failure_receipt_contains_all_phase_records(self):
        write_executable(self.bin / 'systemctl', "print('LoadState=not-found')\n")
        subprocess.run([sys.executable, str(RUNNER), 'run', self.sha, '--profile', 'ptr',
                        '--target-dir', str(self.slot)], env=self.env, capture_output=True, text=True, timeout=10)
        record = json.loads((self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json').read_text())
        self.assertEqual(record['steps']['build']['status'], 'FAILED')
        self.assertEqual(record['steps']['integration']['status'], 'not_run')
        self.assertEqual(record['steps']['prefork']['status'], 'not_applicable')
        self.assertEqual(record['steps']['lib']['status'], 'not_run')
        self.assertEqual(record['failures'], {'integration': [], 'lib': []})

    def test_failed_master_run_preserves_baseline_and_linked_passed_result(self):
        write_executable(self.bin / 'systemctl', "print('LoadState=not-found')\n")
        results = self.home / 'Projects/wow/full-suite-results'
        results.mkdir(parents=True)
        prior = results / ('a' * 40 + '-ptr.json')
        content = json.dumps({'sha': 'a' * 40, 'profile': 'ptr', 'status': 'PASSED', 'failures': {'integration': ['old']}})
        prior.write_text(content)
        (results / 'master-latest-ptr.json').symlink_to(prior.name)
        subprocess.run([sys.executable, str(RUNNER), 'run', 'master', '--profile', 'ptr',
                        '--target-dir', str(self.slot)], env=self.env, capture_output=True, text=True, timeout=10)
        self.assertEqual(prior.read_text(), content)
        self.assertEqual((results / 'master-latest-ptr.json').read_text(), content)
        record = json.loads((results / f'{self.sha}-ptr.json').read_text())
        self.assertEqual(record['status'], 'FAILED')

    def test_failed_same_sha_rerun_preserves_pass_and_records_attempt(self):
        results = self.home / 'Projects/wow/full-suite-results'
        results.mkdir(parents=True)
        prior = results / f'{self.sha}-ptr.json'
        content = json.dumps({'sha': self.sha, 'profile': 'ptr', 'status': 'PASSED', 'failures': {}})
        prior.write_text(content)
        previous_log = results / f'{self.sha}-ptr.log'
        previous_log.write_text('passed log\n')
        process = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha, '--profile', 'ptr',
                                  '--target-dir', str(self.slot)], env=self.env, capture_output=True, text=True, timeout=10)
        self.assertEqual(process.returncode, 0, process.stderr)
        self.assertEqual(prior.read_text(), content)
        self.assertEqual(previous_log.read_text(), 'passed log\n')
        attempts = list(results.glob(f'{self.sha}-ptr-attempt-*.json'))
        self.assertEqual(len(attempts), 1)
        self.assertEqual(json.loads(attempts[0].read_text())['status'], 'FAILED')

    def test_build_log_io_failure_checks_unit_before_releasing_owner(self):
        write_executable(self.bin / 'slot-build.py', "print('launched')\n")
        write_executable(self.bin / 'systemctl',
                         "import os, pathlib, sys\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "if os.environ.get('UNKNOWN_UNIT'): sys.exit(1)\n"
                         "if 'stop' in sys.argv:\n"
                         "    (root / 'owner-at-stop').write_text(str((root / '.cache/wow-ui-sim-targets/slot-2/.owner').exists()))\n"
                         "    (root / 'stopped').touch()\n"
                         "elif (root / 'stopped').exists(): print('ActiveState=inactive')\n"
                         "else: print('ActiveState=active')\n")
        script = self.home / 'inject.py'
        script.write_text('''import builtins, importlib.util, sys
spec = importlib.util.spec_from_file_location("runner", sys.argv[1])
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
original = builtins.open
class Log:
    def __init__(self, stream): self.stream, self.failed = stream, False
    def __enter__(self): return self
    def __exit__(self, *args): return self.stream.__exit__(*args)
    def write(self, value):
        if sys.argv[4] == "write" and value.startswith("===== build:") and not self.failed:
            self.failed = True
            raise OSError("injected write failure")
        return self.stream.write(value)
    def flush(self):
        if sys.argv[4] == "flush" and not self.failed:
            self.failed = True
            raise OSError("injected flush failure")
        return self.stream.flush()
def injected(path, *args, **kwargs):
    stream = original(path, *args, **kwargs)
    return Log(stream) if str(path).endswith(".log") else stream
builtins.open = injected
sys.exit(runner.run(sys.argv[2], 1, sys.argv[3], "ptr"))
''')
        for failure in ('write', 'flush'):
            with self.subTest(failure=failure):
                for name in ('stopped', 'owner-at-stop'):
                    (self.home / name).unlink(missing_ok=True)
                process = subprocess.run([sys.executable, str(script), str(RUNNER), self.sha, str(self.slot), failure],
                                         env=self.env, capture_output=True, text=True, timeout=10)
                self.assertEqual(process.returncode, 0, process.stderr)
                self.assertTrue((self.home / 'owner-at-stop').exists(), 'unit was not stopped before release')
                self.assertEqual((self.home / 'owner-at-stop').read_text(), 'True')
                self.assertFalse((self.slot / '.owner').exists())
        process = subprocess.run([sys.executable, str(script), str(RUNNER), self.sha, str(self.slot), 'write'],
                                 env=dict(self.env, UNKNOWN_UNIT='1'), capture_output=True, text=True, timeout=10)
        self.assertNotEqual(process.returncode, 0)
        self.assertTrue((self.slot / '.owner').exists())
        record = json.loads((self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json').read_text())
        self.assertIn('state unknown', record['error'])

    def test_guard_rejection_records_failure_without_running_tests(self):
        result = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                 '--profile', 'ptr', '--target-dir', str(self.slot)],
                                env=self.env, capture_output=True, text=True, timeout=10)
        path = self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json'
        self.assertTrue(path.exists(), result.stderr)
        receipt = json.loads(path.read_text())
        self.assertIn('FAILED', json.dumps(receipt).upper())
        self.assertIn('guard', json.dumps(receipt).lower())
        self.assertFalse(any(step.get('exit') == 0 for step in receipt['steps'].values()))
        argv = json.loads((self.home / 'slot-argv.json').read_text())
        self.assertIn('--owner', argv)
        self.assertEqual(argv[argv.index('--slot') + 1], '2')
        self.assertIn('sound,gui,casc,client-ptr', argv)
        self.assertEqual(argv[argv.index('--') + 1:argv.index('--') + 8],
                         ['cargo', 'test', '--no-run', '--all-targets', '--locked',
                          '--offline', '--message-format=json'])
        worktree = Path(argv[argv.index('--worktree') + 1])
        self.assertEqual(subprocess.check_output(['git', '-C', str(worktree), 'rev-parse', 'HEAD'],
                                                 env=self.env, text=True).strip(), self.sha)
        self.assertEqual((worktree / 'source').read_text(), 'frozen source\n')
        self.assertEqual(subprocess.check_output(['git', '-C', str(worktree), 'status', '--porcelain'],
                                                 env=self.env, text=True), '')
        self.assertFalse((self.slot / '.owner').exists())
        released = self.home / f'.local/state/wow-ui-sim/handoff/full-suite/{self.sha[:12]}-ptr/slot-2.owner.released'
        self.assertEqual(released.read_text(), f'full-suite {self.sha[:12]} ptr')
        again = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                '--profile', 'ptr', '--target-dir', str(self.slot)],
                               env=self.env, capture_output=True, text=True, timeout=10)
        self.assertEqual(again.returncode, 0, again.stderr)
        second = json.loads((self.home / 'slot-argv.json').read_text())
        self.assertEqual(second[second.index('--worktree') + 1], str(worktree))

    def test_result_waits_for_certified_build_and_test_execution(self):
        test_binary = self.slot / 'integration-test'
        write_executable(test_binary,
                         "import os, pathlib, time\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "(root / 'test-started').touch()\n"
                         "while not (root / 'test-release').exists(): time.sleep(.01)\n")
        write_executable(self.bin / 'slot-build.py',
                         "import json, os, pathlib, sys, time\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "argv = sys.argv[1:]\n"
                         "(root / 'slot-argv.json').write_text(json.dumps(argv))\n"
                         "(root / 'build-started').touch()\n"
                         "while not (root / 'build-release').exists(): time.sleep(.01)\n"
                         "slot = root / '.cache/wow-ui-sim-targets/slot-2'\n"
                         "unit = argv[argv.index('--unit') + 1]\n"
                         "receipt = slot / 'receipts' / (unit + '.json')\n"
                         "receipt.parent.mkdir(exist_ok=True)\n"
                         "records = [{'executable': str(slot / 'integration-test'), "
                         "'target': {'name': n, 'kind': [k]}, 'profile': {'test': True}} "
                         "for n, k in [('integration', 'test'), ('wow_ui_sim', 'lib')]]\n"
                         "receipt.write_text(json.dumps({'result': 'success', 'returncode': 0, "
                         "'reported_artifacts': records}))\n")
        write_executable(self.bin / 'systemctl',
                         "print('ActiveState=inactive\\nSubState=exited\\nResult=success')\n")
        write_executable(self.bin / 'systemd-run',
                         "import json, os, pathlib, subprocess, sys\n"
                         "(pathlib.Path(os.environ['HOME']) / 'test-unit.json').write_text(json.dumps(sys.argv[1:]))\n"
                         "sys.exit(subprocess.call(sys.argv[sys.argv.index('--') + 1:]))\n")
        process = subprocess.Popen([sys.executable, str(RUNNER), 'run', self.sha,
                                    '--profile', 'ptr', '--target-dir', str(self.slot)],
                                   env=self.env, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, text=True, start_new_session=True)
        self.addCleanup(self.stop_worker, process)
        receipt = self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json'
        wait_for(self.home / 'build-started', process)
        self.assertFalse(receipt.exists())
        (self.home / 'build-release').touch()
        wait_for(self.home / 'test-started', process)
        self.assertFalse(receipt.exists())
        (self.home / 'test-release').touch()
        stdout, stderr = process.communicate(timeout=10)
        self.assertEqual(process.returncode, 0, stderr)
        self.assertTrue(receipt.exists())
        self.assertEqual(json.loads(receipt.read_text())['status'], 'PASSED')
        log = (self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.log').read_text()
        self.assertIn('cvar-before', log)
        self.assertIn('cvar-after', log)
        self.assertEqual((self.home / '.local/share/wow-sim/cvars.json').read_text(), '{}')
        unit = json.loads((self.home / 'test-unit.json').read_text())
        self.assertIn('MemoryMax=9G', unit)
        self.assertIn('MemorySwapMax=0', unit)
        argv = json.loads((self.home / 'slot-argv.json').read_text())
        worktree = Path(argv[argv.index('--worktree') + 1])
        mount = self.home / '.cache/wow-ui-sim-srcmount/slot-2'
        self.assertIn(f'BindPaths={worktree}:{mount}', unit)
        self.assertIn(f'--working-directory={mount}', unit)
        self.assertTrue(all(step.get('exit', 0) == 0
                            for step in json.loads(receipt.read_text())['steps'].values()))
        argv = json.loads((self.home / 'slot-argv.json').read_text())
        worktree = Path(argv[argv.index('--worktree') + 1])
        self.assertEqual((worktree / 'source').read_bytes(), b'frozen source\n')
        self.assertEqual(subprocess.check_output(['git', '-C', str(worktree), 'rev-parse', 'HEAD'],
                                                 env=self.env, text=True).strip(), self.sha)

    def test_test_failures_preserve_phase_records_and_continue(self):
        binary = self.slot / 'test-binary'
        write_executable(binary,
                         "import sys\nprint('test sample::fails ... FAILED')\nsys.exit(7)\n")
        write_executable(self.bin / 'slot-build.py',
                         "import json, os, pathlib, sys\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "slot = root / '.cache/wow-ui-sim-targets/slot-2'\n"
                         "unit = sys.argv[sys.argv.index('--unit') + 1]\n"
                         "receipt = slot / 'receipts' / (unit + '.json')\n"
                         "receipt.parent.mkdir(exist_ok=True)\n"
                         "records = [{'executable': str(slot / 'test-binary'), 'profile': {'test': True}, "
                         "'target': {'name': n, 'kind': [k]}} for n, k in "
                         "[('integration', 'test'), ('wow_ui_sim', 'lib')]]\n"
                         "receipt.write_text(json.dumps({'result': 'success', 'returncode': 0, 'reported_artifacts': records}))\n")
        write_executable(self.bin / 'systemctl', "print('ActiveState=inactive\\nSubState=exited\\nResult=success')\n")
        write_executable(self.bin / 'systemd-run',
                         "import subprocess, sys\nsys.exit(subprocess.call(sys.argv[sys.argv.index('--') + 1:]))\n")
        process = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                  '--profile', 'ptr', '--target-dir', str(self.slot)],
                                 env=self.env, capture_output=True, text=True, timeout=10)
        self.assertEqual(process.returncode, 0, process.stderr)
        record = json.loads((self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json').read_text())
        self.assertEqual(record['status'], 'FAILED')
        self.assertEqual(record['steps']['integration']['exit'], 7)
        self.assertEqual(record['steps']['integration']['status'], 'FAILED')
        self.assertEqual(record['steps']['lib']['exit'], 7)
        self.assertEqual(record['steps']['lib']['status'], 'FAILED')
        self.assertEqual(record['steps']['prefork']['status'], 'not_applicable')
        self.assertEqual(record['failures'], {'integration': ['sample::fails'], 'lib': ['sample::fails']})
        self.assertIsNone(record['new_failures'])
        results = self.home / 'Projects/wow/full-suite-results'
        canonical = results / f'{self.sha}-ptr.json'
        passed = json.dumps({'sha': self.sha, 'profile': 'ptr', 'status': 'PASSED', 'failures': {}})
        canonical.write_text(passed)
        rerun = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                '--profile', 'ptr', '--target-dir', str(self.slot)],
                               env=self.env, capture_output=True, text=True, timeout=10)
        self.assertEqual(rerun.returncode, 0, rerun.stderr)
        self.assertEqual(canonical.read_text(), passed)
        attempts = list(results.glob(f'{self.sha}-ptr-attempt-*.json'))
        self.assertEqual(len(attempts), 1)
        self.assertEqual(json.loads(attempts[0].read_text())['steps']['integration']['exit'], 7)

    def test_failed_certification_never_runs_test_binaries(self):
        write_executable(self.bin / 'slot-build.py',
                         "import json, os, pathlib, sys\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "unit = sys.argv[sys.argv.index('--unit') + 1]\n"
                         "receipt = root / '.cache/wow-ui-sim-targets/slot-2/receipts' / (unit + '.json')\n"
                         "receipt.parent.mkdir(exist_ok=True)\n"
                         "receipt.write_text(json.dumps({'result': 'stale-artifacts', 'returncode': 0, 'dep_info_errors': ['bad input']}))\n")
        write_executable(self.bin / 'systemctl',
                         "print('ActiveState=inactive\\nSubState=exited\\nResult=success')\n")
        write_executable(self.bin / 'systemctl',
                         "print('ActiveState=failed\\nSubState=failed\\nResult=exit-code')\n")
        result = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                 '--profile', 'ptr', '--target-dir', str(self.slot)],
                                env=self.env, capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        record = json.loads((self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json').read_text())
        self.assertEqual(record['status'], 'FAILED')
        self.assertIn('bad input', record['error'])
        self.assertEqual(record['steps']['build']['status'], 'FAILED')
        self.assertEqual(record['steps']['integration']['status'], 'not_run')
        self.assertFalse((self.slot / '.owner').exists())

    def test_unknown_build_unit_state_retains_slot_and_fails(self):
        write_executable(self.bin / 'slot-build.py',
                         "import json, os, pathlib, sys\n"
                         "(pathlib.Path(os.environ['HOME']) / 'slot-argv.json').write_text(json.dumps(sys.argv[1:]))\n")
        write_executable(self.bin / 'systemctl',
                         "import os, pathlib, sys\n"
                         "p = pathlib.Path(os.environ['HOME']) / 'systemctl-calls'\n"
                         "with p.open('a') as f: f.write('call\\n')\n"
                         "sys.exit(1)\n")
        process = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                  '--profile', 'ptr', '--target-dir', str(self.slot)],
                                 env=self.env, capture_output=True, text=True, timeout=10)
        self.assertNotEqual(process.returncode, 0)
        self.assertTrue((self.home / 'systemctl-calls').exists())
        self.assertEqual((self.slot / '.owner').read_text(), f'full-suite {self.sha[:12]} ptr')
        argv = json.loads((self.home / 'slot-argv.json').read_text())
        unit = argv[argv.index('--unit') + 1]
        result = json.loads((self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json').read_text())
        self.assertEqual(result['status'], 'FAILED')
        self.assertIn(f'slot 2 retained: unit {unit} state unknown', result['error'])

    def test_owned_slot_is_never_stolen(self):
        owner = self.slot / '.owner'
        owner.write_text('another owner')
        result = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                 '--target-dir', str(self.slot)], env=self.env,
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(owner.read_text(), 'another owner')
        self.assertFalse((self.home / 'slot-argv.json').exists())
        self.assertIn('owner', (result.stdout + result.stderr).lower())

    def test_capacity_refusal_does_not_claim_slot(self):
        write_executable(self.home / '.worktrees/logs/capacity.sh',
                         "print('agents | HOLD: busy')\n")
        result = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                 '--target-dir', str(self.slot)], env=self.env,
                                capture_output=True, text=True, timeout=10)
        self.assertFalse((self.slot / '.owner').exists())
        self.assertFalse((self.home / 'slot-argv.json').exists())
        self.assertIn('HOLD: busy', result.stdout + result.stderr)

    def test_suite_lock_blocks_the_worker_before_checkout(self):
        results = self.home / 'Projects/wow/full-suite-results'
        results.mkdir(parents=True)
        with (results / '.lock').open('w') as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            process = subprocess.Popen([sys.executable, str(RUNNER), 'run', self.sha,
                                        '--target-dir', str(self.slot)], env=self.env,
                                       stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                       text=True, start_new_session=True)
            self.addCleanup(self.stop_worker, process)
            time.sleep(.1)
            self.assertIsNone(process.poll())
            self.assertFalse((self.home / 'slot-argv.json').exists())
            self.assertFalse(list((self.home / '.worktrees').glob('wow-ui-sim-fullsuite-*')))
            fcntl.flock(lock, fcntl.LOCK_UN)
        process.communicate(timeout=10)

    def test_invalid_target_and_profile_are_rejected_before_launch(self):
        for options in (['--target-dir', str(self.slot.with_name('slot-4'))],
                        ['--target-dir', str(self.slot), '--profile', 'unknown'],
                        ['--target-dir', str(self.slot), '--build-jobs', '0']):
            with self.subTest(options=options):
                result = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha, *options],
                                        env=self.env, capture_output=True, text=True, timeout=10)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse((self.home / 'slot-argv.json').exists())

    def test_status_groups_profile_and_legacy_results(self):
        results = self.home / 'Projects/wow/full-suite-results'
        results.mkdir(parents=True)
        profile = {'sha': self.sha, 'profile': 'ptr', 'steps': {'lib': {'exit': 7}}}
        legacy = {'sha': self.sha, 'steps': {'lib': {'exit': 0}}}
        (results / f'{self.sha}-ptr.json').write_text(json.dumps(profile))
        (results / f'{self.sha}.json').write_text(json.dumps(legacy))
        process = subprocess.run([sys.executable, str(RUNNER), 'status', self.sha],
                                 env=self.env, capture_output=True, text=True, timeout=10)
        self.assertEqual(process.returncode, 0, process.stderr)
        output = json.loads(process.stdout)
        self.assertEqual(output['profiles']['ptr'], profile)
        self.assertIsNone(output['profiles']['retail'])
        self.assertEqual(output['legacy_retail'], legacy)

    def test_submit_sets_memory_limits(self):
        write_executable(self.bin / 'systemd-run',
                         "import json, os, pathlib, sys\n"
                         "(pathlib.Path(os.environ['HOME']) / 'unit.json').write_text(json.dumps(sys.argv[1:]))\n")
        result = subprocess.run([sys.executable, str(RUNNER), 'submit', self.sha,
                                 '--target-dir', str(self.slot)], env=self.env,
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        argv = json.loads((self.home / 'unit.json').read_text())
        self.assertIn('MemoryMax=9G', argv)
        self.assertIn('MemorySwapMax=0', argv)


if __name__ == "__main__":
    unittest.main()
