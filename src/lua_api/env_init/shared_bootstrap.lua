if addframetext == nil then
  function addframetext() end
end


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

if UI_LOCALE == nil then
  if type(GetLocale) == "function" then
    UI_LOCALE = GetLocale()
  else
    UI_LOCALE = "enUS"
  end
end

if SetupLocalization == nil then
  local localizeFramesCallbacks = {}

  local function call_localization_function(l10nTable, key)
    if type(l10nTable) ~= "table" then
      return
    end
    local localeTable = l10nTable[UI_LOCALE]
    if type(localeTable) ~= "table" then
      return
    end
    local localizeFn = localeTable[key]
    if type(localizeFn) == "function" then
      localizeFn()
    end
  end

  function LocalizeFrames()
    local callbacks = localizeFramesCallbacks
    localizeFramesCallbacks = {}
    for index = 1, #callbacks do
      local ok = pcall(callbacks[index])
      local _ = ok
    end
  end

  function SetupLocalization(l10nTable)
    call_localization_function(l10nTable, "localize")
    table.insert(localizeFramesCallbacks, function()
      call_localization_function(l10nTable, "localizeFrames")
    end)
  end
end

if EventUtil == nil then
  local eventUtilState = {
    allEventsWatchers = {},
    onceWatchers = {},
    registeredEvents = {},
    seenEvents = {},
    variablesLoadedCallbacks = {},
    variablesLoadedTriggered = false,
  }

  local dispatcher = nil

  local function ensure_dispatcher()
    if dispatcher == nil and type(CreateFrame) == "function" then
      dispatcher = CreateFrame("Frame")
      dispatcher:SetScript("OnEvent", function(_, event, ...)
        eventUtilState.seenEvents[event] = true

        if event == "VARIABLES_LOADED" and not eventUtilState.variablesLoadedTriggered then
          eventUtilState.variablesLoadedTriggered = true
          if type(UIParent) == "table" then
            UIParent.variablesLoaded = true
          end
          local callbacks = eventUtilState.variablesLoadedCallbacks
          eventUtilState.variablesLoadedCallbacks = {}
          for i = 1, #callbacks do
            local callback = callbacks[i]
            if type(callback) == "function" then
              callback()
            end
          end
        end

        for index = #eventUtilState.onceWatchers, 1, -1 do
          local watcher = eventUtilState.onceWatchers[index]
          if watcher.event == event then
            local matches = true
            for requiredIndex = 1, watcher.requiredArgs.n do
              if select(requiredIndex, ...) ~= watcher.requiredArgs[requiredIndex] then
                matches = false
                break
              end
            end
            if matches then
              watcher.handle.registered = false
              table.remove(eventUtilState.onceWatchers, index)
              if type(watcher.callback) == "function" then
                watcher.callback(...)
              end
            end
          end
        end

        for index = #eventUtilState.allEventsWatchers, 1, -1 do
          local watcher = eventUtilState.allEventsWatchers[index]
          local haveAllEvents = true
          for eventIndex = 1, #watcher.events do
            if not eventUtilState.seenEvents[watcher.events[eventIndex]] then
              haveAllEvents = false
              break
            end
          end
          if haveAllEvents then
            table.remove(eventUtilState.allEventsWatchers, index)
            if type(watcher.callback) == "function" then
              watcher.callback()
            end
          end
        end
      end)
    end

    return dispatcher
  end

  local function ensure_event_registration(event)
    local frame = ensure_dispatcher()
    if frame == nil or eventUtilState.registeredEvents[event] then
      return
    end
    eventUtilState.registeredEvents[event] = true
    frame:RegisterEvent(event)
  end

  local function all_events_seen(events)
    for index = 1, #events do
      if not eventUtilState.seenEvents[events[index]] then
        return false
      end
    end
    return true
  end

  local function remove_once_watcher(handle)
    for index = #eventUtilState.onceWatchers, 1, -1 do
      if eventUtilState.onceWatchers[index].handle == handle then
        table.remove(eventUtilState.onceWatchers, index)
        break
      end
    end
  end

  EventUtil = {}

  function EventUtil.ContinueAfterAllEvents(callback, ...)
    local events = {}
    for index = 1, select("#", ...) do
      local event = select(index, ...)
      events[index] = event
      ensure_event_registration(event)
    end

    if all_events_seen(events) then
      if type(callback) == "function" then
        callback()
      end
      return
    end

    table.insert(eventUtilState.allEventsWatchers, {
      callback = callback,
      events = events,
    })
  end

  function EventUtil.AreVariablesLoaded()
    return eventUtilState.variablesLoadedTriggered
      or GlueParent
      or (type(UIParent) == "table" and UIParent.variablesLoaded)
  end

  function EventUtil.ContinueOnVariablesLoaded(callback)
    if EventUtil.AreVariablesLoaded() then
      if type(callback) == "function" then
        callback()
      end
      return
    end

    ensure_event_registration("VARIABLES_LOADED")
    table.insert(eventUtilState.variablesLoadedCallbacks, callback)
  end

  function EventUtil.TriggerOnVariablesLoaded()
    local frame = ensure_dispatcher()
    if frame ~= nil then
      frame:GetScript("OnEvent")(frame, "VARIABLES_LOADED")
    end
  end

  function EventUtil.ContinueOnAddOnLoaded(addOnName, callback)
    local isLoadedOrLoading, isLoaded = C_AddOns.IsAddOnLoaded(addOnName)
    if isLoaded then
      if type(callback) == "function" then
        callback()
      end
      return
    end

    EventUtil.RegisterOnceFrameEventAndCallback("ADDON_LOADED", callback, addOnName)
  end

  function EventUtil.ContinueOnPlayerLogin(callback)
    if IsLoggedIn() then
      if type(callback) == "function" then
        callback()
      end
      return
    end

    EventUtil.RegisterOnceFrameEventAndCallback("PLAYER_LOGIN", callback)
  end

  function EventUtil.RegisterOnceFrameEventAndCallback(frameEvent, callback, ...)
    ensure_event_registration(frameEvent)

    local handle = { registered = true }
    function handle:Unregister()
      if not self.registered then
        return
      end
      self.registered = false
      remove_once_watcher(self)
    end

    table.insert(eventUtilState.onceWatchers, {
      callback = callback,
      event = frameEvent,
      handle = handle,
      requiredArgs = __wow_pack_results(...),
    })

    return handle
  end

  CallbackHandleContainerMixin = CallbackHandleContainerMixin or {}

  function CallbackHandleContainerMixin:Init()
    self.handles = {}
  end

  function CallbackHandleContainerMixin:RegisterCallback(cbr, event, callback, owner)
    self:AddHandle(cbr:RegisterCallbackWithHandle(event, callback, owner))
  end

  function CallbackHandleContainerMixin:AddHandle(handle)
    table.insert(self.handles, handle)
  end

  function CallbackHandleContainerMixin:Unregister()
    for index = 1, #self.handles do
      local handle = self.handles[index]
      if handle and type(handle.Unregister) == "function" then
        handle:Unregister()
      end
    end
    self.handles = {}
  end

  function CallbackHandleContainerMixin:IsEmpty()
    return #self.handles == 0
  end

  function EventUtil.CreateCallbackHandleContainer()
    local cbrHandles = CreateFromMixins(CallbackHandleContainerMixin)
    cbrHandles:Init()
    return cbrHandles
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

