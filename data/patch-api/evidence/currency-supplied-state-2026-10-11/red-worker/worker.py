from pathlib import Path
import datetime, hashlib, json, random, shutil, time
repo = Path('/home/osso/Projects/wow/wow-ui-sim')
root = Path('/home/osso/.local/state/wow-ui-sim/verification/currency-projection-red-current')
epoch = root / datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
epoch.mkdir()
def save(name, value):
    (epoch / name).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')
observations = []
for attempt in range(5):
    memory = {line.split(':')[0]: int(line.split(':')[1].split()[0]) for line in Path('/proc/meminfo').read_text().splitlines()}
    load = float(Path('/proc/loadavg').read_text().split()[0])
    active = []
    for p in Path('/proc').iterdir():
        if p.name.isdigit():
            try:
                if (p / 'comm').read_text().strip() == 'cargo' and (p / 'cwd').resolve() in [repo, repo.parent / 'full-suite-checkout']:
                    active.append(p.name)
            except OSError:
                continue
    available = memory['MemAvailable'] / 1048576
    admitted = available >= 6 and load < 24 and not active
    observations.append({'attempt': attempt + 1, 'available_gib': available, 'load1': load, 'relevant_cargo': active, 'admitted': admitted})
    save('admission.json', observations)
    if admitted:
        break
    if attempt < 4:
        time.sleep(min(10 * 2 ** attempt, 60) + random.uniform(0, 5))
if not admitted:
    save('outcome.json', {'status': 'RESOURCE_BLOCKED', 'executed_cases': 0})
    raise RuntimeError('Currency admission blocked')
paths = ['tests/currency_info_probes.rs', 'src/c_api/item_spell/c_currency.rs', 'src/lua_api/globals/currency_data.rs', 'src/mists/compat_bootstrap.lua', 'Cargo.toml', 'Cargo.lock', 'build.rs']
before = {name: hashlib.sha256((repo / name).read_bytes()).hexdigest() for name in paths}
revision = cli.git('rev-parse', 'HEAD').cwd(str(repo)).capture().run()
assert revision.exit_code == 0
argv = ['/usr/bin/cargo', 'test', '--offline', '--locked', '--jobs', '12', '--test', 'integration', '--no-run', '--message-format=json']
save('compile-submission.json', {'argv': argv, 'revision': revision.stdout.strip(), 'input_hashes_before': before, 'cgroup': Path('/proc/self/cgroup').read_text(), 'started': datetime.datetime.now(datetime.timezone.utc).isoformat()})
built = cli.command(*argv).cwd(str(repo)).env('CARGO_NET_OFFLINE', 'true').capture().run()
(epoch / 'compile.stdout').write_text(built.stdout)
(epoch / 'compile.stderr').write_text(built.stderr)
save('compile-result.json', {'exit': built.exit_code, 'ended': datetime.datetime.now(datetime.timezone.utc).isoformat()})
if built.exit_code != 0:
    save('outcome.json', {'status': 'COMPILE_FAILED', 'executed_cases': 0})
    raise RuntimeError('Currency regression build failed')
records = [json.loads(line) for line in built.stdout.splitlines() if line.startswith('{')]
candidates = [row for row in records if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == 'integration' and row.get('executable')]
assert len(candidates) == 1, 'Ambiguous integration artifact'
record = candidates[0]
binary = epoch / 'integration-test'
shutil.copy2(record['executable'], binary)
save('artifact.json', {'source_executable': record['executable'], 'sealed_copy': str(binary), 'sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'bytes': binary.stat().st_size, 'features': record['features'], 'build_profile': record['profile']})
argv = ['/usr/bin/timeout', '90', str(binary), 'currency_info_probes::', '--nocapture', '--test-threads=1']
value = cli.command(*argv).cwd(str(repo)).env('WOW_SIM_CASC', '0').capture().run()
(epoch / 'currency.stdout').write_text(value.stdout)
(epoch / 'currency.stderr').write_text(value.stderr)
after = {name: hashlib.sha256((repo / name).read_bytes()).hexdigest() for name in paths}
save('outcome.json', {'argv': argv, 'exit': value.exit_code, 'input_hashes_after': after, 'inputs_unchanged': before == after, 'scope': 'Default Retail supplied-state projection regression and existing currency-info controls; RED before producer change, no native/profile acceptance'})
print(str(epoch))
