#!/usr/bin/env python3
"""Conditional I4b static evidence; never runtime publication or absence proof."""

import argparse
from collections import defaultdict
import hashlib
import json
from pathlib import Path, PurePosixPath
import posixpath
import re
import sys
import xml.etree.ElementTree as ET

from validate_patch_12_1_7_inherited import I0_HASHES, validate_i0_bytes

DEPENDENCY = "e30d17b106ce76a872c2bdfe69f95b4ec215fd06"
PIN_HASH = "e84a8de626166c53d7cccbe0cba65e141488f8cf315d4b03a36c4f181585da00"
MANIFEST_HASH = "ba4414999237ad7b50de73a2a61d7dd18fee31b2f2c3b70d5a21ad3cdc34d328"
SOURCES = Path("data/patch-api/sources")
INHERITED = SOURCES / "12.1.7-inherited-1.60.1.json"
OVERLAY = SOURCES / "12.1.7-inherited-static.json"
IDENTIFIER = r"[A-Za-z_]\w*(?:[.:][A-Za-z_]\w*)*"
# Long strings/comments must precede short comments and punctuation.
LUA_TOKEN = re.compile(
    r"(?P<longcomment>--\[(?P<ceq>=*)\[.*?\](?P=ceq)\])"
    r"|(?P<comment>--[^\n]*)"
    r"|(?P<longstring>\[(?P<seq>=*)\[.*?\](?P=seq)\])"
    r"""|(?P<string>"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*')"""
    r"|(?P<word>[A-Za-z_]\w*)|(?P<punct>[^\s])",
    re.DOTALL,
)


def sha256(value):
    return hashlib.sha256(value).hexdigest()


def allowed_game_type(text):
    def matches(value):
        return bool(set(re.split(r"[,\s]+", value.strip())) & {"mainline", "standard"})

    allow = re.search(r"\[AllowLoadGameType\s+([^]]*)]", text)
    exclude = re.search(r"\[ExcludeLoadGameType\s+([^]]*)]", text)
    return (not allow or matches(allow[1])) and not (exclude and matches(exclude[1]))


def parse_toc(text):
    metadata, entries = {}, []
    for number, raw in enumerate(
        text.removeprefix("\u00ef\u00bb\u00bf").split("\n"), 1
    ):
        line = raw.strip()
        if not line or not allowed_game_type(line):
            continue
        if line.startswith("##"):
            if any(
                var in line
                for var in (
                    "[Game]",
                    "[Family]",
                    "[TextLocale]",
                    "[RegionalTextLocale]",
                )
            ):
                continue  # PTR ignores the whole variable-bearing directive.
            match = re.match(r"##\s*([^:]+):\s*(.*)", line)
            if match:
                metadata[match[1].strip()] = re.sub(r"\s*\[.*", "", match[2]).strip()
        elif not line.startswith("#"):
            entries.append((number, line))
    return metadata, entries


def resolve_path(files, base, relative, addon):
    relative = relative.replace("\\", "/")
    if "Interface/AddOns/" in relative:
        candidates = [relative.split("Interface/AddOns/", 1)[1]]
    else:
        candidates = [posixpath.join(base, relative), posixpath.join(addon, relative)]
    for candidate in candidates:
        candidate = posixpath.normpath(candidate)
        if candidate in files:
            return candidate
        matches = sorted(path for path in files if path.lower() == candidate.lower())
        if matches:
            return matches[0]
    return None


def source_edge(files, path, line):
    return {
        "path": path,
        "line": line,
        "text": files[path].split("\n")[line - 1],
        "sha256": sha256(files[path].encode("latin-1")),
    }


def select_tocs(files):
    addons = sorted({path.split("/")[0] for path in files})
    result = []
    for addon in addons:
        for suffix in ("_Mainline", "-Mainline", "", "_Standard", "-Standard"):
            candidate = resolve_path(files, addon, addon + suffix + ".toc", addon)
            if candidate:
                result.append(candidate)
                break
    return result


def walk_source(files, path, context, chain, active, selected, gaps):
    if path in active:
        gaps.append({"path": path, "reason": "XML include cycle; no absence inference"})
        return
    if path.lower().endswith(".lua"):
        selected[path].append({**context, "chain": chain})
        return
    if not path.lower().endswith(".xml"):
        return
    try:
        tree = ET.fromstring(files[path].removeprefix("\u00ef\u00bb\u00bf"))
    except ET.ParseError:
        gaps.append({"path": path, "reason": "XML not parsed; no absence inference"})
        return
    for element in xml_file_elements(tree):
        target = resolve_path(
            files,
            str(PurePosixPath(path).parent),
            element.attrib["file"],
            context["owner"],
        )
        if target is None:
            gaps.append(
                {
                    "path": path,
                    "target": element.attrib["file"],
                    "reason": "include missing from pin",
                }
            )
            continue
        # The parsed edge identifies the tag, not a guessed line in a multiline XML tag.
        edge = {
            "path": path,
            "file_attribute": element.attrib["file"],
            "sha256": sha256(files[path].encode("latin-1")),
        }
        walk_source(
            files, target, context, chain + [edge], active | {path}, selected, gaps
        )