format = format or string.format

SlashCmdList = SlashCmdList or {}

if table.wipe == nil then
  function table.wipe(tbl)
    if type(tbl) ~= "table" then
      return tbl
    end
    for key in pairs(tbl) do
      tbl[key] = nil
    end
    return tbl
  end
end

tWipe = tWipe or table.wipe

local function __wow_pack_results(...)
  return { n = select("#", ...), ... }
end

function hooksecurefunc(target, methodName, hook)
  local object = target
  local key = methodName
  local callback = hook

  if type(target) == "string" and type(methodName) == "function" and hook == nil then
    object = _G
    key = target
    callback = methodName
  end

  if type(object) ~= "table" or type(key) ~= "string" or type(callback) ~= "function" then
    return
  end

  local original = object[key]
  if type(original) ~= "function" then
    return
  end

  object[key] = function(...)
    local results = __wow_pack_results(original(...))
    callback(...)
    return unpack(results, 1, results.n)
  end
end

if getn == nil then
  function getn(tbl)
    if type(tbl) ~= "table" then
      return nil
    end
    return #tbl
  end
end

if table.getn == nil then
  table.getn = getn
end

if strtrim == nil then
  function strtrim(value)
    value = tostring(value or "")
    return (value:gsub("^%s+", ""):gsub("%s+$", ""))
  end
end

if Clamp == nil then
  function Clamp(value, min, max)
    if value > max then
      return max
    elseif value < min then
      return min
    end
    return value
  end
end

if Saturate == nil then
  function Saturate(value)
    return Clamp(value, 0.0, 1.0)
  end
end

