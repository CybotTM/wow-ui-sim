//! Initialization helpers for the WoW Lua environment.
//!
//! Standalone functions extracted from `env.rs` that are called during
//! `WowLuaEnv::new()` and from event/script dispatch paths.

use super::builtin_frames::create_builtin_frames;
use super::state::{AddonRuntimeMetrics, SimState};
use crate::lua_api::frame::methods::{
    rilua_button_anchor_hierarchy, rilua_core_state, rilua_misc, rilua_text_attribute_event,
    rilua_widgets,
};
use crate::lua_api::globals::enum_data::{EXPLICIT_ENUMS, SEQUENTIAL_ENUMS};
use crate::lua_api::rilua_methods::{
    create_table, registry_set, registry_table_or_create, table_get, table_set,
};
use rilua::LuaApiMut;
use rilua::Val;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

const MISSING_ENUMS_LUA: &str = include_str!("globals/enum_data/missing_enums.lua");
const COMPAT_ENUMS_LUA: &str = include_str!("globals/enum_data/compat_enums.lua");
const MISSING_CONSTANTS_LUA: &str = include_str!("globals/enum_data/missing_constants.lua");
const CONSTANTS_VALUES_LUA: &str = include_str!("globals/enum_data/constants_values.lua");
const SHARED_BOOTSTRAP_LUA: &str = r##"
if Mixin == nil then
  function Mixin(object, ...)
    for i = 1, select("#", ...) do
      local mixin = select(i, ...)
      if type(mixin) == "table" then
        for k, v in pairs(mixin) do
          object[k] = v
        end
      end
    end
    return object
  end
end

if CreateFromMixins == nil then
  function CreateFromMixins(...)
    return Mixin({}, ...)
  end
end

if CreateAndInitFromMixin == nil then
  function CreateAndInitFromMixin(mixin, ...)
    local object = CreateFromMixins(mixin)
    if object.Init then
      object:Init(...)
    end
    return object
  end
end

table = table or {}

if unpack == nil then
  function unpack(list, first, last)
    if type(list) ~= "table" then
      return nil
    end
    first = first or 1
    last = last or #list
    if first > last then
      return
    end
    return list[first], unpack(list, first + 1, last)
  end
end

if table.unpack == nil then
  table.unpack = unpack
end

if GetCurrentEnvironment == nil then
  function GetCurrentEnvironment()
    return _G
  end
end

if SwapToGlobalEnvironment == nil then
  function SwapToGlobalEnvironment()
    return _G
  end
end

if CreateSecureDelegate == nil then
  function CreateSecureDelegate(fn)
    return fn
  end
end

if GetFrameMetatable == nil then
  function GetFrameMetatable(frame)
    if frame == nil then
      if CreateFrame == nil then
        return nil
      end
      frame = CreateFrame("Frame")
    end
    return frame and getmetatable(frame) or nil
  end
end

if C_Glue == nil then
  C_Glue = {}
end

if C_Glue.IsOnGlueScreen == nil then
  function C_Glue.IsOnGlueScreen()
    return false
  end
end
"##;

const RUNTIME_SURFACE_BOOTSTRAP_LUA: &str = r##"
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

if GetGameTime == nil then
  function GetGameTime()
    return 12, 0
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

-- `GetInventorySlotInfo(slotName)` — canonical WoW slot id + icon name.
-- Callsites use the numeric return as a TABLE KEY (e.g.
-- `CANCELABLE_ITEMS[GetInventorySlotInfo("MainHandSlot")] = 1`), so a
-- nil return crashes the chunk with "table index is nil". Provide the
-- stable mapping Blizzard has shipped for years; texture name and
-- `checkRelic` aren't used by sim callers yet, returned as placeholders.
if GetInventorySlotInfo == nil then
  local __wow_inventory_slots = {
    HEADSLOT=1, NECKSLOT=2, SHOULDERSLOT=3, SHIRTSLOT=4, CHESTSLOT=5,
    WAISTSLOT=6, LEGSSLOT=7, FEETSLOT=8, WRISTSLOT=9, HANDSSLOT=10,
    FINGER0SLOT=11, FINGER1SLOT=12, TRINKET0SLOT=13, TRINKET1SLOT=14,
    BACKSLOT=15, MAINHANDSLOT=16, SECONDARYHANDSLOT=17, RANGEDSLOT=18,
    TABARDSLOT=19, BAG0SLOT=20,
  }
  function GetInventorySlotInfo(slot_name)
    if type(slot_name) ~= "string" then
      return nil
    end
    local id = __wow_inventory_slots[slot_name:upper()]
    if id == nil then
      return nil
    end
    return id, "Interface\\PaperDoll\\UI-PaperDoll-Slot-" .. slot_name, false
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

