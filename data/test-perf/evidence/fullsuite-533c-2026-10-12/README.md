# Current full-suite 533c — FAIL

[Independent saved-result audit](current-report.md), [complete combined log](suite.log), [runner result](suite-result.json) and [submission](submission.json) bind revision `533c086e046324528d92d581a241021bf455ebdb`, unit `full-suite-1791821148`. Finished October12,2026 at16:25:49UTC. Submission/runner completion is not suite success.

| Phase | PASS | FAIL | Executed | Recorded skips |
|---|---:|---:|---:|---:|
| Integration | 10,777 | 0 | 10,777 | 19 |
| Prefork main | 2,320 | 1 | 2,321 | none |
| Prefork auxiliary | 4 | 0 | 4 | none |
| Library | 2,001 | 0 | 2,001 | 0 |
| Total | **15,102** | **1** | **15,103** | **19** |

Sole failure: `patch_8_1_0_publication_sweep`, strict expected-gap comparison. No new gaps; two stale IDs (`C_FriendList.GetFriendInfo-44`, `C_MountJournal.GetMountFromItem-78`) remain in the54-entry expectation while the observed non-OK set has52. Publication is not signature/output/security/native proof. Corrected current expectations and a separate original-prefork capture live in the [reconciliation packet](../../../patch-api/evidence/publication-533c-reconciliation-2026-10-12/README.md); this saved FAIL is never rewritten as PASS.

Original chat2, cast1 and spellbook1 auxiliary cases each pass within the totals. Original startup, talent-budget and CASC-preparation selectors now pass; successful body timings/cache details are not retained. Whole-test wall durations are not budget measurements. Original numerical thresholds remain; startup/talent observer source changed since BA, so neither historical harness/environment equivalence nor causal repair is established. Frozen BA15,038PASS/3FAIL/19skips remains separate history.

Six inherited dependency-manifest warnings recur in each phase;21 warning lines are not21 distinct warnings. Saved combined stream is retained without invented stdout/stderr separation. Production-scope applicability was independently inspected through `f68ab4ed7`; later expected-gap changes are a separate proof scope.

[Manifest](retention-manifest.json):five byte-exact nonbinary copies/13,396,835bytes. Full log, exact result/submission and full independent proof retained; no executable/seal access or reauthentication. Independent2886 combined retention/link/privacy PASS at5407a5f7e:both packets77/77 copies byte-exact,74 links/six anchors,zero concrete privacy leaks. Private retention reports remain outside the packets. Native/catalog/performance causation/external/full-handoff acceptance and parent remain **OPEN**.
