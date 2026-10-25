#!/usr/bin/env python3
"""I4c static dispositions; no runtime publication, retirement or native proof."""

import argparse
from collections import Counter, defaultdict
import json
from pathlib import Path
import re
import sys

import patch_12_1_7_static as base

OVERLAY = base.SOURCES / "12.1.7-inherited-i4c.json"
PROFILES = ("ptr", "retail", "wowforever")


def observe_assignments(text):
    _, code = base.tokenize_lua(text)
    locals_ = {"self"}
    for match in re.finditer(
        r"\blocal\s+(?:function\s+)?([A-Za-z_]\w*(?:\s*,\s*[A-Za-z_]\w*)*)", code
    ):
        locals_.update(re.split(r"\s*,\s*", match[1]))
    for match in re.finditer(
        r"\bfunction(?:\s+" + base.IDENTIFIER + r")?\s*\(([^)]*)\)", code
    ):
        locals_.update(re.findall(r"[A-Za-z_]\w*", match[1]))
    result = defaultdict(list)
    for match in re.finditer(
        r"(?<![\w.:])(" + base.IDENTIFIER + r")\s*=(?!=)\s*(" + base.IDENTIFIER + r")",
        code,
    ):
        symbol, value = match[1], match[2]
        if symbol.split(".")[0] not in locals_ and value not in {
            "nil",
            "false",
            "true",
        }:
            result[symbol].append(("lua-assignment", match.start()))
    return result


def collect_assignments(files, selected):
    result = defaultdict(list)
    for path, contexts in selected.items():
        for symbol, refs in observe_assignments(files[path]).items():
            for kind, offset in refs:
                line = files[path].count("\n", 0, offset) + 1
                for context in contexts:
                    result[symbol].append(
                        {
                            **base.source_edge(files, path, line),
                            **context,
                            "symbol": symbol,
                            "kind": kind,
                        }
                    )
    return result


def entry_targets(files, owner, raw):
    """Enumerate real pinned sibling paths, not guessed client capabilities."""
    path = re.sub(r"\s+\[.*", "", raw.replace("\\", "/")).strip()
    path = path.replace("[RegionalTextLocale]", "enUS").replace("[TextLocale]", "enUS")
    actual = path.replace("[Game]", "Standard").replace("[Family]", "Mainline")
    if "[Game]" not in path and "[Family]" not in path:
        target = base.resolve_path(files, owner, actual, owner)
        return [(target, False)] if target else []
    pattern = re.escape(owner + "/" + path)
    pattern = pattern.replace(re.escape("[Game]"), "[^/]+")
    pattern = pattern.replace(re.escape("[Family]"), "[^/]+")
    return [
        (candidate, candidate.lower() != (owner + "/" + actual).lower())
        for candidate in sorted(files)
        if re.fullmatch(pattern, candidate, re.I)
    ]


def rejected_sources(files):
    """Trace explicit flavor/game exclusions. Secure pass gaps are not exclusions."""
    chosen = {path.split("/")[0]: path for path in base.select_tocs(files)}
    rejected = defaultdict(list)
    for toc in sorted(path for path in files if path.lower().endswith(".toc")):
        owner = toc.split("/")[0]
        chosen_toc = chosen.get(owner)
        if chosen_toc is None:
            continue
        metadata, _ = base.parse_toc(files[toc])
        filters = " ".join(
            f"[{key} {metadata[key]}]"
            for key in ("AllowLoadGameType", "ExcludeLoadGameType")
            if key in metadata
        )
        for number, raw in enumerate(files[toc].split("\n"), 1):
            raw = raw.strip()
            if not raw or raw.startswith("#"):
                continue
            if toc != chosen_toc:
                reason = "TOC_FLAVOR"
            elif not base.allowed_game_type(filters):
                reason = "ADDON_GAME_TYPE"
            elif not base.allowed_game_type(raw):
                reason = "FILE_GAME_TYPE"
            elif not base.public_file_entry(raw, metadata):
                continue
            else:
                reason = None
            for target, alternate in entry_targets(files, owner, raw):
                exclusion = reason or ("GAME_FAMILY_BRANCH" if alternate else None)
                if exclusion is None:
                    continue
                context = {
                    "owner": owner,
                    "toc": toc,
                    "chosen_toc": chosen_toc,
                    "chosen_toc_sha256": base.sha256(
                        files[chosen_toc].encode("latin-1")
                    ),
                    "exclusion": exclusion,
                    "group": "unselected",
                }
                # The whole TOC hash covers addon metadata as well as the file edge.
                base.walk_source(
                    files,
                    target,
                    context,
                    [base.source_edge(files, toc, number)],
                    set(),
                    rejected,
                    [],
                )
    return rejected


