# API shape-only contracts

Documented shapes, not invented native behavior, are the acceptance contract for these native-unknown surfaces (decision 2026-10-23). Tests live in `tests/api_shape_*.rs`; original consumer selection follows [consumer selection and proof](../wiki/investigations/consumer-selection-and-red-proof.md). All execution is deferred: no PASS or semantic RED is claimed.

## What it must do

### Stable active list

- [ ] Retail/PTR `C_StableInfo.GetActivePetList()` returns exactly one non-nil `table<PetInfo>`; validate every present record and nested numeric ability table without requiring any entry count, order or slot membership.
- [ ] Original `StableActivePetListMixin:Refresh` accepts actual results without Lua error. An empty table is shape-compatible, not native-confirmed. `uiModelSceneID` documents Default=718, not a required value for every record.

Sources: Retail/PTR `Blizzard_APIDocumentationGenerated/StableInfoDocumentation.lua:15-22,193-214`; selected `Blizzard_StableUI/Blizzard_StableUI.lua:447` via `Blizzard_StableUI/Blizzard_StableUI.toc:4`. Forever excluded: selected Camelot StableUI never calls this API despite cached Standard source and documentation.

## How it works

- [Runtime-selected consumers and proof classes](../wiki/investigations/consumer-selection-and-red-proof.md).

## Implementation inventory

- `tests/api_shape_stable_active_list.rs`: Retail/PTR list/record shape and original refresh acceptance; missing member is a shape blocker, not semantic RED.

## Tests asserting this spec

- `api_shape_stable_active_list::stable_active_list_shape_blocker_contract` — source-predicted FAIL for missing member; not executed.

## Known gaps (current cycle)

- [ ] Deferred execution must distinguish setup failures from shape blockers and preservation passes. No producer or vendor changes are authorized here.
- [ ] `GetInventoryItemsForSlot` (#7 q1, Retail/PTR/Forever consumer) and `IsInventoryItemProfessionBag` (#7 q3, Mists/Era/Anniversary consumer): native-unknown with **no generated documentation schema** in the six scoped profiles. No shape test, return type, mutation rule or semantics is inferred from call sites.

## Out of scope

Native behavior remains unknown: defaults, membership, ordering, eligibility/lock/purchase policy, invalid-input behavior, event timing and persistence. A constant shape-compatible implementation can satisfy these contracts; passing never proves a native producer. No Wrath coverage, forced unselected consumers, runtime N/A returns, ignored tests, API replacements or vendor edits.
