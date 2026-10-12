# Publication-after current verification: PASS

Revision `a555f927ae7c2b98ff760f45de017acfc0042119`. Unit `wow-guid-publication-after-20261012t192323301012z`: active/exited, Result=success, ExecMainStatus=0. Already completed when independently observed; no rerun.

## Coverage

| Claim | Result | Evidence |
|---|---|---|
| Original cached Game parent/fork exact case | PASS: 1 passed, 0 failed; runtime exit 0 | `runtime-submission.json`, full `runtime.stdout`/`runtime.stderr`; source paths in proof |
| Publication inventory | PASS: 145 rows, 94 OK, 51 reviewed gaps | Private fresh `current-publication-results.json`; current fixture exact equality |
| GUID row 83 | PASS: kind=member, raw=function; lookup=function | `wt-global-api-C_PlayerInfo.GUIDIsPlayer-83` |
| New/stale gaps | PASS: 0/0 | Independent set differences |
| Baseline identities/expected fields | PASS: all 145 unchanged | old533c comparison; other 144 entire records unchanged |
| Auxiliary groups | 3 groups, 0 cases each; NO CREDIT | Exact runtime stdout |
| Native behavior / full suite / default check | NOT VERIFIED | Explicitly excluded |

## Compile, authentication, source

Observed submitted compile: `/usr/bin/cargo test --test prefork_full_ui --no-run --message-format=json --offline --locked -j 12`; exit 0, build-finished success, 741 JSON records. One selected prefork artifact, fresh=false, optimized test profile, debug assertions on. Full compile JSON stream and stderr retained verbatim in proof.

Cargo 1.99.0 (5f94df478 2026-08-27); rustc 1.99.0 (b940084d7 2026-09-28), x86_64-unknown-linux-gnu, LLVM 23.1.1. Both inspection commands exit 0. Six iced-wgpu-patched manifest lint-name deprecations plus one summary line; no compiler-message records. Full warnings retained, not suppressed.

NEW `prefork-sealed` authenticated once using exclusive `independent-auth-prefork.json`: 509486800 bytes, SHA-256 `46cb6e3e51ba52f2db59464ee78227989c6138d007fc10a3146f3f0136239365`, matches manifest; mode 0555. Authentication at 2026-10-12T19:24:00.865442Z is post-run observation, not independent pre-execution attestation. No prior seal, library or original executable read; no reauthentication.

All 13 saved source hashes match before/after compile/after runtime, independently observed current bytes, and revision blobs. Relevant diff to revision empty, exit 0. Full hashes, Cargo metadata/features, admissions, environment allowlist and commands retained in proof.

Compile admission: 33.226776 GiB available >=6; load1 14.12. Runtime: 33.109482 GiB >=20; load1 13.15. Both admitted first observation, no relevant cargo; pressure snapshots retained. Runtime bounded by timeout 90; exact filter, nocapture, test-threads=1; only recorded override P810_SWEEP_OUT. Source follows ordinary Game preload then libc::fork worker, not a substituted factory fixture. Environment allowlist/cache path code does not attest complete environment or cache-content provenance.

## Preservation and handoff

Old533c capture remains 67850 bytes, SHA-256 `9c042e3c42c5d6770f4602e1f3feacc6f9e055d28ba35023ec1c094c0801f0e1`. Nine frozen capture/review/summary/source artifacts match preparation hashes. Historical validator code deliberately changed at this revision (GUID closure/current capture path); it is not a frozen capture and not byte-identical to preparation baseline. No validator run.

Initial strict 1 FAIL/exit 1/worker 101 and publication-before fixture race remain historical failures, not rerun/reclassified. Old533c full-suite failure remains unchanged.

Private fresh capture: `/home/osso/.local/state/wow-ui-sim/verification/guid-is-player-current/publication-after-20261012T192323372938Z/current-publication-results.json` — 67854 bytes, SHA-256 `1c3ae831ad9434ec3655321f783f091a6da26a82935b28adf0f47a16565d2f47`. Main copies after this report; no copy performed, no full-handoff-complete claim.

Proof: `/home/osso/.local/state/wow-ui-sim/verification/guid-is-player-current/publication-after-current-proof.json`. Both final outputs created exclusively with open('x'); generic proof.json and existing files untouched. No verifier builds/tests/replays/source edits/operations/delegation.
