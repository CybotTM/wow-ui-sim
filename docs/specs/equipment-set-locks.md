# Supplied equipment-set item locks

`C_EquipmentSet.EquipmentSetContainsLockedItems` projects supplied observations onto the existing saved equipment sets on Retail, PTR, Forever and Mists. Native acquisition and lock policy are not inferred. See the [model and proof](../wiki/systems/equipment-set-locks.md).

## What it must do

- [ ] Publish the numeric saved-set-ID query on Retail/PTR/Forever/Mists. Preserve excluded Wrath/Era/Anniversary behavior.
- [ ] `A_Admin.SetEquipmentItemLock(itemID, locked)` supplies or replaces one explicit boolean observation; missing/nil/non-boolean inputs error without mutation. `A_Admin.ClearEquipmentItemLock(itemID)` removes only that observation; neither operation equips gear or infers native event timing.
- [ ] Join observations to the current saved set's slot-to-item-ID contents. With an observation for every referenced item, return exactly one boolean: whether any referenced item is locked. A lock outside the saved set does not affect the result; resaving changes membership immediately.
- [ ] Unknown sets and missing observations raise explicit simulator errors prefixed `native-unknown: C_EquipmentSet.EquipmentSetContainsLockedItems`. Clearing an observation restores the missing-observation error. This wording is simulator policy, not a native error contract.
- [ ] The original runtime-selected `EquipmentManager_EquipSet` rejects locked saved sets without changing equipment or starting a swap, equips unlocked sets, and follows replacements/relocking without leaking observations between environments.

The saved-set representation is slot→item ID, not item-instance identity. Observations therefore use item IDs; two copies of the same item ID share one observation. This is a model limit, not a native claim. No lock flag is seeded implicitly. The query requires complete observations rather than treating unknown items as unlocked. Empty known contents have no referenced locked item under this supplied-state projection; this does not establish native empty-set policy.

Declarations: pinned `Blizzard_APIDocumentationGenerated/EquipmentManagerDocumentation.lua:56–69` on Retail/PTR/Forever and `:53–65` on Mists. All require numeric `equipmentSetID` and return non-null boolean `hasLockedItems`. PTR line numbers were re-authenticated. Selected original consumers are Mainline `EquipmentManager.lua:259–266` on R/P/F and Wrath `EquipmentManager.lua:307–314` on Mists. FrameXML TOC selects `[Family]` at Retail `:99`, PTR/Forever `:105`; Mists selects `Wrath` at `:70`.

## How it works

- [Equipment-set state and original-consumer proof](../wiki/systems/equipment-set-locks.md).

## Implementation inventory

- `src/c_api/equipment_set_locks.rs` — supplied observations, profile gate and locked-item query.
- `src/lua_api/state_types/character_world.rs` — observation map beside existing saved sets.
- `src/c_api/item_spell/c_equipment_set.rs` — namespace publication.
- `src/lua_api/globals/admin.rs` — host input publication.
- `src/lua_api/workarounds/temporary/equipment_set_lock_defaults.rs` — excluded-profile legacy workaround only.

## Tests asserting this spec

- `tests/api_shape_equipment_set.rs` — restored coverage uses real original-consumer equipment/swap transitions, saved-set changes, explicit errors and independent environments, not boolean shape alone.

- `tests/c_item_api.rs` — the previous implicit-unlocked test now requires explicit missing-observation errors on supported profiles and retains the old assertion on excluded profiles.

## Known gaps (current cycle)

- [ ] Record exact paired four-profile runtime, stock and zero-warning check proof before claiming completion.

## Out of scope

Wrath/Era/Anniversary contracts, item upgrades and `C_ItemUpgrade`, item-instance identity, native lock acquisition/defaults/criteria, ignored-slot lock policy, missing-item policy, combat/casting restrictions beyond the unchanged original consumer, native unknown-set/invalid-input wording, native secrecy and event-timing parity. Existing saved-set equip transitions are reused, not redesigned.
