ROOT = Path('/home/osso/.local/state/wow-ui-sim/verification/mount-item-current')
REVISION = 'dceb4c751'
PATHS = ['src/lua_api/state.rs', 'src/lua_api/state/sim_state.rs', 'src/c_api/mod.rs', 'src/c_api/registration.rs', 'src/client_profile.rs', 'Cargo.toml', 'Cargo.lock']
MIN_AVAILABLE = 6

def current_source(epoch, boundary):
    result = cli.git('diff', REVISION, '--', 'src', 'Cargo.toml', 'Cargo.lock', 'build.rs', 'build').cwd(str(REPO)).capture().run()
    hashes = {name: hashlib.sha256((REPO / name).read_bytes()).hexdigest() for name in PATHS}
    save(epoch, boundary + '-source.json', {'revision': REVISION, 'diff': result.stdout, 'exit': result.exit_code, 'hashes': hashes})
    if result.exit_code != 0 or result.stdout:
        raise RuntimeError('Production source changed')
    return hashes

epoch = ROOT / ('earlier-profile-check-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'))
epoch.mkdir()
print(str(epoch), flush=True)
results = []
for profile in ['wrath', 'mists', 'era', 'anniversary', 'wowforever']:
    lane = epoch / profile
    lane.mkdir()
    if not admit(lane, 'check'):
        save(epoch, 'outcome.json', {'status': 'RESOURCE_BLOCKED', 'results': results, 'no_missing_case_inference': True})
        raise RuntimeError('Earlier-profile check admission exhausted')
    before = current_source(lane, 'before')
    for name, argv in [('cargo-version', ['/usr/bin/cargo', '-V']), ('rustc-version', ['/usr/bin/rustc', '-Vv'])]:
        if command(lane, name, argv).exit_code != 0:
            raise RuntimeError('Toolchain inspection failed')
    checked = command(lane, 'check', ['/usr/bin/cargo', 'check', '--offline', '--locked', '--no-default-features', '--features', 'sound,gui,casc,client-' + profile, '-j', '12'])
    after = current_source(lane, 'after')
    results.append({'profile': profile, 'check_exit': checked.exit_code, 'source_unchanged': before == after})
    save(epoch, 'results.json', results)
    if checked.exit_code != 0 or before != after:
        save(epoch, 'outcome.json', {'status': 'CHECK_FAILED_OR_SOURCE_CHANGED', 'results': results})
        raise RuntimeError('Actual compiler failure retained')
save(epoch, 'outcome.json', {'status': 'FIVE_EARLIER_PROFILE_CHECKS_CAPTURED', 'revision': REVISION, 'results': results, 'scope': 'Shared unconditional state/default compile refresh. Retail/PTR excluded from rerun; current API cfg-excluded here. No runtime/native/full-handoff acceptance.', 'full_handoff_complete': False})
print(str(epoch), flush=True)
