"""Test the stable-path launcher without launching systemd or Cargo."""
import contextlib
import importlib.util
import io
import json
import os
import time
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


class SourceFreshnessTests(unittest.TestCase):
    def setUp(self):
        SlotBuildTests.setUp(self)
        self.tree = self.slot / "tree"
        self.tree.mkdir()
        self.git("init", "-q")
        self.git("config", "user.email", "test@example.com")
        self.git("config", "user.name", "Test")
        (self.tree / "src").mkdir()
        (self.tree / "src/main.rs").write_text('fn main() { println!("A"); }')
        (self.tree / "Cargo.toml").write_text('[package]\nname="probe"\nversion="0.1.0"\n')
        self.git("add", ".")
        self.git("commit", "-qm", "A")

    def git(self, *args):
        return subprocess.check_output(["git", "-C", str(self.tree), *args])

    def prepare(self, previous=None):
        self.assertTrue(callable(getattr(self.module, "prepare_source", None)),
                        "launcher does not invalidate older source mtimes")
        return self.module.prepare_source(self.tree, previous)

    def test_older_changed_file_is_touched_without_changing_bytes(self):
        previous = self.prepare()
        source = self.tree / "src/main.rs"
        source.write_text('fn main() { println!("B"); }')
        self.git("commit", "-qam", "B")
        os.utime(source, (1, 1))
        content = source.read_bytes()
        before = time.time_ns()
        current = self.prepare(previous)
        self.assertGreaterEqual(source.stat().st_mtime_ns, before)
        self.assertEqual(source.read_bytes(), content)
        self.assertEqual(current["changed_paths"], ["src/main.rs"])
        self.assertNotEqual(current["identity"], previous["identity"])

    def test_unchanged_identity_does_not_touch_sources(self):
        previous = self.prepare()
        source = self.tree / "src/main.rs"
        os.utime(source, (1, 1))
        self.prepare(previous)
        self.assertEqual(source.stat().st_mtime_ns, 1_000_000_000)

    def test_dirty_untracked_and_reverted_paths_are_invalidated(self):
        previous = self.prepare()
        (self.tree / "src/main.rs").write_text("dirty A")
        (self.tree / "src/new.rs").write_text("untracked A")
        dirty = self.prepare(previous)
        self.git("checkout", "--", "src/main.rs")
        (self.tree / "src/new.rs").write_text("untracked B")
        for name in ["src/main.rs", "src/new.rs"]:
            os.utime(self.tree / name, (1, 1))
        current = self.prepare(dirty)
        self.assertNotEqual(current["identity"], dirty["identity"])
        self.assertEqual(current["changed_paths"], ["src/main.rs", "src/new.rs"])
        self.assertTrue(all((self.tree / p).stat().st_mtime_ns > 1_000_000_000
                            for p in current["changed_paths"]))

    def test_unknown_identity_refreshes_all_tracked_crate_inputs(self):
        source = self.tree / "src/main.rs"
        os.utime(source, (1, 1))
        current = self.prepare({"artifacts": {"debug/probe": "old"}})
        self.assertIn("src/main.rs", current["changed_paths"])
        self.assertGreater(source.stat().st_mtime_ns, 1_000_000_000)

    def test_unknown_commit_invalidates_crate_inputs_not_docs(self):
        (self.tree / "data").mkdir()
        (self.tree / "data/input.csv").write_text("input")
        (self.tree / "README.md").write_text("docs")
        self.git("add", ".")
        self.git("commit", "-qm", "inputs")
        previous = self.prepare()
        previous["identity"]["head"] = "0" * 40
        for name in ["src/main.rs", "data/input.csv", "README.md"]:
            os.utime(self.tree / name, (1, 1))
        current = self.prepare(previous)
        self.assertIn("data/input.csv", current["changed_paths"])
        self.assertGreater((self.tree / "src/main.rs").stat().st_mtime_ns, 1_000_000_000)
        self.assertEqual((self.tree / "README.md").stat().st_mtime_ns, 1_000_000_000)

    def test_interrupted_build_invalidates_inputs_even_at_same_identity(self):
        previous = self.prepare()
        previous["result"] = "building"
        source = self.tree / "src/main.rs"
        os.utime(source, (1, 1))
        current = self.prepare(previous)
        self.assertIn("src/main.rs", current["changed_paths"])
        self.assertGreater(source.stat().st_mtime_ns, 1_000_000_000)

    def test_identical_artifacts_after_rust_change_fail_with_receipt(self):
        previous = self.prepare()
        previous["artifacts"] = {"debug/probe": "abc"}
        (self.tree / "src/main.rs").write_text("changed")
        current = self.prepare(previous)
        receipt = self.slot / "receipt.json"
        with self.assertRaisesRegex(ValueError, "stale artifacts"):
            self.module.finish_build(self.slot, receipt, current, previous,
                                     {"debug/probe": "abc"}, 0)
        self.assertEqual(json.loads(receipt.read_text())["result"], "stale-artifacts")

    def test_worker_rebuilds_older_tree_in_same_persistent_target(self):
        import sys
        binary = self.slot / "debug/probe"
        binary.parent.mkdir()
        (self.slot / ".owner").write_text("test-owner")
        compiler = (
            'import json,sys; from pathlib import Path; '
            'source,binary=map(Path,sys.argv[1:3]); '
            'old=binary.exists() and source.stat().st_mtime_ns <= binary.stat().st_mtime_ns; '
            'None if old else binary.write_bytes(source.read_bytes()); '
            'print(json.dumps({"reason":"compiler-artifact","executable":str(binary)}))'
        )
        request = self.slot / "request.json"
        request.write_text(json.dumps({
            "slot": str(self.slot), "worktree": str(self.tree), "owner": "test-owner",
            "unit": "pair", "receipt": str(self.slot / "pair.json"),
            "cargo": [sys.executable, "-c", compiler, str(self.tree / "src/main.rs"),
                      str(binary), "--message-format=json"]}))
        self.assertEqual(self.module.execute_build(request), 0)
        first = json.loads((self.slot / "pair.json").read_text())
        source = self.tree / "src/main.rs"
        source.write_text('fn main() { println!("B"); }')
        self.git("commit", "-qam", "older B")
        os.utime(source, (1, 1))
        self.assertEqual(self.module.execute_build(request), 0)
        second = json.loads((self.slot / "pair.json").read_text())
        self.assertNotEqual(first["artifacts"], second["artifacts"])
        self.assertEqual(binary.read_bytes(), source.read_bytes())

    def test_worker_records_real_artifact_bytes_and_propagates_failure(self):
        import hashlib
        import sys
        binary = self.slot / "debug/probe"
        binary.parent.mkdir()
        binary.write_bytes(b"A binary")
        (self.slot / ".owner").write_text("test-owner")
        emitter = ('import json,sys; print(json.dumps({"reason":"compiler-artifact",'
                   '"executable":sys.argv[1]})); sys.exit(int(sys.argv[2]))')
        request = self.slot / "request.json"
        for code in [0, 7]:
            request.write_text(json.dumps({
                "slot": str(self.slot), "worktree": str(self.tree),
                "owner": "test-owner", "unit": "test-unit",
                "cargo": [sys.executable, "-c", emitter, str(binary), str(code)],
                "receipt": str(self.slot / "receipt.json")}))
            self.assertEqual(self.module.execute_build(request), code)
            receipt = json.loads((self.slot / "receipt.json").read_text())
            self.assertEqual(receipt["artifacts"], {
                "debug/probe": hashlib.sha256(b"A binary").hexdigest()})
            self.assertEqual(receipt["returncode"], code)

    def test_binary_build_with_hidden_cargo_stdout_is_rejected(self):
        current = self.prepare()
        current["cargo"] = ["cargo", "test", "--no-run"]
        receipt = self.slot / "receipt.json"
        with self.assertRaisesRegex(ValueError, "missing executable artifact messages"):
            self.module.finish_build(self.slot, receipt, current, None, {}, 0)
        self.assertEqual(json.loads(receipt.read_text())["result"], "missing-artifacts")

    def test_changed_artifacts_and_identity_are_recorded(self):
        previous = self.prepare()
        previous["artifacts"] = {"debug/probe": "abc"}
        (self.tree / "src/main.rs").write_text("changed")
        current = self.prepare(previous)
        receipt = self.slot / "receipt.json"
        self.module.finish_build(self.slot, receipt, current, previous,
                                 {"debug/probe": "def"}, 0)
        saved = json.loads(receipt.read_text())
        self.assertEqual(saved["artifacts"], {"debug/probe": "def"})
        self.assertEqual(saved["identity"], current["identity"])


if __name__ == "__main__":
    unittest.main()
