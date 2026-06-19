# Addon Load Order Investigation

> **Status: RESOLVED (verified 2026-06-19).** This symptom no longer reproduces.
> `PaperDollItemSlotButton_OnLoad` is defined by the time the bag buttons load,
> their `OnLoadInternal` runs to completion (e.g.
> `CharacterBag0Slot:IsEventRegistered("ITEM_LOCK_CHANGED")` is `true`, which is
> set after the OnLoad call), and `lua-errors` reports no PaperDoll errors. The
> `workarounds_bags.rs` re-run referenced below has been removed (`70fca4e25`
> dropped the replay, `d4f1287f9` deleted the file). The analysis below is kept
> for historical context. See the wiki version:
> [investigations/addon-load-order.md](wiki/investigations/addon-load-order.md).

## Problem

`Blizzard_MainMenuBarBagButtons` calls `PaperDollItemSlotButton_OnLoad(self)` during OnLoad, but that function is defined in `Blizzard_UIPanels_Game/Mainline/PaperDollFrame.lua` which loads later.

## Load Order Source

Blizzard ships `Interface/ui-toc-list.txt` — a manifest that defines the exact addon load order. Wowless uses this file directly. Our loader uses topological sort on TOC dependencies + alphabetical tiebreaking, which produces the same relative order for these addons.

Relevant positions in `ui-toc-list.txt`:
- Line 138: `Blizzard_FrameXMLUtil`
- Line 209: `Blizzard_MainMenuBarBagButtons`
- Line 327: `Blizzard_UIPanels_Game`

## TOC Analysis

`Blizzard_MainMenuBarBagButtons_Mainline.toc` has **no dependency declarations at all** — no `Dependencies`, `RequiredDep`, `LoadFirst`, or `LoadWith`. Just `AllowLoad: Game`.

## What Happens During Load

1. `Blizzard_MainMenuBarBagButtons` loads:
   - `MainMenuBarBagButtons.lua` defines `BaseBagSlotButtonMixin.OnLoadInternal` (calls `PaperDollItemSlotButton_OnLoad`)
   - `MainMenuBarBagButtons.xml` creates concrete frames (`MainMenuBarBackpackButton`, `CharacterBag0Slot`, etc.) parented to `UIParent`
   - OnLoad fires immediately → `PaperDollItemSlotButton_OnLoad` is nil → **error**
   - Rest of `OnLoadInternal` is skipped (event registration, slot setup, etc.)

2. `Blizzard_UIPanels_Game` loads later:
   - `PaperDollFrame.lua:1496` defines `PaperDollItemSlotButton_OnLoad`

## Historical Workaround (removed)

`src/lua_api/workarounds_bags.rs` used to re-run `PaperDollItemSlotButton_OnLoad`, `PaperDollItemSlotButton_Update`, and `UpdateTextures` on each bag button after all addons loaded. This replay was dropped (`70fca4e25`) and the file later deleted (`d4f1287f9`) once OnLoad began completing on its own. The only surviving bag-adjacent workaround is `src/lua_api/workarounds/temporary/character_frame_surface_refresh.rs`, which re-runs `PaperDollItemSlotButton_Update` and icon textures on the character panel — not `_OnLoad`.

## Resolution

**The real client has this same error.** Both WoW and wowless wrap OnLoad script handlers in `xpcall` — the error is caught, the rest of `OnLoadInternal` is skipped, but frame creation continues. Confirmed in wowless: `wowless/modules/security.lua` uses `xpcall(fun, ErrorHandler, ...)` for all script dispatch.

Our loader does the same (`loader/xml_lifecycle.rs` catches OnLoad errors and continues). In the current build OnLoad no longer errors, so no recovery is needed; historically, later events (`PLAYER_ENTERING_WORLD`, `BAG_UPDATE_DELAYED`) and the now-removed replay re-ran any skipped initialization.

**There is no missing load order mechanism.** The architectural conclusion still holds — relying on `xpcall` recovery mirrors the real client — but the specific partial-OnLoad failure no longer occurs.

## Other Functions Called Before Defined

Same pattern — `Blizzard_MainMenuBarBagButtons` also uses:
- `PaperDollItemSlotButton_Update` (from `Blizzard_UIPanels_Game`)
- `PaperDollItemSlotButton_OnShow` (from `Blizzard_UIPanels_Game`)
- `ItemButtonUtil.*` (from `Blizzard_FrameXMLUtil`, line 138 — this one IS loaded first)

## Addons With LoadFirst (Mainline)

Only 6 Mainline addons have `LoadFirst: 1`:
- Blizzard_EnvironmentCleanup
- Blizzard_FrameXML
- Blizzard_GlueMenuFrame
- Blizzard_GlueParent
- Blizzard_GlueXMLBase
- Blizzard_GlueXML

Plus shared (non-flavor) TOCs: Blizzard_ClassTrial, Blizzard_Flyout, Blizzard_MapCanvasSecureUtil, Blizzard_Menu, Blizzard_CatalogShopSharedTemplates, Blizzard_CatalogShopSharedUtil.

`Blizzard_UIPanels_Game` has `LoadFirst` only on Classic TOC, not Mainline.

## ui-toc-list.txt

Located at `vendor/wow-ui-source/Interface/ui-toc-list.txt`. This is the authoritative load order from the real WoW client. Our loader should consider using this file instead of computing order from TOC dependencies for Blizzard addons.
