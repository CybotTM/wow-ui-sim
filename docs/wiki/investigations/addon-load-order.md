# Addon Load Order

Investigation into why `Blizzard_MainMenuBarBagButtons` historically called functions that weren't defined yet at load time.

> **Status: RESOLVED (verified 2026-06-19).** The symptom no longer reproduces in
> the current build. `PaperDollItemSlotButton_OnLoad` is defined by the time the
> bag buttons load, their `OnLoadInternal` runs to completion, and there are no
> related Lua errors at startup. The old re-run workaround has been removed. The
> historical analysis is kept below for context.

## Verification (current build)

- `lua-errors` (full addon load) reports **no** PaperDoll / ItemSlotButton errors.
- At runtime `type(PaperDollItemSlotButton_OnLoad) == "function"`.
- `CharacterBag0Slot` is fully initialized: `GetID()` = 20, has `icon`,
  `UpdateTextures`, and `IsEventRegistered("ITEM_LOCK_CHANGED")` = `true`. That
  event registration happens in `OnLoadInternal` **after** the
  `PaperDollItemSlotButton_OnLoad` call, proving OnLoad completed rather than
  erroring partway and being swallowed by `xpcall`.

## What changed

The OnLoad re-run replay was removed as obsolete:

- `70fca4e25` — "Drop obsolete bag bar workaround replay" (gutted the OnLoad
  re-run from `workarounds_bags.rs`)
- `747cb23b5` — "Skip PaperDollItemSlotButton_OnLoad for backpack button" (the
  backpack's `OnLoadInternal` never calls it; the replay was wrongly forcing it
  and grabbing the head-slot icon)
- `d4f1287f9` — "Remove bag token tracker workaround" (deleted
  `workarounds_bags.rs` entirely)

The only surviving bag-adjacent workaround is
`src/lua_api/workarounds/temporary/character_frame_surface_refresh.rs`, which
re-runs `PaperDollItemSlotButton_Update` and icon textures on the character
panel — **not** `_OnLoad`.

## Historical finding (no longer reproduces)

`Blizzard_MainMenuBarBagButtons` (line 209 in `ui-toc-list.txt`) called
`PaperDollItemSlotButton_OnLoad` during frame creation, but that function is
defined in `Blizzard_UIPanels_Game` (line 327), which loaded later. The bag
buttons' `OnLoadInternal` failed partway through, skipping event registration
and slot setup. The same pattern applied to `PaperDollItemSlotButton_Update` and
`PaperDollItemSlotButton_OnShow`. `ItemButtonUtil.*` (from
`Blizzard_FrameXMLUtil`, line 138) was always fine — it loads before the bag
buttons.

## Historical root cause

`Blizzard_MainMenuBarBagButtons_Mainline.toc` has **no dependency declarations** —
no `Dependencies`, `RequiredDep`, `LoadFirst`, or `LoadWith`. The real WoW client
had the same error; both WoW and wowless wrap OnLoad in `xpcall`, catching the
error and continuing frame creation with the frame partially initialized.

`LoadFirst` was never a fix for this: only 6 Mainline addons carry `LoadFirst: 1`
(FrameXML, Glue frames, etc.), and `Blizzard_UIPanels_Game` is not one of them on
Mainline. Adding it would have been a fabricated ordering the real client doesn't
ship.

## Load Order Source

`vendor/wow-ui-source/Interface/ui-toc-list.txt` is the authoritative load order
from the real WoW client. Our loader uses topological sort on TOC dependencies
with alphabetical tiebreaking, producing the same relative order.

## Sources

- [addon-load-order-investigation.md](../../addon-load-order-investigation.md) — full historical analysis
