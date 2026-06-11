# CASC Root v2 Misparsing Dropped 89% of FDIDs (Missing Textures)

The dispel-type debuff border ("blue swirly", atlas `ui-debuff-border-magic-icon`) never rendered because its texture `interface/hud/uidebuffframes.blp` (fdid 7553349) could not be extracted from CASC. Root cause was not in wow-ui-sim at all: the pinned cascette-rs rev misparsed the 12.0.5 TSFM v2 root file, silently dropping ~89% of all fdid→content-key records. Any texture whose fdid lived in the lost range was unextractable, and the failure was masked by `.missing` markers and silent error recovery.

## Content

### Symptoms

- `AuraUtil.SetAuraBorderAtlasFromAura(border, aura, true)` correctly set `ui-debuff-border-magic-icon`, and `DEBUFF_TYPE_*_COLOR` globals were correct (commit 752069489), but nothing rendered.
- `asset-cache extraction failed: ... Path not found in root file: interface/hud/uidebuffframes.blp`
- Entire `~/.cache/wow-ui-sim/casc-extract/interface/hud/` was `.blp.missing` markers (55/57 files) — 904 markers repo-wide. Old textures (e.g. `interface/buttons/`) extracted fine.

### Root cause chain

1. **Resolution cache far too small.** `resolution.sqlite` for build `399d19713d...` held 346,277 entries; the root header declares `total_files = 3,191,148` (`named_files = 249,141`, i.e. only ~8% of files have name hashes in 12.0.5).
2. **cascette-rs `0d0e79a` v2 block header misparse.** The 17-byte v2 block header is `num_records u32, locale_flags u32, cf1 u32, cf2 u32, cf3 u8` where content flags are split: `content_flags = cf1 | cf2 | (cf3 << 17)`. The old code read only `cf1` as content flags (treating `cf2`/`cf3` as unknowns) and defined `NO_NAME_HASH = 0x2000` instead of the standard TACT bit `0x10000000` — which lives in `cf2` (observed `cf2 = 0x12000000`).
3. **Misalignment.** Every no-name-hash block (92% of records) was parsed assuming 8 name-hash bytes per record that aren't there. The cursor overshot, subsequent "blocks" were garbage (probe: 48,031 bogus blocks summing to 103 *trillion* records before erroring at offset 49.6MB of 65.8MB).
4. **Silent recovery.** `RootFile::parse` swallows mid-file block errors after the first block ("assume we've reached the end"), so the garbage parse looked like success. The 346K surviving fdids were the early name-hashed blocks plus accidental hash-map hits.
5. **Stale-cache traps.** `resolution.sqlite` freshness only compares mtimes vs `root.bin`/`encoding.bin` — a parser fix does NOT invalidate it. And every extraction failure writes a `<file>.missing` marker that persists.

Verification: a hand-rolled parser using `(cf1|cf2) & 0x10000000` for NoNameHash consumed root.bin to the exact final byte with record count exactly 3,191,148 and named count exactly 249,141, and found fdid 7553349 in block 1195.

### Fix

- cascette-rs `c5de2b9` (branch `improve-header-parsing`, already in `~/Repos/cascette-rs` main) parses the split fields and reconstructs `content_flags = f1 | f2 | (f3 << 17)` with `NO_NAME_HASH = 0x1000_0000`.
- asset-resolver `3ab8a14` bumps its cascette pins to `c5de2b9`.
- wow-ui-sim `598a29909` bumps `asset-resolver` to `3ab8a14` and `cascette-client-storage`/`cascette-crypto` to `c5de2b9`.
- Rebuilt the cache with `casc_refresh` → **1,884,024 entries** (5.4× more); deleted all 904 `*.missing` markers under `~/.cache/wow-ui-sim/casc-extract/`.
- Verified end-to-end: screenshot of a standalone texture with `SetAtlas("ui-debuff-border-magic-icon")` renders the blue dispel border + swirl.

### Operational notes (repeat after any root-parser change)

1. Run `casc_refresh` (asset-resolver bin) — the sqlite cache does not self-invalidate on code changes.
2. Delete `*.missing` markers under `~/.cache/wow-ui-sim/casc-extract/` — they are cached verdicts from the old parser.
3. Sanity check: `sqlite3 resolution.sqlite "SELECT COUNT(*) FROM resolution"` should be within the same order as the root header's `total_files` (millions, not hundreds of thousands).

### Residual upstream smells (cascette-rs, not yet fixed)

- `RootFile::parse` still swallows mid-file block errors silently.
- The `num_records > 1_000_000` "sanity check" returns an empty block **without skipping its record bytes**, guaranteeing misalignment if a legit block ever exceeds 1M records.

## Sources

- cascette-rs `crates/cascette-formats/src/root/{file,block,flags}.rs` (revs `0d0e79a` vs `c5de2b9`)
- asset-resolver `src/casc_cache.rs`, `src/casc_resolver.rs`
- Probe parser run against `~/.cache/asset-resolver/casc/wow/399d19713d9fe33f5c84e6935a515e2d/root.bin`

## See Also

- [[casc-asset-cache]] — the cache layers involved (resolution sqlite, BLP extract cache, `.missing` markers)
- `docs/texture-atlas-system.md` — atlas lookup and texture resolution tiers
