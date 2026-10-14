# Bullet / dressability normal-startup verification

**PASS — existing evidence at `69f2569cb` (`69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c`), not a new run.**

## Coverage matrix

| Claim | Retail | PTR | Proof |
|---|---|---|---|
| Bullet ID 7601001 returns exactly one nil | PASS | PASS | recorded Lua assertion before marker |
| Dressability ID 7601001 returns exactly one false | PASS | PASS | recorded Lua assertion before marker |
| Assertion completion marker | 1 | 1 | stdout line 1 |
| JSON Lua errors | 0 (`[]`) | 0 (`[]`) | independently parsed stdout line 2 |
| Full diagnostic Lua errors | 0 unique / 0 occurrences | 0 unique / 0 occurrences | stderr lines 215–219: CLEAN; owners 0, unattributed 0 |

Both scripts call each API twice: first for return cardinality, second for value. Marker follows both assertions. Evidence is not exit-0 alone; launcher success is corroboration. `expected_cases=0` and empty test summaries are not a test-suite pass.

## Full diagnostics and classification

Retail: 46 stdout bytes / 2 lines, 14,702 stderr bytes / 219 lines. PTR: 46 stdout bytes / 2 lines, 14,697 stderr bytes / 219 lines. Full stdout/stderr read, stderr in contiguous ranges 1–75, 76–150, 151–219.

- **Nil:** expected missing Bullet record nil is asserted; no other nil observations in either complete log. No universal all-nil claim.
- **Missing requirements / loader warnings:** none emitted in either full runtime log. Existing workaround phase names, `seterrorhandler restored`, and zero-error summary are informational, not errors. No proof of absent unlogged gaps.
- **Deprecations:** runtime 0; each existing compile stderr has six `iced-wgpu-patched/Cargo.toml` deprecated hyphenated Clippy lint keys: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity. Each recommends its underscore spelling. The seventh warning line summarizes six, not a seventh deprecation. Compile-result records no source compiler messages. These compile-manifest diagnostics do not negate the narrow normal-startup assertions; no warning-free build claim.
- **Context:** headless `lua-errors`, `--no-addons`, `--no-saved-vars`, CASC=0, sound disabled, network unshared, timeout 86s. Third-party addons and SavedVariables disabled in diagnostics; Blizzard addons loaded (Retail 291, PTR 283). Read-only WTF EditMode layout cache nevertheless loaded. Existing post-load workarounds ran. This is not fully cache-free startup.

## Revision and receipt boundary

Absolute `git -C <repo>` confirmed HEAD `bde7ea277e86f9ad17f7135b3cd676f80e73057b`. Full revision diff contains only `tests/widget_bullet/fixtures.rs`, changing the test helper from `env.eval::<()>(code)` to `env.exec(code)`. Diff excluding tests is empty, exit 0: production/config/manifests unchanged between the two committed revisions. Keep runtime label **69f2569cb**; no current-revision tests claimed.

Both revision/compile/association/runtime receipts name `69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c`. Runtime admission allowed both (MemoryMax 4 GiB, MemoryHigh 3 GiB; recorded argv CPUQuota 200%). Both launcher records show exit 0/SUCCESS. Retail runtime 14.238s, PTR 11.089s.

Existing receipt metadata reused, with stat-only checks matching receipt before/after and runtime result. No fresh hashes or binary/library/archive/source-tree authentication. Feature association is recorded metadata, not re-authenticated provenance. Retail association includes client-retail through retail-12-1-0; PTR includes client-ptr through retail-12-1-5. Profiles remain distinct.

## Evidence paths

Retail: `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-normal-bullet-dress-defaults`

PTR: `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-normal-bullet-dress-defaults`

For each: full `stdout`, `stderr`, `admission.json`, `argv.json`, `result.json`, `launcher.txt`; epoch `revision.json`, `compile-result.json`, `compile.stderr`, `receipt-wow-sim.json`, `normal-artifact-association.json`. Exact assertions, diagnostic counts, deprecation lines, receipt stats, and git command results are in `proof.json`.

## Limits

No new builds, runtime runs, authentication, service operations, delegation, Bash, cwd-changing tools, or repository edits. No native WoW parity, rendering, populated Bullet/dressability data, producer suites, PlayerChoice behavior, or parent-goal promotion. Those requirements remain unproved by this narrow evidence. Report/proof are new exclusive-create 0600 files, fsynced, directory-fsynced, and byte-readback checked.
