# Admin API (A_Admin)

The `A_Admin` namespace provides administrative functions for controlling simulator state from Lua. These functions are unique to the simulator and do not exist in the real WoW client. Use them in test scripts, `--exec-lua` commands, or addon test files to set up specific game states.

**Implementation file:** `src/lua_api/globals/admin_api.rs`

---

## Quick Reference

### Player Identity

| Function | Description |
|----------|-------------|
| `A_Admin.SetPlayerName(name)` | Set character name |
| `A_Admin.SetPlayerClass(classIndex)` | Set class by 1-based index |
| `A_Admin.SetPlayerRace(raceIndex)` | Set race by 0-based index |
| `A_Admin.SetPlayerLevel(level)` | Set character level |
| `A_Admin.SetPlayerSex(sex)` | Set sex (1=unknown, 2=male, 3=female) |

### Combat State

| Function | Description |
|----------|-------------|
| `A_Admin.SetInCombat(inCombat)` | Toggle combat lockdown |
| `A_Admin.SetCasting(spellId, spellName, iconPath, duration)` | Start a cast bar |
| `A_Admin.StopCasting()` | Cancel current cast |
| `A_Admin.SetGCD(duration)` | Trigger global cooldown |
| `A_Admin.SetSpellCooldown(spellId, duration)` | Set spell-specific cooldown |

### Health & Power

| Function | Description |
|----------|-------------|
| `A_Admin.SetPlayerHealth(current, max)` | Set player health |
| `A_Admin.SetPlayerPower(current, max [, powerType])` | Set player power/resource |
| `A_Admin.SetTargetHealth(current, max)` | Set target health |

### Targeting

| Function | Description |
|----------|-------------|
| `A_Admin.SetTarget(name, level, classIndex, isEnemy)` | Create a custom target |
| `A_Admin.ClearTarget()` | Remove current target |
| `A_Admin.SetFocus(name, level, classIndex, isEnemy)` | Create a custom focus target |
| `A_Admin.ClearFocus()` | Remove current focus |

### Party

| Function | Description |
|----------|-------------|
| `A_Admin.SetPartySize(n)` | Set number of party members (0-4) |
| `A_Admin.SetPartyMember(index, name, classIndex, level)` | Configure a party member |
| `A_Admin.SetPartyMemberHealth(index, current, max)` | Set a member's health |
| `A_Admin.KillPartyMember(index)` | Set a member's health to zero |
| `A_Admin.ResPartyMember(index)` | Restore a dead member to full health |
| `A_Admin.SetRotDamage(level)` | Set automatic rot damage intensity |

### Movement

| Function | Description |
|----------|-------------|
| `A_Admin.SetMoving(bool)` | Toggle IsPlayerMoving() |
| `A_Admin.SetMounted(bool)` | Toggle IsMounted() |
| `A_Admin.SetFlying(bool)` | Toggle IsFlying() |
| `A_Admin.SetFalling(bool)` | Toggle IsFalling() |
| `A_Admin.SetSwimming(bool)` | Toggle IsSwimming() |

### Specialization & Talents

| Function | Description |
|----------|-------------|
| `A_Admin.SetSpec(specIndex)` | Set active specialization |
| `A_Admin.SetTalentRank(nodeId, rank)` | Set talent node rank |
| `A_Admin.SetTalentSelection(nodeId, entryId)` | Select a choice node entry |
| `A_Admin.ResetTalents()` | Clear all talent purchases |

### Buffs & Auras

| Function | Description |
|----------|-------------|
| `A_Admin.AddBuff(spellId, name, icon, duration, stacks)` | Add a player buff |
| `A_Admin.RemoveBuff(spellId)` | Remove a specific buff |
| `A_Admin.ClearBuffs()` | Remove all player buffs |

### Zone & Instance

