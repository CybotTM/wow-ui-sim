# Bag Button

Three separate bugs caused bag buttons to render greyed-out, and a fourth caused container item slot buttons to be invisible inside the opened bag frame.

## Root Causes

### 1. `GetInventorySlotInfo` returned nil texture (FIXED)

Returned `(slotId, nil, false)` instead of `(slotId, fileDataID)`. `PaperDollItemSlotButton_OnLoad` passed nil to `icon:SetTexture()`, leaving no icon.

**Fix**: `slot_texture_file_data_id()` now maps all 19 equipment slot names to WoW fileDataIDs (e.g. `Bag0Slot → 136511`). 19 paperdoll textures converted to WebP.

### 2. `ContainerFrame_GetContainerNumSlots` stub returned 0 (FIXED)

Stub always returned 0, causing `BaseBagSlotButtonMixin:UpdateTextures()` to select `bag-border-empty` (dark embedded icon) instead of `bag-border` (transparent golden ring).

**Fix**: Stub delegates to `bag_slot_count()` returning 16 for bags 0–4.

### 3. `ItemContextOverlay` rendered black 80% overlay (FIXED)

`UpdateItemContextMatching()` failed silently (pcall) because `C_Spell.TargetSpellReplacesBonusTree()` was unstubbed. Left `itemContextMatchResult = nil`. Later, `PLAYER_ENTERING_WORLD` → `SetMatchesSearch(true)` → `UpdateItemContextOverlay` treated nil as "context applies" → rendered `SetColorTexture(0,0,0,0.8)` + `SetShown(true)`.

**Fix**: Post-event workaround directly sets `btn.itemContextMatchResult = DoesNotApply` and calls `UpdateItemContextOverlay()`.

### 4. Container item slot buttons invisible (FIXED)

`apply_single_template` in `template/mod.rs` did not apply `frameLevel` from templates. `ContainerFrameItemButtonTemplate` specifies `frameLevel="10"`, but buttons got parent+1 = MEDIUM:73, below the Bg background at MEDIUM:74.

**Fix**: Added `frame_level_offset` field to `Frame`. Templates set the offset; `propagate_strata_level` uses `parent_level + offset`. Item buttons now land at MEDIUM:82 (parent 72 + offset 10), above background.

## Investigation Notes

- `dump.rs` prints `frame.width x frame.height`, not the computed layout rect. Frames sized by anchors show `0x0` in dumps but render correctly via `layout_rect`.
- Atlas lookup falls back from `bag-border` → `bag-border-2x` (only `-2x` variants exist in atlas data).
- Workaround timing: bag texture fix runs before Lua startup; context overlay fix runs in `apply_post_event` (after `PLAYER_ENTERING_WORLD`).

## Files Modified

- `src/lua_api/globals/c_item_api.rs`, `c_stubs_api.rs`, `c_container_api.rs`
- `src/lua_api/workarounds_bags.rs`
- `src/widget/frame.rs`, `src/lua_api/globals/template/mod.rs`
- `src/lua_api/frame/methods/methods_hierarchy.rs`
- `textures/paperdoll/`

## Sources

- [bag-button-investigation.md](../../bag-button-investigation.md) — full investigation

## See Also

- [[addon-load-order]] — why bag buttons are partially initialized at load time
