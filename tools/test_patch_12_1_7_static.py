#!/usr/bin/env python3
"""Static I4b selection controls. These are not simulator producer tests."""

import copy
import hashlib
from pathlib import Path
import tempfile
import unittest

from patch_12_1_7_static import (
    authenticate_files,
    build_overlay,
    load_pinned_files,
    validate_overlay,
)


def row(symbol, section="framexml", direction="added", number=1):
    return {
        "id": f"fixture-{number}-{symbol}",
        "symbol": symbol,
        "section": section,
        "direction": direction,
        "wikitext_line": number,
    }


class StaticSelectionTests(unittest.TestCase):
    def fixture(self):
        files = {
            "Blizzard_Test/Blizzard_Test_Mainline.toc": (
                "## AllowLoad: Game\n[Game]/Game.xml\n"
                "[Family]/Family.lua\nBlocked.lua [AllowLoadGameType camelot]\n"
                "Excluded.lua [ExcludeLoadGameType mainline]\n"
            ),
            "Blizzard_Test/Blizzard_Test.toc": "WrongFlavor.lua\n",
            "Blizzard_Test/Standard/Game.xml": '<Ui><Script file="../Shared.lua"/></Ui>',
            "Blizzard_Test/Shared.lua": (
                "-- function CommentOnly() end\n"
                'local text = "function StringOnly() end"\n'
                "--[=[ function LongCommentOnly() end ]=]\n"
                "local long = [=[ function LongStringOnly() end ]=]\n"
                "local function LocalOnly() end\n"
                "function SelectedMixin:Publish() end\n"
                'local f = CreateFrame("Frame")\nf:RegisterEvent("SELECTED_EVENT")\n'
                'local value = C_CVar.GetCVar("selectedCVar")\n'
                "C_Spell.SelectedCall()\n"
            ),
            "Blizzard_Test/Mainline/Family.lua": "function FamilySelected() end\n",
            "Blizzard_Test/Blocked.lua": "function Blocked() end\n",
            "Blizzard_Test/Excluded.lua": "function Excluded() end\n",
            "Blizzard_Test/WrongFlavor.lua": "function WrongFlavor() end\n",
            "Blizzard_Test/Camelot/Game.xml": '<Ui><Script file="../Unselected.lua"/></Ui>',
            "Blizzard_Test/Unselected.lua": "function Unselected() end\n",
            "Blizzard_Lod/Blizzard_Lod.toc": "## AllowLoad: Game\n## LoadOnDemand: 1\nLod.lua\n",
            "Blizzard_Lod/Lod.lua": "function LoadOnDemandPublisher() end\n",
            "Blizzard_Glue/Blizzard_Glue.toc": "## AllowLoad: Glue\nGlue.lua\n",
            "Blizzard_Glue/Glue.lua": "function GluePublisher() end\n",
            "Blizzard_Both/Blizzard_Both.toc": "## AllowLoad: Both\nBoth.lua\n",
            "Blizzard_Both/Both.lua": "function BothPublisher() end\n",
            "Blizzard_Restricted/Blizzard_Restricted.toc": (
                "## AllowLoadGameType: camelot\nOnly.lua\n"
            ),
            "Blizzard_Restricted/Only.lua": "function Restricted() end\n",
            "Blizzard_Env/Blizzard_Env.toc": "Secure.lua [LoadIntoEnvironment secure]\n",
            "Blizzard_Env/Secure.lua": "function PrivateEnvironment() end\n",
        }
        return files

    def overlay(self, rows, files=None):
        return build_overlay(rows, self.fixture() if files is None else files)

    def test_selected_xml_game_family_and_method_identity(self):
        rows = [row("SelectedMixin.Publish"), row("FamilySelected", number=2)]
        result = self.overlay(rows)
        self.assertEqual(
            [r["ptr"]["applicability"] for r in result["entries"]],
            ["CONDITIONAL_APPLICABLE"] * 2,
        )
        evidence = result["entries"][0]["ptr"]["evidence"][0]
        self.assertEqual(evidence["path"], "Blizzard_Test/Shared.lua")
        self.assertEqual(
            [edge["path"] for edge in evidence["chain"]],
            [
                "Blizzard_Test/Blizzard_Test_Mainline.toc",
                "Blizzard_Test/Standard/Game.xml",
            ],
        )
        self.assertEqual(evidence["group"], "game")
        self.assertEqual(evidence["symbol"], "SelectedMixin.Publish")

    def test_unselected_flavors_filters_and_private_environments_stay_unknown(self):
        names = [
            "WrongFlavor",
            "Blocked",
            "Excluded",
            "Unselected",
            "Restricted",
            "PrivateEnvironment",
        ]
        result = self.overlay([row(name, number=i) for i, name in enumerate(names)])
        self.assertEqual(
            {r["ptr"]["applicability"] for r in result["entries"]}, {"UNKNOWN"}
        )
        self.assertTrue(all(not r["ptr"]["evidence"] for r in result["entries"]))

    def test_lexical_comments_strings_and_locals_are_not_publishers(self):
        names = [
            "CommentOnly",
            "StringOnly",
            "LongCommentOnly",
            "LongStringOnly",
            "LocalOnly",
        ]
        result = self.overlay([row(name, number=i) for i, name in enumerate(names)])
        self.assertTrue(
            all(r["ptr"]["applicability"] == "UNKNOWN" for r in result["entries"])
        )

    def test_game_lod_glue_are_separate_and_both_has_two_groups(self):
        names = ["LoadOnDemandPublisher", "GluePublisher", "BothPublisher"]
        result = self.overlay([row(name, number=i) for i, name in enumerate(names)])
        groups = [{e["group"] for e in r["ptr"]["evidence"]} for r in result["entries"]]
        self.assertEqual(groups, [{"lod"}, {"glue"}, {"game", "glue"}])

    def test_exact_factory_consumers_do_not_grant_suffix_or_value_proof(self):
        rows = [
            row("C_Spell.SelectedCall", "global-api"),
            row("C_Item.SelectedCall", "global-api", number=2),
            row("SELECTED_EVENT", "events", number=3),
            row("selectedCVar", "cvars", number=4),
            row("noSuchCVar", "cvars", number=5),
            row("ScriptRegion:MouseDown", "widgets", number=6),
        ]
        result = self.overlay(rows)
        self.assertEqual(
            [r["ptr"]["applicability"] for r in result["entries"]],
            [
                "CONDITIONAL_APPLICABLE",
                "UNKNOWN",
                "CONDITIONAL_APPLICABLE",
                "CONDITIONAL_APPLICABLE",
                "UNKNOWN",
                "UNKNOWN",
            ],
        )
        for entry in result["entries"]:
            self.assertEqual(entry["retail"]["applicability"], "UNKNOWN")
            self.assertEqual(entry["wowforever"]["applicability"], "UNKNOWN")
        self.assertFalse(result["runtime_complete"])

    def test_opposing_occurrences_do_not_require_simultaneous_presence_absence(self):
        rows = [
            row("SelectedMixin.Publish"),
            row("SelectedMixin.Publish", direction="removed", number=2),
        ]
        result = self.overlay(rows)
        self.assertEqual(len(result["entries"]), 2)
        self.assertEqual(
            result["entries"][0]["ptr"]["applicability"], "CONDITIONAL_APPLICABLE"
        )
        self.assertEqual(result["entries"][1]["ptr"]["applicability"], "UNKNOWN")
        self.assertTrue(result["entries"][1]["ptr"]["evidence"])

    def test_missing_or_unparsed_include_cannot_create_absence_credit(self):
        files = self.fixture()
        files["Blizzard_Test/Standard/Game.xml"] = (
            '<Ui><Script file="Missing.lua"/></Ui>'
        )
        result = self.overlay(
            [row("SelectedMixin.Publish", direction="removed")], files
        )
        self.assertEqual(result["entries"][0]["ptr"]["applicability"], "UNKNOWN")
        self.assertTrue(result["selection_gaps"])

    def test_variable_metadata_is_ignored_for_ptr(self):
        files = {
            "Blizzard_A/Blizzard_A.toc": "## AllowLoad: [Game]\n## LoadOnDemand: [Family]\nA.lua\n",
            "Blizzard_A/A.lua": "function A() end\n",
        }
        result = self.overlay([row("A")], files)
        self.assertEqual(
            [e["group"] for e in result["entries"][0]["ptr"]["evidence"]], ["game"]
        )

    def test_reject_accept_all_and_constant_overlay_answers(self):
        rows = [row("SelectedMixin.Publish"), row("Missing", number=2)]
        expected = self.overlay(rows)
        validate_overlay(expected, rows, self.fixture())
        for constant in ("CONDITIONAL_APPLICABLE", "UNKNOWN", "NOT_APPLICABLE"):
            with self.subTest(constant=constant):
                mutant = copy.deepcopy(expected)
                for entry in mutant["entries"]:
                    entry["ptr"]["applicability"] = constant
                with self.assertRaisesRegex(ValueError, "static overlay"):
                    validate_overlay(mutant, rows, self.fixture())

    def test_reject_evidence_and_dependency_mutations(self):
        rows = [row("SelectedMixin.Publish")]
        expected = self.overlay(rows)
        for field, value in [
            ("dependency", "master"),
            ("runtime_complete", True),
            ("entries", []),
            ("selection_gaps", ["invented"]),
        ]:
            with self.subTest(field=field):
                mutant = copy.deepcopy(expected)
                mutant[field] = value
                with self.assertRaisesRegex(ValueError, "static overlay"):
                    validate_overlay(mutant, rows, self.fixture())
        mutant = copy.deepcopy(expected)
        mutant["entries"][0]["ptr"]["evidence"][0]["path"] = (
            "Blizzard_Test/Unselected.lua"
        )
        with self.assertRaisesRegex(ValueError, "static overlay"):
            validate_overlay(mutant, rows, self.fixture())

    def test_case_resolution_and_addon_root_include_match_loader(self):
        files = self.fixture()
        files["Blizzard_Test/Standard/Game.xml"] = (
            '<Ui><Include file="nested.xml"/></Ui>'
        )
        files["Blizzard_Test/Nested.xml"] = '<Ui><Script file="SHARED.LUA"/></Ui>'
        result = self.overlay([row("SelectedMixin.Publish")], files)
        self.assertEqual(
            result["entries"][0]["ptr"]["applicability"], "CONDITIONAL_APPLICABLE"
        )

    def test_explicit_global_environment_and_locale_selection(self):
        files = {
            "Blizzard_A/Blizzard_A.toc": (
                "Global.lua [AllowLoadEnvironment Global]\n"
                "German.lua [AllowLoadTextLocale deDE]\n"
                "English.lua [AllowLoadTextLocale enUS, frFR]\n"
            ),
            "Blizzard_A/Global.lua": "function GlobalSelected() end\n",
            "Blizzard_A/German.lua": "function GermanOnly() end\n",
            "Blizzard_A/English.lua": "function EnglishSelected() end\n",
            "Blizzard_S/Blizzard_S.toc": "## UseSecureEnvironment: 1\nPrivate.lua\n",
            "Blizzard_S/Private.lua": "function SecureOnly() end\n",
        }
        result = self.overlay(
            [
                row(symbol, number=i)
                for i, symbol in enumerate(
                    ["GlobalSelected", "GermanOnly", "EnglishSelected", "SecureOnly"]
                )
            ],
            files,
        )
        self.assertEqual(
            [r["ptr"]["applicability"] for r in result["entries"]],
            ["CONDITIONAL_APPLICABLE", "UNKNOWN", "CONDITIONAL_APPLICABLE", "UNKNOWN"],
        )

    def test_prefix_colon_locale_gate_is_exclusion_not_missing_cache(self):
        files = {
            "Blizzard_A/Blizzard_A.toc": "[AllowLoadTextLocale: zhCN] Chinese.lua\n",
            "Blizzard_A/Chinese.lua": "function ChineseOnly() end\n",
        }
        result = self.overlay([row("ChineseOnly")], files)
        self.assertEqual(result["entries"][0]["ptr"]["applicability"], "UNKNOWN")
        self.assertEqual(
            [gap["reason"] for gap in result["selection_gaps"]],
            ["non-public environment or non-enUS file; no global witness"],
        )

    def test_nested_frame_script_file_is_not_a_document_include(self):
        files = {
            "Blizzard_A/Blizzard_A.toc": "A.xml\n",
            "Blizzard_A/A.xml": '<Ui><Frame><Scripts><Script file="Hidden.lua"/></Scripts></Frame></Ui>',
            "Blizzard_A/Hidden.lua": "function NotDocumentSelected() end\n",
        }
        result = self.overlay([row("NotDocumentSelected")], files)
        self.assertEqual(result["entries"][0]["ptr"]["applicability"], "UNKNOWN")

    def test_byte_line_receipts_do_not_treat_cp1252_punctuation_as_newlines(self):
        files = {
            "Blizzard_A/Blizzard_A.toc": "A.lua\n",
            "Blizzard_A/A.lua": "-- cp1252 ellipsis "
            + chr(0x85)
            + "\nfunction A() end\n",
        }
        result = self.overlay([row("A")], files)
        evidence = result["entries"][0]["ptr"]["evidence"][0]
        self.assertEqual(evidence["line"], 2)
        self.assertEqual(evidence["text"], "function A() end")

    def test_namespace_shadowing_in_multiple_local_bindings_is_not_native_credit(self):
        files = {
            "Blizzard_A/Blizzard_A.toc": "A.lua\n",
            "Blizzard_A/A.lua": "local first, C_Spell = nil, {}\nC_Spell.SelectedCall()\n",
        }
        result = self.overlay([row("C_Spell.SelectedCall", "global-api")], files)
        self.assertEqual(result["entries"][0]["ptr"]["applicability"], "UNKNOWN")

    def test_generated_docs_preserve_namespace_and_ignore_commented_records(self):
        files = {
            "Blizzard_APIDocumentationGenerated/Blizzard_APIDocumentationGenerated.toc": (
                "## LoadOnDemand: 1\nExampleDocumentation.lua\n"
            ),
            "Blizzard_APIDocumentationGenerated/ExampleDocumentation.lua": (
                'local Example = { Name = "Example", Type = "System", Namespace = "C_Spell",\n'
                'Functions = { {Name = "SelectedCall", Type = "Function"},\n'
                '-- {Name = "CommentOnly", Type = "Function"},\n'
                '}, Events = {{Name = "Selected", Type = "Event", LiteralName = "DOC_EVENT"}}}\n'
            ),
        }
        rows = [
            row("C_Spell.SelectedCall", "global-api"),
            row("C_Item.SelectedCall", "global-api", number=2),
            row("C_Spell.CommentOnly", "global-api", number=3),
            row("DOC_EVENT", "events", number=4),
        ]
        result = self.overlay(rows, files)
        self.assertEqual(
            [r["ptr"]["applicability"] for r in result["entries"]],
            ["CONDITIONAL_APPLICABLE", "UNKNOWN", "UNKNOWN", "CONDITIONAL_APPLICABLE"],
        )
        self.assertEqual(
            result["entries"][0]["ptr"]["evidence"][0]["kind"], "api-documentation"
        )

    def test_authentication_rejects_missing_changed_and_unlisted_files(self):
        with tempfile.TemporaryDirectory() as directory:
            cache = Path(directory)
            body = (
                b"-- byte-preserving comment " + bytes([0x92]) + b"\nfunction A() end\n"
            )
            path = cache / "A.lua"
            path.write_bytes(body)
            index = {
                "files": [
                    {
                        "path": "A.lua",
                        "content_size": len(body),
                        "content_key": hashlib.md5(body).hexdigest(),
                    }
                ]
            }
            files = authenticate_files(index, "A.lua\n", cache)
            self.assertEqual(files["A.lua"].encode("latin-1"), body)
            path.write_bytes(body + b"-- changed")
            with self.assertRaisesRegex(ValueError, "A.lua: content mismatch"):
                authenticate_files(index, "A.lua\n", cache)
            path.unlink()
            with self.assertRaisesRegex(ValueError, "A.lua: missing"):
                authenticate_files(index, "A.lua\n", cache)
            with self.assertRaisesRegex(ValueError, "pin and manifest"):
                authenticate_files(index, "Other.lua\n", cache)

    def test_stale_pin_cannot_use_a_later_cache_or_promote_cells(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "data/blizzard-ui-builds").mkdir(parents=True)
            (root / "data/blizzard-ui-files").mkdir()
            (root / "data/blizzard-ui-builds/ptr.json").write_text(
                '{"version":"12.1.5.69594"}'
            )
            (root / "data/blizzard-ui-files/ptr.txt").write_text("A.lua\n")
            with self.assertRaisesRegex(ValueError, "exact I3 70493 pin/manifest"):
                load_pinned_files(root, root / "cache-not-read")

    def test_xml_cycles_stop_without_inventing_observations(self):
        files = {
            "Blizzard_A/Blizzard_A.toc": "A.xml\n",
            "Blizzard_A/A.xml": '<Ui><Include file="A.xml"/></Ui>',
        }
        result = self.overlay([row("NeverDefined")], files)
        self.assertEqual(result["entries"][0]["ptr"]["applicability"], "UNKNOWN")
        self.assertTrue(result["selection_gaps"])


if __name__ == "__main__":
    unittest.main()
