from pathlib import Path
import datetime, hashlib, json, random, re, shutil, time
repo = Path('/home/osso/Projects/wow/wow-ui-sim')
root = Path('/home/osso/.local/state/wow-ui-sim/verification/currency-other-profiles-current')
epoch = root / datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
epoch.mkdir()
paths = ['tests/currency_info_probes.rs', 'tests/mists_currency_list.rs', 'src/c_api/item_spell/c_currency.rs', 'src/lua_api/globals/currency_data.rs', 'src/mists/compat_bootstrap.lua', 'Cargo.toml', 'Cargo.lock', 'build.rs']
def save(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')
before = {name: hashlib.sha256((repo / name).read_bytes()).hexdigest() for name in paths}
revision = cli.git('rev-parse', 'HEAD').cwd(str(repo)).capture().run()
assert revision.exit_code == 0
save(epoch / 'scope.json', {'revision': revision.stdout.strip(), 'inputs_before': before, 'scope': 'Five additional supported-profile builds of sixteen pure supplied-state probes; no vendor loading/acquisition/source-lane reconfirmation or native/profile startup acceptance', 'cgroup': Path('/proc/self/cgroup').read_text()})
results = []
for profile in ['ptr', 'wrath', 'era', 'anniversary', 'wowforever']:
    lane = epoch / profile
    lane.mkdir()
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
        save(lane / 'admission.json', observations)
        if admitted:
            break
        if attempt < 4:
            time.sleep(min(10 * 2 ** attempt, 60) + random.uniform(0, 5))
    if not admitted:
        results.append({'profile': profile, 'status': 'RESOURCE_BLOCKED', 'executed_cases': 0})
        break
    argv = ['/usr/bin/cargo', 'test', '--offline', '--locked', '--jobs', '12', '--no-default-features', '--features', 'sound,gui,casc,client-' + profile, '--test', 'integration', '--no-run', '--message-format=json']
    save(lane / 'compile-submission.json', {'argv': argv, 'started': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'input_hashes_before': before, 'revision': revision.stdout.strip()})
    built = cli.command(*argv).cwd(str(repo)).env('CARGO_NET_OFFLINE', 'true').capture().run()
    (lane / 'compile.stdout').write_text(built.stdout)
    (lane / 'compile.stderr').write_text(built.stderr)
    save(lane / 'compile-result.json', {'exit': built.exit_code, 'ended': datetime.datetime.now(datetime.timezone.utc).isoformat()})
    if built.exit_code != 0:
        results.append({'profile': profile, 'status': 'COMPILE_FAILED', 'executed_cases': 0})
        save(epoch / 'results.json', results)
        continue
    records = [json.loads(line) for line in built.stdout.splitlines() if line.startswith('{')]
    candidates = [row for row in records if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == 'integration' and row.get('executable')]
    assert len(candidates) == 1, profile
    record = candidates[0]
    binary = lane / 'integration-test'
    shutil.copy2(record['executable'], binary)
    save(lane / 'artifact.json', {'source_executable': record['executable'], 'sealed_copy': str(binary), 'sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'bytes': binary.stat().st_size, 'features': record['features'], 'build_profile': record['profile']})
    argv = ['/usr/bin/timeout', '90', str(binary), 'currency_info_probes::', '--nocapture', '--test-threads=1']
    value = cli.command(*argv).cwd(str(repo)).env('WOW_SIM_CASC', '0').capture().run()
    (lane / 'currency.stdout').write_text(value.stdout)
    (lane / 'currency.stderr').write_text(value.stderr)
    summary = re.search(r'test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', value.stdout)
    named = re.findall(r'^test (currency_info_probes::\S+) \.\.\. (ok|FAILED)$', value.stdout, re.M)
    after = {name: hashlib.sha256((repo / name).read_bytes()).hexdigest() for name in paths}
    result = {'profile': profile, 'argv': argv, 'exit': value.exit_code, 'summary': summary.groups() if summary else None, 'named_results': named, 'input_hashes_after': after, 'inputs_unchanged': before == after, 'exact_sixteen_passed': value.exit_code == 0 and summary is not None and summary.groups() == ('ok', '16', '0', '0') and len(named) == 16}
    results.append(result)
    save(lane / 'execution.json', result)
    save(epoch / 'results.json', results)
after = {name: hashlib.sha256((repo / name).read_bytes()).hexdigest() for name in paths}
save(epoch / 'outcome.json', {'results': results, 'inputs_unchanged': before == after, 'input_hashes_after': after, 'no_vendor_source_lane_or_native_acceptance': True})
print(str(epoch))
