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

def run_red():
    epoch = ROOT / ('retail-red-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
    epoch.mkdir()
    print(str(epoch))
    if not admit(epoch, 'compile'):
        save(epoch, 'outcome.json', {'status': 'RESOURCE_BLOCKED', 'phase': 'compile', 'executed_cases': 0})
        return
    diff = cli.git('diff', REVISION, '--', 'src', 'tests', 'Cargo.toml', 'Cargo.lock').cwd(str(REPO)).capture().run()
    head = cli.git('rev-parse', 'HEAD').cwd(str(REPO)).capture().run()
    save(epoch, 'source-applicability.json', {'source': REVISION, 'head': head.stdout.strip(), 'exit': diff.exit_code, 'diff': diff.stdout, 'stderr': diff.stderr})
    if head.exit_code != 0 or diff.exit_code != 0 or diff.stdout:
        raise RuntimeError('Relevant source changed before friend RED compilation')
    paths = ['tests/friend_info_exact.rs', 'src/lua_api/globals/missing_surface/friend_list.rs', 'Cargo.toml', 'Cargo.lock']
    before = {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in paths}
    save(epoch, 'revision.json', {'head': head.stdout.strip(), 'before': before, 'scope': 'Four exact friend-name/state/security and real whole-popup pre-producer RED cases; no producer/state-model changes'})
    for name, argv in [('cargo-version', ['/usr/bin/cargo', '-V']), ('rustc-version', ['/usr/bin/rustc', '-Vv'])]:
        if command(epoch, name, argv).exit_code != 0:
            raise RuntimeError('Toolchain inspection failed')
    result = command(epoch, 'compile', ['/usr/bin/cargo', 'test', '--test', 'integration', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12'])
    after = {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in paths}
    save(epoch, 'compile-outcome.json', {'exit': result.exit_code, 'after': after, 'sources_unchanged': before == after})
    if result.exit_code != 0:
        save(epoch, 'outcome.json', {'status': 'COMPILE_FAILED', 'executed_cases': 0})
        return
    if before != after:
        raise RuntimeError('Source changed during friend RED compilation')
    records = [json.loads(line) for line in result.stdout.splitlines() if line.strip()]
    candidates = [record for record in records if record.get('reason') == 'compiler-artifact' and record.get('target', {}).get('name') == 'integration' and record.get('executable')]
    if len(candidates) != 1:
        raise RuntimeError('Expected one integration artifact')
    record = candidates[0]
    payload = Path(record['executable']).read_bytes()
    binary = epoch / 'integration-sealed'
    binary.write_bytes(payload)
    binary.chmod(0o555)
    save(epoch, 'artifact.json', {'path': str(binary), 'source': record['executable'], 'bytes': len(payload), 'sha256': hashlib.sha256(payload).hexdigest(), 'cargo_record': record})
    del payload
    if not admit(epoch, 'runtime'):
        save(epoch, 'outcome.json', {'status': 'COMPILED_RESOURCE_BLOCKED', 'executed_cases': 0})
        return
    runtime = command(epoch, 'runtime', ['/usr/bin/timeout', '90', str(binary), 'friend_info_exact::', '--nocapture', '--test-threads=1'], {'WOW_SIM_CASC': '0', 'WOW_SIM_NO_SOUND': '1'})
    save(epoch, 'outcome.json', {'status': 'EXECUTED', 'runtime_exit': runtime.exit_code, 'scope': 'Inspect actual test summary/failure boundaries; expected RED is not inferred from exit alone'})
    print('Friend RED runtime exit:', runtime.exit_code)

run_red()