def row_refs(row, evidence):
    refs = base.row_evidence(row, evidence)
    if row["section"] == "framexml":
        key = row["symbol"].replace(":", ".")
        refs = [
            ref
            for ref in evidence.get(key, [])
            if ref["kind"] in {"lua-declaration", "lua-assignment"}
        ]
    chosen = {}
    for ref in refs:
        chosen.setdefault((ref["kind"], ref["path"], ref["group"]), ref)
    return list(chosen.values())


def literal_paths(rows, files):
    wanted = {row["symbol"].replace(":", ".") for row in rows}
    found = defaultdict(set)
    for path, text in files.items():
        for match in re.finditer(base.IDENTIFIER, text):
            symbol = match[0].replace(":", ".")
            if symbol in wanted:
                found[symbol].add(path)
    return {symbol: sorted(paths) for symbol, paths in found.items()}


def runtime_question(row, reason):
    symbol = row["symbol"]
    if reason.startswith("REMOVED_") and row["section"] in {"cvars", "events"}:
        catalog = "CVar" if row["section"] == "cvars" else "event"
        return (
            f"Does the 70493 native {catalog} catalog omit the exact removed name {symbol}? "
            "Missing documentation or consumers cannot prove native deregistration. "
            "Inspect registration only; do not trigger events or change saved CVars."
        )
    if reason.startswith("REMOVED_"):
        return (
            f"After original PTR Game/Glue and owning LoD loads, is {symbol} absent from "
            "its literal public owner, or republished through dynamic assignment, alias or "
            "native registration? Observe raw lookup before/after load and explain any "
            "opposing occurrence; do not assert retirement from a missing source spelling."
        )
    if row["section"] == "widgets":
        return (
            f"Which native constructible object exposes the literal method {symbol}, "
            "and is raw lookup callable on that object in 70493? Map the documented "
            "ScriptObject owner without suffix matching; 3D effects stay unsupported."
        )
    if row.get("kind") == "command":
        return f"Does the 70493 typed command catalog register {symbol} as a command, not a CVar? Do not execute it."
    if row["section"] == "cvars":
        return (
            f"Does the 70493 CVar catalog register {symbol}? Record registration and "
            "default separately from current saved value; do not execute setters."
        )
    if row["section"] == "framexml":
        return (
            f"Does original selected PTR addon loading publish {symbol} via dynamic "
            "construction/alias or a currently unresolved source/environment path? "
            "Record owner, original load chain and raw value before/after loading."
        )
    if row["section"] == "events":
        return (
            f"Does 70493 register the exact event {symbol} in the native event surface? "
            "Registration does not prove payload or producer dispatch."
        )
    return (
        f"Does 70493 publish the exact raw member {symbol} on its literal native owner "
        "in Game/Glue or after original LoD loads despite no selected witness? "
        "Do not infer availability from an unselected consumer or a namespace suffix."
    )


def declaration_paths(files):
    declarations = defaultdict(set)
    for path, text in files.items():
        if not path.lower().endswith((".lua", ".xml")):
            continue
        for observations in (base.observe_lua(text), observe_assignments(text)):
            for symbol, refs in observations.items():
                if any(
                    kind in {"lua-declaration", "lua-assignment"} for kind, _ in refs
                ):
                    declarations[symbol].add(path)
    return declarations


