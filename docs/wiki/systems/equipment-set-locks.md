# Supplied equipment-set locks

The [contract](../../specs/equipment-set-locks.md) uses explicit item-ID observations alongside the existing saved equipment sets. It does not infer native lock acquisition or criteria.

## Model and input

`EquipmentManagerState.item_lock_observations` maps item IDs to supplied booleans. Saved sets already store slot→item ID, so duplicate copies share one observation. This is a model limit, not native instance parity. `A_Admin.SetEquipmentItemLock` replaces one observation; `A_Admin.ClearEquipmentItemLock` removes one. These methods do not equip gear or synthesize native events. Their publication is limited to Retail, PTR, Forever and Mists using `client_profile::ACTIVE`.

## Query projection and unknown inputs

`src/c_api/equipment_set_locks.rs` finds the current saved set and joins its stored item IDs to the supplied observations. It requires an observation for every stored item, then returns their boolean OR. Resaving changes membership immediately. It does not use combat, current bag position, ignored-slot flags, upgrades or invented lock criteria. Unknown sets and missing observations raise explicit simulator errors with the same `native-unknown:` style as Forever's target-spell policy. Native empty-set/default/unknown-set parity is not established.

On R/P/F/M the temporary bootstrap returns without publishing its old default, so there is no fallback. Excluded W/E/A retain their existing workaround unchanged. Admin input reads require an actual Lua boolean: the general `bool::from_stack` helper accepts nil as false and other values as true, which is not an explicit observation.

## Original-consumer proof

The restored `tests/api_shape_equipment_set.rs` authenticates the selected Game TOC and original `EquipmentManager_EquipSet` source before observing actual equipment, last-used-set and swap transitions. Each profile passes six exact GREEN cases; each of three original-consumer selectors independently fails against the byte-identical constant-false producer sibling. Mainline259–266 is selected on R/P/F; Mists selects Wrath307–314. The mixed locked/unlocked set, all-unlocked transition, relock, unrelated-item lock, resave, clear/error recovery and independent environments use explicit supplied fixtures.

Retail's actual pre-producer temporary path also failed all three consumer selectors before implementation. A separate preliminary RED caught a missing admin method; another caught generic Lua truthiness accepting nil as an unlocked observation. The latter is input-plumbing proof, not native boolean policy. The old `c_item_api` implicit-unlocked assertion was observed failing under the new contract and replaced with explicit supported-profile errors while preserving excluded-profile behavior.

The [contract proof table](../../specs/equipment-set-locks.md#bounded-proof--2026-10-26) owns exact per-profile stock and gate results. All four raw stock maps are unchanged; unrelated startup residuals remain. No loader errors were cleared, no vendor source was patched and no whole-client or native-policy parity is claimed.

## Sources

- [Equipment-set lock contract](../../specs/equipment-set-locks.md) — declarations, selected consumers and exclusions.
- [Equipment-set slash command](../../specs/equipment-set-command.md) — existing saved-set/equip transitions.
- [API-shape restoration](../../specs/api-shape-contracts.md) — historical deferred coverage.

## See Also

- [[consumer-selection-and-red-proof]] — selected-consumer proof requirements.
