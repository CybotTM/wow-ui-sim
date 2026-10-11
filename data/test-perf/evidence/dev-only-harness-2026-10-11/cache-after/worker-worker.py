from pathlib import Path
import datetime, hashlib, json, os, random, time
repo = Path('/home/osso/Projects/wow/wow-ui-sim')
root = Path('/home/osso/.local/state/wow-ui-sim/verification/dev-harness-cache-trial-current')
epoch = root / datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
epoch.mkdir()
def save(name, value):
    (epoch / name).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')
def admit(stage):
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
        admitted = memory['MemAvailable'] / 1048576 >= 6 and load < 24 and not active
        observations.append({'attempt': attempt + 1, 'available_gib': memory['MemAvailable'] / 1048576, 'load1': load, 'relevant_cargo': active, 'admitted': admitted})
        save(stage + '-admission.json', observations)
        if admitted:
            return True
        if attempt < 4:
            time.sleep(min(10 * 2 ** attempt, 60) + random.uniform(0, 5))
    return False
cgroup = Path('/proc/self/cgroup').read_text()
relative = next(line.split(':', 2)[2] for line in cgroup.splitlines() if line.startswith('0::'))
cpu_file = Path('/sys/fs/cgroup') / relative.lstrip('/') / 'cpu.stat'
def cpu_snapshot():
    return {line.split()[0]: int(line.split()[1]) for line in cpu_file.read_text().splitlines()}
paths = ['tests/achievements_api.rs', 'src/lua_api/globals/missing_surface/achievement_info.rs', 'Cargo.toml', 'Cargo.lock', 'build.rs', '.cargo/config.toml', 'crates/test-harness/Cargo.toml', 'crates/test-harness/build.rs', 'crates/test-harness/src/lib.rs', 'tests/integration.rs', 'tests/prefork_full_ui.rs']
before = {name: hashlib.sha256((repo / name).read_bytes()).hexdigest() for name in paths}
argv = ['/usr/bin/cargo', 'test', '--offline', '--locked', '--test', 'integration', '--no-run', '--message-format=json']
revision = cli.git('rev-parse', 'HEAD').cwd(str(repo)).capture().run()
if revision.exit_code != 0:
    raise RuntimeError('Cannot record trial revision')
save('submission.json', {'argv': argv, 'revision': revision.stdout.strip(), 'environment_overrides': {'CARGO_BUILD_JOBS': '12', 'CARGO_NET_OFFLINE': 'true', 'CARGO_LOG': 'cargo::compiler::fingerprint=info,cargo::core::compiler::fingerprint=info'}, 'source_hashes_before': before, 'cgroup': cgroup, 'cpu_stat_path': str(cpu_file), 'scope': 'Controlled default no-run build warm/timestamp trial; no test execution or source-content changes'})
def compile_stage(stage):
    if not admit(stage):
        raise RuntimeError(stage + ' admission blocked')
    cpu_before = cpu_snapshot()
    started = time.monotonic()
    value = cli.command(*argv).cwd(str(repo)).env('CARGO_BUILD_JOBS', '12').env('CARGO_NET_OFFLINE', 'true').env('CARGO_LOG', 'cargo::compiler::fingerprint=info,cargo::core::compiler::fingerprint=info').capture().run()
    wall = time.monotonic() - started
    cpu_after = cpu_snapshot()
    (epoch / (stage + '.stdout')).write_text(value.stdout)
    (epoch / (stage + '.stderr')).write_text(value.stderr)
    rows = [json.loads(line) for line in value.stdout.splitlines() if line.startswith('{')]
    artifacts = [row for row in rows if row.get('reason') == 'compiler-artifact']
    project = [row for row in artifacts if row['package_id'].startswith('path+file://' + str(repo) + '#') and row['target']['kind'] != ['custom-build']]
    outputs = {}
    build_outputs = [(row['package_id'], row['out_dir']) for row in rows if row.get('reason') == 'build-script-executed' and row['package_id'].startswith('path+file://' + str(repo))]
    for package, directory in build_outputs:
        for path in Path(directory).iterdir():
            if path.is_file() and (path.suffix in ['.rs', '.o', '.a']):
                outputs[str(path)] = {'package_id': package, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'mtime_ns': path.stat().st_mtime_ns, 'bytes': path.stat().st_size}
    helper = [{'name': row['target']['name'], 'kind': row['target']['kind'], 'fresh': row['fresh'], 'features': row['features']} for row in artifacts if row['package_id'].startswith('path+file://' + str(repo / 'crates/test-harness') + '#')]
    usage = (cpu_after['usage_usec'] - cpu_before['usage_usec']) / 1000000
    result = {'exit': value.exit_code, 'wall_seconds': wall, 'cpu_before': cpu_before, 'cpu_after': cpu_after, 'cpu_usage_seconds': usage, 'average_cpu_equivalents': usage / wall, 'artifact_records': len(artifacts), 'fresh': sum(row['fresh'] for row in artifacts), 'nonfresh': sum(not row['fresh'] for row in artifacts), 'project_targets': [{'name': row['target']['name'], 'kind': row['target']['kind'], 'fresh': row['fresh'], 'features': row['features']} for row in project], 'generated_outputs': outputs, 'helper_targets': helper, 'cpu_scope': 'Whole worker cgroup including Cargo/rustc/linker and launcher overhead; not instantaneous busy cores or per-bin CPU attribution'}
    save(stage + '-result.json', result)
    return result
# Restore default build cache after required Mists profile compilation; not a timing control.
prepare = compile_stage('prepare-default-after-mists')
if prepare['exit'] != 0:
    raise RuntimeError('Default cache preparation failed')
warm = compile_stage('warm')
if warm['exit'] != 0 or len(warm['project_targets']) != 8 or not all(row['fresh'] for row in warm['project_targets']):
    save('outcome.json', {'status': 'WARM_CONTROL_NOT_ACCEPTED', 'warm': warm, 'treatment_performed': False})
    raise RuntimeError('Warm control must have exactly eight fresh project targets; no automatic repeat')
selected = repo / 'tests/achievements_api.rs'
original_stat = selected.stat()
try:
    selected.touch()
    save('treatment.json', {'path': str(selected.relative_to(repo)), 'original_mtime_ns': original_stat.st_mtime_ns, 'treatment_mtime_ns': selected.stat().st_mtime_ns, 'source_content_unchanged_before_build': hashlib.sha256(selected.read_bytes()).hexdigest() == before['tests/achievements_api.rs']})
    treatment = compile_stage('test-mtime')
finally:
    os.utime(selected, ns=(original_stat.st_atime_ns, original_stat.st_mtime_ns))
after = {name: hashlib.sha256((repo / name).read_bytes()).hexdigest() for name in paths}
save('outcome.json', {'status': 'COMPLETED', 'warm': warm, 'treatment': treatment, 'source_bytes_unchanged': before == after, 'source_hashes_after': after, 'test_mtime_restored': selected.stat().st_mtime_ns == original_stat.st_mtime_ns, 'production_targets_stay_fresh': all(row['fresh'] for row in treatment['project_targets'] if row['kind'] in [['lib'], ['bin']]), 'scope': 'One controlled warm/timestamp pair; no changed source bytes, runtime behavior, manifest, profile, binaries deployed or full-suite execution'})
print(str(epoch))
