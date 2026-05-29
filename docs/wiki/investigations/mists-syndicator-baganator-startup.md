# Mists Syndicator and Baganator Startup

Mists full-profile startup errors in Syndicator and Baganator came from two simulator-side compatibility gaps: Mists/Classic item class labels differ from the default item taxonomy, and TokenUI can load before a real CharacterFrame source split is available.

## Content

### Symptoms

`wow-sim lua-errors` under `--features sound,gui,casc,client-mists` reported a Syndicator `ADDON_LOADED` error from its search keyword initialization, followed by Baganator failing to index `TokenFramePopup`.

### Root Cause

Syndicator's Classic/Mists search table asserts that enUS `C_Item.GetItemClassInfo` and `C_Item.GetItemSubClassInfo` strings match its English keywords. The shared simulator item taxonomy returned mainline-style or placeholder names for several Classic/Mists labels, including `Tradeskill`, missing trade-goods subclasses, exotic weapon subclasses, and singular consumable subclass names.

Baganator's currency panel expected `TokenFramePopup`, but `Blizzard_TokenUI` failed earlier because its XML parented `TokenFrame` to `CharacterFrame`. The Mists cache declares a CharacterFrame dependency without providing the full CharacterFrame/PaperDoll source split, and loading full PaperDoll source as a preload introduced unrelated PaperDoll mixin errors.

### Fix

Keep item taxonomy changes behind the `client-mists` feature with Mists-specific overrides rather than changing the shared item tables. The overrides cover the Classic/Mists labels exercised by Syndicator while preserving default-profile labels.

For TokenUI, create only the minimal hidden `CharacterFrame` and tab anchors before `Blizzard_UIPanels_Game` or `Blizzard_TokenUI` load. The post-load container-frame token tracker patch also ensures `BackpackTokenFrame` exists and reconnects `ContainerFrameSettingsManager.TokenTracker` after TokenUI has loaded.

### Verification

The final debug Mists probes returned no startup Lua errors:

- `timeout 90 target/debug/wow-sim lua-errors` -> `[]`
- `timeout 90 target/debug/wow-sim --no-addons --no-saved-vars lua-errors` -> `[]`

Focused coverage includes Mists item-label tests and the TokenUI integration test that verifies named TokenUI top-level frames are created.

## Sources

- [helpers.rs](../../src/c_api/item_spell/helpers.rs) — Mists-specific item class and subclass overrides
- [character_frame_preload.rs](../../src/mists/character_frame_preload.rs) — minimal Mists CharacterFrame bootstrap
- [container_frame_token_tracker.rs](../../src/lua_api/workarounds/temporary/container_frame_token_tracker.rs) — BackpackTokenFrame and token tracker recovery

## See Also

- [[mists-elvui-startup-compat]] — related Mists addon startup compatibility gaps
- [[addon-startup-settings-and-item-load]] — addon startup failures tied to item API behavior
