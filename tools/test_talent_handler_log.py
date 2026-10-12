"""Behavioral fixtures for the talent dispatch-interval stderr report."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from tools.talent_handler_log import parse_log


COUNTS = {"baseline": 2, "settle": 1, "steady": 1}
CANONICAL = {"baseline": 20, "settle": 10, "steady": 24}
SCRIPT = Path(__file__).with_name("talent_handler_log.py")
UPDATE = (
    "[handler] addon=Blizzard_Class Talents handler=OnUpdate "
    "frame=Talent Frame duration_ms=0.123 source=@UI/Some File.lua:12"
)
POST = "[handler] addon=Blizzard_X handler=OnPostUpdate frame=Post duration_ms=0.200"
OTHER = "[handler] addon=Blizzard_X handler=OnEvent frame=Event duration_ms=1.000"


def window(phase, tick, *records):
    return "\n".join([
        f"[talent-tick] phase={phase} tick={tick} begin",
        *records,
        f"[talent-tick] phase={phase} tick={tick} end",
    ])


def complete_log(counts=COUNTS, records=None):
    records = records or {}
    return "\n".join(
        window(phase, tick, *records.get((phase, tick), []))
        for phase, count in counts.items()
        for tick in range(count)
    )


class ParserTests(unittest.TestCase):
    def test_repeated_dispatches_categories_phases_and_grouping(self):
        log = complete_log(records={
            ("baseline", 0): [UPDATE, UPDATE, POST, OTHER],
            ("baseline", 1): [UPDATE],
            ("steady", 0): [POST],
        })
        report = parse_log(log, COUNTS)
        self.assertIn("ticks", report)
        tick = report["ticks"][0]
        self.assertEqual((tick["phase"], tick["tick"]), ("baseline", 0))
        self.assertEqual(tick["count"], 4)
        self.assertEqual(tick["interval_total_ms"], 1.446)
        self.assertEqual(tick["interval_max_ms"], 1.0)
        self.assertEqual(tick["by_handler_kind"]["OnUpdate"], {
            "count": 2, "interval_total_ms": 0.246, "interval_max_ms": 0.123,
        })
        self.assertEqual(tick["by_handler_kind"]["OnPostUpdate"]["count"], 1)
        self.assertEqual(tick["by_handler_kind"]["other"]["count"], 1)
        self.assertEqual(report["phases"]["baseline"]["count"], 5)
        self.assertEqual(report["phases"]["baseline"]["interval_total_ms"], 1.569)
        self.assertEqual(report["phases"]["steady"]["interval_total_ms"], 0.2)
        self.assertEqual(report["phases"]["settle"]["count"], 0)
        self.assertEqual(report["all_logged_dispatches"]["count"], 6)
        groups = report["grouped_handlers"]
        self.assertEqual([group["handler"] for group in groups],
                         ["OnEvent", "OnPostUpdate", "OnUpdate"])
        self.assertEqual(groups[2]["count"], 3)
        self.assertEqual(groups[2]["addon"], "Blizzard_Class Talents")
        self.assertEqual(groups[2]["frame"], "Talent Frame")
        self.assertEqual(groups[2]["source"], "@UI/Some File.lua:12")
        self.assertEqual(groups[1]["source"], None)

    def test_outside_records_and_unparsed_lines_remain_visible(self):
        log = "\n".join([UPDATE, "startup message", complete_log(records={
            ("baseline", 0): ["unrelated diagnostic", UPDATE],
        }), POST, "", "shutdown message"])
        report = parse_log(log, COUNTS)
        self.assertIn("outside_markers", report)
        self.assertEqual(report["outside_markers"]["count"], 2)
        self.assertEqual(report["outside_markers"]["interval_total_ms"], 0.323)
        self.assertEqual(report["outside_markers"]["other_line_count"], 3)
        self.assertEqual(report["other_line_count"], 4)
        self.assertEqual(report["ticks"][0]["other_line_count"], 1)
        self.assertEqual(report["all_logged_dispatches"]["count"], 3)
        self.assertEqual(sum(g["count"] for g in report["grouped_handlers"]), 3)

    def test_zero_optional_source_and_source_entire_trailing_text(self):
        records = [
            "[handler] addon=Blizzard_X handler=OnUpdate frame=F duration_ms=0.000",
            "[handler] addon=Blizzard_X handler=OnUpdate frame=F duration_ms=0.000 source=@x:1 trailing text source=literal",
            "[handler] addon=Blizzard_X handler=OnUpdate frame=G duration_ms=0.000",
            "[handler] addon=Blizzard_Y handler=OnUpdate frame=F duration_ms=0.000",
        ]
        report = parse_log(complete_log(records={("baseline", 0): records}), COUNTS)
        self.assertIn("grouped_handlers", report)
        self.assertEqual(report["ticks"][0]["count"], 4)
        self.assertEqual(report["ticks"][0]["interval_total_ms"], 0.0)
        groups = report["grouped_handlers"]
        self.assertEqual(len(groups), 4)
        self.assertEqual([g["source"] for g in groups],
                         [None, "@x:1 trailing text source=literal", None, None])
        self.assertEqual(report, parse_log(complete_log(
            records={("baseline", 0): list(reversed(records))}), COUNTS))

    def test_measurement_limits_are_explicit(self):
        report = parse_log(complete_log(), COUNTS)
        self.assertIn("measurement", report)
        limits = report["measurement"]
        self.assertFalse(limits["all_attempts_verified"])
        self.assertEqual(limits["count_unit"], "observed logged attempted dispatch intervals")
        self.assertIn("WOW_SIM_LOG_HANDLER_TIMINGS=0", limits["completeness_condition"])
        self.assertIn("OnceLock", limits["completeness_condition"])
        self.assertIn("nested", limits["interval_caveat"])
        self.assertIn("rounded", limits["interval_caveat"])
        self.assertIn("not exclusive", limits["interval_caveat"])
        self.assertFalse(limits["performance_acceptance"])

    def test_rejects_malformed_handler_records(self):
        malformed = [
            "[handler]", UPDATE.replace("addon=", "owner="),
            UPDATE.replace("handler=OnUpdate", "handler="),
            UPDATE.replace("frame=Talent Frame", "frame="),
            UPDATE.replace("addon=Blizzard_Class Talents", "addon="),
            POST + " trailing", POST + " source=", " " + POST,
        ]
        for record in malformed:
            with self.subTest(record=record), self.assertRaisesRegex(ValueError, "line 2.*handler"):
                parse_log(complete_log(records={("baseline", 0): [record]}), COUNTS)

    def test_rejects_noncanonical_or_nonfinite_duration(self):
        durations = ["-0.001", "-0.000", "NaN", "inf", "1e3", "+0.123",
                     "1", ".123", "1.12", "1.1234", "9" * 400 + ".000"]
        for duration in durations:
            record = POST.replace("0.200", duration)
            with self.subTest(duration=duration), self.assertRaisesRegex(ValueError, "line 2.*handler"):
                parse_log(complete_log(records={("baseline", 0): [record]}), COUNTS)

    def test_rejects_malformed_markers(self):
        malformed = [
            "[talent-tick]", "[talent-tick] phase=baseline tick=0 start",
            "[talent-tick] phase=baseline tick=-1 begin",
            "[talent-tick] phase=baseline tick=00 begin",
            "[talent-tick] phase=baseline tick=0 begin trailing",
            " [talent-tick] phase=baseline tick=0 begin",
        ]
        for marker in malformed:
            with self.subTest(marker=marker), self.assertRaisesRegex(ValueError, "line 1.*marker"):
                parse_log(marker + "\n" + complete_log(), COUNTS)

    def test_rejects_wrong_order_duplicate_missing_or_overlapping_windows(self):
        valid = complete_log()
        bad_logs = [
            window("baseline", 1) + "\n" + valid,
            window("steady", 0) + "\n" + valid,
            window("unknown", 0) + "\n" + valid,
            window("baseline", 0) + "\n" + valid,
            valid.replace(window("baseline", 1) + "\n", ""),
            valid + "\n" + window("steady", 0),
            valid.replace("phase=baseline tick=0 end", "phase=baseline tick=1 end", 1),
            valid.replace("phase=baseline tick=0 end", "phase=settle tick=0 end", 1),
            valid.replace("phase=baseline tick=0 end", "phase=baseline tick=0 begin", 1),
            "[talent-tick] phase=baseline tick=0 end\n" + valid,
        ]
        for log in bad_logs:
            with self.subTest(log=log), self.assertRaisesRegex(ValueError, "line .*marker"):
                parse_log(log, COUNTS)

    def test_rejects_incomplete_or_unclosed_windows(self):
        for log in ["", window("baseline", 0),
                    complete_log().removesuffix("[talent-tick] phase=steady tick=0 end")]:
            with self.subTest(log=log), self.assertRaisesRegex(ValueError, "missing|unclosed"):
                parse_log(log, COUNTS)

    def test_rejects_bad_record_even_outside_markers(self):
        with self.assertRaisesRegex(ValueError, "line 1.*handler"):
            parse_log("[handler] broken\n" + complete_log(), COUNTS)


class CliTests(unittest.TestCase):
    def run_cli(self, log):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "raw stderr.log"
            path.write_text(log, encoding="utf-8")
            return subprocess.run([sys.executable, str(SCRIPT), str(path)],
                                  text=True, capture_output=True, check=False)

    def test_cli_real_file_full_canonical_54_windows_json(self):
        result = self.run_cli(complete_log(CANONICAL, {("steady", 23): [UPDATE]}))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, "")
        report = json.loads(result.stdout)
        self.assertIn("ticks", report)
        self.assertEqual(len(report["ticks"]), 54)
        self.assertEqual(report["ticks"][-1]["phase"], "steady")
        self.assertEqual(report["ticks"][-1]["tick"], 23)
        self.assertEqual(report["ticks"][-1]["count"], 1)
        self.assertEqual(report["expected_counts"], CANONICAL)

    def test_cli_rejects_bad_record_and_noncanonical_counts(self):
        for log in ["[handler] broken\n" + complete_log(CANONICAL), complete_log()]:
            with self.subTest(log=log):
                result = self.run_cli(log)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(result.stdout, "")
                self.assertTrue(result.stderr.strip())
                self.assertNotIn("Traceback", result.stderr)


if __name__ == "__main__":
    unittest.main()