| Function | Description |
|----------|-------------|
| `A_Admin.SetZone(name, zoneId)` | Set current zone name and ID |
| `A_Admin.SetSubZone(name)` | Set current sub-zone name |
| `A_Admin.SetInstanceInfo(name, instanceType, difficulty, maxPlayers)` | Configure instance info |
| `A_Admin.SetInInstance(bool)` | Toggle IsInInstance() |

### Economy & Items

| Function | Description |
|----------|-------------|
| `A_Admin.SetMoney(copper)` | Set player money in copper |
| `A_Admin.SetItemLevel(ilvl)` | Set average item level |

### Collections

| Function | Description |
|----------|-------------|
| `A_Admin.AddTransmog(appearanceId)` | Mark a transmog appearance as collected |
| `A_Admin.RemoveTransmog(appearanceId)` | Mark a transmog appearance as uncollected |
| `A_Admin.SetMountCollected(mountId, collected)` | Toggle a mount as collected |
| `A_Admin.SetPetCollected(petId, collected)` | Toggle a battle pet as collected |
| `A_Admin.SetToyCollected(toyId, collected)` | Toggle a toy as collected |
| `A_Admin.SetAchievementEarned(achieveId, earned)` | Toggle an achievement as earned |

### PvP

| Function | Description |
|----------|-------------|
| `A_Admin.SetPvPEnabled(enabled)` | Toggle PvP flag |
| `A_Admin.SetHonorLevel(level)` | Set honor level |

### Guild

| Function | Description |
|----------|-------------|
| `A_Admin.SetGuildInfo(name, rank, numMembers)` | Configure guild membership |
| `A_Admin.ClearGuild()` | Remove the player from any guild |

### Events

| Function | Description |
|----------|-------------|
| `A_Admin.FireEvent(event, ...)` | Fire a game event with optional arguments |

---

## Player Identity

### A_Admin.SetPlayerName(name)

Sets the player character's name.

- **name** `string` -- The character name to display
- **Affects:** `UnitName("player")`, `GetUnitName("player")`
- **Example:**
```lua
A_Admin.SetPlayerName("Arthas")
print(UnitName("player"))  -- "Arthas"
```

---

### A_Admin.SetPlayerClass(classIndex)

Sets the player's class by 1-based index.

- **classIndex** `number` -- 1-based class index:

| Index | Class | File Token |
|-------|-------|------------|
| 1 | Warrior | WARRIOR |
| 2 | Paladin | PALADIN |
| 3 | Hunter | HUNTER |
| 4 | Rogue | ROGUE |
| 5 | Priest | PRIEST |
| 6 | Death Knight | DEATHKNIGHT |
| 7 | Shaman | SHAMAN |
| 8 | Mage | MAGE |
| 9 | Warlock | WARLOCK |
| 10 | Monk | MONK |
| 11 | Druid | DRUID |
| 12 | Demon Hunter | DEMONHUNTER |
| 13 | Evoker | EVOKER |

- **Affects:** `UnitClass("player")`, `UnitClassBase("player")`
- **Example:**
```lua
A_Admin.SetPlayerClass(2)   -- Paladin
local name, file, idx = UnitClass("player")
-- name="Paladin", file="PALADIN", idx=2
```

---

### A_Admin.SetPlayerRace(raceIndex)

Sets the player's race by 0-based index.

- **raceIndex** `number` -- 0-based race index:

| Index | Race | Faction |
|-------|------|---------|
| 0 | Human | Alliance |
| 1 | Orc | Horde |
| 2 | Dwarf | Alliance |
| 3 | Night Elf | Alliance |
| 4 | Undead | Horde |
| 5 | Tauren | Horde |
| 6 | Gnome | Alliance |
| 7 | Troll | Horde |
| 8 | Blood Elf | Horde |
| 9 | Draenei | Alliance |
| 10 | Worgen | Alliance |
| 11 | Goblin | Horde |
| 12 | Pandaren | Neutral |
| 13 | Dracthyr | Neutral |
| 14 | Earthen | Neutral |