def classify(row, selected_evidence, rejected_evidence, literals, declarations):
    refs = row_refs(row, selected_evidence)
    symbol = row["symbol"]
    state = {
        "applicability": "UNKNOWN",
        "evidence": refs,
        "literal_source_files": literals.get(symbol.replace(":", "."), []),
    }
    if refs and row["direction"] == "added":
        status, reason = "CONDITIONAL_APPLICABLE", "SELECTED_ADDITION"
    elif refs and row["direction"] == "removed" and row["section"] == "framexml":
        status, reason = "CONDITIONAL_REMOVAL_CONTRADICTED", "SELECTED_REMOVAL_SURVIVES"
    elif (
        row["section"] == "framexml"
        and (rejected := row_refs(row, rejected_evidence))
        and declarations[symbol.replace(":", ".")] <= {ref["path"] for ref in rejected}
    ):
        status = "CONDITIONAL_UNSELECTED_PUBLISHER"
        reason = rejected[0]["exclusion"]
        state["evidence"] = rejected
    else:
        status = "UNKNOWN"
        if row["direction"] == "removed":
            reason = (
                "REMOVED_LITERAL_PRESENT"
                if state["literal_source_files"]
                else "REMOVED_LITERAL_ABSENT"
            )
        elif row["section"] == "framexml":
            reason = (
                "UNRESOLVED_PUBLISHER_SELECTION"
                if state["literal_source_files"]
                else "ADDED_LITERAL_ABSENT"
            )
        elif row["section"] == "widgets":
            reason = "WIDGET_FACTORY_UNPROVED"
        elif row.get("kind") == "command":
            reason = "COMMAND_REGISTRATION_UNPROVED"
        elif row["section"] == "cvars":
            reason = "CVAR_REGISTRATION_UNPROVED"
        else:
            reason = "NATIVE_WITHOUT_SELECTED_WITNESS"
    state.update(
        applicability=status,
        reason_code=reason,
        runtime_question=runtime_question(row, reason) if status == "UNKNOWN" else None,
    )
    return state


def build_overlay(rows, files):
    selected, gaps = base.select_sources(files)
    selected_evidence = base.collect_evidence(files, selected)
    rejected = rejected_sources(files)
    rejected_evidence = base.collect_evidence(files, rejected)
    for table, contexts in [
        (selected_evidence, selected),
        (rejected_evidence, rejected),
    ]:
        for symbol, refs in collect_assignments(files, contexts).items():
            table[symbol].extend(refs)
    literals = literal_paths(rows, files)
    declarations = declaration_paths(files)
    entries = []
    for row in rows:
        entry = {
            "id": row["id"],
            "ptr": classify(
                row, selected_evidence, rejected_evidence, literals, declarations
            ),
        }
        for profile, version in [("retail", "12.1.5"), ("wowforever", "1.60.1")]:
            entry[profile] = {
                "applicability": "UNKNOWN",
                "reason_code": "INDEPENDENT_PROFILE_SOURCE_REQUIRED",
                "runtime_question": (
                    f"For {profile} {version}, which original selected TOC/XML/Lua "
                    f"chain or native owner decides {row['symbol']} ({row['direction']})? "
                    "Acquire that profile's source proof first; PTR is not transferable."
                ),
            }
        entries.append(entry)
    return {
        "schema": "patch-inherited-static/i4c-v1",
        "patch": "12.1.7",
        "dependency": base.DEPENDENCY,
        "runtime_complete": False,
        "proof_scope": "Conditional static literal-source applicability only; no native retirement, runtime publication, behavior or successful consumer execution",
        "source_binding": {
            "pin_sha256": base.PIN_HASH,
            "manifest_sha256": base.MANIFEST_HASH,
            "inherited_sha256": base.I0_HASHES[base.INHERITED.name],
        },
        "literal_absence_scope": "Exact identifier spelling in all authenticated pinned text, including comments/strings; no inference about dynamic publication, aliases or native registration",
        "selection_gaps": gaps,
        "entries": entries,
    }


def validate_overlay(overlay, rows, files):
    if overlay != build_overlay(rows, files):
        raise ValueError("I4c overlay differs from pinned evidence")


def summarize(overlay):
    counts = Counter(
        (profile, entry[profile]["applicability"], entry[profile]["reason_code"])
        for entry in overlay["entries"]
        for profile in PROFILES
    )
    return [
        {"profile": p, "applicability": a, "reason": r, "count": count}
        for (p, a, r), count in sorted(counts.items())
    ]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root", type=Path, default=Path(__file__).resolve().parents[1]
    )
    parser.add_argument("--source-root", type=Path, required=True)
    parser.add_argument("--cache", type=Path, required=True)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    try:
        base.validate_i0_bytes(args.root)
        rows = json.loads((args.root / base.INHERITED).read_text())["entries"]
        files = base.load_pinned_files(args.source_root, args.cache)
        result = build_overlay(rows, files)
        output = args.root / OVERLAY
        if args.write:
            output.write_text(json.dumps(result, indent=2) + "\n")
        elif json.loads(output.read_text()) != result:
            raise ValueError("I4c overlay differs from pinned evidence")
        print(
            json.dumps(
                {
                    "counts": summarize(result),
                    "authenticated_files": len(files),
                    "runtime_complete": False,
                },
                sort_keys=True,
            )
        )
    except (ValueError, OSError, KeyError, TypeError) as error:
        print(f"12.1.7 I4c static: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
