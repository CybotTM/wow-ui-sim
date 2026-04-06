# Economy & Items

### A_Admin.SetMoney(copper)

Sets the player's total money in copper.

- **copper** `number` -- Total copper amount. Conversion: 1 gold = 10,000 copper (10 silver per gold, 100 copper per silver).
- **Affects:** `GetMoney()`
- **Fires:** `PLAYER_MONEY`
- **Example:**
```lua
A_Admin.SetMoney(1000000)   -- 100 gold
A_Admin.SetMoney(50000)     -- 5 gold
print(GetMoney())           -- 50000

-- Build from components
local gold, silver, copper = 10, 5, 23
A_Admin.SetMoney(gold * 10000 + silver * 100 + copper)
```

### A_Admin.SetItemLevel(ilvl)

Sets the player's displayed average item level.

- **ilvl** `number` -- Item level value
- **Affects:** `GetAverageItemLevel()` -- returns ilvl for all three values (equipped, bag, pvp)
- **Example:**
```lua
A_Admin.SetItemLevel(480)
local equipped, bag, pvp = GetAverageItemLevel()
-- equipped=480
```
