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
