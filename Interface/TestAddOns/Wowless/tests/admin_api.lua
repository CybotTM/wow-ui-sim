-- Tests for A_Admin namespace: verify admin setters affect WoW API getters.

-- ============================================================================
-- Identity
-- ============================================================================

test("SetPlayerName changes UnitName", function()
    A_Admin.SetPlayerName("Arthas")
    local name = UnitName("player")
    assertEquals("Arthas", name)
end)

test("SetPlayerClass changes UnitClass", function()
    A_Admin.SetPlayerClass(2) -- Paladin
    local className, classFile, classIndex = UnitClass("player")
    assertEquals(2, classIndex)
    assertEquals("PALADIN", classFile)
    assertEquals("Paladin", className)
end)

test("SetPlayerRace changes UnitRace", function()
    A_Admin.SetPlayerRace(1) -- Human
    local raceName, raceFile = UnitRace("player")
    assertEquals("Human", raceName)
    assertEquals("Human", raceFile)
end)

test("SetPlayerLevel changes UnitLevel", function()
    A_Admin.SetPlayerLevel(70)
    assertEquals(70, UnitLevel("player"))
end)

test("SetPlayerSex changes UnitSex", function()
    A_Admin.SetPlayerSex(3) -- Female
    assertEquals(3, UnitSex("player"))
end)

-- ============================================================================
-- Combat
-- ============================================================================

test("SetInCombat changes InCombatLockdown and UnitAffectingCombat", function()
    A_Admin.SetInCombat(true)
    assertTrue(InCombatLockdown())
    assertTrue(UnitAffectingCombat("player"))

    A_Admin.SetInCombat(false)
    assertFalse(InCombatLockdown())
    assertFalse(UnitAffectingCombat("player"))
end)

-- ============================================================================
-- Health & Power
-- ============================================================================

test("SetPlayerHealth changes UnitHealth and UnitHealthMax", function()
    A_Admin.SetPlayerHealth(5000, 10000)
    assertEquals(5000, UnitHealth("player"))
    assertEquals(10000, UnitHealthMax("player"))
end)

test("SetPlayerPower changes UnitPower and UnitPowerMax", function()
    A_Admin.SetPlayerPower(3000, 8000, 0) -- Mana
    assertEquals(3000, UnitPower("player", 0))
    assertEquals(8000, UnitPowerMax("player", 0))
end)

-- ============================================================================
-- Targeting
-- ============================================================================

test("SetTarget makes UnitExists return true for target", function()
    A_Admin.ClearTarget()
    assertFalse(UnitExists("target"))

    A_Admin.SetTarget("Onyxia", 60, 1, true)
    assertTrue(UnitExists("target"))
    assertEquals("Onyxia", UnitName("target"))
    assertEquals(60, UnitLevel("target"))
end)

test("ClearTarget makes UnitExists return false for target", function()
    A_Admin.SetTarget("Ragnaros", 63, 1, true)
    assertTrue(UnitExists("target"))

    A_Admin.ClearTarget()
    assertFalse(UnitExists("target"))
end)

test("SetFocus makes UnitExists return true for focus", function()
    A_Admin.ClearFocus()
    assertFalse(UnitExists("focus"))

    A_Admin.SetFocus("Thrall", 80, 7, false)
    assertTrue(UnitExists("focus"))
end)

-- ============================================================================
-- Party
-- ============================================================================

test("SetPartySize creates party members", function()
    A_Admin.SetPartySize(3)
    assertEquals(3, GetNumGroupMembers())
    assertTrue(IsInGroup())
    assertTrue(UnitExists("party1"))
    assertTrue(UnitExists("party2"))
    assertTrue(UnitExists("party3"))
    assertFalse(UnitExists("party4"))
end)

test("SetPartyMember updates member info", function()
    A_Admin.SetPartySize(1)
    A_Admin.SetPartyMember(1, "Jaina", 8, 80) -- Mage, level 80
    assertEquals("Jaina", UnitName("party1"))
    assertEquals(80, UnitLevel("party1"))
    local _, classFile = UnitClass("party1")
    assertEquals("MAGE", classFile)
end)

test("SetPartyMemberHealth updates member health", function()
    A_Admin.SetPartySize(1)
    A_Admin.SetPartyMemberHealth(1, 4000, 12000)
    assertEquals(4000, UnitHealth("party1"))
    assertEquals(12000, UnitHealthMax("party1"))
end)

test("KillPartyMember makes UnitIsDead true", function()
    A_Admin.SetPartySize(1)
    A_Admin.KillPartyMember(1)
    assertTrue(UnitIsDead("party1"))

    A_Admin.ResPartyMember(1)
    assertFalse(UnitIsDead("party1"))
end)

-- ============================================================================
-- Movement
-- ============================================================================

test("SetMounted changes IsMounted", function()
    A_Admin.SetMounted(true)
    assertTrue(IsMounted())
    A_Admin.SetMounted(false)
    assertFalse(IsMounted())
end)

test("SetFlying changes IsFlying", function()
    A_Admin.SetFlying(true)
    assertTrue(IsFlying())
    A_Admin.SetFlying(false)
    assertFalse(IsFlying())
end)

test("SetFalling changes IsFalling", function()
    A_Admin.SetFalling(true)
    assertTrue(IsFalling())
    A_Admin.SetFalling(false)
    assertFalse(IsFalling())
end)

test("SetSwimming changes IsSwimming", function()
    A_Admin.SetSwimming(true)
    assertTrue(IsSwimming())
    A_Admin.SetSwimming(false)
    assertFalse(IsSwimming())
end)

-- ============================================================================
-- Spec & Talents
-- ============================================================================

