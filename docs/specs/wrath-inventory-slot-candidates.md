# Wrath inventory slot candidates

`GetInventoryItemsForSlot` supplies the original Wrath paper-doll flyout with
location-keyed item IDs. The selected `FrameXML/PaperDollFrame.lua:1793` calls
it, and `FrameXML/EquipmentManager.lua:132` decodes its locations. The location
flags and bag shift are defined in `FrameXML/Constants.lua:182–186` of the
profile-scoped Wrath cache. Native 12340 laptop answer #7 establishes ring
candidates in both finger slots and carried bags, duplicate item IDs at distinct
locations, closed-bank exclusion, and no results for numeric slots outside 1–19.

The modeled bounded slice uses supplied equippability and catalog inventory
classes. Other slot mappings and bank-open enumeration are **unknown**, not
native-empty claims. Non-Wrath behavior is not modeled by this slice; the old
unmodeled no-op is retired. Argument behavior outside the exercised numeric-slot
and table cases is not established.

`tests/wrath_inventory_slot_candidates.rs` exercises the selected original
consumer and observes candidate locations before and after removing one carried
item. It rejects a constant or no-op producer. Equipment-set locking and complete
native equipment eligibility are outside this contract.