- **Affects:** `UnitRace("player")`, `UnitFactionGroup("player")`
- **Example:**
```lua
A_Admin.SetPlayerRace(8)    -- Blood Elf
local race, raceFile = UnitRace("player")
local faction, factionFile = UnitFactionGroup("player")
-- race="Blood Elf", faction="Horde"
```

---

### A_Admin.SetPlayerLevel(level)

Sets the player's character level.

- **level** `number` -- Character level (typically 1-80)
- **Affects:** `UnitLevel("player")`, `UnitEffectiveLevel("player")`
- **Example:**
```lua
A_Admin.SetPlayerLevel(60)
print(UnitLevel("player"))  -- 60
```

---

### A_Admin.SetPlayerSex(sex)

Sets the player's displayed sex for localization purposes.

- **sex** `number` -- Sex identifier: `1` = unknown, `2` = male, `3` = female
- **Affects:** `UnitSex("player")`
- **Example:**
```lua
A_Admin.SetPlayerSex(3)     -- female
print(UnitSex("player"))    -- 3
```

---

## Combat State

### A_Admin.SetInCombat(inCombat)

Toggles the player's combat state.

- **inCombat** `boolean` -- `true` to enter combat, `false` to leave
- **Affects:** `InCombatLockdown()`, `UnitAffectingCombat("player")`
- **Fires:** `PLAYER_REGEN_DISABLED` when entering combat, `PLAYER_REGEN_ENABLED` when leaving
- **Example:**
```lua
A_Admin.SetInCombat(true)
print(InCombatLockdown())   -- true
A_Admin.SetInCombat(false)
print(InCombatLockdown())   -- false
```

---

### A_Admin.SetCasting(spellId, spellName, iconPath, duration)

Starts a simulated cast bar on the player.

- **spellId** `number` -- Spell ID for the cast
- **spellName** `string` -- Display name shown in the cast bar
- **iconPath** `string` -- Texture path for the cast bar icon
- **duration** `number` -- Cast time in seconds
- **Affects:** `UnitCastingInfo("player")`
- **Fires:** `UNIT_SPELLCAST_START`
- **Note:** The cast does not auto-complete. Call `StopCasting()` to cancel, or overwrite with another `SetCasting`.
- **Example:**
```lua
A_Admin.SetCasting(19750, "Flash of Light", "Interface\\Icons\\Spell_Holy_FlashHeal", 1.5)
local name, text, texture, startTime, endTime = UnitCastingInfo("player")
-- name="Flash of Light"
```

---

### A_Admin.StopCasting()

Cancels the current simulated cast.

- **Affects:** `UnitCastingInfo("player")` returns `nil`
- **Fires:** `UNIT_SPELLCAST_STOP`
- **Example:**
```lua
A_Admin.SetCasting(19750, "Flash of Light", "", 1.5)
A_Admin.StopCasting()
print(UnitCastingInfo("player"))    -- nil
```

---

### A_Admin.SetGCD(duration)

Triggers the global cooldown.

- **duration** `number` -- GCD duration in seconds (typically 1.5)
- **Affects:** `GetSpellCooldown(61304)` -- spell ID 61304 is the GCD sentinel
- **Example:**
```lua
A_Admin.SetGCD(1.5)
local start, dur, enabled = GetSpellCooldown(61304)
-- start=current_time, dur=1.5, enabled=1
```

---

### A_Admin.SetSpellCooldown(spellId, duration)

Sets a cooldown on a specific spell.

- **spellId** `number` -- Spell ID to put on cooldown
- **duration** `number` -- Cooldown duration in seconds
- **Affects:** `GetSpellCooldown(spellId)`
- **Example:**
```lua
A_Admin.SetSpellCooldown(31935, 15.0)   -- Avenger's Shield on 15s CD
local start, dur, enabled = GetSpellCooldown(31935)
-- dur=15.0
```

---

## Health & Power

### A_Admin.SetPlayerHealth(current, max)

Sets the player's current and maximum health.