def xml_file_elements(tree):
    for element in tree:
        tag = element.tag.rsplit("}", 1)[-1].lower()
        if tag in {"script", "include"} and "file" in element.attrib:
            yield element
        elif tag == "scopedmodifier":
            yield from xml_file_elements(element)


def public_file_entry(raw, metadata):
    locale = re.search(r"\[AllowLoadTextLocale([^]]*)]", raw)
    if locale and "enUS" not in re.split(r"[,\s]+", locale[1].strip()):
        return False
    lower = raw.lower()
    if metadata.get("UseSecureEnvironment", "0") == "1":
        return False  # Public bootstrap/override ordering needs a later selected-pass proof.
    return (
        "[loadintoenvironment secure]" not in lower
        and "[allowloadenvironment secure]" not in lower
    )


def select_sources(files):
    selected, gaps = defaultdict(list), []
    for toc in select_tocs(files):
        metadata, entries = parse_toc(files[toc])
        filters = " ".join(
            f"[{key} {metadata[key]}]"
            for key in ("AllowLoadGameType", "ExcludeLoadGameType")
            if key in metadata
        )
        if not allowed_game_type(filters):
            continue
        allow = metadata.get("AllowLoad", "Game").lower()
        screens = (
            ["game", "glue"]
            if allow == "both"
            else ["glue" if allow == "glue" else "game"]
        )
        lod = metadata.get("LoadOnDemand", "0").lower() in {"1", "true"}
        owner = toc.split("/")[0]
        for number, raw in entries:
            if not public_file_entry(raw, metadata):
                gaps.append(
                    {
                        "path": toc,
                        "line": number,
                        "reason": "non-public environment or non-enUS file; no global witness",
                    }
                )
                continue
            target = raw.replace("[Game]", "Standard").replace("[Family]", "Mainline")
            target = target.replace("[RegionalTextLocale]", "enUS").replace(
                "[TextLocale]", "enUS"
            )
            target = re.sub(r"\s*\[.*", "", target).strip()
            path = resolve_path(files, owner, target, owner)
            if path is None:
                gaps.append(
                    {
                        "path": toc,
                        "line": number,
                        "target": target,
                        "reason": "TOC entry missing from pin",
                    }
                )
                continue
            for screen in screens:
                context = {
                    "owner": owner,
                    "toc": toc,
                    "screen": screen,
                    "load_on_demand": lod,
                    "group": "glue" if screen == "glue" else "lod" if lod else "game",
                }
                walk_source(
                    files,
                    path,
                    context,
                    [source_edge(files, toc, number)],
                    set(),
                    selected,
                    gaps,
                )
    unique_gaps = {json.dumps(gap, sort_keys=True): gap for gap in gaps}
    return selected, [unique_gaps[key] for key in sorted(unique_gaps)]


def tokenize_lua(text):
    tokens, masked = [], list(text)
    for match in LUA_TOKEN.finditer(text):
        kind = match.lastgroup
        value = match[0]
        if kind in {"comment", "longcomment", "string", "longstring"}:
            masked[match.start() : match.end()] = [
                "\n" if ch == "\n" else " " for ch in value
            ]
        if kind not in {"comment", "longcomment"}:
            tokens.append((kind, value, match.start()))
    return tokens, "".join(masked)


def observe_lua(text):
    tokens, code = tokenize_lua(text)
    observations = defaultdict(list)
    local_owners = set()
    for match in re.finditer(
        r"\blocal\s+(?:function\s+)?([A-Za-z_]\w*(?:\s*,\s*[A-Za-z_]\w*)*)", code
    ):
        local_owners.update(re.split(r"\s*,\s*", match[1]))
    for match in re.finditer(r"(?<![\w.:])(" + IDENTIFIER + r")\s*\(", code):
        symbol = match[1].replace(":", ".")
        if symbol.split(".")[0] in local_owners:
            continue
        before = code[: match.start()].rstrip()
        kind = "lua-declaration" if re.search(r"\bfunction$", before) else "lua-call"
        observations[symbol].append((kind, match.start()))
    for index in range(len(tokens) - 2):
        kind, name, start = tokens[index]
        if (
            kind != "word"
            or tokens[index + 1][1] != "("
            or tokens[index + 2][0] != "string"
        ):
            continue
        argument = tokens[index + 2][1][1:-1]
        if "\\" in argument:
            continue  # Do not infer escaped argument identities.
        if (
            name in {"RegisterEvent", "RegisterUnitEvent"}
            and index
            and tokens[index - 1][1] == ":"
        ):
            observations["event:" + argument].append(("event-consumer", start))
        if name in {"GetCVar", "GetCVarBool", "GetCVarDefault", "SetCVar"}:
            prefix = [token[1] for token in tokens[max(0, index - 2) : index]]
            if name not in local_owners and (
                not prefix or prefix[-1] not in {".", ":"} or prefix == ["C_CVar", "."]
            ):
                observations["cvar:" + argument].append(("cvar-consumer", start))
    return observations


