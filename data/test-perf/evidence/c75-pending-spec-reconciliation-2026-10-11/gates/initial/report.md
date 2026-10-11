# c75 spec-reuse gate — FAIL (one documentation path)

Revision: `e1134f74286b93c353afbad58889369212d27cf6`. Independent artifact/doc audit, 2026-10-11. Followed verify skill as verifier; no delegation or test execution.

- [EXIST] FAIL for one referenced implementation path; all six retained packet files, three specs and c75 wiki page exist.
- [SUBSTANTIVE] PASS: three exact audit copies, 13 named raw PASS records, bounded assertions and explicit limitations; no placeholders.
- [WIRED] FAIL for events spec inventory path only; packet/spec/wiki links resolve. Actual event guards are wired in existing text_attribute_event implementation.
- [ANTI-PATTERN] No new executable code. Historical failure narratives retained, not counted as current runtime failures or new requirements.

## Exact artifact proof

3/3 copies match original handoff bytes and manifest SHA-256/length: 4,063 + 6,071 + 3,835 = **13,969 bytes**. Hashes and per-file observations are in `report.json`.

13/13 extracted line/text records exactly match original c75 log at 5792, 5793, 5837, 7176, 7650, 7696–7698, 7719, 8244–8247. Five roleset PASS records are separately extracted execution evidence; the retained roleset source audit had not found them. No rerun.

Read-only Git exit0/empty diff: `git diff --name-only c75ccfe02c52f76a0456ae393581a3b9ca601ce7 -- src tests Cargo.toml Cargo.lock`. Current tracked source/test/pin scope unchanged. No binary/cache/native provenance inferred.

## Four-document claim audit

| Document | Result and assertion boundary |
|---|---|
| patch-12-1-0-onupdate.md | Bounded claims PASS. `tests/on_update_modes.rs:164–211` uses actual managed dirty phases; hidden mode2/dirty survives, visible dispatch and next dispatch clean/mode0; empty errors. Registration retention explicitly NOT asserted. |
| patch-12-1-0-events.md | Bounded claims PASS; inventory FAIL below. `tests/forbidden_aspect_enforcement.rs:205–245` proves intrinsic aspect, Blizzard registration retained, addon registration/query/clear rejected, secure registration allowed, plain-frame control. Eight-operation secure/addon matrix and companion listener/callback controls in `tests/forbidden_aspect_creation.rs`. Mask-zero RED historical. No ManagedAura obligation by name. |
| scenario-unit-criteria.md | Bounded claims PASS. `tests/scenario_unit_criteria.rs:8–17,69–132` supplies `identity_restricted` per row; tainted plain-token query returns three opaque values, preserves caller taint, denies unwrap/secret-token input and keeps public row readable. `src/c_api/c_scenario_info.rs:91–142` reads that field; no global resolver/native identity credit. |
| Wiki c75 reuse section (lines19–21) | PASS: 13 existing cases, historical RED distinguished, no extra executions or native/whole-goal closure. |

Roleset body assertions: `tests/patch_12_1_0_rolesets.rs:20–140` covers shown-but-invisible tagged frame, ancestor child visibility, untagged controls, tag changes, XML tags, allow/block cases, strata exclusion/restoration. Adjacent active-list and membership controls at `tests/patch_12_1_0_added_globals.rs:417–437` and `tests/patch_12_1_0_surface_closures.rs:129–139`. `src/c_api/c_roleset.rs:40–67,136–161` implements supplied block/allow policy. Native filter argument shape/IDs/defaults/coercion remain unknown.

## Finding

`docs/specs/patch-12-1-0-events.md:17` names **nonexistent** `src/lua_api/frame/methods/forbidden_aspects/events.rs`. Actual eight event guards are in `src/lua_api/frame/methods/text_attribute_event/events.rs:24–208`; shared secure/mask check in `forbidden_aspects.rs:36–40`, intrinsic mapping at174–192. Read-only c75 `git ls-tree` also contains only the actual path. The exact retained historical aura audit repeats this incorrect path. Correct current inventory reference; preserve original audit bytes. No source change or architectural requirement follows.

## Accounting

13 cases already included in **15,024 PASS / 1 Garrison FAIL / 19 skips / 15,025 executions**. Raw integration summary10732, prefork main13306, auxiliary13311/13315/13319, library15830 corroborate accounting. **0 additional executions.** Enclosing suite FAIL; native/full-profile/whole-handoff remain OPEN.

Only artifact reads/comparisons and read-only Git; no tests, builds, checks, reruns, operations, delegation, Bash, rg, fd or binary hashing. Reports are the only writes.
