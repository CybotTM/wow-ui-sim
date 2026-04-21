# Tooltip Double Shell

Tooltips could render with two visible boxes because the simulator had two competing tooltip chrome paths: an invalid bootstrap-created `NineSlice` surface on the Lua side, and an unconditional Rust fallback background on the render side.

## Root Cause

The early runtime bootstrap created a fake `tooltip.NineSlice` child by calling `CreateFrame("Frame", nil, tooltip)` before Blizzard tooltip templates were loaded. That object was not the real `NineSlicePanelTemplate` surface Blizzard expects, so `SharedTooltip_SetBackdropStyle()` and related tooltip code could operate on a malformed frame tree.

Separately, the Rust tooltip renderer always emitted its own tooltip background and border, even when the Lua/UI layer already owned a `NineSlice` child and its texture pieces.

That produced two failures:

- a malformed Lua `NineSlice` surface could render as a stray extra box
- a real Lua `NineSlice` plus the Rust fallback shell could render duplicate tooltip chrome

## Fix

Removed the bootstrap-time fake `NineSlice` injection from `runtime_surface_bootstrap.lua`.

Added a post-load workaround that creates a tooltip `NineSlice` only after Blizzard tooltip infrastructure is available, using `NineSlicePanelTemplate` and `SharedTooltip_SetBackdropStyle()` so the frame matches the Blizzard ownership model.

Changed the Rust tooltip renderer to draw its fallback background only when the tooltip frame does **not** already expose a `NineSlice` child.

## Verification

- `cargo test --test tooltip_text` passes, including `test_tooltip_nineslice_child_accessible`
- `iced_app::tooltip::tests::tooltip_renderer_skips_fallback_background_when_lua_nineslice_exists` passes

## Sources

- [runtime_surface_bootstrap.lua](../../../src/lua_api/env_init/runtime_surface_bootstrap.lua) — removed fake bootstrap tooltip `NineSlice`
- [workarounds.rs](../../../src/lua_api/workarounds.rs) — post-load tooltip `NineSlice` repair
- [tooltip.rs](../../../src/iced_app/tooltip.rs) — fallback background gating
- [quad_builders.rs](../../../src/iced_app/quad_builders.rs) — tooltip background ownership check
- [tooltip_text.rs](../../../tests/tooltip_text.rs) — full-env tooltip `NineSlice` regression coverage
- [SharedTooltipTemplates.xml](../../../Interface/BlizzardUI/Blizzard_SharedXML/SharedTooltipTemplates.xml) — Blizzard tooltip template ownership model
- [SharedTooltipTemplates.lua](../../../Interface/BlizzardUI/Blizzard_SharedXML/SharedTooltipTemplates.lua) — Blizzard tooltip backdrop behavior

## See Also

- [[tooltip-alignment]] — tooltip text inset and NineSlice content box alignment
- [[rendering-pipeline]] — where tooltip fallback quads are emitted
