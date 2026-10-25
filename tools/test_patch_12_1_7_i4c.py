#!/usr/bin/env python3
"""Behavioral static-classifier controls, not runtime API acceptance."""

import copy
from collections import Counter
import json
from pathlib import Path
import unittest

from patch_12_1_7_i4c import build_overlay, validate_overlay
from test_patch_12_1_7_static import row


class I4cTests(unittest.TestCase):
    def fixture(self):
        return {
            "Blizzard_A/Blizzard_A_Mainline.toc": "[Game]/A.xml\nPublic.lua\n",
            "Blizzard_A/Blizzard_A.toc": "Generic.lua\n",
            "Blizzard_A/Standard/A.xml": '<Ui><Script file="Selected.lua"/></Ui>',
            "Blizzard_A/Standard/Selected.lua": "function Survives() end\nSelectedAlias = Other\n",
            "Blizzard_A/Camelot/A.xml": '<Ui><Script file="Camelot.lua"/></Ui>',
            "Blizzard_A/Camelot/Camelot.lua": "function CamelotOnly() end\n",
            "Blizzard_A/Generic.lua": "function GenericOnly() end\n",
            "Blizzard_A/Public.lua": 'local Private = {}\nfunction Private.Member() end\n-- function Comment() end\nlocal s = "function String() end"\n',
            "Blizzard_B/Blizzard_B.toc": "## AllowLoadGameType: camelot\nB.lua\n",
            "Blizzard_B/B.lua": "function RejectedAddon() end\nC_Native.RejectedConsumer()\n",
        }

    def state(self, symbol, direction="added", section="framexml", files=None):
        result = build_overlay(
            [row(symbol, section, direction)], files or self.fixture()
        )
        return result["entries"][0]["ptr"]

    def test_removed_selected_declaration_contradicts_literal_retirement(self):
        state = self.state("Survives", "removed")
        self.assertEqual(state["applicability"], "CONDITIONAL_REMOVAL_CONTRADICTED")
        self.assertEqual(state.get("reason_code"), "SELECTED_REMOVAL_SURVIVES")
        self.assertEqual(
            state["evidence"][0]["path"], "Blizzard_A/Standard/Selected.lua"
        )

    def test_selected_assignment_is_source_not_runtime_publication(self):
        state = self.state("SelectedAlias", "removed")
        self.assertEqual(state["applicability"], "CONDITIONAL_REMOVAL_CONTRADICTED")
        self.assertEqual(state["evidence"][0]["kind"], "lua-assignment")
        state = self.state("SelectedAlias")
        self.assertEqual(state["applicability"], "CONDITIONAL_APPLICABLE")

    def test_excluded_variable_branch_and_toc_flavor(self):
        for symbol, reason in [
            ("CamelotOnly", "GAME_FAMILY_BRANCH"),
            ("GenericOnly", "TOC_FLAVOR"),
            ("RejectedAddon", "ADDON_GAME_TYPE"),
        ]:
            with self.subTest(symbol=symbol):
                state = self.state(symbol)
                self.assertEqual(
                    state["applicability"], "CONDITIONAL_UNSELECTED_PUBLISHER"
                )
                self.assertEqual(state.get("reason_code"), reason)
                self.assertTrue(state["evidence"][0]["chain"])
                self.assertEqual(
                    state["evidence"][0]["chosen_toc"],
                    "Blizzard_A/Blizzard_A_Mainline.toc"
                    if symbol != "RejectedAddon"
                    else "Blizzard_B/Blizzard_B.toc",
                )

    def test_selected_occurrence_overrides_excluded_occurrence(self):
        files = self.fixture()
        files["Blizzard_A/Public.lua"] += "\nfunction CamelotOnly() end\n"
        self.assertEqual(
            self.state("CamelotOnly", files=files)["applicability"],
            "CONDITIONAL_APPLICABLE",
        )

    def test_missing_literal_has_bounded_absence_receipt_and_runtime_question(self):
        state = self.state("Missing", "removed")
        self.assertEqual(state["applicability"], "UNKNOWN")
        self.assertEqual(state.get("reason_code"), "REMOVED_LITERAL_ABSENT")
        self.assertEqual(state.get("literal_source_files"), [])
        self.assertIn("Missing", state.get("runtime_question", ""))
        self.assertIn("dynamic", state.get("runtime_question", ""))

    def test_comments_strings_and_local_owners_do_not_prove_removal_or_addition(self):
        for symbol in ["Comment", "String", "Private.Member"]:
            state = self.state(symbol, "removed")
            self.assertEqual(state["applicability"], "UNKNOWN")
            self.assertNotEqual(state.get("reason_code"), "SELECTED_REMOVAL_SURVIVES")

    def test_parameter_assignments_do_not_publish_globals(self):
        files = self.fixture()
        files["Blizzard_A/Public.lua"] += (
            "\nfunction Wrapper(ParameterAlias)\nParameterAlias = Other\nend\n"
        )
        state = self.state("ParameterAlias", "removed", files=files)
        self.assertEqual(state["applicability"], "UNKNOWN")

    def test_one_excluded_definition_does_not_hide_unresolved_environment(self):
        files = self.fixture()
        files["Blizzard_Private/Blizzard_Private.toc"] = (
            "## UseSecureEnvironment: 1\nP.lua\n"
        )
        files["Blizzard_Private/P.lua"] = "function CamelotOnly() end\n"
        state = self.state("CamelotOnly", files=files)
        self.assertEqual(state["applicability"], "UNKNOWN")
        self.assertEqual(state.get("reason_code"), "UNRESOLVED_PUBLISHER_SELECTION")

    def test_native_unselected_consumer_is_not_native_inapplicability(self):
        state = self.state("C_Native.RejectedConsumer", section="global-api")
        self.assertEqual(state["applicability"], "UNKNOWN")
        self.assertEqual(state.get("reason_code"), "NATIVE_WITHOUT_SELECTED_WITNESS")
        self.assertIn("C_Native.RejectedConsumer", state.get("runtime_question", ""))

    def test_secure_environment_cannot_prove_global_exclusion(self):
        files = self.fixture()
        files["Blizzard_B/Blizzard_B.toc"] = "## UseSecureEnvironment: 1\nB.lua\n"
        state = self.state("RejectedAddon", files=files)
        self.assertEqual(state["applicability"], "UNKNOWN")
        self.assertEqual(state.get("reason_code"), "UNRESOLVED_PUBLISHER_SELECTION")

    def test_opposing_occurrences_keep_distinct_dispositions(self):
        rows = [row("Survives"), row("Survives", direction="removed", number=2)]
        result = build_overlay(rows, self.fixture())
        self.assertEqual(
            [r["ptr"]["applicability"] for r in result["entries"]],
            ["CONDITIONAL_APPLICABLE", "CONDITIONAL_REMOVAL_CONTRADICTED"],
        )
        self.assertFalse(result["runtime_complete"])
        for entry in result["entries"]:
            for profile in ["retail", "wowforever"]:
                self.assertEqual(entry[profile]["applicability"], "UNKNOWN")
                self.assertIn("Survives", entry[profile].get("runtime_question", ""))

    def test_checked_in_concrete_occurrences(self):
        root = Path(__file__).resolve().parents[1] / "data/patch-api/sources"
        original = json.loads((root / "12.1.7-inherited-1.60.1.json").read_text())[
            "entries"
        ]
        entries = json.loads((root / "12.1.7-inherited-i4c.json").read_text())[
            "entries"
        ]
        self.assertEqual(
            Counter(r["id"] for r in entries), Counter(r["id"] for r in original)
        )
        indexed = {r["id"]: r["ptr"] for r in entries}
        examples = [
            (
                "wt-framexml-UnitFrameUtil.UpdateArenaOpponentSpecDisplay-768",
                "CONDITIONAL_APPLICABLE",
                "Blizzard_UnitFrame/Shared/UnitFrameUtil.lua",
            ),
            (
                "wt-framexml-LFD_CURRENT_FILTER-1333",
                "CONDITIONAL_REMOVAL_CONTRADICTED",
                "Blizzard_GroupFinder/Mainline/LFDFrame.lua",
            ),
            (
                "wt-framexml-SCENARIOS_CURRENT_FILTER-1782",
                "CONDITIONAL_REMOVAL_CONTRADICTED",
                "Blizzard_GroupFinder/Shared/ScenarioFinder_Shared.lua",
            ),
            (
                "wt-framexml-FormatProfession-995",
                "CONDITIONAL_UNSELECTED_PUBLISHER",
                "Blizzard_UIPanels_Game/Cata/SpellBookProfessions.lua",
            ),
            (
                "wt-framexml-ToyBox_UpdateProgressBar-1803",
                "CONDITIONAL_UNSELECTED_PUBLISHER",
                "Blizzard_Collections/Classic/Blizzard_ToyBox.lua",
            ),
            (
                "wt-framexml-PvPObjectiveBannerFrame_PlayBanner-686",
                "CONDITIONAL_APPLICABLE",
                "Blizzard_UIPanels_Game/Mainline/PVPBanner.lua",
            ),
            (
                "wt-framexml-PvPObjectiveBannerFrame_PlayBanner-1694",
                "CONDITIONAL_REMOVAL_CONTRADICTED",
                "Blizzard_UIPanels_Game/Mainline/PVPBanner.lua",
            ),
        ]
        for occurrence, status, path in examples:
            with self.subTest(occurrence=occurrence):
                self.assertEqual(indexed[occurrence]["applicability"], status)
                self.assertEqual(indexed[occurrence]["evidence"][0]["path"], path)
        native = indexed["wt-global-api-C_SwingTimer.EnableRangeCheck-226"]
        self.assertEqual(native["applicability"], "UNKNOWN")
        self.assertEqual(native["reason_code"], "NATIVE_WITHOUT_SELECTED_WITNESS")
        self.assertTrue(
            all(
                r[p]["runtime_question"]
                for r in entries
                for p in ["ptr", "retail", "wowforever"]
                if r[p]["applicability"] == "UNKNOWN"
            )
        )

    def test_validator_rejects_disposition_question_receipt_and_id_mutations(self):
        rows = [row("Survives", direction="removed"), row("Missing", number=2)]
        result = build_overlay(rows, self.fixture())
        validate_overlay(result, rows, self.fixture())
        for key, value in [
            ("applicability", "CONDITIONAL_APPLICABLE"),
            ("reason_code", "invented"),
            ("runtime_question", "invented"),
            ("evidence", []),
        ]:
            mutant = copy.deepcopy(result)
            mutant["entries"][0]["ptr"][key] = value
            with self.assertRaisesRegex(ValueError, "I4c overlay"):
                validate_overlay(mutant, rows, self.fixture())
        mutant = copy.deepcopy(result)
        mutant["entries"].pop()
        with self.assertRaises(ValueError):
            validate_overlay(mutant, rows, self.fixture())


if __name__ == "__main__":
    unittest.main()