def observe_documentation(text):
    observations = defaultdict(list)
    tokens, _ = tokenize_lua(text)
    fields = []
    for index in range(len(tokens) - 2):
        key, equals, value = tokens[index : index + 3]
        if key[0] == "word" and equals[1] == "=" and value[0] == "string":
            fields.append((key[1], value[1][1:-1], key[2], index))
    header_end = next(
        (
            offset
            for kind, value, offset in tokens
            if kind == "word" and value == "Functions"
        ),
        len(text),
    )
    header = {key: value for key, value, offset, _ in fields if offset < header_end}
    if header.get("Type") != "System":
        return observations  # ScriptObject docs do not prove constructible factories.
    namespace = header.get("Namespace", "")
    for key, value, offset, index in fields:
        if key == "Name" and [t[1] for t in tokens[index + 3 : index + 7]] == [
            ",",
            "Type",
            "=",
            '"Function"',
        ]:
            symbol = (namespace + "." if namespace else "") + value
            observations[symbol].append(("api-documentation", offset))
        if key == "LiteralName":
            observations["event:" + value].append(("event-documentation", offset))
    return observations


def collect_evidence(files, selected):
    evidence = defaultdict(list)
    for path in sorted(selected):
        text = files[path]
        observations = observe_lua(text)
        if path.startswith("Blizzard_APIDocumentationGenerated/") and path.endswith(
            "Documentation.lua"
        ):
            for symbol, refs in observe_documentation(text).items():
                observations[symbol].extend(refs)
        for symbol, refs in observations.items():
            for kind, offset in refs:
                line = text.count("\n", 0, offset) + 1
                for selection in selected[path]:
                    evidence[symbol].append(
                        {
                            **source_edge(files, path, line),
                            **selection,
                            "kind": kind,
                            "symbol": symbol,
                        }
                    )
    return evidence


def row_evidence(row, evidence):
    section, symbol = row["section"], row["symbol"]
    key = {"events": "event:", "cvars": "cvar:"}.get(section, "") + symbol.replace(
        ":", "."
    )
    refs = (
        evidence.get(key, [])
        if section != "widgets" and row.get("kind") != "command"
        else []
    )
    if section == "framexml":
        refs = [ref for ref in refs if ref["kind"] == "lua-declaration"]
    # One concrete witness per kind/screen/load group is enough; no coverage count from repetitions.
    chosen = {}
    for ref in refs:
        chosen.setdefault((ref["kind"], ref["group"]), {**ref, "symbol": symbol})
    return list(chosen.values())


def classify_row(row, evidence):
    refs = row_evidence(row, evidence)
    if row["direction"] == "removed":
        reason = (
            "UNKNOWN: selected spelling survives; resolve removal/republication/alias and opposing occurrences"
            if refs
            else "UNKNOWN: no selected witness; a missing lexical hit is not absence or retirement proof"
        )
    elif refs:
        return {
            "applicability": "CONDITIONAL_APPLICABLE",
            "reason": "Selected source obligation only, effective after I3 dependency merges; not runtime publication or behavior",
            "evidence": refs,
        }
    elif row["section"] == "widgets":
        reason = "UNKNOWN: ScriptObject docs alone do not establish literal owner and constructible factory; 3D effects remain unsupported"
    elif row.get("kind") == "command":
        reason = "UNKNOWN: typed command registration not established; allocator command not executed"
    else:
        reason = "UNKNOWN: no supported selected declaration/documentation/literal consumer witness; native registration and unresolved source paths not inferred"
    return {"applicability": "UNKNOWN", "reason": reason, "evidence": refs}


def build_overlay(rows, files):
    selected, gaps = select_sources(files)
    evidence = collect_evidence(files, selected)
    entries = []
    for row in rows:
        entries.append(
            {
                "id": row["id"],
                "ptr": classify_row(row, evidence),
                "retail": {
                    "applicability": "UNKNOWN",
                    "reason": "12.1.5 preservation; PTR evidence is not Retail selection proof",
                },
                "wowforever": {
                    "applicability": "UNKNOWN",
                    "reason": "Independent 1.60.1 source/selection proof required; PTR evidence is not transferable",
                },
            }
        )
    return {
        "schema": "patch-inherited-static/i4b-v1",
        "patch": "12.1.7",
        "dependency": DEPENDENCY,
        "activation": "only after p1217-source dependency merges; no effective promotions on the I4b base",
        "runtime_complete": False,
        "proof_scope": "static selected-source obligations only; I4a runtime proof stages remain UNKNOWN",
        "selection_gaps": gaps,
        "entries": entries,
    }


