import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from tools import gen_limited_listfile


class LimitedListfileGenerationTests(unittest.TestCase):
    def test_collects_blizzard_ui_files_from_manifest(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = root / "blizzard-ui-files.txt"
            manifest.write_text(
                "Blizzard_Test/Blizzard_Test.lua\n"
                "Blizzard_Test/Textures/Icon.blp\n",
                encoding="utf-8",
            )

            with patch.object(gen_limited_listfile, "BLIZZARD_UI_FILE_MANIFEST", manifest):
                blizzard_files = gen_limited_listfile.collect_blizzard_ui_files()

            self.assertEqual(
                {
                    "interface/addons/blizzard_test/blizzard_test.lua",
                    "interface/addons/blizzard_test/textures/icon.blp",
                },
                blizzard_files,
            )

    def test_prefers_community_listfile_path_match_for_blizzard_ui_file(self):
        blizzard_files = {"interface/addons/blizzard_test/blizzard_test.lua"}
        rows = gen_limited_listfile.resolve_rows(
            by_path={
                "interface/addons/blizzard_test/blizzard_test.lua": (
                    67890,
                    "interface/addons/blizzard_test/blizzard_test.lua",
                )
            },
            by_fdid={},
            requested_paths=set(),
            requested_fdids=set(),
            blizzard_files=blizzard_files,
        )

        self.assertEqual(
            [(67890, "interface/addons/blizzard_test/blizzard_test.lua")],
            rows,
        )


if __name__ == "__main__":
    unittest.main()
