# Talent Panel OnUpdate Loop (Fixed)

## Symptom

Opening the talent panel (Blizzard_PlayerSpells) caused OnUpdate to fire every tick at 90-190ms, dropping the sim to ~5 FPS.

## Root Cause

**All 134 talent buttons had `IsRectValid() = false`.** Their parent chain (ButtonsParent → TalentsFrame) was rect-dirty and the dirty-flag cache system prevented resolution.

`TalentEdgeArrowMixin:UpdatePosition()` (Blizzard_SharedTalentEdgeTemplates.lua:161) checks `startButton:IsRectValid()` — when false, it calls `MarkEdgesDirty(startButton)` → `RegisterOnUpdate()`. This happened for every edge on every tick, creating an infinite loop.

### Why buttons appeared rect-invalid

`is_rect_dirty()` walks up the ancestor chain. When an ancestor (ButtonsParent) was in `rect_dirty_ids`, all descendants appeared dirty. `ensure_layout_rects()` correctly drained `rect_dirty_ids` before OnUpdate, but:

1. **Stale `Some(true)` caches**: `is_rect_dirty()` cached `Some(true)` on all frames in the walked path during ancestor lookups. After `drain_rect_dirty()` cleared the ancestor from `rect_dirty_ids`, descendants retained stale `Some(true)` caches. The next `is_rect_dirty()` call trusted the stale cache and returned `true` without re-walking.

2. **`resolve_rect_if_dirty()` didn't clear ancestors**: When `GetSize()` triggered `resolve_rect_if_dirty(button)`, it only cleared the button's own dirty flag. The ancestor (ButtonsParent) remained in `rect_dirty_ids`, so the next `is_rect_dirty(button)` still found a dirty ancestor.

## Fix (Applied)

### 1. Don't cache dirty results from ancestor walks (`registry.rs`)

`is_rect_dirty()` now only caches `Some(false)` (clean) results on walked paths. Dirty results are never cached because they become stale when ancestors are cleared by `drain_rect_dirty()` or `resolve_rect_if_dirty()`.

### 2. Resolve dirty ancestors in `resolve_rect_if_dirty` (`state.rs`)

`resolve_rect_if_dirty(id)` now walks up the ancestor chain, finds all dirty roots in `rect_dirty_ids`, computes their layout rects, and clears their dirty flags before resolving the target frame.

### 3. Extract data types to `state_types.rs`

Moved `CursorInfo`, `PendingTimer`, `AddonRuntimeMetrics`, `AppFrameMetrics`, `AddonInfo`, `GreatVaultActivity`, `MovementState` from `state.rs` to `state_types.rs` to keep file under 750 lines.

### 4. Remove `workarounds_talents.rs`

The Lua-side workaround (wrapping OnUpdate to clear `definitionInfoCache`) was targeting the wrong cause. Removed entirely.
