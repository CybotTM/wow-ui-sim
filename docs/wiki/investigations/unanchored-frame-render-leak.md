# Unanchored Frame Render Leak

Unanchored frames have no valid WoW rect, but the simulator render path was still assigning them a parent-origin fallback rectangle. That made inactive addon UI, such as RaiderIO's hidden-region search box, appear at startup.

## Content

### Root Cause

Lua geometry already treated unanchored frames as invalid: `GetRect()` returns no values unless a frame has anchors, is `UIParent`, or is the root frame. Rendering did not follow that contract. `build_render_list()` called `compute_frame_rect()` for every visible frame, and `compute_frame_rect()` uses `anchorless_rect()` to place anchorless frames at the parent's top-left.

That fallback is useful inside layout computation for special cases, but it is not a valid render rect for ordinary unanchored UI. RaiderIO creates a `regionBox` search `EditBox` under `UIParent` and only reparents/anchors it when debug mode is enabled. In normal mode the box remains unanchored, so the simulator drew it at `UIParent` origin.

### Fix

`build_render_list()` now requires a renderable rect for the frame and its ancestors. A frame renders only when it has anchors, is `UIParent`/root, or is the statusbar bar child special case. Descendants of an unanchored frame are skipped too, preventing anchored child textures from leaking through after the parent frame is skipped.

### Verification

- Unit coverage checks that unanchored child frames and their descendants do not enter the render list, while anchored children still render.
- A reduced startup screenshot that creates an unanchored `AutoCompleteEditBoxTemplate` edit box no longer shows the editbox border/text at the top-left origin.

## Sources

- [strata_emit.rs](../../../src/iced_app/strata_emit.rs) — render-list filtering and tests.
- [rect_geometry.rs](../../../src/lua_api/rect_geometry.rs) — unanchored frames have no queryable rect.
- [RaiderIO core.lua](</syncthing/World of Warcraft/_classic_/Interface/AddOns/RaiderIO/core.lua>) — creates the search region box and only anchors it in debug mode.

## See Also

- [[rendering-pipeline]] — frame render-list construction.
- [[editbox-render-text-cache]] — other EditBox rendering behavior.
