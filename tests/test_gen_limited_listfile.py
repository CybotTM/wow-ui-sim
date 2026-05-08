import sqlite3
import tempfile
import unittest
from pathlib import Path

from tools import gen_limited_listfile


class LimitedListfileGenerationTests(unittest.TestCase):
    def test_resolves_blizzard_ui_file_by_content_key_when_path_is_missing(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            blizzard_root = root / "Interface" / "BlizzardUI"
            source_file = blizzard_root / "Blizzard_Test" / "Blizzard_Test.lua"
            source_file.parent.mkdir(parents=True)
            source_file.write_bytes(b"print('from local blizzard source')\n")

            resolution_db = root / "resolution.sqlite"
            content_key = gen_limited_listfile.hash_file_md5(source_file)
            with sqlite3.connect(resolution_db) as connection:
                connection.execute(
                    "create table resolution ("
                    "fdid integer primary key, "
                    "content_key blob not null, "
                    "encoding_key blob not null)"
                )
                connection.execute(
                    "insert into resolution values (?, ?, ?)",
                    (12345, content_key, b"encoding-key"),
                )

            blizzard_files = gen_limited_listfile.collect_blizzard_ui_files(
                blizzard_root
            )
            rows = gen_limited_listfile.resolve_rows(
                by_path={},
                by_fdid={},
                requested_paths=set(),
                requested_fdids=set(),
                blizzard_files=blizzard_files,
                resolution_db=resolution_db,
            )

            self.assertEqual(
                [(12345, "interface/addons/blizzard_test/blizzard_test.lua")],
                rows,
            )

    def test_prefers_community_listfile_path_match_for_blizzard_ui_file(self):
        with tempfile.TemporaryDirectory() as tmp:
            blizzard_root = Path(tmp) / "Interface" / "BlizzardUI"
            source_file = blizzard_root / "Blizzard_Test" / "Blizzard_Test.lua"
            source_file.parent.mkdir(parents=True)
            source_file.write_bytes(b"local source differs from casc\n")

            blizzard_files = gen_limited_listfile.collect_blizzard_ui_files(
                blizzard_root
            )
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
                resolution_db=None,
            )

            self.assertEqual(
                [(67890, "interface/addons/blizzard_test/blizzard_test.lua")],
                rows,
            )


if __name__ == "__main__":
    unittest.main()
