"""Exercise slot pruning using synthetic Cargo trees only."""
import contextlib
import importlib.util
import io
import json
import os
from pathlib import Path
import tempfile
import time
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "scripts/slot-build.py"
MIB = 1024 * 1024


class SlotPruneTests(unittest.TestCase):
    def setUp(self):
        spec = importlib.util.spec_from_file_location("slot_build", SCRIPT)
        self.module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.module)
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.slot = Path(directory.name)
        (self.slot / ".owner").write_text("owner")
        self.now = time.time()

    def artifact(self, name, age=4, marker=None):
        paths = [self.slot / "debug" / suffix for suffix in (
            f".fingerprint/{name}-abc123/lib-{marker or name}",
            f"deps/{name}-abc123.d", f"deps/lib{name}-abc123.rlib",
            f"build/{name}-abc123/output", f"incremental/{name}-abc123/session/data",
        )]
        for path in paths:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"x" * MIB)
        stamp = self.now - age * 3600
        for path in paths:
            while path.name not in {".fingerprint", "deps", "build", "incremental"}:
                os.utime(path, (stamp, stamp))
                path = path.parent
        return paths

    def age_tree(self, age):
        stamp = self.now - age * 3600
        for path in self.slot.rglob("*"):
            os.utime(path, (stamp, stamp))

    def prune(self, cap=8 * MIB, **kwargs):
        self.assertTrue(callable(getattr(self.module, "prune_slot", None)),
                        "whole-unit slot pruning is not implemented")
        with contextlib.redirect_stdout(io.StringIO()) as output:
            result = self.module.prune_slot(
                self.slot, "owner", cap, now=self.now, **kwargs)
        return result, output.getvalue()

    def test_under_cap_keeps_every_member(self):
        paths = self.artifact("alpha")
        result, _ = self.prune()
        self.assertEqual(result["removed"], [])
        self.assertTrue(all(path.exists() for path in paths))

    def test_over_cap_removes_oldest_whole_units_to_watermark(self):
        oldest = self.artifact("zeta", age=8)
        middle = self.artifact("beta", age=6)
        newest = self.artifact("alpha", age=4)
        result, output = self.prune(cap=13 * MIB)
        self.assertFalse(any(path.exists() for path in oldest))
        self.assertTrue(all(path.exists() for path in middle + newest))
        self.assertLess(result["remaining_bytes"], 13 * MIB * .8)
        self.assertEqual(len(result["removed"]), 1)
        self.assertIn("zeta", output)
        for entry in result["removed"]:
            self.assertGreater(entry["size_bytes"], 0)
            self.assertTrue(entry["paths"])

    def test_recent_member_retains_entire_unit(self):
        paths = self.artifact("alpha")
        os.utime(paths[-1], (self.now, self.now))
        result, _ = self.prune(cap=MIB)
        self.assertEqual(result["removed"], [])
        self.assertTrue(all(path.exists() for path in paths))
        self.assertGreater(result["remaining_bytes"], MIB)

    def test_target_alias_with_matching_hash_is_removed_with_fingerprint(self):
        paths = self.artifact("some-package", marker="some_binary")
        alias = self.slot / "debug/deps/libsome_binary-abc123.rlib"
        alias.write_bytes(b"x" * MIB)
        self.age_tree(4)
        result, _ = self.prune(cap=MIB)
        self.assertEqual(len(result["removed"]), 1)
        self.assertFalse(alias.exists())
        self.assertFalse(any(path.exists() for path in paths))

    def test_recent_independent_incremental_hash_does_not_pin_old_artifacts(self):
        paths = self.artifact("alpha")
        incremental = self.slot / "debug/incremental/alpha-unrelated/session"
        incremental.mkdir(parents=True)
        (incremental / "data").write_bytes(b"x" * MIB)
        result, _ = self.prune(cap=2 * MIB)
        self.assertEqual(len(result["removed"]), 1)
        self.assertFalse(any(path.exists() for path in paths))
        self.assertTrue(incremental.exists())

    def test_newer_artifact_hash_is_not_coalesced_with_older_hash(self):
        paths = self.artifact("alpha")
        newer = self.slot / "debug/deps/libalpha-def456.rlib"
        newer.write_bytes(b"x" * MIB)
        result, _ = self.prune(cap=2 * MIB)
        self.assertEqual(len(result["removed"]), 1)
        self.assertFalse(any(path.exists() for path in paths))
        self.assertTrue(newer.exists())

    def test_previous_successful_receipt_protects_whole_unit(self):
        paths = self.artifact("alpha")
        receipt = {"result": "success", "reported_artifacts": [
            {"filenames": [str(paths[2])]}], "artifacts": {}}
        (self.slot / ".verified-source-identity.json").write_text(json.dumps(receipt))
        result, _ = self.prune(cap=MIB)
        self.assertEqual(result["removed"], [])
        self.assertTrue(all(path.exists() for path in paths))

    def test_mismatched_owner_never_deletes(self):
        paths = self.artifact("alpha")
        (self.slot / ".owner").write_text("someone else")
        with self.assertRaisesRegex(ValueError, "owner mismatch"):
            self.prune(cap=MIB)
        self.assertTrue(all(path.exists() for path in paths))

    def test_dry_run_needs_no_claim_and_changes_nothing(self):
        paths = self.artifact("alpha")
        (self.slot / ".owner").unlink()
        result, output = self.prune(cap=MIB, dry_run=True)
        self.assertEqual(len(result["removed"]), 1)
        self.assertIn('"dry_run": true', output)
        self.assertTrue(all(path.exists() for path in paths))

    def test_external_symlink_is_not_followed(self):
        paths = self.artifact("alpha")
        external = self.slot.parent / (self.slot.name + "-external")
        external.write_bytes(b"outside")
        self.addCleanup(external.unlink)
        (paths[-1].parent / "link").symlink_to(external)
        self.age_tree(4)
        self.prune(cap=MIB)
        self.assertEqual(external.read_bytes(), b"outside")


if __name__ == "__main__":
    unittest.main()
