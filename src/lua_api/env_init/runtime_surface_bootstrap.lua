local function __wow_make_color(r, g, b, a)
  local color = {
    r = r or 1,
    g = g or 1,
    b = b or 1,
    a = a or 1,
  }

  function color:GetRGB()
    return self.r, self.g, self.b
  end

  function color:GetRGBA()
    return self.r, self.g, self.b, self.a
  end

  function color:GenerateHexColor()
    return string.format("%02X%02X%02X", math.floor(self.r * 255), math.floor(self.g * 255), math.floor(self.b * 255))
  end

  function color:GenerateHexColorMarkup()
    return "|cFF" .. self:GenerateHexColor()
  end

  function color:WrapTextInColorCode(text)
    return self:GenerateHexColorMarkup() .. tostring(text or "") .. "|r"
  end

  return color
end

if CreateColor == nil then
  function CreateColor(r, g, b, a)
    return __wow_make_color(r, g, b, a)
  end
end

local function __wow_noop()
end

local __wow_clock_start = os.clock and os.clock() or 0

if GetText == nil then
  function GetText(token)
    if type(token) ~= "string" then
      return token
    end
    local value = rawget(_G, token)
    return value ~= nil and value or token
  end
end

if BreakUpLargeNumbers == nil then
  function BreakUpLargeNumbers(value)
    return tostring(value)
  end
end

if StaticPopup_Hide == nil then
  function StaticPopup_Hide(_which, _data)
  end
end

if StaticPopup_Show == nil then
  function StaticPopup_Show(_which, _text_arg1, _text_arg2, _data)
    return nil
  end
end

if ReloadUI == nil then
  function ReloadUI()
  end
end

if GetGameTime == nil then
  function GetGameTime()
    return 12, 0
  end
end

if GetLocale == nil then
  function GetLocale()
    return "enUS"
  end
end

if GetMoney == nil then
  function GetMoney()
    return 0
  end
end

if GetFramerate == nil then
  function GetFramerate()
    return 60
  end
end

if GetCategoryList == nil then
  function GetCategoryList()
    return {}
  end
end

if UI_SPECIAL_FRAMES == nil then
  UI_SPECIAL_FRAMES = {}
end

if UISpecialFrames == nil then
  UISpecialFrames = UI_SPECIAL_FRAMES
end

if GetGuildCategoryList == nil then
  function GetGuildCategoryList()
    return {}
  end
end

if GetStatisticsCategoryList == nil then
  function GetStatisticsCategoryList()
    return {}
  end
end

if GetDefaultScale == nil then
  function GetDefaultScale()
    return 1
  end
end

if GetInventoryItemLink == nil then
  function GetInventoryItemLink(_unit, _slot)
    return nil
  end
end

if GetTutorialsEnabled == nil then
  function GetTutorialsEnabled()
    return false
  end
end

if GetChatWindowInfo == nil then
  function GetChatWindowInfo(id)
    -- Default chat frame color: black with 25% alpha (DEFAULT_CHATFRAME_COLOR / DEFAULT_CHATFRAME_ALPHA)
    -- Only ChatFrame1 (General) and ChatFrame2 (CombatLog) shown by default;
    -- ChatFrame3-10 exist in XML but stay hidden until user creates them.
    local realId = id or 1
    local shown = (realId == 1) or (realId == 2)
    local docked = (realId == 1)
    return "Chat " .. tostring(realId), 12, 0, 0, 0, 0.25, shown, false, docked, false
  end
end

ChatTypeInfo = ChatTypeInfo or {}
ChatTypeInfo.SYSTEM = ChatTypeInfo.SYSTEM or {
  r = 1,
  g = 1,
  b = 0,
  id = 1,
}

local __wow_chat_window_state = __wow_chat_window_state or {}

if SetChatWindowShown == nil then
  function SetChatWindowShown(id, shown)
    local chat = __wow_chat_window_state[id] or {}
    chat.shown = shown == true
    __wow_chat_window_state[id] = chat
  end
end

if GetChatWindowSavedDimensions == nil then
  function GetChatWindowSavedDimensions(id)
    local chat = __wow_chat_window_state[id]
    if not chat then
      return nil, nil
    end
    return chat.width, chat.height
  end
end

if SetChatWindowSavedDimensions == nil then
  function SetChatWindowSavedDimensions(id, width, height)
    local chat = __wow_chat_window_state[id] or {}
    chat.width = width
    chat.height = height
    __wow_chat_window_state[id] = chat
  end
end

if GetChatWindowSavedPosition == nil then
  function GetChatWindowSavedPosition(id)
    local chat = __wow_chat_window_state[id]
    if not chat then
      return nil, nil, nil
    end
    return chat.point, chat.xOffset, chat.yOffset
  end
end

if SetChatWindowSavedPosition == nil then
  function SetChatWindowSavedPosition(id, point, xOffset, yOffset)
    local chat = __wow_chat_window_state[id] or {}
    chat.point = point
    chat.xOffset = xOffset
    chat.yOffset = yOffset
    __wow_chat_window_state[id] = chat
  end
end

if GetTime == nil then
  function GetTime()
    if os.clock == nil then
      return 0
    end
    return os.clock() - __wow_clock_start
  end
end

if GetActionInfo == nil then
  function GetActionInfo()
    return nil, nil, nil
  end
end

-- `GetInventorySlotInfo(slotName)` — canonical WoW slot id + icon fileDataID.
-- Callsites use the numeric return as a TABLE KEY (e.g.
-- `CANCELABLE_ITEMS[GetInventorySlotInfo("MainHandSlot")] = 1`), so a
-- nil return crashes the chunk with "table index is nil". The fileDataID
-- mirrors `PaperDollItemFrame.SlotIconFileID` — DB maps ItemButtonName to
-- a specific icon, not a naive `UI-PaperDoll-Slot-<slotName>` concat.
-- Mismatches (WristSlot→Wrists, BackSlot→Rear, Bag*Slot→Bag,
-- ReagentBag0Slot→Bag, AmmoSlot→Ammo) cause "Not found" warnings for
-- visible slots.
if GetInventorySlotInfo == nil then
  local __wow_inventory_slots = {
    HEADSLOT          = {1,  136516},
    NECKSLOT          = {2,  136519},
    SHOULDERSLOT      = {3,  136526},
    SHIRTSLOT         = {4,  136525},
    CHESTSLOT         = {5,  136512},
    WAISTSLOT         = {6,  136529},
    LEGSSLOT          = {7,  136517},
    FEETSLOT          = {8,  136513},
    WRISTSLOT         = {9,  136530},
    HANDSSLOT         = {10, 136515},
    FINGER0SLOT       = {11, 136514},
    FINGER1SLOT       = {12, 136514},
    TRINKET0SLOT      = {13, 136528},
    TRINKET1SLOT      = {14, 136528},
    BACKSLOT          = {15, 136521},
    MAINHANDSLOT      = {16, 136518},
    SECONDARYHANDSLOT = {17, 136524},
    RANGEDSLOT        = {18, 136520},
    TABARDSLOT        = {19, 136527},
    AMMOSLOT          = {0,  136510},
    BAG0SLOT          = {20, 136511},
    BAG1SLOT          = {21, 136511},
    BAG2SLOT          = {22, 136511},
    BAG3SLOT          = {23, 136511},
    BAG4SLOT          = {24, 136511},
    REAGENTBAG0SLOT   = {25, 136511},
    REAGENTBAGSLOT    = {25, 136511},
  }
  function GetInventorySlotInfo(slot_name)
    if type(slot_name) ~= "string" then
      return nil
    end
    local entry = __wow_inventory_slots[slot_name:upper()]
    if entry == nil then
      return nil
    end
    return entry[1], entry[2]
  end
end

-- `C_PvP` namespace used by ZoneText. The sim has no PVP zone concept,
-- so `GetZonePVPInfo` reports a neutral zone. Full namespace defined
-- because the callsite dereferences the field directly.
if C_PvP == nil then
  C_PvP = {}
end
if C_PvP.GetZonePVPInfo == nil then
  function C_PvP.GetZonePVPInfo()
    -- (pvpType, isSubZonePvP, factionName) — neutral zone, no subzone PVP
    return "contested", false, nil
  end
end

-- Zone / sub-zone text probes: sim has no world, so empty string is
-- the accurate "no zone info" answer that OnLoad handlers expect.
if GetZoneText == nil then
  function GetZoneText() return "" end
end
if GetSubZoneText == nil then
  function GetSubZoneText() return "" end
end
if GetMinimapZoneText == nil then
  function GetMinimapZoneText() return "" end
end
if GetRealZoneText == nil then
  function GetRealZoneText() return "" end
end
if UnitGroupRolesAssigned == nil then
  function UnitGroupRolesAssigned() return "NONE" end
end
if UnitGroupRolesAssignedEnum == nil then
  function UnitGroupRolesAssignedEnum() return -1 end
end
if GetInventoryItemID == nil then
  function GetInventoryItemID() return nil end
end
if GetChatWindowChannels == nil then
  function GetChatWindowChannels() return end
end
if IsInventoryItemLocked == nil then
  function IsInventoryItemLocked() return false end
end

-- Modifier-key state. Sim never has modifier keys pressed at load
-- time, so `Is{Shift,Control,Alt}KeyDown` return false. IsShiftKeyDown
-- is already in GLOBAL_FALSE_STUBS; add the three remaining modifiers
-- inline here so SecureTemplates / binding code doesn't crash.
if IsControlKeyDown == nil then function IsControlKeyDown() return false end end
if IsAltKeyDown == nil then function IsAltKeyDown() return false end end
if IsModifierKeyDown == nil then function IsModifierKeyDown() return false end end
if IsMetaKeyDown == nil then function IsMetaKeyDown() return false end end

-- Guild logo: the sim has no guild, so all colour channels are black
-- and the emblem filename is empty. Returns 10 values matching
-- Blizzard's (bkgR, bkgG, bkgB, borderR, borderG, borderB, emblemR,
-- emblemG, emblemB, emblemFilename) tuple that guild-related UI
-- destructures.
if GetGuildLogoInfo == nil then
  function GetGuildLogoInfo()
    return 0, 0, 0, 0, 0, 0, 0, 0, 0, ""
  end
