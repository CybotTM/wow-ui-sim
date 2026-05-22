use super::*;

pub(super) fn patch_ui_parent_panel_toggles(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(GETGLOBAL_HELPER_LUA);
    let _ = env.exec(TOGGLE_ACHIEVEMENT_FRAME_LUA);
    let _ = env.exec(TOGGLE_ENCOUNTER_JOURNAL_LUA);
    let _ = env.exec(TOGGLE_COLLECTIONS_JOURNAL_LUA);
    let _ = env.exec(MAIN_MENU_MICROBUTTON_CLICK_WORKAROUND_LUA);
}

pub(super) fn patch_damage_meter_initial_scrollbox_extent(env: &crate::lua_api::LoaderEnv<'_>) {
    temporary::damage_meter_scrollbox::patch(env);
}

pub(super) fn patch_housing_dashboard_preload(env: &crate::lua_api::LoaderEnv<'_>) {
    temporary::housing_dashboard_preload::patch(env);
}

pub(super) fn patch_uiparent_onupdate_worklists(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(UIPARENT_ONUPDATE_WORKLISTS_WORKAROUND_LUA);
}

pub(super) fn patch_vignette_pin_template(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(VIGNETTE_PIN_TEMPLATE_WORKAROUND_LUA);
}

pub(super) fn patch_character_select_selected_name(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(
        r#"
        if type(CharacterSelect_SetSelectedCharacterName) == "function"
            and not rawget(_G, "__wow_character_select_selected_name_patched") then
            local original = CharacterSelect_SetSelectedCharacterName
            CharacterSelect_SetSelectedCharacterName = function(name, timerunningSeasonID)
                if type(CharSelectCharacterName) ~= "table" then
                    return
                end
                return original(name, timerunningSeasonID)
            end
            rawset(_G, "__wow_character_select_selected_name_patched", true)
        end
        "#,
    );
}

pub(super) fn patch_chat_voice_button_surface(env: &crate::lua_api::WowLuaEnv) {
    temporary::chat_voice_button_surface::patch(env);
}

pub(super) fn patch_item_socketing_tooltips(env: &crate::lua_api::WowLuaEnv) {
    temporary::item_socketing_tooltips::patch(env);
}

pub(super) fn patch_action_bar_button_event_fanout(env: &crate::lua_api::WowLuaEnv) {
    let trace_fanout = std::env::var_os("WOW_SIM_TRACE_ACTIONBAR_BUTTON_FANOUT").is_some();
    let script = action_bar_button_event_fanout_script(trace_fanout);
    let _ = env.exec(&script);
}

pub(super) fn action_bar_button_event_fanout_script(trace_fanout: bool) -> String {
    let trace_fanout = if trace_fanout { "true" } else { "false" };
    ACTION_BAR_BUTTON_EVENT_FANOUT_WORKAROUND_LUA.replace("{trace_fanout}", trace_fanout)
}

pub(super) fn patch_game_time_defaults(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(
        r#"
        if type(GameTimeFrame) == "table" and GameTimeFrame.pendingCalendarInvites == nil then
            GameTimeFrame.pendingCalendarInvites = 0
        end
        "#,
    );
}

/// Initialize `LFGLockList` so the Dungeons & Raids panel can populate
/// its dungeon list. In retail this happens via `LFG_LOCK_INFO_RECEIVED`,
/// but firing that event also triggers RaidFinder/ScenarioFinder
/// availability checks that depend on many unmodeled APIs. Direct
/// assignment is the minimal fix: `UpdateLFDDungeonList` reads
/// `LFGLockList[id]` (LFDFrame.lua:697) before `LFGDungeonList_Setup`
/// initializes it on demand.
pub(super) fn patch_lfg_lock_list(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(
        r#"
        if type(GetLFGLockList) == "function" and LFGLockList == nil then
            LFGLockList = GetLFGLockList()
        end
        "#,
    );
}