C_PetBattles = __wow_merge_namespace(C_PetBattles, {
  GetAllEffectNames = function() return end,
})

C_VoiceChat = __wow_merge_namespace(C_VoiceChat, {
  GetTtsVoices = function() return {} end,
  IsTranscriptionAllowed = function() return false end,
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
SecureTypes.CreateSecureMap = SecureTypes.CreateSecureMap or function() return {} end
SecureTypes.CreateSecureFunction = SecureTypes.CreateSecureFunction or function(fn) return fn end
SecureTypes.CreateSecureNumber = SecureTypes.CreateSecureNumber or function(value) return value or 0 end
SecureTypes.CreateSecureArray = SecureTypes.CreateSecureArray or function()
  local array = {}
  function array:Insert(value)
    self[#self + 1] = value
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
if GetAvailableLocaleInfo == nil then
  function GetAvailableLocaleInfo()
    return {}
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
C_ContentTracking = C_ContentTracking or __wow_namespace()
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
})

C_QuestLog = __wow_merge_namespace(C_QuestLog, {
  ReadyForTurnIn = function()
    return false
  end,
})

C_ColorOverrides = __wow_merge_namespace(C_ColorOverrides, {
  GetColorForQuality = function()
    return CreateColor(1, 1, 1)
  end,
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

local __global_mt = getmetatable(_G) or {}
local __prev_index = __global_mt.__index
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

  if key == "HIGHLIGHT_FONT_COLOR" then
    value = __wow_make_color(1, 1, 1, 1)
  elseif key == "PLAYER_FACTION_COLOR_HORDE" then
    value = __wow_make_color(1, 0.1, 0.1, 1)
  elseif key == "PLAYER_FACTION_COLOR_ALLIANCE" then
    value = __wow_make_color(0.2, 0.4, 1, 1)
  elseif type(key) == "string" and key:match("^ERR_") then
    value = key
  elseif type(key) == "string" and key:match("^[A-Z0-9_]+$") then
    value = key
  end

  if value ~= nil then
    rawset(t, key, value)
    return value
  end
  return nil
end
setmetatable(_G, __global_mt)
"##;

/// Increment threshold counters for a frame's addon time.
pub(super) fn update_threshold_counters(rt: &mut AddonRuntimeMetrics, ms: f64) {
    if ms > 1.0 {
        rt.count_over_1ms += 1;
    }
    if ms > 5.0 {
        rt.count_over_5ms += 1;
    }
    if ms > 10.0 {
        rt.count_over_10ms += 1;
    }
    if ms > 50.0 {
        rt.count_over_50ms += 1;
    }
    if ms > 100.0 {
        rt.count_over_100ms += 1;
    }
    if ms > 500.0 {
        rt.count_over_500ms += 1;
    }
    if ms > 1000.0 {
        rt.count_over_1000ms += 1;
    }
}

/// Stamp addon taint on a handler and call it. The VM applies fixedtaint on entry.
/// For Blizzard addons (is_blizzard=true), clear the handler's taint so issecure()
/// returns true during execution, matching real WoW behavior.
pub(super) fn call_with_taint<L, H, A>(
    _lua: &L,
    _handler: H,
    _taint: Option<String>,
    _is_blizzard: bool,
    _args: A,
) -> crate::Result<()> {
    Ok(())
}

/// Look up the addon folder name for a given owner_addon index.
pub(super) fn addon_taint_name(state: &Rc<RefCell<SimState>>, idx: Option<u16>) -> Option<String> {
    idx.and_then(|i| {
        state
            .borrow()
            .addons
            .get(i as usize)
            .map(|a| a.folder_name.clone())
    })
}

/// Check whether an addon index refers to a Blizzard addon (runs secure).
pub(super) fn is_blizzard_addon(state: &Rc<RefCell<SimState>>, idx: Option<u16>) -> bool {
    idx.map(|i| {
        state
            .borrow()
            .addons
            .get(i as usize)
            .is_some_and(|a| a.folder_name.starts_with("Blizzard_"))
    })
    .unwrap_or(true)
}

/// Record per-addon timing from an Instant.
pub(super) fn record_addon_time(state: &Rc<RefCell<SimState>>, idx: Option<u16>, start: &Instant) {
    if let Some(i) = idx {
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        if let Some(addon) = state.borrow_mut().addons.get_mut(i as usize) {
            addon.runtime.current_frame_ms += ms;
        }
    }
}

/// Create built-in frames in the widget registry before Lua loads.
/// Registers a `__BuiltIn` pseudo-addon as their owner.
pub(super) fn init_builtin_frames(state: &Rc<RefCell<SimState>>) {
    let mut s = state.borrow_mut();
    let owner = s.addons.len() as u16;
    s.addons.push(super::AddonInfo {
        folder_name: "__BuiltIn".to_string(),
        title: "Built-in Frames".to_string(),
        enabled: true,
        loaded: true,
        ..Default::default()
    });
    let (w, h) = (s.screen_width, s.screen_height);
    create_builtin_frames(&mut s.widgets, w, h, owner);
}

/// Initialize the primary rilua state: seed registries, globals, frame methods, and taint.
pub(super) fn init_lua_state(
    lua: &mut rilua::Lua,
    state: Rc<RefCell<SimState>>,
) -> crate::Result<()> {
    init_registry_tables(lua, &state)?;
    init_shared_bootstrap(lua)?;
    init_enum_globals(lua)?;
    init_frame_metatable(lua)?;
    super::globals::register_globals(lua, state.clone())?;
    init_runtime_surface_bootstrap(lua)?;
    super::globals::rilua_security::create_secure_environment(lua)?;
    enable_taint_and_wrap_loadstring(lua)?;
    crate::loader::precompiled::init(lua)?;
    remove_sandbox_globals(lua)?;
    init_frame_metatable(lua)?;
    Ok(())
}

fn init_enum_globals(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let enum_table = ensure_global_table(state, "Enum");
    for &(enum_name, entries) in EXPLICIT_ENUMS.iter() {
        let enum_values = create_table(state);
        for &(variant_name, value) in entries {
            table_set(state, enum_values, variant_name, Val::Num(value as f64));
        }
        table_set(state, enum_table, enum_name, enum_values);
    }
    for &(enum_name, entries) in SEQUENTIAL_ENUMS.iter() {
        let enum_values = create_table(state);
        for (index, &variant_name) in entries.iter().enumerate() {
            table_set(state, enum_values, variant_name, Val::Num(index as f64));
        }
        table_set(state, enum_table, enum_name, enum_values);
    }
    lua.exec(MISSING_ENUMS_LUA)?;
    lua.exec(COMPAT_ENUMS_LUA)?;
    lua.exec(
        r#"
        Constants = Constants or {}
        setmetatable(Constants, {
            __index = function(t, key)
                local value = {}
                rawset(t, key, value)
                return value
            end,
        })
        "#,
    )?;
    lua.exec(MISSING_CONSTANTS_LUA)?;
    lua.exec(CONSTANTS_VALUES_LUA)?;
    Ok(())
}

pub(super) fn init_shared_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(SHARED_BOOTSTRAP_LUA)?;
    Ok(())
}

pub(super) fn init_runtime_surface_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(RUNTIME_SURFACE_BOOTSTRAP_LUA)?;
    Ok(())
}

fn ensure_global_table(state: &mut rilua::vm::state::LuaState, key: &str) -> Val {
    let global = Val::Table(state.global);
    let existing = table_get(state, global, key);
    if matches!(existing, Val::Table(_)) {
        return existing;
    }
    let table = create_table(state);
    table_set(state, global, key, table);
    table
}

/// Set up registry tables for event dispatch and taint fallback.
fn init_registry_tables(lua: &mut rilua::Lua, state: &Rc<RefCell<SimState>>) -> crate::Result<()> {
    let lua_state = lua.state_mut();
    let _ = state;
    let _ = registry_table_or_create(lua_state, "__addon_names");
    let _ = registry_table_or_create(lua_state, "__addon_timing");
    let _ = registry_table_or_create(lua_state, "__event_individual");
    let _ = registry_table_or_create(lua_state, "__event_all");
    let _ = registry_table_or_create(lua_state, "__scripts");
    let _ = registry_table_or_create(lua_state, "__on_update_scripts");
    let _ = registry_table_or_create(lua_state, "__on_post_update_scripts");
    let _ = registry_table_or_create(lua_state, "__rilua_frame_fields");
    super::on_update::register(lua_state, state)
}

/// Enable Elune taint tracking and wrap loadstring as secure.
fn enable_taint_and_wrap_loadstring(lua: &mut rilua::Lua) -> crate::Result<()> {
    super::rilua_taint::enable_taint_mode(lua);
    Ok(())
}

/// Remove globals that WoW's sandbox doesn't expose and internal helpers
/// now stored in the Lua registry.
fn remove_sandbox_globals(_lua: &mut rilua::Lua) -> crate::Result<()> {
    Ok(())
}

fn init_frame_metatable(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let frame_mt = Val::Table(state.gc.alloc_table(rilua::vm::table::Table::new()));
    table_set(state, frame_mt, "__index", frame_mt);
    registry_set(state, "__rilua_frame_mt", frame_mt);

    let Val::Table(frame_mt_ref) = frame_mt else {
        unreachable!("frame metatable must be a table");
    };
    super::rilua_timer_layout::register_layout_fns_on_table(state, frame_mt_ref)?;
    rilua_core_state::register_all(state, frame_mt_ref)?;
    rilua_misc::register_all(state, frame_mt_ref)?;
    rilua_text_attribute_event::register_all(state, frame_mt_ref)?;
    rilua_button_anchor_hierarchy::register_all(state, frame_mt_ref)?;
    rilua_widgets::register_all(state, frame_mt_ref)?;

    // Replace the self-referencing `__index` with a shallow clone that omits
    // metamethod keys. Blizzard's restricted code does
    // `CopyTable(GetFrameMetatable().__index)` (RestrictedExecution.lua), which
    // would infinitely recurse if `__index` pointed back at the metatable
    // itself. The clone captures the methods registered above and stays
    // stable for the lifetime of the VM (method registration only happens
    // here at init).
    let frame_index = build_frame_index_table(state, frame_mt_ref);
    table_set(state, frame_mt, "__index", Val::Table(frame_index));
    Ok(())
}

/// Build a shallow, non-cyclic clone of the frame metatable's method entries.
///
/// Skips keys that start with `__` (metamethods) so the resulting table only
/// exposes frame methods — matching what Blizzard's restricted loader expects
/// from `GetFrameMetatable().__index`.
fn build_frame_index_table(
    state: &mut rilua::vm::state::LuaState,
    frame_mt_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> rilua::vm::gc::arena::GcRef<rilua::vm::table::Table> {
    let new_ref = state.gc.alloc_table(rilua::vm::table::Table::new());
    let entries = state
        .gc
        .tables
        .get(frame_mt_ref)
        .map(|table| table.hash_entries())
        .unwrap_or_default();
    for (key, value) in entries {
        if let Val::Str(str_ref) = key
            && let Some(name) = state.gc.string_arena.get(str_ref)
            && name.data().starts_with(b"__")
        {
            continue;
        }
        if let Some(t) = state.gc.tables.get_mut(new_ref) {
            let _ = t.raw_set(key, value, &state.gc.string_arena);
        }
    }
    new_ref
}
