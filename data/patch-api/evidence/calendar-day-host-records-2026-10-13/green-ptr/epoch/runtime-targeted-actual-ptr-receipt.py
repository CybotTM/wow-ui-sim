from pathlib import Path
import datetime, json, re

# Inputs EPOCH, RUNTIME_DIR, PROFILE, FILTER, EXPECTED_CASES, PROOF_SCOPE are provided by calling Pyrun.
R = Path('<REPO>')
E = Path(EPOCH)
G = E / RUNTIME_DIR
G.mkdir(exist_ok=False)
receipt = json.loads((E / 'receipt-integration.json').read_text())
seal = json.loads((E / 'seal-integration.json').read_text())
assert receipt['status'] == 'PASS' and receipt['sha256_matches_seal']
assert receipt['hash_reads'] == 1 and receipt['path'] == seal['sealed_path']
binary = Path(receipt['path'])
stat = binary.stat()
assert set(receipt['stat_after']) == {'dev', 'inode', 'size', 'mtime_ns'}
stat_attributes = {'dev': 'st_dev', 'inode': 'st_ino', 'size': 'st_size', 'mtime_ns': 'st_mtime_ns'}
current_stat = {key: getattr(stat, stat_attributes[key]) for key in receipt['stat_after']}
assert current_stat == receipt['stat_after'], 'Artifact changed; no launch'
assert (E / 'source/target').is_dir(), 'Missing sealed-source overlay mountpoint'
result = cli.systemctl('--user', 'show', 'agents.slice', '-p', 'ControlGroup').capture().run()
assert result.returncode == 0
cgroup = Path('/sys/fs/cgroup') / result.stdout.strip().split('=', 1)[1].lstrip('/')
raw = {name: (cgroup / name).read_text() for name in ['memory.current', 'memory.high', 'memory.max', 'memory.stat', 'memory.pressure', 'cpu.pressure', 'io.pressure']}
stats = {key: int(value) for key, value in (line.split() for line in raw['memory.stat'].splitlines())}
clean = max(0, stats['file'] - stats['file_mapped'] - stats['file_dirty'] - stats['file_writeback'] - stats.get('shmem', 0))
reclaim = (clean + stats['slab_reclaimable']) // 2
budget = 4 * 1024**3
projected = max(0, int(raw['memory.current']) - reclaim) + budget
available = int(re.search(r'MemAvailable:\s+(\d+)', Path('/proc/meminfo').read_text())[1]) * 1024
pressure = float(re.search(r'some avg10=([\d.]+)', raw['memory.pressure'])[1])
allowed = projected < min(int(raw['memory.high']), int(raw['memory.max'])) and available > budget and pressure < 5
admission = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'raw': raw, 'half_clean_file_and_slab_reclaim': reclaim, 'projected': projected, 'host_available': available, 'allowed': allowed, 'worker_max': budget, 'worker_high': 3 * 1024**3, 'profile': PROFILE}
(G / 'admission.json').write_text(json.dumps(admission, indent=2) + '\n')
print('ADMISSION', G, allowed, 'projected', projected)
if allowed:
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ').lower()
    unit = 'wow-speak-' + PROFILE + '-green-runtime-' + stamp
    argv = ['systemd-run', '--user', '--wait', '--unit=' + unit, '--slice=agents.slice', '-p', 'MemoryMax=4G', '-p', 'MemoryHigh=3G', '-p', 'CPUQuota=200%', '-p', 'KillMode=control-group', '-p', 'StandardOutput=file:' + str(G / 'stdout'), '-p', 'StandardError=file:' + str(G / 'stderr'), '--working-directory=' + str(R), '/usr/bin/timeout', '--signal=TERM', '--kill-after=2', '86', '/usr/bin/bwrap', '--die-with-parent', '--unshare-net', '--ro-bind', '/', '/', '--ro-bind', str(E / 'source'), str(R), '--ro-bind', str(R / 'target'), str(R / 'target'), '--tmpfs', '/tmp', '--dev', '/dev', '--proc', '/proc', '--chdir', str(R), '--setenv', 'HOME', '<HOME>', '--setenv', 'WOW_SIM_CASC', '0', '--setenv', 'WOW_SIM_NO_SOUND', '1', str(binary), FILTER, '--nocapture', '--test-threads=1']
    (G / 'argv.json').write_text(json.dumps(argv, indent=2) + '\n')
    result = cli.command(*argv).capture().run()
    (G / 'launcher.txt').write_text(result.stdout + result.stderr)
    stdout = (G / 'stdout').read_text() if (G / 'stdout').exists() else ''
    stderr = (G / 'stderr').read_text() if (G / 'stderr').exists() else ''
    summaries = re.findall(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', stdout)
    outcome = {'source': seal['source'], 'profile': PROFILE, 'filter': FILTER, 'expected_cases': EXPECTED_CASES, 'launcher_exit': result.returncode, 'summaries': summaries, 'stdout_bytes': len(stdout.encode()), 'stderr_bytes': len(stderr.encode()), 'receipt_stat_reused': current_stat, 'unit': unit, 'scope': PROOF_SCOPE}
    (G / 'result.json').write_text(json.dumps(outcome, indent=2) + '\n')
    print(json.dumps(outcome, indent=2))
else:
    print('Denied; zero launch. Preserve admission evidence.')
