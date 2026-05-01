# LFD Dungeon List Empty

The Dungeons & Raids panel showed an empty dungeon list when "Specific Dungeons" was selected. Three missing globals (`GetLFDChoiceCollapseState`, `GetLFDChoiceEnabledState`, `GetLFGLockList`) made `LFGDungeonList_Setup()` error out, and the LFD panel was never told to populate its data provider because `LFG_UPDATE_RANDOM_INFO` did not fire at startup.

## Symptoms

`PVEFrame → Dungeons & Raids → Specific Dungeons` showed `Type: Specific Dungeons` selected but the scroll list was empty. No tooltips, no headers, no entries — just blank background.

Follow-up symptom: the list later populated, but the dungeon checkboxes were not preselected and the queue button stayed disabled. Blizzard's `LFDQueueFrameFindGroupButton_Update()` disables the button when no tank/healer/DPS role is selected, and the simulator still had `GetLFGRoles` as a one-value `false` stub.

## Root Causes

### 1. Missing list-init globals

`Blizzard_GroupFinder/Shared/LFGFrame.lua:1080` defines `LFGDungeonList_Setup()`:

```lua
LFGCollapseList = GetLFDChoiceCollapseState(LFGCollapseList);
LFGEnabledList  = GetLFDChoiceEnabledState(LFGEnabledList);
LFGLockList     = GetLFGLockList();
```

All three globals were unregistered in the simulator, so the first call (`GetLFDChoiceCollapseState`) raised `attempt to call global ... a nil value` and the rest of `Setup` never ran. Subsequent calls returned without doing anything because `hasSetUp = true` had already been set.

### 2. `LFGLockList` read before init

`UpdateLFDDungeonList()` (LFDFrame.lua:697) reads `LFGLockList[id]` *before* `LFGQueueFrame_UpdateLFGDungeonList` runs `LFGDungeonList_Setup`. In retail, `LFGLockList` is populated by `LFG_LOCK_INFO_RECEIVED` (LFGFrame.lua:173) which fires shortly after login. The simulator never fired this event.

### 3. Random/Specific frame never shown

`LFDQueueFrame.Specific` is the actual list container, with `<OnShow function="LFDQueueFrame_Update"/>` (LFDFrame.xml:273). It is `hidden="true"` by default. In retail, `LFG_UPDATE_RANDOM_INFO` fires at startup and `LFDQueueFrame_OnEvent` calls `LFDQueueFrame_SetType(...)` which shows either Specific, Follower, or Random — triggering OnShow and populating the list. The simulator never fired this event.

### 4. Header marked as random

`default_lfd_dungeons` had `is_random=true` on the negative-id header. This made `GetRandomDungeonBestChoice()` return `-1`, which `LFDQueueFrame_SetType(-1)` then routed through `GetLFGDungeonInfo(-1)` and `LFDQueueFrame_SetTypeRandomDungeon` — which calls `GetLFGDungeonRewards` on the header.

### 5. Empty selection and role state

`GetLFDChoiceEnabledState()` returned an empty table, so Blizzard's row initializer treated every specific dungeon as unchecked. `GetLFGRoles()` was also a false stub and `SetLFGRoles()` was a no-op, so role checkbox state could not be initialized or persisted. The LFD queue button is role-gated before queue state checks, so this left "Join as Party" greyed out even with visible dungeons.

### 6. Adventure Journal dungeon action used the wrong id family

Clicking a dungeon entry in the Adventure Journal fires `AJ_DUNGEON_ACTION`, and `LFDFrame_OnEvent` passes that id through `DungeonAppearsInRandomLFD()` before showing the Group Finder panel. The simulator had no `DungeonAppearsInRandomLFD` global, and `C_EncounterJournal.GetInstanceInfo`/`EJ_GetInstanceInfo` returned the Encounter Journal instance id as `linkDungeonID` for dungeon rows. That id family does not match the seeded LFD dungeon ids consumed by `GetLFGDungeonInfo`.

## Fix

Implementation:

- **`battlefield_lfg_probes.rs`**: registered seven new globals — `GetLFDChoiceCollapseState`, `GetLFDChoiceEnabledState`, `GetLFGLockList`, `GetBestRFChoice`, `GetRandomScenarioBestChoice`, `GetLFGDungeonRewards` (all return empty/nil since the sim has no server state).
- **`group_queries.rs`**: added `UnitHasLFGRandomCooldown` → `false`.
- **`runtime_surface_bootstrap.lua`**: added Lua-fallback stubs for `GetNumRandomScenarios`, `GetRandomScenarioInfo`, `GetLFDRoleRestrictions`, `GetLFGRoleShortageRewards`.
- **`startup.rs`**: fire `LFG_UPDATE_RANDOM_INFO` in the post-login event sequence so `LFDQueueFrame_SetType` runs and the Specific/Random sub-frame becomes visible (`OnShow` populates the data provider).
- **`workarounds.rs`** (`patch_lfg_lock_list`): assign `LFGLockList = GetLFGLockList()` directly instead of firing `LFG_LOCK_INFO_RECEIVED`. The event would also wake up RaidFinder and ScenarioFinder, which require many additional unmodeled APIs (`GetNumRFDungeons`, etc.). Direct assignment satisfies the LFD panel without that cascade.
- **`state.rs`** (`default_lfd_dungeons`): split the random header (negative id, `is_random=false`) from a real positive-id "Random Heroic Dungeon" entry (`id=999, is_random=true`). Matches retail data shape.

