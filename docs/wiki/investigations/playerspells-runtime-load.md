# PlayerSpells runtime load

## Summary

Retail `PlayerSpellsUtil.ToggleSpellBookFrame()` and `ToggleClassTalentFrame()` can demand-load `Blizzard_PlayerSpells` from keybindings. The simulator must preserve the active rilua call frame while `C_AddOns.LoadAddOn()` runs nested addon loads and `ADDON_LOADED`; otherwise the caller sees a bare `not a function` even though the addon finished loading.

## Findings

- `C_AddOns.LoadAddOn("Blizzard_PlayerSpells")` reached `event Blizzard_PlayerSpells` but still raised after returning to the original Lua call. Preserving `top`, `base`, and `ci` around runtime addon loading keeps the native function ABI stable before pushing `LoadAddOn` return values.
- `PlayerSpellsFrame_LoadUI` is needed before the real addon is loaded, and the fallback should prefer modeled `C_AddOns.LoadAddOn` over `UIParentLoadAddOn`.
- The talents tab can show before some child OnLoad work has run. The temporary PlayerSpells backfill seeds ModelScene camera tables and PvP talent slot indices before showing the ClassTalents tab.
- PvP talent slot defaults must leave `selectedTalentID` nil for empty slots. Lua treats `0` as truthy, causing Blizzard code to query talent ID 0 and index a nil talent info record.

## Verification

- `keybind_n_loads_blizzard_player_spells_and_shows_talents`
- `raw_toggle_spellbook_frame_loads_blizzard_player_spells_and_shows_spellbook`
- `installs_pvp_talent_default_shapes`
- `installs_playerspells_util_bootstrap_defaults`

## 2026-08-27 SpellBook fallback completion

The global `ToggleSpellBook()` path must not treat a successful Lua call as proof that `PlayerSpellsUtil.ToggleSpellBookFrame()` handled the toggle. The temporary bootstrap wrapper can return `nil` without error while a failed `Blizzard_PlayerSpells` load leaves a partially initialized, hidden `PlayerSpellsFrame`; treating that return as handled skipped the simulator fallback and left `SimState.open_panels["SpellBook"]` closed in a bare environment.

The fix checks whether `PlayerSpellsFrame` reached the expected toggled visibility after the helper call. If not, `ToggleSpellBook()` uses its existing `open_panels`/frame-visibility fallback. Real full-UI helper behavior remains Blizzard-owned; the fallback covers only the helper no-op or failure path.

## Sources

- [panel_toggle_verbs.rs](../../../src/lua_api/globals/panel_toggle_verbs.rs) — SpellBook helper/fallback decision
- [panel_toggle_verbs.rs tests](../../../tests/panel_toggle_verbs.rs) — bare-environment regression coverage
- [player_spells_onload_backfill.rs](../../../src/lua_api/workarounds/temporary/player_spells_onload_backfill.rs) — temporary helper bootstrap
