# CASC encoding-key local-header seam

**Scope:** bounded read-only source inspection only. Read prior handoffs and pinned `archive_file.rs`, `local_header.rs`, and the asset-resolver lookup/read function. No index/archive/content reads, acquisition, cache mutation, network, tests, compilation, operations, or delegation. No current candidate is treated as historical effective lookup.

## Result

Prior handoffs do not capture the local-header seam, so this source-only follow-up is not `NONE`.

The retained candidate is archive 174, offset 849,023,636, encoded size 187,471,535; archive size is reported as 1,073,670,372. Its offset is the **local entry header address**, not the BLTE start, under the pinned cascette archive writer/reader format: `write_content_with_mode` writes a 30-byte header followed by BLTE and returns the entry offset and total size including header; `read_content` reads from that offset and checks BLTE at byte 30. The asset-resolver path separately reads raw bytes starting at the index offset and strips 30 bytes only when bytes 30..34 equal `BLTE`.

A single 64-byte read at archive-relative offset **849,023,636** would cover the full header and BLTE magic at bytes 30..33. This is sufficient to discriminate these cases:

- BLTE magic at bytes 30..33, header key (after reversing stored bytes) differs from the requested key: candidate key mismatch; the BLTE payload is present at +30.
- BLTE magic absent at bytes 30..33: candidate does not have the expected local-header-plus-BLTE prefix; inspect header fields for invalid/other format, but this alone does not prove why the historical lookup failed.
- Header key matches and BLTE magic present: this candidate's prefix is consistent; it does not prove historical effective selection or successful decode/key availability.

No such read was performed; main owns any subsequent bounded read.

## Exact source layout and checks

Pinned cascette source: `crates/cascette-client-storage/src/storage/local_header.rs` and `storage/archive_file.rs`, revision `758d8db`.

- Header size: `0x1E` (30) bytes.
- `0x00..0x10`: 16-byte encoding key, stored reversed; `original_encoding_key()` reverses it back.
- `0x10..0x14`: total size including header, big-endian `u32`.
- `0x14..0x16`: flags, little-endian `u16`.
- `0x16..0x1A`: ChecksumA, little-endian `u32`.
- `0x1A..0x1E`: ChecksumB, little-endian `u32`.
- `read_content` recognizes BLTE at `offset + 30`; writer emits header then BLTE, with header size including itself.
- The inspected reader does not validate the header key, size, flags, or checksums before its BLTE-prefix check. Therefore the 64-byte probe can reveal values and prefix placement but cannot label nonzero checksums/flags invalid by this implementation's checks.

Pinned asset-resolver source: `src/casc_resolver.rs`, revision `462097b`. `read_file_by_encoding_key_with_keys` obtains `lookup(encoding_key)`, calls `read_raw(archive_id, archive_offset, entry.size)`, then tests for `BLTE` at `[30..34]` and passes bytes from `[30..]` to BLTE parsing; otherwise it treats the raw entry as BLTE bytes. This confirms the index offset is passed directly as the raw-entry start, with local-header skipping conditional on magic. It does not show which historical index entry was effective.

## Limit

The header-key comparison checks the candidate's own recorded key against the requested key; it is not a cryptographic verification that the payload hashes to that key. No producer repair, native contract, historical lookup, or broad-failure conclusion is asserted. Preserve broad FAIL and the full handoff OPEN.
