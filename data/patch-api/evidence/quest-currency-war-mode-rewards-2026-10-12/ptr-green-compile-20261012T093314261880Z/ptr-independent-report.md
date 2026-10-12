# PTR independent precursor artifact verification — PARTIAL

Inspection epoch: 2026-10-12T09:43:32.322860+00:00
Report epoch: 2026-10-12T09:45:03.114639+00:00

## Complete proofs

- **Integration authentication PASS:** streamed NEW `integration-sealed` exactly once; 467185280 bytes equals `artifact.json` bytes; SHA256 `3fec92817fc9032a28fe8c2ae130a914534eba2c335f7e41f457544ddadf4ab1` equals manifest. Persisted `ptr-independent-auth-integration.json` before stream review. Initial generic size-field detection missed `bytes`; corrected from already observed size without rereading binary.
- **Compile-only PASS:** `compile-result.json`: exit 0, source_unchanged true. Entire `compile.stdout` read/parsed: 574600 bytes, 741 lines; 665 compiler-artifact records, 75 build-script-executed records, 1 build-finished record (`success: true`, line 741). Integration artifact at line 740, `fresh: false`, `client-ptr` included. Zero compiler-message records. Full 9-line `compile.stderr` read: 6 deprecated manifest lint-name warnings from iced-wgpu-patched; finished test profile in 3m 00s. Not warning-free.
- Stored command is Cargo integration **--no-run**, offline/locked, no-default-features, sound/gui/casc/client-ptr; compile success proves no runtime results. Toolchain records: Cargo/rustc 1.99.0, x86_64-unknown-linux-gnu, LLVM 23.1.1.

## Partial / unavailable proofs

- **Runtime PARTIAL, pending resource admission:** exact outcome `COMPILED_NOT_EXECUTED`, phase `ptr-green`, executed_cases **0**. No runtime stdout/stderr or completed runtime result available at inspection. **Actual 13 PASS is not established; 0 observed passes, not 13 passing tests and not a test failure.**
- Five runtime admission attempts recorded, all `admitted: false`: 09:36:16, 09:36:48, 09:37:51, 09:39:52, 09:42:53 UTC on October 12, 2026. Last exact timestamp `2026-10-12T09:42:53.872476+00:00`, available_gib 18.86835479736328, load1 36.02, relevant_cargo empty. Main owns deferred admission; no waiting, polling, replay, build, test, or ops performed.
- **Normal authentication/runtime UNAVAILABLE:** `normal-artifact.json` and `wow-sim-sealed` pair absent at initial inspection. Saved unavailable `ptr-independent-auth-normal.json`; no normal binary streamed, no normal streams available. No later availability sought.

## Revision limits

Pinned test/source commit `4e19a8bad30ffe039a7604f3dc0cff45334e46ee`; recorded build HEAD `7d82b44a6271b93e8c77a8812faae5cac438b37e`; test source SHA256 `ee2b5cedce9beaebc211353a01084cc6be127a7f23d1ea97450d209a851148a5` (record only, not rehashed). Stored applicability diff_exit 0, empty stdout/stderr, covers only these stored revisions. User reports docs drift and current `a5eaf4fc6` validator consolidation superseding pinned source. **No current final correctness, test pass, or normal build claim.** No old seal rehash.

Proof records: `ptr-independent-auth-integration.json`, `ptr-independent-auth-normal.json`, `ptr-independent-stream-review.json`, `proof.json`. All available compile streams and outcome/admission/revision/toolchain records read in full; absent runtime/normal streams cannot prove completion.