if CooldownFrame_Set == nil then
  function CooldownFrame_Set(self, start, duration, enable, forceShowDrawEdge, modRate)
    if enable and enable ~= 0 and start > 0 and duration > 0 then
      self:SetDrawEdge(forceShowDrawEdge)
      self:SetCooldown(start, duration, modRate)
    else
      CooldownFrame_Clear(self)
    end
  end
end

if CooldownFrame_Clear == nil then
  function CooldownFrame_Clear(self)
    self:Clear()
    self:SetCooldown(0, 0)
  end
end

if CooldownFrame_SetDisplayAsPercentage == nil then
  function CooldownFrame_SetDisplayAsPercentage(self, percentage)
    local seconds = 100
    self:Pause()
    self:SetCooldown(GetTime() - (seconds * Saturate(percentage)), seconds)
  end
end

if FadingFrame_SetFadeInTime == nil then
  function FadingFrame_SetFadeInTime(fadingFrame, time)
    fadingFrame.fadeInTime = time
  end
end

if FadingFrame_SetHoldTime == nil then
  function FadingFrame_SetHoldTime(fadingFrame, time)
    fadingFrame.holdTime = time
  end
end

if FadingFrame_SetFadeOutTime == nil then
  function FadingFrame_SetFadeOutTime(fadingFrame, time)
    fadingFrame.fadeOutTime = time
  end
end

if FadingFrame_OnLoad == nil then
  function FadingFrame_OnLoad(fadingFrame)
    assert(fadingFrame)
    fadingFrame.fadeInTime = 0
    fadingFrame.holdTime = 0
    fadingFrame.fadeOutTime = 0
    fadingFrame:Hide()
  end
end

if FadingFrame_Show == nil then
  function FadingFrame_Show(fadingFrame)
    assert(fadingFrame)
    fadingFrame.startTime = GetTime()
    fadingFrame:Show()
  end
end

if FadingFrame_OnUpdate == nil then
  function FadingFrame_OnUpdate(fadingFrame)
    assert(fadingFrame)
    local elapsed = GetTime() - fadingFrame.startTime
    local fadeInTime = fadingFrame.fadeInTime
    if elapsed < fadeInTime then
      fadingFrame:SetAlpha(elapsed / fadeInTime)
      return
    end

    local holdTime = fadingFrame.holdTime
    if elapsed < (fadeInTime + holdTime) then
      fadingFrame:SetAlpha(1.0)
      return
    end

    local fadeOutTime = fadingFrame.fadeOutTime
    if elapsed < (fadeInTime + holdTime + fadeOutTime) then
      fadingFrame:SetAlpha(1.0 - ((elapsed - holdTime - fadeInTime) / fadeOutTime))
      return
    end

    fadingFrame:Hide()
  end
end

if FadingFrame_GetRemainingTime == nil then
  function FadingFrame_GetRemainingTime(fadingFrame)
    local elapsed = GetTime() - fadingFrame.startTime
    return fadingFrame.holdTime + fadingFrame.fadeInTime + fadingFrame.fadeOutTime - elapsed
  end
end

if FadingFrame_CopyTimes == nil then
  function FadingFrame_CopyTimes(src, dest)
    dest.fadeInTime = src.fadeInTime
    dest.holdTime = src.holdTime
    dest.fadeOutTime = src.fadeOutTime
    dest.startTime = src.startTime
  end
end

local function __wow_deep_copy_table(source, seen)
  if type(source) ~= "table" then
    return source
  end
  seen = seen or {}
  if seen[source] ~= nil then
    return seen[source]
  end
  local copy = {}
  seen[source] = copy
  for key, value in pairs(source) do
    copy[__wow_deep_copy_table(key, seen)] = __wow_deep_copy_table(value, seen)
  end
  local mt = getmetatable(source)
  if mt ~= nil then
    setmetatable(copy, __wow_deep_copy_table(mt, seen))
  end
  return copy
end

if CopyTable == nil then
  function CopyTable(source)
    return __wow_deep_copy_table(source)
  end
end

if TextureKitConstants == nil then
  TextureKitConstants = {
    SetVisibility = true,
    DoNotSetVisibility = false,
    UseAtlasSize = true,
    IgnoreAtlasSize = false,
    AddressModeClamp = 1,
    AddressModeWrap = 2,
    AddressModeAllowAssetToDetermine = 3,
  }
end

local __wow_lfg_role_icons = {
  GUIDE = "UI-LFG-RoleIcon-Leader",
  TANK = "UI-LFG-RoleIcon-Tank",
  HEALER = "UI-LFG-RoleIcon-Healer",
  DAMAGER = "UI-LFG-RoleIcon-DPS",
  NONE = "UI-LFG-RoleIcon-DPS",
}

