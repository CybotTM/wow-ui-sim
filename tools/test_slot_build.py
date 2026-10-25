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
             "--setenv=CARGO_INCREMENTAL=1", "--setenv=RUSTC_WRAPPER=",
             "--setenv=CARGO_TARGET_DIR=/home/osso/.cache/wow-ui-sim-targets/slot-2",
             "--property=BindPaths=/home/osso/.worktrees/lane:/home/osso/.cache/wow-ui-sim-srcmount/slot-2",
             "--working-directory=/home/osso/.cache/wow-ui-sim-srcmount/slot-2",
             "--", "cargo", "test", "--no-run"],
        )

    def test_launch_disables_configured_rustc_wrapper(self):
        argv = self.module.build_argv(0, self.slot, "wrapper-test", ["cargo", "build"])
        self.assertIn("--setenv=RUSTC_WRAPPER=", argv[:argv.index("--")])

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

    def test_unknown_refresh_includes_workspace_and_path_dependency_inputs(self):
        inputs = ["iced-wgpu-patched/src/lib.rs", "iced-dynamic/src/lib.rs",
                  "xtask/src/main.rs", "examples/probe.rs", "assets/icon.bin"]
        for name in inputs:
            path = self.tree / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("input")
        self.git("add", ".")
        self.git("commit", "-qm", "additional inputs")
        for name in inputs:
            os.utime(self.tree / name, (1, 1))
        current = self.prepare()
        for name in inputs:
            self.assertIn(name, current["changed_paths"])
            self.assertGreater((self.tree / name).stat().st_mtime_ns, 1_000_000_000)
            self.assertEqual((self.tree / name).read_text(), "input")

    def test_metadata_failure_refreshes_every_tracked_file(self):
        (self.tree / "Cargo.toml").write_text("invalid manifest")
        (self.tree / "outside-input.bin").write_bytes(b"input")
        self.git("add", ".")
        self.git("commit", "-qm", "unavailable metadata")
        os.utime(self.tree / "outside-input.bin", (1, 1))
        current = self.prepare()
        self.assertIn("outside-input.bin", current["changed_paths"])
        self.assertGreater((self.tree / "outside-input.bin").stat().st_mtime_ns,
                           1_000_000_000)

    def test_virtual_workspace_uses_metadata_member_and_dependency_roots(self):
        (self.tree / "Cargo.toml").write_text(
            '[workspace]\nmembers=["app"]\nexclude=["path-library"]\nresolver="2"\n')
        for name, manifest in {
            "app": '[package]\nname="app"\nversion="0.1.0"\n'
                   '[dependencies]\npath-library={path="../path-library"}\n',
            "path-library": '[package]\nname="path-library"\nversion="0.1.0"\n',
        }.items():
            root = self.tree / name
            (root / "src").mkdir(parents=True)
            (root / "src/lib.rs").write_text("pub fn input() {}")
            (root / "Cargo.toml").write_text(manifest)
        (self.tree / "README.md").write_text("docs")
        self.git("add", ".")
        self.git("commit", "-qm", "virtual workspace")
        for name in ["app/src/lib.rs", "path-library/src/lib.rs", "README.md"]:
            os.utime(self.tree / name, (1, 1))
        current = self.prepare()
        for name in ["app/src/lib.rs", "path-library/src/lib.rs"]:
            self.assertIn(name, current["changed_paths"])
            self.assertGreater((self.tree / name).stat().st_mtime_ns, 1_000_000_000)
        self.assertEqual((self.tree / "README.md").stat().st_mtime_ns, 1_000_000_000)

    def test_unknown_commit_refreshes_root_package_including_docs(self):
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
        self.assertGreater((self.tree / "README.md").stat().st_mtime_ns, 1_000_000_000)

    def test_interrupted_build_invalidates_inputs_even_at_same_identity(self):
        previous = self.prepare()
        previous["result"] = "building"
        source = self.tree / "src/main.rs"
        os.utime(source, (1, 1))
        current = self.prepare(previous)
        self.assertIn("src/main.rs", current["changed_paths"])
        self.assertGreater(source.stat().st_mtime_ns, 1_000_000_000)

    def add_reported_executable(self, current, fresh=False):
        current.update(worktree=str(self.tree), slot=str(self.slot),
                       package_roots={"probe": [str(self.tree)]},
                       target_inputs={str(self.slot / "debug/probe"):
                                      [str(self.tree / "src/main.rs")]},
                       reported_artifacts=[{"package_id": "probe", "fresh": fresh,
                                            "executable": str(self.slot / "debug/probe")}])

    def test_identical_artifacts_after_rust_change_fail_with_receipt(self):
        previous = self.prepare()
        previous["artifacts"] = {"debug/probe": "abc"}
        (self.tree / "src/main.rs").write_text("changed")
        current = self.prepare(previous)
        receipt = self.slot / "receipt.json"
        self.add_reported_executable(current)
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
            'binary.chmod(0o755); '
            'binary.with_suffix(".d").write_text(str(binary) + ": " + str(source) + "\\n"); '
            'print(json.dumps({"reason":"compiler-artifact","executable":str(binary),"fresh":False}))'
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
        binary.with_suffix(".d").write_text(f"{binary}: {self.tree / 'src/main.rs'}\n")
        (self.slot / ".owner").write_text("test-owner")
        emitter = ('import json,sys,os; from pathlib import Path; '
                   'Path(sys.argv[1]).chmod(0o755); os.utime(sys.argv[1], None); '
                   'print(json.dumps({"reason":"compiler-artifact",'
                   '"executable":sys.argv[1],"fresh":False})); sys.exit(int(sys.argv[2]))')
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

    def test_capturing_wrappers_record_hashes_and_reject_identical_binary(self):
        import hashlib
        import sys
        binary = self.slot / "debug/probe"
        binary.parent.mkdir()
        (self.slot / ".owner").write_text("test-owner")
        # This process redirects its own streams, like cargo-exec.py; no Cargo
        # messages are visible to the worker. Its argv must remain untouched.
        wrapper = self.slot / "wrapper.py"
        wrapper.write_text(
            'import os,sys; from pathlib import Path\n'
            'assert len(sys.argv) == 2, sys.argv\n'
            'os.dup2(os.open(sys.argv[1], os.O_WRONLY|os.O_CREAT, 0o600), 1)\n'
            f'p=Path({str(binary)!r}); p.write_bytes(b"binary"); p.chmod(0o755)\n'
            'print("captured cargo output")\n')
        request = self.slot / "request.json"
        receipt = self.slot / "receipt.json"
        request.write_text(json.dumps({
            "slot": str(self.slot), "worktree": str(self.tree), "owner": "test-owner",
            "unit": "wrapped", "receipt": str(receipt),
            "cargo": [sys.executable, str(wrapper), str(self.slot / "compile.stdout")]}))
        with self.assertRaisesRegex(ValueError, "no Cargo compiler-artifact"):
            self.module.execute_build(request)
        self.assertEqual(json.loads(receipt.read_text())["artifacts"], {
            "debug/probe": hashlib.sha256(b"binary").hexdigest()})
        self.assertEqual((self.slot / "compile.stdout").read_text(),
                         "captured cargo output\n")
        (self.tree / "src/main.rs").write_text("changed Rust")
        with self.assertRaisesRegex(ValueError, "stale artifacts"):
            self.module.execute_build(request)

    def run_mixed_output_pair(self, changed_input, changed_outputs, fresh=False,
                              reported_names=None, dep_inputs=None, second_dep_inputs=None,
                              before_second=None):
        import sys
        (self.slot / ".owner").write_text("test-owner")
        emitter = (
            'import json,sys; from pathlib import Path\n'
            'slot=Path(sys.argv[1])\n'
            'for name,content in json.loads(sys.argv[2]).items():\n'
            '    path=slot/name\n'
            '    path.parent.mkdir(parents=True, exist_ok=True)\n'
            '    path.write_text(content)\n'
            '    if path.suffix != ".rmeta": path.chmod(0o755)\n'
            'for name,inputs in json.loads(sys.argv[6]).items():\n'
            '    path=(slot/name).with_suffix(".d")\n'
            '    if inputs is None:\n'
            '        path.unlink(missing_ok=True)\n'
            '    else:\n'
            '        path.parent.mkdir(parents=True,exist_ok=True)\n'
            '        path.write_text(str(slot/name) + ": " + " ".join(inputs) + "\\n")\n'
            'for name in json.loads(sys.argv[5]):\n'
            '    print(json.dumps({"reason":"compiler-artifact", "package_id":sys.argv[3],\n'
            '        "executable":str(slot/name) if not name.endswith(".rmeta") else None,\n'
            '        "fresh":name in json.loads(sys.argv[4])}))\n'
        )
        request = self.slot / "request.json"
        receipt = self.slot / "receipt.json"
        metadata = json.loads(subprocess.check_output(
            ["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"],
            cwd=self.tree))
        package_id = metadata["packages"][0]["id"]
        outputs = {"debug/probe": "A binary", "debug/other": "A other",
                   "debug/deps/probe.rmeta": "A metadata"}
        dep_inputs = {**{name: [str(self.tree / "src/main.rs")]
                         for name in outputs if not name.endswith(".rmeta")},
                      **(dep_inputs or {})}
        for phase in ["A", "B"]:
            if phase == "B":
                comment = "//" if changed_input.endswith(".rs") else "#"
                with (self.tree / changed_input).open("a") as stream:
                    stream.write(f"\n{comment} changed build input\n")
                outputs = changed_outputs
                if before_second:
                    before_second()
            request.write_text(json.dumps({
                "slot": str(self.slot), "worktree": str(self.tree),
                "owner": "test-owner", "unit": "mixed-output", "receipt": str(receipt),
                "cargo": [sys.executable, "-c", emitter, str(self.slot),
                          json.dumps(outputs), package_id,
                          json.dumps((list(dep_inputs) if fresh is True else fresh or [])
                                     if phase == "B" else []),
                          json.dumps(reported_names if phase == "B" and reported_names
                                     is not None else list(outputs)),
                          json.dumps(second_dep_inputs if phase == "B" and
                                     second_dep_inputs is not None else dep_inputs)]}))
            result = subprocess.run(
                [sys.executable, str(self.module.__file__), "--execute", str(request)],
                capture_output=True)
            if phase == "A":
                self.assertEqual(result.returncode, 0, result.stderr)
                first = json.loads(receipt.read_text())
        return result, first, json.loads(receipt.read_text())

    def test_same_package_test_edit_does_not_judge_fresh_bin(self):
        (self.tree / "tests").mkdir()
        (self.tree / "tests/x.rs").write_text("#[test] fn x() {}")
        self.git("add", ".")
        self.git("commit", "-qm", "test target")
        result, first, second = self.run_mixed_output_pair(
            "tests/x.rs", {"debug/other": "B test"}, fresh=["debug/probe"],
            reported_names=["debug/probe", "debug/other"],
            dep_inputs={"debug/probe": [str(self.tree / "src/main.rs")],
                        "debug/other": [str(self.tree / "tests/x.rs")]})
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(first["artifacts"]["debug/probe"],
                         second["artifacts"]["debug/probe"])

    def test_reported_affected_target_requires_a_current_hash(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/deps/probe.rmeta": "B metadata"},
            reported_names=["debug/probe"],
            before_second=lambda: (self.slot / "debug/probe").unlink())
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_missing_dep_info_fails_closed(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/probe": "B binary"},
            reported_names=["debug/probe"],
            second_dep_inputs={"debug/probe": None})
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"dep-info", result.stderr)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_missing_previous_dep_info_fails_even_when_child_recreates_it(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/probe": "B binary"},
            reported_names=["debug/probe"],
            before_second=lambda: (self.slot / "debug/probe.d").unlink())
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"missing dep-info", result.stderr)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_unreadable_previous_dep_info_fails_even_when_child_replaces_it(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/probe": "B binary"},
            reported_names=["debug/probe"],
            before_second=lambda: (self.slot / "debug/probe.d").write_bytes(b"\\xff"))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"unreadable dep-info", result.stderr)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_previous_dep_info_cannot_be_erased_by_child(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/probe": "B binary"}, fresh=True,
            reported_names=["debug/probe"],
            second_dep_inputs={"debug/probe": [str(self.tree / "unrelated.rs")]})
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_new_dep_info_is_unioned_with_previous_inputs(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/probe": "B binary"}, fresh=True,
            reported_names=["debug/probe"],
            dep_inputs={"debug/probe": [str(self.tree / "unrelated.rs")]},
            second_dep_inputs={"debug/probe": [str(self.tree / "src/main.rs")]})
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_deps_dep_info_handles_relative_escaped_paths_and_continuations(self):
        source = self.tree / "src/space #$.rs"
        source.write_text("pub fn input() {}")
        binary = self.slot / "debug/deps/probe-012345"
        binary.parent.mkdir(parents=True)
        binary.write_bytes(b"binary")
        dep_info = binary.with_suffix(".d")
        dep_info.write_text(
            f"{binary}: src/main.rs \\\n src/space\\ #$$.rs\n\n"
            "src/main.rs:\n\n# env-dep:EXAMPLE=value\n")
        current = {"worktree": str(self.tree), "slot": str(self.slot),
                   "package_roots": {"probe": [str(self.tree)]},
                   "changed_paths": ["src/space #$.rs"],
                   "reported_artifacts": [{"package_id": "probe", "fresh": True,
                                           "executable": str(binary)}]}
        snapshot = self.module.read_dep_info_snapshot(self.slot, self.tree)
        current["target_inputs"], errors = self.module.read_target_inputs(
            current, snapshot, snapshot)
        self.assertEqual(errors, [])
        self.assertIn(str(source), current["target_inputs"][str(binary)])
        self.assertEqual(self.module.find_stale_artifacts(current, {}, {}),
                         ["debug/deps/probe-012345"])

    def test_fresh_report_rejects_even_changed_executable_bytes(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/probe": "B binary", "debug/other": "B other"},
            fresh=True)
        self.assertNotEqual(first["artifacts"]["debug/probe"],
                            second["artifacts"]["debug/probe"])
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_unreported_same_package_executable_is_not_judged(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/other": "B other"})
        self.assertEqual(first["artifacts"]["debug/probe"],
                         second["artifacts"]["debug/probe"])
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_library_record_does_not_require_unreported_executables(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/deps/probe.rmeta": "B metadata"})
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(second["result"], "success")

    def test_no_reported_artifact_fails_for_changed_package_inputs(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/probe": "B binary", "debug/other": "B other"},
            reported_names=[])
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"no Cargo compiler-artifact records", result.stderr)

    def test_changed_metadata_cannot_certify_unchanged_executable(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/deps/probe.rmeta": "B metadata"},
            reported_names=["debug/probe", "debug/deps/probe.rmeta"])
        self.assertEqual(second["changed_paths"], ["src/main.rs"])
        self.assertEqual(first["artifacts"]["debug/probe"],
                         second["artifacts"]["debug/probe"])
        self.assertNotEqual(first["artifacts"]["debug/deps/probe.rmeta"],
                            second["artifacts"]["debug/deps/probe.rmeta"])
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"stale artifacts", result.stderr)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_changed_executable_cannot_certify_another_unchanged_executable(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/probe": "B binary"},
            reported_names=["debug/probe", "debug/other"])
        self.assertNotEqual(first["artifacts"]["debug/probe"],
                            second["artifacts"]["debug/probe"])
        self.assertEqual(first["artifacts"]["debug/other"],
                         second["artifacts"]["debug/other"])
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_build_input_change_rejects_unchanged_executable(self):
        result, first, second = self.run_mixed_output_pair(
            "Cargo.toml", {"debug/deps/probe.rmeta": "B metadata"},
            reported_names=["debug/probe", "debug/deps/probe.rmeta"])
        self.assertEqual(second["changed_paths"], ["Cargo.toml"])
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_build_script_is_an_input_even_when_absent_from_dep_info(self):
        (self.tree / "build.rs").write_text("fn main() {}")
        self.git("add", ".")
        self.git("commit", "-qm", "build script")
        result, first, second = self.run_mixed_output_pair(
            "build.rs", {"debug/deps/probe.rmeta": "B metadata"},
            reported_names=["debug/probe"])
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_lockfile_is_an_input_even_when_absent_from_dep_info(self):
        # Metadata writes the lockfile before phase A; only phase B edits it.
        subprocess.check_output(
            ["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"],
            cwd=self.tree)
        self.git("add", ".")
        self.git("commit", "-qm", "lockfile")
        result, first, second = self.run_mixed_output_pair(
            "Cargo.lock", {"debug/deps/probe.rmeta": "B metadata"},
            reported_names=["debug/probe"])
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(second["result"], "stale-artifacts")

    def test_changed_executables_allow_unchanged_library_metadata(self):
        result, first, second = self.run_mixed_output_pair(
            "src/main.rs", {"debug/probe": "B binary", "debug/other": "B other"})
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(first["artifacts"]["debug/deps/probe.rmeta"],
                         second["artifacts"]["debug/deps/probe.rmeta"])
        self.assertEqual(second["result"], "success")

    def run_test_only_pair(self, report_application):
        import hashlib
        import sys
        (self.slot / ".owner").write_text("test-owner")
        (self.tree / "app").mkdir()
        self.git("mv", "src", "app/src")
        self.git("mv", "Cargo.toml", "app/Cargo.toml")
        (self.tree / "Cargo.toml").write_text(
            '[workspace]\nmembers=["app", "tests"]\nresolver="2"\n')
        (self.tree / "tests").mkdir()
        source = self.tree / "tests/foo.rs"
        source.write_text("#[test] fn foo() {}")
        (self.tree / "tests/Cargo.toml").write_text(
            '[package]\nname="tests"\nversion="0.1.0"\n'
            '[[test]]\nname="foo"\npath="foo.rs"\n')
        self.git("add", ".")
        self.git("commit", "-qm", "test A")
        metadata = json.loads(subprocess.check_output(
            ["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"],
            cwd=self.tree))
        packages = {p["name"]: p["id"] for p in metadata["packages"]}
        emitter = (
            'import json,sys; from pathlib import Path\n'
            'slot,phase,report,app_id,test_id=sys.argv[1:]\n'
            'binary=Path(slot)/"debug/wow-sim"\n'
            'test_binary=Path(slot)/"debug/deps/foo-test"\n'
            'if phase == "A":\n'
            '    binary.parent.mkdir(parents=True,exist_ok=True)\n'
            '    binary.write_bytes(b"unchanged application executable")\n'
            '    binary.chmod(0o755)\n'
            'test_binary.parent.mkdir(parents=True,exist_ok=True)\n'
            'test_binary.write_bytes(("test executable " + phase).encode())\n'
            'test_binary.chmod(0o755)\n'
            f'binary.with_suffix(".d").write_text(str(binary) + ": " + '
            f'{str(self.tree / "app/src/main.rs")!r} + "\\n")\n'
            f'test_binary.with_suffix(".d").write_text(str(test_binary) + ": " + '
            f'{str(source)!r} + "\\n")\n'
            'if phase == "A" or report == "yes":\n'
            '    print(json.dumps({"reason":"compiler-artifact", "package_id":app_id,\n'
            '        "executable":str(binary), "fresh":phase == "B"}))\n'
            'print(json.dumps({"reason":"compiler-artifact", "package_id":test_id,\n'
            '    "executable":str(test_binary), "fresh":False}))\n'
        )
        request = self.slot / "request.json"
        receipt = self.slot / "receipt.json"
        binary = self.slot / "debug/wow-sim"
        for phase in ["A", "B"]:
            if phase == "B":
                before = binary.stat()
                digest = hashlib.sha256(binary.read_bytes()).hexdigest()
                source.write_text("#[test] fn foo() { assert_eq!(2, 2); }")
                self.git("commit", "-qam", "test B only")
            request.write_text(json.dumps({
                "slot": str(self.slot), "worktree": str(self.tree),
                "owner": "test-owner", "unit": "test-only", "receipt": str(receipt),
                "cargo": [sys.executable, "-c", emitter, str(self.slot), phase,
                          "yes" if report_application else "no", packages["probe"],
                          packages["tests"]]}))
            result = subprocess.run([sys.executable, "-B", str(self.module.__file__),
                                     "--execute", str(request)], capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
        saved = json.loads(receipt.read_text())
        self.assertEqual(saved["changed_paths"], ["tests/foo.rs"])
        self.assertEqual(saved["result"], "success")
        self.assertEqual(binary.stat().st_mtime_ns, before.st_mtime_ns)
        self.assertEqual(binary.stat().st_ctime_ns, before.st_ctime_ns)
        self.assertEqual(hashlib.sha256(binary.read_bytes()).hexdigest(), digest)

    def test_test_only_diff_ignores_unreported_application(self):
        self.run_test_only_pair(False)

    def test_test_only_diff_ignores_fresh_disjoint_application(self):
        self.run_test_only_pair(True)

    def test_worker_preserves_binary_streams_and_launcher_arguments(self):
        import sys
        (self.slot / ".owner").write_text("test-owner")
        self.module.write_json(self.slot / ".source-identity.json", self.prepare())
        request = self.slot / "request.json"
        request.write_text(json.dumps({
            "slot": str(self.slot), "worktree": str(self.tree), "owner": "test-owner",
            "unit": "streams", "receipt": str(self.slot / "receipt.json"),
            "cargo": [sys.executable, "-c",
                      'import os,sys; assert len(sys.argv)==1; '
                      'os.write(1,b"stdout\\xff\\x00"); os.write(2,b"stderr\\xfe\\x00")']}))
        result = subprocess.run([sys.executable, str(self.module.__file__),
                                 "--execute", str(request)], capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, b"stdout\xff\x00")
        self.assertEqual(result.stderr, b"stderr\xfe\x00")

    def test_hidden_stream_allows_diff_outside_package_roots(self):
        current = self.prepare()
        self.add_reported_executable(current)
        current.update(changed_paths=["README.md"], reported_artifacts=[],
                       package_roots={"probe": [str(self.tree / "app")]})
        receipt = self.slot / "receipt.json"
        self.module.finish_build(self.slot, receipt, current, None, {}, 0)
        self.assertEqual(json.loads(receipt.read_text())["result"], "success")

    def test_changed_artifacts_and_identity_are_recorded(self):
        previous = self.prepare()
        previous["artifacts"] = {"debug/probe": "abc"}
        (self.tree / "src/main.rs").write_text("changed")
        current = self.prepare(previous)
        receipt = self.slot / "receipt.json"
        self.add_reported_executable(current)
        self.module.finish_build(self.slot, receipt, current, previous,
                                 {"debug/probe": "def"}, 0)
        saved = json.loads(receipt.read_text())
        self.assertEqual(saved["artifacts"], {"debug/probe": "def"})
        self.assertEqual(saved["identity"], current["identity"])


if __name__ == "__main__":
    unittest.main()
