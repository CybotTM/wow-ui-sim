REVISION = '609033535'
ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/mount-item-current')
PATHS = ['src/c_api/c_mount_item.rs', 'src/c_api/mod.rs', 'src/c_api/registration.rs', 'src/lua_api/state.rs', 'src/lua_api/state/sim_state.rs', 'tests/mount_from_item.rs', 'tests/mount_item_player_choice.rs', 'tests/mount_spell_identifier.rs', 'docs/specs/mount-item-relations.md', 'Cargo.toml', 'Cargo.lock']

def hashes():
    return {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in PATHS}

def require_capacity(epoch, phase):
    global MIN_AVAILABLE
    MIN_AVAILABLE = 6 if phase == 'compile' else 20
    if not admit(epoch, phase):
        raise RuntimeError(phase + ' capacity exhausted; no inferred cases')

def run_observations(profile, features):
    epoch = ROOT / (profile + '-observations-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    print(str(epoch), flush=True)
    require_capacity(epoch, 'compile')
    diff = cli.git('diff', REVISION, '--', 'src', 'tests', 'tools', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
    save(epoch, 'source-applicability.json', {'revision': REVISION, 'diff': diff.stdout, 'exit': diff.exit_code})
    if diff.exit_code != 0 or diff.stdout:
        raise RuntimeError('Relevant source changed')
    before = hashes()
    save(epoch, 'source-before.json', before)
    for name, argv in [('cargo-version', ['/usr/bin/cargo', '-V']), ('rustc-version', ['/usr/bin/rustc', '-Vv'])]:
        if command(epoch, name, argv).exit_code != 0:
            raise RuntimeError('Toolchain inspection failed')
    compiled = command(epoch, 'compile', ['/usr/bin/cargo', 'test', '--test', 'integration', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12'] + features)
    save(epoch, 'source-after-compile.json', hashes())
    if compiled.exit_code != 0 or before != hashes():
        save(epoch, 'outcome.json', {'status': 'COMPILE_FAILED_OR_SOURCE_CHANGED', 'compile_exit': compiled.exit_code})
        raise RuntimeError('Compile failed or source changed')
    records = [json.loads(line) for line in compiled.stdout.splitlines() if line.strip()]
    selected = [row for row in records if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == 'integration' and row.get('executable') and row.get('profile', {}).get('test')]
    if len(selected) != 1:
        raise RuntimeError('Expected one integration executable')
    record = selected[0]
    payload = Path(record['executable']).read_bytes()
    binary = epoch / 'integration-sealed'
    binary.write_bytes(payload)
    binary.chmod(0o555)
    save(epoch, 'integration-artifact.json', {'path': str(binary), 'source': record['executable'], 'bytes': len(payload), 'sha256': hashlib.sha256(payload).hexdigest(), 'cargo_record': record})
    require_capacity(epoch, 'runtime')
    import os
    keys = ['WOW_SIM_CASC', 'WOW_SIM_NO_SOUND', 'WOW_SIM_NO_ADDONS', 'WOW_SIM_NO_SAVED_VARS', 'WOW_PRODUCT', 'WOW_INSTALL_PATH', 'WOW_DATA_PATH', 'WOW_SIM_LOG_HANDLER_TIMINGS', 'WOW_SIM_ENABLE_BYTECODE_CACHE', 'WOW_SIM_DISABLE_BYTECODE_CACHE', 'ASSET_RESOLVER_CACHE_DIR', 'WOW_SIM_VERBOSE', 'XDG_CACHE_HOME']
    save(epoch, 'input-context.json', {'inherited_allowlist': {key: os.environ.get(key) for key in keys}, 'consumer_overrides': {}, 'scope': 'Current parent context; no native/historical/child/cache attestation'})
    result = command(epoch, 'consumer', ['/usr/bin/timeout', '90', str(binary), 'mount_item_player_choice::', '--nocapture', '--test-threads=1'])
    lines = [line for line in result.stderr.splitlines() if line.startswith('[mount-item consumer observation] ')]
    labels = ['provenance', 'input', 'stage', 'API observations', 'result', 'predicate observation', 'predicate source']
    counts = {label: sum('[mount-item consumer ' + label + ']' in line for line in lines) for label in labels}
    expected = {'provenance': 16, 'input': 4, 'stage': 8, 'API observations': 4, 'result': 4, 'predicate observation': 4, 'predicate source': 4}
    final = hashes()
    save(epoch, 'source-after-runtime.json', final)
    save(epoch, 'outcome.json', {'revision': REVISION, 'profile': profile, 'consumer_exit': result.exit_code, 'executed_cases': 2, 'observation_counts': counts, 'expected_observation_counts': expected, 'raw_observation_lines': lines, 'source_unchanged': before == final, 'scope': 'Observation transport fix; existing whole-caller behavior preserved, no prior contrast RED or native dressability claim', 'full_handoff_complete': False})
    if result.exit_code != 0 or 'test result: ok. 2 passed; 0 failed;' not in result.stdout or counts != expected or before != final:
        raise RuntimeError('Consumer behavior/raw observations/source gate failed; output retained')
    fmt = command(epoch, 'fmt', ['/usr/bin/cargo', 'fmt', '--check'])
    if fmt.exit_code != 0:
        raise RuntimeError('Formatting failed')
    reuse = cli.git('diff', '7487c8bec', '--', 'src', 'tests/mount_from_item.rs', 'tests/mount_spell_identifier.rs', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
    save(epoch, 'unchanged-proof-scope.json', {'base_revision': '7487c8bec', 'diff': reuse.stdout, 'exit': reuse.exit_code, 'scope': 'Prior query/preservation/normal/default-library check unchanged; no replay or seal access'})
    if reuse.exit_code != 0 or reuse.stdout:
        raise RuntimeError('Prior proof scope invalid')
    return str(epoch)

results = {'retail': run_observations('retail', []), 'ptr': run_observations('ptr', ['--no-default-features', '--features', 'sound,gui,casc,client-ptr'])}
save(ROOT, 'observation-phases-result.json', results)
print(json.dumps(results), flush=True)
