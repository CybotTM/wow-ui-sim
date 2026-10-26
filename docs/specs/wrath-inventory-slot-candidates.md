# Wrath inventory slot candidates

`GetInventoryItemsForSlot` supplies the original Wrath paper-doll flyout with
location-keyed item IDs. The selected `FrameXML/PaperDollFrame.lua:1793` calls
it, and `FrameXML/EquipmentManager.lua:132` decodes its locations. The location
flags and bag shift are defined in `FrameXML/Constants.lua:182–186` of the
profile-scoped Wrath cache. Historical 3.3.5a build 12340 laptop evidence (#7
and 2026-10-26 inventory answer) establishes ring candidates in both finger
slots and carried bags, duplicate item IDs at distinct locations, closed-bank
exclusion, and no results for numeric slots outside 1–19. Exact Wrath Classic
3.4.x behavior remains unknown.

The modeled bounded slice uses supplied equippability and catalog inventory
classes; this supplied equippability is not a claim that native eligibility uses
the same filter. Other slots and bank-open enumeration are not modeled here.
Equip-bypass, Titan's Grip, ammo, transmogrify mode, and third-argument stack
behavior are **unknown** for this slice. Non-Wrath behavior is not modeled; the
old unmodeled no-op is retired. Argument behavior outside the exercised
numeric-slot and table cases is not established.

`tests/wrath_inventory_slot_candidates.rs` exercises the selected original
consumer and observes candidate locations before and after removing one carried
item. It rejects a constant or no-op producer. Equipment-set locking and complete
native equipment eligibility are outside this contract.

`GetInventoryItemGems` projects equipped item `gem_ids` for the original
`FrameXML/EquipmentManager.lua:280` consumer. As a scenario default, zero gem
IDs are empty sockets, trailing empty sockets return no values, and an empty
slot returns no values. Exact native return behavior is unknown.
