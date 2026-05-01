# Journeys Midnight Empty

The Journeys tab rendered its chrome but no content when the expansion dropdown was set to Midnight. Root cause: the simulator defined `LE_EXPANSION_LEVEL_CURRENT` and `LE_EXPANSION_MIDNIGHT` as 11, while `default_major_factions()` only seeded War Within rows with `expansion_filter = 10`, so Blizzard's `JourneysFrameMixin:Refresh()` received an empty `C_MajorFactions.GetMajorFactionIDs(11)` result.

## Content

Blizzard Journeys builds its list entirely from `C_MajorFactions.GetMajorFactionIDs(self.expansionFilter)`. Rows with `ShouldDisplayMajorFactionAsJourney(id) == false` go into `renownJourneyData`; rows with it true go into `encountersJourneyData`.

The simulator fix seeds the four Midnight major factions in `src/lua_api/game_data.rs`: Silvermoon Court, Amani Tribe, Hara'ti, and The Singularity. Their texture kits use the current `majorfactions_icons_<kit>512` atlas names from `data/atlas.rs`: `light`, `sky`, `root`, and `origin`.

Regression coverage now asserts both the C API and the panel path:

- `tests/c_major_factions_globals.rs` checks `GetMajorFactionIDs(LE_EXPANSION_MIDNIGHT)` returns four current rows, not War Within rows.
- `tests/c_major_factions_globals.rs` checks each Midnight texture kit resolves through `C_Texture.GetAtlasInfo`.
- `tests/panel_harness_runtime.rs` checks `EncounterJournal.JourneysFrame:Refresh()` seeds the four current rows.

## Sources

- [src/lua_api/game_data.rs](../../../src/lua_api/game_data.rs) - default major-faction seed data
- [src/c_api/c_major_factions.rs](../../../src/c_api/c_major_factions.rs) - expansion-filtered `GetMajorFactionIDs`
- [tests/c_major_factions_globals.rs](../../../tests/c_major_factions_globals.rs) - API regression coverage
- [tests/panel_harness_runtime.rs](../../../tests/panel_harness_runtime.rs) - panel harness regression coverage
- `/syncthing/World of Warcraft/_retail_/BlizzardInterfaceCode/Interface/AddOns/Blizzard_EncounterJournal/Mainline/Blizzard_Journeys.lua` - Blizzard list-building behavior

## See Also

- [[lua-api]] - C API namespace registration and Lua compatibility surface
- [[addon-loading]] - Blizzard addon loading in simulator tests
