from pathlib import Path
import datetime, hashlib, json, random, time

REPO = Path('/home/osso/Projects/wow/wow-ui-sim')
ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/friend-info-exact-current')
REVISION = '41fb7d160'

def save(epoch, name, value):
    (epoch / name).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')

def capacity():
    memory = {line.split(':')[0]: int(line.split(':')[1].split()[0]) for line in Path('/proc/meminfo').read_text().splitlines()}
    load = float(Path('/proc/loadavg').read_text().split()[0])
    active = []
    for process in Path('/proc').iterdir():
        if not process.name.isdigit():
            continue
        try:
            if (process / 'comm').read_text().strip() == 'cargo' and (process / 'cwd').resolve() == REPO.resolve():
                active.append(process.name)
        except OSError:
            continue
    available = memory['MemAvailable'] / 1048576
    return {'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'load1': load, 'available_gib': available, 'relevant_cargo': active, 'admitted': available >= 6 and load < 24 and not active}

def admit(epoch, phase):
    observations = []
    for attempt in range(6):
        observation = capacity()
        observations.append(observation)
        save(epoch, phase + '-admission-history.json', observations)
        if observation['admitted']:
            save(epoch, phase + '-admission.json', observation)
            return True
        if attempt < 5:
            time.sleep(min(30 * 2 ** attempt, 180) + random.uniform(0, 3))
    save(epoch, phase + '-blocked.json', {'status': 'RESOURCE_BLOCKED', 'executed_cases': 0})
    return False

def command(epoch, name, argv, env=None):
    save(epoch, name + '-submission.json', {'argv': argv, 'cwd': str(REPO), 'env': env or {}})
    builder = cli.command(*argv).cwd(str(REPO))
    for key, value in (env or {}).items():
        builder = builder.env(key, value)
    result = builder.capture().run()
    (epoch / (name + '.stdout')).write_text(result.stdout)
    (epoch / (name + '.stderr')).write_text(result.stderr)
    save(epoch, name + '-result.json', {'exit': result.exit_code})
    return result

# Uses the already inspected capacity/command helpers from red-worker.py.
REVISION = '6ec0c1cb8'
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
        if profile == 'retail':
            previous = ROOT / 'retail-green-20261012T122455266641Z'
            unchanged = cli.git('diff', 'be9c734a4', '--', 'src', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
            if unchanged.exit_code != 0 or unchanged.stdout:
                raise RuntimeError('Cannot reuse normal artifact: production source changed')
            normal = previous / 'normal-sealed'
            save(epoch, 'normal-reuse.json', {'epoch': str(previous), 'manifest': 'normal-artifact.json', 'authentication': 'independent-auth-normal.json', 'source_diff': unchanged.stdout, 'reason': 'Only secret test assertions changed; no production/Cargo changes, no reseal or rehash'})
        else:
            normal = seal_record(epoch, records, 'wow-sim', 'normal')
        require_admission(epoch, 'runtime')
        exact = ('exact', 'friend_info_exact::exact_friend_lookup_authenticates_secret_name_by_caller_taint', 1) if profile == 'retail' else ('exact', 'friend_info_exact::', 5)
        groups = [exact, ('legacy-state', 'friend_list_who::', 2), ('seeded-state', 'system_api_seeded::test_c_friend_list_returns_seeded_wow_friends', 1)]
        results = []
        for label, selector, expected in groups:
            argv = ['/usr/bin/timeout', '90', str(integration), selector, '--nocapture', '--test-threads=1']
            if label == 'seeded-state' or (profile == 'retail' and label == 'exact'):
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
        save(epoch, 'outcome.json', {'status': 'EXECUTED', 'profile': profile, 'runtime_cases': 4 if profile == 'retail' else 8, 'normal_exit': 0, 'source_unchanged': before == source_hashes()})
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


COMPILED = ROOT / 'ptr-green-20261012T123249639935Z'
ORIGINAL_UNIT = 'wow-friend-info-exact-green-retry2-20261012t123241950756z'

def safe_runtime(epoch):
    status = cli.systemctl('--user', 'show', ORIGINAL_UNIT, '--property=MainPID,SubState').capture().run()
    observed = capacity()
    memory_pressure = Path('/proc/pressure/memory').read_text()
    cpu_pressure = Path('/proc/pressure/cpu').read_text()
    full = next(line for line in memory_pressure.splitlines() if line.startswith('full '))
    full_avg10 = float(next(field.split('=')[1] for field in full.split() if field.startswith('avg10=')))
    safe = status.exit_code == 0 and 'MainPID=0' in status.stdout and 'SubState=exited' in status.stdout and observed['available_gib'] >= 20 and not observed['relevant_cargo'] and full_avg10 < 5
    save(epoch, 'runtime-only-admission.json', {'original_status': status.stdout, 'capacity': observed, 'cpu_pressure': cpu_pressure, 'memory_pressure': memory_pressure, 'safe': safe, 'change': 'Original load<24 gate exhausted6/no runtime. Runtime-only nonperformance scope: omit arbitrary load gate; require20GiB available above16GiB ceiling/no relevant Cargo/memory full avg10<5; CPUQuota200%/90s bounds. Not performance acceptance.'})
    if not safe:
        raise RuntimeError('Runtime-only safety admission denied')

phase = ROOT / ('ptr-runtime-retry-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
phase.mkdir()
print(str(phase), flush=True)
safe_runtime(phase)
before = check_source(phase)
save(phase, 'source-before.json', before)
save(phase, 'artifact-reuse.json', {'compiled_epoch': str(COMPILED), 'receipts': ['independent-auth-integration.json', 'independent-auth-normal.json'], 'no_rebuild_reseal_rehash': True})
for receipt in ['independent-auth-integration.json', 'independent-auth-normal.json']:
    if not (COMPILED / receipt).exists():
        raise RuntimeError('Missing preexisting authentication receipt')
integration = COMPILED / 'integration-sealed'
results = []
for label, selector, expected in [('exact', 'friend_info_exact::', 5), ('legacy-state', 'friend_list_who::', 2), ('seeded-state', 'system_api_seeded::test_c_friend_list_returns_seeded_wow_friends', 1)]:
    argv = ['/usr/bin/timeout', '90', str(integration), selector, '--nocapture', '--test-threads=1']
    if label == 'seeded-state':
        argv.append('--exact')
    value = command(phase, label, argv, {'WOW_SIM_CASC': '0', 'WOW_SIM_NO_SOUND': '1'})
    results.append({'group': label, 'expected_cases': expected, 'exit': value.exit_code})
    save(phase, 'runtime-results.json', results)
    if value.exit_code != 0 or ('test result: ok. ' + str(expected) + ' passed; 0 failed;') not in value.stdout:
        raise RuntimeError(label + ' failed')
value = command(phase, 'normal', ['/usr/bin/timeout', '90', str(COMPILED / 'normal-sealed'), '--no-addons', '--no-saved-vars', '--exec-lua', '@' + str(ROOT / 'normal-probe.lua'), 'lua-errors'], {'WOW_SIM_NO_SOUND': '1'})
if value.exit_code != 0 or value.stdout.strip() != '[]':
    raise RuntimeError('PTR normal startup/probe failed')
after = source_hashes()
save(phase, 'source-after.json', after)
if before != after:
    raise RuntimeError('Source changed during PTR runtime')
save(phase, 'outcome.json', {'status': 'EXECUTED', 'runtime_cases': 8, 'normal_exit': 0, 'sources_unchanged': True})
check_epoch = final_check()
save(ROOT, 'runtime-retry-phases-result.json', {'ptr_runtime': str(phase), 'check': check_epoch})
print(json.dumps({'ptr_runtime': str(phase), 'check': check_epoch}), flush=True)
