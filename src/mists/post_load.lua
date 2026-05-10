-- Mists post-load workarounds that need to wrap functions defined by
-- FrameXML / Blizzard_* addons.

if type(MicroButtonTooltipText) == "function"
   and rawget(_G, "__wow_sim_mists_micro_button_tooltip_wrapped") ~= true then
  local original = MicroButtonTooltipText
  function MicroButtonTooltipText(text, action)
    return original(text or "", action)
  end
  rawset(_G, "__wow_sim_mists_micro_button_tooltip_wrapped", true)
end

if RaidFrame and RaidFrame.RoleCount == nil then
  RaidFrame.RoleCount = CreateFrame("Frame", nil, RaidFrame)
  RaidFrame.RoleCount:Hide()
end
