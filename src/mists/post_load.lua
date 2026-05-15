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

if type(ToggleStoreUI) == "function"
   and rawget(_G, "__wow_sim_mists_store_toggle_wrapped") ~= true then
  local function syncStoreFrameVisibility(shown)
    rawset(_G, "__wow_sim_mists_store_shown", shown and true or false)
    pcall(LoadAddOn, "Blizzard_CatalogShop")
    local catalogFrame = rawget(_G, "CatalogShopFrame")
    if catalogFrame and type(catalogFrame.SetShown) == "function" then
      catalogFrame:SetShown(shown and true or false)
    end
  end

  local originalStoreFrameIsShown = StoreFrame_IsShown
  function StoreFrame_IsShown()
    if rawget(_G, "__wow_sim_mists_store_shown") == true then
      return true
    end
    return originalStoreFrameIsShown and originalStoreFrameIsShown() or false
  end

  function ToggleStoreUI()
    local wasShown = rawget(_G, "__wow_sim_mists_store_shown") == true
    syncStoreFrameVisibility(not wasShown)
    if type(UpdateMicroButtons) == "function" then
      UpdateMicroButtons()
    end
  end

  if StoreMicroButton and type(StoreMicroButton.SetScript) == "function" then
    StoreMicroButton:SetScript("OnClick", function(self)
      return ToggleStoreUI()
    end)
  end
  if type(StoreMicroButtonMixin) == "table" then
    StoreMicroButtonMixin.OnClick = function(self)
      return ToggleStoreUI()
    end
  end

  if type(SetStoreUIShown) == "function" then
    function SetStoreUIShown(shown, ...)
      syncStoreFrameVisibility(shown)
      if type(UpdateMicroButtons) == "function" then
        UpdateMicroButtons()
      end
    end
  end

  rawset(_G, "__wow_sim_mists_store_toggle_wrapped", true)
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

local function ClearMainBackpackSlotNormalTextures(frame)
  if not frame or type(frame.GetID) ~= "function" or frame:GetID() ~= 0 then
    return
  end

  if type(frame.GetName) ~= "function" then
    return
  end

  local frameName = frame:GetName()
  local slotCount = frame.size or 0
  for slot = 1, slotCount do
    local button = rawget(_G, frameName .. "Item" .. slot)
    if button and type(button.ClearNormalTexture) == "function" then
      button:ClearNormalTexture()
    end
  end
end

if type(ContainerFrame_Update) == "function"
   and rawget(_G, "__wow_sim_mists_backpack_slot_normals_wrapped") ~= true then
  local originalContainerFrameUpdate = ContainerFrame_Update
  function ContainerFrame_Update(frame, ...)
    local result = originalContainerFrameUpdate(frame, ...)
    ClearMainBackpackSlotNormalTextures(frame)
    return result
  end

  rawset(_G, "__wow_sim_mists_backpack_slot_normals_wrapped", true)
end

local function HideInactivePvpReadyDialog()
  local dialog = rawget(_G, "PVPReadyDialog")
  if not dialog or not dialog.Hide then
    return
  end

  if dialog.activeIndex == nil then
    dialog:Hide()
  end
end

HideInactivePvpReadyDialog()

local function RefreshCharacterPetTabAvailability()
  if rawget(_G, "__wow_sim_mists_refreshing_character_pet_tab") == true then
    return
  end
  if type(PetPaperDollFrame_UpdateIsAvailable) ~= "function" then
    return
  end
  if type(PetPaperDollFrame) ~= "table" or type(CharacterFrameTab2) ~= "table" then
    return
  end

  rawset(_G, "__wow_sim_mists_refreshing_character_pet_tab", true)
  PetPaperDollFrame_UpdateIsAvailable()
  rawset(_G, "__wow_sim_mists_refreshing_character_pet_tab", false)
end

local function PatchCharacterPetTabOpenRefresh()
  if type(ToggleCharacter) ~= "function"
     or rawget(_G, "__wow_sim_mists_character_pet_tab_refresh_wrapped") == true then
    return
  end

  local original = ToggleCharacter
  function ToggleCharacter(...)
    local result = { original(...) }
    RefreshCharacterPetTabAvailability()
    return unpack(result)
  end

  rawset(_G, "__wow_sim_mists_character_pet_tab_refresh_wrapped", true)
end

PatchCharacterPetTabOpenRefresh()
RefreshCharacterPetTabAvailability()

local function ResizeVisibleSpellBookBottomTabs()
  if type(SpellBookFrame) ~= "table" or type(PanelTemplates_TabResize) ~= "function" then
    return
  end

  for index = 1, 5 do
    local tab = rawget(_G, "SpellBookFrameTabButton" .. index)
    if tab and tab:IsShown() then
      PanelTemplates_TabResize(tab, 0, nil, 36, SpellBookFrame.maxTabWidth or 88)
    end
  end
end

local function PatchSpellBookBottomTabSizing()
  if type(SpellBookFrame) ~= "table"
     or type(SpellBookFrame.Update) ~= "function"
     or rawget(_G, "__wow_sim_mists_spellbook_tabs_wrapped") == true then
    return
  end

  local original = SpellBookFrame.Update
  function SpellBookFrame:Update(...)
    local result = { original(self, ...) }
    ResizeVisibleSpellBookBottomTabs()
    return unpack(result)
  end

  rawset(_G, "__wow_sim_mists_spellbook_tabs_wrapped", true)
end

PatchSpellBookBottomTabSizing()
ResizeVisibleSpellBookBottomTabs()

function CombatLog_LoadUI()
  return true
end

local function PatchMistsCollectionsJournal()
  if type(CollectionsJournal) ~= "table"
     or rawget(_G, "__wow_sim_mists_collections_journal_patched") == true then
    return
  end

  local wardrobeTab = rawget(_G, "CollectionsJournalTab5")
  if wardrobeTab and wardrobeTab.Hide then
    wardrobeTab:Hide()
    if wardrobeTab.Disable then
      wardrobeTab:Disable()
    end
  end
  rawset(_G, "CollectionsJournalTab5", nil)
  rawset(_G, "WardrobeCollectionFrame", nil)
  rawset(_G, "WardrobeFrame", nil)

  PanelTemplates_SetNumTabs(CollectionsJournal, 4)
  if CollectionsJournal.selectedTab and CollectionsJournal.selectedTab > 4 then
    PanelTemplates_SetTab(CollectionsJournal, 1)
  end

  local titles = {
    [1] = MOUNTS,
    [2] = PET_JOURNAL,
    [3] = TOY_BOX,
    [4] = HEIRLOOMS,
  }

  function CollectionsJournal_ValidateTab(tabNum)
    return type(tabNum) == "number" and tabNum >= 1 and tabNum <= 4
  end

  function CollectionsJournal_UpdateSelectedTab(self)
    local selected = CollectionsJournal_GetTab(self)
    if not CollectionsJournal_ValidateTab(selected) then
      PanelTemplates_SetTab(self, 1)
      selected = 1
    end

    MountJournal:SetShown(selected == 1)
    PetJournal:SetShown(selected == 2)
    ToyBox:SetShown(selected == 3)
    HeirloomsJournal:SetShown(selected == 4)
    self:SetTitle(titles[selected] or COLLECTIONS)

    if EventRegistry and EventRegistry.TriggerEvent then
      EventRegistry:TriggerEvent("CollectionsJournal.TabSet", CollectionsJournal, selected)
    end
  end

  rawset(_G, "__wow_sim_mists_collections_journal_patched", true)
end

PatchMistsCollectionsJournal()