- **current** `number` -- Current health value
- **max** `number` -- Maximum health value
- **Affects:** `UnitHealth("player")`, `UnitHealthMax("player")`, `UnitIsDead("player")`
- **Fires:** `UNIT_HEALTH`
- **Example:**
```lua
A_Admin.SetPlayerHealth(50000, 100000)
print(UnitHealth("player"), UnitHealthMax("player"))    -- 50000, 100000

-- Simulate player death
A_Admin.SetPlayerHealth(0, 100000)
print(UnitIsDead("player"))     -- true
```

---

### A_Admin.SetPlayerPower(current, max [, powerType])

Sets the player's current and maximum power/resource.

- **current** `number` -- Current power value
- **max** `number` -- Maximum power value
- **powerType** `number` (optional) -- Power type index (defaults to the class primary resource):

| Value | Power Type |
|-------|------------|
| 0 | Mana |
| 1 | Rage |
| 2 | Focus |
| 3 | Energy |
| 4 | Combo Points |
| 6 | Runic Power |

- **Affects:** `UnitPower("player")`, `UnitPowerMax("player")`
- **Fires:** `UNIT_POWER_UPDATE`
- **Example:**
```lua
A_Admin.SetPlayerPower(50000, 100000, 0)    -- 50% mana
A_Admin.SetPlayerPower(0, 100, 1)           -- 0 rage
```

---

### A_Admin.SetTargetHealth(current, max)

Sets the current target's health values.

- **current** `number` -- Current health value
- **max** `number` -- Maximum health value
- **Affects:** `UnitHealth("target")`, `UnitHealthMax("target")`
- **Fires:** `UNIT_HEALTH`
- **Note:** Requires an active target. Has no effect if no target is set.
- **Example:**
```lua
A_Admin.SetTarget("Hogger", 11, 1, true)
A_Admin.SetTargetHealth(22000, 45000)   -- target at ~50% HP
```

---

## Targeting

### A_Admin.SetTarget(name, level, classIndex, isEnemy)

Creates a custom target unit.

- **name** `string` -- Display name for the target
- **level** `number` -- Unit level
- **classIndex** `number` -- 1-based class index (see `SetPlayerClass`)
- **isEnemy** `boolean` -- `true` for hostile NPC/player, `false` for friendly
- **Affects:** `UnitExists("target")`, `UnitName("target")`, `UnitLevel("target")`, `UnitClass("target")`, `UnitIsEnemy("player", "target")`
- **Fires:** `PLAYER_TARGET_CHANGED`
- **Example:**
```lua
A_Admin.SetTarget("Hogger", 11, 1, true)    -- hostile Warrior-class mob
A_Admin.SetTarget("Jaina", 70, 8, false)    -- friendly Mage
```

---

### A_Admin.ClearTarget()

Removes the current target.

- **Affects:** `UnitExists("target")` returns `false`
- **Fires:** `PLAYER_TARGET_CHANGED`
- **Example:**
```lua
A_Admin.ClearTarget()
print(UnitExists("target"))     -- false
```

---

### A_Admin.SetFocus(name, level, classIndex, isEnemy)

Creates a custom focus target unit. Same parameters as `SetTarget`.

- **name** `string` -- Display name
- **level** `number` -- Unit level
- **classIndex** `number` -- 1-based class index
- **isEnemy** `boolean` -- Hostile or friendly
- **Affects:** `UnitExists("focus")`, `UnitName("focus")`, `UnitLevel("focus")`
- **Fires:** `PLAYER_FOCUS_CHANGED`
- **Example:**
```lua
A_Admin.SetFocus("Thrall", 80, 7, false)    -- friendly Shaman focus
```

---

### A_Admin.ClearFocus()

Removes the current focus target.

- **Affects:** `UnitExists("focus")` returns `false`
- **Fires:** `PLAYER_FOCUS_CHANGED`
- **Example:**
```lua
A_Admin.ClearFocus()
print(UnitExists("focus"))      -- false
```

---

## Party

### A_Admin.SetPartySize(n)

Sets the number of active party members, resizing the party array.

