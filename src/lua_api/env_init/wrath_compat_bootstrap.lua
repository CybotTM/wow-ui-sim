-- Wrath-era global aliases and stubs.
-- Loaded only when the client-wrath feature is active.
-- All definitions are guarded by nil-checks so later Blizzard Lua can override.

-- ─── Lua 5.0-era string/math aliases ────────────────────────────────────────
-- These lived as top-level globals in Wrath but retail removed them.

if gsub == nil and string ~= nil then gsub = string.gsub end
if strfind == nil and string ~= nil then strfind = string.find end
if strlower == nil and string ~= nil then strlower = string.lower end
if strupper == nil and string ~= nil then strupper = string.upper end
if strlen == nil and string ~= nil then strlen = string.len end
-- strsub and strconcat are registered from Rust (missing_surface.rs)
if strchar == nil and string ~= nil then strchar = string.char end
if strbyte == nil and string ~= nil then strbyte = string.byte end
if strrep == nil and string ~= nil then strrep = string.rep end
if strrev == nil and string ~= nil then strrev = string.reverse end
if mod == nil and math ~= nil then mod = math.fmod end

-- strjoin and strsplit are registered from Rust (utility_system_spell).
-- strconcat is registered from Rust (missing_surface.rs).

-- ─── Wrath-only API stubs ────────────────────────────────────────────────────

if BNGetMaxPlayersInConversation == nil then
  function BNGetMaxPlayersInConversation()
    return 10
  end
end

if CreateWorldMapArrowFrame == nil then
  function CreateWorldMapArrowFrame()
    return nil
  end
end

if DropCursorMoney == nil then
  function DropCursorMoney()
  end
end

if FCF_DockUpdate == nil then
  function FCF_DockUpdate()
  end
end

if FillLocalizedClassList == nil then
  function FillLocalizedClassList(classTable)
    if type(classTable) ~= "table" then
      return
    end
    -- Populate with minimal wrath class stubs so iterators don't crash.
    local classes = {
      "WARRIOR", "PALADIN", "HUNTER", "ROGUE", "PRIEST",
      "SHAMAN", "MAGE", "WARLOCK", "DRUID", "DEATHKNIGHT",
    }
    for _, class in ipairs(classes) do
      if classTable[class] == nil then
        classTable[class] = class
      end
    end
  end
end

if GetActionBarPage == nil then
  function GetActionBarPage()
    return 1
  end
end

if GetArenaTeam == nil then
  function GetArenaTeam()
    return nil
  end
end

if GetAvailableRoles == nil then
  function GetAvailableRoles()
    return false, false, false
  end
end

if GetCompanionInfo == nil then
  function GetCompanionInfo()
    return nil
  end
end

if GetCurrentMultisampleFormat == nil then
  function GetCurrentMultisampleFormat()
    return 0
  end
end

if GetCurrentResolution == nil then
  function GetCurrentResolution()
    return 1
  end
end

if GetExistingLocales == nil then
  function GetExistingLocales()
    return "enUS"
  end
end

if GetGMTicket == nil then
  function GetGMTicket()
    return nil
  end
end

if GetMasterLootCandidate == nil then
  function GetMasterLootCandidate()
    return nil
  end
end

if GetModifiedClick == nil then
  function GetModifiedClick()
    return nil
  end
end

if GetNumBattlegroundTypes == nil then
  function GetNumBattlegroundTypes()
    return 0
  end
end

if GetNumTrackingTypes == nil then
  function GetNumTrackingTypes()
    return 0
  end
end

if GetNumVoiceSessions == nil then
  function GetNumVoiceSessions()
    return 0
  end
end

if GetNumWorldStateUI == nil then
  function GetNumWorldStateUI()
    return 0
  end
end

if GetPVPYesterdayStats == nil then
  function GetPVPYesterdayStats()
    return 0, 0
  end
end

if GetQuestTimers == nil then
  function GetQuestTimers()
    return nil
  end
end

if GetRefreshRates == nil then
  function GetRefreshRates()
    return 60
  end
end

if GetRuneType == nil then
  function GetRuneType()
    return 1
  end
end

if GetScreenResolutions == nil then
  function GetScreenResolutions()
    return "1024x768"
  end
end

if GetSelectedDisplayChannel == nil then
  function GetSelectedDisplayChannel()
    return nil
  end
end

if GetTextHeight == nil then
  function GetTextHeight()
    return 12
  end
end

if GetTrackingTexture == nil then
  function GetTrackingTexture()
    return nil
  end
end

if GetZonePVPInfo == nil then
  function GetZonePVPInfo()
    return nil
  end
