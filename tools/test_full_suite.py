"""Bounded subprocess coverage of the detached full-suite worker."""

import fcntl
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest


RUNNER = Path(__file__).with_name("full_suite.py")
SCOPES = ("integration", "prefork_full_ui", "lib")
PROFILES = ("retail", "ptr", "wrath", "mists", "wowforever", "era", "anniversary")


def write_executable(path, body):
    path.write_text(f"#!{sys.executable}\n" + body)
    path.chmod(0o755)


def wait_for(path, process):
    deadline = time.monotonic() + 5
    while not path.exists():
        if process.poll() is not None:
            raise AssertionError(
                f"Worker exited before {path.name}: {process.communicate()}"
            )
        if time.monotonic() >= deadline:
            raise AssertionError(f"Timed out waiting for {path.name}")
        time.sleep(0.01)


class CertifiedSuiteTests(unittest.TestCase):
    @staticmethod
    def stop_worker(process):
        if process.poll() is None:
            os.killpg(process.pid, signal.SIGKILL)
        process.communicate(timeout=5)

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.home = Path(self.temp.name)
        self.bin = self.home / 'bin'
        self.bin.mkdir()
        self.repo = self.home / 'Projects/wow/wow-ui-sim'
        self.repo.mkdir(parents=True)
        self.slot = self.home / '.cache/wow-ui-sim-targets/slot-2'
        self.slot.mkdir(parents=True)
        capacity = self.home / '.worktrees/logs/capacity.sh'
        capacity.parent.mkdir(parents=True)
        write_executable(capacity, "print('agents | ADD: available')\n")
        cvars = self.home / '.local/share/wow-sim/cvars.json'
        cvars.parent.mkdir(parents=True)
        cvars.write_text('{}')
        self.env = dict(os.environ, HOME=str(self.home),
                        PATH=f"{self.bin}{os.pathsep}{os.environ['PATH']}",
                        GIT_CONFIG_NOSYSTEM='1')
        def git(*args):
            return subprocess.run(['git', *args], cwd=self.repo, env=self.env,
                                  check=True, capture_output=True, text=True).stdout.strip()
        git('init', '-q')
        git('config', 'user.name', 'Test')
        git('config', 'user.email', 'test@example.invalid')
        (self.repo / 'source').write_text('frozen source\n')
        git('add', 'source')
        git('commit', '-qm', 'fixture')
        self.sha = git('rev-parse', 'HEAD')
        git('remote', 'add', 'origin', str(self.repo))
        write_executable(self.bin / 'cargo',
                         "import sys\nsys.exit('unexpected cargo invocation')\n")
        write_executable(self.bin / 'slot-build.py',
                         "import json, os, pathlib, sys\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "(root / 'slot-argv.json').write_text(json.dumps(sys.argv[1:]))\n"
                         "print('guard rejected: owner mismatch', file=sys.stderr)\n"
                         "sys.exit(1)\n")

    def test_guard_rejection_records_failure_without_running_tests(self):
        result = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                 '--profile', 'ptr', '--target-dir', str(self.slot)],
                                env=self.env, capture_output=True, text=True, timeout=10)
        path = self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json'
        self.assertTrue(path.exists(), result.stderr)
        receipt = json.loads(path.read_text())
        self.assertIn('FAILED', json.dumps(receipt).upper())
        self.assertIn('guard', json.dumps(receipt).lower())
        self.assertFalse(any(step.get('exit') == 0 for step in receipt['steps'].values()))
        argv = json.loads((self.home / 'slot-argv.json').read_text())
        self.assertIn('--owner', argv)
        self.assertEqual(argv[argv.index('--slot') + 1], '2')
        self.assertIn('sound,gui,casc,client-ptr', argv)
        worktree = Path(argv[argv.index('--worktree') + 1])
        self.assertEqual(subprocess.check_output(['git', '-C', str(worktree), 'rev-parse', 'HEAD'],
                                                 env=self.env, text=True).strip(), self.sha)
        self.assertEqual((worktree / 'source').read_text(), 'frozen source\n')

    def test_result_waits_for_certified_build_and_test_execution(self):
        test_binary = self.slot / 'integration-test'
        write_executable(test_binary,
                         "import os, pathlib, time\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "(root / 'test-started').touch()\n"
                         "while not (root / 'test-release').exists(): time.sleep(.01)\n")
        write_executable(self.bin / 'slot-build.py',
                         "import json, os, pathlib, sys, time\n"
                         "root = pathlib.Path(os.environ['HOME'])\n"
                         "argv = sys.argv[1:]\n"
                         "(root / 'slot-argv.json').write_text(json.dumps(argv))\n"
                         "(root / 'build-started').touch()\n"
                         "while not (root / 'build-release').exists(): time.sleep(.01)\n"
                         "slot = root / '.cache/wow-ui-sim-targets/slot-2'\n"
                         "unit = argv[argv.index('--unit') + 1]\n"
                         "receipt = slot / 'receipts' / (unit + '.json')\n"
                         "receipt.parent.mkdir(exist_ok=True)\n"
                         "records = [{'executable': str(slot / 'integration-test'), "
                         "'target': {'name': n, 'kind': [k]}, 'profile': {'test': True}} "
                         "for n, k in [('integration', 'test'), ('wow_sim', 'lib')]]\n"
                         "receipt.write_text(json.dumps({'result': 'success', 'returncode': 0, "
                         "'reported_artifacts': records}))\n")
        write_executable(self.bin / 'systemctl',
                         "print('SubState=exited\\nResult=success')\n")
        process = subprocess.Popen([sys.executable, str(RUNNER), 'run', self.sha,
                                    '--profile', 'ptr', '--target-dir', str(self.slot)],
                                   env=self.env, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, text=True, start_new_session=True)
        self.addCleanup(self.stop_worker, process)
        receipt = self.home / f'Projects/wow/full-suite-results/{self.sha}-ptr.json'
        wait_for(self.home / 'build-started', process)
        self.assertFalse(receipt.exists())
        (self.home / 'build-release').touch()
        wait_for(self.home / 'test-started', process)
        self.assertFalse(receipt.exists())
        (self.home / 'test-release').touch()
        stdout, stderr = process.communicate(timeout=10)
        self.assertEqual(process.returncode, 0, stderr)
        self.assertTrue(receipt.exists())
        self.assertTrue(all(step.get('exit', 0) == 0
                            for step in json.loads(receipt.read_text())['steps'].values()))
        argv = json.loads((self.home / 'slot-argv.json').read_text())
        worktree = Path(argv[argv.index('--worktree') + 1])
        self.assertEqual((worktree / 'source').read_bytes(), b'frozen source\n')
        self.assertEqual(subprocess.check_output(['git', '-C', str(worktree), 'rev-parse', 'HEAD'],
                                                 env=self.env, text=True).strip(), self.sha)

    def test_owned_slot_is_never_stolen(self):
        owner = self.slot / '.owner'
        owner.write_text('another owner')
        result = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                 '--target-dir', str(self.slot)], env=self.env,
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(owner.read_text(), 'another owner')
        self.assertFalse((self.home / 'slot-argv.json').exists())
        self.assertIn('owner', (result.stdout + result.stderr).lower())

    def test_capacity_refusal_does_not_claim_slot(self):
        write_executable(self.home / '.worktrees/logs/capacity.sh',
                         "print('agents | HOLD: busy')\n")
        result = subprocess.run([sys.executable, str(RUNNER), 'run', self.sha,
                                 '--target-dir', str(self.slot)], env=self.env,
                                capture_output=True, text=True, timeout=10)
        self.assertFalse((self.slot / '.owner').exists())
        self.assertFalse((self.home / 'slot-argv.json').exists())
        self.assertIn('HOLD: busy', result.stdout + result.stderr)

    def test_suite_lock_blocks_the_worker_before_checkout(self):
        results = self.home / 'Projects/wow/full-suite-results'
        results.mkdir(parents=True)
        with (results / '.lock').open('w') as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            process = subprocess.Popen([sys.executable, str(RUNNER), 'run', self.sha,
                                        '--target-dir', str(self.slot)], env=self.env,
                                       stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                       text=True, start_new_session=True)
            self.addCleanup(self.stop_worker, process)
            time.sleep(.1)
            self.assertIsNone(process.poll())
            self.assertFalse((self.home / 'slot-argv.json').exists())
            self.assertFalse(list((self.home / '.worktrees').glob('wow-ui-sim-fullsuite-*')))
            fcntl.flock(lock, fcntl.LOCK_UN)
        process.communicate(timeout=10)

    def test_submit_sets_memory_limits(self):
        write_executable(self.bin / 'systemd-run',
                         "import json, os, pathlib, sys\n"
                         "(pathlib.Path(os.environ['HOME']) / 'unit.json').write_text(json.dumps(sys.argv[1:]))\n")
        result = subprocess.run([sys.executable, str(RUNNER), 'submit', self.sha,
                                 '--target-dir', str(self.slot)], env=self.env,
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        argv = json.loads((self.home / 'unit.json').read_text())
        self.assertIn('MemoryMax=9G', argv)
        self.assertIn('MemorySwapMax=0', argv)


if __name__ == "__main__":
    unittest.main()