pub(super) fn patch_tooltip_nineslice_surface(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(
        r#"
        local function ensure_tooltip_nineslice(tooltip)
            if type(tooltip) ~= "table" or tooltip.NineSlice ~= nil then
                return
            end

            if type(CreateFrame) ~= "function" or type(NineSliceUtil) ~= "table" then
                return
            end

            local nineSlice = CreateFrame("Frame", nil, tooltip, "NineSlicePanelTemplate")
            if nineSlice == nil then
                return
            end

            tooltip.NineSlice = nineSlice
            if type(nineSlice.SetParentKey) == "function" then
                pcall(nineSlice.SetParentKey, nineSlice, "NineSlice", true)
            end
            if type(NineSliceUtil.DisableSharpening) == "function" then
                NineSliceUtil.DisableSharpening(nineSlice)
            end
            if type(SharedTooltip_SetBackdropStyle) == "function" then
                pcall(SharedTooltip_SetBackdropStyle, tooltip, nil, false)
            end
        end

        ensure_tooltip_nineslice(GameTooltip)
        ensure_tooltip_nineslice(GlueTooltip)
        "#,
    );
}

pub(super) fn patch_container_frame_token_tracker(env: &crate::lua_api::WowLuaEnv) {
    // Startup emits a consolidated ADDON_LOADED("WoWUISim") event after
    // bootstrap. Bag setup expects Blizzard_TokenUI's per-addon callback to
    // have initialized ContainerFrameSettingsManager.TokenTracker.
    let _ = env.exec(
        r#"
        if type(ContainerFrameSettingsManager) ~= "table" then
            return
        end
        if ContainerFrameSettingsManager.TokenTracker ~= nil then
            return
        end
        if type(ContainerFrameSettingsManager.OnAddonLoaded) ~= "function" then
            return
        end

        local tokenUiLoaded = false
        if type(C_AddOns) == "table" and type(C_AddOns.IsAddOnLoaded) == "function" then
            tokenUiLoaded = C_AddOns.IsAddOnLoaded("Blizzard_TokenUI")
        end

        if tokenUiLoaded then
            pcall(
                ContainerFrameSettingsManager.OnAddonLoaded,
                ContainerFrameSettingsManager,
                "Blizzard_TokenUI"
            )
        end
        "#,
    );
}

pub(super) fn patch_paging_controls_page_text(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(PAGING_CONTROLS_PAGE_TEXT_WORKAROUND_LUA);
}

pub(super) fn patch_achievement_display_set_achievements(env: &crate::lua_api::WowLuaEnv) {
    // Blizzard_FrameXML/AchievementDisplayFrame.lua reassigns
    // `AchievementDisplayMixin = {}` and re-defines `:SetAchievements`
    // on top of the bootstrap stub. The live body iterates through
    // a frame pool and reads `GetAchievementInfo` per criteria — both
    // out of scope for a 2D-only simulator. Reinstate the stub so the
    // AlliedRaces panel call site doesn't error.
    let _ = env.exec(ACHIEVEMENT_DISPLAY_SET_ACHIEVEMENTS_WORKAROUND_LUA);
}

pub(super) const ACHIEVEMENT_DISPLAY_SET_ACHIEVEMENTS_WORKAROUND_LUA: &str = r#"
    if type(AchievementDisplayMixin) ~= "table" then
        AchievementDisplayMixin = {}
    end
    AchievementDisplayMixin.SetAchievements = function(self, achievementIds)
        self.achievementIds = achievementIds
    end
"#;

pub(super) fn patch_talent_edge_frame_level_sync(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(TALENT_EDGE_FRAME_LEVEL_SYNC_WORKAROUND_LUA);
}

pub(super) const PAGING_CONTROLS_PAGE_TEXT_WORKAROUND_LUA: &str = r#"
    if type(PagingControlsMixin) ~= "table"
        or type(PagingControlsMixin.UpdateControls) ~= "function" then
        return
    end

    if rawget(_G, "__wow_paging_controls_update_controls_wrapper") then
        return
    end

    local original_update_controls = PagingControlsMixin.UpdateControls
    PagingControlsMixin.UpdateControls = function(self, ...)
        original_update_controls(self, ...)

        local pageText = self and self.PageText
        if type(pageText) ~= "table" or type(pageText.SetText) ~= "function" then
            return
        end

        local currentPage = tonumber(self.currentPage) or 1
        local maxPages = tonumber(self.maxPages) or 1
        local formatString
        local formatted

        if self.displayMaxPages then
            formatString = self.currentPageWithMaxText or PAGE_NUMBER_WITH_MAX
            formatted = string.format(formatString, currentPage, maxPages)
        else
            formatString = self.currentPageOnlyText or PAGE_NUMBER
            formatted = string.format(formatString, currentPage)
        end

        pageText:SetText(formatted)
    end

    rawset(_G, "__wow_paging_controls_update_controls_wrapper", true)