end

-- Network-stats: no real socket in the sim, so bandwidth and latency are 0.
-- Returns four values so `local a, b, c, d = GetNetStats()` works for the
-- latency comparisons in Blizzard_MicroMenu and friends.
if GetNetStats == nil then
  function GetNetStats()
    return 0, 0, 0, 0
  end
end

-- Store UI is never shown in the sim, so the "is it visible" probe
-- reports false. Used by MainMenuBarMicroButtons to decide whether the
-- Store micro-button should render as pushed.
if StoreFrame_IsShown == nil then
  function StoreFrame_IsShown()
    return false
  end
end

-- `UnitIsPlayer(unit)` — true when `unit` resolves to a player-character
-- entity. In the sim, "player" and party/raid slots are players; other
-- unit IDs (target/focus/mouseover/etc.) only exist when the GUI wires
-- them up to a player, so default to false. Callers in TargetFrame,
-- PlayerFrame, etc. check this before running player-specific rendering.
if UnitIsPlayer == nil then
  function UnitIsPlayer(unit)
    if type(unit) ~= "string" then
      return false
    end
    if unit == "player" or unit == "self" then
      return true
    end
    if unit:match("^party[1-4]$") or unit:match("^raid%d+$") then
      return true
    end
    return false
  end
end

if UnitIsHumanPlayer == nil then
  function UnitIsHumanPlayer(_unit)
    return false
  end
end

if IsTargetLoose == nil then
  function IsTargetLoose()
    return false
  end
end

if UnitThreatSituation == nil then
  function UnitThreatSituation(_unit, _other_unit)
    return 0
  end
end

if UnitDetailedThreatSituation == nil then
  function UnitDetailedThreatSituation(_unit, _other_unit)
    return false, 0, 0, 0, 0
  end
end

if UnitThreatPercentageOfLead == nil then
  function UnitThreatPercentageOfLead(_unit, _other_unit)
    return 0
  end
end

if UnitTrialBankedLevels == nil then
  function UnitTrialBankedLevels(_unit)
    return 0
  end
end

if SetPortraitTexture == nil then
  function SetPortraitTexture(texture, _unit, _disablePortraitMask)
    if texture and texture.SetTexture then
      texture:SetTexture("Interface\\ICONS\\INV_Misc_QuestionMark")
    end
  end
end

if SetPortraitTextureFromCreatureDisplayID == nil then
  function SetPortraitTextureFromCreatureDisplayID(texture, _creatureDisplayID)
    if texture and texture.SetTexture then
      texture:SetTexture("Interface\\ICONS\\INV_Misc_QuestionMark")
    end
  end
end


if LE_TOKEN_REDEEM_TYPE_GAME_TIME == nil then
  LE_TOKEN_REDEEM_TYPE_GAME_TIME = 1
end

if LE_TOKEN_REDEEM_TYPE_BALANCE == nil then
  LE_TOKEN_REDEEM_TYPE_BALANCE = 2
end

if LE_TOKEN_RESULT_ERROR_BALANCE_NEAR_CAP == nil then
  LE_TOKEN_RESULT_ERROR_BALANCE_NEAR_CAP = 10
end

if IsTrialAccount == nil then
  function IsTrialAccount()
    return false
  end
end

if IsRestrictedAccount == nil then
  function IsRestrictedAccount()
    return false
  end
end

if IsTutorialFlagged == nil then
  function IsTutorialFlagged()
    return false
  end
end

if GetFileStreamingStatus == nil then
  function GetFileStreamingStatus()
    return 0
  end
end

if GetNumArenaOpponentSpecs == nil then
  function GetNumArenaOpponentSpecs()
    return 0
  end
end

if GetErrorCallstackHeight == nil then
  function GetErrorCallstackHeight()
    return 0
  end
end

if GetBackgroundLoadingStatus == nil then
  function GetBackgroundLoadingStatus()
    return 0
  end
end

if GetWebTicket == nil then
  function GetWebTicket()
    return nil
  end
end

if GetDungeonDifficultyID == nil then
  function GetDungeonDifficultyID()
    return 1
  end
end

if UnitInVehicle == nil then
  function UnitInVehicle()
    return false
  end
end

if UnitGetAvailableRoles == nil then
  function UnitGetAvailableRoles()
    return true, true, true
  end
end

if debugstack == nil then
  function debugstack()
    return ""
  end
end

if debuglocals == nil then
  function debuglocals()
    return ""
  end
end

if issecure == nil then
  function issecure()
    return true
  end
end

local function __wow_namespace(defaults)
  return setmetatable(defaults or {}, {
    __index = function(t, key)
      local fn = function()
        return nil
      end
      rawset(t, key, fn)
      return fn
    end,
  })
end

local function __wow_merge_namespace(existing, defaults)
  local namespace = type(existing) == "table" and existing or {}
  for key, value in pairs(defaults or {}) do
    if rawget(namespace, key) == nil then
      rawset(namespace, key, value)
    end
  end
  local mt = getmetatable(namespace)
  if mt == nil or mt.__index == nil then
    setmetatable(namespace, getmetatable(__wow_namespace()))
  end
  return namespace
end

local function __wow_copy_table(source)
  local copy = {}
  for key, value in pairs(source or {}) do
    copy[key] = value
  end
  return copy
end

local function __wow_make_calendar_time(dayOffset, minuteOffset)
  local day = 14 + (tonumber(dayOffset) or 0)
  local totalMinutes = (12 * 60) + (tonumber(minuteOffset) or 0)
  local hour = math.floor(totalMinutes / 60)
  local minute = totalMinutes % 60
  while minute < 0 do
    minute = minute + 60
    hour = hour - 1
  end
  while minute >= 60 do
    minute = minute - 60
    hour = hour + 1
  end
  while hour < 0 do
    hour = hour + 24
    day = day - 1
  end
  while hour >= 24 do
    hour = hour - 24
    day = day + 1
  end
  return {
    year = 2026,
    month = 4,
    monthDay = day,
    weekday = 3,
    hour = hour,
    minute = minute,
  }
end

Kiosk = __wow_merge_namespace(Kiosk, {
  IsEnabled = function() return false end,
  IsCompetitiveModeEnabled = function() return false end,
})

C_ChatInfo = __wow_merge_namespace(C_ChatInfo, {
  PerformEmote = function() return false end,
  CancelEmote = __wow_noop,
  IsValidChatLine = function() return false end,
  ReplaceIconAndGroupExpressions = function(message) return message end,
  SendChatMessage = __wow_noop,
  AreOutgoingAddonChatMessagesRestricted = function() return false end,
  GetNumReservedChatWindows = function() return 0 end,
  GetNumActiveChannels = function() return 0 end,
  GetChannelRulesetForChannelID = function() return 0 end,
  GetChannelRuleset = function() return 0 end,
  GetChannelInfoFromIdentifier = function() return nil end,
  GetChatLineText = function() return nil end,
  IsTimerunningPlayer = function() return false end,
  UncensorChatLine = __wow_noop,
  DropCautionaryChatMessage = __wow_noop,
  SendCautionaryChatMessage = __wow_noop,
  GetChannelShortcut = function(index) return tostring(index or "") end,
  GetGeneralChannelLocalID = function() return 0 end,
  GetGeneralChannelID = function() return 0 end,
  GetChannelShortcutForChannelID = function() return "" end,
  IsChannelRegionalForChannelID = function() return false end,
})

C_LFGList = __wow_merge_namespace(C_LFGList, {
  GetApplications = function() return {} end,
  GetApplicationInfo = function() return nil end,
  GetAvailableCategories = function() return {} end,
  GetAvailableRoles = function() return false, false, false end,
  GetNumApplications = function() return 0, 0 end,
  GetNumApplicants = function() return 0, 0 end,
  GetPremadeGroupFinderStyle = function() return 0 end,
  GetActivityFullName = function() return "" end,
  GetActivityInfoTable = function() return nil end,
  GetSearchResultInfo = function() return nil end,
  GetSearchResultMemberCounts = function() return 0, 0, 0, 0 end,
  HasActiveEntryInfo = function() return false end,
  HasSearchResultInfo = function() return false end,
  CanCreateQuestGroup = function() return false end,
  CanCreateScenarioGroup = function() return false end,
  IsPremadeGroupFinderEnabled = function() return false end,
  RemoveListing = __wow_noop,
})

C_AddOnProfiler = __wow_merge_namespace(C_AddOnProfiler, {
  CheckForPerformanceMessage = function() return nil end,
})

C_Ping = __wow_merge_namespace(C_Ping, {
  GetDefaultPingOptions = function() return {} end,
})

C_ZoneAbility = __wow_merge_namespace(C_ZoneAbility, {
  GetActiveAbilities = function() return {} end,
})

C_AuthChallenge = __wow_merge_namespace(C_AuthChallenge, {
  SetFrame = __wow_noop,
  Submit = __wow_noop,
  Cancel = __wow_noop,
  OnTabPressed = __wow_noop,
  DidChallengeSucceed = function() return false end,
})

C_ClassTrial = __wow_merge_namespace(C_ClassTrial, {
  IsClassTrialCharacter = function() return false end,
  GetClassTrialLogoutTimeSeconds = function() return 0 end,
})

C_CharacterServices = __wow_merge_namespace(C_CharacterServices, {
  HasRequiredBoostForClassTrial = function() return false end,
  GetCharacterServiceDisplayData = function(_boostType)
    return {
      boostLevel = GetMaxPlayerLevel and GetMaxPlayerLevel() or 80,
      flowTitle = CHARACTER_UPGRADE or "Character Upgrade",
      popupInfo = {
        textureKit = "characterupdate",
      },
    }
  end,
})

C_SocialQueue = __wow_merge_namespace(C_SocialQueue, {
  GetAllGroups = function() return {} end,
  GetConfig = function() return {} end,
  GetGroupForPlayer = function() return nil end,
  GetGroupInfo = function() return nil end,
  GetGroupMembers = function() return {} end,
  GetGroupQueues = function() return {} end,
  RequestToJoin = __wow_noop,
  SignalToastDisplayed = __wow_noop,
})