def validate_overlay(overlay, rows, files):
    if overlay != build_overlay(rows, files):
        raise ValueError(
            "static overlay differs from pinned selected evidence or dependency"
        )


def authenticate_files(index, manifest, cache):
    paths = manifest.splitlines()
    if paths != [entry["path"] for entry in index["files"]]:
        raise ValueError("pin and manifest paths differ")
    files, failures = {}, []
    for entry in index["files"]:
        path = entry["path"]
        if (
            PurePosixPath(path).is_absolute()
            or ".." in PurePosixPath(path).parts
            or "\\" in path
        ):
            raise ValueError("unsafe pinned path")
        try:
            body = (cache / path).read_bytes()
        except FileNotFoundError:
            failures.append(path + ": missing")
            continue
        if (
            len(body) != entry["content_size"]
            or hashlib.md5(body).hexdigest() != entry["content_key"]
        ):
            failures.append(path + ": content mismatch")
            continue
        # A byte-preserving view of ASCII Lua/XML/TOC syntax, not a locale decode.
        files[path] = body.decode("latin-1")
    if failures:
        raise ValueError("incomplete 70493 cache: " + "; ".join(failures))
    return files


def load_pinned_files(source_root, cache):
    pin = (source_root / "data/blizzard-ui-builds/ptr.json").read_bytes()
    manifest = (source_root / "data/blizzard-ui-files/ptr.txt").read_bytes()
    if sha256(pin) != PIN_HASH or sha256(manifest) != MANIFEST_HASH:
        raise ValueError("requires exact I3 70493 pin/manifest from " + DEPENDENCY)
    index = json.loads(pin)
    expected = {
        "profile": "ptr",
        "product": index["product"],
        "version": index["version"],
        "build_key": index["build_key"],
        "install_key": index["install_key"],
        "cdn_key": index["cdn_key"],
        "gethe_revision": index["gethe_revision"],
        "manifest_sha256": MANIFEST_HASH,
        "content_index_sha256": PIN_HASH,
        "source": "blizzard-cdn-pinned",
        "fallback": "none",
        "schema": "2",
    }
    provenance = dict(
        line.split("=", 1)
        for line in (cache / ".wow-ui-sim-blizzard-ui-provenance")
        .read_text()
        .splitlines()
    )
    if (
        provenance != expected
        or (cache / ".wow-ui-sim-blizzard-ui-complete").read_bytes() != b"ok\n"
    ):
        raise ValueError("cache provenance/completion differs from I3 70493")
    return authenticate_files(index, manifest.decode(), cache)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root", type=Path, default=Path(__file__).resolve().parents[1]
    )
    parser.add_argument(
        "--source-root", type=Path, required=True, help="I3 source checkout, read-only"
    )
    parser.add_argument(
        "--cache",
        type=Path,
        required=True,
        help="I3 authenticated AddOns root, read-only",
    )
    parser.add_argument(
        "--write",
        action="store_true",
        help="write the conditional companion instead of validating it",
    )
    args = parser.parse_args()
    try:
        validate_i0_bytes(args.root)
        rows = json.loads((args.root / INHERITED).read_text())["entries"]
        files = load_pinned_files(args.source_root, args.cache)
        result = build_overlay(rows, files)
        result["source_binding"] = {
            "pin_sha256": PIN_HASH,
            "manifest_sha256": MANIFEST_HASH,
            "inherited_sha256": I0_HASHES[INHERITED.name],
        }
        output = args.root / OVERLAY
        if args.write:
            output.write_text(json.dumps(result, indent=2) + "\n")
        elif json.loads(output.read_text()) != result:
            raise ValueError(
                "static overlay differs from pinned selected evidence or dependency"
            )
        resolved = sum(
            row["ptr"]["applicability"] == "CONDITIONAL_APPLICABLE"
            for row in result["entries"]
        )
        print(
            json.dumps(
                {
                    "conditional_ptr_resolved": resolved,
                    "effective_resolved_on_base": 0,
                    "conditional_unknown": len(rows) * 3 - resolved,
                    "dependency": DEPENDENCY,
                    "source_files_authenticated": len(files),
                    "selection_gaps": len(result["selection_gaps"]),
                    "runtime_complete": False,
                },
                sort_keys=True,
            )
        )
    except (ValueError, OSError, KeyError, TypeError) as error:
        print(f"12.1.7 inherited static: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
