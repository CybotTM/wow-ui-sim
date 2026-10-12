REVISION = 'd6d8de29a'
ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/guid-is-player-current')
PATHS = ['src/c_api/c_player_info_guid.rs', 'src/c_api/mod.rs', 'src/c_api/registration.rs', 'src/lua_api/globals/unit_misc.rs', 'src/lua_api/globals/unit_probes.rs', 'tests/guid_is_player.rs', 'docs/specs/guid-is-player.md', 'Cargo.toml', 'Cargo.lock']
MIN_AVAILABLE = 6

def source_hashes():
    return {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in PATHS}

def require_capacity(epoch, phase):
    global MIN_AVAILABLE
    MIN_AVAILABLE = 6 if phase == 'compile' else 20
    if not admit(epoch, phase):
        raise RuntimeError(phase + ' resource blocked; no inferred case credit')

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
    keys = ['WOW_SIM_CASC', 'WOW_SIM_NO_SOUND', 'WOW_SIM_NO_ADDONS', 'WOW_SIM_NO_SAVED_VARS', 'WOW_PRODUCT', 'WOW_INSTALL_PATH', 'WOW_DATA_PATH', 'WOW_SIM_LOG_HANDLER_TIMINGS', 'WOW_SIM_ENABLE_BYTECODE_CACHE', 'WOW_SIM_DISABLE_BYTECODE_CACHE', 'XDG_CACHE_HOME']
    save(epoch, 'input-context.json', {'inherited_allowlist': {key: os.environ.get(key) for key in keys}, 'normal_overrides': {'WOW_SIM_NO_SOUND': '1'}, 'scope': 'Current recorded allowlist, not complete inherited-environment/cache/native attestation'})

def run_profile(profile):
    epoch = ROOT / (profile + '-green-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
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
    argv = ['/usr/bin/cargo', 'test', '--test', 'integration', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12']
    if profile == 'ptr':
        argv.extend(['--no-default-features', '--features', 'sound,gui,casc,client-ptr'])
    compiled = command(epoch, 'compile', argv)
    after = source_hashes()
    save(epoch, 'source-after-compile.json', after)
    if compiled.exit_code != 0 or before != after:
        save(epoch, 'outcome.json', {'status': 'COMPILE_FAILED_OR_SOURCE_CHANGED', 'compile_exit': compiled.exit_code})
        raise RuntimeError('Compile failed or source changed')
    records = [json.loads(line) for line in compiled.stdout.splitlines() if line.strip()]
    integration = seal_record(epoch, records, 'integration', 'integration')
    normal = seal_record(epoch, records, 'wow-sim', 'normal')
    require_capacity(epoch, 'runtime')
    capture_inputs(epoch)
    runtime = command(epoch, 'runtime', ['/usr/bin/timeout', '90', str(integration), 'guid_is_player::', '--nocapture', '--test-threads=1'])
    save(epoch, 'runtime-outcome.json', {'exit': runtime.exit_code, 'expected_cases': 9})
    if runtime.exit_code != 0 or 'test result: ok. 9 passed; 0 failed;' not in runtime.stdout:
        save(epoch, 'outcome.json', {'status': 'RUNTIME_FAILED', 'runtime_exit': runtime.exit_code, 'source_unchanged': before == source_hashes()})
        raise RuntimeError('Actual query/consumer failure retained')
    require_capacity(epoch, 'normal')
    probe = (ROOT / 'normal-probe.lua').read_text()
    normal_run = command(epoch, 'normal', ['/usr/bin/timeout', '90', str(normal), '--no-addons', '--no-saved-vars', '--exec-lua', probe, 'lua-errors'], {'WOW_SIM_NO_SOUND': '1'})
    marker = 'GUID_PLAYER_NORMAL_PROBE_PASS'
    marker_lines = [line for line in (normal_run.stdout + '\n' + normal_run.stderr).splitlines() if line.rstrip().endswith(marker)]
    clean_stdout = '\n'.join(line for line in normal_run.stdout.splitlines() if not line.rstrip().endswith(marker)).strip()
    save(epoch, 'normal-outcome.json', {'exit': normal_run.exit_code, 'probe_marker_lines': marker_lines, 'json_stdout': clean_stdout})
    errors = [] if not clean_stdout else json.loads(clean_stdout)
    if normal_run.exit_code != 0 or len(marker_lines) != 1 or errors != []:
        raise RuntimeError('Normal startup/probe/Lua-errors failed; actual records retained')
    fmt = command(epoch, 'fmt', ['/usr/bin/cargo', 'fmt', '--check'])
    final = source_hashes()
    save(epoch, 'source-after-runtime.json', final)
    save(epoch, 'outcome.json', {'status': 'PROFILE_CAPTURED', 'profile': profile, 'runtime_cases': 9, 'runtime_exit': runtime.exit_code, 'normal_exit': normal_run.exit_code, 'normal_marker_count': len(marker_lines), 'fmt_exit': fmt.exit_code, 'source_unchanged': before == final, 'default_check': 'PENDING final integration', 'publication': 'PENDING separate current reconciliation', 'full_handoff_complete': False})
    if fmt.exit_code != 0 or before != final:
        raise RuntimeError('Source/formatting failure retained')
    return str(epoch)

results = []
for profile in ['retail', 'ptr']:
    results.append({'profile': profile, 'epoch': run_profile(profile)})
    save(ROOT, 'green-phases-result.json', results)
