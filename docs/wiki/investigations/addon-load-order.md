# Addon Load Order

Investigation into why `Blizzard_MainMenuBarBagButtons` calls functions that aren't defined yet at load time.

## Finding

`Blizzard_MainMenuBarBagButtons` (line 209 in `ui-toc-list.txt`) calls `PaperDollItemSlotButton_OnLoad` during frame creation, but that function is defined in `Blizzard_UIPanels_Game` (line 327), which loads later. The bag buttons' `OnLoadInternal` fails partway through, skipping event registration and slot setup.

## Root Cause

`Blizzard_MainMenuBarBagButtons_Mainline.toc` has **no dependency declarations** — no `Dependencies`, `RequiredDep`, `LoadFirst`, or `LoadWith`. The real WoW client has the same error; both WoW and wowless wrap OnLoad in `xpcall`, catching the error and continuing frame creation with the frame partially initialized.

The same pattern applies to `PaperDollItemSlotButton_Update` and `PaperDollItemSlotButton_OnShow`. `ItemButtonUtil.*` (from `Blizzard_FrameXMLUtil`, line 138) is fine — it loads before the bag buttons.

## Resolution

There is no missing load order mechanism to fix. This is real WoW behavior. The workaround in `src/lua_api/workarounds_bags.rs` re-runs `PaperDollItemSlotButton_OnLoad`, `PaperDollItemSlotButton_Update`, and `UpdateTextures` on each bag button after all addons load — mirroring what the real client recovers through `PLAYER_ENTERING_WORLD` and `BAG_UPDATE_DELAYED` events.

## Load Order Source

`vendor/wow-ui-source/Interface/ui-toc-list.txt` is the authoritative load order from the real WoW client. Our loader uses topological sort on TOC dependencies with alphabetical tiebreaking, producing the same relative order. Only 6 Mainline addons have `LoadFirst: 1` (FrameXML, Glue frames, etc.) — `Blizzard_UIPanels_Game` does not.

## Sources

- [addon-load-order-investigation.md](../../addon-load-order-investigation.md) — full analysis

## See Also

- [[bag-button]] — downstream effects of partial bag button initialization
