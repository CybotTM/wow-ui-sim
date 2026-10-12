# Current publication-before verification — FAIL expectation / coherent bounded capture

Unit `wow-guid-publication-before-20261012t191819959411z` is terminal (`active/exited`, systemd Result=success, process status0), completed October 12, 2026 19:18:43 UTC. Controller stdout is nevertheless a JSON error: `Initial expected stale-GUID observation not confirmed; output retained`.

## Actual strict result

Original exact prefork sweep: **0 PASS / 1 FAIL / 1 total, exit1**, not101. Panic at `tests/common/publication_sweep.rs:315:5`: `new gaps: []; resolved/stale gaps: ["wt-global-api-C_PlayerInfo.GUIDIsPlayer-83"]`. Capture written before strict assertion. Three auxiliary groups ran zero tests; zero credit.

Main phase script incorrectly required101 (Cargo/libtest convention). Original custom prefork runner returns `ExitCode::FAILURE` (1): `tests/common/prefork.rs:102-109`, `tests/prefork_full_ui.rs:346-354`. **Pipeline exit101 assumption falsified; genuine test FAIL retained.** Not a producer/new-gap failure. No rerun, patch, or broadened acceptance.

## Literal actual-map comparison

| Map | Rows | OK | Gaps |
|---|---:|---:|---:|
| Fresh actual | 145 | 94 | 51 |
| Old533c unchanged | 145 | 93 | 52 |
| Frozen source unchanged | 145 | 87 | 58 |

Known fixture52 vs observed51: sole resolved/stale ID `wt-global-api-C_PlayerInfo.GUIDIsPlayer-83`; no new gaps. All145 row IDs and all expected objects unchanged. Other144 old533c rows equal actual literally. GUID83 alone changes `ok=false` to `true`, `raw=nil; lookup=function` to **`raw=function; lookup=function`**; observed kind remains `member`, default/value fields unchanged. Literal source row remains global-api/added, line83, same symbol and no supersession. No invented behavioral credit.

## Authentication / compiler / source

Fresh epoch `publication-before-20261012T191820067096Z/prefork-sealed` independently read/hashed exactly once; exclusive receipt `immediateindependent-auth-prefork.json`. 509486864 bytes, mode0555, SHA256 `cd4df4e9920239cf2b0a6306275a571d9451306a870a4d332f4f14baa866bab2`, matches sole Cargo prefork executable record (`fresh=false`). Authentication occurred 19:19:11 UTC, after runtime; it does not independently attest execution-time identity or libraries/cache bytes.

Full compile output inspected: 665 artifact records,75 build-script records,one successful build-finished; zero compiler-message records; exit0. Six manifest deprecation warnings retained in proof/logs (hyphenated clippy lint names in iced-wgpu-patched). Full runtime stdout/stderr inspected, including full strict gap-set panic. Toolchain: Cargo1.99.0 (5f94df478 2026-08-27), rustc1.99.0 (b940084d7 2026-09-28), x86_64-unknown-linux-gnu, LLVM23.1.1.

All13 source hashes identical before/after compile/after runtime/independent observation. Scoped git diff against d6d8de29a exit0/empty; HEAD separately observed at db881bff81389cc6ec39fdef9a988b68cfd7f966, not d6d8de29a. Relevant compiler/fixture source unchanged despite HEAD advance. All15 preparation baseline files preserved byte-for-byte, including both old maps, historical accounting and source register. Input/later-register hashes retained as current observations, not historical environment attestation.

## Boundary / limits

Original Game parent preload retained: cached addon discovery/full/bootstrap loading, normal workarounds/startup events, then selected inherited-state fork; no new environment/provider implementation. Runtime submission overrides only P810_SWEEP_OUT; retained allowlist is null throughout. P810_SWEEP_REGISTER was not attested in that allowlist: absence of a submitted override and literal map match are evidence, not full inherited-environment proof.

No RED/GREEN, old seals or libraries read. No source edit, build, test, replay, operations or delegation. Historical full-suite FAIL/full handoff OPEN retained; no post-reconciliation PASS, PTR/native parity, output/security/signature or GUID-classification behavior claim. Main harness error does not invalidate the observed bounded145/94/51 map, but requested exit101 acceptance is FAIL.

Full logs, exact artifact/submission/source/context records, warning text, map delta and preservation digests are in `publication-before-current-proof.json`. No existing report or generic proof.json overwritten.

**FAIL: bounded map/source/authentication coherence also failed. See proof; no broadened acceptance.**
