# Mists Heirloom Tooltip

Mists `Blizzard_HeirloomCollection.lua` calls `GameTooltip:SetHeirloomByItemID`
from heirloom button `OnEnter`. The simulator had `C_Heirloom` collection data
and generic item tooltip data, but the `GameTooltip` method and matching
`C_TooltipInfo.GetHeirloomByItemID` probe were not registered, so the collection
panel could throw a nil-method Lua error while initializing heirloom button
tooltips.

## Root Cause

The failing path was a tooltip widget API gap, not missing heirloom collection
state. Blizzard's shared tooltip mapping treats `SetHeirloomByItemID` as a
tooltip-data method that resolves through `GetHeirloomByItemID`; the simulator's
tooltip surface already had comparable item-like methods such as `SetToyByItemID`
and `SetItemByID`, but no heirloom route.

## Fix

`GameTooltip:SetHeirloomByItemID(itemID)` is now registered on tooltip widgets and
routes to `C_TooltipInfo.GetHeirloomByItemID(itemID)`, which reuses the existing
item tooltip builder for the item ID. This matches the Blizzard path closely
enough for the Mists Collections heirloom tab: it populates normal item tooltip
lines and fires the item tooltip script path instead of throwing.

## Verification

- `cargo test --test heirloom_probes --no-default-features --features "sound,gui,casc,client-mists"` — 8/8 passed.
- `scripts/mists-panel-parity.sh --skip-build --with-saved-vars --panel collections` — Collections row passed with retained artifacts under `/home/osso/.cache/wow-ui-sim/mists-audits/heirloom-tooltip-fix-collections`.
- Base Mists `lua-errors` after the fix returned `[]`.

## Sources

- `vendor/wow-ui-source-mists/Interface/AddOns/Blizzard_Collections/Classic/Blizzard_HeirloomCollection.lua` — caller uses `GameTooltip:SetHeirloomByItemID`.
- `/home/osso/.cache/wow-ui-sim/blizzard-ui/Blizzard_SharedXMLGame/Tooltip/TooltipDataHandler.lua` — maps `SetHeirloomByItemID` to `GetHeirloomByItemID`.
- `src/lua_api/frame/methods/widgets/tooltip.rs` — tooltip widget method registration.
- `src/lua_api/globals/missing_surface/tooltip_info/` — `C_TooltipInfo` probe registration and item tooltip builders.

## See Also

- [[tooltip-double-box]]
- [[addon-compatibility]]
