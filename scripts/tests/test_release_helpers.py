"""Bounded helper behavior; synthetic tools do not establish native macOS proof."""

import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]


def load_helper(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


MACOS = load_helper("bundle_macos_icu", ROOT / "scripts/bundle-macos-icu.py")
DOCKER = load_helper("verify_docker_image", ROOT / ".github/scripts/verify_docker_image.py")


class MacOSBundleTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.executable = self.root / "App/Contents/MacOS/wow-sim"
        self.executable.parent.mkdir(parents=True)
        self.icu = self.root / "homebrew/lib"
        self.icu.mkdir(parents=True)
        self.system = "/usr/lib/libSystem.B.dylib"
        self.i18n = str(self.icu / "libicui18n.74.dylib")
        self.uc = str(self.icu / "libicuuc.74.dylib")
        self.data = str(self.icu / "libicudata.74.dylib")
        self.write(self.executable, [self.i18n, self.system])
        self.write(Path(self.i18n), [self.uc, self.system])
        self.write(Path(self.uc), [self.data, self.system])
        self.write(Path(self.data), [self.system])

    @staticmethod
    def write(path, imports):
        path.write_text(json.dumps(imports))

    @staticmethod
    def otool(command, **kwargs):
        path = Path(command[-1])
        return str(path) + ":\n" + "".join(
            f"\t{item} (compatibility version 1.0.0, current version 1.0.0)\n"
            for item in json.loads(path.read_text())
        )

    @staticmethod
    def install_name_tool(command, **kwargs):
        if command[1] == "-change":
            path = Path(command[-1])
            old, new = command[2:4]
            path.write_text(json.dumps([
                new if item == old else item for item in json.loads(path.read_text())
            ]))
        return subprocess.CompletedProcess(command, 0)

    def bundle(self):
        with patch.object(MACOS.subprocess, "check_output", self.otool), patch.object(
            MACOS.subprocess, "run", self.install_name_tool
        ):
            MACOS.bundle(self.executable, self.icu)

    def test_transitive_libraries_are_copied_and_imports_relocated(self):
        self.bundle()
        libraries = self.executable.parent.parent / "Frameworks"
        self.assertEqual(
            {path.name for path in libraries.iterdir()},
            {Path(item).name for item in [self.i18n, self.uc, self.data]},
        )
        for binary in [self.executable, *libraries.iterdir()]:
            for dependency in json.loads(binary.read_text()):
                if dependency != self.system:
                    self.assertTrue(dependency.startswith("@executable_path/../Frameworks/"))
                    self.assertTrue((libraries / Path(dependency).name).is_file())

    def test_missing_transitive_library_fails(self):
        Path(self.data).unlink()
        with self.assertRaises(FileNotFoundError):
            self.bundle()

    def test_non_system_unbundled_import_fails(self):
        self.write(self.executable, [self.i18n, "/opt/local/lib/libother.dylib"])
        with self.assertRaisesRegex(RuntimeError, "Unbundled dependency"):
            self.bundle()

    def test_no_icu_imports_fails(self):
        self.write(self.executable, [self.system])
        with self.assertRaisesRegex(RuntimeError, "no ICU dependencies"):
            self.bundle()


class DockerPublicationTests(unittest.TestCase):
    def verify(self, startup, summary):
        outputs = ["wow-sim 0.1.0\n", startup, summary]
        results = [subprocess.CompletedProcess([], 0, output, "") for output in outputs]
        with patch.object(DOCKER, "run_container", side_effect=results):
            DOCKER.verify("synthetic-image")

    def test_exact_two_test_success_is_accepted(self):
        self.verify("[]\n", "\x1b[32m2 tests, 2 passed, 0 failed\x1b[0m\n")

    def test_zero_tests_cannot_publish(self):
        with self.assertRaisesRegex(RuntimeError, "exactly two"):
            self.verify("[]\n", "0 tests, 0 passed, 0 failed\n")

    def test_failed_addon_test_cannot_publish(self):
        with self.assertRaisesRegex(RuntimeError, "exactly two"):
            self.verify("[]\n", "2 tests, 1 passed, 1 failed\n")

    def test_startup_errors_cannot_publish(self):
        with self.assertRaisesRegex(RuntimeError, "startup Lua errors"):
            self.verify('[{"message":"missing API"}]', "2 tests, 2 passed, 0 failed\n")


if __name__ == "__main__":
    unittest.main()