local __wow_lfg_role_icons_disabled = {
  GUIDE = "UI-LFG-RoleIcon-Leader-Disabled",
  TANK = "UI-LFG-RoleIcon-Tank-Disabled",
  HEALER = "UI-LFG-RoleIcon-Healer-Disabled",
  DAMAGER = "UI-LFG-RoleIcon-DPS-Disabled",
  NONE = "UI-LFG-RoleIcon-DPS-Disabled",
}

if GetIconForRole == nil then
  function GetIconForRole(role, showDisabled)
    local iconSet = showDisabled and __wow_lfg_role_icons_disabled or __wow_lfg_role_icons
    return iconSet[role] or iconSet.NONE
  end
end

local function __wow_lfg_role_name_from_enum(role)
  if role == 0 then
    return "TANK"
  end
  if role == 1 then
    return "HEALER"
  end
  if role == 2 then
    return "DAMAGER"
  end
  if Constants ~= nil
      and Constants.LFG_ROLEConstants ~= nil
      and role == Constants.LFG_ROLEConstants.LFG_ROLE_NO_ROLE then
    return "GUIDE"
  end
  return "NONE"
end

if GetIconForRoleEnum == nil then
  function GetIconForRoleEnum(role, showDisabled)
    return GetIconForRole(__wow_lfg_role_name_from_enum(role), showDisabled)
  end
end

if C_Map == nil then
  C_Map = {}
end

function C_Map.GetBestMapForUnit(unitToken)
  if unitToken ~= nil and unitToken ~= "player" then
    return nil
  end
  if C_Map.GetCurrentMapID ~= nil then
    local currentMapID = C_Map.GetCurrentMapID()
    if currentMapID ~= nil then
      return currentMapID
    end
  end
  return 2248
end

function C_Map.GetFallbackWorldMapID()
  if C_Map.GetCurrentMapID ~= nil then
    local currentMapID = C_Map.GetCurrentMapID()
    if currentMapID ~= nil then
      return currentMapID
    end
  end
  return 2248
end

function C_Map.MapHasArt(mapID)
  if mapID == nil then
    return false
  end
  if C_Map.GetMapArtID ~= nil then
    local artID = C_Map.GetMapArtID(mapID)
    if artID ~= nil then
      return artID ~= 0
    end
  end
  return true
end

if MapUtil == nil then
  MapUtil = {}
end

local __wow_maputil_child_map_cache = {}

local function __wow_maputil_safe_map_info(mapID)
  if C_Map == nil or C_Map.GetMapInfo == nil or mapID == nil then
    return nil
  end
  return C_Map.GetMapInfo(mapID)
end

local function __wow_maputil_safe_map_has_art(mapID)
  if C_Map == nil then
    return false
  end
  if C_Map.MapHasArt ~= nil then
    local ok, hasArt = pcall(C_Map.MapHasArt, mapID)
    if ok and hasArt ~= nil then
      return hasArt
    end
  end
  if C_Map.GetMapArtID ~= nil then
    local ok, artID = pcall(C_Map.GetMapArtID, mapID)
    if ok and artID ~= nil then
      return artID ~= 0
    end
  end
  return mapID ~= nil
end

if MapUtil.IsMapTypeZone == nil then
  function MapUtil.IsMapTypeZone(mapID)
    local mapInfo = __wow_maputil_safe_map_info(mapID)
    return mapInfo ~= nil and mapInfo.mapType == Enum.UIMapType.Zone
  end
end

if MapUtil.GetMapParentInfo == nil then
  function MapUtil.GetMapParentInfo(mapID, mapType, topMost)
    local candidate = nil
    local mapInfo = __wow_maputil_safe_map_info(mapID)
    while mapInfo do
      if mapInfo.mapType == mapType then
        if topMost then
          candidate = mapInfo
        else
          return mapInfo
        end
      end
      mapInfo = __wow_maputil_safe_map_info(mapInfo.parentMapID)
    end
    return candidate
  end
end

if MapUtil.ShouldMapTypeShowQuests == nil then
  function MapUtil.ShouldMapTypeShowQuests(mapType)
    return mapType ~= Enum.UIMapType.World
      and mapType ~= Enum.UIMapType.Continent
      and mapType ~= Enum.UIMapType.Cosmic
  end
end

