# Usage Examples

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
