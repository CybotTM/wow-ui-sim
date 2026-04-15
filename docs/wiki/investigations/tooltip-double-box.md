---
title: Spell Tooltip Double-Box Rendering
status: investigating
area: rendering
---

# Spell Tooltip Double-Box Rendering

User report: two tooltip boxes render for one spell tooltip (e.g. spellbook
"Hammer of Justice"). Screenshot shows one solid tooltip with a second
offset border box behind it.

## Reproduction

Headless repro (no spellbook hover needed — directly trigger the same
tooltip API the spellbook button OnEnter uses):

```
LD_LIBRARY_PATH=target/debug ./target/debug/wow-sim --no-addons --no-saved-vars \
  --exec-lua '
    GameTooltip:SetOwner(UIParent, "ANCHOR_PRESERVE")
    GameTooltip:SetSpellByID(853)  -- Hammer of Justice
    GameTooltip:Show()
  ' dump-tree --filter Tooltip --visible-only
```

## Dump-tree findings (static, no hover)

All `[GameTooltip]`-typed frames in the tree:

| Name | State | Strata | Source | Notes |
|------|-------|--------|--------|-------|
| `SettingsTooltip` | hidden | TOOLTIP:1 | `Blizzard_Settings_Shared` | |
| `SharedTooltipDefaultContainer` | hidden | LOW:1 | `Blizzard_SharedXML` | **duplicated** |
| `SharedTooltipDefaultContainer` | hidden | LOW:2 | `Blizzard_SharedXML` | **duplicated** |
| `CatalogShopTooltip` | hidden | TOOLTIP:2 | `Blizzard_CatalogShop` | |
| `QuickKeybindTooltip` | hidden | TOOLTIP:2 | `Blizzard_QuickKeybind` | |
| `ShoppingTooltip1` | hidden | TOOLTIP:2 | `Blizzard_GameTooltip` | |
| `ShoppingTooltip2` | hidden | TOOLTIP:2 | `Blizzard_GameTooltip` | |
| `GameTooltip` | hidden | TOOLTIP:2 | `Blizzard_GameTooltip` | `.Tooltip` child |
| `EmbeddedItemTooltip` | hidden | TOOLTIP:2 | `Blizzard_GameTooltip` | `.Tooltip` child |
| `GameNoHeaderTooltip` | hidden | TOOLTIP:2 | `Blizzard_GameTooltip` | |
| `GameSmallHeaderTooltip` | hidden | TOOLTIP:2 | `Blizzard_GameTooltip` | |
| `ItemRefShoppingTooltip1/2` | hidden | TOOLTIP:2 | `Blizzard_UIPanels_Game` | |
| `ItemRefTooltip` | hidden | TOOLTIP:2 | `Blizzard_UIPanels_Game` | `.Tooltip` child |
| `PrivateAurasTooltip` | hidden | TOOLTIP:2 | `Blizzard_PrivateAurasUI` | |

Total 19 `GameTooltip`-type widgets in the tree.

## Symptoms confirmed via dump

1. **`SharedTooltipDefaultContainer` is duplicated**. Two entries at identical
   coordinates `(250x150) @ x=1341,y=965` on frame levels `LOW:1` and `LOW:2`.
   Both are parented to `Blizzard_SharedXML`'s default container. Loader
   likely created one in the pre-register phase and another via XML. This
   is the frame-re-creation pattern documented in `CLAUDE.md` ("Frame
   Re-creation and Orphaned Children").

2. **`GameTooltip` and `AddonTooltip` resolve to the same userdata**:
   ```
   > tostring(GameTooltip)      → GameTooltip: 0x00000D00
   > tostring(AddonTooltip)     → GameTooltip: 0x00000D00
   > GameTooltip == AddonTooltip → true
   ```
   That means either `AddonTooltip = GameTooltip` aliasing happens somewhere,
   or `AddonTooltip` is resolved to the same widget via the name lookup
   fallback. Not necessarily a bug on its own, but worth noting since it
   confused an earlier iteration of this investigation (counting each once
   through globals iteration reported "two visible tooltips" when it was the
   same frame).

## When only the GameTooltip API is used, only one tooltip is visible

Triggering via `GameTooltip:SetSpellByID(853); GameTooltip:Show()` shows one
`GameTooltip` visible at TOOLTIP:2. None of the other tooltip frames become
visible in this path.

## Backdrop double-render check (PLAN.md #53 checkbox 2)

**The spell tooltip itself does NOT take the Frame+BackdropTemplate path.**
`GameTooltip:SetSpellByID(...)` dispatches through `emit_tooltip_quads` →
`build_tooltip_quads` → `emit_nine_slice_with_center_color`
(`src/iced_app/tooltip.rs:266`). One tooltip, one render call.

However, `build_frame_quads` (`src/iced_app/quad_builders.rs:50-52`)
unconditionally emits a 2 px `(0.6, 0.45, 0.15)` gold fallback border on
any `WidgetType::Frame` / `WidgetType::StatusBar` whose
`nine_slice_layout.is_some()`:

```rust
if f.nine_slice_layout.is_some() {
    batch.push_border(bounds, 2.0, [0.6, 0.45, 0.15, alpha]);
}
```

There is no matching "real" nine-slice rendering for a plain `Frame`; the
nine-slice pieces actually draw through the child textures
(`quad_builders_textures.rs:22` → `emit_nine_slice_atlas`). When a
`NineSliceContainer` child renders the proper slice and the parent Frame
ALSO emits that 2 px debug border, the visible result is a solid
tooltip-looking frame with a thin gold rectangle behind it — the exact
"offset border box" symptom the user reported.

This affects Frame+BackdropTemplate widgets (which is the path other
tooltip-like frames might take), not `GameTooltip`-typed widgets. It
still needs removal because the fallback is a leftover debug marker:
nothing relies on it for correctness, and any frame with a
`nine_slice_layout` set already has a proper nine-slice render path
through its child textures.

## Outstanding items (see PLAN.md #53)

- [ ] Track the orphaned `SharedTooltipDefaultContainer`: the LOW:1 copy vs
  LOW:2 copy — which one gets tooltip content parented to it at runtime,
  and does the other one render anything?
- [ ] Fix the root cause — remove the unconditional 2 px fallback in
  `build_frame_quads`, and/or reparent / delete the orphaned
  `SharedTooltipDefaultContainer`.

## Screenshots

* `/tmp/claude/tooltip_repro.webp` — `GameTooltip:SetSpellByID(853)`,
  taken 2026-04-15. Shows a single visible GameTooltip. User's reported
  "double box" is not triggered by the direct API path; it is specific to
  whatever the spellbook button OnEnter actually does.
