-- Mists post-load workarounds that need to wrap functions defined by
-- FrameXML / Blizzard_* addons.

if AuctionHouseFrame and rawget(_G, "__wow_sim_mists_auction_house_hidden_on_startup") ~= true then
  AuctionHouseFrame:Hide()
  rawset(_G, "__wow_sim_mists_auction_house_hidden_on_startup", true)
end

if VideoOptionsFrame == nil and type(CreateFrame) == "function" then
  VideoOptionsFrame = CreateFrame("Frame", "VideoOptionsFrame", UIParent)
  VideoOptionsFrame:Hide()
end

if Syndicator ~= nil and SYNDICATOR_CONFIG == nil then
  SYNDICATOR_CONFIG = {}
end

if type(Settings) == "table" then
  local categoriesByID = rawget(Settings, "__wow_sim_mists_categories_by_id")
  if type(categoriesByID) ~= "table" then
    categoriesByID = {}
    rawset(Settings, "__wow_sim_mists_categories_by_id", categoriesByID)
  end
  local categoriesByName = rawget(Settings, "__wow_sim_mists_categories_by_name")
  if type(categoriesByName) ~= "table" then
    categoriesByName = {}
    rawset(Settings, "__wow_sim_mists_categories_by_name", categoriesByName)
  end
  local originalRegisterCategory = Settings.RegisterCategory
  local originalRegisterAddOnCategory = Settings.RegisterAddOnCategory
  local originalRegisterCanvasLayoutCategory = Settings.RegisterCanvasLayoutCategory
  local originalRegisterCanvasLayoutSubcategory = Settings.RegisterCanvasLayoutSubcategory
  local originalRegisterVerticalLayoutCategory = Settings.RegisterVerticalLayoutCategory
  local originalRegisterVerticalLayoutSubcategory = Settings.RegisterVerticalLayoutSubcategory
  local originalGetCategory = Settings.GetCategory
  local alreadyWrapped =
    rawget(Settings, "__wow_sim_mists_wrapped_get_category") == originalGetCategory
    and rawget(Settings, "__wow_sim_mists_wrapped_register_category") == originalRegisterCategory
    and rawget(Settings, "__wow_sim_mists_wrapped_register_addon_category") == originalRegisterAddOnCategory
    and rawget(Settings, "__wow_sim_mists_wrapped_register_canvas_layout_category") == originalRegisterCanvasLayoutCategory
    and rawget(Settings, "__wow_sim_mists_wrapped_register_canvas_layout_subcategory") == originalRegisterCanvasLayoutSubcategory
    and rawget(Settings, "__wow_sim_mists_wrapped_register_vertical_layout_category") == originalRegisterVerticalLayoutCategory
    and rawget(Settings, "__wow_sim_mists_wrapped_register_vertical_layout_subcategory") == originalRegisterVerticalLayoutSubcategory

  if not alreadyWrapped then
    local function rememberCategory(category)
      if type(category) ~= "table" then
        return category
      end
      if type(category.GetID) == "function" then
        local ok, id = pcall(category.GetID, category)
        if ok and id ~= nil then
          categoriesByID[id] = category
        end
      end
      if type(category.GetName) == "function" then
        local ok, name = pcall(category.GetName, category)
        if ok and name ~= nil then
          categoriesByName[name] = category
        end
      end
      return category
    end

    if type(originalRegisterCategory) == "function" then
      function Settings.RegisterCategory(category, ...)
        rememberCategory(category)
        return originalRegisterCategory(category, ...)
      end
    end

    if type(originalRegisterAddOnCategory) == "function" then
      function Settings.RegisterAddOnCategory(category, ...)
        rememberCategory(category)
        return originalRegisterAddOnCategory(category, ...)
      end
    end

    if type(originalRegisterCanvasLayoutCategory) == "function" then
      function Settings.RegisterCanvasLayoutCategory(...)
        local category, layout = originalRegisterCanvasLayoutCategory(...)
        rememberCategory(category)
        return category, layout
      end
    end

    if type(originalRegisterCanvasLayoutSubcategory) == "function" then
      function Settings.RegisterCanvasLayoutSubcategory(parentCategory, ...)
        local category, layout = originalRegisterCanvasLayoutSubcategory(parentCategory, ...)
        rememberCategory(parentCategory)
        rememberCategory(category)
        return category, layout
      end
    end

    if type(originalRegisterVerticalLayoutCategory) == "function" then
      function Settings.RegisterVerticalLayoutCategory(...)
        local category, layout = originalRegisterVerticalLayoutCategory(...)
        rememberCategory(category)
        return category, layout
      end
    end

    if type(originalRegisterVerticalLayoutSubcategory) == "function" then
      function Settings.RegisterVerticalLayoutSubcategory(parentCategory, ...)
        local category, layout = originalRegisterVerticalLayoutSubcategory(parentCategory, ...)
        rememberCategory(parentCategory)
        rememberCategory(category)
        return category, layout
      end
    end

    if type(originalGetCategory) == "function" then
      function Settings.GetCategory(categoryID)
        local category = originalGetCategory(categoryID)
        if category ~= nil then
          return rememberCategory(category)
        end
        return categoriesByID[categoryID] or categoriesByName[categoryID]
      end
    end

    rawset(Settings, "__wow_sim_mists_wrapped_get_category", Settings.GetCategory)
    rawset(Settings, "__wow_sim_mists_wrapped_register_category", Settings.RegisterCategory)
    rawset(Settings, "__wow_sim_mists_wrapped_register_addon_category", Settings.RegisterAddOnCategory)
    rawset(Settings, "__wow_sim_mists_wrapped_register_canvas_layout_category", Settings.RegisterCanvasLayoutCategory)
    rawset(Settings, "__wow_sim_mists_wrapped_register_canvas_layout_subcategory", Settings.RegisterCanvasLayoutSubcategory)
    rawset(Settings, "__wow_sim_mists_wrapped_register_vertical_layout_category", Settings.RegisterVerticalLayoutCategory)
    rawset(Settings, "__wow_sim_mists_wrapped_register_vertical_layout_subcategory", Settings.RegisterVerticalLayoutSubcategory)
  end

  rawset(_G, "__wow_sim_mists_settings_category_lookup_wrapped", true)
