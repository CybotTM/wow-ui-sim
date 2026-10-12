REVISION = '1d2b0ea30'
ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/talent-handler-log-current')
PATHS = ['tests/talent_panel_onupdate_perf.rs', 'tools/talent_handler_log.py', 'tools/test_talent_handler_log.py', 'Cargo.toml', 'Cargo.lock']
SELECTOR = 'talent_panel_onupdate_perf::talent_panel_open_onupdate_tick_latency_stays_under_budget'

def hashes():
    return {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in PATHS}

def require_capacity(epoch, phase):
    if not admit(epoch, phase):
        raise RuntimeError(phase + ' capacity exhausted; no inferred execution')

def assert_source(epoch):
    head = cli.git('rev-parse', 'HEAD').cwd(str(REPO)).capture().run()
    diff = cli.git('diff', REVISION, '--', 'src', 'tests', 'tools', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
    save(epoch, 'source-applicability.json', {'revision': REVISION, 'head': head.stdout.strip(), 'diff': diff.stdout, 'exit': diff.exit_code, 'stderr': diff.stderr})
    if head.exit_code != 0 or diff.exit_code != 0 or diff.stdout:
        raise RuntimeError('Diagnostic relevant source changed')

def run_observation():
    import os
    epoch = ROOT / ('diagnostic-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    print(str(epoch), flush=True)
    require_capacity(epoch, 'compile')
    assert_source(epoch)
    before = hashes()
    save(epoch, 'source-before.json', before)
    for name, argv in [('cargo-version', ['/usr/bin/cargo', '-V']), ('rustc-version', ['/usr/bin/rustc', '-Vv'])]:
        if command(epoch, name, argv).exit_code != 0:
            raise RuntimeError('Toolchain inspection failed')
    compiled = command(epoch, 'compile', ['/usr/bin/cargo', 'test', '--test', 'integration', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12'])
    after = hashes()
    save(epoch, 'source-after-compile.json', after)
    if compiled.exit_code != 0 or before != after:
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
    inherited = {key: os.environ.get(key) for key in ['WOW_SIM_CASC', 'WOW_SIM_NO_SOUND', 'WOW_SIM_NO_ADDONS', 'WOW_SIM_NO_SAVED_VARS', 'WOW_PRODUCT', 'WOW_INSTALL_PATH', 'WOW_DATA_PATH']}
    save(epoch, 'input-context.json', {'inherited_allowlist': inherited, 'explicit_override': {'WOW_SIM_LOG_HANDLER_TIMINGS': '0'}, 'scope': 'Unchanged canonical fixture, isolated one selector/thread; historical sixteen-thread/full environment equivalence unproved. No CASC/sound/addon/SV overrides added.'})
    manifest = REPO / 'data/blizzard-ui-files/retail.txt'
    provenance = Path('/home/osso/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/.wow-ui-sim-blizzard-ui-provenance')
    save(epoch, 'cache-context.json', {'manifest_path': str(manifest), 'manifest_sha256': hashlib.sha256(manifest.read_bytes()).hexdigest(), 'provenance_path': str(provenance), 'provenance_text': provenance.read_text() if provenance.exists() else None, 'scope': 'Current manifest/cache metadata, not historical identity or archive/decode attestation'})
    runtime = command(epoch, 'runtime', ['/usr/bin/timeout', '90', str(binary), SELECTOR, '--exact', '--nocapture', '--test-threads=1'], {'WOW_SIM_LOG_HANDLER_TIMINGS': '0'})
    save(epoch, 'runtime-outcome.json', {'exit': runtime.exit_code, 'scope': 'Keep original assertion/timeout outcome. Logging perturbs latency; do not promote observed PASS or erase FAIL.'})
    if runtime.exit_code == 124:
        save(epoch, 'outcome.json', {'status': 'RUNTIME_TIMEOUT', 'scope': 'No completed observation inferred; inspect owned cgroup for lingering children'})
        raise RuntimeError('Diagnostic runtime timeout')
    parsed = command(epoch, 'parse', ['/usr/bin/python3', '-B', 'tools/talent_handler_log.py', str(epoch / 'runtime.stderr')])
    if parsed.exit_code != 0:
        save(epoch, 'outcome.json', {'status': 'PARSER_FAILED', 'runtime_exit': runtime.exit_code, 'parse_exit': parsed.exit_code})
        raise RuntimeError('Incomplete or malformed real diagnostic log')
    report = json.loads(parsed.stdout)
    if len(report['ticks']) != 54 or report['expected_counts'] != {'baseline': 20, 'settle': 10, 'steady': 24}:
        raise RuntimeError('Unexpected observation sample counts')
    fmt = command(epoch, 'fmt', ['/usr/bin/cargo', 'fmt', '--check'])
    final = hashes()
    save(epoch, 'source-after-runtime.json', final)
    save(epoch, 'outcome.json', {'status': 'OBSERVED', 'runtime_exit': runtime.exit_code, 'parse_exit': parsed.exit_code, 'fmt_exit': fmt.exit_code, 'samples': 54, 'source_unchanged': before == final, 'performance_repair_claim': False, 'default_check_reuse': 'Friend check-20261012T124825577210Z; src/Cargo production source unchanged, new test compiled by targeted integration build'})
    if fmt.exit_code != 0 or before != final:
        raise RuntimeError('Formatting failed or observation source changed')
    print(str(epoch), flush=True)

run_observation()
