# Item-upgrade eligibility

The [contract](../../specs/item-upgrade-eligibility.md) separates supplied observations from unknown native upgrade policy. This is not an upgrade-cost, currency or upgrade-window model.

## State and lifecycle

Each environment owns a location-keyed observation map. An observation records the resolved item ID and a supplied boolean, not a catalog-wide classification. The cursor carries an observation separately, guarded by the current item ID and origin. Existing bag/cursor/equipment verbs move both observations with their items. Admin replacement/removal discards the old observation. Clearing all inputs also clears the cursor observation. New split cursor stacks have no inferred observation.

`SetItemUpgradeEligibility` resolves the same ItemLocation fields as `C_Item`; it cannot seed an absent item. `ClearItemUpgradeEligibility` clears one location or the environment's inputs. Neither method changes items or invents an upgrade event. The query checks the actual item before projecting supplied eligibility; unresolved and unsupplied cases raise an explicit simulator `native-unknown` error rather than returning a policy default.

Equipment-set transfers are excluded. `apply_equipment_set` rebuilds equipped records from saved item IDs, not item-instance identity. This lane does not modify that path or add identity tokens.

## Consumer and proof boundary

The fixture loads the selected FrameXMLUtil and ObjectAPI dependency closure. It authenticates the original generic ItemUtil function and supplies a plain shown `ItemUpgradeFrame` as context input. The fixture does not replace a Blizzard function or claim that an original upgrade window loaded. This permits bounded generic-consumer proof on Era/Anniversary without force-loading the unselected Mists window. Forever's small Camelot utility extension is not the target function.

The restored selector observes original Match/Mismatch for two distinct items/locations, changes, bag swaps, equipment transfers, removal, clear and a second environment. The separate diagnostic selector covers unsupplied/unresolved inputs and replacement. Execution is pending; source preparation alone is not RED/GREEN acceptance.

## Sources

- [Producer](../../../src/c_api/item_upgrade_eligibility.rs).
- [Existing inventory transfers](../../../src/c_api/container_inventory.rs).
- [Behavioral fixtures](../../../tests/api_shape_item_upgrade.rs).
- [Contract and pinned declaration/consumer lines](../../specs/item-upgrade-eligibility.md).

## See Also

- [[consumer-selection-and-red-proof]] — selection, preservation and producer acceptance.
- [[cf2-tts-integration]] — historical deferred shape coverage.
