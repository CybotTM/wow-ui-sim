REVISION = '0808b3cd0'
ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/mount-item-current')
PATHS = ['tests/mount_from_item.rs', 'docs/specs/mount-item-relations.md', 'Cargo.toml', 'Cargo.lock']

def source_hashes():
    return {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in PATHS}

def require_capacity(epoch, phase):
    global MIN_AVAILABLE
    MIN_AVAILABLE = 20 if phase == 'runtime' else 6
    if not admit(epoch, phase):
        raise RuntimeError(phase + ' capacity exhausted; no cases inferred')

def observe_red():
    epoch = ROOT / ('red-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    print(str(epoch), flush=True)
    require_capacity(epoch, 'compile')
    diff = cli.git('diff', REVISION, '--', 'src', 'tests', 'tools', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
    save(epoch, 'source-applicability.json', {'revision': REVISION, 'diff': diff.stdout, 'exit': diff.exit_code})
    if diff.exit_code != 0 or diff.stdout:
        raise RuntimeError('Relevant source changed')
    before = source_hashes()
    save(epoch, 'source-before.json', before)
    for name, argv in [('cargo-version', ['/usr/bin/cargo', '-V']), ('rustc-version', ['/usr/bin/rustc', '-Vv'])]:
        if command(epoch, name, argv).exit_code != 0:
            raise RuntimeError('Toolchain inspection failed')
    compiled = command(epoch, 'compile', ['/usr/bin/cargo', 'test', '--test', 'integration', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12'])
    after = source_hashes()
    save(epoch, 'source-after-compile.json', after)
    if compiled.exit_code != 0 or before != after:
        save(epoch, 'outcome.json', {'status': 'COMPILE_FAILED_OR_SOURCE_CHANGED', 'compile_exit': compiled.exit_code})
        raise RuntimeError('Compilation failed or source changed')
    records = [json.loads(line) for line in compiled.stdout.splitlines() if line.strip()]
    selected = [row for row in records if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == 'integration' and row.get('executable')]
    if len(selected) != 1:
        raise RuntimeError('Expected one integration artifact')
    record = selected[0]
    payload = Path(record['executable']).read_bytes()
    binary = epoch / 'integration-sealed'
    binary.write_bytes(payload)
    binary.chmod(0o555)
    save(epoch, 'artifact.json', {'path': str(binary), 'source': record['executable'], 'bytes': len(payload), 'sha256': hashlib.sha256(payload).hexdigest(), 'cargo_record': record})
    del payload
    require_capacity(epoch, 'runtime')
    runtime = command(epoch, 'runtime', ['/usr/bin/timeout', '90', str(binary), 'mount_from_item::', '--nocapture', '--test-threads=1'])
    fmt = command(epoch, 'fmt', ['/usr/bin/cargo', 'fmt', '--check'])
    final = source_hashes()
    save(epoch, 'source-after-runtime.json', final)
    save(epoch, 'outcome.json', {'status': 'CAPTURED', 'runtime_exit': runtime.exit_code, 'fmt_exit': fmt.exit_code, 'source_unchanged': before == final, 'scope': 'Pre-producer original failure boundary retained; independent classification needed; no producer/caller proof'})
    if runtime.exit_code == 124:
        raise RuntimeError('Runtime timeout; inspect owned cgroup children')
    if before != final or fmt.exit_code != 0:
        raise RuntimeError('Source/formatting proof failed')
    print(str(epoch), flush=True)

observe_red()
