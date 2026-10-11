"""Behavioral coverage for the native source snapshot boundary."""

import importlib.util
import subprocess
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "build-host.py"
spec = importlib.util.spec_from_file_location("wow_build_host", SCRIPT)
build_host = importlib.util.module_from_spec(spec)
spec.loader.exec_module(build_host)


class SnapshotTests(unittest.TestCase):
    def test_snapshot_preserves_workspace_harness_inputs(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "repo"
            root.mkdir()
            subprocess.run(
                ["git", "init", "--quiet", str(root)], check=True, capture_output=True
            )
            inputs = {
                "Cargo.toml": (
                    '[package]\nname = "snapshot-fixture"\nversion = "0.1.0"\n'
                    '[workspace]\nmembers = ["crates/test-harness"]\n'
                    '[dev-dependencies]\nfixture-harness = { path = "crates/test-harness" }\n'
                ),
                "Cargo.lock": (
                    'version = 4\n[[package]]\nname = "snapshot-fixture"\n'
                    'version = "0.1.0"\ndependencies = ["fixture-harness"]\n'
                    '[[package]]\nname = "fixture-harness"\nversion = "0.1.0"\n'
                ),
                "build.rs": "fn main() {}\n",
                "src/lib.rs": "pub fn fixture_value() -> u32 { 41 }\n",
                "crates/test-harness/Cargo.toml": (
                    '[package]\nname = "fixture-harness"\nversion = "0.1.0"\n'
                    'edition = "2024"\npublish = false\n'
                ),
                "crates/test-harness/build.rs": "fn main() {}\n",
                "crates/test-harness/src/lib.rs": (
                    "pub fn fixture_value() -> u32 { 42 }\n"
                ),
            }
            for relative, content in inputs.items():
                source = root / relative
                source.parent.mkdir(parents=True, exist_ok=True)
                source.write_text(content)

            context = Path(directory) / "snapshot"
            build_host.snapshot(root, context)
            staged = context / "wow-ui-sim"
            for relative, content in inputs.items():
                with self.subTest(relative=relative):
                    copied = staged / relative
                    self.assertTrue(copied.is_file(), f"snapshot omitted {relative}")
                    self.assertEqual(copied.read_text(), content)


if __name__ == "__main__":
    unittest.main()