test("SetSpec changes GetSpecialization", function()
    A_Admin.SetSpec(2)
    assertEquals(2, GetSpecialization())
end)

-- ============================================================================
-- Zone & Instance
-- ============================================================================

test("SetZone changes GetZoneText", function()
    A_Admin.SetZone("Orgrimmar", 1637)
    assertEquals("Orgrimmar", GetZoneText())
end)

test("SetSubZone changes GetSubZoneText", function()
    A_Admin.SetSubZone("Valley of Strength")
    assertEquals("Valley of Strength", GetSubZoneText())
end)

test("SetInInstance changes IsInInstance", function()
    A_Admin.SetInInstance(true)
    local inInstance = IsInInstance()
    assertTrue(inInstance)

    A_Admin.SetInInstance(false)
    inInstance = IsInInstance()
    assertFalse(inInstance)
end)

-- ============================================================================
-- Economy
-- ============================================================================

test("SetMoney changes GetMoney", function()
    A_Admin.SetMoney(123456)
    assertEquals(123456, GetMoney())
end)

-- ============================================================================
-- Collections
-- ============================================================================

test("AddTransmog and RemoveTransmog affect C_TransmogCollection", function()
    A_Admin.AddTransmog(12345)
    assertTrue(C_TransmogCollection.PlayerHasTransmog(12345))

    A_Admin.RemoveTransmog(12345)
    assertFalse(C_TransmogCollection.PlayerHasTransmog(12345))
end)

test("SetMountCollected affects C_MountJournal", function()
    A_Admin.SetMountCollected(100, true)
    -- GetMountInfoByID returns: name, spellID, icon, isActive, isUsable,
    -- sourceType, isFavorite, isFactionSpecific, faction, shouldHideOnChar,
    -- isCollected, mountID, isForDragonriding
    local info = {C_MountJournal.GetMountInfoByID(100)}
    assertTrue(info[11]) -- isCollected

    A_Admin.SetMountCollected(100, false)
    info = {C_MountJournal.GetMountInfoByID(100)}
    assertFalse(info[11])
end)

test("SetToyCollected affects C_ToyBox", function()
    A_Admin.SetToyCollected(200, true)
    assertTrue(C_ToyBox.PlayerHasToy(200))

    A_Admin.SetToyCollected(200, false)
    assertFalse(C_ToyBox.PlayerHasToy(200))
end)

test("SetAchievementEarned affects GetAchievementInfo", function()
    A_Admin.SetAchievementEarned(500, true)
    -- GetAchievementInfo returns multiple values, 4th is completed
    local _, _, _, completed = GetAchievementInfo(500)
    assertTrue(completed)

    A_Admin.SetAchievementEarned(500, false)
    _, _, _, completed = GetAchievementInfo(500)
    assertFalse(completed)
end)

-- ============================================================================
-- PvP & Guild
-- ============================================================================

test("SetPvPEnabled changes UnitIsPVP", function()
    A_Admin.SetPvPEnabled(true)
    assertTrue(UnitIsPVP("player"))

    A_Admin.SetPvPEnabled(false)
    assertFalse(UnitIsPVP("player"))
end)

test("SetHonorLevel changes GetHonorLevel", function()
    A_Admin.SetHonorLevel(25)
    assertEquals(25, GetHonorLevel())
end)

test("SetGuildInfo changes GetGuildInfo and IsInGuild", function()
    A_Admin.SetGuildInfo("Horde Elite", "Guild Master", 50)
    assertTrue(IsInGuild())
    local guildName, guildRank, rankIndex = GetGuildInfo("player")
    assertEquals("Horde Elite", guildName)
    assertEquals("Guild Master", guildRank)
end)

test("ClearGuild changes IsInGuild", function()
    A_Admin.ClearGuild()
    assertFalse(IsInGuild())
end)

-- ============================================================================
-- Buffs
-- ============================================================================

test("AddBuff and RemoveBuff affect UnitBuff", function()
    A_Admin.ClearBuffs()
    A_Admin.AddBuff(1459, "Arcane Intellect", "136930", 3600, 1)

    local name = UnitBuff("player", 1)
    assertEquals("Arcane Intellect", name)

    A_Admin.RemoveBuff(1459)
    name = UnitBuff("player", 1)
    assertNil(name)
end)

test("ClearBuffs removes all buffs", function()
    A_Admin.AddBuff(1459, "Arcane Intellect", "136930", 3600, 1)
    A_Admin.AddBuff(21562, "Power Word: Fortitude", "135987", 3600, 1)
    A_Admin.ClearBuffs()

    local name = UnitBuff("player", 1)
    assertNil(name)
end)

-- ============================================================================
-- Events
-- ============================================================================

test("FireEvent dispatches to registered handlers", function()
    local received = false
    local receivedArg = nil
    local f = CreateFrame("Frame")
    f:RegisterEvent("CUSTOM_TEST_EVENT")
    f:SetScript("OnEvent", function(self, event, arg1)
        if event == "CUSTOM_TEST_EVENT" then
            received = true
            receivedArg = arg1
        end
    end)

    A_Admin.FireEvent("CUSTOM_TEST_EVENT", "hello")
    -- Events are queued; need an OnUpdate tick to dispatch
end)

async_test("FireEvent dispatches after tick", function(done)
    local f = CreateFrame("Frame")
    f:RegisterEvent("ADMIN_TEST_ASYNC")
    local receivedArg = nil
    f:SetScript("OnEvent", function(self, event, arg1)
        if event == "ADMIN_TEST_ASYNC" then
            receivedArg = arg1
        end
    end)

    A_Admin.FireEvent("ADMIN_TEST_ASYNC", "world")

    C_Timer.After(0, function()
        done(function()
            assertEquals("world", receivedArg)
        end)
    end)
end)
