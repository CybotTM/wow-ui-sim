#!/usr/bin/env python3
"""Validate the prose-only 12.1.7 ledger, not native/runtime API coverage."""
import argparse
import hashlib
import json
from pathlib import Path
import sys

from gen_patch_wikitext_register import parse_section, split_sections


SOURCES = Path("data/patch-api/sources")
LEDGER = SOURCES / "12.1.7-source-accounting.json"
PATCH_SOURCE = str(SOURCES / "12.1.7-api-changes.wikitext")
TOC_SOURCE = str(SOURCES / "12.1.7-toc-format.wikitext")
DIRECT_LOCATIONS = [
    (PATCH_SOURCE, [4]),
    *[(PATCH_SOURCE, [5, 8])] * 5,
    (PATCH_SOURCE, [6, 7, 8]),
    (PATCH_SOURCE, [9]),
    (PATCH_SOURCE, [10]),
    (TOC_SOURCE, [55, 56, 57, 512, 513]),
    (TOC_SOURCE, [46, 47, 49, 50, 51, 52, 53, 54]),
    (PATCH_SOURCE, [1, 13]),
]
DISCOVERY_FIELDS = {
    "ptr_applicability", "retail_applicability", "wowforever_applicability",
    "status", "candidate_file_lines",
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_json(root, path):
    return json.loads((root / path).read_text())


def validate_sources(root, ledger):
    provenance = read_json(root, ledger["provenance"])
    require(provenance["target"] == {
        "version": "12.1.7", "build": 70424, "interface": 120107,
    }, "target source identity must remain 12.1.7/70424/120107")
    sources = {}
    for source in provenance["sources"]:
        raw = (root / source["path"]).read_bytes()
        require(hashlib.sha256(raw).hexdigest() == source["sha256"]
                and len(raw) == source["bytes"], "source body identity mismatch")
        sources[source["path"]] = raw.decode("utf-8")
    return sources


def validate_direct(ledger, sources):
    rows = ledger["direct"]
    require([row["id"] for row in rows] == [f"P{i:02}" for i in range(1, 13)],
            "direct prose rows must retain P01–P12 exactly once in order")
    for number, (row, (source, lines)) in enumerate(zip(rows, DIRECT_LOCATIONS), 1):
        require(row["source"] == source and row["lines"] == lines and row["change"],
                f"direct prose source identity mismatch: {row['id']}")
        require(all(sources[source].splitlines()[line - 1].strip() for line in lines),
                f"direct prose source line missing: {row['id']}")
        expected_status = ("UNKNOWN until I4" if number == 1 else
                           "DEFERRED:" if number <= 7 else "PENDING I")
        require(row["status"].startswith(expected_status),
                f"direct prose status cannot claim runtime completion: {row['id']}")
    require(ledger["guid_decision"].startswith("DEFERRED pending user decision"),
            "GUID decision must remain explicitly deferred")


def validate_inherited(root, ledger, sources):
    link = ledger["inherited"]
    require(link["patch"] == "1.60.1" and link["status"] == "UNKNOWN until I4"
            and link["occurrences"] == 1876, "inherited linkage must remain UNKNOWN until I4")
    inherited = read_json(root, link["path"])
    expected_entries, expected_counts = [], []
    for section, lines in split_sections(sources[inherited["source"]]).items():
        entries, counts = parse_section(section, lines)
        expected_entries.extend(entries)
        expected_counts.extend(counts)
    require(len(expected_entries) == 1876, "inherited source must contain 1876 occurrences")
    require(inherited["header_counts"] == expected_counts, "inherited header counts mismatch")
    literal_entries = [{key: value for key, value in row.items()
                        if key not in DISCOVERY_FIELDS} for row in inherited["entries"]]
    require(literal_entries == expected_entries,
            "inherited occurrence identities differ from literal source rows")
    require(all(row["status"] == "UNKNOWN; not dynamically assessed"
                for row in inherited["entries"]), "inherited rows must remain UNKNOWN until I4")


def validate_index(root):
    index = read_json(root, "data/patch-api/patch-page-index.json")
    rows = [row for row in index["api_change_pages"]
            if row["title"] == "Patch 12.1.7/API changes"]
    require(len(rows) == 1 and rows[0].get("source_accounting") == str(LEDGER)
            and rows[0].get("audit_status") == "source-accounted-runtime-unproven",
            "patch index entry missing or not bound to source-only ledger")


def validate(root):
    ledger = read_json(root, LEDGER)
    require(ledger.get("schema") == "patch-source-accounting/v1"
            and ledger.get("patch") == "12.1.7", "source-accounting ledger required")
    require(ledger.get("coverage") == "source-accounting-only; runtime coverage incomplete",
            "source ledger cannot claim complete runtime coverage")
    sources = validate_sources(root, ledger)
    validate_direct(ledger, sources)
    validate_inherited(root, ledger, sources)
    validate_index(root)
    return {"patch": "12.1.7", "source_accounted": True, "runtime_complete": False,
            "direct_rows": 12, "inherited_unknown": 1876}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        result = validate(args.root)
    except (OSError, ValueError, KeyError, IndexError, TypeError) as error:
        print(f"12.1.7 source accounting: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
