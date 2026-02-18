# Bag Button Investigation

## Problem

Bag buttons (CharacterBag0Slot through CharacterBag3Slot) appeared greyed-out in the simulator.

## Root Causes Found

### 1. `GetInventorySlotInfo` returned nil texture (FIXED, committed)

`GetInventorySlotInfo(slotName)` returned `(slotId, nil, false)` instead of `(slotId, fileDataID)`. The Blizzard code in `PaperDollItemSlotButton_OnLoad` unpacks this and calls `icon:SetTexture(textureName)` — with nil, no texture was set.

**Fix**: Return fileDataIDs matching WoW's modern API. Added `slot_texture_file_data_id()` mapping all 19 equipment slot names to their WoW fileDataIDs (e.g., `Bag0Slot → 136511`). Also converted 19 paperdoll BLP textures to WebP.

Files: `src/lua_api/globals/c_item_api.rs`, `textures/paperdoll/`

### 2. `ContainerFrame_GetContainerNumSlots` stub returned 0 (FIXED, committed)

The stub in `c_stubs_api.rs` always returned 0, telling `BaseBagSlotButtonMixin:UpdateTextures()` that bags had zero slots. This selected the `bag-border-empty` atlas (dark background with embedded bag icon silhouette) instead of `bag-border` (transparent-center golden ring).

**Fix**: Stub now delegates to `bag_slot_count()` which returns 16 for bags 0-4.

Files: `src/lua_api/globals/c_stubs_api.rs`, `src/lua_api/globals/c_container_api.rs`

### 3. `ItemContextOverlay` rendered black 80% overlay on bag icons (FIXED)

`ItemButton`'s `PostOnShow` calls `UpdateItemContextMatching()` during creation. This calls `self:GetItemContextMatchResult()` which for bag buttons calls `ItemButtonUtil.GetItemContextMatchResultForContainer()` → `ItemButtonUtil.GetItemContext()`. `GetItemContext()` calls unstubbed `C_Spell.TargetSpellReplacesBonusTree()` etc., causing errors. The pcalled `UpdateItemContextMatching` fails, leaving `self.itemContextMatchResult = nil`.

Later, `PLAYER_ENTERING_WORLD` fires `UpdateBagMatchesSearch` → `SetMatchesSearch(true)` → `UpdateItemContextOverlay` → `GetItemContextOverlayMode`. With `itemContextMatchResult = nil`: `nil ~= DoesNotApply(3)` → `contextApplies = true` → returns `Standard` → `SetColorTexture(0,0,0,0.8)` + `SetShown(true)`.

**Fix**: Post-event workaround directly sets `btn.itemContextMatchResult = DoesNotApply` and calls `UpdateItemContextOverlay()` to clear the overlay.

Files: `src/lua_api/workarounds_bags.rs`

## Investigation Notes

### dump-tree shows 0x0 for anchor-dependent frames

`dump.rs` prints `frame.width x frame.height` directly, not the computed layout rect. Frames that derive size from anchors (e.g., `SetAllPoints`) show `(0x0)` in the dump even though `compute_frame_rect_cached` correctly resolves them. The rendering pipeline uses `frame.layout_rect` (populated by `ensure_layout_rects()`), not `frame.width`/`frame.height`.

### Bag button texture flow

1. `PaperDollItemSlotButton_OnLoad` → `GetInventorySlotInfo(slotName)` → `icon:SetTexture(fileDataID)`
2. `BaseBagSlotButtonMixin:UpdateTextures()` → `ContainerFrame_GetContainerNumSlots(bagID)` → selects atlas:
   - `bag-border` (slots > 0): golden ring, transparent center
   - `bag-border-empty` (slots = 0): golden ring, dark embedded bag icon
3. Atlas lookup falls back from `bag-border` → `bag-border-2x` (only `-2x` variants exist in atlas data)

### Workaround timing

`Blizzard_MainMenuBarBagButtons` loads before `Blizzard_UIPanels_Game` (which defines the real `ContainerFrame_GetContainerNumSlots`). The workaround in `workarounds_bags.rs:update_bag_button_textures()` re-runs `UpdateTextures()` after all addons load to fix this ordering issue.

The `ItemContextOverlay` fix runs in `apply_post_event` (after startup events) because the overlay is set visible by `PLAYER_ENTERING_WORLD` → `SetMatchesSearch` which fires during startup events.

