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

### A_Admin.SetInstanceInfo(name, instanceType, difficulty, maxPlayers)

Configures the current instance information.

- **name** `string` -- Instance name (e.g., `"Blackrock Depths"`)
- **instanceType** `string` -- Instance category: `"none"`, `"party"`, `"raid"`, `"pvp"`, `"arena"`, `"scenario"`
- **difficulty** `number` -- Difficulty ID (e.g., `1` = Normal, `2` = Heroic, `14` = Normal Raid)
- **maxPlayers** `number` -- Maximum group size for this instance
- **Affects:** `GetInstanceInfo()` -- returns name, instanceType, difficultyID, difficultyName, maxPlayers, ...
- **Example:**
```lua
A_Admin.SetInstanceInfo("Blackrock Depths", "party", 1, 5)
local name, itype, diff, diffName, max = GetInstanceInfo()
-- name="Blackrock Depths", itype="party", max=5
```

### A_Admin.SetInInstance(inInstance)

Toggles the IsInInstance() return value.

- **inInstance** `boolean`
- **Affects:** `IsInInstance()`
- **Example:**
```lua
A_Admin.SetInInstance(true)
local isInstance, instanceType = IsInInstance()
-- isInstance=true
```
