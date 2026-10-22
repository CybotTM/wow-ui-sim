from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from tools import gen_blizzard_ui_manifest as generator


class PtrManifestTests(unittest.TestCase):
    def test_ptr_enumerates_ptr_not_ptr2(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for branch, filename in (("ptr", "New.lua"), ("ptr2", "Stale.lua")):
                addons = root / branch / "Interface/AddOns/Blizzard_Example"
                addons.mkdir(parents=True)
                (addons / filename).write_text("return true\n")
            output = root / "repo/data/blizzard-ui-files"
            output.mkdir(parents=True)
            with mock.patch.object(generator, "CACHE_ROOT", root), mock.patch.object(
                generator, "REPO_ROOT", root / "repo"
            ), mock.patch.object(sys, "argv", ["generator", "ptr", "--no-refresh"]):
                generator.main()
            self.assertEqual(
                (output / "ptr.txt").read_text(), "Blizzard_Example/New.lua\n"
            )


if __name__ == "__main__":
    unittest.main()
