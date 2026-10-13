# Zone & Instance

### A_Admin.SetZone(name, zoneId)

Sets the current zone name and ID.

- **name** `string` -- Zone name (e.g., `"Orgrimmar"`, `"Stormwind City"`)
- **zoneId** `number` -- Zone area ID
- **Affects:** `GetZoneText()`, `GetCurrentMapAreaID()`
- **Fires:** `ZONE_CHANGED_NEW_AREA`
- **Example:**
```lua
A_Admin.SetZone("Orgrimmar", 1637)
print(GetZoneText())    -- "Orgrimmar"
```

### A_Admin.SetSubZone(name)

Sets the current sub-zone name (interior area within a zone).

- **name** `string` -- Sub-zone name (e.g., `"The Valley of Strength"`)
- **Affects:** `GetSubZoneText()`
- **Fires:** `ZONE_CHANGED`
- **Example:**
```lua
A_Admin.SetSubZone("The Valley of Strength")
print(GetSubZoneText())     -- "The Valley of Strength"
```

### A_Admin.SetInstanceInfo(name, instanceType, difficulty, maxPlayers, lfgDungeonID?)

Configures the current instance information and sets `in_instance` to true.

- **name** `string` -- Instance name (e.g., `"Blackrock Depths"`)
- **instanceType** `string` -- Instance category: `"none"`, `"party"`, `"raid"`, `"pvp"`, `"arena"`, `"scenario"`
- **difficulty** `number` -- Difficulty ID (e.g., `1` = Normal, `2` = Heroic, `14` = Normal Raid)
- **maxPlayers** `number` -- Maximum group size for this instance
- **lfgDungeonID** `number?` -- Current LFG dungeon ID; omitted or nil clears the previous ID. Stored without catalog validation or inference from other inputs.
- **Affects:** `GetInstanceInfo()` -- existing first four inputs retain their outputs; tenth result is the supplied LFG ID or nil. `IsInLFGDungeon()` reports active instance with any supplied ID. `C_LFGInfo.IsInLFGFollowerDungeon()` reports active instance with a known follower-classified ID; unknown/missing/non-follower IDs return false.
- **Example:**
```lua
A_Admin.SetInstanceInfo("Blackrock Depths", "party", 1, 5)
local name, itype, diff, diffName, max = GetInstanceInfo()
-- name="Blackrock Depths", itype="party", max=5
```

### A_Admin.SetInInstance(inInstance)

Toggles the IsInInstance() return value without clearing the current LFG dungeon ID.

- **inInstance** `boolean`
- **Affects:** `IsInInstance()`, `IsInLFGDungeon()`, `C_LFGInfo.IsInLFGFollowerDungeon()`; outside-instance membership is false even with a retained LFG ID.
- **Example:**
```lua
A_Admin.SetInInstance(true)
local isInstance, instanceType = IsInInstance()
-- isInstance=true
```
