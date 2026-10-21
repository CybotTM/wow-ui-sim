#!/usr/bin/env python3
"""Extract the 12340 enUS auction catalog from read-only, authenticated WDBC files."""

import argparse
import hashlib
import json
from pathlib import Path
import struct

# Native AuctionHouse_Scripts.cpp:29-43. Only the final selector has no subclasses.
AUCTION_CLASS_IDS = (2, 4, 1, 0, 16, 7, 6, 11, 9, 3, 15, 12)
SOURCES = {
    "ItemClass.dbc": {
        "sha256": "f14b09254010765ab75c3c04ddf55a4b03e75163655c5015d188b04e31fa1b20",
        "records": 17, "fields": 20, "archive": "Data/enUS/patch-enUS.MPQ",
    },
    "ItemSubClass.dbc": {
        "sha256": "ed037619b6277fb2b05c3cf05233b19687a4004427d5e63a12d038804bc8b0e5",
        "records": 119, "fields": 44, "archive": "Data/enUS/patch-enUS-3.MPQ",
    },
}


def read_wdbc(path, source):
    raw = path.read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    if digest != source["sha256"]:
        raise ValueError(f"{path}: SHA256 {digest}, expected {source['sha256']}")
    magic, count, fields, stride, string_size = struct.unpack_from("<4s4I", raw)
    if magic != b"WDBC" or count != source["records"] or fields != source["fields"]:
        raise ValueError(f"{path}: unexpected WDBC header {(magic, count, fields)}")
    records_end = 20 + count * stride
    if stride != fields * 4 or len(raw) != records_end + string_size:
        raise ValueError(f"{path}: record/string lengths do not match the file")
    records = list(struct.iter_unpack(f"<{fields}I", raw[20:records_end]))
    return records, raw[records_end:]


def read_string(block, offset):
    if offset >= len(block):
        raise ValueError(f"string offset {offset} exceeds block size {len(block)}")
    end = block.find(b"\0", offset)
    if end < 0:
        raise ValueError(f"unterminated string at offset {offset}")
    return block[offset:end].decode("utf-8")


def project_classes(records, strings):
    return [
        {"class_id": row[0], "name": read_string(strings, row[3])}
        for row in records
        if row[0] in AUCTION_CLASS_IDS
    ]


def project_subclasses(records, strings):
    # Iteration preserves physical DB order, including hidden rows. Never sort IDs.
    # Both names use enUS columns; only the native verbose/display rule applies.
    return [
        {
            "class_id": row[0],
            "flags": row[4],
            "display_flags": row[5],
            "name": read_string(strings, row[27]) or read_string(strings, row[10]),
        }
        for row in records
        if row[0] in AUCTION_CLASS_IDS[:-1]
    ]


def derive_catalog(directory):
    classes, names = read_wdbc(directory / "ItemClass.dbc", SOURCES["ItemClass.dbc"])
    subclasses, sub_names = read_wdbc(
        directory / "ItemSubClass.dbc", SOURCES["ItemSubClass.dbc"]
    )
    return {
        "selectors": [
            {"class_id": value, "has_subclasses": index != len(AUCTION_CLASS_IDS) - 1}
            for index, value in enumerate(AUCTION_CLASS_IDS)
        ],
        "classes": project_classes(classes, names),
        "subclasses": project_subclasses(subclasses, sub_names),
    }


def build_provenance(encoded):
    return {
        "build": 12340,
        "locale": "enUS",
        "client_evidence": "wow-data and wow-enus component manifests both version 12340",
        "sources": SOURCES,
        "layout_status": "established by laptop clean-room record-loader reading",
        "layout_sources": [
            "ItemClassRec.cpp:21-40,66",
            "ItemSubClassRec.cpp:25-68,118-119",
            "AuctionHouse_Scripts.cpp:29-43,98,124-127,160-163",
        ],
        "zero_based_fields": {
            "ItemClass": {"class_id": 0, "flags": 2, "enUS_name": 3},
            "ItemSubClass": {
                "class_id": 0, "subclass_id": 1, "flags": 4, "display_flags": 5,
                "enUS_display_name": 10, "enUS_verbose_name": 27,
            },
        },
        "record_sizes": {"ItemClass": 80, "ItemSubClass": 176},
        "string_block_sizes": {"ItemClass": 158, "ItemSubClass": 1226},
        "name_projection": "nonempty enUS verboseName, else enUS displayName; no other locale",
        "subclass_order": "physical WDBC order, never sorted subclass IDs",
        "consumable_subclass_ids_in_order": [0, 5, 1, 2, 3, 7, 6, 4, 8],
        "scope": "auction selector classes only; excludes raw DBCs and unused columns",
        "catalog_sha256": hashlib.sha256(encoded).hexdigest(),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("dbc_directory", type=Path)
    parser.add_argument(
        "--output-directory", type=Path,
        default=Path(__file__).resolve().parents[1] / "data",
    )
    args = parser.parse_args()
    catalog = derive_catalog(args.dbc_directory)
    encoded = (json.dumps(catalog, indent=2, ensure_ascii=False) + "\n").encode("utf-8")
    args.output_directory.mkdir(parents=True, exist_ok=True)
    output = args.output_directory / "wrath-auction-catalog.json"
    output.write_bytes(encoded)
    metadata = args.output_directory / "wrath-auction-catalog.provenance.json"
    metadata.write_text(json.dumps(build_provenance(encoded), indent=2) + "\n", encoding="utf-8")
    print(f"{output}: {len(catalog['classes'])} classes, {len(catalog['subclasses'])} subclass rows")
    print(f"SHA256 {hashlib.sha256(encoded).hexdigest()}")


if __name__ == "__main__":
    main()
