pub(super) const ADVENTURE_MAP_FRAME_SURFACE_LUA: &str = r#"
local function __wow_seed_adventure_map_canvas_state(frame)
    frame.dataProviders = frame.dataProviders or {}
    frame.dataProviderEventsCount = frame.dataProviderEventsCount or {}
    frame.pinPools = frame.pinPools or {}
    frame.pinTemplateTypes = frame.pinTemplateTypes or {}
    frame.activeAreaTriggers = frame.activeAreaTriggers or {}
    frame.lockReasons = frame.lockReasons or {}
    frame.pinsToNudge = frame.pinsToNudge or {}
    frame.pinSuppressors = frame.pinSuppressors or {}

    if type(frame.pinFrameLevelsManager) ~= "table" then
        if type(CreateFromMixins) == "function" and type(MapCanvasPinFrameLevelsManagerMixin) == "table" then
            local ok, manager = pcall(CreateFromMixins, MapCanvasPinFrameLevelsManagerMixin)
            if ok then
                frame.pinFrameLevelsManager = manager
            end
        end

        frame.pinFrameLevelsManager = frame.pinFrameLevelsManager or {}
    end

    if type(frame.pinFrameLevelsManager.Initialize) == "function" then
        pcall(frame.pinFrameLevelsManager.Initialize, frame.pinFrameLevelsManager)
    end

    frame.pinFrameLevelsManager.definitions = frame.pinFrameLevelsManager.definitions or {}
end

local function __wow_seed_adventure_map_border_frame(frame)
    if type(frame) ~= "table" or type(CreateFrame) ~= "function" then
        return
    end

    if type(frame.BorderFrame) ~= "table" then
        frame.BorderFrame = CreateFrame("Frame", nil, frame)
    end

    local borderFrame = frame.BorderFrame
    if type(borderFrame.SetPortraitToAsset) ~= "function" then
        borderFrame.SetPortraitToAsset = function() end
    end
    if type(borderFrame.Underlay) ~= "table" then
        borderFrame.Underlay = CreateFrame("Frame", nil, borderFrame)
    end
    if type(borderFrame.TitleText) ~= "table" and type(borderFrame.CreateFontString) == "function" then
        borderFrame.TitleText = borderFrame:CreateFontString(nil, "ARTWORK")
    end
    if type(borderFrame.Bg) ~= "table" and type(borderFrame.CreateTexture) == "function" then
        borderFrame.Bg = borderFrame:CreateTexture(nil, "BACKGROUND")
    end
    if type(borderFrame.TopTileStreaks) ~= "table" and type(borderFrame.CreateTexture) == "function" then
        borderFrame.TopTileStreaks = borderFrame:CreateTexture(nil, "ARTWORK")
    end
end

local function __wow_adventure_map_has_provider(frame, mixin)
    if type(frame.dataProviders) ~= "table" or type(mixin) ~= "table" then
        return true
    end

    for provider in pairs(frame.dataProviders) do
        if provider.OnAdded == mixin.OnAdded then
            return true
        end
    end

    return false
end

local function __wow_add_adventure_map_provider(frame, mixin)
    if type(frame.AddDataProvider) ~= "function"
        or type(CreateFromMixins) ~= "function"
        or __wow_adventure_map_has_provider(frame, mixin)
    then
        return
    end

    local ok, provider = pcall(CreateFromMixins, mixin)
    if ok and type(provider) == "table" then
        pcall(frame.AddDataProvider, frame, provider)
    end
end

local function __wow_seed_adventure_map_inset_pool(frame)
    if type(frame) ~= "table"
        or frame.mapInsetPool ~= nil
        or type(CreateFramePool) ~= "function"
        or type(frame.GetCanvas) ~= "function"
        or type(frame.SetMapInsetPool) ~= "function"
    then
        return
    end

    local canvasOk, canvas = pcall(frame.GetCanvas, frame)
    if not canvasOk or type(canvas) ~= "table" then
        return
    end

    local function releaseMapInset(pool, mapInset)
        if type(mapInset) == "table" and type(mapInset.OnReleased) == "function" then
            mapInset:OnReleased()
        end
    end

    local poolOk, mapInsetPool = pcall(CreateFramePool, "FRAME", canvas, "AdventureMapInsetTemplate", releaseMapInset)
    if poolOk and type(mapInsetPool) == "table" then
        pcall(frame.SetMapInsetPool, frame, mapInsetPool)
    end
end

