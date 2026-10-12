from pathlib import Path
import datetime, hashlib, json

REPO = Path('/home/osso/Projects/wow/wow-ui-sim')
ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/startup-addon-inner-timing-current')
CARGO_LOG = Path('/home/osso/.local/state/wow-ui-sim/verification/bounty-set-maps-current/default-green-compile-20261012T025403623595Z/compile.stdout')

def save(epoch, name, value):
    (epoch / name).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')

def capacity():
    memory = {line.split(':')[0]: int(line.split(':')[1].split()[0]) for line in Path('/proc/meminfo').read_text().splitlines()}
    load = float(Path('/proc/loadavg').read_text().split()[0])
    cargo = []
    for process in Path('/proc').iterdir():
        if not process.name.isdigit():
            continue
        try:
            if (process / 'comm').read_text().strip() == 'cargo' and (process / 'cwd').resolve() in [REPO, REPO.parent / 'full-suite-checkout']:
                cargo.append(process.name)
        except OSError:
            continue
    available = memory['MemAvailable'] / 1048576
    return {'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'load1': load, 'available_gib': available, 'relevant_cargo': cargo, 'admitted': available >= 6 and load < 24 and not cargo}

def select_library(records, name):
    selected = [record for record in records if record.get('reason') == 'compiler-artifact' and record.get('target', {}).get('name') == name and 'lib' in record.get('target', {}).get('kind', [])]
    if len(selected) != 1:
        raise RuntimeError('Expected one library Cargo record for ' + name)
    record = selected[0]
    libraries = [Path(path) for path in record['filenames'] if path.endswith('.rlib')]
    if len(libraries) != 1 or not libraries[0].exists():
        raise RuntimeError('Selected library unavailable: ' + name)
    library = libraries[0]
    return library, {'cargo_record': record, 'bytes': library.stat().st_size, 'mtime_ns': library.stat().st_mtime_ns}

def compile_probe():
    epoch = ROOT / ('compile-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    admission = capacity()
    save(epoch, 'admission.json', admission)
    if not admission['admitted']:
        save(epoch, 'outcome.json', {'status': 'RESOURCE_BLOCKED', 'compiled': False, 'executed_cases': 0})
        print(str(epoch))
        return
    payload = (ROOT / 'probe.rs').read_bytes()
    source = epoch / 'probe.rs'
    source.write_bytes(payload)
    records = [json.loads(line) for line in CARGO_LOG.read_text().splitlines() if line.startswith('{')]
    libraries = {name: select_library(records, name) for name in ['wow_ui_sim', 'serde_json']}
    if 'client-retail' not in libraries['wow_ui_sim'][1]['cargo_record']['features']:
        raise RuntimeError('Wrong profile library selected')
    save(epoch, 'selected-libraries.json', {'records': {name: metadata for name, (_, metadata) in libraries.items()}, 'cargo_log': str(CARGO_LOG), 'scope': 'Saved Cargo projection and current path/stat only; library payloads not hashed; not hermetic or frozen-ba provenance'})
    revision = cli.git('rev-parse', 'HEAD').cwd(str(REPO)).capture().run()
    if revision.exit_code != 0:
        raise RuntimeError('Cannot record current revision')
    save(epoch, 'revision.json', {'head': revision.stdout.strip(), 'library_epoch': 'default-green-compile-20261012T025403623595Z', 'library_producer': '262987b1a65f049d5c3e158b18a8eb73cc22e4ba', 'scope': 'Private observational wrapper, no source/library rebuild; not frozen ba acceptance artifact'})
    version = cli.command('/usr/bin/rustc', '-Vv').capture().run()
    save(epoch, 'toolchain.json', {'exit': version.exit_code, 'stdout': version.stdout, 'stderr': version.stderr})
    if version.exit_code != 0:
        raise RuntimeError('Cannot inspect rustc')
    binary = epoch / 'probe'
    argv = ['/usr/bin/rustc', '--edition=2024', '--crate-name', 'startup_addon_inner_diagnostic', '-C', 'opt-level=1', '-C', 'linker=clang', '-C', 'target-cpu=native', '-C', 'link-arg=-fuse-ld=mold', '-C', 'link-arg=-Wl,--thread-count=4']
    for name, (library, _) in libraries.items():
        argv.extend(['--extern', name + '=' + str(library)])
    argv.extend(['-L', 'dependency=' + str(libraries['wow_ui_sim'][0].parent), str(source), '-o', str(binary)])
    save(epoch, 'compile-submission.json', {'argv': argv, 'cwd': str(REPO), 'source_sha256': hashlib.sha256(payload).hexdigest(), 'scope': 'One new private diagnostic wrapper; no Cargo rebuild or acceptance-test execution'})
    result = cli.command(*argv).cwd(str(REPO)).capture().run()
    (epoch / 'compile.stdout').write_text(result.stdout)
    (epoch / 'compile.stderr').write_text(result.stderr)
    unchanged = payload == (ROOT / 'probe.rs').read_bytes()
    save(epoch, 'compile-result.json', {'exit': result.exit_code, 'source_unchanged': unchanged})
    if result.exit_code != 0 or not unchanged:
        save(epoch, 'outcome.json', {'status': 'COMPILE_FAILED_OR_SOURCE_CHANGED', 'executed_cases': 0})
        print(str(epoch))
        return
    with binary.open('rb') as stream:
        digest = hashlib.file_digest(stream, 'sha256').hexdigest()
    binary.chmod(0o555)
    save(epoch, 'artifact.json', {'path': str(binary), 'bytes': binary.stat().st_size, 'sha256': digest})
    save(epoch, 'outcome.json', {'status': 'COMPILED_NOT_EXECUTED', 'executed_cases': 0})
    (ROOT / 'latest-compile.json').write_text(json.dumps({'epoch': str(epoch)}) + '\n')
    print(str(epoch))

compile_probe()
