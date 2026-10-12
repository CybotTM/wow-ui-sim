def submit_runtime(epoch):
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    admission = capacity()
    save(epoch, 'runtime-admission-' + stamp + '.json', admission)
    if not admission['admitted']:
        save(epoch, 'runtime-blocked-' + stamp + '.json', {'status': 'RESOURCE_BLOCKED', 'executed_observations': 0, 'submitted': False})
        return {'status': 'RESOURCE_BLOCKED', 'admission': admission}
    if (epoch / 'runtime-submission.json').exists():
        raise RuntimeError('Runtime already submitted; refusing duplicate')
    binary = epoch / 'probe'
    if not binary.exists():
        raise RuntimeError('Compiled probe missing')
    unit = 'wow-startup-cost-observation-' + stamp.lower()
    overrides = {'WOW_SIM_PROFILE_LUA_FILES': '1', 'WOW_SIM_CASC': '0', 'WOW_SIM_NO_SOUND': '1', 'WOW_SIM_NO_ADDONS': '1', 'WOW_SIM_NO_SAVED_VARS': '1'}
    argv = ['/usr/bin/systemd-run', '--user', '--no-block', '--unit=' + unit, '--slice=agents.slice', '--property=Type=exec', '--property=RemainAfterExit=yes', '--property=CPUQuota=1200%', '--property=MemoryMax=16G', '--property=KillMode=control-group', '--property=RuntimeMaxSec=90', '--property=TimeoutStopSec=3', '--property=WorkingDirectory=' + str(REPO), '--property=StandardOutput=append:' + str(epoch / 'runtime.stdout'), '--property=StandardError=append:' + str(epoch / 'runtime.stderr'), '/usr/bin/env']
    argv.extend(key + '=' + value for key, value in overrides.items())
    argv.append(str(binary))
    result = cli.command(*argv).capture().run()
    submission = {'argv': argv, 'exit': result.exit_code, 'stdout': result.stdout, 'stderr': result.stderr, 'unit': unit, 'environment_overrides': overrides, 'scope': 'Warm-cache CASC-disabled stock startup observation with profiling overhead; not acceptance/control replay or frozen-ba environment equality'}
    save(epoch, 'runtime-submission.json', submission)
    save(epoch, 'runtime-admission.json', admission)
    if result.exit_code != 0:
        raise RuntimeError('Startup diagnostic runtime submission failed')
    return submission