"#;

pub(super) const TALENT_EDGE_FRAME_LEVEL_SYNC_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_talent_edge_frame_level_sync_wrapped") then
        return
    end

    if type(TalentFrameBaseMixin) ~= "table"
        or type(TalentFrameBaseMixin.UpdateButtonFrameLevel) ~= "function"
        or type(TalentFrameBaseMixin.MarkEdgesDirty) ~= "function" then
        return
    end

    local originalUpdateButtonFrameLevel = TalentFrameBaseMixin.UpdateButtonFrameLevel

    TalentFrameBaseMixin.UpdateButtonFrameLevel = function(self, talentButton, ...)
        local oldLevel = (talentButton and talentButton.GetFrameLevel) and talentButton:GetFrameLevel() or nil
        local result = originalUpdateButtonFrameLevel(self, talentButton, ...)
        if not talentButton or type(self) ~= "table" or type(self.MarkEdgesDirty) ~= "function" then
            return result
        end
        local newLevel = talentButton.GetFrameLevel and talentButton:GetFrameLevel() or nil
        if oldLevel ~= nil and newLevel ~= nil and oldLevel ~= newLevel then
            self:MarkEdgesDirty(talentButton)
        end
        return result
    end

    rawset(_G, "__wow_talent_edge_frame_level_sync_wrapped", true)
"#;

