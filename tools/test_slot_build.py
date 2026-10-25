"""Test the stable-path launcher without launching systemd or Cargo."""
import contextlib
import importlib.util
import io
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch


SCRIPT = Path(__file__).resolve().parents[1] / "scripts/slot-build.py"


class SlotBuildTests(unittest.TestCase):
    def setUp(self):
        self.assertTrue(SCRIPT.is_file(), "stable-path helper is not implemented")
        spec = importlib.util.spec_from_file_location("slot_build", SCRIPT)
        self.module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.module)
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.slot = Path(directory.name)

    def test_launch_argv_keeps_source_and_target_paths_stable(self):
        self.assertEqual(
            self.module.build_argv(2, Path("/home/osso/.worktrees/lane"),
                                   "lane-build", ["cargo", "test", "--no-run"]),
            ["/usr/bin/systemd-run", "--user", "--no-block", "--remain-after-exit",
             "--unit=lane-build", "--slice=agents.slice",
             "--property=MemoryMax=9G", "--property=MemorySwapMax=0",
             "--property=CPUAccounting=yes",
             "--setenv=CARGO_INCREMENTAL=1",
             "--setenv=CARGO_TARGET_DIR=/home/osso/.cache/wow-ui-sim-targets/slot-2",
             "--property=BindPaths=/home/osso/.worktrees/lane:/home/osso/.cache/wow-ui-sim-srcmount/slot-2",
             "--working-directory=/home/osso/.cache/wow-ui-sim-srcmount/slot-2",
             "--", "cargo", "test", "--no-run"],
        )

    def test_matching_owner_is_accepted(self):
        (self.slot / ".owner").write_text("lane-owner")
        self.module.check_owner(self.slot, "lane-owner")

    def test_missing_or_different_owner_is_rejected(self):
        with self.assertRaises(FileNotFoundError):
            self.module.check_owner(self.slot, "lane-owner")
        (self.slot / ".owner").write_text("other-owner")
        with self.assertRaisesRegex(ValueError, "owner mismatch"):
            self.module.check_owner(self.slot, "lane-owner")

    def test_owner_comparison_is_exact(self):
        (self.slot / ".owner").write_text("lane-owner\n")
        with self.assertRaisesRegex(ValueError, "owner mismatch"):
            self.module.check_owner(self.slot, "lane-owner")

    def test_capacity_requires_literal_add_verdict(self):
        for output in ["agents 1/32 | ADD: available\n", "ADD\n"]:
            with self.subTest(output=output), patch.object(
                self.module.subprocess, "run",
                return_value=subprocess.CompletedProcess([], 0, output, ""),
            ) as run:
                self.module.check_capacity("lane-capacity")
                self.assertEqual(run.call_args.kwargs["env"]["CAPACITY_ID"],
                                 "lane-capacity")
                self.assertNotIn("CAPACITY_PEEK", run.call_args.kwargs["env"])
        for output in ["agents 1/32 | HOLD: ADD later", "NOT_ADD", "SHED"]:
            with self.subTest(output=output), patch.object(
                self.module.subprocess, "run",
                return_value=subprocess.CompletedProcess([], 0, output, ""),
            ):
                with self.assertRaisesRegex(ValueError, "capacity refused"):
                    self.module.check_capacity("lane-capacity")

    def test_status_reports_finished_wall_time(self):
        output = ("SubState=exited\nResult=success\nCPUUsageNSec=434560000\n"
                  "ExecMainStartTimestampMonotonic=1000000\n"
                  "ExecMainExitTimestampMonotonic=2075731\n")
        with patch.object(self.module.subprocess, "run", return_value=
                          subprocess.CompletedProcess([], 0, output, "")), \
                contextlib.redirect_stdout(io.StringIO()) as stdout:
            self.module.print_status("lane-build")
        self.assertEqual(stdout.getvalue(),
                         "SubState=exited\nResult=success\nCPUUsageNSec=434560000\n"
                         "WallTimeSec=1.075731\n")


if __name__ == "__main__":
    unittest.main()
