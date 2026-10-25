#!/usr/bin/env python3
"""Validate immutable I4a accounting, never runtime or selected-source acceptance."""
import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import sys

from validate_patch_12_1_7_source import DISCOVERY_FIELDS, validate as validate_i0

SOURCES = Path("data/patch-api/sources")
MATRIX = SOURCES / "12.1.7-inherited-applicability.json"
INHERITED = SOURCES / "12.1.7-inherited-1.60.1.json"
BASE = "571284aaa8bb29f17d27e90110e4f863921d9330"
I0_HASHES = {
    "12.1.7-api-changes.provenance.json": "1d83fa103b00a6067b3fbdb1fb314d2c506a81c2e6f48bac38cc40f95b9e819c",
    "12.1.7-api-changes.wikitext": "8c7081afc2b249188f96e5032d7e8a78803a72953562406ab22845492d26c22c",
    "12.1.7-inherited-1.60.1.json": "85b7ace7bd50fa6496457f3146e6d1ac9b0cedea7da5ad8544a065fe35b00771",
    "12.1.7-inherited-1.60.1.wikitext": "470ec0cb303d5e5756b51f12b0a21d805cc5126cee044c80223b674c5f9f8941",
    "12.1.7-source-accounting.json": "5f8d0c8f57c4af12679b44976518077087ef2427f82ae69d3e864fa0f6133e70",
    "12.1.7-toc-format.wikitext": "1be73ae99d92d15cea78fd15d0cb754b0aa920e267ac79316e0ff1a6d3890e21",
}
# Bounded historical scope, not a glob over future patch additions.
SWEEP_PATCHES = (
    "3.3.3", "3.3.5", "4.0.1", "4.1.0", "5.0.4", "5.2.0", "6.0.2",
    "9.0.1", "10.0.2", "10.0.7", "11.1.0", "11.2.0", "12.0.0", "12.1.0",
)
PROFILES = {"ptr", "wowforever", "retail"}
STAGES = ("applicability", "publication", "behavior", "native", "original_consumer")
DOMAINS = {"global-api": "global-api", "framexml": "vendor-source",
           "widgets": "widget", "events": "event", "cvars": "cvar"}
FACTORY = ["p1217_inherited_factory_publication"]
PUBLISHERS = ["p1217_inherited_game_publication", "p1217_inherited_lod_publication",
              "p1217_inherited_glue_publication"]
NEGATIVE = ["p1217_inherited_negative_controls"]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_json(root, path):
    return json.loads((root / path).read_text())


def reject_private_paths(value):
    if isinstance(value, dict):
        for item in value.values():
            reject_private_paths(item)
    elif isinstance(value, list):
        for item in value:
            reject_private_paths(item)
    elif isinstance(value, str):
        require(not re.search(r"(?:^|\s)(?:/(?:home|tmp|var|syncthing)/|[A-Z]:\\|~/)", value),
                "private path in companion")


def validate_i0_bytes(root):
    for name, expected in I0_HASHES.items():
        actual = hashlib.sha256((root / SOURCES / name).read_bytes()).hexdigest()
        require(actual == expected, f"immutable I0 bytes changed: {name}")
    validate_i0(root)  # Reparse frozen raw occurrences, not counts alone.


def read_opposing_sweeps(root, literals):
    by_identity = defaultdict(list)
    for row in literals:
        by_identity[(row["section"], row["symbol"])].append(row)
    result = defaultdict(list)
    for patch in SWEEP_PATCHES:
        slug = patch.replace(".", "_")
        test = f"tests/patch_{slug}_publication_sweep.rs"
        register = str(SOURCES / f"{patch}-wikitext-register.json")
        source = (root / test).read_text()
        require('#![cfg(feature = "client-retail")]' in source,
                f"historical sweep profile changed: {test}")
        primary = re.search(r'(?:let register =|register:) include_str!\("\.\./([^\"]+)"\)', source)
        require(primary is not None and primary[1] == register,
                f"historical primary register changed: {test}")
        gaps = read_json(root, f"tests/data/patch_{slug}_sweep_known_gaps.json")
        for old in read_json(root, register)["entries"]:
            for row in by_identity[(old["section"], old["symbol"])]:
                require(old["direction"] != row["direction"], "historical direction changed")
                result[row["id"]].append({
                    "test": test, "register": register, "old_id": old["id"],
                    "direction": old["direction"], "profile": "retail",
                    "known_gap": old["id"] in gaps, "proof": "OPPOSING_DIRECTION_ONLY",
                })
    return result


def validate_profile(profile, state):
    require(set(state) == set(STAGES) | {
        "reason", "selected_source", "toc_chain", "selected_owner",
    }, "profile fields differ from I4a worklist")
    require(all(state[key] == "UNKNOWN" for key in STAGES),
            "I4a statuses must remain UNKNOWN; lexical/sweep evidence is not acceptance")
    require(state["selected_source"] is None and state["toc_chain"] == []
            and state["selected_owner"] is None,
            "selected-source evidence unavailable at I4a; unselected/wrong-build hits forbidden")
    require(isinstance(state["reason"], str) and state["reason"].startswith("UNKNOWN:"),
            "UNKNOWN requires concrete missing selection evidence")
    if profile == "ptr":
        require(all(pin in state["reason"] for pin in (
            "I3", "e30d17b106ce76a872c2bdfe69f95b4ec215fd06", "wowt 12.1.7.70493",
            "2cdbc7ff511ec6a8c2f4f8341accd1da44a3c9af", "TOC",
        )), "missing PTR pin or selection dependency")