Follow-up implementation:

- **`state.rs` / `collections.rs`**: added persistent `lfg_roles` and `lfd_enabled_dungeons` state.
- **`battlefield_lfg_probes.rs`**: added state-backed `GetLFGRoles`, `SetLFGRoles`, and `SetLFGDungeonEnabled`; `GetLFDChoiceEnabledState` now defaults joinable positive non-follower specific dungeons to checked.
- **`global_stubs.rs`**: removed `GetLFGRoles` / `SetLFGRoles` from static stubs so the state-backed implementations own those globals.
- **`tests/lfd_globals.rs`**: covers default DPS role selection, role persistence, default specific-dungeon checkbox selection, and `SetLFGDungeonEnabled` persistence.

Adventure Journal follow-up:

- **`battlefield_lfg_probes.rs`**: added state-backed `DungeonAppearsInRandomLFD`, returning `LE_LFG_CATEGORY_LFD` only for positive seeded LFD dungeon ids.
- **`encounter_journal.rs`**: maps current seeded dungeon instance names to their LFD ids for the `linkDungeonID` return.
- **`tests/lfd_globals.rs` / `tests/c_encounter_journal_probes.rs`**: cover the new LFD membership global and the Encounter Journal → LFD id bridge.

Stack-overflow follow-up:

- **`encounter_journal.rs`**: split the modern `C_EncounterJournal.GetInstanceInfo` shape from legacy `EJ_GetInstanceInfo`. The modern C API keeps `linkDungeonID` in return slot 9, while Blizzard's legacy Encounter Journal Lua uses slot 9 as `shouldDisplayDifficulty` and slot 12 as `isRaid`.
- **`buttons.rs` / `frame.rs`**: added a same-button `Button:Click()` reentry guard. The Adventure Journal dungeon display path can execute a programmatic tab click while another handler is still on the stack; the guard prevents that from becoming a native stack overflow.
- **`tests/panel_harness_runtime.rs`**: covers both direct dungeon display (`EncounterJournal_DisplayInstance(1271)`) and the `AJ_DUNGEON_ACTION` LFD handoff.

## Why direct LFGLockList assignment over event firing

Firing `LFG_LOCK_INFO_RECEIVED` triggers `RaidFinderFrame_OnEvent` → `GetBestRFChoice` → `RaidFinderFrame_UpdateAvailability` → `GetNumRFDungeons` → `ScenarioFinderFrame_UpdateAvailability` → `GetNumRandomScenarios`. None of those exist in the sim. We could stub all of them, but the goal is just to populate `LFGLockList` for the LFD panel; the event broadcast is wider than needed.

## Sources

- `Interface/BlizzardUI/Blizzard_GroupFinder/Shared/LFGFrame.lua` — `LFGDungeonList_Setup` (line 1080), `LFG_LOCK_INFO_RECEIVED` handler (line 173)
- `Interface/BlizzardUI/Blizzard_GroupFinder/Mainline/LFDFrame.lua` — `UpdateLFDDungeonList` (line 686), `LFG_UPDATE_RANDOM_INFO` handler (line 80), `AJ_DUNGEON_ACTION` handler (line 106)
- `Interface/BlizzardUI/Blizzard_GroupFinder/Mainline/LFDFrame.xml` — Specific frame `OnShow="LFDQueueFrame_Update"` (line 273)
- `Interface/BlizzardUI/Blizzard_EncounterJournal/Mainline/Blizzard_EncounterJournal.lua` — Adventure Journal suggestion click calls `C_AdventureJournal.ActivateEntry`; dungeon display uses `EJ_GetInstanceInfo` legacy slots and programmatic tab `:Click()`
- `src/lua_api/globals/battlefield_lfg_probes.rs` — simulator LFD globals and selection/role state registration
- `src/lua_api/globals/missing_surface/encounter_journal.rs` — simulator Encounter Journal C API and legacy EJ tuple implementations
- `src/lua_api/frame/methods/button_anchor_hierarchy/buttons.rs` — simulator `Button:Click()` dispatch
- `tests/lfd_globals.rs` — regression coverage for LFD role and checkbox state

## See Also

- [[generated-stubs-audit]] — broader stub-gap pattern this fits into
- [[event-system]] — startup event sequence in `startup.rs`
