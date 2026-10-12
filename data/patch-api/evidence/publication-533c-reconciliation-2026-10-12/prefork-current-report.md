# Current prefork publication gate — PASS

- Revision: `61cd81963`; original cached Game parent-preload/fork case `patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep`: **1/1 PASS**, runtime exit 0. Three `running 0 tests` auxiliary groups receive no credit; embedded controller conformance is not another sweep.
- Terminal unit: `wow-p810-reconcile-20261012t163312687421z.service`, active/exited, success, ExecMainStatus 0. Observed terminal within bounded 20-minute window; no wait/replay/build/unit mutation needed.
- Compiler: offline, locked, `-j 12`, prefork executable only; exit 0, build-finished success, selected `fresh=false`. Cargo 1.99.0 / rustc 1.99.0 (LLVM 23.1.1), x86_64-unknown-linux-gnu. Exact selected metadata/profile/features saved in proof; test profile opt-level 1, line-tables-only debuginfo, debug assertions and overflow checks enabled.
- New seal independently read/hashed exactly once; exclusive `independent-auth-prefork.json` immediately saved. 509725528 bytes; SHA256 `ff713f8b72a715e5165f72a782345bb76f7bca45c223da48aacd9b1e137c465f`; producer metadata matches. No prior-seal access or reauthentication.
- Full compiler stdout/stderr (741 JSON records), runtime stdout/stderr, metadata, input context, admissions, source records and 145-row result JSON read completely and preserved in proof. Compiler retains six manifest deprecation warnings; no compiler-message records.
- Rows: **145 = 93 OK + 52 known gaps**; exact non-OK/known-gap set equality; **0 new gaps, 0 stale gaps**. Only reviewed `C_FriendList.GetFriendInfo-44` and `C_MountJournal.GetMountFromItem-78` IDs removed from historical expected set; both `raw=function; lookup=function`, publication OK.
- All 13 source hashes match before compilation, after compilation, after runtime and current files. Current compiler/fixture diff against 61cd81963 empty, deliberately excluding concurrently owned historical-validator Python edits. Original helper assertion and parent-preload/fork path unchanged from 533c086e0; no provider/vendor revision changes. Admission: compile 37.991 GiB/load 9.52; runtime 39.405 GiB/load 9.81; no relevant cargo process at either boundary.

## Retained history and limits

Historical 533c086e0 full suite remains **FAIL: 15102 PASS, 1 FAIL, 19 skips** (user-specified history, not rerun/reclassified here). Prechange historical validator remains **FAIL exit 1 at validate.py:81**, original traceback preserved. This current targeted pass does not rewrite that full-suite failure or prove native whole-handoff completion. Input allowlist is not complete inherited-environment attestation. Publication/absence only; no signature/output/security/behavior parity claim.

## Artifacts

Epoch: `prefork-green-20261012T163312758313Z/`
Proof: `prefork-current-proof.json`
Receipt: `prefork-green-20261012T163312758313Z/independent-auth-prefork.json`
