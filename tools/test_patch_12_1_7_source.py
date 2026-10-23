"""Offline source-accounting boundary: an empty API inventory is not coverage."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
SOURCES = Path("data/patch-api/sources")
LEDGER = SOURCES / "12.1.7-source-accounting.json"
INHERITED = SOURCES / "12.1.7-inherited-1.60.1.json"


class SourceAccountingTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        (self.root / SOURCES).mkdir(parents=True)
        for source in (ROOT / SOURCES).glob("12.1.7-*"):
            shutil.copyfile(source, self.root / SOURCES / source.name)
        shutil.copyfile(ROOT / "data/patch-api/patch-page-index.json",
                        self.root / "data/patch-api/patch-page-index.json")

    def mutate(self, path, change):
        target = self.root / path
        data = json.loads(target.read_text())
        change(data)
        target.write_text(json.dumps(data))

    def validate(self):
        return subprocess.run(
            [sys.executable, "-B", str(ROOT / "tools/validate_patch_12_1_7_source.py"),
             "--root", str(self.root)], capture_output=True, text=True, check=False)

    def assert_rejected(self, reason):
        result = self.validate()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(reason, result.stderr)

    def test_empty_api_register_cannot_substitute_for_source_ledger(self):
        (self.root / LEDGER).write_text(json.dumps({
            "schema": "patch-api-wikitext-register/v1", "patch": "12.1.7",
            "header_counts": [], "entries": [],
        }))
        self.assert_rejected("source-accounting ledger required")

    def test_all_rows_accounted_does_not_claim_runtime_coverage(self):
        result = self.validate()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout), {
            "patch": "12.1.7", "source_accounted": True,
            "runtime_complete": False, "direct_rows": 12, "inherited_unknown": 1876,
        })

    def test_omitted_direct_prose_row_is_rejected(self):
        self.mutate(LEDGER, lambda data: data["direct"].pop(7))
        self.assert_rejected("direct prose rows")

    def test_inherited_count_mismatch_is_rejected(self):
        self.mutate(INHERITED, lambda data: data["header_counts"][0].update(parsed_count=176))
        self.assert_rejected("inherited header counts")

    def test_repeated_symbol_occurrences_keep_distinct_source_identity(self):
        data = json.loads((self.root / INHERITED).read_text())
        seen = {}
        for position, row in enumerate(data["entries"]):
            key = (row["section"], row["symbol"])
            if key in seen:
                original = data["entries"][seen[key]]
                self.assertNotEqual(row["id"], original["id"])
                data["entries"][position] = original.copy()
                break
            seen[key] = position
        else:
            self.fail("fixture requires a repeated literal symbol")
        (self.root / INHERITED).write_text(json.dumps(data))
        self.assert_rejected("inherited occurrence identities")

    def test_patch_index_omission_is_rejected(self):
        self.mutate(Path("data/patch-api/patch-page-index.json"), lambda data: data.update(
            api_change_pages=[row for row in data["api_change_pages"]
                              if row["title"] != "Patch 12.1.7/API changes"]))
        self.assert_rejected("patch index entry")

    def test_changed_source_body_is_rejected(self):
        with (self.root / SOURCES / "12.1.7-api-changes.wikitext").open("a") as handle:
            handle.write("\n* A changed source.\n")
        self.assert_rejected("source body identity")

    def test_inherited_unknown_cannot_be_credited_as_implemented(self):
        self.mutate(INHERITED, lambda data: data["entries"][0].update(status="IMPLEMENTED"))
        self.assert_rejected("inherited rows must remain UNKNOWN")

    def test_direct_status_cannot_be_credited_as_complete(self):
        self.mutate(LEDGER, lambda data: data["direct"][1].update(status="IMPLEMENTED"))
        self.assert_rejected("direct prose status")


if __name__ == "__main__":
    unittest.main()