- **n** `number` -- Party size from `0` (solo) to `4`
- **Affects:** `GetNumGroupMembers()`, `UnitExists("party1")` through `UnitExists("party4")`
- **Fires:** `GROUP_ROSTER_UPDATE`
- **Note:** When growing the party, new members are initialized with default values. When shrinking, excess members are removed.
- **Example:**
```lua
A_Admin.SetPartySize(0)     -- go solo
A_Admin.SetPartySize(4)     -- full 4-member party
print(GetNumGroupMembers()) -- 4
```

---

### A_Admin.SetPartyMember(index, name, classIndex, level)

Configures a specific party member's identity.

- **index** `number` -- 1-based party index (1-4)
- **name** `string` -- Character name
- **classIndex** `number` -- 1-based class index (see `SetPlayerClass`)
- **level** `number` -- Character level
- **Affects:** `UnitName("party1")`, `UnitClass("party1")`, `UnitLevel("party1")`, etc.
- **Fires:** `GROUP_ROSTER_UPDATE`
- **Example:**
```lua
A_Admin.SetPartySize(2)
A_Admin.SetPartyMember(1, "Thrynn", 2, 80)      -- Paladin
A_Admin.SetPartyMember(2, "Kazzara", 1, 80)     -- Warrior
print(UnitName("party1"))   -- "Thrynn"
```

---

### A_Admin.SetPartyMemberHealth(index, current, max)

Sets a party member's health values.

- **index** `number` -- 1-based party index (1-4)
- **current** `number` -- Current health
- **max** `number` -- Maximum health
- **Affects:** `UnitHealth("partyN")`, `UnitHealthMax("partyN")`
- **Fires:** `UNIT_HEALTH`
- **Example:**
```lua
A_Admin.SetPartyMemberHealth(1, 60000, 120000)  -- party1 at 50% HP
```

---

### A_Admin.KillPartyMember(index)

Sets a party member's health to zero, marking them as dead.

- **index** `number` -- 1-based party index (1-4)
- **Affects:** `UnitHealth("partyN")` returns `0`, `UnitIsDead("partyN")` returns `true`
- **Fires:** `UNIT_HEALTH`
- **Example:**
```lua
A_Admin.KillPartyMember(2)
print(UnitIsDead("party2"))     -- true
```

---

### A_Admin.ResPartyMember(index)

Restores a dead party member to full health.

- **index** `number` -- 1-based party index (1-4)
- **Affects:** `UnitHealth("partyN")` returns full HP, `UnitIsDead("partyN")` returns `false`
- **Fires:** `UNIT_HEALTH`
- **Example:**
```lua
A_Admin.KillPartyMember(2)
A_Admin.ResPartyMember(2)
print(UnitIsDead("party2"))     -- false
```

---

### A_Admin.SetRotDamage(level)

Sets the automatic rot damage intensity applied to party members each tick.

- **level** `number` -- Intensity level:

| Level | Label | Damage Per Tick |
|-------|-------|-----------------|
| 0 | Off | None |
| 1 | Light | 1% of max HP |
| 2 | Moderate | 3% of max HP |
| 3 | Heavy | 5% of max HP |
| 4 | Brutal | 10% of max HP |

- **Note:** Matches the `ROT_DAMAGE_LEVELS` constant in `src/lua_api/game_data.rs`.
- **Example:**
```lua
A_Admin.SetRotDamage(2)     -- moderate damage for healing practice
A_Admin.SetRotDamage(0)     -- disable rot
```

---

## Movement

### A_Admin.SetMoving(moving)

Toggles the player movement state.

- **moving** `boolean`
- **Affects:** `IsPlayerMoving()`
- **Example:** `A_Admin.SetMoving(true)`

---

### A_Admin.SetMounted(mounted)

Toggles the player's mounted state.

- **mounted** `boolean`
- **Affects:** `IsMounted()`
- **Example:** `A_Admin.SetMounted(true)`

---

### A_Admin.SetFlying(flying)

Toggles the player's flying state.

