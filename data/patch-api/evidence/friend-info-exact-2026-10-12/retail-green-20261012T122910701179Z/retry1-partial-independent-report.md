# Retry1 partial independent audit

**PARTIAL — Retail recorded runtime passes; normal probe blocked by CLI parsing. Full acceptance OPEN.**

Epoch: `retail-green-20261012T122910701179Z`; revision `6ec0c1cb84489963a24cd2017a4099a5c571301a`.

## Authentication and source

New `integration-sealed` independently streamed through SHA-256 exactly once: 468017128 bytes, `e34ee735e4dc984939a1a5421797cfb39515ea898d712a4931b237222a356ddc`, matching `integration-artifact.json`. Receipt saved immediately as `independent-auth-integration.json`. Cargo artifact record matches compile stdout; compile exit0/build-finished success. Six deprecated manifest lint-key warnings; not warning-free. Recorded Cargo/rustc1.99.0.

Normal authentication reused ONLY from `normal-reuse.json` → prior122455 `independent-auth-normal.json`; receipt matches prior manifest path/size/hash. Normal seal never opened or rehashed by audit.

Source-before/after nine-entry maps agree. Compared with122455, only `tests/friend_info_exact.rs` changes; producer/Cargo/compatibility tests unchanged. Epoch applicability records target6ec, empty relevant diff/exit0. Read-only Git comparison independently confirms no producer/Cargo delta and security-test-only function delta. Removed two canaccessvalue helper prerequisites; authentic secret identity, wrapped-type rejection, supplied fields/arity, caller-tainted secret rejection, ordinary tainted acceptance and taint preservation/restoration remain. No producer/helper fix claimed.

Reuse `../source-proof.json` for unchanged producer at source-only level and `../secret-fixture-delta-proof.json` for6ec fixture preservation/readability. Earlier HEAD/test identity is not target runtime evidence. Four original exact test function segments and supplied_env remain unchanged; saved Git contents in `retry1-partial-independent-git-evidence.json`.

## Concrete coverage

| Scope | Recorded evidence | Proof level |
|---|---|---|
| Secret fixture | exact stdout:1 passed,0 failed; exit0 | Executed this epoch, authenticated integration seal |
| Legacy social/Who state | legacy-state stdout:2 passed,0 failed; exit0 | Executed this epoch, same seal |
| Seeded friends | seeded-state stdout:1 passed,0 failed; exit0 | Executed this epoch, same seal |
| Original alias/live-note/miss-type/popup cases | Four unchanged functions PASS in prior122455 exact stdout | Reused prior execution, NOT rerun here |
| Normal loaded startup/publication | exit2; CLI parser rejected Lua argument | Probe/UI startup NOT reached; no producer-cause evidence |
| PTR, cargo fmt --check, cargo check | Retry1 controller stopped after normal command | NOT EXECUTED in this retry1 |

Fresh epoch total **4PASS/0FAIL (secret1+legacy2+seeded1), all group exits0**. Original four PASS reused separately from122455. Do not call this an eight-case current invocation. Original122455 group remains4PASS/1FAIL, exit101; no retroactive replacement.

## Normal failure boundary

`normal-submission.json` passes inline --exec-lua text beginning `-- Bounded normal Retail/PTR...`. `normal.stderr` states:

```
error: unexpected argument '-- Bounded normal Retail/PTR loaded-publication probe for FriendInfo exact lookup.
-- Run only against normal Retail and PTR wow-sim binaries after build; do not seed state.
local ns ' found

Usage: normal-sealed --no-addons --no-saved-vars
```

Process-start/resource-limit lines precede parser failure, not loaded UI startup. stdout empty, exit2. `failure.json` says “Normal publication/startup failed”, but actual boundary is command-line parsing before UI startup/Lua probe. Neither startup failure nor getter/producer cause inferred. No loaded-publication PASS. Controller raises before PTR/final-check calls. Later retry2 files excluded.

## Preservation and limits

No builds, runtime executions, delegation, repository edits, or prior normal hashing. Original logs/seals/receipts unchanged. Only outside-repo audit artifacts written. Initial undefined-p evaluation did nothing; corrected authentication completed once. Structured evidence/exact runtime output and stderr: `retry1-partial-independent-proof.json`. Full/native acceptance remain open; helper parity gap remains separate.