if type(AdventureMapFrame) ~= "table"
    and type(UIParent) == "table"
    and type(CreateFrame) == "function"
    and type(MapCanvasMixin) == "table"
then
    AdventureMapFrame = CreateFrame("Frame", "AdventureMapFrame", UIParent)
    AdventureMapFrame:SetFrameStrata("DIALOG")
    AdventureMapFrame:SetSize(1004, 689)
    __wow_seed_adventure_map_canvas_state(AdventureMapFrame)
    __wow_seed_adventure_map_border_frame(AdventureMapFrame)

    if type(Mixin) == "function" then
        pcall(Mixin, AdventureMapFrame, MapCanvasMixin)
        if type(AdventureMapMixin) == "table" then
            pcall(Mixin, AdventureMapFrame, AdventureMapMixin)
        end
    end

    local scrollContainer = CreateFrame("ScrollFrame", nil, AdventureMapFrame)
    scrollContainer.Child = CreateFrame("Frame", nil, scrollContainer)
    AdventureMapFrame.ScrollContainer = scrollContainer

    __wow_seed_adventure_map_canvas_state(AdventureMapFrame)
    __wow_seed_adventure_map_border_frame(AdventureMapFrame)
    __wow_seed_adventure_map_inset_pool(AdventureMapFrame)

    if type(AdventureMapFrame.RegisterEvent) == "function" then
        pcall(AdventureMapFrame.RegisterEvent, AdventureMapFrame, "ADVENTURE_MAP_UPDATE_INSETS")
    end

    __wow_add_adventure_map_provider(AdventureMapFrame, AdventureMap_QuestChoiceDataProviderMixin)
    __wow_add_adventure_map_provider(AdventureMapFrame, AdventureMap_QuestOfferDataProviderMixin)
    __wow_add_adventure_map_provider(AdventureMapFrame, QuestSessionDataProviderMixin)
end

if type(AdventureMapFrame) == "table" then
    __wow_seed_adventure_map_border_frame(AdventureMapFrame)
    __wow_seed_adventure_map_inset_pool(AdventureMapFrame)
end
"#;

pub(super) const TOGGLE_ACHIEVEMENT_FRAME_LUA: &str = r#"
if __wow_panel_getglobal ~= nil then
    local __wow_getglobal = __wow_panel_getglobal

    local function __wow_patch_summary_empty_text_overlap()
        if rawget(_G, "__wow_achievement_summary_empty_text_patched") then
            return
        end
        if type(AchievementFrameSummary_UpdateAchievements) ~= "function" then
            return
        end

        local original = AchievementFrameSummary_UpdateAchievements
        AchievementFrameSummary_UpdateAchievements = function(...)
            local numAchievements = select('#', ...)
            local results = { original(...) }
            local emptyText = __wow_getglobal("AchievementFrameSummaryAchievementsEmptyText")
            local summary = __wow_getglobal("AchievementFrameSummaryAchievements")
            local buttons = summary and summary.buttons
            local hasVisibleSummaryButton = false

            if type(buttons) == "table" then
                for _, button in ipairs(buttons) do
                    if (type(button) == "table" or type(button) == "userdata")
                        and type(button.IsShown) == "function"
                        and button:IsShown()
                    then
                        hasVisibleSummaryButton = true
                        break
                    end
                end
            end

            if (type(emptyText) == "table" or type(emptyText) == "userdata")
                and type(emptyText.SetShown) == "function"
            then
                emptyText:SetShown(numAchievements == 0 and not hasVisibleSummaryButton)
            end

            return unpack(results)
        end

        rawset(_G, "__wow_achievement_summary_empty_text_patched", true)
    end

    function ToggleAchievementFrame(stats, toggleGuildView)
        local kiosk = __wow_getglobal("Kiosk")
        if ( (kiosk and kiosk.IsEnabled and kiosk.IsEnabled()) or __wow_getglobal("DISALLOW_FRAME_TOGGLING") ) then
            return;
        end

        local cAddOns = __wow_getglobal("C_AddOns")
        if cAddOns and cAddOns.LoadAddOn and cAddOns.IsAddOnLoaded and not cAddOns.IsAddOnLoaded("Blizzard_AchievementUI") then
            cAddOns.LoadAddOn("Blizzard_AchievementUI");
        end
        __wow_patch_summary_empty_text_overlap()

        local achievementFrame = __wow_getglobal("AchievementFrame")
        if not achievementFrame then
            return;
        end

        local achievementToggle = __wow_getglobal("AchievementFrame_ToggleAchievementFrame")
        if type(achievementToggle) == "function" then
            return achievementToggle(stats, toggleGuildView)
        end

        local requestedTab = stats and 3 or 1
        if achievementFrame:IsShown() and achievementFrame.selectedTab == requestedTab then
            local hideUIPanel = __wow_getglobal("HideUIPanel")
            if type(hideUIPanel) == "function" then
                hideUIPanel(achievementFrame)
            else
                achievementFrame:Hide();
            end
        else
            achievementFrame.selectedTab = requestedTab
            local showUIPanel = __wow_getglobal("ShowUIPanel")
            if type(showUIPanel) == "function" then
                showUIPanel(achievementFrame)
            else
                achievementFrame:Show();
            end
        end
    end
