# Independent cached-prefix / primary-source / retention gate

**PASS — cached coherence, exact retention and primary-source limits ONLY.**

Revision: `3092985a5b90c68b28e1fcc5a88d00b14ebe3e6d`. No native/archive authentication, root-cause or repair acceptance. Frozen ba **FAIL**, full handoff **OPEN** remain unchanged.

## Cached recomputation

All computation used retained JSON, not game files.

| Check | Independent cached result |
|---|---|
| Prefix | 34 bytes; SHA256 `2a1942d08f21c22eb45441ed55e2ae2e5dc62069f2fc15848826a8770e155532` matches |
| Reversed first16 | `53dcc8beeb4c9e75b600000000000000`; first9 match requested `53dcc8beeb4c9e75b69c5f2a8a67c7e4`; tail7 zero; full key differs |
| Size bytes | LE 187,471,535 equals retained entry; BE 2,945,854,475 |
| Magic | BLTE at30..34, not0..4; absolute start849,023,666 |
| Packed entry | 18 bytes; location `2bb29b1294`; high43, low word `b29b1294`; `(43<<2)|(word>>30)` =174; `word&0x3fffffff` =849,023,636; final4 LE =187,471,535 |
| Snapshot bounds | Prefix [849,023,636,849,023,670); candidate [849,023,636,1,036,495,171); both inside main-reported1,073,670,372 bytes; remaining37,175,201. BE-interpreted end3,794,878,111 exceeds snapshot bounds, not an archive-invalidity finding |

Archive11186 diagnosis rejected by retained packed decoding. Current existing candidate is reported by main; this gate does not independently prove physical existence or effective selection.

## Primary text inspected

Pinned cascette `758d8db`, under `/home/osso/.cargo/git/checkouts/cascette-rs-1b72b53a71929177/758d8db/crates/cascette-client-storage/`:

- `src/index/mod.rs:34–56,146–169,209–229`: location decoder, 9-byte key, LE encoded size and packed reader.
- `src/storage/local_header.rs:1–115`: helper represents reversed16-byte key and BE size, with LE flags/checksums. This is source representation, **not authoritative native header semantics**.
- `src/storage/archive_file.rs:170–218,267–327`: raw range bound and direct entry offset; header-plus-BLTE writer; magic-at30 skip without local-field validation.

Pinned asset-resolver `462097b`, `/home/osso/.cargo/git/checkouts/asset-resolver-b5f2cd5846cc4706/462097b/src/casc_resolver.rs:189–222,569–607`: lookup → read_raw with index archive/offset/size → magic-at30 → slice from30 → BLTE parser/decompression. Neither inspected path validates local header key, size, flags or checksums. Full-key/BE-size differences therefore **do not prove invalid archive, failure cause, or justified repair**. No payload hash was checked.

`source-seam.md` is retained prior source-only proposal, including its then-unperformed64-byte probe and key-mismatch case. README/interpretation.json expressly qualify it. It is not a new physical observation, native contract or stale-index diagnosis.

## Retention and documentation

Four source/current/committed exact byte comparisons and manifest byte/hash checks PASS:

| Copy | Bytes | SHA256 |
|---|---:|---|
| header-observation.json |1184|`ad148a82d24ebf70591b9eed72db66be253e1a709d9460757c3d6d94483ff81e`|
| interpretation.json |1264|`aba559fdaaa905b681511b9c9f6ff3a161e231e0716229006d302d877614e687`|
| packed-entry-disposition.json |379|`745bce8d31c5366903b73bf785b4025028d043c73e488ec796c927940ad51f36`|
| source-seam.md |3612|`fec12cb0a36d7db151a6a151067d908b5c5497b1657d0c38421614531190d376`|

**4/4 copies, 6,439 bytes total.** `proof.json` records exact source paths. Git committed-text retrieval exit0 for each.

Commit changes exactly10 paths: packet README/header-observation/interpretation/packed-entry-disposition/retention-manifest/source-seam plus `docs/wiki/index.md`, `docs/wiki/log.md`, `docs/wiki/investigations/fullsuite-ba-2026-10-11.md`, `docs/wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md`. **10/10 current bytes equal committed bytes**, no production/vendor changes in this commit. Six packet links plus five added documentation links resolve; both CASC section-anchor links match the target heading. Unrelated linked talent artifact checked for link existence only, not authenticated.

Changed prose preserves rejected11186 diagnosis, bounded main observation, source-field nonvalidation, effective/historical/native/decode limits and broad FAIL/full handoff OPEN. Pending gate markers remain historical pending statements at the audited commit; verifier does not edit repository status/docs. Initial Git status contained untracked `.code-index.db` and `verification/`; no tracked modifications reported.

## Boundaries

ZERO verifier reads of actual game .idx/archive/payloads; ZERO physical stat/read-count reauthentication. Archive bytes/mtime/inode, `read_calls=1`, returned34 and unchanged-stat claims are **main snapshot metadata only**. ZERO binary/library artifact reads, runtime/test execution, production operations or delegation; Git was used solely for committed text/path metadata.

Historical/effective index/L0 selection, historical product/build/install linkage, payload checksum/hash/decrypt/decode and cold-cache preparation success remain **UNPROVED**. Prior suite counts/metrics/talent/executables were not reauthenticated. Only `report.md` and `proof.json` were written in the requested private output directory.

No root repair or native PASS. **Broad ba FAIL / full handoff OPEN.**
