from pathlib import Path
import datetime, hashlib, json, os, random, shutil, time
repo = Path('/home/osso/Projects/wow/wow-ui-sim')
root = Path('/home/osso/.local/state/wow-ui-sim/verification/dev-harness-green-current')
epoch = root / datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
epoch.mkdir()
def save(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')
def admit(path):
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
        save(path, observations)
        if admitted:
            return True
        if attempt < 4:
            time.sleep(min(10 * 2 ** attempt, 60) + random.uniform(0, 5))
    return False
reserved = ['PREFORK_CONFORMANCE_SUITE', 'PREFORK_CONFORMANCE_DRIVER', 'PREFORK_CONFORMANCE_TREE_CHILD', 'PREFORK_EXACT_FIXTURE_GROUP']
if any(name in os.environ for name in reserved):
    raise RuntimeError('Reserved prefork environment already set; no silent normalization')
if not admit(epoch / 'baseline-admission.json'):
    raise RuntimeError('Baseline listing admission blocked')
old_integrations = {
    'retail': Path('/home/osso/.local/state/wow-ui-sim/verification/achievement-reward-projection-current/20261011T044347Z/integration-test'),
    'mists': Path('/home/osso/.local/state/wow-ui-sim/verification/unit2-green-current/20261010T235452Z/mists/integration-test'),
}
old_preforks = {'retail': 'd84a52f78dca94c5', 'mists': 'e8b005486f5873d6'}
for profile in ['retail', 'mists']:
    folder = epoch / profile
    folder.mkdir()
    fingerprint = repo / 'target/debug/.fingerprint' / ('wow-ui-sim-' + old_preforks[profile]) / 'test-integration-test-prefork_full_ui.json'
    (folder / 'old-prefork-fingerprint.json').write_bytes(fingerprint.read_bytes())
    old_prefork = folder / 'old-prefork'
    shutil.copy2(repo / 'target/debug/deps' / ('prefork_full_ui-' + old_preforks[profile]), old_prefork)
    save(folder / 'old-prefork-artifact.json', {'source': str(repo / 'target/debug/deps' / ('prefork_full_ui-' + old_preforks[profile])), 'copy': str(old_prefork), 'sha256': hashlib.sha256(old_prefork.read_bytes()).hexdigest(), 'bytes': old_prefork.stat().st_size, 'scope': 'Previously unsealed local binary first copied/sealed for listing; Cargo fingerprint feature record, not whole-source/native provenance'})
    for kind, binary in [('integration', old_integrations[profile]), ('prefork', old_prefork)]:
        argv = ['/usr/bin/timeout', '90', str(binary), '--list']
        result = cli.command(*argv).cwd(str(repo)).env('WOW_SIM_CASC', '0').capture().run()
        (folder / ('old-' + kind + '.stdout')).write_text(result.stdout)
        (folder / ('old-' + kind + '.stderr')).write_text(result.stderr)
        save(folder / ('old-' + kind + '.json'), {'argv': argv, 'exit': result.exit_code})
        if result.exit_code != 0:
            raise RuntimeError('Old ' + profile + ' ' + kind + ' listing failed')
inputs = ['Cargo.toml', 'Cargo.lock', 'build.rs', 'crates/test-harness/Cargo.toml', 'crates/test-harness/build.rs', 'crates/test-harness/src/lib.rs', 'tests/integration.rs', 'tests/prefork_full_ui.rs', 'tests/achievements_api.rs']
before = {name: hashlib.sha256((repo / name).read_bytes()).hexdigest() for name in inputs}
revision = cli.git('rev-parse', 'HEAD').cwd(str(repo)).capture().run()
if revision.exit_code != 0:
    raise RuntimeError('Cannot record harness revision')
for profile, features in [('retail', []), ('mists', ['--no-default-features', '--features', 'sound,gui,casc,client-mists'])]:
    folder = epoch / profile
    if not admit(folder / 'admission.json'):
        save(folder / 'outcome.json', {'status': 'RESOURCE_BLOCKED', 'executed_cases': 0})
        continue
    argv = ['/usr/bin/cargo', 'test', '--offline', '--locked', '--test', 'integration', '--test', 'prefork_full_ui', '--no-run', '--message-format=json'] + features
    save(folder / 'compile-submission.json', {'argv': argv, 'revision': revision.stdout.strip(), 'input_hashes_before': before, 'environment_overrides': {'CARGO_BUILD_JOBS': '12', 'CARGO_NET_OFFLINE': 'true'}, 'cgroup': Path('/proc/self/cgroup').read_text(), 'started': datetime.datetime.now(datetime.timezone.utc).isoformat()})
    built = cli.command(*argv).cwd(str(repo)).env('CARGO_BUILD_JOBS', '12').env('CARGO_NET_OFFLINE', 'true').capture().run()
    (folder / 'compile.stdout').write_text(built.stdout)
    (folder / 'compile.stderr').write_text(built.stderr)
    save(folder / 'compile-result.json', {'exit': built.exit_code, 'ended': datetime.datetime.now(datetime.timezone.utc).isoformat()})
    if built.exit_code != 0:
        save(folder / 'outcome.json', {'status': 'COMPILE_FAILED', 'executed_cases': 0})
        continue
    rows = [json.loads(line) for line in built.stdout.splitlines() if line.startswith('{')]
    artifacts = {}
    for kind, target in [('integration', 'integration'), ('prefork', 'prefork_full_ui')]:
        matches = [row for row in rows if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == target and row.get('executable')]
        if len(matches) != 1:
            raise RuntimeError('Ambiguous ' + profile + ' ' + target)
        record = matches[0]
        binary = folder / ('new-' + kind)
        shutil.copy2(record['executable'], binary)
        artifacts[kind] = {'source_executable': record['executable'], 'sealed_copy': str(binary), 'sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'bytes': binary.stat().st_size, 'features': record['features'], 'build_profile': record['profile']}
        argv = ['/usr/bin/timeout', '90', str(binary), '--list']
        result = cli.command(*argv).cwd(str(repo)).env('WOW_SIM_CASC', '0').capture().run()
        (folder / ('new-' + kind + '.stdout')).write_text(result.stdout)
        (folder / ('new-' + kind + '.stderr')).write_text(result.stderr)
        save(folder / ('new-' + kind + '.json'), {'argv': argv, 'exit': result.exit_code})
        if result.exit_code != 0:
            raise RuntimeError('New listing failed')
    save(folder / 'artifacts.json', artifacts)
    def names(file):
        return {line[:-6] for line in file.read_text().splitlines() if line.endswith(': test')}
    parity = {}
    for kind in ['integration', 'prefork']:
        old = names(folder / ('old-' + kind + '.stdout'))
        new = names(folder / ('new-' + kind + '.stdout'))
        expected_addition = {'achievements_api::test_reward_item_id_tracks_supplied_achievement_state'} if profile == 'mists' and kind == 'integration' else set()
        parity[kind] = {'old_count': len(old), 'new_count': len(new), 'removed': sorted(old - new), 'added': sorted(new - old), 'known_prior_test_addition': sorted(expected_addition), 'matches': new == old | expected_addition, 'scope': 'Mists baseline predates069f1cfbd; that single previously added testcase is explicit, not harness drift normalization' if expected_addition else 'Exact old/new registry set parity'}
    save(folder / 'parity.json', parity)
    selectors = [
        ('integration', 'achievements_api::test_reward_item_id_tracks_supplied_achievement_state', False),
        ('prefork', 'conformance::filtering_and_listing', True),
        ('prefork', 'conformance::exact_fixture_groups_list_each_case_once', True),
        ('prefork', 'conformance::read_only_bytecode_cache_child', True),
    ]
    if profile == 'retail':
        selectors.extend([
            ('integration', 'method_diff_coverage::diff_methods_extra_snapshot_matches_current_metatable_surface', False),
            ('integration', 'method_diff_coverage::diff_methods_missing_snapshot_matches_current_metatable_surface', False),
            ('prefork', 'conformance::generated_registry_lists_nested_marker_case', True),
            ('prefork', 'conformance::generated_registry_lists_path_declared_marker_case', True),
        ])
    else:
        selectors.append(('prefork', 'conformance::mists_registry_lists_only_original_fixtures', True))
    results = []
    for index, (kind, selector, conformance) in enumerate(selectors):
        argv = ['/usr/bin/timeout', '90', str(folder / ('new-' + kind)), selector, '--exact', '--nocapture', '--test-threads=1']
        command = cli.command(*argv).cwd(str(repo)).env('WOW_SIM_CASC', '0')
        if conformance:
            command = command.env('PREFORK_CONFORMANCE_SUITE', '1')
        result = command.capture().run()
        (folder / (str(index) + '.stdout')).write_text(result.stdout)
        (folder / (str(index) + '.stderr')).write_text(result.stderr)
        results.append({'argv': argv, 'selector': selector, 'kind': kind, 'conformance': conformance, 'exit': result.exit_code, 'named_case_passed': 'test ' + selector + ' ... ok' in result.stdout})
        save(folder / 'execution-results.json', results)
    save(folder / 'outcome.json', {'parity': parity, 'execution_results': results, 'scope': 'Selected default and explicit Mists only; original Mists Lua/UI fixtures not normalized or reclassified'})
checks = [['/usr/bin/cargo', 'fmt', '--all', '--', '--check'], ['/usr/bin/cargo', 'check', '--offline', '--locked', '--tests']]
for index, argv in enumerate(checks):
    if not admit(epoch / ('check-' + str(index) + '-admission.json')):
        raise RuntimeError('Final check admission blocked')
    result = cli.command(*argv).cwd(str(repo)).env('CARGO_BUILD_JOBS', '12').env('CARGO_NET_OFFLINE', 'true').capture().run()
    (epoch / ('check-' + str(index) + '.stdout')).write_text(result.stdout)
    (epoch / ('check-' + str(index) + '.stderr')).write_text(result.stderr)
    save(epoch / ('check-' + str(index) + '.json'), {'argv': argv, 'exit': result.exit_code})
after = {name: hashlib.sha256((repo / name).read_bytes()).hexdigest() for name in inputs}
save(epoch / 'outcome.json', {'source_hashes_after': after, 'inputs_unchanged': before == after, 'scope': 'Nine source/config inputs only; not whole-dependency/native/full-suite proof'})
print(str(epoch))