end
"#;

pub(super) const TOGGLE_ENCOUNTER_JOURNAL_LUA: &str = r#"
function ToggleEncounterJournal()
    if DISALLOW_FRAME_TOGGLING then
        return
    end
    if not EncounterJournal and type(EncounterJournal_LoadUI) == "function" then
        EncounterJournal_LoadUI()
    end
    if not EncounterJournal and type(C_AddOns) == "table" and type(C_AddOns.LoadAddOn) == "function" then
        C_AddOns.LoadAddOn("Blizzard_EncounterJournal")
    end
    if EncounterJournal then
        if EncounterJournal:IsShown() then
            if type(HideUIPanel) == "function" then
                HideUIPanel(EncounterJournal)
            else
                EncounterJournal:Hide()
            end
        else
            if type(ShowUIPanel) == "function" then
                ShowUIPanel(EncounterJournal)
            else
                EncounterJournal:Show()
            end
        end
        return true;
    end
    return false;
end
"#;

pub(super) const MAIN_MENU_MICROBUTTON_CLICK_WORKAROUND_LUA: &str = r#"
local function __wow_show_game_menu(frame)
    if type(ShowUIPanel) == "function" then
        ShowUIPanel(frame)
    end
    if type(frame.IsShown) == "function" and not frame:IsShown() and type(frame.Show) == "function" then
        frame:Show()
    end
end

local function __wow_hide_game_menu(frame)
    if type(HideUIPanel) == "function" then
        HideUIPanel(frame)
    end
    if type(frame.IsShown) == "function" and frame:IsShown() and type(frame.Hide) == "function" then
        frame:Hide()
    end
end

local function __wow_toggle_main_menu()
    local gameMenuFrame = rawget(_G, "GameMenuFrame")
    if not gameMenuFrame then
        return
    end
    if type(AreAllPanelsDisallowed) == "function" and AreAllPanelsDisallowed() then
        return
    end
    if gameMenuFrame:IsShown() then
        if type(PlaySound) == "function" and SOUNDKIT and SOUNDKIT.IG_MAINMENU_QUIT then
            PlaySound(SOUNDKIT.IG_MAINMENU_QUIT)
        end
        __wow_hide_game_menu(gameMenuFrame)
    else
        if type(SettingsPanel) == "table" and type(SettingsPanel.IsShown) == "function" and SettingsPanel:IsShown() and type(SettingsPanel.Close) == "function" then
            SettingsPanel:Close()
        end
        if type(CloseMenus) == "function" then
            CloseMenus()
        end
        if type(CloseAllWindows) == "function" then
            CloseAllWindows()
        end
        if type(PlaySound) == "function" and SOUNDKIT and SOUNDKIT.IG_MAINMENU_OPEN then
            PlaySound(SOUNDKIT.IG_MAINMENU_OPEN)
        end
        __wow_show_game_menu(gameMenuFrame)
    end
end

if type(MainMenuMicroButtonMixin) == "table" and not MainMenuMicroButtonMixin.__wow_uisim_click_patched then
    MainMenuMicroButtonMixin.__wow_uisim_click_patched = true
    MainMenuMicroButtonMixin.OnClick = function(self, button, down)
        return __wow_toggle_main_menu()
    end
end

if type(MainMenuMicroButton) == "table" and type(MainMenuMicroButton.SetScript) == "function" then
    MainMenuMicroButton:SetScript("OnClick", function(self, button, down)
        return __wow_toggle_main_menu()
    end)
end
"#;

