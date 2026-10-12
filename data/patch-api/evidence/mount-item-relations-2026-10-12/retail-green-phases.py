REVISION = 'ab8dd8739'
ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/mount-item-current')
PATHS = ['src/c_api/c_mount_item.rs', 'src/c_api/mod.rs', 'src/c_api/registration.rs', 'src/lua_api/state.rs', 'src/lua_api/state/sim_state.rs', 'tests/mount_from_item.rs', 'tests/mount_item_player_choice.rs', 'tests/mount_spell_identifier.rs', 'docs/specs/mount-item-relations.md', 'Cargo.toml', 'Cargo.lock']

def hashes():
    return {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in PATHS}

def require_capacity(epoch, phase):
    global MIN_AVAILABLE
    MIN_AVAILABLE = 20 if phase != 'compile' else 6
    if not admit(epoch, phase):
        raise RuntimeError(phase + ' capacity exhausted; no inferred cases')

def seal_record(epoch, records, target, label):
    selected = [row for row in records if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == target and row.get('executable') and row.get('profile', {}).get('test') == (target == 'integration')]
    if len(selected) != 1:
        raise RuntimeError('Expected one ' + target + ' executable')
    record = selected[0]
    payload = Path(record['executable']).read_bytes()
    binary = epoch / (label + '-sealed')
    binary.write_bytes(payload)
    binary.chmod(0o555)
    save(epoch, label + '-artifact.json', {'path': str(binary), 'source': record['executable'], 'bytes': len(payload), 'sha256': hashlib.sha256(payload).hexdigest(), 'cargo_record': record})
    return binary

def capture_inputs(epoch):
    import os
    keys = ['WOW_SIM_CASC', 'WOW_SIM_NO_SOUND', 'WOW_SIM_NO_ADDONS', 'WOW_SIM_NO_SAVED_VARS', 'WOW_PRODUCT', 'WOW_INSTALL_PATH', 'WOW_DATA_PATH', 'WOW_SIM_LOG_HANDLER_TIMINGS', 'WOW_SIM_PROFILE_LUA_FILES', 'WOW_SIM_TIMEOUT_FIXTURE_CAPACITY', 'WOW_SIM_ENABLE_BYTECODE_CACHE', 'WOW_SIM_DISABLE_BYTECODE_CACHE', 'ASSET_RESOLVER_CACHE_DIR', 'WOW_SIM_VERBOSE', 'XDG_CACHE_HOME']
    save(epoch, 'input-context.json', {'inherited_allowlist': {key: os.environ.get(key) for key in keys}, 'query_consumer_overrides': {}, 'normal_override': {'WOW_SIM_NO_SOUND': '1'}, 'scope': 'Current source/fixture context, no child/cache-content/historical equivalence attestation'})

def run_retail():
    epoch = ROOT / ('retail-green-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
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
    compiled = command(epoch, 'compile', ['/usr/bin/cargo', 'test', '--test', 'integration', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12'])
    after = hashes()
    save(epoch, 'source-after-compile.json', after)
    if compiled.exit_code != 0 or before != after:
        save(epoch, 'outcome.json', {'status': 'COMPILE_FAILED_OR_SOURCE_CHANGED', 'compile_exit': compiled.exit_code})
        raise RuntimeError('Compile failed or source changed')
    records = [json.loads(line) for line in compiled.stdout.splitlines() if line.strip()]
    integration = seal_record(epoch, records, 'integration', 'integration')
    normal = seal_record(epoch, records, 'wow-sim', 'normal')
    require_capacity(epoch, 'runtime')
    capture_inputs(epoch)
    results = []
    for label, selector, expected in [('query', 'mount_from_item::', 8), ('spell-preservation', 'mount_spell_identifier::', 16)]:
        result = command(epoch, label, ['/usr/bin/timeout', '90', str(integration), selector, '--nocapture', '--test-threads=1'])
        results.append({'label': label, 'selector': selector, 'expected': expected, 'exit': result.exit_code})
        save(epoch, 'runtime-results.json', results)
        if result.exit_code != 0 or ('test result: ok. ' + str(expected) + ' passed; 0 failed;') not in result.stdout:
            save(epoch, 'outcome.json', {'status': 'QUERY_OR_PRESERVATION_FAILED', 'results': results})
            raise RuntimeError('Actual query/preservation failure retained')
    consumer = command(epoch, 'consumer-baseline', ['/usr/bin/timeout', '90', str(integration), 'mount_item_player_choice::mount_item_player_choice_real_item_consumer_returns_boolean', '--exact', '--nocapture', '--test-threads=1'])
    save(epoch, 'consumer-baseline-outcome.json', {'exit': consumer.exit_code, 'scope': 'Actual original caller boundary, no guessed dressability or mount-branch success. Keep prior prerequisites/failure if present.'})
    if consumer.exit_code == 124:
        raise RuntimeError('Consumer timeout; inspect owned cgroup children')
    require_capacity(epoch, 'normal')
    probe = (ROOT / 'normal-probe.lua').read_text()
    tested = command(epoch, 'normal', ['/usr/bin/timeout', '90', str(normal), '--no-addons', '--no-saved-vars', '--exec-lua', probe, 'lua-errors'], {'WOW_SIM_NO_SOUND': '1'})
    marker = 'MOUNT_ITEM_NORMAL_PROBE_PASS'
    marker_lines = [line for line in (tested.stdout + '\n' + tested.stderr).splitlines() if line.rstrip().endswith(marker)]
    clean_stdout = '\n'.join(line for line in tested.stdout.splitlines() if not line.rstrip().endswith(marker)).strip()
    save(epoch, 'normal-outcome.json', {'exit': tested.exit_code, 'probe_marker_lines': marker_lines, 'json_stdout': clean_stdout})
    errors = [] if not clean_stdout else json.loads(clean_stdout)
    if tested.exit_code != 0 or len(marker_lines) != 1 or errors != []:
        raise RuntimeError('Normal startup/probe/JSON result failed; actual output retained')
    fmt = command(epoch, 'fmt', ['/usr/bin/cargo', 'fmt', '--check'])
    final = hashes()
    save(epoch, 'source-after-runtime.json', final)
    save(epoch, 'outcome.json', {'status': 'RETAIL_QUERY_CAPTURED', 'query_cases': 8, 'preservation_cases': 16, 'consumer_exit': consumer.exit_code, 'normal_exit': tested.exit_code, 'normal_probe_marker_count': len(marker_lines), 'fmt_exit': fmt.exit_code, 'source_unchanged': before == final, 'fresh_default_check': 'PENDING final producer integration; old source check invalid after library/state changes', 'ptr': 'PENDING', 'full_handoff_complete': False})
    if fmt.exit_code != 0 or before != final:
        raise RuntimeError('Formatting/source proof failed')
    print(str(epoch), flush=True)

run_retail()
