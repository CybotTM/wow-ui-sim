# Supplied equipment-set locks

The [contract](../../specs/equipment-set-locks.md) uses explicit item-ID observations alongside the existing saved equipment sets. It does not infer native lock acquisition or criteria.

## Model and input

`EquipmentManagerState.item_lock_observations` maps item IDs to supplied booleans. Saved sets already store slot→item ID, so duplicate copies share one observation. This is a model limit, not native instance parity. `A_Admin.SetEquipmentItemLock` replaces one observation; `A_Admin.ClearEquipmentItemLock` removes one. These methods do not equip gear or synthesize native events. Their publication is limited to Retail, PTR, Forever and Mists using `client_profile::ACTIVE`.

## Staged proof

The restored `tests/api_shape_equipment_set.rs` authenticates the actual selected TOC and original `EquipmentManager_EquipSet` source before driving equipment changes. The preliminary Retail input test failed at the missing admin method, before adding the observation store. The query still uses its prior constant-false workaround at this checkpoint; original-consumer RED and modeled query GREEN remain pending.

## Sources

- [Equipment-set lock contract](../../specs/equipment-set-locks.md) — declarations, selected consumers and exclusions.
- [Equipment-set slash command](../../specs/equipment-set-command.md) — existing saved-set/equip transitions.
- [API-shape restoration](../../specs/api-shape-contracts.md) — historical deferred coverage.

## See Also

- [[consumer-selection-and-red-proof]] — selected-consumer proof requirements.
