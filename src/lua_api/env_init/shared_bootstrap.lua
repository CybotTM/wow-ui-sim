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

local __wow_maputil_shadowlands_continent_map_id = 1550
local __wow_maputil_oribos_ui_map_ids = { 1670, 1671, 1672, 1673 }

MapUtil = MapUtil or {}

if MapUtil.IsMapTypeZone == nil then
  function MapUtil.IsMapTypeZone(mapID)
    if not C_Map or not C_Map.GetMapInfo then
      return false
    end
    local mapInfo = C_Map.GetMapInfo(mapID)
    return mapInfo and mapInfo.mapType == Enum.UIMapType.Zone or false
  end
end

if MapUtil.GetMapParentInfo == nil then
  function MapUtil.GetMapParentInfo(mapID, mapType, topMost)
    if not C_Map or not C_Map.GetMapInfo then
      return nil
    end
    local candidate
    local mapInfo = C_Map.GetMapInfo(mapID)
    while mapInfo do
      if mapInfo.mapType == mapType then
        if topMost then
          candidate = mapInfo
        else
          return mapInfo
        end
      end
      mapInfo = C_Map.GetMapInfo(mapInfo.parentMapID)
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

if MapUtil.ShouldShowTask == nil then
  function MapUtil.ShouldShowTask(mapID, info)
    if type(info) ~= "table" then
      return false
    end
    if info.isQuestStart and info.inProgress then
      return false
    end
    if HaveQuestData and not HaveQuestData(info.questID) then
      return false
    end
    if C_QuestLog and C_QuestLog.IsQuestCalling
        and C_QuestLog.IsQuestCalling(info.questID)
        and MapUtil.IsMapTypeZone(mapID) then
      return true
    end
    return mapID == info.mapID
  end
end

if MapUtil.MapHasUnlockedBounties == nil then
  function MapUtil.MapHasUnlockedBounties(mapID)
    if not C_QuestLog or not C_QuestLog.GetBountySetInfoForMapID then
      return false
    end
    local displayLocation, lockedQuestID = C_QuestLog.GetBountySetInfoForMapID(mapID)
    if displayLocation and (not lockedQuestID or not C_QuestLog.IsOnQuest
        or not C_QuestLog.IsOnQuest(lockedQuestID)) then
      local bounties = C_QuestLog.GetBountiesForMapID and C_QuestLog.GetBountiesForMapID(mapID)
      return bounties and #bounties > 0 or false
    end
    return false
  end
end

if MapUtil.MapHasEmissaries == nil then
  function MapUtil.MapHasEmissaries(mapID)
    if not C_QuestLog or not C_QuestLog.GetBountySetInfoForMapID then
      return false
    end
    local displayLocation = C_QuestLog.GetBountySetInfoForMapID(mapID)
    return displayLocation ~= nil
  end
end

if MapUtil.FindBestAreaNameAtMouse == nil then
  function MapUtil.FindBestAreaNameAtMouse(mapID, normalizedCursorX, normalizedCursorY)
    if not C_MapExplorationInfo or not C_MapExplorationInfo.GetExploredAreaIDsAtPosition
        or not CreateVector2D or not C_Map or not C_Map.GetAreaInfo then
      return nil
    end
    local exploredAreaIDs = C_MapExplorationInfo.GetExploredAreaIDsAtPosition(
      mapID,
      CreateVector2D(normalizedCursorX, normalizedCursorY)
    )
    if exploredAreaIDs then
      for _, areaID in ipairs(exploredAreaIDs) do
        local name = C_Map.GetAreaInfo(areaID)
        if name then
          return name
        end
      end
    end
    return nil
  end
end

if MapUtil.GetDisplayableMapForPlayer == nil then
  function MapUtil.GetDisplayableMapForPlayer()
    if not C_Map or not C_Map.GetBestMapForUnit then
      return nil
    end
    local mapID = C_Map.GetBestMapForUnit("player")
    if mapID then
      repeat
        if C_Map.MapHasArt and C_Map.MapHasArt(mapID) then
          return mapID
        end
        local mapInfo = C_Map.GetMapInfo and C_Map.GetMapInfo(mapID)
        mapID = mapInfo and mapInfo.parentMapID or 0
      until mapID == 0
    end
    if C_Map.GetFallbackWorldMapID then
      return C_Map.GetFallbackWorldMapID()
    end
    return nil
  end
end

if MapUtil.GetBountySetMaps == nil then
  function MapUtil.GetBountySetMaps(bountySetID)
    MapUtil.bountySetMaps = MapUtil.bountySetMaps or {}
    local bountySetMaps = MapUtil.bountySetMaps[bountySetID]
    if bountySetMaps == nil then
      if C_Map and C_Map.GetBountySetMaps then
        bountySetMaps = C_Map.GetBountySetMaps(bountySetID)
      end
      if bountySetMaps == nil then
        bountySetMaps = {}
      end
      MapUtil.bountySetMaps[bountySetID] = bountySetMaps
    end
    return bountySetMaps
  end
end

if MapUtil.GetMapCenterOnMap == nil then
  function MapUtil.GetMapCenterOnMap(mapID, topMapID)
    if not C_Map or not C_Map.GetMapRectOnMap then
      return nil, nil
    end
    local left, right, top, bottom = C_Map.GetMapRectOnMap(mapID, topMapID)
    if left == nil then
      return nil, nil
    end
    local centerX = left + (right - left) * 0.5
    local centerY = top + (bottom - top) * 0.5
    return centerX, centerY
  end
end

if MapUtil.IsChildMap == nil then
  function MapUtil.IsChildMap(mapID, ancestorMapID)
    if not C_Map or not C_Map.GetMapInfo then
      return false
    end
    local mapInfo = C_Map.GetMapInfo(mapID)
    while mapInfo and mapInfo.parentMapID do
      if mapInfo.parentMapID == ancestorMapID then
        return true
      end
      mapInfo = C_Map.GetMapInfo(mapInfo.parentMapID)
    end
    return false
  end
end

if MapUtil.IsChildMapCached == nil then
  local childMapCache = {}
  function MapUtil.IsChildMapCached(mapID, ancestorMapID)
    local key = tostring(mapID) .. ":" .. tostring(ancestorMapID)
    local result = childMapCache[key]
    if result ~= nil then
      return result
    end
    result = MapUtil.IsChildMap(mapID, ancestorMapID)
    childMapCache[key] = result
    return result
  end
end

if MapUtil.IsOribosMap == nil then
  function MapUtil.IsOribosMap(mapID)
    for _, candidate in ipairs(__wow_maputil_oribos_ui_map_ids) do
      if candidate == mapID then
        return true
      end
    end
    return false
  end
end

if MapUtil.IsShadowlandsZoneMap == nil then
  function MapUtil.IsShadowlandsZoneMap(mapID)
    if mapID == __wow_maputil_shadowlands_continent_map_id or MapUtil.IsOribosMap(mapID) then
      return true
    end
    if not C_Map or not C_Map.GetMapInfo then
      return false
    end
    local mapInfo = C_Map.GetMapInfo(mapID)
    if not mapInfo then
      return false
    end
    if mapInfo.mapType ~= Enum.UIMapType.Zone and mapInfo.mapType ~= Enum.UIMapType.Continent then
      return false
    end
    return MapUtil.IsChildMap(mapID, __wow_maputil_shadowlands_continent_map_id)
  end
end

if MapUtil.MapShouldShowWorldQuestFilters == nil then
  function MapUtil.MapShouldShowWorldQuestFilters(mapID)
    return MapUtil.MapHasEmissaries(mapID) or MapUtil.IsShadowlandsZoneMap(mapID)
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
