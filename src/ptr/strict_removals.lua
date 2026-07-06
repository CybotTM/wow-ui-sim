local function __wow_hide_namespace_key(tableValue, key)
  if type(tableValue) ~= "table" then
    return
  end
  rawset(tableValue, key, nil)

  local removedKeys = rawget(tableValue, "__wow_removed_keys")
  if type(removedKeys) ~= "table" then
    removedKeys = {}
    rawset(tableValue, "__wow_removed_keys", removedKeys)
  end
  removedKeys[key] = true

  local mt = getmetatable(tableValue) or {}
  if rawget(mt, "__wow_removed_filter") then
    return
  end

  local oldIndex = mt.__index
  mt.__index = function(t, lookupKey)
    local removed = rawget(t, "__wow_removed_keys")
    if type(removed) == "table" and removed[lookupKey] then
      return nil
    end
    if type(oldIndex) == "function" then
      return oldIndex(t, lookupKey)
    end
    if type(oldIndex) == "table" then
      return oldIndex[lookupKey]
    end
    return nil
  end
  mt.__wow_removed_filter = true
  setmetatable(tableValue, mt)
end

local function __wow_remove_table_field(tableValue, key)
  if type(tableValue) == "table" then
    rawset(tableValue, key, nil)
  end
end

__wow_hide_namespace_key(C_DyeColor, "GetDyeColorForItemLocation")
__wow_hide_namespace_key(C_DyeColor, "GetDyeColorForItem")
__wow_hide_namespace_key(C_Housing, "IsInsideOwnHouse")
__wow_hide_namespace_key(C_Ping, "GetContextualPingTypeForUnit")
__wow_hide_namespace_key(C_RecruitAFriend, "IsEnabled")
__wow_hide_namespace_key(C_SuperTrack, "GetNextWaypointForMap")
__wow_hide_namespace_key(C_UnitAuras, "TriggerPrivateAuraShowDispelType")
__wow_remove_table_field(Enum.EditModeUnitFrameSetting, "IconSize")
rawset(_G, "GetInventorySlotInfo", nil)

local __wow_removed_cvars = {
  lastlockeddelvescompanionabilities = true,
  slugsupersampling = true,
}

local function __wow_is_removed_cvar(name)
  return type(name) == "string" and __wow_removed_cvars[string.lower(name)] == true
end

if type(GetCVar) == "function" and rawget(_G, "__wow_original_GetCVar") == nil then
  rawset(_G, "__wow_original_GetCVar", GetCVar)
  function GetCVar(name)
    if __wow_is_removed_cvar(name) then
      return nil
    end
    return __wow_original_GetCVar(name)
  end
end

if type(GetCVarDefault) == "function" and rawget(_G, "__wow_original_GetCVarDefault") == nil then
  rawset(_G, "__wow_original_GetCVarDefault", GetCVarDefault)
  function GetCVarDefault(name)
    if __wow_is_removed_cvar(name) then
      return nil
    end
    return __wow_original_GetCVarDefault(name)
  end
end

if type(C_CVar) == "table" and type(C_CVar.GetCVar) == "function" and rawget(C_CVar, "__wow_original_GetCVar") == nil then
  rawset(C_CVar, "__wow_original_GetCVar", C_CVar.GetCVar)
  function C_CVar.GetCVar(name)
    if __wow_is_removed_cvar(name) then
      return nil
    end
    return C_CVar.__wow_original_GetCVar(name)
  end
end
