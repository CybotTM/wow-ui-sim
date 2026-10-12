"""Report observed dispatch intervals from original talent performance stderr.

Usage: python3 tools/talent_handler_log.py RAW_LOG_PATH
JSON describes logged attempts, not unique handlers or exclusive CPU/body time.
"""

import argparse
import json
import math
from pathlib import Path
import re
import sys


EXPECTED_COUNTS = {"baseline": 20, "settle": 10, "steady": 24}
MARKER = re.compile(
    r"\[talent-tick\] phase=([a-z]+) tick=(0|[1-9][0-9]*) (begin|end)"
)
HANDLER = re.compile(
    r"\[handler\] addon=(?P<addon>\S(?:.*?\S)?) "
    r"handler=(?P<handler>[A-Za-z_][A-Za-z0-9_]*) "
    r"frame=(?P<frame>\S(?:.*?\S)?) "
    r"duration_ms=(?P<duration>[0-9]+\.[0-9]{3})"
    r"(?: source=(?P<source>.+))?"
)
MEASUREMENT = {
    "count_unit": "observed logged attempted dispatch intervals",
    "all_attempts_verified": False,
    "completeness_condition": (
        "Counts cover all attempted dispatches only if the process started with "
        "WOW_SIM_LOG_HANDLER_TIMINGS=0 before OnceLock initialization; "
        "the raw log alone does not verify this condition."
    ),
    "interval_caveat": (
        "Intervals include helper and nested work, are rounded to three decimal "
        "milliseconds, and nested dispatches can double-count time. Sums are "
        "not exclusive body time; counts are not unique or top-level handlers."
    ),
    "performance_acceptance": False,
}


def parse_handler(line, line_number):
    match = HANDLER.fullmatch(line)
    if match is None or not math.isfinite(float(match["duration"])):
        raise ValueError(f"line {line_number}: malformed handler record: {line!r}")
    return {
        "addon": match["addon"], "handler": match["handler"],
        "frame": match["frame"], "source": match["source"],
        "interval_thousandths_ms": int(match["duration"].replace(".", "")),
    }


def validate_marker(line, line_number, position, expected_markers):
    match = MARKER.fullmatch(line)
    if match is None:
        raise ValueError(f"line {line_number}: malformed marker: {line!r}")
    marker = (match[1], int(match[2]), match[3])
    expected = expected_markers[position] if position < len(expected_markers) else None
    if marker != expected:
        raise ValueError(
            f"line {line_number}: unexpected marker {marker!r}; expected {expected!r}"
        )
    return marker


def new_bucket():
    return {"records": [], "other_line_count": 0}


def collect_windows(text, expected_markers):
    ticks, outside = [], new_bucket()
    active, position = None, 0
    for line_number, line in enumerate(text.splitlines(), 1):
        prefix = line.lstrip()
        if prefix.startswith("[talent-tick]"):
            phase, tick, edge = validate_marker(
                line, line_number, position, expected_markers
            )
            position += 1
            if edge == "begin":
                active = {"phase": phase, "tick": tick, **new_bucket()}
                ticks.append(active)
            else:
                active = None
        elif prefix.startswith("[handler]"):
            bucket = active if active is not None else outside
            bucket["records"].append(parse_handler(line, line_number))
        else:
            bucket = active if active is not None else outside
            bucket["other_line_count"] += 1
    if active is not None:
        raise ValueError(f"unclosed marker window: {active['phase']} tick={active['tick']}")
    if position != len(expected_markers):
        raise ValueError(f"missing expected marker: {expected_markers[position]!r}")
    return ticks, outside


def interval_stats(records):
    intervals = [record["interval_thousandths_ms"] for record in records]
    return {
        "count": len(intervals),
        "interval_total_ms": sum(intervals) / 1000,
        "interval_max_ms": max(intervals, default=0) / 1000,
    }


def summarize(records):
    kinds = {"OnUpdate": [], "OnPostUpdate": [], "other": []}
    for record in records:
        kind = record["handler"] if record["handler"] in kinds else "other"
        kinds[kind].append(record)
    return {
        **interval_stats(records),
        "by_handler_kind": {kind: interval_stats(items) for kind, items in kinds.items()},
    }


def group_handlers(records):
    groups = {}
    for record in records:
        key = tuple(record[field] for field in ("addon", "handler", "frame", "source"))
        groups.setdefault(key, []).append(record)
    rows = [
        dict(zip(("addon", "handler", "frame", "source"), key), **interval_stats(items))
        for key, items in groups.items()
    ]
    return sorted(rows, key=lambda row: (
        -row["interval_total_ms"], row["addon"], row["handler"],
        row["frame"], row["source"] or "",
    ))


def summarize_phases(ticks, expected_counts):
    phases = {}
    for phase in expected_counts:
        windows = [tick for tick in ticks if tick["phase"] == phase]
        records = [record for tick in windows for record in tick["records"]]
        phases[phase] = {
            "tick_count": len(windows), **summarize(records),
            "other_line_count": sum(tick["other_line_count"] for tick in windows),
            "grouped_handlers": group_handlers(records),
        }
    return phases


def parse_log(text, expected_counts=None):
    """Pure parser; explicit ordered counts support small behavioral fixtures.

    Global grouped_handlers includes outside-window records. Phase aggregates
    and ticks include only their matched windows; outside_markers is separate.
    """
    counts = dict(EXPECTED_COUNTS if expected_counts is None else expected_counts)
    expected_markers = [
        (phase, tick, edge) for phase, count in counts.items()
        for tick in range(count) for edge in ("begin", "end")
    ]
    ticks, outside = collect_windows(text, expected_markers)
    records = outside["records"] + [record for tick in ticks for record in tick["records"]]
    return {
        "expected_counts": counts, "measurement": dict(MEASUREMENT),
        "ticks": [
            {"phase": tick["phase"], "tick": tick["tick"],
             "other_line_count": tick["other_line_count"], **summarize(tick["records"])}
            for tick in ticks
        ],
        "phases": summarize_phases(ticks, counts),
        "all_logged_dispatches": summarize(records),
        "grouped_handlers": group_handlers(records),
        "outside_markers": {
            **summarize(outside["records"]),
            "other_line_count": outside["other_line_count"],
            "grouped_handlers": group_handlers(outside["records"]),
        },
        "other_line_count": outside["other_line_count"] + sum(
            tick["other_line_count"] for tick in ticks
        ),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("log_path", type=Path)
    args = parser.parse_args()
    try:
        report = parse_log(args.log_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, ValueError) as error:
        print(f"talent handler log: {error}", file=sys.stderr)
        return 1
    print(json.dumps(report, indent=2, allow_nan=False))
    return 0


if __name__ == "__main__":
    sys.exit(main())
