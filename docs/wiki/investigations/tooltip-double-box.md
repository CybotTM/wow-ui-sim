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

## Outstanding items (see PLAN.md #53)

- [ ] Check whether the spell tooltip path the spellbook button uses is a
  different widget (a Frame+BackdropTemplate) that gets both
  `build_frame_quads` AND nine-slice child rendering (the "double render"
  hypothesis). If the spellbook uses `SharedTooltipDefaultContainer` wrapper
  rather than `GameTooltip` directly, the duplicated container is the
  visible "second box".
- [ ] Track the orphaned `SharedTooltipDefaultContainer`: the LOW:1 copy vs
  LOW:2 copy — which one gets tooltip content parented to it at runtime,
  and does the other one render anything?
- [ ] Fix either by suppressing the frame backdrop for `GameTooltip`-typed
  widgets when a nine-slice child is present, or by reparenting /
  deleting the orphaned `SharedTooltipDefaultContainer`.

## Screenshots

* `/tmp/claude/tooltip_repro.webp` — `GameTooltip:SetSpellByID(853)`,
  taken 2026-04-15. Shows a single visible GameTooltip. User's reported
  "double box" is not triggered by the direct API path; it is specific to
  whatever the spellbook button OnEnter actually does.
