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


class FullSuiteWorkerTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.checkout = self.root / "Projects/wow/full-suite-checkout"
        (self.checkout / ".git").mkdir(parents=True)
        (self.root / "Projects/wow/wow-ui-sim").mkdir()
        self.results = self.root / "Projects/wow/full-suite-results"
        self.results.mkdir()
        self.target_dir = self.root / ".cache/wow-ui-sim-targets/slot-2"
        self.target_dir.mkdir(parents=True)
        self.wrapper = self.bin / "build-lock"
        self.lock_path = self.root / "builder.lock"
        write_executable(
            self.bin / "git",
            "import json, os, pathlib, sys\n"
            "root = pathlib.Path(os.environ['HOME'])\n"
            "with (root / (os.environ['WORKER'] + '.git')).open('a') as log:\n"
            "    log.write(json.dumps({'argv': sys.argv[1:], 'cwd': os.getcwd()}) + '\\n')\n"
            "if sys.argv[1] == 'rev-parse': print('sandbox-sha')\n",
        )
        write_executable(
            self.wrapper,
            "import fcntl, os, pathlib, subprocess, sys\n"
            "root = pathlib.Path(os.environ['HOME'])\n"
            "with (root / 'builder.lock').open('w') as lock:\n"
            "    fcntl.flock(lock, fcntl.LOCK_EX)\n"
            "    sys.exit(subprocess.call(sys.argv[1:]))\n",
        )
        write_executable(
            self.bin / "cargo",
            "import json, os, pathlib, sys, time\n"
            "root = pathlib.Path(os.environ['HOME'])\n"
            "name = 'lib' if '--lib' in sys.argv else sys.argv[sys.argv.index('--test') + 1]\n"
            "prefix = os.environ['WORKER'] + '.' + name\n"
            "(root / (prefix + '.started')).write_text(json.dumps({\n"
            "    'argv': sys.argv[1:], 'jobs': os.environ.get('CARGO_BUILD_JOBS'),\n"
            "    'cwd': os.getcwd(), 'target_dir': os.environ.get('CARGO_TARGET_DIR'),\n"
            "    'fixture_capacity': os.environ.get('WOW_SIM_TIMEOUT_FIXTURE_CAPACITY')}))\n"
            "deadline = time.monotonic() + 5\n"
            "while not (root / (prefix + '.release')).exists():\n"
            "    if time.monotonic() > deadline: sys.exit(98)\n"
            "    time.sleep(0.01)\n"
            "print(name + ' stdout retained')\n"
            "print(name + ' stderr retained', file=sys.stderr)\n"
            "if os.environ['FAIL_SUITES'] == '1':\n"
            "    if name == 'prefork_full_ui':\n"
            "        print('test sandbox::prefork_failure ... FAILED')\n"
            "        sys.exit(7)\n"
            "    print('FAIL [0.01s] integration sandbox::' + name + '_failure', file=sys.stderr)\n"
            "    sys.exit(8)\n",
        )
        self.harness = self.root / "worker.py"
        self.harness.write_text(
            "import os, pathlib, runpy, sys\n"
            f"worker = runpy.run_path({str(RUNNER)!r})\n"
            # Sandbox the obsolete dependency if a regression reintroduces it.
            "worker['run'].__globals__['BUILD_LOCK'] = pathlib.Path(sys.argv[1])\n"
            "(pathlib.Path(os.environ['HOME']) / (os.environ['WORKER'] + '.ready')).touch()\n"
            f"worker['run']('sandbox-ref', 2, {str(self.target_dir)!r})\n"
        )
        self.env = dict(
            os.environ,
            HOME=str(self.root),
            PATH=f"{self.bin}{os.pathsep}{os.environ['PATH']}",
            FULL_SUITE_JOBS="3",
            CARGO_BUILD_JOBS="29",
            CARGO_TARGET_DIR=str(self.root / "unapproved-inherited-target"),
            FAIL_SUITES="0",
        )

    def start_worker(self, name="first"):
        process = subprocess.Popen(
            [sys.executable, str(self.harness), str(self.wrapper)],
            env=dict(self.env, WORKER=name),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            start_new_session=True,
        )
        self.addCleanup(self.stop_worker, process)
        return process

    @staticmethod
    def stop_worker(process):
        if process.poll() is None:
            os.killpg(process.pid, signal.SIGKILL)
        process.communicate(timeout=5)

    def assert_controller_locked(self):
        with (self.results / ".lock").open("w") as lock:
            with self.assertRaises(BlockingIOError):
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)

    def finish_scopes(self, process, worker="first"):
        expected_argv = {
            "integration": [
                "nextest",
                "run",
                "-j2",
                "--test",
                "integration",
                "--no-fail-fast",
                "--test-threads",
                "3",
                "--offline",
                "--locked",
            ],
            "prefork_full_ui": [
                "test",
                "-j2",
                "--test",
                "prefork_full_ui",
                "--offline",
                "--locked",
            ],
            "lib": [
                "nextest",
                "run",
                "-j2",
                "--lib",
                "--no-fail-fast",
                "--test-threads",
                "3",
                "--offline",
                "--locked",
            ],
        }
        for name in SCOPES:
            started = self.root / f"{worker}.{name}.started"
            wait_for(started, process)
            self.assert_controller_locked()
            record = json.loads(started.read_text())
            self.assertEqual(record["argv"], expected_argv[name])
            self.assertEqual(record["jobs"], "2")
            self.assertEqual(record["target_dir"], str(self.target_dir))
            self.assertEqual(record["cwd"], str(self.checkout))
            (self.root / f"{worker}.{name}.release").touch()
        stdout, stderr = process.communicate(timeout=5)
        self.assertEqual(process.returncode, 0, stderr)
        return stdout

    def exercise_worker(self, fail_suites):
        self.env["FAIL_SUITES"] = str(int(fail_suites))
        with self.lock_path.open("w") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            process = self.start_worker()
            # Real fake-Cargo children must run before the old builder lock releases.
            stdout = self.finish_scopes(process)
            with self.lock_path.open("w") as contender:
                with self.assertRaises(BlockingIOError):
                    fcntl.flock(contender, fcntl.LOCK_EX | fcntl.LOCK_NB)
        with (self.results / ".lock").open("w") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        result = json.loads((self.results / "sandbox-sha.json").read_text())
        expected_exits = {"integration": 8, "prefork": 7, "lib": 8}
        self.assertEqual(set(result["steps"]), set(expected_exits))
        for step, failure_exit in expected_exits.items():
            self.assertEqual(
                result["steps"][step]["exit"], failure_exit if fail_suites else 0
            )
            self.assertEqual(
                result["failures"][step],
                [f"sandbox::{step}_failure"] if fail_suites else [],
            )
        self.assertEqual(json.loads(stdout)["steps"], result["steps"])
        log = (self.results / "sandbox-sha.log").read_text()
        for name in SCOPES:
            self.assertIn(f"{name} stdout retained", log)
            self.assertIn(f"{name} stderr retained", log)
        if fail_suites:
            self.assertIn("exit 7", log)
            self.assertIn("exit 8", log)
            self.assertIn("test sandbox::prefork_failure ... FAILED", log)

    def assert_step_metadata(self, fixture_capacity):
        self.env.pop("WOW_SIM_TIMEOUT_FIXTURE_CAPACITY", None)
        if fixture_capacity is not None:
            self.env["WOW_SIM_TIMEOUT_FIXTURE_CAPACITY"] = fixture_capacity
        process = self.start_worker()
        stdout = self.finish_scopes(process)
        result = json.loads((self.results / "sandbox-sha.json").read_text())
        steps = (
            ("integration", "integration"),
            ("prefork", "prefork_full_ui"),
            ("lib", "lib"),
        )
        for step, child in steps:
            actual = json.loads((self.root / f"first.{child}.started").read_text())
            self.assertIn("command", result["steps"][step])
            self.assertEqual(
                result["steps"][step]["command"], ["cargo", *actual["argv"]]
            )
            self.assertEqual(
                result["steps"][step]["environment"],
                {"WOW_SIM_TIMEOUT_FIXTURE_CAPACITY": actual["fixture_capacity"]},
            )
            self.assertEqual(actual["fixture_capacity"], fixture_capacity)
        self.assertEqual(json.loads(stdout)["steps"], result["steps"])

    def test_step_metadata_records_unset_reservation_as_null(self):
        self.assert_step_metadata(None)

    def test_step_metadata_records_reserved_value_without_changing_children(self):
        self.assert_step_metadata("reserved")

    def test_step_metadata_preserves_empty_reservation_distinct_from_unset(self):
        self.assert_step_metadata("")

    def test_direct_children_run_while_old_builder_lock_held(self):
        self.exercise_worker(False)

    def test_all_failures_and_both_streams_retained_without_builder_lock(self):
        self.exercise_worker(True)

    def test_controller_lock_serializes_checkout_and_children(self):
        with (self.results / ".lock").open("w") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            first = self.start_worker()
            wait_for(self.root / "first.ready", first)
            time.sleep(0.1)
            self.assertIsNone(first.poll())
            self.assertFalse((self.root / "first.git").exists())
            fcntl.flock(lock, fcntl.LOCK_UN)
        wait_for(self.root / "first.integration.started", first)
        second = self.start_worker("second")
        wait_for(self.root / "second.ready", second)
        time.sleep(0.1)
        self.assertIsNone(second.poll())
        self.assertFalse((self.root / "second.git").exists())
        self.assertFalse((self.root / "second.integration.started").exists())
        self.finish_scopes(first)
        self.finish_scopes(second, "second")
        for worker in ("first", "second"):
            records = [
                json.loads(line)
                for line in (self.root / f"{worker}.git").read_text().splitlines()
            ]
            checkout_commands = [
                r["argv"][0] for r in records if r["cwd"] == str(self.checkout)
            ]
            self.assertEqual(checkout_commands, ["fetch", "checkout", "clean"])

    def test_missing_old_wrapper_does_not_prevent_cargo_execution(self):
        self.wrapper.unlink()
        process = self.start_worker()
        self.finish_scopes(process)

    def test_submit_spawns_detached_worker_with_native_resource_limits(self):
        record_path = self.root / "submitted.json"
        write_executable(
            self.bin / "systemd-run",
            "import json, os, pathlib, sys\n"
            "root = pathlib.Path(os.environ['HOME'])\n"
            "(root / 'submitted.json').write_text(json.dumps(sys.argv[1:]))\n",
        )
        process = subprocess.Popen(
            [
                sys.executable, str(RUNNER), "submit", "sandbox-ref",
                "--build-jobs", "2", "--target-dir", str(self.target_dir),
            ],
            env=self.env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        try:
            stdout, stderr = process.communicate(timeout=5)
        finally:
            if process.poll() is None:
                process.kill()
                process.communicate(timeout=5)
        self.assertEqual(process.returncode, 0, stderr)
        argv = json.loads(record_path.read_text())
        unit = stdout.strip()
        self.assertTrue(unit.startswith("full-suite-"))
        self.assertEqual(
            argv,
            [
                "--user",
                f"--unit={unit}",
                "--collect",
                "--slice=agents.slice",
                "-p",
                "CPUQuota=1200%",
                "-p",
                "MemoryHigh=16G",
                "-p",
                "MemoryMax=16G",
                "--setenv=FULL_SUITE_JOBS=3",
                sys.executable,
                str(RUNNER),
                "run",
                "sandbox-ref",
                "--build-jobs",
                "2",
                "--target-dir",
                str(self.target_dir),
            ],
        )
        self.assertFalse(list(self.root.glob("*.started")))
        self.assertFalse(list(self.root.glob("*.git")))

    def invoke_controller(self, action, *options):
        return subprocess.run(
            [sys.executable, str(RUNNER), action, "sandbox-ref", *options],
            env=dict(self.env, WORKER="validation"),
            capture_output=True,
            text=True,
            timeout=5,
        )

    def test_submit_accepts_all_four_slots_and_defaults_build_jobs_to_one(self):
        write_executable(
            self.bin / "systemd-run",
            "import json, os, pathlib, sys\n"
            "(pathlib.Path(os.environ['HOME']) / 'submitted.json').write_text(\n"
            "    json.dumps(sys.argv[1:]))\n",
        )
        self.env.pop("FULL_SUITE_JOBS", None)
        for slot in range(4):
            target = self.target_dir.with_name(f"slot-{slot}")
            with self.subTest(slot=slot):
                result = self.invoke_controller("submit", "--target-dir", str(target))
                self.assertEqual(result.returncode, 0, result.stderr)
                argv = json.loads((self.root / "submitted.json").read_text())
                self.assertEqual(argv[-4:], ["--build-jobs", "1", "--target-dir", str(target)])
                self.assertIn("--setenv=FULL_SUITE_JOBS=16", argv)

    def test_invalid_options_rejected_before_launch_or_checkout(self):
        write_executable(
            self.bin / "systemd-run",
            "import os, pathlib\n"
            "(pathlib.Path(os.environ['HOME']) / 'unexpected-service').touch()\n",
        )
        targets = [
            self.root / "Projects/wow/wow-ui-sim/target",
            self.checkout / "target",
            Path("/home/osso/Projects/wow/wow-ui-sim/target"),
            self.target_dir.with_name("slot-4"),
            self.target_dir / "nested",
            self.root / "other-target",
            Path("relative-target"),
        ]
        cases = [([], "--target-dir")]
        cases.extend((["--target-dir", str(target)], "target") for target in targets)
        cases.extend(
            (["--target-dir", str(self.target_dir), "--build-jobs", jobs], "--build-jobs")
            for jobs in ("0", "-1", "invalid")
        )
        for action in ("submit", "run"):
            for options, diagnostic in cases:
                with self.subTest(action=action, options=options):
                    result = self.invoke_controller(action, *options)
                    self.assertNotEqual(result.returncode, 0)
                    self.assertIn(diagnostic, result.stderr)
        self.assertFalse((self.root / "unexpected-service").exists())
        self.assertFalse((self.root / "validation.git").exists())
        self.assertFalse((self.results / ".lock").exists())

    def test_slot_symlinks_cannot_redirect_to_checkout_targets_or_outside_slots(self):
        targets = [
            self.root / "Projects/wow/wow-ui-sim/target",
            self.checkout / "target",
            self.root / "outside-slots",
        ]
        link = self.target_dir.with_name("slot-0")
        for target in targets:
            link.symlink_to(target, target_is_directory=True)
            try:
                for action in ("submit", "run"):
                    with self.subTest(action=action, target=target):
                        result = self.invoke_controller(action, "--target-dir", str(link))
                        self.assertNotEqual(result.returncode, 0)
                        self.assertIn("target", result.stderr)
            finally:
                link.unlink()
        self.assertFalse((self.root / "validation.git").exists())
        self.assertFalse((self.results / ".lock").exists())

    def test_deploy_installs_only_full_suite_in_home_bin(self):
        result = subprocess.run(
            [sys.executable, str(RUNNER.parent.parent / "deploy.sh")],
            cwd=self.root,
            env=self.env,
            capture_output=True,
            text=True,
            timeout=5,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        installed = self.root / "bin/full-suite"
        self.assertEqual(installed.read_bytes(), RUNNER.read_bytes())
        self.assertEqual(installed.stat().st_mode & 0o777, 0o755)
        self.assertFalse(list(self.root.glob("*.started")))
        self.assertFalse(list(self.root.glob("*.git")))


if __name__ == "__main__":
    unittest.main()