pub(super) const OBJECTIVE_TRACKER_QUEST_HEADER_WORKAROUND_LUA: &str = r#"
        local function ensure_quest_header_text()
        local function module_has_visible_contents(module)
            if not module then
                return false
            end
            if module.GetContentsHeight and module:GetContentsHeight() > 0 then
                return true
            end
            local used = module.usedBlocks
            if type(used) ~= "table" then
                return false
            end
            for _, blocks in pairs(used) do
                if type(blocks) == "table" and next(blocks) ~= nil then
                    return true
                end
            end
            return false
        end

        local function resolve_quest_header()
            if QuestObjectiveTracker and QuestObjectiveTracker.Header then
                return QuestObjectiveTracker.Header
            end
            local legacy = ObjectiveTrackerBlocksFrame and ObjectiveTrackerBlocksFrame.QuestHeader
            if not legacy then
                return nil
            end
            return legacy.Header or legacy
        end

        local function set_region_alpha(region, target_alpha)
            if type(region) ~= "table"
                or type(region.GetAlpha) ~= "function"
                or type(region.SetAlpha) ~= "function" then
                return
            end
            local current = region:GetAlpha()
            if type(current) ~= "number" or math.abs(current - target_alpha) > 0.001 then
                region:SetAlpha(target_alpha)
            end
        end

        local function normalize_quest_header_surface(module, header)
            if type(header) ~= "table" then
                return
            end
            -- During startup the AddAnim can leave the quest header in a half-faded
            -- state (dim background + fully lit shine/glow). Force the expanded visuals
            -- once the module has quest content so the texture stack matches Blizzard.
            if module and module.collapsed then
                return
            end
            if not module_has_visible_contents(module) then
                return
            end
            if type(header.AddAnim) == "table" and type(header.AddAnim.Stop) == "function" then
                header.AddAnim:Stop()
            end
            set_region_alpha(header, 1)
            set_region_alpha(header.Background, 1)
            set_region_alpha(header.Shine, 0)
            set_region_alpha(header.Glow, 0)
            set_region_alpha(header.MinimizeButton, 1)
        end

        local module = QuestObjectiveTracker
        local header = resolve_quest_header()
        local textRegion = header and header.Text
        if not textRegion then
            return
        end

        normalize_quest_header_surface(module, header)

        if header.Show and module_has_visible_contents(module) and not header:IsShown() then
            header:Show()
        end

        local text = textRegion.GetText and textRegion:GetText() or nil
        if type(text) ~= "string" or text == "" then
            local fallback = TRACKER_HEADER_QUESTS or QUESTS or "Quests"
            textRegion:SetText(fallback)
        end

        if textRegion.Show and not textRegion:IsShown() then
            textRegion:Show()
        end

        if textRegion.GetAlpha and textRegion.SetAlpha and textRegion:GetAlpha() <= 0 then
            textRegion:SetAlpha(1)
        end

        if textRegion.GetTextColor and textRegion.SetTextColor then
            local r, g, b, a = textRegion:GetTextColor()
            local effectively_black = (r or 0) < 0.02 and (g or 0) < 0.02 and (b or 0) < 0.02
            local fully_transparent = a ~= nil and a <= 0
            if effectively_black or fully_transparent then
                local color =
                    (type(OBJECTIVE_TRACKER_COLOR) == "table" and OBJECTIVE_TRACKER_COLOR["Header"])
                    or NORMAL_FONT_COLOR
                if type(color) == "table" and color.r and color.g and color.b then
                    textRegion:SetTextColor(color.r, color.g, color.b, color.a or 1)
                elseif fully_transparent then
                    textRegion:SetTextColor(r or 1, g or 0.82, b or 0.0, 1)
                end
            end
        end
        normalize_quest_header_surface(module, header)
        end

        if not rawget(_G, "__wow_objective_tracker_quest_header_update_wrapper")
            and ObjectiveTrackerContainerMixin
            and type(ObjectiveTrackerContainerMixin.Update) == "function" then
            local originalUpdate = ObjectiveTrackerContainerMixin.Update
            ObjectiveTrackerContainerMixin.Update = function(self, dirtyUpdate)
                local result = originalUpdate(self, dirtyUpdate)
                pcall(ensure_quest_header_text)
                return result
            end
            rawset(_G, "__wow_objective_tracker_quest_header_update_wrapper", true)
        end

        if not rawget(_G, "__wow_objective_tracker_header_play_add_anim_wrapper")
            and ObjectiveTrackerModuleHeaderMixin
            and type(ObjectiveTrackerModuleHeaderMixin.PlayAddAnimation) == "function" then
            local originalPlayAddAnimation = ObjectiveTrackerModuleHeaderMixin.PlayAddAnimation
            ObjectiveTrackerModuleHeaderMixin.PlayAddAnimation = function(self, ...)
                local result = originalPlayAddAnimation(self, ...)
                local module = self.GetParent and self:GetParent() or nil
                if module == QuestObjectiveTracker then
                    pcall(normalize_quest_header_surface, module, self)
                end
                return result
            end
            rawset(_G, "__wow_objective_tracker_header_play_add_anim_wrapper", true)
        end

        if not rawget(_G, "__wow_objective_tracker_module_end_layout_wrapper")
            and ObjectiveTrackerModuleMixin
            and type(ObjectiveTrackerModuleMixin.EndLayout) == "function" then
            local originalEndLayout = ObjectiveTrackerModuleMixin.EndLayout
            ObjectiveTrackerModuleMixin.EndLayout = function(self, ...)
                local result = originalEndLayout(self, ...)
                if self == QuestObjectiveTracker then
                    pcall(normalize_quest_header_surface, self, self.Header)
                end
                return result
            end
            rawset(_G, "__wow_objective_tracker_module_end_layout_wrapper", true)
        end

        pcall(ensure_quest_header_text)
    "#;

pub(super) fn patch_catalog_shop_product_card_defaults(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(CATALOG_SHOP_PRODUCT_CARD_DEFAULTS_WORKAROUND_LUA);
}

pub(super) fn patch_objective_tracker_quest_header(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(OBJECTIVE_TRACKER_QUEST_HEADER_WORKAROUND_LUA);
}

pub(super) fn patch_fog_of_war_pin_mixin(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(FOG_OF_WAR_PIN_WORKAROUND_LUA);
}

pub(super) fn patch_map_exploration_pin_mixin(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(MAP_EXPLORATION_PIN_WORKAROUND_LUA);
}

pub(super) fn patch_map_canvas_data_provider_attachment(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(MAP_CANVAS_DATA_PROVIDER_WORKAROUND_LUA);
}

pub(super) fn patch_character_create_defaults(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(CHARACTER_CREATE_DEFAULTS_WORKAROUND_LUA);
}

pub(super) fn patch_character_frame_title_refresh(env: &crate::lua_api::WowLuaEnv) {
    refresh_character_frame_surface(env);
}