- **flying** `boolean`
- **Affects:** `IsFlying()`
- **Example:** `A_Admin.SetFlying(true)`

---

### A_Admin.SetFalling(falling)

Toggles the player's falling state.

- **falling** `boolean`
- **Affects:** `IsFalling()`
- **Example:** `A_Admin.SetFalling(true)`

---

### A_Admin.SetSwimming(swimming)

Toggles the player's swimming state.

- **swimming** `boolean`
- **Affects:** `IsSwimming()`
- **Example:** `A_Admin.SetSwimming(true)`

---

## Specialization & Talents

### A_Admin.SetSpec(specIndex)

Sets the player's active specialization.

- **specIndex** `number` -- 1-based spec index within the player's class
- **Affects:** `GetSpecialization()`, `GetSpecializationInfo()`
- **Fires:** `PLAYER_SPECIALIZATION_CHANGED`
- **Example:**
```lua
-- For a Paladin: 1=Holy, 2=Protection, 3=Retribution
A_Admin.SetSpec(3)
print(GetSpecialization())  -- 3
local id, name = GetSpecializationInfo(3)
-- name="Retribution"
```

---

### A_Admin.SetTalentRank(nodeId, rank)

Sets the purchased rank on a talent tree node.

- **nodeId** `number` -- Trait node ID from the talent database
- **rank** `number` -- Number of ranks purchased (0 = unlearned)
- **Affects:** `C_Traits.GetNodeInfo(configID, nodeId).currentRank`
- **Fires:** `TRAIT_CONFIG_UPDATED`
- **Example:**
```lua
A_Admin.SetTalentRank(87800, 1)     -- learn a specific talent node
A_Admin.SetTalentRank(87800, 0)     -- unlearn it
```

---

### A_Admin.SetTalentSelection(nodeId, entryId)

Selects which entry is active for a choice node in the talent tree.

- **nodeId** `number` -- Trait node ID
- **entryId** `number` -- Entry ID of the selected choice
- **Affects:** `C_Traits.GetNodeInfo(configID, nodeId).activeEntry`
- **Fires:** `TRAIT_CONFIG_UPDATED`
- **Example:**
```lua
-- Select the right-side talent on a choice node
A_Admin.SetTalentSelection(88100, 88101)
```

---

### A_Admin.ResetTalents()

Clears all talent node ranks and selections, returning the tree to an unspent state.

- **Affects:** All `C_Traits.GetNodeInfo()` calls return rank 0 and no selection
- **Fires:** `TRAIT_CONFIG_UPDATED`
- **Example:**
```lua
A_Admin.ResetTalents()
```

---

## Buffs & Auras

### A_Admin.AddBuff(spellId, name, icon, duration, stacks)

Adds a buff to the player's aura list.

- **spellId** `number` -- Spell ID for the aura
- **name** `string` -- Display name
- **icon** `number` -- File data ID for the aura icon texture
- **duration** `number` -- Duration in seconds (0 = permanent/no expiry)
- **stacks** `number` -- Stack count (`applications` in aura data)
- **Affects:** `UnitBuff("player", index)`, `UnitAura("player", index)`, `GetPlayerAuraBySpellID(spellId)`
- **Fires:** `UNIT_AURA`
- **Example:**
```lua
-- Add Power Word: Fortitude (permanent raid buff)
A_Admin.AddBuff(21562, "Power Word: Fortitude", 135987, 0, 0)

-- Add a 30-second 5-stack buff
A_Admin.AddBuff(12345, "Test Buff", 136001, 30.0, 5)

local name, icon, count = UnitBuff("player", 1)
```

---

### A_Admin.RemoveBuff(spellId)

Removes the buff with the given spell ID from the player's aura list.

- **spellId** `number` -- Spell ID of the buff to remove
- **Affects:** `UnitBuff("player")` and `GetPlayerAuraBySpellID(spellId)` no longer return this aura
- **Fires:** `UNIT_AURA`
- **Example:**
```lua
A_Admin.RemoveBuff(21562)   -- remove Power Word: Fortitude
```

