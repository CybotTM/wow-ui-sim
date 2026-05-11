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

if type(LoadMicroButtonTextures) == "function"
   and rawget(_G, "__wow_sim_mists_micro_button_textures_wrapped") ~= true then
  local original = LoadMicroButtonTextures
  local aliases = {
    GuildCommunities = "Socials",
    ["GuildCommunities-GuildColor"] = "Socials",
  }

  function LoadMicroButtonTextures(button, name, ...)
    if type(name) == "string" then
      name = name:match("^[-%w]+") or name
      name = aliases[name] or name
    end
    return original(button, name, ...)
  end

  local buttonTextures = {
    CharacterMicroButton = "Character",
    SpellbookMicroButton = "Spellbook",
    TalentMicroButton = "Talents",
    AchievementMicroButton = "Achievement",
    QuestLogMicroButton = "Quest",
    SocialsMicroButton = "Socials",
    GuildMicroButton = "GuildCommunities",
    EJMicroButton = "EJ",
    CollectionsMicroButton = "Mounts",
    LFGMicroButton = "LFG",
    MainMenuMicroButton = "MainMenu",
    HelpMicroButton = "Help",
    StoreMicroButton = "BStore",
  }

  for buttonName, textureName in pairs(buttonTextures) do
    local button = rawget(_G, buttonName)
    if button then
      LoadMicroButtonTextures(button, textureName)
    end
  end

  if CharacterMicroButton then
    CharacterMicroButton:SetNormalTexture("Interface\\Buttons\\UI-MicroButtonCharacter-Up")
    CharacterMicroButton:SetPushedTexture("Interface\\Buttons\\UI-MicroButtonCharacter-Down")
    CharacterMicroButton:SetDisabledTexture("Interface\\Buttons\\UI-MicroButtonCharacter-Up")
    CharacterMicroButton:SetHighlightTexture("Interface\\Buttons\\UI-MicroButton-Hilight")
  end
  if PVPMicroButton then
    PVPMicroButton:SetNormalTexture("Interface\\Buttons\\UI-MicroButtonCharacter-Up")
    PVPMicroButton:SetPushedTexture("Interface\\Buttons\\UI-MicroButtonCharacter-Down")
    PVPMicroButton:SetDisabledTexture("Interface\\Buttons\\UI-MicroButtonCharacter-Up")
    PVPMicroButton:SetHighlightTexture("Interface\\Buttons\\UI-MicroButton-Hilight")
  end

  rawset(_G, "__wow_sim_mists_micro_button_textures_wrapped", true)
end

if RaidFrame and RaidFrame.RoleCount == nil then
  RaidFrame.RoleCount = CreateFrame("Frame", nil, RaidFrame)
  RaidFrame.RoleCount:Hide()
end

if rawget(_G, "__wow_sim_mists_spellbook_button_sizes_applied") ~= true then
  for i = 1, 12 do
    local button = rawget(_G, "SpellButton" .. i)
    if button and button.GetWidth and button.SetSize
       and (button:GetWidth() == 0 or button:GetHeight() == 0) then
      button:SetSize(37, 37)
    end
  end

  rawset(_G, "__wow_sim_mists_spellbook_button_sizes_applied", true)
end

function CombatLog_LoadUI()
  return true
end
