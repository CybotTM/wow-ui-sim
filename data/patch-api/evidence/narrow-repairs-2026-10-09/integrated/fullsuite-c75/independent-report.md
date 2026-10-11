# Independent saved c75 full-suite audit — 2026-10-11

**FAIL. 15,024 PASS / 1 FAIL / 19 recorded skips; 15,025 actual executions.**

Read/followed verify skill as independent verifier. Both complete raw logs scanned end-to-end; saved JSON exits and actual failure identities agree. No reruns, builds, operations, delegation, binary hashing, mutable master-latest comparison, or whole-log copies. Only requested reports written.

## Exact matrix

| Step/group | Executed | PASS | FAIL | Recorded skips | Step exit |
|---|---:|---:|---:|---:|---:|
| integration | 10,699 | 10,699 | 0 | 19 | 0 |
| prefork main | 2,321 | 2,320 | 1 | 0 | 1 overall |
| prefork auxiliary chat | 2 | 2 | 0 | 0 | same step |
| prefork auxiliary cast | 1 | 1 | 0 | 0 | same step |
| prefork auxiliary spellbook | 1 | 1 | 0 | 0 | same step |
| prefork subtotal | 2,325 | 2,324 | 1 | 0 | 1 |
| lib | 2,001 | 2,001 | 0 | 0 | 0 |
| **Total (subtotal not added twice)** | **15,025** | **15,024** | **1** | **19** | **FAIL** |

Integration summary raw line10732; prefork main line13306; auxiliaries line13311/13315/13319; lib line15830. Skipped identities are not listed in raw log: 19 is summary proof, not invented case identities. Zero-test groups receive no test credit; none of the four named prefork execution groups is zero. Counts sum actual case events without cross-group deduplication. Repeated nextest FAIL echoes and nested detail summaries in 469 do not represent extra executions.

## Saved469 comparison — historical FAIL preserved

469: integration10,696PASS/2FAIL/19skips (exit100); prefork main2,320PASS/1FAIL plus auxiliary4PASS (exit1); lib2,001PASS (exit0). Total15,021PASS/3FAIL/19skips,15,024 executions. Current actual identity multiset adds exactly one integration case, removes none; prefork/lib multisets unchanged. Not based on `new_failures` labels.

Raw c75 PASS proof:
- line6841: `method_diff_coverage::diff_methods_extra_snapshot_matches_current_metatable_surface` (469 FAIL line6844).
- line6842: `method_diff_coverage::diff_methods_missing_snapshot_matches_current_metatable_surface` (469 FAIL line7465).
- line116: `achievements_api::test_reward_item_id_tracks_supplied_achievement_state` — sole added identity.

Auxiliary PASS proof: chat editbox click/submit line13308; chat text color line13309; cast bar edit-mode lock line13313; mainline spellbook keybind line13317. Four actual executions, included once in totals. Method snapshot PASS proves these assertions in this execution, not native semantics or stable full-suite completion.

## Only recorded failure — exact boundary

`blizzard_garrison_ui_loads::blizzard_garrison_ui_loads_explicitly_via_load_addon_without_errors`: FAILED case line11379, stderr heading11409, first non-3D error11566, panic11596 at `tests/blizzard_garrison_ui_loads.rs:286:5`.

Explicit post-fork Garrison load clears errors, loads addon at pinned source270-271, then filters Garrison/non-3D errors at273-285 and fails assertion286. Raw error: `Blizzard_AdventuresCombatLog.lua:90`: `bad argument #1 to 'ipairs' (table expected, got nil)` during frame creation/OnLoad. Two reported copies in panic are one failing case. SetTargetDistance/SetFacingLeft errors are accepted 3D gaps, not this failure. Saved469 fails same identity/boundary (panic12278); therefore retained failure, not newly established regression.

Current retail cache inspection: OnLoad line88 calls `C_Garrison.GetAutoCombatDamageClassValues()`, line90 iterates returned damageClassStrings. This is current-cache context, not retroactive cache provenance. Native result/catalog contract remains unproved. No path miss identified. No empty-table, shim, enum-derived-row, or vendor-refresh proposal.

## Revision and OPEN boundaries

Audited SHA: `c75ccfe02c52f76a0456ae393581a3b9ca601ce7`; comparison SHA: `469a92d27bd2c45e590b1d22bf9137ce4c70f3d1`. Later observed log tip `da1220d38`; tracked non-doc/evidence differences are Dockerfile, scripts/build-host.py, tools/test_build_host.py. Rust/runtime scope unchanged in compared committed diff; retained data Rust copies are evidence, not implementation changes. No working-tree parity claim. This result is not execution proof for later Docker/Python changes or later full-branch readiness.

Original native **OPEN**; caller-known native auth/catalog **BLOCKED**; other profiles **OPEN**; full handoff **OPEN**. Previous469 **FAIL** retained. No native parity, stability, or broader acceptance claim.

Machine-readable exact counts, raw line references, identity deltas, failures, and limitations: `report.json`. Initial parser calibration and failed cwd/file-search probes provided no acceptance credit; final parser independently reconciles ordinals, group declarations, summaries, exits, and saved failure identities.