---

### A_Admin.ClearBuffs()

Removes all buffs from the player's aura list.

- **Affects:** `UnitBuff("player", 1)` returns `nil`
- **Fires:** `UNIT_AURA`
- **Example:**
```lua
A_Admin.ClearBuffs()
print(UnitBuff("player", 1))    -- nil
```

---

## Zone & Instance

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

---

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

---

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

---

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

---

## Economy & Items

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

---

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

---

## Collections

### A_Admin.AddTransmog(appearanceId)

Marks a transmog appearance as collected by the player.

- **appearanceId** `number` -- Appearance ID from the transmog database
- **Affects:** `C_Transmog.GetAppearanceSourceInfo()` -- `isCollected` field returns `true`
- **Example:**
```lua
A_Admin.AddTransmog(12345)
```

---

### A_Admin.RemoveTransmog(appearanceId)

Marks a transmog appearance as uncollected.

- **appearanceId** `number` -- Appearance ID
- **Affects:** `C_Transmog.GetAppearanceSourceInfo()` -- `isCollected` field returns `false`
- **Example:**
```lua
A_Admin.RemoveTransmog(12345)
```

---

### A_Admin.SetMountCollected(mountId, collected)

Toggles whether a mount is collected.

- **mountId** `number` -- Mount ID from `C_MountJournal`
- **collected** `boolean` -- `true` = collected, `false` = not collected
- **Affects:** `C_MountJournal.GetMountInfoByID(mountId)` -- `isCollected` field
- **Example:**
```lua
A_Admin.SetMountCollected(232, true)    -- Invincible
```

---

### A_Admin.SetPetCollected(petId, collected)

Toggles whether a battle pet is collected.

- **petId** `number` -- Pet species ID
- **collected** `boolean`
- **Affects:** `C_PetJournal.GetNumCollectedInfo(petId)`
- **Example:**
```lua
A_Admin.SetPetCollected(39, true)       -- Mechanical Squirrel
```

---

### A_Admin.SetToyCollected(toyId, collected)

Toggles whether a toy is in the toy box.

- **toyId** `number` -- Item ID for the toy
- **collected** `boolean`
- **Affects:** `PlayerHasToy(toyId)`
- **Example:**
```lua
A_Admin.SetToyCollected(37710, true)    -- Paper Flying Machine Kit
```

---

### A_Admin.SetAchievementEarned(achieveId, earned)

Toggles whether an achievement has been earned.

- **achieveId** `number` -- Achievement ID
- **earned** `boolean`
- **Affects:** `GetAchievementInfo(achieveId)` -- `completed` field
- **Example:**
```lua
A_Admin.SetAchievementEarned(2144, true)    -- "What a Long, Strange Trip It's Been"
```

---

## PvP

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

---

### A_Admin.SetHonorLevel(level)

Sets the player's honor level.

- **level** `number` -- Honor level (1-500)
- **Affects:** `UnitHonorLevel("player")`
- **Example:**
```lua
A_Admin.SetHonorLevel(50)
print(UnitHonorLevel("player"))     -- 50
```

---

## Guild

### A_Admin.SetGuildInfo(name, rank, numMembers)

Configures the player's guild membership.

- **name** `string` -- Guild name
- **rank** `string` -- Player's rank title within the guild
- **numMembers** `number` -- Total guild member count
- **Affects:** `GetGuildInfo("player")` -- returns name, rank, realm; `GetNumGuildMembers()`
- **Example:**
```lua
A_Admin.SetGuildInfo("Knights of the Ebon Blade", "Champion", 42)
local name, rank = GetGuildInfo("player")
-- name="Knights of the Ebon Blade", rank="Champion"
```

---

### A_Admin.ClearGuild()

Removes the player from any guild.

- **Affects:** `GetGuildInfo("player")` returns `nil`
- **Example:**
```lua
A_Admin.ClearGuild()
print(GetGuildInfo("player"))   -- nil
```

