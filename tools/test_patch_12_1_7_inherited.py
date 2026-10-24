#!/usr/bin/env python3
"""Disposable CLI controls for the I4a accounting boundary; no runtime proof."""
import copy
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
MATRIX = Path("data/patch-api/sources/12.1.7-inherited-applicability.json")
VALIDATOR = ROOT / "tools/validate_patch_12_1_7_inherited.py"


class InheritedAccountingTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        shutil.copytree(ROOT / "data/patch-api/sources", self.root / "data/patch-api/sources")
        (self.root / "data/patch-api").mkdir(exist_ok=True)
        shutil.copy2(ROOT / "data/patch-api/patch-page-index.json", self.root / "data/patch-api")
        shutil.copytree(ROOT / "tests", self.root / "tests", ignore=shutil.ignore_patterns("*.png", "*.webp"))
        shutil.copytree(ROOT / "patch-tests", self.root / "patch-tests")
        path = self.root / MATRIX
        self.matrix = json.loads(path.read_text())

    def run_validator(self):
        (self.root / MATRIX).write_text(json.dumps(self.matrix))
        return subprocess.run(
            [sys.executable, "-B", str(VALIDATOR), "--root", str(self.root)],
            text=True, capture_output=True, check=False,
        )

    def reject(self, diagnostic):
        result = self.run_validator()
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn(diagnostic, result.stderr)

    def row(self, symbol=None):
        return next(row for row in self.matrix["entries"]
                    if symbol is None or row["source"]["symbol"] == symbol)

    def test_unknown_matrix_is_accounting_not_completion(self):
        result = self.run_validator()
        self.assertEqual(result.returncode, 0, result.stderr)
        summary = json.loads(result.stdout)
        self.assertEqual(summary["source_rows"], 1876)
        self.assertEqual(summary["directions"], {"added": 816, "removed": 1060})
        self.assertEqual(summary["applicability_resolved"], 0)
        self.assertEqual(summary["applicability_unknown"], 5628)
        self.assertEqual(summary["opposing_sweep_intersections"], 63)
        self.assertFalse(summary["runtime_complete"])

    def test_reject_missing_occurrence(self):
        self.matrix["entries"] = self.matrix["entries"][1:]
        self.reject("occurrence IDs")

    def test_reject_duplicate_occurrence(self):
        self.matrix["entries"].append(copy.deepcopy(self.row()))
        self.reject("occurrence IDs")

    def test_reject_changed_identity_fields(self):
        for field, value in (("symbol", "Other"), ("direction", "removed"),
                             ("wikitext_line", 1), ("annotation", "invented")):
            with self.subTest(field=field):
                original = copy.deepcopy(self.row()["source"])
                self.row()["source"][field] = value
                self.reject("literal occurrence")
                self.row()["source"] = original

    def test_reject_collapsed_opposing_occurrences(self):
        rows = self.matrix["entries"]
        self.matrix["entries"] = [row for row in rows if not (
            row["source"]["symbol"] == "PvPObjectiveBannerFrame_PlayBanner"
            and row["source"]["direction"] == "removed")]
        self.reject("occurrence IDs")

    def test_reject_command_as_cvar(self):
        self.row("dumpSmallAlloc")["domain"] = "cvar"
        self.reject("domain")

    def test_reject_changed_default_or_kind(self):
        for symbol, field, value in (("volumeFogUse16BitTexture", "page_default", "0"),
                                     ("dumpSmallAlloc", "kind", "cvar")):
            with self.subTest(symbol=symbol):
                row = self.row(symbol)
                original = copy.deepcopy(row["source"])
                row["source"][field] = value
                self.reject("literal occurrence")
                row["source"] = original

    def test_reject_suffix_cooldown_credit(self):
        self.row("C_Spell.GetItemCooldown")["existing_tests"].append({
            "path": "tests/item_cooldowns.rs", "line": 1,
            "symbol": "C_Item.GetItemCooldown", "proof": "LEXICAL_ONLY",
        })
        self.reject("exact test identity")

    def test_reject_unselected_consumers(self):
        for profile, path in (("wowforever", "Standard/Blizzard_StableUI.lua"),
                              ("ptr", "Camelot/Blizzard_StableUI.lua")):
            with self.subTest(profile=profile):
                state = self.row("C_StableInfo.GetNumStableSlots")["profiles"][profile]
                state["selected_source"] = path
                self.reject("selected-source evidence")
                state["selected_source"] = None

    def test_reject_lexical_applicability_upgrade(self):
        self.row()["profiles"]["ptr"]["applicability"] = "APPLICABLE"
        self.reject("UNKNOWN")

    def test_reject_wrong_profile_or_build_evidence(self):
        for evidence in ({"profile": "retail", "build": "12.1.7.70493"},
                         {"profile": "ptr", "build": "12.1.5.69594"}):
            with self.subTest(evidence=evidence):
                self.row()["profiles"]["ptr"]["selected_source"] = evidence
                self.reject("selected-source evidence")

    def test_reject_missing_concrete_ptr_pin(self):
        self.row()["profiles"]["ptr"]["reason"] = "UNKNOWN: not checked"
        self.reject("missing PTR pin")

    def test_reject_retired_sweep_as_completion(self):
        row = next(row for row in self.matrix["entries"] if row["opposing_sweeps"])
        row["profiles"]["retail"]["publication"] = "PASS"
        self.reject("UNKNOWN")

    def test_reject_dropped_sweep_conflict(self):
        row = next(row for row in self.matrix["entries"] if row["opposing_sweeps"])
        row["opposing_sweeps"] = []
        self.reject("opposing sweep join")

    def test_reject_private_paths(self):
        self.row()["profiles"]["ptr"]["reason"] += " " + str(self.root)
        self.reject("private path")

    def test_reject_changed_headers(self):
        self.matrix["header_counts"] = []
        self.reject("header disagreements")

    def test_reject_i0_byte_changes(self):
        path = self.root / "data/patch-api/sources/12.1.7-source-accounting.json"
        path.write_bytes(path.read_bytes() + b"\n")
        self.reject("immutable I0 bytes")

    def test_reject_all_false_proof_stages(self):
        for profile in ("ptr", "wowforever", "retail"):
            for stage in ("publication", "behavior", "native", "original_consumer"):
                with self.subTest(profile=profile, stage=stage):
                    state = self.row()["profiles"][profile]
                    state[stage] = "PASS"
                    self.reject("UNKNOWN")
                    state[stage] = "UNKNOWN"

    def test_reject_missing_profile(self):
        del self.row()["profiles"]["wowforever"]
        self.reject("PTR/Forever/Retail")

    def test_reject_register_only_source_drift(self):
        path = self.root / "data/patch-api/sources/2.4.0-wikitext-register.json"
        register = json.loads(path.read_text())
        register["entries"] = [row for row in register["entries"]
                               if row["symbol"] != "GetArmorPenetration"]
        path.write_text(json.dumps(register))
        self.reject("register-only source")

    def test_accept_unrelated_later_patch(self):
        path = self.root / "data/patch-api/sources/99.0.0-wikitext-register.json"
        path.write_text(json.dumps({"entries": [{"id": "future", "symbol": "Unrelated"}]}))
        result = self.run_validator()
        self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == "__main__":
    unittest.main()
