# Menu Element Pool — SetToDefaults Size/Anchor Reset

The Guild roster Mythic+ Rating dropdown rendered as a horizontal stripe spanning most of the screen. Root cause: `Frame:SetToDefaults` did not reset size or clear anchors, so frames recycled through the menu element pool inflated `MeasureFrameExtents` width via stale `GetSize()`.

## Content

### Symptoms

Opening `CommunitiesFrame.GuildMemberListDropdown` produced a menu that stretched far beyond the dropdown's 180px width, with menu item labels truncated to the right edge ("Ac", "Ar", "M+"). Repeatedly opening the same dropdown grew the menu further on each cycle.

### Root Cause

`Menu.lua` `ResetMenuElement` calls `frame:SetToDefaults()` then `Pool_HideAndClearAnchors`. `MeasureFrameExtents` later iterates the pool's frames and computes `width = math.max(width, frame:GetSize())`. If `SetToDefaults` does not reset the frame's size, the previous user's width is read back as the "minimum" for the new menu.

In `Compositor.lua` Blizzard documents the contract:

> SetToDefaults() will set the frame's size to 0,0
> The anchors were already cleared in SetToDefaults()

The simulator had two `SetToDefaults` Rust functions registered on the shared frame metatable:

- `src/lua_api/frame/methods/misc/group_timer.rs::set_to_defaults` — cleared minimap fields.
- `src/lua_api/frame/methods/map_frames.rs::set_to_defaults` — cleared minimap and quest-blob fields.

`map_frames::register_all` runs after `misc::register_all` in `env_init/frames.rs`, so the map_frames version is the active one (the misc registration is dead code). Neither implementation reset size or cleared anchors, so pooled frames carried over their previous dimensions.

Reproduction (`/tmp/claude/repro-probe.lua`): create a 900px-wide synthetic dropdown menu first, then open the Guild dropdown. Before the fix, the Guild menu measured 1036×103 (instead of 180×103), and re-opening it grew it to 1172×103 because the +20 child padding from `MenuStyle1` insets accumulated each cycle.

### Fix

Extended `map_frames::set_to_defaults` to clear anchors and reset size to 0,0, matching real WoW semantics:

```rust
sim.widgets.remove_all_anchor_dependents_for(id);
if let Some(frame) = sim.widgets.get_mut_visual(id) {
    frame.clear_all_points();
    frame.set_size(0.0, 0.0);
    frame.width_is_text_auto = false;
    frame.layout_rect = None;
    // ... existing minimap/quest-blob field clears ...
}
sim.widgets.mark_rect_dirty(id);
```

`frame.layout_rect = None` is required because `frame_size` falls back to `layout_rect` dimensions when the frame is unanchored, which would otherwise return the pre-reset size.

The initializer path in `Menu.lua` `CallInitializers` restores the template size via `frame:SetSize(templateInfo.width, templateInfo.height)` before each measurement, so resetting to 0,0 on release is safe — the next acquisition sees template dimensions, not stale ones.

### Verification

After the fix the repro probe reports the Guild dropdown menu at 180×103 on both first and second open, regardless of any wider menu opened beforehand. Existing minimap_specialized tests (8 tests) still pass.

## Sources

- [Compositor.lua](../../../Interface/BlizzardUI/Blizzard_Menu/Compositor.lua) — documents the size=0,0 / clear-anchors contract
- [Menu.lua](../../../Interface/BlizzardUI/Blizzard_Menu/Menu.lua) — `ResetMenuElement`, `MeasureFrameExtents`, `CallInitializers`
- [map_frames.rs](../../../src/lua_api/frame/methods/map_frames.rs) — active `SetToDefaults` registration on the shared frame metatable
- [env_init/frames.rs](../../../src/lua_api/env_init/frames.rs) — registration order (map_frames overrides misc)

## See Also

- [[dropdown-intrinsic-script-chain]] — the click-handler-side fix that made the menu open in the first place
- [[frame-data-flow]] — frame metatable method dispatch