C_UnitAuras = __wow_merge_namespace(C_UnitAuras, {
  SetPrivateWarningTextAnchor = __wow_noop,
})

C_UnitAurasPrivate = __wow_merge_namespace(C_UnitAurasPrivate, {
  SetPrivateAuraAnchorAddedCallback = function(callback)
    C_UnitAurasPrivate._anchorAddedCallback = callback
  end,
  SetPrivateAuraAnchorRemovedCallback = function(callback)
    C_UnitAurasPrivate._anchorRemovedCallback = callback
  end,
  GetPrivateAuraAnchors = function()
    return {}
  end,
})

C_PetBattles = __wow_merge_namespace(C_PetBattles, {
  GetAllEffectNames = function() return end,
  GetAbilityState = function() return false, 0, 0 end,
  GetActivePet = function() return 1 end,
  IsTrapAvailable = function() return false, 0 end,
  ShouldShowPetSelect = function() return false end,
})

C_VoiceChat = __wow_merge_namespace(C_VoiceChat, {
  GetTtsVoices = function() return {} end,
  IsTranscriptionAllowed = function() return false end,
})

C_TTSSettings = __wow_merge_namespace(C_TTSSettings, {
  GetSpeechVolume = function() return 100 end,
  SetSpeechVolume = __wow_noop,
  GetSpeechRate = function() return 0 end,
  SetSpeechRate = __wow_noop,
  GetVoiceOptionID = function() return 0 end,
  SetVoiceOptionID = __wow_noop,
})

C_ClubFinder = __wow_merge_namespace(C_ClubFinder, {
  GetClubRecruitmentSettings = function()
    return {
      playStyleDungeon = false,
      playStyleRaids = false,
      playStylePvp = false,
      playStyleRP = false,
      playStyleSocial = false,
      maxLevelOnly = false,
      enableListing = false,
    }
  end,
  GetPlayerApplicantSettings = function()
    return {
      playStyleDungeon = false,
      playStyleRaids = false,
      playStylePvp = false,
      playStyleRP = false,
      playStyleSocial = false,
      roleTank = false,
      roleHealer = false,
      roleDps = false,
      sizeSmall = false,
      sizeMedium = false,
      sizeLarge = false,
      sortRelevance = true,
      sortMembers = false,
      sortNewest = false,
      crossFaction = false,
    }
  end,
})

C_PartyInfo = __wow_merge_namespace(C_PartyInfo, {
  AllowedToDoPartyConversion = function() return false end,
  IsPartyWalkIn = function() return false end,
})

C_Map = __wow_merge_namespace(C_Map, {
  GetBestMapForUnit = function() return nil end,
})

-- Bonus / world-quest objective trackers iterate the task list at startup.
-- Return an empty table so the `for ... in ipairs(tasksTable)` loops no-op.
if GetTasksTable == nil then
  function GetTasksTable()
    return {}
  end
end

-- Auto quest popups (tutorial toasts). Not simulated; `for i = 1, N do`
-- loops in AutoQuestPopUpTracker iterate zero times.
if GetNumAutoQuestPopUps == nil then
  function GetNumAutoQuestPopUps() return 0 end
end
if GetAutoQuestPopUp == nil then
  function GetAutoQuestPopUp(_index) return nil, nil end
end

-- Not in a scenario by default. Blizzard_ScenarioObjectiveTracker.lua:186
-- calls `numStages > 0` on the returned value, so numStages must be a
-- real zero, not nil.
C_Scenario = __wow_merge_namespace(C_Scenario, {
  GetInfo = function()
    -- scenarioName, currentStage, numStages, flags, _, _, _, xp, money,
    -- scenarioType, _, textureKit, scenarioID
    return nil, 0, 0, 0, nil, nil, nil, 0, 0, 0, nil, "evergreen-scenario", 0
  end,
  IsInScenario = function() return false end,
  GetStepInfo = function() return nil, 0, 0, false, false, 0, 0, 0, 0, false, false end,
})

-- AccountStore / DamageMeter / CooldownViewer: Blizzard data-provider init
-- iterates the returned category / session / cooldown list with ipairs.
-- None of these subsystems are simulated; return empty tables.
C_AccountStore = __wow_merge_namespace(C_AccountStore, {
  GetCategories = function() return {} end,
  GetCategoryInfo = function() return nil end,
  GetItemInfo = function() return nil end,
  GetCurrencyAvailable = function() return 0 end,
})

C_DamageMeter = __wow_merge_namespace(C_DamageMeter, {
  GetAvailableCombatSessions = function() return {} end,
  GetCurrentCombatSessionID = function() return nil end,
  GetDamageMeterEntries = function() return {} end,
})

C_CooldownViewer = __wow_merge_namespace(C_CooldownViewer, {
  GetCooldownViewerCategorySet = function() return {} end,
  GetCooldownViewerCooldownInfo = function() return nil end,
  GetCooldownID = function() return nil end,
})

C_Minimap = __wow_merge_namespace(C_Minimap, {
  GetNumTrackingTypes = function() return 0 end,
  GetTrackingInfo = function() return nil end,
  GetTrackingFilter = function()
    return { filterID = 0, spellID = 0 }
  end,
  SetTracking = __wow_noop,
  ClearAllTracking = __wow_noop,
})

C_Navigation = __wow_merge_namespace(C_Navigation, {
  WasClampedToScreen = function() return false end,
  GetTargetState = function() return 0 end,
  HasValidScreenPosition = function() return false end,
  GetDistance = function() return 0 end,
  GetNearestPartyMemberToken = function() return nil end,
  GetFrame = function() return UIParent end,
})

C_DateAndTime = __wow_merge_namespace(C_DateAndTime, {
  GetCurrentCalendarTime = function()
    return __wow_make_calendar_time(0, 0)
  end,
  AdjustTimeByDays = function(calendarTime, deltaDays)
    local time = __wow_copy_table(calendarTime)
    time.monthDay = (time.monthDay or 14) + (tonumber(deltaDays) or 0)
    return time
  end,
  AdjustTimeByMinutes = function(calendarTime, deltaMinutes)
    local base = __wow_copy_table(calendarTime)
    local totalMinutes = ((base.hour or 12) * 60) + (base.minute or 0) + (tonumber(deltaMinutes) or 0)
    local hour = math.floor(totalMinutes / 60)
    local minute = totalMinutes % 60
    while minute < 0 do
      minute = minute + 60
      hour = hour - 1
    end
    while minute >= 60 do
      minute = minute - 60
      hour = hour + 1
    end
    while hour < 0 do
      hour = hour + 24
      base.monthDay = (base.monthDay or 14) - 1
    end
    while hour >= 24 do
      hour = hour - 24
      base.monthDay = (base.monthDay or 14) + 1
    end
    base.hour = hour
    base.minute = minute
    return base
  end,
  GetCalendarTimeFromEpoch = function(epoch)
    return __wow_make_calendar_time(0, math.floor((tonumber(epoch) or 0) / 60))
  end,
  GetWeeklyResetStartTime = function()
    return 0
  end,
  GetSecondsUntilDailyReset = function()
    return 0
  end,
})

C_CatalogShop = __wow_merge_namespace(C_CatalogShop, {
  IsShop2Enabled = function() return false end,
  HasNewProducts = function() return false end,
  GetAvailableCategoryIDs = function() return {} end,
  GetFailureInfo = function() return nil, nil end,
  RefreshVirtualCurrencyBalance = __wow_noop,
  GetVirtualCurrencyBalance = function() return 0 end,
  OpenCatalogShopInteractionFromShop = function() return nil end,
  OpenCatalogShopInteractionFromHouse = function() return nil end,
  CloseCatalogShopInteraction = __wow_noop,
  GetFirstCategoryByProductID = function() return nil end,
  ShouldShowHousingWarning = function() return false end,
  GetProductInfo = function() return nil end,
  GetCatalogShopProductDisplayInfo = function() return nil end,
  GetProductIDsForBundle = function() return {} end,
  GetSpellVisualInfoForMount = function() return nil end,
  PurchaseProduct = __wow_noop,
  ConfirmHousingPurchase = __wow_noop,
  ProductDisplayedTelemetry = __wow_noop,
  OnLegalDisclaimerClicked = __wow_noop,
  FindBestCurrencyProductForNeededAmount = function() return nil end,
  IsProductIncludedInAnyBundle = function() return false end,
  GetProductAvailabilityTimeRemainingSecs = function() return 0 end,
})

C_WowTokenPublic = __wow_merge_namespace(C_WowTokenPublic, {
  GetCommerceSystemStatus = function() return false, 0, false end,
  UpdateTokenCount = __wow_noop,
  GetCurrentMarketPrice = function() return 0, 0 end,
  GetGuaranteedPrice = function() return 0 end,
  BuyToken = __wow_noop,
  UpdateListedAuctionableTokens = __wow_noop,
  UpdateMarketPrice = __wow_noop,
  IsAuctionableWowToken = function() return false end,
})

C_WowTokenSecure = __wow_merge_namespace(C_WowTokenSecure, {
  CancelRedeem = __wow_noop,
  GetBalanceRedeemAmount = function() return 0 end,
  SetBalanceAmountString = __wow_noop,
  GetBalanceRedemptionInfo = function() return 0, 0, false, nil end,
  GetGameTimeRedemptionInfo = function() return false, 0 end,
  GetRemainingGameTime = function() return 0 end,
  CanRedeemForBalance = function() return false end,
  RedeemToken = __wow_noop,
  WillKickFromWorld = function() return false end,
  GetTokenCount = function() return 0 end,
  RedeemTokenConfirm = __wow_noop,
  IsRedemptionStillValid = function() return false end,
  ConfirmSellToken = __wow_noop,
  ConfirmBuyToken = __wow_noop,
  GetPriceLockDuration = function() return 0 end,
})

if EnumUtil == nil then
  EnumUtil = {}
