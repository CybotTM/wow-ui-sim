-- Wrath-only no-op frame proxies.
--
-- These globals are referenced by wrath UI code as frame objects (`:Show()`,
-- `:Hide()`, `:SetTexture()`). Wrath's UI source doesn't bundle a
-- `Blizzard_SharedXML` addon that defines them, so they have to be stubbed.
--
-- This file is NOT loaded under mists because mists DOES bundle
-- `Blizzard_SharedXML` (which creates real `MiniMapTrackingIcon` etc. as
-- legitimate frames). Loading these proxies under mists conflicted with the
-- real frames and caused runaway recursion in mists's error handler.

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
