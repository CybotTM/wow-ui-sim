# Uses the already inspected capacity/command helpers from red-worker.py.
REVISION = 'be9c734a4'
SOURCE_PATHS = ['src/c_api/c_friend_list.rs', 'src/c_api/mod.rs', 'src/lua_api/globals/missing_surface.rs', 'src/lua_api/globals/missing_surface/friend_list.rs', 'tests/friend_info_exact.rs', 'tests/friend_list_who.rs', 'tests/system_api_seeded.rs', 'Cargo.toml', 'Cargo.lock']

def source_hashes():
    return {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in SOURCE_PATHS}

def require_admission(epoch, phase):
    if not admit(epoch, phase):
        raise RuntimeError(phase + ' resource admission blocked')

def check_source(epoch):
    head = cli.git('rev-parse', 'HEAD').cwd(str(REPO)).capture().run()
    diff = cli.git('diff', REVISION, '--', 'src', 'tests', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
    save(epoch, 'source-applicability.json', {'revision': REVISION, 'head': head.stdout.strip(), 'diff': diff.stdout, 'exit': diff.exit_code, 'stderr': diff.stderr})
    if head.exit_code != 0 or diff.exit_code != 0 or diff.stdout:
        raise RuntimeError('Relevant source changed')
    return source_hashes()

def seal_record(epoch, records, target, label):
    selected = [row for row in records if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == target and row.get('executable') and row.get('profile', {}).get('test') == (target == 'integration')]
    if len(selected) != 1:
        raise RuntimeError('Expected one ' + target + ' artifact')
    record = selected[0]
    payload = Path(record['executable']).read_bytes()
    binary = epoch / (label + '-sealed')
    binary.write_bytes(payload)
    binary.chmod(0o555)
    save(epoch, label + '-artifact.json', {'path': str(binary), 'source': record['executable'], 'bytes': len(payload), 'sha256': hashlib.sha256(payload).hexdigest(), 'cargo_record': record})
    return binary

def profile_green(profile, features):
    epoch = ROOT / (profile + '-green-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    print(str(epoch), flush=True)
    try:
        require_admission(epoch, 'compile')
        before = check_source(epoch)
        save(epoch, 'source-before.json', before)
        for name, argv in [('cargo-version', ['/usr/bin/cargo', '-V']), ('rustc-version', ['/usr/bin/rustc', '-Vv'])]:
            if command(epoch, name, argv).exit_code != 0:
                raise RuntimeError('Toolchain inspection failed')
        argv = ['/usr/bin/cargo', 'test', '--test', 'integration', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12'] + features
        compiled = command(epoch, 'compile', argv)
        after = source_hashes()
        save(epoch, 'source-after.json', after)
        if compiled.exit_code != 0 or before != after:
            raise RuntimeError('Compile failed or source changed during compile')
        records = [json.loads(line) for line in compiled.stdout.splitlines() if line.strip()]
        integration = seal_record(epoch, records, 'integration', 'integration')
        normal = seal_record(epoch, records, 'wow-sim', 'normal')
        require_admission(epoch, 'runtime')
        groups = [('exact', 'friend_info_exact::', 5), ('legacy-state', 'friend_list_who::', 2), ('seeded-state', 'system_api_seeded::test_c_friend_list_returns_seeded_wow_friends', 1)]
        results = []
        for label, selector, expected in groups:
            argv = ['/usr/bin/timeout', '90', str(integration), selector, '--nocapture', '--test-threads=1']
            if label == 'seeded-state':
                argv.append('--exact')
            result = command(epoch, label, argv, {'WOW_SIM_CASC': '0', 'WOW_SIM_NO_SOUND': '1'})
            results.append({'group': label, 'expected_cases': expected, 'exit': result.exit_code})
            save(epoch, 'runtime-results.json', results)
            summary = 'test result: ok. ' + str(expected) + ' passed; 0 failed;'
            if result.exit_code != 0 or summary not in result.stdout:
                raise RuntimeError(label + ' runtime failed or unexpected case count')
        require_admission(epoch, 'normal')
        probe = (ROOT / 'normal-probe.lua').read_text()
        result = command(epoch, 'normal', ['/usr/bin/timeout', '90', str(normal), '--no-addons', '--no-saved-vars', '--exec-lua', probe, 'lua-errors'], {'WOW_SIM_NO_SOUND': '1'})
        if result.exit_code != 0 or result.stdout.strip() != '[]':
            raise RuntimeError('Normal publication/startup failed')
        save(epoch, 'outcome.json', {'status': 'EXECUTED', 'profile': profile, 'runtime_cases': 8, 'normal_exit': 0, 'source_unchanged': before == source_hashes()})
        return str(epoch)
    except Exception as error:
        save(epoch, 'failure.json', {'error': str(error), 'scope': 'Inspect actual saved results; no inferred passes'})
        raise

def final_check():
    epoch = ROOT / ('check-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    require_admission(epoch, 'check')
    before = check_source(epoch)
    save(epoch, 'source-before.json', before)
    fmt = command(epoch, 'fmt', ['/usr/bin/cargo', 'fmt', '--check'])
    checked = command(epoch, 'check', ['/usr/bin/cargo', 'check', '--offline', '--locked', '-j', '12'])
    after = source_hashes()
    save(epoch, 'source-after.json', after)
    save(epoch, 'outcome.json', {'fmt_exit': fmt.exit_code, 'check_exit': checked.exit_code, 'source_unchanged': before == after})
    if fmt.exit_code != 0 or checked.exit_code != 0 or before != after:
        raise RuntimeError('Final check failed or source changed')
    return str(epoch)

results = {}
results['retail'] = profile_green('retail', [])
results['ptr'] = profile_green('ptr', ['--no-default-features', '--features', 'sound,gui,casc,client-ptr'])
results['check'] = final_check()
save(ROOT, 'green-phases-result.json', results)
print(json.dumps(results), flush=True)
