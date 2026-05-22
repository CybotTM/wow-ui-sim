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
