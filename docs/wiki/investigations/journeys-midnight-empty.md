# Journeys Midnight Empty

The Journeys tab rendered its chrome but no content when the expansion dropdown was set to Midnight. Root cause: the simulator defined `LE_EXPANSION_LEVEL_CURRENT` and `LE_EXPANSION_MIDNIGHT` as 11, while `default_major_factions()` only seeded War Within rows with `expansion_filter = 10`, so Blizzard's `JourneysFrameMixin:Refresh()` received an empty `C_MajorFactions.GetMajorFactionIDs(11)` result.

## Content

Blizzard Journeys builds its list entirely from `C_MajorFactions.GetMajorFactionIDs(self.expansionFilter)`. Rows with `ShouldDisplayMajorFactionAsJourney(id) == false` go into `renownJourneyData`; rows with it true go into `encountersJourneyData`.

The simulator fix seeds the four Midnight major factions in `src/lua_api/game_data.rs`: Silvermoon Court, Amani Tribe, Hara'ti, and The Singularity. Their texture kits use the current `majorfactions_icons_<kit>512` atlas names from `data/atlas.rs`: `light`, `sky`, `root`, and `origin`.

Later overlap between the Midnight renown cards and the breadcrumb/header was a separate XML layout-order bug. Blizzard declares `EncounterJournalJourneysFrame` with both `setAllPoints="true"` and explicit anchors to `$parentInset`; the simulator applied explicit anchors first and then applied `setAllPoints`, which cleared those anchors and stretched the Journeys frame to the full `EncounterJournal` parent. The fix makes `setAllPoints` establish default points before explicit XML anchors run, so the inset anchors remain authoritative in both normal XML loading and runtime template-child creation.

Regression coverage now asserts both the C API and the panel path:

- `tests/c_major_factions_globals.rs` checks `GetMajorFactionIDs(LE_EXPANSION_MIDNIGHT)` returns four current rows, not War Within rows.
- `tests/c_major_factions_globals.rs` checks each Midnight texture kit resolves through `C_Texture.GetAtlasInfo`.
- `tests/panel_harness_runtime.rs` checks `EncounterJournal.JourneysFrame:Refresh()` seeds the four current rows.
- `src/loader/tests/xml_basics.rs` checks that `setAllPoints="true"` does not override explicit `$parentInset` anchors.

## Sources

- [src/lua_api/game_data.rs](../../../src/lua_api/game_data.rs) - default major-faction seed data
- [src/c_api/c_major_factions.rs](../../../src/c_api/c_major_factions.rs) - expansion-filtered `GetMajorFactionIDs`
- [src/loader/xml_frame/setup.rs](../../../src/loader/xml_frame/setup.rs) - XML direct-property application order
- [src/lua_api/globals/create_frame/template_chain/runtime.rs](../../../src/lua_api/globals/create_frame/template_chain/runtime.rs) - runtime template-child direct-property application order
- [src/loader/tests/xml_basics.rs](../../../src/loader/tests/xml_basics.rs) - XML anchor precedence regression coverage
- [tests/c_major_factions_globals.rs](../../../tests/c_major_factions_globals.rs) - API regression coverage
- [tests/panel_harness_runtime.rs](../../../tests/panel_harness_runtime.rs) - panel harness regression coverage
- `/syncthing/World of Warcraft/_retail_/BlizzardInterfaceCode/Interface/AddOns/Blizzard_EncounterJournal/Mainline/Blizzard_Journeys.lua` - Blizzard list-building behavior
- `/syncthing/World of Warcraft/_retail_/BlizzardInterfaceCode/Interface/AddOns/Blizzard_EncounterJournal/Mainline/Blizzard_EncounterJournal.xml` - `JourneysFrame` anchors to `$parentInset`

## See Also

- [[lua-api]] - C API namespace registration and Lua compatibility surface
- [[addon-loading]] - Blizzard addon loading in simulator tests
