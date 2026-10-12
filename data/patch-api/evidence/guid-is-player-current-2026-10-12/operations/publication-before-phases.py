ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/guid-is-player-current')
REVISION = 'd6d8de29a'
MIN_AVAILABLE = 6
PATHS = ['tests/data/patch_8_1_0_sweep_known_gaps.json', 'tests/patch_8_1_0_publication_sweep.rs', 'tests/common/publication_sweep.rs', 'tests/common/publication_probe.rs', 'tests/prefork_full_ui.rs', 'tests/common/prefork_full_ui_preload.rs', 'src/c_api/c_player_info_guid.rs', 'src/c_api/mod.rs', 'src/c_api/registration.rs', 'src/lua_api/state.rs', 'src/lua_api/state/sim_state.rs', 'Cargo.toml', 'Cargo.lock']

def snapshot(epoch, boundary):
    hashes = {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in PATHS}
    diff = cli.git('diff', REVISION, '--', 'src', 'tests', 'Cargo.toml', 'Cargo.lock', 'build.rs', 'build', 'crates/test-harness').cwd(str(REPO)).capture().run()
    save(epoch, boundary + '-source.json', {'revision': REVISION, 'hashes': hashes, 'diff': diff.stdout, 'exit': diff.exit_code, 'scope': 'Compiler/fixture input scope; independent Python historical validator edits excluded'})
    if diff.exit_code != 0 or diff.stdout:
        raise RuntimeError('Compiler/fixture source changed')
    return hashes

epoch = ROOT / ('publication-before-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
epoch.mkdir()
print(str(epoch), flush=True)
if not admit(epoch, 'compile'):
    raise RuntimeError('Compile capacity exhausted')
before = snapshot(epoch, 'before')
for name, argv in [('cargo-version', ['/usr/bin/cargo', '-V']), ('rustc-version', ['/usr/bin/rustc', '-Vv'])]:
    if command(epoch, name, argv).exit_code != 0:
        raise RuntimeError('Toolchain inspection failed')
compiled = command(epoch, 'compile', ['/usr/bin/cargo', 'test', '--test', 'prefork_full_ui', '--no-run', '--message-format=json', '--offline', '--locked', '-j', '12'])
after = snapshot(epoch, 'after-compile')
if compiled.exit_code != 0 or before != after:
    save(epoch, 'outcome.json', {'status': 'COMPILE_FAILED_OR_SOURCE_CHANGED', 'compile_exit': compiled.exit_code})
    raise RuntimeError('Compile failed or source changed')
records = [json.loads(line) for line in compiled.stdout.splitlines() if line.strip()]
selected = [row for row in records if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == 'prefork_full_ui' and row.get('executable')]
if len(selected) != 1:
    raise RuntimeError('Expected one prefork executable')
record = selected[0]
payload = Path(record['executable']).read_bytes()
binary = epoch / 'prefork-sealed'
binary.write_bytes(payload)
binary.chmod(0o555)
save(epoch, 'prefork-artifact.json', {'path': str(binary), 'source': record['executable'], 'bytes': len(payload), 'sha256': hashlib.sha256(payload).hexdigest(), 'cargo_record': record})
MIN_AVAILABLE = 20
if not admit(epoch, 'runtime'):
    raise RuntimeError('Runtime capacity exhausted')
import os
keys = ['WOW_SIM_CASC', 'WOW_SIM_NO_SOUND', 'WOW_SIM_NO_ADDONS', 'WOW_SIM_NO_SAVED_VARS', 'WOW_SIM_TIMEOUT_FIXTURE_CAPACITY', 'PREFORK_CONFORMANCE_SUITE', 'PREFORK_EXACT_FIXTURE_GROUP', 'WOW_SIM_ENABLE_BYTECODE_CACHE', 'WOW_SIM_DISABLE_BYTECODE_CACHE', 'ASSET_RESOLVER_CACHE_DIR', 'WOW_SIM_VERBOSE', 'XDG_CACHE_HOME']
output = epoch / 'current-publication-results.json'
save(epoch, 'input-context.json', {'inherited_allowlist': {key: os.environ.get(key) for key in keys}, 'overrides': {'P810_SWEEP_OUT': str(output)}, 'scope': 'Current original parent-preloaded prefork boundary; no mocked providers or historical environmental attestation'})
result = command(epoch, 'runtime', ['/usr/bin/timeout', '90', str(binary), 'patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep', '--exact', '--nocapture', '--test-threads=1'], {'P810_SWEEP_OUT': str(output)})
final = snapshot(epoch, 'after-runtime')
rows = json.loads(output.read_text()) if output.exists() else {}
known = set(json.loads((REPO / 'tests/data/patch_8_1_0_sweep_known_gaps.json').read_text()))
gaps = {name for name, row in rows.items() if not row['ok']}
resolved = ['wt-global-api-C_PlayerInfo.GUIDIsPlayer-83']
save(epoch, 'outcome.json', {'status': 'CURRENT_SWEEP_CAPTURED', 'revision': REVISION, 'runtime_exit': result.exit_code, 'rows': len(rows), 'ok': len(rows) - len(gaps), 'gaps': len(gaps), 'expected_gaps': len(known), 'gap_set_equal': gaps == known, 'resolved_rows': {name: rows.get(name) for name in resolved}, 'source_unchanged': before == final, 'scope': 'One original prefork sweep; old533c full-suiteFAIL retained, no fresh broad-suitePASS/native claim', 'full_handoff_complete': False})
if result.exit_code != 101 or len(rows) != 145 or len(gaps) != 51 or known - gaps != set(resolved) or gaps - known or any(not rows.get(name, {}).get('ok') for name in resolved) or before != final:
    raise RuntimeError('Initial expected stale-GUID observation not confirmed; output retained')
print(str(epoch), flush=True)