end

if EnumUtil.MakeEnum == nil then
  function EnumUtil.MakeEnum(...)
    local enum = {}
    for index = 1, select("#", ...) do
      local name = select(index, ...)
      enum[name] = index
    end
    return enum
  end
end

if CreateCounter == nil then
  function CreateCounter()
    local nextID = 0
    return function()
      nextID = nextID + 1
      return nextID
    end
  end
end

if GetOrCreateTableEntry == nil then
  function GetOrCreateTableEntry(tbl, key)
    local value = tbl[key]
    if value == nil then
      value = {}
      tbl[key] = value
    end
    return value
  end
end

if GenerateClosure == nil then
  function GenerateClosure(fn, _owner, ...)
    local bound = {...}
    return function(...)
      local args = {}
      for i = 1, #bound do
        args[#args + 1] = bound[i]
      end
      for i = 1, select("#", ...) do
        args[#args + 1] = select(i, ...)
      end
      return fn(unpack(args))
    end
  end
end

if SecureTypes == nil then
  SecureTypes = {}
end
SecureTypes.CreateSecureMap = SecureTypes.CreateSecureMap or function()
  local map = {}
  function map:Insert(key, value)
    self[key] = value
  end
  function map:Remove(key)
    local value = self[key]
    self[key] = nil
    return value
  end
  function map:Find(key)
    return self[key]
  end
  function map:Contains(key)
    return self[key] ~= nil
  end
  function map:Enumerate()
    return next, self, nil
  end
  function map:Clear()
    for key in pairs(self) do
      self[key] = nil
    end
  end
  return map
end
SecureTypes.CreateSecureFunction = SecureTypes.CreateSecureFunction or function(fn) return fn end
SecureTypes.CreateSecureNumber = SecureTypes.CreateSecureNumber or function(value) return value or 0 end
SecureTypes.CreateSecureArray = SecureTypes.CreateSecureArray or function()
  local array = {}
  function array:Insert(value)
    self[#self + 1] = value
  end
  function array:Remove(value)
    for index, existing in ipairs(self) do
      if existing == value then
        table.remove(self, index)
        return true
      end
    end
    return false
  end
  function array:Clear()
    for index = #self, 1, -1 do
      self[index] = nil
    end
  end
  function array:Enumerate()
    local index = 0
    return function()
      index = index + 1
      if index <= #self then
        return self[index]
      end
    end
  end
  return array
end

ProxyUtil = ProxyUtil or {}
ProxyConvertableMixin = ProxyConvertableMixin or {}
ProxyUtil.CreateProxy = ProxyUtil.CreateProxy or function(value) return value end
ProxyUtil.CreateProxyMixin = ProxyUtil.CreateProxyMixin or function() return {} end
ProxyUtil.SetPrivateReference = ProxyUtil.SetPrivateReference or __wow_noop
ProxyUtil.ReleasePrivateReference = ProxyUtil.ReleasePrivateReference or __wow_noop
ProxyUtil.CreateProxyDirectory = ProxyUtil.CreateProxyDirectory or function()
  return {
    ToPrivate = function(_, value) return value end,
    ToPublic = function(_, value) return value end,
  }
end

if CreateFramePool == nil then
  function CreateFramePool(frameType, parent, template, resetter)
    local pool = {
      frameType = frameType or "Frame",
      parent = parent,
      template = template,
      resetter = resetter,
      inactive = {},
      active = {},
      known = {},
    }

    function pool:Acquire()
      local frame = table.remove(self.inactive)
      local isNew = false
      if frame == nil then
        frame = CreateFrame(self.frameType, nil, self.parent, self.template)
        isNew = true
        self.known[frame] = true
      end
      self.active[frame] = true
      return frame, isNew
    end

    function pool:Release(frame)
      if frame == nil or not self:DoesObjectBelongToPool(frame) then
        return false
      end
      self.active[frame] = nil
      if self.resetter then
        self.resetter(self, frame, false, self.template)
      end
      table.insert(self.inactive, frame)
      return true
    end

    function pool:ReleaseAll()
      local frames = {}
      for frame in pairs(self.active) do
        table.insert(frames, frame)
      end
      for _, frame in ipairs(frames) do
        self:Release(frame)
      end
    end

    function pool:GetNumActive()
      local count = 0
      for _ in pairs(self.active) do
        count = count + 1
      end
      return count
    end

    function pool:DoesObjectBelongToPool(frame)
      return self.known[frame] == true
    end

    function pool:EnumerateActive()
      local frames = {}
      for frame in pairs(self.active) do
        frames[#frames + 1] = frame
      end
      local index = 0
      return function()
        index = index + 1
        return frames[index]
      end
    end

    return pool
  end
end

local function __wow_make_region_pool(acquire_region)
  return function(parent, layer, subLevel, template, resetter)
    local pool = {
      parent = parent,
      layer = layer,
      subLevel = subLevel,
      template = template,
      resetter = resetter,
      inactive = {},
      active = {},
      known = {},
    }

    function pool:Acquire()
      local region = table.remove(self.inactive)
      local isNew = false
      if region == nil then
        region = acquire_region(self.parent, self.layer, self.subLevel, self.template)
        isNew = true
        self.known[region] = true
      end
      self.active[region] = true
      return region, isNew
    end

    function pool:Release(region)
      if region == nil or not self:DoesObjectBelongToPool(region) then
        return false
      end
      self.active[region] = nil
      if self.resetter then
        self.resetter(self, region, false, self.template)
      end
      table.insert(self.inactive, region)
      return true
    end

    function pool:GetNumActive()
      local count = 0
      for _ in pairs(self.active) do
        count = count + 1
      end
      return count
    end

    function pool:DoesObjectBelongToPool(region)
      return self.known[region] == true
    end

    function pool:ReleaseAll()
      local regions = {}
      for region in pairs(self.active) do
        regions[#regions + 1] = region
      end
      for _, region in ipairs(regions) do
        self:Release(region)
      end
    end

    function pool:EnumerateActive()
      local regions = {}
      for region in pairs(self.active) do
        regions[#regions + 1] = region
      end
      local index = 0
      return function()
        index = index + 1
        return regions[index]
      end
    end

    return pool
  end
end

if CreateTexturePool == nil then
  CreateTexturePool = __wow_make_region_pool(function(parent, layer)
    return parent:CreateTexture(nil, layer or "ARTWORK")
  end)
end

if CreateFontStringPool == nil then
  CreateFontStringPool = __wow_make_region_pool(function(parent, layer)
    return parent:CreateFontString(nil, layer or "ARTWORK")
  end)
end

if CreateFramePoolCollection == nil then
  function CreateFramePoolCollection()
    local collection = { pools = {} }

    local function pool_key(frameType, parent, template, specialization)
      return table.concat({
        tostring(frameType or "Frame"),
        tostring(parent),
        tostring(template),
        tostring(specialization),
      }, "|")
    end

    function collection:CreatePool(frameType, parent, template, resetter, _forbidden, specialization)
      local key = pool_key(frameType, parent, template, specialization)
      local pool = CreateFramePool(frameType, parent, template, resetter)
      self.pools[key] = pool
      return pool
    end

    function collection:GetOrCreatePool(frameType, parent, template, resetter, forbidden, specialization)
      local key = pool_key(frameType, parent, template, specialization)
      local pool = self.pools[key]
      if pool == nil then
        pool = self:CreatePool(frameType, parent, template, resetter, forbidden, specialization)
      end
      return pool
    end

    function collection:GetNumActive()
      local total = 0
      for _, pool in pairs(self.pools) do
        total = total + (pool.GetNumActive and pool:GetNumActive() or 0)
      end
      return total
    end

    function collection:DoesObjectBelongToPool(object)
      for _, pool in pairs(self.pools) do
        if pool.DoesObjectBelongToPool and pool:DoesObjectBelongToPool(object) then
          return true
        end
      end
      return false
    end

    function collection:Release(object)
      for _, pool in pairs(self.pools) do
        if pool.Release and pool:Release(object) then
          return true
        end
      end
      return false
    end

    function collection:ReleaseAll()
      for _, pool in pairs(self.pools) do
        if pool.ReleaseAll then
          pool:ReleaseAll()
        end
      end
    end

    function collection:EnumerateActive()
      local objects = {}
      for _, pool in pairs(self.pools) do
        if pool.EnumerateActive then
          for object in pool:EnumerateActive() do
            objects[#objects + 1] = object
          end
        end
      end
      local index = 0
      return function()
        index = index + 1
        return objects[index]
      end
    end

    return collection
  end
end

if CreateFrameFactory == nil then
  function CreateFrameFactory()
    local cache = {}

    function cache:GetTemplateInfo(template)
      if C_XMLUtil and C_XMLUtil.GetTemplateInfo then
        local info = C_XMLUtil.GetTemplateInfo(template)
        if info then
          return info
        end
      end
      return { width = 0, height = 0 }
    end

    local factory = {}

    function factory:GetTemplateInfoCache()
      return cache
    end

    function factory:Create(parent, frameTypeOrTemplate, resetFunc)
      local frame = nil
      if type(frameTypeOrTemplate) == "string" and C_XMLUtil and C_XMLUtil.GetTemplateInfo and C_XMLUtil.GetTemplateInfo(frameTypeOrTemplate) then
        frame = CreateFrame("Frame", nil, parent, frameTypeOrTemplate)
      else
        frame = CreateFrame(type(frameTypeOrTemplate) == "string" and frameTypeOrTemplate or "Frame", nil, parent)
      end
      if resetFunc then
        resetFunc(nil, frame, true, frameTypeOrTemplate)
      end
      return frame, true
    end

    return factory
  end
end

if AddSourceLocationExclude == nil then
  function AddSourceLocationExclude()
  end
end

if GetGlobalEnvironment == nil then
  function GetGlobalEnvironment()
    return _G
  end
end

if GetButtonMetatable == nil then
  function GetButtonMetatable()
    if CreateFrame == nil then
      return nil
    end
    local frame = CreateFrame("Button")
    return frame and getmetatable(frame) or nil
  end
end

if secretwrap == nil then
  function secretwrap(fn)
    return fn
  end
end

if GetCallstackHeight == nil then
  function GetCallstackHeight()
    return 0
  end
end

if SetErrorCallstackHeight == nil then
  function SetErrorCallstackHeight()
  end
end

if GetBuildInfo == nil then
  function GetBuildInfo()
    return "12.0.5", "66102", "Apr 14 2026", 120005, "", " "
  end
end

if GetRealmName == nil then
  function GetRealmName()
    return "SimulatedRealm"
  end
end

if GetNormalizedRealmName == nil then
  function GetNormalizedRealmName()
    return "SimulatedRealm"
  end
end

if GetRealmID == nil then
  function GetRealmID()
    return 1
  end
end

if GetExpansionLevel == nil then
  function GetExpansionLevel()
    return 10
  end
end

if IsMacClient == nil then
  function IsMacClient()
    return false
  end
end

if UnitClass == nil then
  function UnitClass(_unit)
    return "Paladin", "PALADIN", 2
  end
end

if UnitRace == nil then
  function UnitRace(_unit)
    return "Human", "Human", 1
  end
end

if UnitNameUnmodified == nil then
  function UnitNameUnmodified(_unit)
    return "SimPlayer", GetRealmName()
  end
end

if GetChatTypeIndex == nil then
  function GetChatTypeIndex()
    return 1
  end
end

if GetScenariosChoiceOrder == nil then
  function GetScenariosChoiceOrder()
    return {}
  end
end

if GetProfessionSkillLineID == nil then
  function GetProfessionSkillLineID()
    return 0
  end
end

if UnitSex == nil then
  function UnitSex()
    return 2
  end
end

if UnitIsGhost == nil then
  function UnitIsGhost(_unit)
    return false
  end
end

if UnitIsDead == nil then
  function UnitIsDead(_unit)
    return false
  end
end

if CanAutoSetGamePadCursorControl == nil then
  function CanAutoSetGamePadCursorControl(_enabled)
    return false
  end
end

if SetGamePadCursorControl == nil then
  function SetGamePadCursorControl(_enabled)
  end
end

if LocalizedClassList == nil then
  function LocalizedClassList(_female)
    return {
      WARRIOR = "Warrior",
      PALADIN = "Paladin",
      HUNTER = "Hunter",
      ROGUE = "Rogue",
      PRIEST = "Priest",
      DEATHKNIGHT = "Death Knight",
      SHAMAN = "Shaman",
      MAGE = "Mage",
      WARLOCK = "Warlock",
      MONK = "Monk",
      DRUID = "Druid",
      DEMONHUNTER = "Demon Hunter",
      EVOKER = "Evoker",
    }
  end
end

StaticPopupDialogs = StaticPopupDialogs or {}

if StaticPopup_AddShowCondition == nil then
  function StaticPopup_AddShowCondition()
  end
end

if RegisterUIPanel == nil then
  function RegisterUIPanel()
  end
end

if CloseAllWindows == nil then
  function CloseAllWindows()
    return false
  end
end

if AddTooltipDataAccessor == nil then
  function AddTooltipDataAccessor()
  end
end

if RegisterEventCallback == nil then
  function RegisterEventCallback(_event, _callback)
  end
end

if DevTools_AddMessageHandler == nil then
  function DevTools_AddMessageHandler(_handler)
  end
end

if UnregisterEventCallback == nil then
  function UnregisterEventCallback(_event, _callback)
  end
end

if RegisterUnitEventCallback == nil then
  function RegisterUnitEventCallback(_event, _callback, _unit)
  end
end

if UnregisterUnitEventCallback == nil then
  function UnregisterUnitEventCallback(_event, _callback, _unit)
  end
end

if GetScreenWidth == nil then
  function GetScreenWidth()
    return 1024
  end
end

if GetScreenHeight == nil then
  function GetScreenHeight()
    return 768
  end
end

if GetPhysicalScreenSize == nil then
  function GetPhysicalScreenSize()
    return GetScreenWidth(), GetScreenHeight()
  end
end

if GetNumLanguages == nil then
  function GetNumLanguages()
    return 0
  end
end

if UnitName == nil then
  function UnitName(unit)
    return UnitNameUnmodified(unit)
  end
end

if UnitGUID == nil then
  function UnitGUID(unit)
    return "Player-0-00000000-" .. tostring(unit or "player")
  end
end

if UnitIsConnected == nil then
  function UnitIsConnected(_unit)
    return true
  end
end

if UnitIsPossessed == nil then
  function UnitIsPossessed(_unit)
    return false
  end
end

if LE_REALM_RELATION_SAME == nil then
  LE_REALM_RELATION_SAME = 0
end

if UnitRealmRelationship == nil then
  function UnitRealmRelationship(_unit)
    return LE_REALM_RELATION_SAME
  end
end

if UnitPosition == nil then
  function UnitPosition(_unit)
    return 0, 0, 0, 0
  end
end

if UnitLevel == nil then
  function UnitLevel(unit)
    if unit == "player" or unit == "pet" then
      return GetMaxPlayerLevel and GetMaxPlayerLevel() or 80
    end
    return 1
  end
end

if UnitEffectiveLevel == nil then
  function UnitEffectiveLevel(unit)
    return UnitLevel(unit)
  end
end

if GetMaxPlayerLevel == nil then
  function GetMaxPlayerLevel()
    return 80
  end
end

if EJ_GetInstanceInfo == nil then
  function EJ_GetInstanceInfo(_instanceID)
    return "", "", 0, 0, 0, 0, 0, 0, false, 0, 0, false
  end
end

if GetClientDisplayExpansionLevel == nil then
  function GetClientDisplayExpansionLevel()
    return 10
  end
end

if GetAccountExpansionLevel == nil then
  function GetAccountExpansionLevel()
    return GetClientDisplayExpansionLevel()
  end
end

if GetMaxLevelForExpansionLevel == nil then
  function GetMaxLevelForExpansionLevel(_expansion_level)
    return GetMaxPlayerLevel()
  end
end

if GetMaxLevelForPlayerExpansion == nil then
  function GetMaxLevelForPlayerExpansion()
    return GetMaxLevelForExpansionLevel(GetAccountExpansionLevel())
  end
end

if GetExpansionDisplayInfo == nil then
  function GetExpansionDisplayInfo(_expansionLevel, _desiredReleaseType)
    return {
      logo = 0,
      banner = "",
      features = {},
      highResBackgroundID = 0,
      lowResBackgroundID = 0,
      textureKit = "",
      glueAmbianceSoundKit = nil,
      glueMusicSoundKit = nil,
      glueCreditsSoundKit = nil,
    }
  end
end

if UnitInPartyIsAI == nil then
  function UnitInPartyIsAI(_unit)
    return false
  end
end

if UnitAffectingCombat == nil then
  function UnitAffectingCombat(_unit)
    return false
  end
end

if UnitIsPVPFreeForAll == nil then
  function UnitIsPVPFreeForAll(_unit)
    return false
  end
end

if UnitPhaseReason == nil then
  function UnitPhaseReason(_unit)
    return nil
  end
end

if PartialPlayTime == nil then
  function PartialPlayTime()
    return false
  end
end

if NoPlayTime == nil then
  function NoPlayTime()
    return false
  end
end

if GetBillingTimeRested == nil then
  function GetBillingTimeRested()
    return 0
  end
end

if GetUnitTotalModifiedMaxHealthPercent == nil then
  function GetUnitTotalModifiedMaxHealthPercent(_unit)
    return 0
  end
end

if GetNumShapeshiftForms == nil then
  function GetNumShapeshiftForms()
    return 0
  end
end

if GetShapeshiftForm == nil then
  function GetShapeshiftForm()
    return 0
  end
end

if GetShapeshiftFormInfo == nil then
  function GetShapeshiftFormInfo(_index)
    return nil, false, false, nil
  end
end

if GetTotemInfo == nil then
  function GetTotemInfo(_slot)
    return false, nil, 0, 0, nil
  end
end

if GetPetActionInfo == nil then
  function GetPetActionInfo(_index)
    return nil, nil, false, false, false, false, nil, false, false
  end
end

if GetPetActionCooldown == nil then
  function GetPetActionCooldown(_index)
    return 0, 0, 0
  end
end

if PetHasActionBar == nil then
  function PetHasActionBar()
    return false
  end
end

if GetNumSpecializations == nil then
  function GetNumSpecializations()
    return 3
  end
end

if GetSpecializationInfoForClassID == nil then
  function GetSpecializationInfoForClassID(classID, index, _sex)
    local specID = ((tonumber(classID) or 0) * 100) + (tonumber(index) or 1)
    return specID, "Spec " .. tostring(index or 1), "", 0, "DAMAGER", false, true
  end
end

if GetDifficultyInfo == nil then
  function GetDifficultyInfo(_difficultyID)
    return "Normal", 0, false, false, false, false
  end
end

if GetReleaseTimeRemaining == nil then
  function GetReleaseTimeRemaining()
    return 0
  end
end

if GetExpansionTrialInfo == nil then
  function GetExpansionTrialInfo()
    return false, 0
  end
end

if GetInventoryItemTexture == nil then
  function GetInventoryItemTexture(_unit, _slot)
    return nil
  end
end

if IsInventoryItemProfessionBag == nil then
  function IsInventoryItemProfessionBag(_unit, _slot)
    return false
  end
end

if GetSendMailPrice == nil then
  function GetSendMailPrice()
    return 0
  end
end

if GetMerchantFilter == nil then
  function GetMerchantFilter()
    return 0
  end
end

if SetMerchantFilter == nil then
  function SetMerchantFilter(_filter)
  end
end

if IsVeteranTrialAccount == nil then
  function IsVeteranTrialAccount()
    return false
  end
end

if IsAccountSecured == nil then
  function IsAccountSecured()
    return true
  end
end

if AbbreviateNumbers == nil then
  function AbbreviateNumbers(value)
    return tostring(value or 0)
  end
end

if BNGetInfo == nil then
  function BNGetInfo()
    return nil
  end
end

if GetLFGDeserterExpiration == nil then
  function GetLFGDeserterExpiration()
    return 0
  end
end

if StoreSecureReference == nil then
  function StoreSecureReference(name, value)
    if type(name) == "string" then
      rawset(_G, name, value)
    end
  end
end

if UnitStagger == nil then
  function UnitStagger(_unit)
    return 0
  end
end

if GetPossessInfo == nil then
  function GetPossessInfo(_index)
    return nil, nil, false
  end
end

if IsInJailersTower == nil then
  function IsInJailersTower()
    return false
  end
end

TooltipDataProcessor = TooltipDataProcessor or __wow_namespace({
  AllTypes = 0,
  AddTooltipPostCall = __wow_noop,
  AddLinePostCall = __wow_noop,
})

EventRegistry = EventRegistry or __wow_namespace({
  RegisterCallback = __wow_noop,
  TriggerEvent = __wow_noop,
  RegisterFrameEventAndCallback = __wow_noop,
})

UIWidgetManager = UIWidgetManager or __wow_namespace({
  RegisterWidgetVisTypeTemplate = __wow_noop,
})

Settings = Settings or __wow_namespace({
  GetOrCreateSettingsGroup = function()
    return __wow_namespace({
      AddInitializer = __wow_noop,
      AddSetting = __wow_noop,
      AddCategory = __wow_noop,
      SetValue = __wow_noop,
      GetValue = function() return nil end,
    })
  end,
})

EditModeAccountSettingsMixin = EditModeAccountSettingsMixin or {}
BaseActionButtonMixin = BaseActionButtonMixin or {}

if bit == nil then
  local function normalize(v)
    v = math.floor(tonumber(v) or 0)
    if v < 0 then
      v = 0x100000000 + v
    end
    return v % 0x100000000
  end

  local function fold(values, identity, step)
    local result = identity
    for i = 1, #values do
      result = step(result, normalize(values[i]))
    end
    return normalize(result)
  end

  local function lshift(a, n)
    return normalize(normalize(a) * (2 ^ normalize(n)))
  end

  local function rshift(a, n)
    return math.floor(normalize(a) / (2 ^ normalize(n)))
  end

  local function band2(a, b)
    local result = 0
    local bitValue = 1
    a = normalize(a)
    b = normalize(b)
    while a > 0 or b > 0 do
      local abit = a % 2
      local bbit = b % 2
      if abit == 1 and bbit == 1 then
        result = result + bitValue
      end
      a = math.floor(a / 2)
      b = math.floor(b / 2)
      bitValue = bitValue * 2
    end
    return result
  end

  local function bor2(a, b)
    local result = 0
    local bitValue = 1
    a = normalize(a)
    b = normalize(b)
    while a > 0 or b > 0 do
      local abit = a % 2
      local bbit = b % 2
      if abit == 1 or bbit == 1 then
        result = result + bitValue
      end
      a = math.floor(a / 2)
      b = math.floor(b / 2)
      bitValue = bitValue * 2
    end
    return result
  end

  bit = {
    band = function(...)
      return fold({...}, 0xFFFFFFFF, band2)
    end,
    bor = function(...)
      return fold({...}, 0, bor2)
    end,
    bxor = function(a, b)
      a = normalize(a)
      b = normalize(b)
      local result = 0
      local bitValue = 1
      while a > 0 or b > 0 do
        local abit = a % 2
        local bbit = b % 2
        if abit ~= bbit then
          result = result + bitValue
        end
        a = math.floor(a / 2)
        b = math.floor(b / 2)
        bitValue = bitValue * 2
      end
      return result
    end,
    bnot = function(a)
      return 0xFFFFFFFF - normalize(a)
    end,
    lshift = lshift,
    rshift = rshift,
    arshift = rshift,
    mod = function(a, b)
      return normalize(a) % normalize(b)
    end,
  }
end

local __cvars = rawget(_G, "__wow_cvars") or {}
rawset(_G, "__wow_cvars", __cvars)
-- Pre-seed CVars that Blizzard OnLoad code reads before any addon has
-- a chance to SetCVar. Each default matches the value WoW ships with.
local __cvar_defaults = {
  timeMgrAlarmTime = "0",
  timeMgrUseMilitaryTime = "0",
  timeMgrUseLocalTime = "0",
  showTimestamps = "none",
  spellActivationOverlayOpacity = "1.0",
}
for k, v in pairs(__cvar_defaults) do
  if __cvars[k] == nil then __cvars[k] = v end
end

C_CVar = C_CVar or __wow_namespace({
  GetCVar = function(name)
    return __cvars[name]
  end,
  SetCVar = function(name, value)
    __cvars[name] = value == nil and nil or tostring(value)
    return true
  end,
  GetCVarBool = function(name)
    local value = __cvars[name]
    return value ~= nil and value ~= "0" and value ~= false
  end,
  GetCVarDefault = function(name)
    return __cvars[name] or "0"
  end,
  RegisterCVar = __wow_noop,
  ResetTestCVars = __wow_noop,
  GetCVarBitfield = function() return false end,
  SetCVarBitfield = function() return true end,
})

C_UIColor = C_UIColor or __wow_namespace({
  GetColors = function()
    return {
      { baseTag = "HIGHLIGHT_FONT_COLOR", color = { r = 1, g = 1, b = 1, a = 1 } },
      { baseTag = "PLAYER_FACTION_COLOR_HORDE", color = { r = 1, g = 0.1, b = 0.1, a = 1 } },
      { baseTag = "PLAYER_FACTION_COLOR_ALLIANCE", color = { r = 0.2, g = 0.4, b = 1, a = 1 } },
      { baseTag = "NORMAL_FONT_COLOR", color = { r = 1, g = 0.82, b = 0, a = 1 } },
    }
  end,
})

C_ColorUtil = C_ColorUtil or __wow_namespace({
  ConvertRGBToHSV = function(r, g, b)
    return 0, 0, math.max(r or 0, g or 0, b or 0)
  end,
  ConvertHSVToHSL = function(h, s, v)
    return h or 0, s or 0, v or 0
  end,
  GenerateTextColorCode = function(color)
    local r = math.floor((color.r or 1) * 255)
    local g = math.floor((color.g or 1) * 255)
    local b = math.floor((color.b or 1) * 255)
    return string.format("ff%02x%02x%02x", r, g, b)
  end,
  WrapTextInColor = function(text, color)
    return "|c" .. C_ColorUtil.GenerateTextColorCode(color) .. tostring(text or "") .. "|r"
  end,
  WrapTextInColorCode = function(text, colorCode)
    local code = tostring(colorCode or "ffffffff"):gsub("^|c", "")
    return "|c" .. code .. tostring(text or "") .. "|r"
  end,
})

C_CurveUtil = C_CurveUtil or __wow_namespace({
  CreateCurve = function()
    local curve = { points = {}, curveType = 0 }
    function curve:AddPoint(x, y)
      self.points[#self.points + 1] = { x = x, y = y }
    end
    function curve:SetType(curveType)
      self.curveType = curveType
    end
    return curve
  end,
})

C_EventUtils = C_EventUtils or __wow_namespace({
  IsEventValid = function() return true end,
})

C_FunctionContainers = C_FunctionContainers or __wow_namespace({
  CreateCallback = function(fn) return fn end,
})

C_Sound = C_Sound or __wow_namespace()
C_GameRules = C_GameRules or __wow_namespace()
-- Explicit members so their return shapes are accurate (callers multiply
-- by the return value, so nil-from-namespace-default crashes).
-- No game rules are active in the sim: IsGameRuleActive = false,
-- GetGameRuleAsFloat = 0, GetGameRuleAsInt = 0, GetGameRuleAsString = "".
if rawget(C_GameRules, "IsGameRuleActive") == nil then
  function C_GameRules.IsGameRuleActive(_rule) return false end
end
if rawget(C_GameRules, "GetGameRuleAsFloat") == nil then
  function C_GameRules.GetGameRuleAsFloat(_rule) return 0 end
end
if rawget(C_GameRules, "GetGameRuleAsInt") == nil then
  function C_GameRules.GetGameRuleAsInt(_rule) return 0 end
end
if rawget(C_GameRules, "GetGameRuleAsString") == nil then
  function C_GameRules.GetGameRuleAsString(_rule) return "" end
end
if rawget(C_GameRules, "IsPlunderstorm") == nil then
  function C_GameRules.IsPlunderstorm() return false end
end
if rawget(C_GameRules, "GetActiveGameMode") == nil then
  function C_GameRules.GetActiveGameMode()
    return (Enum and Enum.GameMode and Enum.GameMode.Standard) or 0
  end
end
if rawget(C_GameRules, "GetGameModeGlueScreenName") == nil then
  function C_GameRules.GetGameModeGlueScreenName() return "CharacterSelect" end
end

-- Pet battles: not simulated. `GetNumPets` is compared numerically
-- during PetBattleFrame OnLoad refresh, so returning nil crashes
-- `petIndex > GetNumPets(owner)`. Zero is the accurate "no pets" answer.
C_PetBattles = C_PetBattles or __wow_namespace()
if rawget(C_PetBattles, "GetNumPets") == nil then
  function C_PetBattles.GetNumPets(_owner) return 0 end
end
if rawget(C_PetBattles, "GetBattleState") == nil then
  -- Enum.PetbattleState.PVEInvitationSent = 0 in Blizzard's enums; return
  -- 0 as a safe "no active battle" sentinel.
  function C_PetBattles.GetBattleState() return 0 end
end

-- LFG group-finder probes. Neither applies in the sim: no group-finder
-- usage and no active proposal. `GetLFGProposal` returns 15 values
-- callers destructure, so match that shape.
if rawget(C_LFGInfo or {}, "CanPlayerUseGroupFinder") == nil then
  C_LFGInfo = C_LFGInfo or __wow_namespace()
  function C_LFGInfo.CanPlayerUseGroupFinder()
    return false, ""
  end
end
if rawget(C_LFGInfo or {}, "IsInLFGFollowerDungeon") == nil then
  C_LFGInfo = C_LFGInfo or __wow_namespace()
  function C_LFGInfo.IsInLFGFollowerDungeon()
    return false
  end
end
if GetLFGProposal == nil then
  function GetLFGProposal()
    -- (proposalExists, id, typeID, subtypeID, name, backgroundTexture,
    --  role, hasResponded, totalEncounters/numBosses, completedEncounters,
    --  numMembers, isLeader, isHoliday, _, isSilent)
    return false, 0, 0, 0, "", "", "", false, 0, 0, 0, false, false, nil, false
  end
end
if GetLFGProposalEncounter == nil then
  function GetLFGProposalEncounter(_i)
    return "", "", false
  end
end
if GetLFGInfoServer == nil then
  function GetLFGInfoServer()
    return false, false, false, false, false, 0, 0, 0, ""
  end
end
if GetLFGRoleUpdate == nil then
  function GetLFGRoleUpdate()
    -- (inProgress, slots, members, category, lfgID, bgQueue)
    return false, 0, 0, 0, 0, false
  end
end
function HasCompletedAnyAchievement()
  return true
end
function CanShowAchievementUI()
  return true
end
if GetTotalAchievementPoints == nil then
  function GetTotalAchievementPoints()
    return 0
  end
end
if GetPartyLFGID == nil then
  function GetPartyLFGID() return 0 end
end

-- Adventure journal: the sim has no adventure content.
C_AdventureJournal = C_AdventureJournal or __wow_namespace()

-- Store / shop public API: sim has no store.
C_StorePublic = C_StorePublic or __wow_namespace()
if rawget(C_StorePublic, "IsEnabled") == nil then
  function C_StorePublic.IsEnabled() return false end
end
if rawget(C_StorePublic, "IsDisabledByParentalControls") == nil then
  function C_StorePublic.IsDisabledByParentalControls() return false end
end

-- Additional LFG helpers.
if GetLFGCategoryForID == nil then
  function GetLFGCategoryForID() return 0 end
end

-- Battle.net friends count: sim has no BNet connection.
if BNGetNumFriends == nil then
  function BNGetNumFriends() return 0, 0 end
end
if BNGetNumFriendInvites == nil then
  function BNGetNumFriendInvites() return 0 end
end

-- Friend list: sim has no social layer.
C_FriendList = C_FriendList or __wow_namespace()
if rawget(C_FriendList, "GetNumFriends") == nil then
  function C_FriendList.GetNumFriends() return 0 end
end
if rawget(C_FriendList, "GetNumOnlineFriends") == nil then
  function C_FriendList.GetNumOnlineFriends() return 0 end
end

-- Region / language / battlefield stubs.
if GetCurrentRegionName == nil then
  function GetCurrentRegionName() return "US" end
end
if GetDefaultLanguage == nil then
  function GetDefaultLanguage() return "Common", 1 end
end
if GetMaxBattlefieldID == nil then
  function GetMaxBattlefieldID() return 0 end
end
if IsActiveBattlefieldArena == nil then
  function IsActiveBattlefieldArena() return false end
end
if UnitExists == nil then
  function UnitExists(unit)
    return unit == "player"
  end
end

-- Social / commentator: sim has no social restrictions or spectator.
C_SocialRestrictions = C_SocialRestrictions or __wow_namespace()
if rawget(C_SocialRestrictions, "IsChatDisabled") == nil then
  function C_SocialRestrictions.IsChatDisabled() return false end
end
C_Commentator = C_Commentator or __wow_namespace()
if rawget(C_Commentator, "IsSpectating") == nil then
  function C_Commentator.IsSpectating() return false end
end

-- Guild bank: not simulated; single callsite in GuildControlUI.
C_GuildBank = C_GuildBank or __wow_namespace()

-- Guild-info probes: sim has no guild, no locale variants, no ranks.
-- Accurate "empty guild state" returns keep MainMenuBarMicroButtons
-- and the guild UIs from crashing their OnLoad chains.
C_GuildInfo = C_GuildInfo or __wow_namespace()
if rawget(C_GuildInfo, "GetClubId") == nil then
  function C_GuildInfo.GetClubId() return nil end
end
if rawget(C_GuildInfo, "IsGuildOfficer") == nil then
  function C_GuildInfo.IsGuildOfficer() return false end
end
if rawget(C_GuildInfo, "CanSpeakInGuildChat") == nil then
  function C_GuildInfo.CanSpeakInGuildChat() return true end
end
-- `GetAvailableLocaleInfo()` — list of shipped retail locales.
-- Each entry mirrors the Blizzard LocaleInfo shape:
--   { localeId: integer, localeName: "xxYY", englishName, displayName }.
-- localeId values follow Blizzard's internal 1..N enumeration; consumers
-- (Settings/Language dropdown, Blizzard_Settings) iterate the list and
-- match by localeName, so the numbering only matters for stable order.
if GetAvailableLocaleInfo == nil then
  function GetAvailableLocaleInfo()
    return {
      { localeId = 1,  localeName = "enUS", englishName = "English (US)",         displayName = "English (US)"         },
      { localeId = 2,  localeName = "enGB", englishName = "English (UK)",         displayName = "English (UK)"         },
      { localeId = 3,  localeName = "frFR", englishName = "French",               displayName = "Français"             },
      { localeId = 4,  localeName = "deDE", englishName = "German",               displayName = "Deutsch"              },
      { localeId = 5,  localeName = "esES", englishName = "Spanish (Spain)",      displayName = "Español (EU)"         },
      { localeId = 6,  localeName = "esMX", englishName = "Spanish (Latin America)", displayName = "Español (AL)"      },
      { localeId = 7,  localeName = "itIT", englishName = "Italian",              displayName = "Italiano"             },
      { localeId = 8,  localeName = "ptBR", englishName = "Portuguese (Brazil)",  displayName = "Português (Brasil)"   },
      { localeId = 9,  localeName = "ruRU", englishName = "Russian",              displayName = "Русский"              },
      { localeId = 10, localeName = "koKR", englishName = "Korean",               displayName = "한국어"                },
      { localeId = 11, localeName = "zhCN", englishName = "Chinese (Simplified)", displayName = "简体中文"              },
      { localeId = 12, localeName = "zhTW", englishName = "Chinese (Traditional)",displayName = "繁體中文"              },
    }
  end
end
if GetGuildFactionGroup == nil then
  function GetGuildFactionGroup()
    return 1
  end
end
if GuildControlSetRank == nil then
  function GuildControlSetRank(_rankIndex) end
end
if GuildControlGetRankName == nil then
  function GuildControlGetRankName(_index) return "" end
end
if GuildControlGetNumRanks == nil then
  function GuildControlGetNumRanks() return 0 end
end
if GuildControlGetRankFlags == nil then
  function GuildControlGetRankFlags() return {} end
end
if GetGroupMemberCounts == nil then
  function GetGroupMemberCounts()
    return {
      TANK = 0,
      HEALER = 0,
      DAMAGER = 0,
      NOROLE = 0,
    }
  end
end
if GetLootSpecialization == nil then
  function GetLootSpecialization()
    return 0
  end
end

if GetSpellConfirmationPromptsInfo == nil then
  function GetSpellConfirmationPromptsInfo()
    return {}
  end
end

if GetActiveLootRollIDs == nil then
  function GetActiveLootRollIDs()
    return {}
  end
end

if GetNumArenaOpponents == nil then
  function GetNumArenaOpponents()
    return 0
  end
end
if C_EditMode == nil then
  C_EditMode = __wow_namespace()
end
if rawget(C_EditMode, "GetAccountSettings") == nil then
  local function __wow_default_edit_mode_account_setting(setting)
    if setting == Enum.EditModeAccountSetting.ShowGrid then
      return 0
    elseif setting == Enum.EditModeAccountSetting.GridSpacing then
      return Constants.EditModeConsts.EditModeDefaultGridSpacing or 100
    elseif setting == Enum.EditModeAccountSetting.SettingsExpanded then
      return 0
    elseif setting == Enum.EditModeAccountSetting.EnableAdvancedOptions then
      return 0
    end
    return 1
  end

  function C_EditMode.GetAccountSettings()
    local settings = {}
    for _, setting in pairs(Enum.EditModeAccountSetting or {}) do
      if type(setting) == "number" then
        table.insert(settings, {
          setting = setting,
          value = __wow_default_edit_mode_account_setting(setting),
        })
      end
    end
    table.sort(settings, function(a, b) return a.setting < b.setting end)
    return settings
  end

  function C_EditMode.GetLayouts()
    return {
      layouts = {},
      activeLayout = 1,
    }
  end

  function C_EditMode.SetActiveLayout(_layoutIndex)
  end
end
if WorldLootObjectExists == nil then
  function WorldLootObjectExists(_unit)
    return false
  end
end

-- Housing: not simulated. The only surface accessed at load is
-- IsHousingServiceEnabled (MainMenuBarMicroButtons gates the housing
-- micro-button on it).
C_Housing = C_Housing or __wow_namespace()
if rawget(C_Housing, "IsHousingServiceEnabled") == nil then
  function C_Housing.IsHousingServiceEnabled() return false end
end
C_RestrictedActions = C_RestrictedActions or __wow_namespace()
C_ScriptedAnimations = C_ScriptedAnimations or __wow_namespace()
C_PaperDollInfo = C_PaperDollInfo or __wow_namespace()
C_CombatAudioAlert = C_CombatAudioAlert or __wow_namespace()
C_ContentTracking = __wow_merge_namespace(C_ContentTracking, {
  -- AchievementObjectiveTracker iterates `#trackedAchievements` at load.
  -- Return an empty list so the subsequent for-loop is a no-op.
  GetTrackedIDs = function() return {} end,
  IsTracking = function() return false end,
})

-- InitiativeTasksObjectiveTracker indexes `.trackedIDs` on the returned
-- value, so return a real table even when there are no initiatives.
C_NeighborhoodInitiative = __wow_merge_namespace(C_NeighborhoodInitiative, {
  GetTrackedInitiativeTasks = function()
    return { trackedIDs = {} }
  end,
  GetInitiativeTaskInfo = function() return nil end,
  RemoveTrackedInitiativeTask = __wow_noop,
  AddTrackedInitiativeTask = __wow_noop,
})
C_Widget = C_Widget or __wow_namespace()
C_SuperTrack = __wow_merge_namespace(C_SuperTrack, {
  GetSuperTrackedQuestID = function() return 0 end,
  SetSuperTrackedQuestID = __wow_noop,
})
C_AutoComplete = __wow_merge_namespace(C_AutoComplete, {
  GetAutoCompleteRealms = function() return {} end,
})
C_TransmogOutfitInfo = C_TransmogOutfitInfo or __wow_namespace({
  GetOutfitInfo = function() return nil end,
})
C_Macro = C_Macro or __wow_namespace({
  GetNumMacros = function() return 0, 0 end,
})
C_ActionBar = C_ActionBar or __wow_namespace({
  HasVehicleActionBar = function() return false end,
  HasOverrideActionBar = function() return false end,
  GetOverrideBarSkin = function() return nil end,
  HasBonusActionBar = function() return false end,
  HasTempShapeshiftActionBar = function() return false end,
  HasExtraActionBar = function() return false end,
  IsPossessBarVisible = function() return false end,
  HasAssistedCombatActionButtons = function() return false end,
  IsAssistedCombatAction = function() return false end,
  GetVehicleBarIndex = function() return 1 end,
  GetOverrideBarIndex = function() return 1 end,
  GetTempShapeshiftBarIndex = function() return 1 end,
  GetBonusBarIndex = function() return 1 end,
  GetExtraBarIndex = function() return 1 end,
  GetMultiCastBarIndex = function() return 1 end,
  GetActionBarPage = function() return 1 end,
  SetActionBarPage = __wow_noop,
  HasAction = function() return false end,
  GetActionTexture = function() return nil end,
  UsesActionText = function() return false end,
  GetActionText = function() return "" end,
  FindSpellActionButtons = function() return {} end,
  FindPetActionButtons = function() return {} end,
  FindFlyoutActionButtons = function() return {} end,
  GetPetActionPetBarIndices = function() return {} end,
})

C_Traits = C_Traits or __wow_namespace({
  GetTreeNodes = function() return {} end,
  GetNodeInfo = function()
    return {
      ranksIncreased = 0,
      entryIDToRanksIncreased = {},
      totalMaxRanks = 0,
    }
  end,
})

C_TradeSkillUI = __wow_merge_namespace(C_TradeSkillUI, {
  GetProfessionSkillLineID = function(professionID)
    return tonumber(professionID) or 0
  end,
  IsGuildTradeSkillsEnabled = function()
    return false
  end,
  GetTradeSkillTexture = function()
    return nil
  end,
  GetTradeSkillDisplayName = function()
    return ""
  end,
  -- GetRecipesTracked / IsRecipeTracked / SetRecipeTracked are now backed
  -- by Rust functions in `globals/rilua_missing_surface/professions.rs`.
  -- The merge here is a no-op because those names are already populated
  -- by `register_profession_surface` before runtime-surface bootstrap runs.
})

C_QuestLog = __wow_merge_namespace(C_QuestLog, {
  ReadyForTurnIn = function()
    return false
  end,
  -- World-quest watch list: empty (no watched quests).
  GetNumWorldQuestWatches = function() return 0 end,
  GetQuestIDForWorldQuestWatchIndex = function() return nil end,
  -- Objective-tracker iteration for active quests: empty.
  GetNumQuestWatches = function() return 0 end,
  GetQuestIDForQuestWatchIndex = function() return nil end,
})

C_ColorOverrides = __wow_merge_namespace(C_ColorOverrides, {
  GetColorForQuality = function()
    return CreateColor(1, 1, 1)
  end,
})

C_PvP = __wow_merge_namespace(C_PvP, {
  IsInBrawl = function() return false end,
  IsSoloShuffle = function() return false end,
  GetArenaCrowdControlInfo = function() return nil, 0, 0 end,
})

C_ScriptedAnimations = __wow_merge_namespace(C_ScriptedAnimations, {
  GetAllScriptedAnimationEffects = function()
    return {}
  end,
})

C_XMLUtil = C_XMLUtil or __wow_namespace({
  GetTemplateInfo = function()
    return nil
  end,
})

C_ToyBoxInfo = __wow_merge_namespace(C_ToyBoxInfo, {
  IsUsingDefaultFilters = function() return true end,
})

C_HeirloomInfo = __wow_merge_namespace(C_HeirloomInfo, {
  IsUsingDefaultFilters = function() return true end,
})

C_TransmogCollection = __wow_merge_namespace(C_TransmogCollection, {
  IsUsingDefaultFilters = function() return true end,
})

C_MountJournal = __wow_merge_namespace(C_MountJournal, {
  IsUsingDefaultFilters = function() return true end,
  GetDynamicFlightModeSpellID = function() return 0 end,
})

C_PetJournal = __wow_merge_namespace(C_PetJournal, {
  IsUsingDefaultFilters = function() return true end,
  GetSummonBattlePetCooldown = function() return 0, 0, false end,
})

C_SpecializationInfo = __wow_merge_namespace(C_SpecializationInfo, {
  GetInspectSelectedPvpTalent = function() return nil end,
})

C_PerksActivities = __wow_merge_namespace(C_PerksActivities, {
  AddTrackedPerksActivity = function(_id) end,
  ClearPerksActivitiesPendingCompletion = function() end,
  GetAllPerksActivityTags = function()
    return { tagName = {} }
  end,
  GetPerksActivitiesInfo = function()
    return {
      activePerksMonth = 0,
      displayMonthName = "",
      secondsRemaining = 0,
      activities = {},
      thresholds = {},
    }
  end,
  GetPerksActivitiesPendingCompletion = function()
    return { pendingIDs = {} }
  end,
  GetPerksActivityChatLink = function(_id)
    return ""
  end,
  GetPerksActivityInfo = function(_id)
    return nil
  end,
  GetPerksUIThemePrefix = function()
    return ""
  end,
  GetTrackedPerksActivities = function()
    return { trackedIDs = {} }
  end,
  RemoveTrackedPerksActivity = function(_id) end,
})

if CreateTemplateInfoCache == nil then
  function CreateTemplateInfoCache()
    local cache = {
      templateInfos = {},
      infoAddedCallback = nil,
    }

    function cache:Init()
    end

    function cache:SetInfoAddedCallback(callback)
      self.infoAddedCallback = callback
    end

    function cache:FlushTemplateInfos()
      self.templateInfos = {}
    end

    function cache:GetTemplateInfo(frameTemplate)
      local info = self.templateInfos[frameTemplate]
      if info == nil and C_XMLUtil and C_XMLUtil.GetTemplateInfo then
        info = C_XMLUtil.GetTemplateInfo(frameTemplate)
        self.templateInfos[frameTemplate] = info
      end
      if info ~= nil and self.infoAddedCallback then
        self.infoAddedCallback(info)
      end
      return info
    end

    function cache:GetTemplateInfos()
      return self.templateInfos
    end

    cache:Init()
    return cache
  end
end

EVERY_X_PERCENT = EVERY_X_PERCENT or "%d%%"
TRANSMOGRIFY_TOOLTIP_APPEARANCE_KNOWN = TRANSMOGRIFY_TOOLTIP_APPEARANCE_KNOWN or "Known"
ERR_QUEST_SESSION_RESULT_RESYNC = ERR_QUEST_SESSION_RESULT_RESYNC or "Resync"
CLASS_SORT_ORDER = CLASS_SORT_ORDER or { "WARRIOR", "PALADIN", "HUNTER", "ROGUE", "PRIEST", "DEATHKNIGHT", "SHAMAN", "MAGE", "WARLOCK", "MONK", "DRUID", "DEMONHUNTER", "EVOKER" }
EVENT_TOAST_MANAGER_OFFSET_Y_OVERRIDE = false
CLOCK_TICKER_Y_OVERRIDE = CLOCK_TICKER_Y_OVERRIDE or false

local __global_mt = getmetatable(_G) or {}
local __prev_index = __global_mt.__index
local function __wow_is_color_constant_key(key)
  if type(key) ~= "string" then
    return false
  end
  if key:match("_COLOR$") then
    return true
  end
  if not key:match("_COLOR_[A-Z0-9_]+$") then
    return false
  end
  return not key:match("_COLOR_CODE")
     and not key:match("_COLOR_TABLE")
     and not key:match("_COLOR_ATLASES")
end
local function __wow_preserve_nil_global(key)
  if type(key) ~= "string" then
    return false
  end
  return key:match("^SLASH_[A-Z0-9_]+%d+$") ~= nil
      or key:match("^EMOTE%d+_CMD%d+$") ~= nil
      or key:match("^EMOTE%d+_TOKEN$") ~= nil
end
__global_mt.__index = function(t, key)
  local value = nil
  if __prev_index ~= nil then
    if type(__prev_index) == "function" then
      value = __prev_index(t, key)
    else
      value = __prev_index[key]
    end
  end
  if value ~= nil then
    return value
  end
  if __wow_preserve_nil_global(key) then
    return nil
  end

  if key == "HIGHLIGHT_FONT_COLOR" then
    value = __wow_make_color(1, 1, 1, 1)
  elseif __wow_is_color_constant_key(key) then
    value = __wow_make_color(1, 1, 1, 1)
  elseif key == "PLAYER_FACTION_COLOR_HORDE" then
    value = __wow_make_color(1, 0.1, 0.1, 1)
  elseif key == "PLAYER_FACTION_COLOR_ALLIANCE" then
    value = __wow_make_color(0.2, 0.4, 1, 1)
  elseif type(key) == "string" and key:match("^C_[A-Za-z0-9_]+$") then
    value = __wow_namespace()
  elseif type(key) == "string" and key:match("^ERR_") then
    value = key
  end

  if value ~= nil then
    rawset(t, key, value)
    return value
  end
  return nil
end
setmetatable(_G, __global_mt)