if MapUtil.GetDisplayableMapForPlayer == nil then
  function MapUtil.GetDisplayableMapForPlayer()
    if C_Map == nil then
      return 1
    end
    local mapID = C_Map and C_Map.GetBestMapForUnit and C_Map.GetBestMapForUnit("player")
    if mapID == nil then
      if C_Map.GetFallbackWorldMapID then
        mapID = C_Map.GetFallbackWorldMapID()
      elseif C_Map.GetCurrentMapID then
        mapID = C_Map.GetCurrentMapID()
      else
        return 1
      end
    end
    while mapID and mapID ~= 0 do
      if __wow_maputil_safe_map_has_art(mapID) then
        return mapID
      end
      local mapInfo = __wow_maputil_safe_map_info(mapID)
      mapID = mapInfo and mapInfo.parentMapID or 0
    end
    if C_Map and C_Map.GetFallbackWorldMapID then
      local fallbackMapID = C_Map.GetFallbackWorldMapID()
      if fallbackMapID ~= nil then
        return fallbackMapID
      end
    end
    if C_Map and C_Map.GetCurrentMapID then
      local currentMapID = C_Map.GetCurrentMapID()
      if currentMapID ~= nil then
        return currentMapID
      end
    end
    return 1
  end
end

if MapUtil.GetMapCenterOnMap == nil then
  function MapUtil.GetMapCenterOnMap(mapID, topMapID)
    if C_Map == nil or C_Map.GetMapRectOnMap == nil then
      return nil, nil
    end
    local left, right, top, bottom = C_Map.GetMapRectOnMap(mapID, topMapID)
    if left == nil then
      return nil, nil
    end
    return left + (right - left) * 0.5, top + (bottom - top) * 0.5
  end
end

if MapUtil.IsChildMap == nil then
  function MapUtil.IsChildMap(mapID, ancestorMapID)
    local mapInfo = __wow_maputil_safe_map_info(mapID)
    while mapInfo ~= nil and mapInfo.parentMapID ~= nil do
      if mapInfo.parentMapID == ancestorMapID then
        return true
      end
      mapInfo = __wow_maputil_safe_map_info(mapInfo.parentMapID)
    end
    return false
  end
end

if MapUtil.IsChildMapCached == nil then
  function MapUtil.IsChildMapCached(mapID, ancestorMapID)
    local key = tostring(mapID) .. ":" .. tostring(ancestorMapID)
    local cached = __wow_maputil_child_map_cache[key]
    if cached ~= nil then
      return cached
    end
    local result = MapUtil.IsChildMap(mapID, ancestorMapID)
    __wow_maputil_child_map_cache[key] = result
    return result
  end
end

if MapUtil.MapHasEmissaries == nil then
  function MapUtil.MapHasEmissaries(_mapID)
    return false
  end
end

if MapUtil.IsOribosMap == nil then
  function MapUtil.IsOribosMap(mapID)
    return mapID == 1670 or mapID == 1671 or mapID == 1672 or mapID == 1673
  end
end

if MapUtil.IsShadowlandsZoneMap == nil then
  function MapUtil.IsShadowlandsZoneMap(mapID)
    if mapID == 1550 or MapUtil.IsOribosMap(mapID) then
      return true
    end
    local mapInfo = __wow_maputil_safe_map_info(mapID)
    if mapInfo == nil then
      return false
    end
    if mapInfo.mapType ~= Enum.UIMapType.Zone and mapInfo.mapType ~= Enum.UIMapType.Continent then
      return false
    end
    return MapUtil.IsChildMap(mapID, 1550)
  end
end

if MapUtil.MapShouldShowWorldQuestFilters == nil then
  function MapUtil.MapShouldShowWorldQuestFilters(mapID)
    return MapUtil.MapHasEmissaries(mapID) or MapUtil.IsShadowlandsZoneMap(mapID)
  end
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


-- Rilua's C-level secureexecuterange is a no-op stub (taint.rs TODO).
-- Always install our Lua implementation to override it. Must match Elune:
--   1. Iterate with lua_next (i.e. `pairs`), NOT ipairs — hash-keyed tables
--      (CallbackRegistryMixin stores callbacks keyed by owner ID) must be
--      visited, not just the array part.
--   2. Continue iterating even if the callback errors — WoW routes errors
--      to the error handler but the loop keeps going, so each invocation
--      is wrapped in pcall.
function secureexecuterange(tbl, callback, ...)
  if type(tbl) ~= "table" or type(callback) ~= "function" then
    return
  end
  local extra = {...}
  local n = select("#", ...)
  for key, value in pairs(tbl) do
    pcall(callback, key, value, unpack(extra, 1, n))
  end
end

if debug ~= nil and debug.getfenv ~= nil then
  local __wow_debug_getfenv = debug.getfenv

  function debug.getfenv(obj)
    if type(obj) == "table" and rawget(obj, "GetObjectType") ~= nil then
      return obj
    end
    return __wow_debug_getfenv(obj)
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