pub(super) const TOGGLE_COLLECTIONS_JOURNAL_LUA: &str = r#"
function ToggleCollectionsJournal(tabIndex)
    if DISALLOW_FRAME_TOGGLING then
        return
    end
    if not CollectionsJournal and type(CollectionsJournal_LoadUI) == "function" then
        CollectionsJournal_LoadUI()
    end
    if not CollectionsJournal and type(C_AddOns) == "table" and type(C_AddOns.LoadAddOn) == "function" then
        C_AddOns.LoadAddOn("Blizzard_Collections")
    end
    if not CollectionsJournal then
        return
    end

    if type(SetCollectionsJournalShown) == "function" then
        local tabMatches = not tabIndex or tabIndex == PanelTemplates_GetSelectedTab(CollectionsJournal)
        local isShown = CollectionsJournal:IsShown() and tabMatches
        SetCollectionsJournalShown(not isShown, tabIndex)
    elseif CollectionsJournal:IsShown() then
        if type(HideUIPanel) == "function" then
            HideUIPanel(CollectionsJournal)
        else
            CollectionsJournal:Hide()
        end
    else
        if type(ShowUIPanel) == "function" then
            ShowUIPanel(CollectionsJournal)
        else
            CollectionsJournal:Show()
        end
    end
end
"#;

pub(super) const MOUNT_JOURNAL_DYNAMIC_FLIGHT_POPUP_WORKAROUND_LUA: &str = r#"
local function __wow_patch_mount_journal_dynamic_flight_animation()
    if type(MountJournalToggleDynamicFlightFlyoutButtonMixin) ~= "table" then
        return
    end
    if rawget(_G, "__wow_mount_journal_dynamic_flight_popup_patched") then
        return
    end
    if type(MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation) ~= "function" then
        return
    end

    MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation = function(self)
        local isPopupOpen = type(self.IsPopupOpen) == "function" and self:IsPopupOpen() or false
        if self.UnspentGlyphsAnim and type(self.UnspentGlyphsAnim.SetPlaying) == "function" then
            self.UnspentGlyphsAnim:SetPlaying(self.canSpendDragonridingGlyphs and not isPopupOpen)
        end

        local popup = rawget(self, "popup")
        local popupButton = type(popup) == "table" and rawget(popup, "OpenDynamicFlightSkillTreeButton") or nil
        local popupAnim = popupButton and popupButton.UnspentGlyphsAnim or nil
        if popupAnim and type(popupAnim.SetPlaying) == "function" then
            popupAnim:SetPlaying(self.canSpendDragonridingGlyphs and isPopupOpen)
        end
    end

    rawset(_G, "__wow_mount_journal_dynamic_flight_popup_patched", true)
end

__wow_patch_mount_journal_dynamic_flight_animation()
"#;

pub(super) const DAMAGE_METER_INITIAL_SCROLLBOX_EXTENT_LUA: &str = r#"
local function patch_damage_meter_window_initialize_scrollbox(mixinName)
    local mixin = rawget(_G, mixinName)
    if type(mixin) ~= "table" or type(mixin.InitializeScrollBox) ~= "function" or mixin.__wow_initial_extent_patch then
        return
    end

    mixin.__wow_initial_extent_patch = true
    local original = mixin.InitializeScrollBox
    mixin.InitializeScrollBox = function(self, ...)
        local result = original(self, ...)
        local scrollBox = type(self.GetScrollBox) == "function" and self:GetScrollBox() or nil
        local view = scrollBox and type(scrollBox.GetView) == "function" and scrollBox:GetView() or nil
        if view and type(view.SetElementExtent) == "function" then
            view:SetElementExtent(self:GetBarHeight())
        end
        return result
    end
end

patch_damage_meter_window_initialize_scrollbox("DamageMeterSessionWindowMixin")
patch_damage_meter_window_initialize_scrollbox("DamageMeterSourceWindowMixin")

local function apply_damage_meter_scrollbox_extent(window)
    if type(window) ~= "table" or type(window.GetScrollBox) ~= "function" or type(window.GetBarHeight) ~= "function" then
        return
    end
    local scrollBox = window:GetScrollBox()
    local view = scrollBox and type(scrollBox.GetView) == "function" and scrollBox:GetView() or nil
    if view and type(view.SetElementExtent) == "function" then
        view:SetElementExtent(window:GetBarHeight())
        if type(scrollBox.FullUpdate) == "function" and ScrollBoxConstants then
            scrollBox:FullUpdate(ScrollBoxConstants.UpdateImmediately)
        end
    end
end

if type(DamageMeter) == "table" and type(DamageMeter.ForEachSessionWindow) == "function" then
    DamageMeter:ForEachSessionWindow(function(sessionWindow)
        apply_damage_meter_scrollbox_extent(sessionWindow)
        if type(sessionWindow.GetSourceWindow) == "function" then
            apply_damage_meter_scrollbox_extent(sessionWindow:GetSourceWindow())
        end
    end)
