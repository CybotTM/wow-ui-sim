from pathlib import Path
import datetime, hashlib, json
repo = Path('/home/osso/Projects/wow/wow-ui-sim')
root = Path('/home/osso/.local/state/wow-ui-sim/verification/talent-stage-diagnostic-current')
epoch = root / ('compile-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
epoch.mkdir()
def save(name, value):
    (epoch / name).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')
memory = {line.split(':')[0]: int(line.split(':')[1].split()[0]) for line in Path('/proc/meminfo').read_text().splitlines()}
load = float(Path('/proc/loadavg').read_text().split()[0])
active = []
for process in Path('/proc').iterdir():
    if not process.name.isdigit():
        continue
    try:
        if (process / 'comm').read_text().strip() == 'cargo' and (process / 'cwd').resolve() in [repo, repo.parent / 'full-suite-checkout']:
            active.append(process.name)
    except OSError:
        continue
admitted = memory['MemAvailable'] / 1048576 >= 6 and load < 24 and not active
save('admission.json', {'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'load1': load, 'available_gib': memory['MemAvailable'] / 1048576, 'relevant_cargo': active, 'admitted': admitted})
if not admitted:
    save('outcome.json', {'status': 'RESOURCE_BLOCKED', 'compiled': False, 'executed_cases': 0})
    raise RuntimeError('Talent stage diagnostic compilation admission denied')
source = root / 'probe.rs'
payload = source.read_bytes()
frozen_source = epoch / 'probe.rs'
frozen_source.write_bytes(payload)
records = [json.loads(line) for line in Path('/home/osso/.local/state/wow-ui-sim/verification/reversed-range-interpolation-current/20261011T222722Z/compile.stdout').read_text().splitlines()]
selected = [record for record in records if record.get('reason') == 'compiler-artifact' and record.get('target', {}).get('name') == 'wow_ui_sim' and 'lib' in record.get('target', {}).get('kind', [])]
assert len(selected) == 1
record = selected[0]
assert 'client-retail' in record['features']
libraries = [Path(path) for path in record['filenames'] if path.endswith('.rlib')]
assert len(libraries) == 1 and libraries[0].exists()
library = libraries[0]
save('selected-library.json', {'cargo_record': record, 'bytes': library.stat().st_size, 'mtime_ns': library.stat().st_mtime_ns, 'scope': 'Saved Cargo projection and current path/stat only; prior library bytes not rehashed, no hermetic source/input identity claim'})
revision = cli.git('-C', str(repo), 'rev-parse', 'HEAD').capture().run()
assert revision.exit_code == 0
save('revision.json', {'head': revision.stdout.strip(), 'library_cargo_epoch_revision': '6a4801a406c80a53bd95091f60f31e6590b90273', 'scope': 'Private diagnostic source, production unchanged; not exact frozen ba artifact'})
version = cli.command('/usr/bin/rustc', '-Vv').capture().run()
save('toolchain.json', {'exit': version.exit_code, 'stdout': version.stdout, 'stderr': version.stderr})
assert version.exit_code == 0
binary = epoch / 'probe'
argv = ['/usr/bin/rustc', '--edition=2024', '--crate-name', 'talent_stage_diagnostic', '-C', 'opt-level=1', '-C', 'linker=clang', '-C', 'target-cpu=native', '-C', 'link-arg=-fuse-ld=mold', '-C', 'link-arg=-Wl,--thread-count=4', '--extern', 'wow_ui_sim=' + str(library), '-L', 'dependency=' + str(library.parent), str(frozen_source), '-o', str(binary)]
save('compile-submission.json', {'argv': argv, 'cwd': str(repo), 'source_sha256': hashlib.sha256(payload).hexdigest(), 'scope': 'One new private observational wrapper; no Cargo/library rebuild or acceptance test rerun'})
value = cli.command(*argv).cwd(str(repo)).capture().run()
(epoch / 'compile.stdout').write_text(value.stdout)
(epoch / 'compile.stderr').write_text(value.stderr)
save('compile-result.json', {'exit': value.exit_code, 'source_unchanged': payload == source.read_bytes()})
if value.exit_code != 0:
    save('outcome.json', {'status': 'COMPILE_FAILED', 'executed_cases': 0})
    raise RuntimeError('Talent stage diagnostic compilation failed')
with binary.open('rb') as stream:
    digest = hashlib.file_digest(stream, 'sha256').hexdigest()
binary.chmod(0o555)
save('artifact.json', {'path': str(binary), 'bytes': binary.stat().st_size, 'sha256': digest, 'scope': 'New private diagnostic artifact; runtime pending'})
save('outcome.json', {'status': 'COMPILED_NOT_EXECUTED', 'executed_cases': 0})
print(str(epoch))
