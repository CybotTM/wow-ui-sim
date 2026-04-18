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
