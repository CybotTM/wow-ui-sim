#!/usr/bin/env python3
"""Generate Mists 5.5.4.70268 strings from its pinned CSV or decrypted DB2."""

import argparse
import csv
import hashlib
import json
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "data/db2/mists-5.5.4.70268/GlobalStrings.csv"
OUTPUT = ROOT / "crates/sim-data-profile/data/global_strings_mists.rs"
DB2_SHA256 = "e71f5424775e8911697db48e026ffbd25507a82902ba0a21a56ef40f9a6d36bf"
CONTENT_KEY = "7feb3e49aa1d9ca1e0253ca7837ef750"
COLUMNS = ["ID", "BaseTag", "TagText_lang", "Flags"]
RECORD_SIZE = 12


def read_sections(data):
    if len(data) < 404 or data[:4] != b"WDC5":
        raise ValueError("expected complete decrypted WDC5 GlobalStrings payload")
    header = struct.unpack_from("<9I2H7I", data, 136)
    rows, fields, record_size, string_size = header[:4]
    if (fields, record_size, header[9:]) != (3, RECORD_SIZE, (4, 0, 3, 8, 0, 72, 0, 28, 2)):
        raise ValueError("unsupported GlobalStrings record/storage layout")
    sections = [struct.unpack_from("<Q8I", data, 204 + index * 40) for index in range(2)]
    if sum(section[2] for section in sections) != rows:
        raise ValueError("GlobalStrings section row counts disagree with header")
    if sum(section[3] for section in sections) != string_size:
        raise ValueError("GlobalStrings section string sizes disagree with header")
    for _, offset, count, size, end, ids_size, relationships, offsets, copies in sections:
        if (end, ids_size, relationships, offsets, copies) != (0, count * 4, 0, 0, 0):
            raise ValueError("unsupported GlobalStrings section layout")
        if offset + count * RECORD_SIZE + size + ids_size > len(data):
            raise ValueError("truncated GlobalStrings section")
    return rows, sections


def read_relative_string(strings, relative, logical_field, record_bytes):
    if relative == 0:
        return ""
    offset = logical_field + relative - record_bytes
    if not 0 <= offset < len(strings):
        raise ValueError(f"GlobalStrings string offset outside string space: {offset}")
    end = strings.find(b"\0", offset)
    if end < 0:
        raise ValueError("unterminated GlobalStrings string")
    return strings[offset:end].decode("utf-8")


def decode_db2(data):
    row_count, sections = read_sections(data)
    strings = b"".join(
        data[offset + count * RECORD_SIZE:offset + count * RECORD_SIZE + size]
        for _, offset, count, size, *_ in sections
    )
    palette = struct.unpack_from("<7I", data, 368)
    rows = []
    for _, offset, count, size, *_ in sections:
        ids_offset = offset + count * RECORD_SIZE + size
        for index in range(count):
            tag, text, flag_index = struct.unpack_from("<iiI", data, offset + index * RECORD_SIZE)
            logical_record = len(rows) * RECORD_SIZE
            base_tag = read_relative_string(strings, tag, logical_record, row_count * RECORD_SIZE)
            tag_text = read_relative_string(strings, text, logical_record + 4, row_count * RECORD_SIZE)
            if flag_index >= len(palette):
                raise ValueError(f"invalid GlobalStrings flags palette index: {flag_index}")
            record_id = struct.unpack_from("<I", data, ids_offset + index * 4)[0]
            rows.append((record_id, base_tag, tag_text, palette[flag_index]))
    if len({row[0] for row in rows}) != row_count:
        raise ValueError("duplicate GlobalStrings record ID")
    return rows


def write_csv(rows, output):
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.writer(handle, lineterminator="\r\n")
        writer.writerow(COLUMNS)
        writer.writerows(rows)


def read_strings(source):
    # Same multiline-safe CSV algorithm as gen_forever_global_strings.py.
    strings = {}
    with source.open(encoding="utf-8", newline="") as handle:
        reader = csv.DictReader(handle)
        if reader.fieldnames != COLUMNS:
            raise ValueError(f"{source}: unexpected GlobalStrings columns")
        for row in reader:
            tag = row["BaseTag"]
            if not tag or tag in strings:
                raise ValueError(f"{source}: empty or duplicate tag {tag!r}")
            strings[tag] = row["TagText_lang"]
    return strings


def rust_string(value):
    return json.dumps(value, ensure_ascii=False).replace("\\u00", "\\x")


def generate(source, output):
    strings = read_strings(source)
    lines = [
        "//! Generated from Mists 5.5.4.70268 enUS GlobalStrings.csv (FDID 1394440).",
        "//! Regenerate: python3 tools/gen_mists_global_strings.py",
        "use phf::phf_map;",
        "pub fn get_global_string(name: &str) -> Option<&'static str> {",
        "    GLOBAL_STRINGS.get(name).copied()",
        "}",
        "pub static GLOBAL_STRINGS: phf::Map<&'static str, &'static str> = phf_map! {",
    ]
    lines.extend(f"    {rust_string(tag)} => {rust_string(value)}," for tag, value in sorted(strings.items()))
    lines.append("};")
    output.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Generated {len(strings)} strings: {output}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=SOURCE)
    parser.add_argument("--output", type=Path, default=OUTPUT)
    parser.add_argument("--db2", type=Path, help="verified, fully decrypted pinned DB2")
    args = parser.parse_args()
    if args.db2:
        data = args.db2.read_bytes()
        if hashlib.sha256(data).hexdigest() != DB2_SHA256 or hashlib.md5(data).hexdigest() != CONTENT_KEY:
            raise ValueError("DB2 does not match pinned Mists 5.5.4.70268 payload")
        write_csv(decode_db2(data), args.source)
    generate(args.source, args.output)


if __name__ == "__main__":
    main()
