"""Synthetic WDBC parser fixtures, not a substitute game catalog."""

import hashlib
import importlib.util
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest

GENERATOR = Path(__file__).resolve().parents[1] / "gen_wrath_auction_catalog.py"


def load_generator():
    spec = importlib.util.spec_from_file_location("wrath_catalog_generator", GENERATOR)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def encode_wdbc(rows, strings):
    fields = len(rows[0])
    header = struct.pack("<4s4I", b"WDBC", len(rows), fields, fields * 4, len(strings))
    return header + b"".join(struct.pack(f"<{fields}I", *row) for row in rows) + strings


class WrathCatalogGeneratorTests(unittest.TestCase):
    def test_malformed_digest_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "ItemClass.dbc").write_bytes(b"not authenticated client data")
            result = subprocess.run(
                [sys.executable, str(GENERATOR), directory, "--output-directory", str(root / "out")],
                text=True, capture_output=True, check=False,
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("SHA256", result.stderr)
            self.assertFalse((root / "out/wrath-auction-catalog.json").exists())

    def test_wdbc_reader_keeps_record_order_and_input_bytes(self):
        generator = load_generator()
        rows = [[0] * 44, [0] * 44]
        rows[0][1], rows[1][1] = 5, 1
        raw = encode_wdbc(rows, b"\0")
        source = {"sha256": hashlib.sha256(raw).hexdigest(), "records": 2, "fields": 44}
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "fixture.dbc"
            path.write_bytes(raw)
            records, strings = generator.read_wdbc(path, source)
            self.assertEqual([row[1] for row in records], [5, 1])
            self.assertEqual(strings, b"\0")
            self.assertEqual(path.read_bytes(), raw)

    def test_name_projection_flags_and_no_alternate_locale(self):
        generator = load_generator()
        strings = b"\0first\0second\0verbose\0alternate\0"
        rows = [[0] * 44, [0] * 44]
        rows[0][1], rows[1][1] = 5, 1
        rows[0][4], rows[0][5], rows[0][10] = 0x200, 1, 1
        rows[1][4], rows[1][5], rows[1][10], rows[1][27] = 0, 2, 7, 14
        projected = generator.project_subclasses(rows, strings)
        self.assertEqual(projected, [
            {"class_id": 0, "flags": 0x200, "display_flags": 1, "name": "first"},
            {"class_id": 0, "flags": 0, "display_flags": 2, "name": "verbose"},
        ])
        item_class = [0] * 20
        item_class[0], item_class[4] = 2, 22
        self.assertEqual(generator.project_classes([item_class], strings), [{"class_id": 2, "name": ""}])

    def test_record_lengths_and_string_offsets_are_validated(self):
        generator = load_generator()
        raw = encode_wdbc([[0] * 20], b"\0") + b"trailing"
        source = {"sha256": hashlib.sha256(raw).hexdigest(), "records": 1, "fields": 20}
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "fixture.dbc"
            path.write_bytes(raw)
            with self.assertRaisesRegex(ValueError, "length"):
                generator.read_wdbc(path, source)
        with self.assertRaisesRegex(ValueError, "offset"):
            generator.read_string(b"\0", 1)
        with self.assertRaisesRegex(ValueError, "unterminated"):
            generator.read_string(b"\0text", 1)


if __name__ == "__main__":
    unittest.main()
