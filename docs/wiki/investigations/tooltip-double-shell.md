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

Follow-up: that ownership check must only suppress the fallback border/shell. The tooltip frame still emits a center-only black fill when Lua owns the `NineSlice`, because `GameTooltip.NineSlice` can exist before the simulator has a renderable opaque center texture. Without that fill, underlying UI text can show through the tooltip body.

## 2026-05-18 ElvUI direct tooltip skin regions

ElvUI can add tooltip skin textures directly under `GameTooltip` instead of only using the `GameTooltip.NineSlice` child. The simulator already rendered the keyed `NineSlice` child before the tooltip frame so internal tooltip text stayed on top, but direct texture regions on `GameTooltip` still rendered after the frame. Since tooltip text is emitted by the `GameTooltip` frame renderer itself, those direct ElvUI skin textures covered the item text and appeared as a blank gray rectangle over Character panel item tooltips.

`GameTooltip` now uses tooltip-specific region placement: direct texture regions render before the tooltip frame/text, while direct FontString regions remain deferred above it. The keyed `NineSlice` child still keeps the existing pre-frame behavior.

## Verification

- `cargo test --test tooltip_text` passes, including `test_tooltip_nineslice_child_accessible`
- `iced_app::tooltip::tests::tooltip_renderer_skips_fallback_background_when_lua_nineslice_exists` passes
- `iced_app::tooltip::tests::tooltip_renderer_keeps_opaque_center_when_lua_nineslice_exists` passes
- `state_render_buckets::tests::tooltip_texture_regions_render_before_internal_text_emitter` passes
- Full-addon Mists screenshot repro with `GameTooltip:SetInventoryItem("player", 1)` shows the ElvUI tooltip text above the skinned tooltip body.

## Sources

- [runtime_surface_bootstrap.lua](../../../src/lua_api/env_init/runtime_surface_bootstrap.lua) — removed fake bootstrap tooltip `NineSlice`
- [workarounds.rs](../../../src/lua_api/workarounds.rs) — post-load tooltip `NineSlice` repair
- [tooltip.rs](../../../src/iced_app/tooltip.rs) — fallback background gating
- [quad_builders.rs](../../../src/iced_app/quad_builders.rs) — tooltip background ownership check
- [state_render_buckets.rs](../../../src/lua_api/state_render_buckets.rs) — tooltip-specific region ordering
- [tooltip_text.rs](../../../tests/tooltip_text.rs) — full-env tooltip `NineSlice` regression coverage
- [SharedTooltipTemplates.xml](../../../Interface/BlizzardUI/Blizzard_SharedXML/SharedTooltipTemplates.xml) — Blizzard tooltip template ownership model
- [SharedTooltipTemplates.lua](../../../Interface/BlizzardUI/Blizzard_SharedXML/SharedTooltipTemplates.lua) — Blizzard tooltip backdrop behavior

## See Also

- [[tooltip-alignment]] — tooltip text inset and NineSlice content box alignment
- [[rendering-pipeline]] — where tooltip fallback quads are emitted
