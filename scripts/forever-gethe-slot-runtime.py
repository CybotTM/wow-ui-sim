from pathlib import Path
import datetime
import hashlib
import json
import os
import tempfile

H = Path('/home/osso/.local/state/wow-ui-sim/handoff/forever-gethe-missing')
ORIGINAL = Path('/home/osso/.local/share/wow-sim/cvars.json')
CACHE = H / 'runtime-cache-wowf'

def original_snapshot():
    fd = os.open(ORIGINAL, os.O_RDONLY | os.O_NOATIME)
    try:
        before = os.fstat(fd)
        with os.fdopen(fd, 'rb', closefd=False) as stream:
            data = stream.read()
        after = os.fstat(fd)
        keys = ('st_dev', 'st_ino', 'st_mode', 'st_uid', 'st_gid', 'st_size',
                'st_atime_ns', 'st_mtime_ns', 'st_ctime_ns')
        stat = {key: getattr(before, key) for key in keys}
        assert stat == {key: getattr(after, key) for key in keys}, 'CVar changed during read'
        return {'stat': stat, 'sha256': hashlib.sha256(data).hexdigest()}, data
    finally:
        os.close(fd)

def dump(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')

def guarded_run(label, cwd, argv):
    assert Path(cwd).is_absolute()
    assert Path(argv[0]).is_absolute()
    root = Path(tempfile.mkdtemp(prefix=label + '-', dir=H))
    unit = 'forever-gethe-' + root.name
    before, data = original_snapshot()
    dump(root / 'original-before.json', before)
    meta = {'cwd': cwd, 'requested_argv': argv, 'unit': unit,
            'started': datetime.datetime.now(datetime.timezone.utc).isoformat()}
    try:
        destination = root / 'data-home/wow-sim/cvars.json'
        destination.parent.mkdir(parents=True)
        destination.write_bytes(data)
        assert destination.read_bytes() == data
        env = {key: os.environ[key] for key in
               ('HOME', 'USER', 'LOGNAME', 'PATH', 'LANG', 'LC_ALL',
                'XDG_RUNTIME_DIR', 'DBUS_SESSION_BUS_ADDRESS', 'LD_LIBRARY_PATH')
               if key in os.environ}
        env.update(XDG_DATA_HOME=str(root / 'data-home'), XDG_CACHE_HOME=str(CACHE),
                   WOW_PRODUCT='wowf', WOW_INSTALL_PATH='/syncthing/World of Warcraft')
        meta['env'] = env
        meta['binary_sha256'] = hashlib.sha256(Path(argv[0]).read_bytes()).hexdigest()
        if Path(argv[0]).is_relative_to(H / 'sealed-tests'):
            identity = json.loads((Path(argv[0]).parent / 'identity.json').read_text())
            assert identity['binary'] == argv[0]
            assert identity['revision'] == Path(argv[0]).parent.name
            assert identity['sha256'] == meta['binary_sha256'], 'Sealed harness drift'
        command = ['/usr/bin/systemd-run', '--user', '--wait', '--remain-after-exit',
                   '--unit=' + unit, '--slice=agents.slice',
                   '--property=MemoryMax=9G', '--property=MemorySwapMax=0',
                   '--property=RestrictAddressFamilies=AF_UNIX',
                   '--property=WorkingDirectory=' + cwd,
                   '--property=StandardOutput=append:' + str(root / 'stdout'),
                   '--property=StandardError=append:' + str(root / 'stderr'),
                   '/usr/bin/env', '-i', *[key + '=' + value for key, value in env.items()],
                   '/usr/bin/timeout', '90', *argv]
        meta['launcher_argv'] = command
        result = cli.command(*command).capture().run()
        meta['launcher_exit'] = result.exit_code
        (root / 'launcher.stdout').write_text(result.stdout)
        (root / 'launcher.stderr').write_text(result.stderr)
        status = cli.command('/usr/bin/systemctl', '--user', 'show', unit,
                             '--property=LoadState,ActiveState,SubState,Result,ExecMainStatus').capture().run()
        meta['status_argv'] = ['/usr/bin/systemctl', '--user', 'show', unit,
                               '--property=LoadState,ActiveState,SubState,Result,ExecMainStatus']
        assert status.exit_code == 0, status.stderr
        (root / 'unit-status').write_text(status.stdout)
        properties = dict(line.split('=', 1) for line in status.stdout.splitlines() if '=' in line)
        assert properties['LoadState'] == 'loaded'
        assert properties['ActiveState'] in ('active', 'failed')
        assert properties['SubState'] in ('exited', 'failed')
        assert properties['Result'] in ('success', 'exit-code')
        meta['exit_code'] = int(properties['ExecMainStatus'])
        meta['stdout'] = (root / 'stdout').read_text()
        meta['stderr'] = (root / 'stderr').read_text()
    except Exception as error:
        meta['exception'] = repr(error)
        raise
    finally:
        try:
            after, _ = original_snapshot()
            dump(root / 'original-after.json', after)
            meta['original_unchanged'] = before == after
            assert before == after, 'Original CVar changed; do not reset or restore it'
        finally:
            meta['finished'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
            dump(root / 'result.json', meta)
    return meta

def run_case(label, cwd, list_argv, expected_name, run_argv, expected_exit):
    listed = guarded_run(label + '-list', cwd, list_argv)
    assert listed['exit_code'] == 0, listed
    assert [line for line in listed['stdout'].splitlines() if line] == [
        expected_name + ': test', '1 test, 0 benchmarks'], listed
    executed = guarded_run(label + '-run', cwd, run_argv)
    assert 'running 1 test' in executed['stdout'], executed
    assert executed['exit_code'] == expected_exit, executed
    if expected_exit == 101:
        assert 'nonresident Gethe source is not enabled:' in executed['stderr'], executed
    return executed