## Container Storage Slots Investigation (FIXED)

### Problem

`ToggleBag(0)` opens the ContainerFrameCombinedBags frame — the nine-slice border, portrait, money frame all render correctly. But the 80 item slot buttons inside are invisible.

Root cause: item buttons were rendered underneath the background texture because their `frame_level` wasn't being applied from templates during dynamic `CreateFrame` (pool creation).

### What works

- **Bag frame structure**: NineSlice, portrait icon, money display, close button all render
- **Item buttons created**: 80 `ItemButton` frames created via `CreateFramePool("ItemButton", self, "ContainerFrameItemButtonTemplate")` → pool `Acquire()` → `CreateFrame("ItemButton", nil, parent, template)`
- **Buttons shown**: `Initialize()` calls `self:Show()`, buttons report `IsShown()=true`, `GetWidth()=37`, `GetHeight()=37`
- **Textures set**: `.icon` has atlas `bags-item-slot64` → texture `Interface\containerframe\bagsitemslot2x` (webp exists). `NormalTexture` has `Interface\Buttons\UI-Quickslot2`
- **Positions computed**: dump-tree shows buttons at correct positions within bag frame (e.g., `x=1545, y=1053`), `layout_rect` appears set (no `[layout_rect=None]` in dump output)
- **Anchors set**: `AnchorUtil.GridLayout` calls `SetPoint` on each button via `SetPointWithExtraOffset`. First button anchored `BOTTOMRIGHT → MoneyFrame TOPRIGHT (0,4)`, subsequent buttons offset by `(col-1) * (37+5) * -1` horizontally

### What didn't work

- **No quads rendered**: Full screenshot with `WOW_SIM_DEBUG_ELEMENTS=1` showed NO red borders inside bag area — item buttons produced zero quads
- **Only 35 quads**: Filtered screenshot `--filter ContainerFrameCombinedBags` produced only 35 quads (NineSlice + chrome), not the 300+ expected from 80 buttons with child textures

### Fix

`apply_single_template` in `src/lua_api/globals/template/mod.rs` did not apply `frameLevel` from templates. The template `ContainerFrameItemButtonTemplate` specifies `frameLevel="10"`, but this was ignored during dynamic `CreateFrame` (pool creation). Item buttons were assigned the default `frame_level` of parent+1, placing them at MEDIUM:73, which put them underneath the `.Bg.TopSection` background at MEDIUM:74 — hidden beneath it.

The fix introduced a `frame_level_offset` field on the `Frame` struct. Templates now set this offset (e.g., 10 for `ContainerFrameItemButtonTemplate`) instead of an absolute frame level. `propagate_strata_level` uses `parent_level + offset` instead of `parent_level + 1`. This correctly places item buttons at MEDIUM:82 (parent 72 + offset 10), well above the background at MEDIUM:74.

Files modified:
- `src/widget/frame.rs` — added `frame_level_offset` field
- `src/lua_api/globals/template/mod.rs` — `apply_single_template` now reads and stores `frameLevel` as offset
- `src/lua_api/frame/methods/methods_hierarchy.rs` — `propagate_strata_level` uses `parent_level + offset`

### Investigation Notes

#### Rendering pipeline trace

The render pipeline for each frame in `emit_single_strata` (render.rs):

1. **layout_rect check** (line 184): `let Some(rect) = f.layout_rect else { continue }` — skips if no rect
2. **effective_alpha check** (lines 185-199): needs `eff_alpha > 0.0`
3. **should_skip_frame** (button_vis.rs): checks subtree filter, alpha, HIGHLIGHT layer, button state texture visibility
4. **Size check** (line 212): skips if `width <= 0.0` or `height <= 0.0`
5. **emit_frame_quads**: dispatches to type-specific quad builder

### GetRect coordinate divergence (separate known issue)

`GetRect()` returns `L=-47, B=1720` for item 1 (WoW coordinate system), while dump-tree shows `x=1545, y=1053` (screen coordinates). The Lua-side anchor resolution computes different positions than the Rust layout engine. The render uses `layout_rect` (Rust side), which shows correct positions in dump-tree. This coordinate divergence is a separate bug and is unrelated to the rendering failure.
