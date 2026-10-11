# CASC encoding candidate — current prefix evidence

Main observed exactly **34 bytes in one read**, 2026-10-11T23:46:39Z, from existing `/syncthing/World of Warcraft/Data/data/data.174` at offset849,023,636. No new index reads, payload/decode, acquisition or cache mutation. [Observation](header-observation.json), [cached interpretation](interpretation.json), [packed-entry correction](packed-entry-disposition.json), [source seam](source-seam.md), [exact-copy manifest](retention-manifest.json). Independent2463 [gate](report.md)/[proof](proof.json) PASS for cached coherence, primary-source limits and initial4-copy retention only; not independent archive authentication. Report/proof follow-up copies checked by main.

| Current candidate check | Observation |
|---|---|
| Archive/range | 1,073,670,372-byte file; offset + candidate187,471,535 bytes within it |
| Magic | `BLTE` at entry+30 |
| Reversed key | First9 bytes match requested53dcc8beeb4c9e75b69c5f2a8a67c7e4; remaining7 bytes zero, so full16-byte comparison differs |
| Size bytes | Little-endian187,471,535 matches retained index; pinned LocalHeader big-endian view2,945,854,475 does not |

Pinned asset-resolver `462097b::read_file_by_encoding_key_with_keys` checks magic at+30 and skips30 bytes; it does **not** validate those local header key/size/flags/checksum fields. Therefore the differences do not establish an invalid candidate, failure cause or required dependency repair. Header-key comparison is not payload-hash validation. `source-seam.md` describes the pinned helper layout, not authoritative native header semantics; `interpretation.json` owns this observed candidate's disposition.

Retained packed location bytes decode as archive174/offset849,023,636, not11186. The old missing-archive11186 diagnosis is rejected. Prior index investigation exceeded its32MiB cap; that task stays closed and no index bytes were reread here.

Effective lookup/L0 updates, selected product/build/install during the failed test, payload hashes/checksums and decrypt/decompress remain unproved. Current file/prefix state cannot backdate historical input identity or repair cold-cache preparation. Frozen [ba result](../../../../docs/wiki/investigations/fullsuite-ba-2026-10-11.md) remains **FAIL**; full handoff **OPEN**. No archive bytes beyond header/magic, executable, library or vendor Lua payload retained.