end

if rawget(_G, "__wow_sim_mists_blizzmove_startup_scan_disabled") ~= true then
  local blizzMove = rawget(_G, "BlizzMove")
  if type(blizzMove) ~= "table" and type(LibStub) == "function" then
    local ok, addon = pcall(function()
      return LibStub("AceAddon-3.0"):GetAddon("BlizzMove", true)
    end)
    if ok then
      blizzMove = addon
    end
  end

  if type(blizzMove) == "table" then
    function blizzMove:OnInitialize()
      self.initialized = true
    end

    function blizzMove:ProcessFrames(_addOnName)
    end

    function blizzMove:ADDON_LOADED(_event, _addOnName)
    end

    rawset(_G, "__wow_sim_mists_blizzmove_startup_scan_disabled", true)
  end
end

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

local function HideDuplicateCharacterFrameTitle()
  if type(CharacterFrame) ~= "table" then
    return
  end

  local directTitle = CharacterFrame.TitleText
  local titleContainer = CharacterFrame.TitleContainer
  local containerTitle = type(titleContainer) == "table" and titleContainer.TitleText or nil
  if directTitle ~= nil and containerTitle ~= nil and directTitle ~= containerTitle
     and type(containerTitle.Hide) == "function" then
    containerTitle:Hide()
  end
end

HideDuplicateCharacterFrameTitle()

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

local function LayoutSpecializationBody(frame)
  if type(frame) ~= "table" or type(frame.spellsScroll) ~= "table" then
    return
  end

  local scrollChild = frame.spellsScroll.child
  if type(scrollChild) ~= "table"
     or type(scrollChild.description) ~= "table"
     or type(scrollChild.Seperator) ~= "table" then
    return
  end

  local separator = scrollChild.Seperator
  separator:ClearAllPoints()
  separator:SetPoint("TOP", scrollChild.description, "BOTTOM", 0, -8)

  local ability1 = scrollChild.abilityButton1
  local ability2 = scrollChild.abilityButton2
  local ability3 = scrollChild.abilityButton3
  local ability4 = scrollChild.abilityButton4
  if not (ability1 and ability2 and ability3 and ability4) then
    return
  end

  ability1:ClearAllPoints()
  ability1:SetPoint("TOPLEFT", separator, "BOTTOMLEFT", -5, -18)
  ability2:ClearAllPoints()
  ability2:SetPoint("TOPLEFT", ability1, "TOPLEFT", 180, 0)
  ability3:ClearAllPoints()
  ability3:SetPoint("TOPLEFT", ability1, "BOTTOMLEFT", 0, -20)
  ability4:ClearAllPoints()
  ability4:SetPoint("TOPLEFT", ability3, "TOPLEFT", 180, 0)
end

local function PatchSpecializationBodyLayout()
  if type(PlayerTalentFrame_UpdateSpecFrame) ~= "function"
     or rawget(_G, "__wow_sim_mists_specialization_body_layout_wrapped") == true then
    return
  end

  local original = PlayerTalentFrame_UpdateSpecFrame
  function PlayerTalentFrame_UpdateSpecFrame(frame, ...)
    local result = { original(frame, ...) }
    LayoutSpecializationBody(frame)
    return unpack(result)
  end

  rawset(_G, "__wow_sim_mists_specialization_body_layout_wrapped", true)
end

PatchSpecializationBodyLayout()
LayoutSpecializationBody(rawget(_G, "PlayerTalentFrameSpecialization"))

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