end

if HasPetUI == nil then
  function HasPetUI()
    return false, false
  end
end

if IsListedInLFR == nil then
  function IsListedInLFR()
    return false
  end
end

if IsPartyLeader == nil then
  function IsPartyLeader()
    if type(UnitIsGroupLeader) == "function" then
      return UnitIsGroupLeader("party1")
    end
    return false
  end
end

if IsPetAttackAction == nil then
  function IsPetAttackAction()
    return false
  end
end

if IsPossessBarVisible == nil then
  function IsPossessBarVisible()
    return false
  end
end

if IsStereoVideoAvailable == nil then
  function IsStereoVideoAvailable()
    return false
  end
end

if IsVoiceChatAllowedByServer == nil then
  function IsVoiceChatAllowedByServer()
    return false
  end
end

-- QuestDifficultyColors is a global table (not a function).
if QuestDifficultyColors == nil then
  QuestDifficultyColors = {
    Trivial    = { r = 0.5, g = 0.5, b = 0.5 },
    Easy       = { r = 0.5, g = 1.0, b = 0.5 },
    Standard   = { r = 1.0, g = 1.0, b = 1.0 },
    Difficult  = { r = 1.0, g = 0.5, b = 0.0 },
    Impossible = { r = 1.0, g = 0.1, b = 0.1 },
  }
end

if RegisterForSave == nil then
  function RegisterForSave()
  end
end

if SetGuildRosterSelection == nil then
  function SetGuildRosterSelection()
  end
end

if SetMapToCurrentZone == nil then
  function SetMapToCurrentZone()
  end
end

if SetMaxBytes == nil then
  function SetMaxBytes()
  end
end

if SetPlayerTextureWidth == nil then
  function SetPlayerTextureWidth()
  end
end

if SetSelectedSkill == nil then
  function SetSelectedSkill()
  end
end

if debuginfo == nil then
  function debuginfo()
    return ""
  end
end

-- ─── Additional Wrath-only stubs (batch 3.3b) ────────────────────────────────

if strmatch == nil and string ~= nil then strmatch = string.match end

if Blizzard_CombatLog_RefreshGlobalLinks == nil then
  function Blizzard_CombatLog_RefreshGlobalLinks()
  end
end

if GetActionBarToggles == nil then
  function GetActionBarToggles()
    return false, false, false, false, false, false
  end
end

if GetActiveVoiceChannel == nil then
  function GetActiveVoiceChannel()
    return nil
  end
end

if GetAdjustedSkillPoints == nil then
  function GetAdjustedSkillPoints(level)
    return level
  end
end

if GetCVarMin == nil then
  function GetCVarMin()
    return 0
  end
end

if GetMultiCastBarOffset == nil then
  function GetMultiCastBarOffset()
    return 0
  end
end

if GetMultisampleFormats == nil then
  function GetMultisampleFormats()
    return ""
  end
end

if GetNumBankSlots == nil then
  function GetNumBankSlots()
    return 0, 0
  end
end

if GetPVPLifetimeStats == nil then
  function GetPVPLifetimeStats()
    return 0, 0, 0
  end
end

if GetPVPSessionStats == nil then
  function GetPVPSessionStats()
    return 0, 0
  end
end

if GetVoiceCurrentSessionID == nil then
  function GetVoiceCurrentSessionID()
    return nil
  end
end

if GetVoiceSessionInfo == nil then
  function GetVoiceSessionInfo()
    return nil
  end
end

if InitWorldMapPing == nil then
  function InitWorldMapPing()
  end
end

if IsAutoRepeatAction == nil then
  function IsAutoRepeatAction()
    return false
  end
end

if IsEquippedAction == nil then
  function IsEquippedAction()
    return false
  end
end

if IsRaidOfficer == nil then
  function IsRaidOfficer()
    return false
  end
end

if IsUsableAction == nil then
  function IsUsableAction()
    return true, false
  end
end

if IsVoiceChatEnabled == nil then
  function IsVoiceChatEnabled()
    return false
  end
end

if SelectQuestLogEntry == nil then
  function SelectQuestLogEntry()
  end
end

-- Frame references: return a no-op proxy table so method calls like
-- :Show(), :Hide(), :SetTexture() don't crash.
local function noopFrame()
  local t = {}
  setmetatable(t, { __index = function() return function() end end })
  return t
end

if MiniMapTrackingIcon == nil then
  MiniMapTrackingIcon = noopFrame()
end

if PlayerArrowEffectFrame == nil then
  PlayerArrowEffectFrame = noopFrame()
end