def validate_test_refs(root, row, texts):
    symbol = row["source"]["symbol"]
    exact = re.compile(r"(?<![A-Za-z0-9_.:])" + re.escape(symbol) + r"(?![A-Za-z0-9_.:])")
    seen = set()
    for ref in row["existing_tests"]:
        require(set(ref) == {"path", "line", "symbol", "proof"}
                and ref["symbol"] == symbol and ref["proof"] == "LEXICAL_ONLY",
                "exact test identity required; no suffix or behavior credit")
        path = PurePosixPath(ref["path"])
        require(not path.is_absolute() and ".." not in path.parts
                and path.parts[0] in {"tests", "patch-tests"} and path.suffix == ".rs",
                "exact test identity needs repository-relative Rust test")
        if str(path) not in texts:
            texts[str(path)] = (root / path).read_text().splitlines()
        line = ref["line"]
        lines = texts[str(path)]
        require(isinstance(line, int) and 0 < line <= len(lines)
                and exact.search(lines[line - 1]) is not None,
                "exact test identity not present at referenced line")
        require(str(path) not in seen, "duplicate exact test identity")
        seen.add(str(path))


def validate_register_only_source(root):
    entries = read_json(root, SOURCES / "2.4.0-wikitext-register.json")["entries"]
    matches = [row for row in entries if row["section"] == "global-api"
               and row["symbol"] == "GetArmorPenetration"]
    require(len(matches) == 1 and matches[0]["id"] == "wt-global-api-GetArmorPenetration-29-1"
            and matches[0]["direction"] == "added", "historical register-only source changed")


def validate_row(root, row, literal, sweeps, texts):
    require(set(row) == {"id", "source", "domain", "literal_owner", "domain_limit",
                         "profiles", "discovery_candidates", "existing_tests",
                         "opposing_sweeps", "register_only", "proposed_selectors"},
            "unexpected I4a row fields")
    require(row["source"] == literal, f"literal occurrence changed: {row['id']}")
    domain = "command" if literal.get("kind") == "command" else DOMAINS[literal["section"]]
    require(row["domain"] == domain, f"domain differs from literal kind: {row['id']}")
    symbol = literal["symbol"]
    owner = re.split(r"[.:]", symbol)[0] if "." in symbol or ":" in symbol else None
    require(row["literal_owner"] == owner, "literal owner is not selected publisher proof")
    limit = ("permanent unsupported 3D effects; publication/factory unmeasured"
             if symbol.startswith("ModelSceneActorBase:") else None)
    require(row["domain_limit"] == limit, "permanent 3D boundary changed")
    require(set(row["profiles"]) == PROFILES, "PTR/Forever/Retail profiles required")
    for profile, state in row["profiles"].items():
        validate_profile(profile, state)
    validate_test_refs(root, row, texts)
    expected = sorted(sweeps[row["id"]], key=lambda ref: (ref["test"], ref["old_id"]))
    require(row["opposing_sweeps"] == expected, f"opposing sweep join changed: {row['id']}")
    expected_register = ([{
        "register": str(SOURCES / "2.4.0-wikitext-register.json"),
        "symbol": symbol, "old_id": "wt-global-api-GetArmorPenetration-29-1",
        "direction": "added", "proof": "HISTORICAL_REGISTER_ONLY",
    }] if literal["section"] == "global-api" and symbol == "GetArmorPenetration" else [])
    require(row["register_only"] == expected_register, "historical register-only join changed")
    selectors = PUBLISHERS if domain == "vendor-source" else FACTORY
    require(row["proposed_selectors"] == selectors + NEGATIVE,
            "grouped selectors are proposals, not applicability")


def validate(root):
    validate_i0_bytes(root)
    inherited = read_json(root, INHERITED)
    matrix = read_json(root, MATRIX)
    reject_private_paths(matrix)
    require(matrix["schema"] == "patch-inherited-applicability/i4a-v1"
            and matrix["patch"] == "12.1.7" and matrix["base_revision"] == BASE,
            "I4a source/base identity changed")
    require(matrix["source"] == str(INHERITED)
            and matrix["source_sha256"] == I0_HASHES[INHERITED.name], "source binding changed")
    require(matrix["i3_accepted"] is False and matrix["runtime_complete"] is False,
            "I4a cannot claim I3 or runtime acceptance")
    require(matrix["coverage"] == "immutable accounting and unresolved selected-source worklist only",
            "accounting is not applicability completion")
    require(matrix["header_counts"] == inherited["header_counts"], "header disagreements changed")
    originals = inherited["entries"]
    rows = matrix["entries"]
    require(Counter(row["id"] for row in rows) == Counter(row["id"] for row in originals),
            "occurrence IDs must match frozen source exactly once")
    original_by_id = {row["id"]: row for row in originals}
    literals = [{k: v for k, v in row.items() if k not in DISCOVERY_FIELDS} for row in originals]
    literal_by_id = {row["id"]: row for row in literals}
    sweeps = read_opposing_sweeps(root, literals)
    validate_register_only_source(root)
    texts = {}
    for row in rows:
        validate_row(root, row, literal_by_id[row["id"]], sweeps, texts)
        require(row["discovery_candidates"] == original_by_id[row["id"]]["candidate_file_lines"],
                "lexical discovery snapshot changed; not selected-source evidence")
    return {"patch": "12.1.7", "source_rows": len(rows), "source_accounted": True,
            "directions": dict(Counter(row["direction"] for row in literals)),
            "domains": dict(Counter(row["domain"] for row in rows)),
            "applicability_resolved": 0, "applicability_unknown": len(rows) * len(PROFILES),
            "opposing_sweep_intersections": sum(map(len, sweeps.values())),
            "runtime_complete": False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        result = validate(args.root)
    except (OSError, ValueError, KeyError, IndexError, TypeError) as error:
        print(f"12.1.7 inherited accounting: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