---

## Events

### A_Admin.FireEvent(event, ...)

Fires a WoW game event directly to all registered listeners.

- **event** `string` -- Event name (e.g., `"ZONE_CHANGED_NEW_AREA"`)
- **...** -- Optional event arguments passed to `OnEvent` handlers
- **Example:**
```lua
-- Fire a simple event
A_Admin.FireEvent("ZONE_CHANGED_NEW_AREA")

-- Fire an event with arguments
A_Admin.FireEvent("ADDON_LOADED", "MyAddon")
A_Admin.FireEvent("CHAT_MSG_SAY", "Hello world", "Arthas", "", "", "Arthas")
A_Admin.FireEvent("UNIT_HEALTH", "player")
```

**Note:** `A_Admin.FireEvent` is a namespaced alias for the internal `FireEvent` global, making it clear in test scripts that this is a simulator-only call with no real WoW equivalent.

---

## Usage Examples

### Setting Up a Raid Healing Environment

Simulate a 4-member party taking rot damage for testing healing addon displays:

```lua
-- Configure the party
A_Admin.SetPartySize(4)
A_Admin.SetPartyMember(1, "Thrynn",   2, 80)    -- Paladin tank
A_Admin.SetPartyMember(2, "Kazzara", 1, 80)    -- Warrior DPS
A_Admin.SetPartyMember(3, "Sylvanas", 3, 80)   -- Hunter DPS
A_Admin.SetPartyMember(4, "Jaina",    8, 80)   -- Mage DPS

-- Set varying health states to test deficit display
A_Admin.SetPartyMemberHealth(1, 120000, 120000)  -- full HP
A_Admin.SetPartyMemberHealth(2, 80000,  180000)  -- 44% HP
A_Admin.SetPartyMemberHealth(3, 30000,  100000)  -- critical HP
A_Admin.KillPartyMember(4)                       -- dead

-- Enable light rot for continuous incoming damage
A_Admin.SetRotDamage(1)

-- Simulate a raid instance
A_Admin.SetInstanceInfo("Amirdrassil", "raid", 17, 25)
A_Admin.SetInInstance(true)

A_Admin.FireEvent("GROUP_ROSTER_UPDATE")
```

---

### Simulating Combat for UI Testing

Test combat-sensitive elements like action bars, cast bars, and cooldowns:

```lua
-- Enter combat with a hostile target
A_Admin.SetInCombat(true)
A_Admin.SetTarget("Ragnaros", 85, 1, true)

-- Start a cast bar
A_Admin.SetCasting(19750, "Flash of Light", "Interface\\Icons\\Spell_Holy_FlashHeal", 1.5)

-- Apply GCD and a spell cooldown
A_Admin.SetGCD(1.5)
A_Admin.SetSpellCooldown(31935, 15.0)

-- Swap buffs for combat state
A_Admin.ClearBuffs()
A_Admin.AddBuff(465,  "Devotion Aura", 135893, 0,    0)
A_Admin.AddBuff(6673, "Battle Shout",  132333, 3600, 0)

-- Take some damage
A_Admin.SetPlayerHealth(75000, 100000)
```

---

### Testing Transmog Collection UI

Configure a specific character state for testing the Appearances frame:

```lua
-- Set class and spec for Retribution Paladin
A_Admin.SetPlayerClass(2)   -- Paladin
A_Admin.SetPlayerRace(9)    -- Draenei (Alliance)
A_Admin.SetSpec(3)          -- Retribution

-- Set a realistic item level
A_Admin.SetItemLevel(480)

-- Mark specific appearances as collected
for _, id in ipairs({ 12345, 12346, 12347, 99001, 99002 }) do
    A_Admin.AddTransmog(id)
end

-- Put the player in a city (out of instance)
A_Admin.SetZone("Orgrimmar", 1637)
A_Admin.SetInInstance(false)

A_Admin.FireEvent("TRANSMOG_COLLECTION_UPDATED")
```
