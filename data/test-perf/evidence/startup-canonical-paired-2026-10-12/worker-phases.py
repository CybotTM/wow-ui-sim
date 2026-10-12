REVISION = '59f3ff136'
ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/startup-canonical-paired-current')
PATHS = ['tests/startup_perf.rs', 'tests/startup_perf/observation.rs', 'tests/perf/base_game.rs', 'Cargo.toml', 'Cargo.lock']
SELECTOR = 'startup_perf::full_game_startup_stays_under_budget'
SERIALIZER = 'startup_perf::observation::tests::addon_observation_roundtrip_preserves_values_and_only_diagnostic_counts'

def source_hashes():
    return {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in PATHS}

def require_capacity(epoch, phase):
    if not admit(epoch, phase):
        raise RuntimeError(phase + ' capacity exhausted; no inferred execution')

def assert_source(epoch):
    head = cli.git('rev-parse', 'HEAD').cwd(str(REPO)).capture().run()
    diff = cli.git('diff', REVISION, '--', 'src', 'tests', 'tools', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
    save(epoch, 'source-applicability.json', {'revision': REVISION, 'head': head.stdout.strip(), 'diff': diff.stdout, 'exit': diff.exit_code, 'stderr': diff.stderr})
    if head.exit_code != 0 or diff.exit_code != 0 or diff.stdout:
        raise RuntimeError('Relevant source changed')

def capture_inputs(epoch):
    import os
    keys = ['WOW_SIM_CASC', 'WOW_SIM_NO_SOUND', 'WOW_SIM_NO_ADDONS', 'WOW_SIM_NO_SAVED_VARS', 'WOW_PRODUCT', 'WOW_INSTALL_PATH', 'WOW_DATA_PATH', 'WOW_SIM_LOG_HANDLER_TIMINGS', 'WOW_SIM_PROFILE_LUA_FILES', 'WOW_SIM_TIMEOUT_FIXTURE_CAPACITY', 'WOW_SIM_BYTECODE_CACHE_DIR', 'XDG_CACHE_HOME']
    save(epoch, 'input-context.json', {'inherited_allowlist': {key: os.environ.get(key) for key in keys}, 'explicit_override': {}, 'scope': 'Canonical current fixture, isolated one selector/thread; no historical equivalence or cache contents attestation'})
    manifest = REPO / 'data/blizzard-ui-files/retail.txt'
    provenance = Path('/home/osso/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/.wow-ui-sim-blizzard-ui-provenance')
    save(epoch, 'cache-context.json', {'manifest_path': str(manifest), 'manifest_sha256': hashlib.sha256(manifest.read_bytes()).hexdigest(), 'provenance_path': str(provenance), 'provenance_text': provenance.read_text() if provenance.exists() else None, 'scope': 'Current metadata, not historical identity or cache contents/decode attestation'})

def observe_startup():
    epoch = ROOT / ('diagnostic-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    print(str(epoch), flush=True)
    require_capacity(epoch, 'compile')
    assert_source(epoch)
    before = source_hashes()
    save(epoch, 'source-before.json', before)
    for name, argv in [('cargo-version', ['/usr/bin/cargo', '-V']), ('rustc-version', ['/usr/bin/rustc', '-Vv'])]:
        if command(epoch, name, argv).exit_code != 0:
            raise RuntimeError('Toolchain inspection failed')
    compiled = command(epoch, 'compile', ['/usr/bin/cargo', 'test', '--test', 'integration', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12'])
    after = source_hashes()
    save(epoch, 'source-after-compile.json', after)
    if compiled.exit_code != 0 or before != after:
        save(epoch, 'outcome.json', {'status': 'COMPILE_FAILED_OR_SOURCE_CHANGED', 'compile_exit': compiled.exit_code, 'source_unchanged': before == after})
        raise RuntimeError('Compile failed or source changed')
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
    capture_inputs(epoch)
    serialized = command(epoch, 'serialization', ['/usr/bin/timeout', '90', str(binary), SERIALIZER, '--exact', '--nocapture', '--test-threads=1'])
    runtime = command(epoch, 'runtime', ['/usr/bin/timeout', '90', str(binary), SELECTOR, '--exact', '--nocapture', '--test-threads=1'])
    if runtime.exit_code == 124 or serialized.exit_code == 124:
        save(epoch, 'outcome.json', {'status': 'RUNTIME_TIMEOUT', 'serialization_exit': serialized.exit_code, 'runtime_exit': runtime.exit_code, 'scope': 'Inspect owned cgroup children; no completed observation inferred'})
        raise RuntimeError('Runtime timeout')
    rows = [json.loads(line.split('[startup-observation] ', 1)[1]) for line in runtime.stderr.splitlines() if line.startswith('[startup-observation] ')]
    summaries = [row for row in rows if row.get('kind') == 'startup_summary']
    addon_rows = [row for row in rows if row.get('kind') == 'startup_addon_load']
    complete = len(summaries) == 1 and len(addon_rows) == summaries[0]['addon_count'] and [row['ordinal'] for row in addon_rows] == list(range(1, len(addon_rows) + 1)) and rows[-1] == summaries[0]
    save(epoch, 'observations.json', {'rows': rows, 'complete_ordered': complete, 'scope': 'Main extraction only; independent same-run coherence audit required. Buckets overlap; no causal repair'})
    fmt = command(epoch, 'fmt', ['/usr/bin/cargo', 'fmt', '--check'])
    final = source_hashes()
    save(epoch, 'source-after-runtime.json', final)
    save(epoch, 'outcome.json', {'status': 'OBSERVED' if complete else 'INCOMPLETE_OBSERVATION', 'serialization_exit': serialized.exit_code, 'runtime_exit': runtime.exit_code, 'fmt_exit': fmt.exit_code, 'addon_rows': len(addon_rows), 'complete_ordered': complete, 'source_unchanged': before == final, 'performance_repair_claim': False, 'default_check_reuse': 'Friend check-20261012T124825577210Z unchanged production/library scope only; new test compilation separate'})
    if fmt.exit_code != 0 or before != final or serialized.exit_code != 0 or not complete:
        raise RuntimeError('Observation/serialization/format/source proof failed; retain actual outcome')
    print(str(epoch), flush=True)

observe_startup()
