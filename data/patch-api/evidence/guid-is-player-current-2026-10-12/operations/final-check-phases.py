ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/guid-is-player-current')
REVISION = 'd7a34be4a'
MIN_AVAILABLE = 6
PATHS = ['src/c_api/c_player_info_guid.rs', 'src/c_api/mod.rs', 'src/c_api/registration.rs', 'tests/guid_is_player.rs', 'tests/data/patch_8_1_0_sweep_known_gaps.json', 'tools/test_patch_8_1_0_validation.py', 'data/patch-api/evidence/8.1.0-session-2026-10-08/validate.py', 'Cargo.toml', 'Cargo.lock']
CAPTURES = ['data/patch-api/evidence/guid-is-player-current-2026-10-12/current-publication-results.json', 'data/patch-api/evidence/publication-533c-reconciliation-2026-10-12/current-publication-results.json', 'data/patch-api/evidence/8.1.0-session-2026-10-08/patch_8_1_0_publication_sweep-results.json', 'data/patch-api/evidence/8.1.0-session-2026-10-08/p810-sweep-summary.json', 'data/patch-api/evidence/8.1.0-session-2026-10-08/p810-gap-review.json']

def hashes(names):
    return {name: {'bytes': (REPO / name).stat().st_size, 'sha256': hashlib.sha256((REPO / name).read_bytes()).hexdigest()} for name in names}

epoch = ROOT / ('final-check-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
epoch.mkdir()
print(str(epoch), flush=True)
if not admit(epoch, 'check'):
    raise RuntimeError('Check resource blocked; zero commands inferred')
diff = cli.git('diff', REVISION, '--', *PATHS).cwd(str(REPO)).capture().run()
save(epoch, 'source-applicability.json', {'revision': REVISION, 'diff': diff.stdout, 'exit': diff.exit_code})
if diff.exit_code != 0 or diff.stdout:
    raise RuntimeError('Assigned check source changed')
# Both packet names are flat evidence children; exclude both before scratch writes.
current = REPO / CAPTURES[0]
previous = REPO / CAPTURES[1]
for capture in (current, previous):
    if capture.is_symlink() or capture.parent.is_symlink() or not capture.is_file():
        raise RuntimeError('Capture storage is not fresh regular packet storage')
before = hashes(PATHS + CAPTURES)
save(epoch, 'source-captures-before.json', before)
results = []
for label, argv in [('default-check', ['/usr/bin/cargo', 'check', '--offline', '--locked', '-j', '12']), ('validator-tests', ['/usr/bin/python3', 'tools/test_patch_8_1_0_validation.py']), ('validator', ['/usr/bin/python3', 'data/patch-api/evidence/8.1.0-session-2026-10-08/validate.py'])]:
    result = command(epoch, label, argv, {'PYTHONDONTWRITEBYTECODE': '1'})
    results.append({'label': label, 'exit': result.exit_code, 'invocations': 1})
    save(epoch, 'command-results.json', results)
    after = hashes(PATHS + CAPTURES)
    save(epoch, label + '-source-captures-after.json', after)
    if result.exit_code != 0 or after != before:
        save(epoch, 'outcome.json', {'status': 'COMMAND_FAILED_OR_SOURCE_CAPTURE_CHANGED', 'results': results, 'unchanged': after == before})
        raise RuntimeError('Actual command failure/preservation failure retained; no automatic replay')
save(epoch, 'outcome.json', {'status': 'CHECKS_CAPTURED', 'results': results, 'source_captures_unchanged': True, 'rust_fmt': 'Reused unchanged Rust source Retail/PTR fmt proof; no redundant rerun', 'full_handoff_complete': False})
print(str(epoch), flush=True)