pub(super) fn refresh_character_frame_surface(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(CHARACTER_FRAME_SURFACE_REFRESH_WORKAROUND_LUA);
}

pub(super) fn patch_fog_of_war_pin_mixin_for_runtime_addon_load(
    env: &crate::lua_api::LoaderEnv<'_>,
) {
    let _ = env.exec(FOG_OF_WAR_PIN_WORKAROUND_LUA);
}

pub(super) fn patch_map_exploration_pin_mixin_for_runtime_addon_load(
    env: &crate::lua_api::LoaderEnv<'_>,
) {
    let _ = env.exec(MAP_EXPLORATION_PIN_WORKAROUND_LUA);
}

pub(super) fn patch_toggle_collections_journal_for_runtime_addon_load(
    env: &crate::lua_api::LoaderEnv<'_>,
) {
    let _ = env.exec(TOGGLE_COLLECTIONS_JOURNAL_LUA);
    let _ = env.exec(MOUNT_JOURNAL_DYNAMIC_FLIGHT_POPUP_WORKAROUND_LUA);
}

pub(super) fn patch_toggle_encounter_journal_for_runtime_addon_load(
    env: &crate::lua_api::LoaderEnv<'_>,
) {
    let _ = env.exec(TOGGLE_ENCOUNTER_JOURNAL_LUA);
}

pub(super) fn patch_map_canvas_data_provider_attachment_for_runtime_addon_load(
    env: &crate::lua_api::LoaderEnv<'_>,
) {
    let _ = env.exec(MAP_CANVAS_DATA_PROVIDER_WORKAROUND_LUA);
}

pub(super) fn ensure_adventure_map_frame_surface(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(ADVENTURE_MAP_FRAME_SURFACE_LUA);
}

pub(super) fn ensure_adventure_map_frame_surface_for_runtime_addon_load(
    env: &crate::lua_api::LoaderEnv<'_>,
) {
    let _ = env.exec(ADVENTURE_MAP_FRAME_SURFACE_LUA);
}

pub(super) fn patch_item_quality_color_data_methods(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(ITEM_QUALITY_COLOR_DATA_METHODS_WORKAROUND_LUA);
}

pub(super) fn patch_artifact_ui_show_panel_guard(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(ARTIFACT_UI_SHOW_PANEL_GUARD_WORKAROUND_LUA);
}

pub(super) fn patch_auction_house_categories_refresh_count(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(AUCTION_HOUSE_CATEGORIES_REFRESH_COUNT_WORKAROUND_LUA);
}

pub(super) fn patch_auction_house_browse_results_event(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(AUCTION_HOUSE_BROWSE_RESULTS_EVENT_WORKAROUND_LUA);
}

pub(super) fn patch_auction_house_browse_results_event_from_env(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(AUCTION_HOUSE_BROWSE_RESULTS_EVENT_WORKAROUND_LUA);
}

pub(super) fn patch_auction_house_search_context_aliases(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(AUCTION_HOUSE_SEARCH_CONTEXT_ALIASES_WORKAROUND_LUA);
}

pub(super) fn patch_auction_house_search_context_aliases_from_env(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(AUCTION_HOUSE_SEARCH_CONTEXT_ALIASES_WORKAROUND_LUA);
}

pub(super) fn patch_auth_challenge_frame_parent(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(AUTH_CHALLENGE_FRAME_PARENT_WORKAROUND_LUA);
}

pub(super) fn patch_auth_challenge_frame_parent_from_env(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(AUTH_CHALLENGE_FRAME_PARENT_WORKAROUND_LUA);
}

pub(crate) fn patch_account_store_set_storefront(
    env: &crate::lua_api::LoaderEnv<'_>,
) -> Result<(), crate::Error> {
    env.exec(
        r#"
        local function __wow_account_store_set_storefront_id(self, storeFrontID)
            self.storeFrontID = storeFrontID
        end

        if type(AccountStoreMixin) == "table" then
            AccountStoreMixin.SetStoreFrontID = __wow_account_store_set_storefront_id
        end
        if type(AccountStoreFrame) == "table" then
            AccountStoreFrame.SetStoreFrontID = __wow_account_store_set_storefront_id
        end
        "#,
    )
}
