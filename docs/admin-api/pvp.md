# PvP

### A_Admin.SetPvPEnabled(enabled)

Toggles the player's PvP flag.

- **enabled** `boolean`
- **Affects:** `UnitIsPVP("player")`
- **Fires:** `UPDATE_FACTION`
- **Example:**
```lua
A_Admin.SetPvPEnabled(true)
print(UnitIsPVP("player"))  -- true
```

### A_Admin.SetHonorLevel(level)

Sets the player's honor level.

- **level** `number` -- Honor level (1-500)
- **Affects:** `UnitHonorLevel("player")`
- **Example:**
```lua
A_Admin.SetHonorLevel(50)
print(UnitHonorLevel("player"))     -- 50
```
