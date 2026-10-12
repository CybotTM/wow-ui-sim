REVISION = '7487c8bec'
ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/mount-item-current')
PATHS = ['src/c_api/c_mount_item.rs', 'src/c_api/mod.rs', 'src/c_api/registration.rs', 'src/lua_api/state.rs', 'src/lua_api/state/sim_state.rs', 'tests/mount_from_item.rs', 'tests/mount_item_player_choice.rs', 'tests/mount_spell_identifier.rs', 'docs/specs/mount-item-relations.md', 'Cargo.toml', 'Cargo.lock']

def hashes():
    return {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in PATHS}

def require_capacity(epoch, phase):
    global MIN_AVAILABLE
    MIN_AVAILABLE = 6 if phase in ['compile', 'check'] else 20
    if not admit(epoch, phase):
        raise RuntimeError(phase + ' capacity exhausted; no inferred cases')

def check_source(epoch):
    diff = cli.git('diff', REVISION, '--', 'src', 'tests', 'tools', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
    save(epoch, 'source-applicability.json', {'revision': REVISION, 'diff': diff.stdout, 'exit': diff.exit_code})
    if diff.exit_code != 0 or diff.stdout:
        raise RuntimeError('Relevant source changed')
    return hashes()

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

def run_profile(profile, features):
    epoch = ROOT / (profile + '-final-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    print(str(epoch), flush=True)
    require_capacity(epoch, 'compile')
    before = check_source(epoch)
    save(epoch, 'source-before.json', before)
    for name, argv in [('cargo-version', ['/usr/bin/cargo', '-V']), ('rustc-version', ['/usr/bin/rustc', '-Vv'])]:
        if command(epoch, name, argv).exit_code != 0:
            raise RuntimeError('Toolchain inspection failed')
    compiled = command(epoch, 'compile', ['/usr/bin/cargo', 'test', '--test', 'integration', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12'] + features)
    after = hashes()
    save(epoch, 'source-after-compile.json', after)
    if compiled.exit_code != 0 or before != after:
        save(epoch, 'outcome.json', {'status': 'COMPILE_FAILED_OR_SOURCE_CHANGED', 'compile_exit': compiled.exit_code})
        raise RuntimeError('Compile failed or source changed')
    records = [json.loads(line) for line in compiled.stdout.splitlines() if line.strip()]
    integration = seal_record(epoch, records, 'integration', 'integration')
    normal = seal_record(epoch, records, 'wow-sim', 'normal') if profile == 'ptr' else None
    require_capacity(epoch, 'runtime')
    import os
    keys = ['WOW_SIM_CASC', 'WOW_SIM_NO_SOUND', 'WOW_SIM_NO_ADDONS', 'WOW_SIM_NO_SAVED_VARS', 'WOW_PRODUCT', 'WOW_INSTALL_PATH', 'WOW_DATA_PATH', 'WOW_SIM_LOG_HANDLER_TIMINGS', 'WOW_SIM_PROFILE_LUA_FILES', 'WOW_SIM_TIMEOUT_FIXTURE_CAPACITY', 'WOW_SIM_ENABLE_BYTECODE_CACHE', 'WOW_SIM_DISABLE_BYTECODE_CACHE', 'ASSET_RESOLVER_CACHE_DIR', 'WOW_SIM_VERBOSE', 'XDG_CACHE_HOME']
    save(epoch, 'input-context.json', {'inherited_allowlist': {key: os.environ.get(key) for key in keys}, 'query_consumer_overrides': {}, 'normal_override': {'WOW_SIM_NO_SOUND': '1'} if profile == 'ptr' else {}, 'scope': 'Current parent context only; no historical/child/cache attestation'})
    groups = [('consumer', 'mount_item_player_choice::', 2)]
    if profile == 'ptr':
        groups = [('query', 'mount_from_item::', 8), ('spell-preservation', 'mount_spell_identifier::', 16)] + groups
    results = []
    for label, selector, expected in groups:
        result = command(epoch, label, ['/usr/bin/timeout', '90', str(integration), selector, '--nocapture', '--test-threads=1'])
        results.append({'label': label, 'selector': selector, 'expected': expected, 'exit': result.exit_code})
        save(epoch, 'runtime-results.json', results)
        if result.exit_code != 0 or ('test result: ok. ' + str(expected) + ' passed; 0 failed;') not in result.stdout:
            save(epoch, 'outcome.json', {'status': 'RUNTIME_FAILED', 'results': results})
            raise RuntimeError('Actual current runtime failure retained')
    if profile == 'retail':
        reuse = cli.git('diff', 'ab8dd8739', '--', 'src', 'tests/mount_from_item.rs', 'tests/mount_spell_identifier.rs', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
        save(epoch, 'retail-query-normal-reuse.json', {'revision': 'ab8dd8739', 'prior_epoch': 'retail-green-20261012T152010421981Z', 'diff': reuse.stdout, 'exit': reuse.exit_code, 'scope': '8 query +16 preservation +normal source scope unchanged; consumer source changed and executed here. No old seal read/hash/execution.'})
        if reuse.exit_code != 0 or reuse.stdout:
            raise RuntimeError('Retail prior proof no longer applicable')
    else:
        require_capacity(epoch, 'normal')
        probe = (ROOT / 'normal-probe.lua').read_text()
        tested = command(epoch, 'normal', ['/usr/bin/timeout', '90', str(normal), '--no-addons', '--no-saved-vars', '--exec-lua', probe, 'lua-errors'], {'WOW_SIM_NO_SOUND': '1'})
        marker = 'MOUNT_ITEM_NORMAL_PROBE_PASS'
        marker_lines = [line for line in (tested.stdout + '\n' + tested.stderr).splitlines() if line.rstrip().endswith(marker)]
        clean_stdout = '\n'.join(line for line in tested.stdout.splitlines() if not line.rstrip().endswith(marker)).strip()
        errors = [] if not clean_stdout else json.loads(clean_stdout)
        save(epoch, 'normal-outcome.json', {'exit': tested.exit_code, 'marker_lines': marker_lines, 'json_stdout': clean_stdout})
        if tested.exit_code != 0 or len(marker_lines) != 1 or errors != []:
            raise RuntimeError('Normal current probe/startup failed')
    final = hashes()
    save(epoch, 'source-after-runtime.json', final)
    save(epoch, 'outcome.json', {'status': 'PROFILE_CAPTURED', 'profile': profile, 'executed_cases': sum(row['expected'] for row in results), 'results': results, 'source_unchanged': before == final, 'full_handoff_complete': False})
    if before != final:
        raise RuntimeError('Source changed during profile capture')
    return str(epoch)

def check_final_source():
    epoch = ROOT / ('check-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    require_capacity(epoch, 'check')
    before = check_source(epoch)
    save(epoch, 'source-before.json', before)
    fmt = command(epoch, 'fmt', ['/usr/bin/cargo', 'fmt', '--check'])
    checked = command(epoch, 'check', ['/usr/bin/cargo', 'check', '--offline', '--locked', '-j', '12'])
    final = hashes()
    save(epoch, 'source-after.json', final)
    save(epoch, 'outcome.json', {'fmt_exit': fmt.exit_code, 'check_exit': checked.exit_code, 'source_unchanged': before == final})
    if fmt.exit_code != 0 or checked.exit_code != 0 or before != final:
        raise RuntimeError('Final source check/format failed')
    return str(epoch)

results = {}
results['retail'] = run_profile('retail', [])
results['ptr'] = run_profile('ptr', ['--no-default-features', '--features', 'sound,gui,casc,client-ptr'])
results['check'] = check_final_source()
save(ROOT, 'final-phases-result.json', results)
print(json.dumps(results), flush=True)