end
"#;

pub(super) const SETTINGS_CANVAS_LAYOUT_HIDE_LUA: &str = r#"
local function __wow_hide_settings_canvas_frame(frame, layout)
    if type(frame) ~= "table" or type(frame.Hide) ~= "function" then
        return
    end

    local panel = rawget(_G, "SettingsPanel")
    local isCurrentCanvas = false
    if type(panel) == "table"
        and type(panel.IsShown) == "function"
        and panel:IsShown()
        and type(panel.GetCurrentLayout) == "function"
    then
        local ok, currentLayout = pcall(panel.GetCurrentLayout, panel)
        isCurrentCanvas = ok and currentLayout == layout
    end

    if not isCurrentCanvas then
        frame:Hide()
    end
end

local function __wow_hide_registered_settings_canvas_frames()
    local panel = rawget(_G, "SettingsPanel")
    if type(panel) ~= "table"
        or type(panel.GetAllCategories) ~= "function"
        or type(panel.GetLayout) ~= "function"
    then
        return
    end

    local ok, categories = pcall(panel.GetAllCategories, panel)
    if not ok or type(categories) ~= "table" then
        return
    end

    for _, category in ipairs(categories) do
        local layoutOk, layout = pcall(panel.GetLayout, panel, category)
        if layoutOk
            and type(layout) == "table"
            and type(layout.GetFrame) == "function"
            and type(layout.GetLayoutType) == "function"
            and SettingsLayoutMixin
            and layout:GetLayoutType() == SettingsLayoutMixin.LayoutType.Canvas
        then
            local frameOk, frame = pcall(layout.GetFrame, layout)
            if frameOk then
                __wow_hide_settings_canvas_frame(frame, layout)
            end
        end
    end
end

local function __wow_show_current_settings_canvas_frame()
    local panel = rawget(_G, "SettingsPanel")
    if type(panel) ~= "table"
        or type(panel.GetCurrentCategory) ~= "function"
        or type(panel.GetLayout) ~= "function"
    then
        return
    end

    local categoryOk, category = pcall(panel.GetCurrentCategory, panel)
    if not categoryOk or type(category) ~= "table" then
        return
    end

    local layoutOk, layout = pcall(panel.GetLayout, panel, category)
    if not layoutOk
        or type(layout) ~= "table"
        or type(layout.GetFrame) ~= "function"
        or type(layout.GetLayoutType) ~= "function"
        or not SettingsLayoutMixin
        or layout:GetLayoutType() ~= SettingsLayoutMixin.LayoutType.Canvas
    then
        return
    end

    local frameOk, frame = pcall(layout.GetFrame, layout)
    if frameOk and type(frame) == "table" and type(frame.Show) == "function" then
        frame:Show()
    end
end

local function __wow_patch_settings_canvas_registration()
    if type(Settings) ~= "table" or rawget(Settings, "__wow_canvas_layout_hide_patch") then
        return
    end

    if type(Settings.RegisterCanvasLayoutCategory) == "function" then
        local original = Settings.RegisterCanvasLayoutCategory
        Settings.RegisterCanvasLayoutCategory = function(frame, ...)
            local category, layout = original(frame, ...)
            __wow_hide_settings_canvas_frame(frame, layout)
            return category, layout
        end
    end

    if type(Settings.RegisterCanvasLayoutSubcategory) == "function" then
        local original = Settings.RegisterCanvasLayoutSubcategory
        Settings.RegisterCanvasLayoutSubcategory = function(parentCategory, frame, ...)
            local category, layout = original(parentCategory, frame, ...)
            __wow_hide_settings_canvas_frame(frame, layout)
            return category, layout
        end
    end

    if type(Settings.OpenToCategory) == "function" then
        local original = Settings.OpenToCategory
        Settings.OpenToCategory = function(...)
            local result = original(...)
            __wow_hide_registered_settings_canvas_frames()
            __wow_show_current_settings_canvas_frame()
            return result
        end
    end

    rawset(Settings, "__wow_canvas_layout_hide_patch", true)
end

__wow_patch_settings_canvas_registration()
__wow_hide_registered_settings_canvas_frames()
"#;

pub(super) const CLOSE_STARTUP_SPECIAL_WINDOWS_LUA: &str = r#"
if type(CloseAllWindows) == "function" then
    CloseAllWindows(1)
end
"#;
