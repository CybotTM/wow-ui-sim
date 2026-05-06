//! Post-load workarounds that are still required on the live rilua path.

use std::time::Instant;

const CATALOG_SHOP_PRODUCT_CARD_DEFAULTS_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_catalog_shop_product_card_defaults_wrapped") then
        return
    end

    if type(CatalogShopDefaultProductCardMixin) ~= "table"
        or type(CatalogShopDefaultProductCardMixin.Layout) ~= "function" then
        return
    end

    local original_layout = CatalogShopDefaultProductCardMixin.Layout

    local function resolve_product_id(card)
        if type(card.productInfo) == "table"
            and type(card.productInfo.catalogShopProductID) == "number" then
            return card.productInfo.catalogShopProductID
        end

        if type(card.GetElementData) == "function" then
            local elementData = card:GetElementData()
            if type(elementData) == "table" then
                local productID = elementData.catalogShopProductID or elementData.productID
                if type(productID) == "number" then
                    if type(card.productInfo) == "table" then
                        card.productInfo.catalogShopProductID = productID
                    end
                    return productID
                end
            end
        end

        return nil
    end

    CatalogShopDefaultProductCardMixin.Layout = function(self, ...)
        if resolve_product_id(self) == nil then
            return
        end
        return original_layout(self, ...)
    end

    rawset(_G, "__wow_catalog_shop_product_card_defaults_wrapped", true)
"#;

const ITEM_QUALITY_COLOR_DATA_METHODS_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_item_quality_color_data_methods_wrapped") then
        return
    end

    local function ensureColorDataMethods(colorData)
        if type(colorData) ~= "table" then
            return
        end

        if type(colorData.GetRGB) ~= "function" then
            function colorData:GetRGB()
                return self.r, self.g, self.b
            end
        end

        if type(colorData.GetRGBA) ~= "function" then
            function colorData:GetRGBA()
                return self.r, self.g, self.b, self.a or 1
            end
        end
    end

    local function ensureAllItemQualityColorMethods()
        if type(ITEM_QUALITY_COLORS) ~= "table" then
            return
        end

        for _, colorData in pairs(ITEM_QUALITY_COLORS) do
            ensureColorDataMethods(colorData)
        end
    end

    ensureAllItemQualityColorMethods()

    if type(ColorManager) == "table" and type(ColorManager.UpdateColorsForItemQuality) == "function" then
        local originalUpdateColorsForItemQuality = ColorManager.UpdateColorsForItemQuality
        function ColorManager.UpdateColorsForItemQuality(...)
            originalUpdateColorsForItemQuality(...)
            ensureAllItemQualityColorMethods()
        end
    end

    rawset(_G, "__wow_item_quality_color_data_methods_wrapped", true)
"#;

const ARTIFACT_UI_SHOW_PANEL_GUARD_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_artifact_ui_show_panel_guard_wrapped") then
        return
    end

    if type(ShowUIPanel) ~= "function" then
        return
    end

    local originalShowUIPanel = ShowUIPanel

    local function shouldBlockArtifactPanel(frame)
        return frame == ArtifactFrame
            and type(ArtifactUI_CanViewArtifact) == "function"
            and not ArtifactUI_CanViewArtifact()
    end

    local function callArtifactShowFailedFunc()
        local entry = type(UIPanelWindows) == "table" and UIPanelWindows["ArtifactFrame"] or nil
        local showFailedFunc = type(entry) == "table" and entry.showFailedFunc or nil
        if type(showFailedFunc) == "function" then
            showFailedFunc()
        end
    end

    ShowUIPanel = function(frame, ...)
        if frame and frame:IsShown() then
            return originalShowUIPanel(frame, ...)
        end

        if shouldBlockArtifactPanel(frame) then
            callArtifactShowFailedFunc()
            return
        end

        return originalShowUIPanel(frame, ...)
    end

    rawset(_G, "__wow_artifact_ui_show_panel_guard_wrapped", true)
"#;

const AUCTION_HOUSE_CATEGORIES_REFRESH_COUNT_WORKAROUND_LUA: &str = r#"
    local function getNumElementsForRefresh()
        return type(AuctionCategories) == "table" and #AuctionCategories or 0
    end

    local categoriesList = AuctionHouseFrame and AuctionHouseFrame.CategoriesList or nil
    if type(categoriesList) == "table"
        and type(categoriesList.GetNumElementsForRefresh) ~= "function" then
        categoriesList.GetNumElementsForRefresh = getNumElementsForRefresh
    end

    if type(AuctionHouseCategoriesListMixin) ~= "table" then
        return
    end

    if type(AuctionHouseCategoriesListMixin.GetNumElementsForRefresh) == "function" then
        return
    end

    AuctionHouseCategoriesListMixin.GetNumElementsForRefresh = getNumElementsForRefresh
"#;

const AUCTION_HOUSE_BROWSE_RESULTS_EVENT_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_auction_house_browse_results_event_wrapped") then
        return
    end

    if type(AuctionHouseFrameMixin) ~= "table" then
        return
    end

    local browseResultsEvent = "AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"

    local function registerBrowseResultsEvent(frame)
        if type(frame) == "table" and type(frame.RegisterEvent) == "function" then
            frame:RegisterEvent(browseResultsEvent)
        end
    end

    local function unregisterBrowseResultsEvent(frame)
        if type(frame) == "table" and type(frame.UnregisterEvent) == "function" then
            frame:UnregisterEvent(browseResultsEvent)
        end
    end

    local originalOnShow = AuctionHouseFrameMixin.OnShow
    local originalOnHide = AuctionHouseFrameMixin.OnHide

    AuctionHouseFrameMixin.OnShow = function(self, ...)
        if type(originalOnShow) == "function" then
            originalOnShow(self, ...)
        end
        registerBrowseResultsEvent(self)
    end

    AuctionHouseFrameMixin.OnHide = function(self, ...)
        unregisterBrowseResultsEvent(self)
        if type(originalOnHide) == "function" then
            originalOnHide(self, ...)
        end
    end

    local frame = AuctionHouseFrame
    if type(frame) == "table" then
        local frameOnShow = frame:GetScript("OnShow")
        frame:SetScript("OnShow", function(self, ...)
            if type(frameOnShow) == "function" then
                frameOnShow(self, ...)
            end
            registerBrowseResultsEvent(self)
        end)

        local frameOnHide = frame:GetScript("OnHide")
        frame:SetScript("OnHide", function(self, ...)
            unregisterBrowseResultsEvent(self)
            if type(frameOnHide) == "function" then
                frameOnHide(self, ...)
            end
        end)

        registerBrowseResultsEvent(frame)
    end

    rawset(_G, "__wow_auction_house_browse_results_event_wrapped", true)
"#;

const AUCTION_HOUSE_SEARCH_CONTEXT_ALIASES_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_auction_house_search_context_aliases_patched") then
        return
    end

    if type(AuctionHouseSearchContext) ~= "table" then
        return
    end

    if AuctionHouseSearchContext.Auctions == nil then
        AuctionHouseSearchContext.Auctions = AuctionHouseSearchContext.AllAuctions
    end

    if AuctionHouseSearchContext.BrowseFavorites == nil then
        AuctionHouseSearchContext.BrowseFavorites = AuctionHouseSearchContext.AllFavorites
    end

    rawset(_G, "__wow_auction_house_search_context_aliases_patched", true)
"#;

const AUTH_CHALLENGE_FRAME_PARENT_WORKAROUND_LUA: &str = r#"
    local authChallengeFunctions = {
        "AuthChallengeUI_OnLoad",
        "AuthChallengeUI_Submit",
        "AuthChallengeUI_Cancel",
        "AuthChallengeUI_OnTabPressed",
        "AuthChallengeUI_OnKeyDown",
    }

    for _, functionName in ipairs(authChallengeFunctions) do
        if rawget(_G, functionName) == nil
            and type(__secureenv) == "table"
            and type(rawget(__secureenv, functionName)) == "function" then
            rawset(_G, functionName, rawget(__secureenv, functionName))
        end
    end

    if type(AuthChallengeFrame) ~= "table" or type(UIParent) ~= "table" then
        return
    end

    if AuthChallengeFrame:GetParent() ~= UIParent then
        AuthChallengeFrame:SetParent(UIParent)
    end

    local inputFrame = AuthChallengeFrame.InputFrame
    if inputFrame and inputFrame.Submit == nil and type(inputFrame.GetChildren) == "function" then
        for _, child in ipairs({ inputFrame:GetChildren() }) do
            if type(child.GetObjectType) == "function"
                and child:GetObjectType() == "Button"
                and type(child.GetText) == "function"
                and child:GetText() == BLIZZARD_CHALLENGE_SUBMIT then
                inputFrame.Submit = child
                break
            end
        end
    end
"#;

struct WorkaroundStep {
    label: &'static str,
    apply: fn(&crate::lua_api::WowLuaEnv),
}

const POST_LOAD_WORKAROUNDS: &[WorkaroundStep] = &[
    WorkaroundStep {
        label: "patch_edit_mode_manager",
        apply: patch_edit_mode_manager,
    },
    WorkaroundStep {
        label: "init_edit_mode_layout",
        apply: init_edit_mode_layout,
    },
    WorkaroundStep {
        label: "patch_ui_parent_panel_toggles",
        apply: patch_ui_parent_panel_toggles,
    },
    WorkaroundStep {
        label: "patch_uiparent_onupdate_worklists",
        apply: patch_uiparent_onupdate_worklists,
    },
    WorkaroundStep {
        label: "init_chat_type_colors",
        apply: init_chat_type_colors,
    },
    WorkaroundStep {
        label: "patch_chat_voice_button_surface",
        apply: patch_chat_voice_button_surface,
    },
    WorkaroundStep {
        label: "patch_item_socketing_tooltips",
        apply: patch_item_socketing_tooltips,
    },
    WorkaroundStep {
        label: "patch_character_select_selected_name",
        apply: patch_character_select_selected_name,
    },
    WorkaroundStep {
        label: "patch_character_create_defaults",
        apply: patch_character_create_defaults,
    },
    WorkaroundStep {
        label: "patch_character_frame_title_refresh",
        apply: patch_character_frame_title_refresh,
    },
    WorkaroundStep {
        label: "patch_vignette_pin_template",
        apply: patch_vignette_pin_template,
    },
    WorkaroundStep {
        label: "patch_fog_of_war_pin_mixin",
        apply: patch_fog_of_war_pin_mixin,
    },
    WorkaroundStep {
        label: "patch_map_exploration_pin_mixin",
        apply: patch_map_exploration_pin_mixin,
    },
    WorkaroundStep {
        label: "patch_map_canvas_data_provider_attachment",
        apply: patch_map_canvas_data_provider_attachment,
    },
    WorkaroundStep {
        label: "ensure_adventure_map_frame_surface",
        apply: ensure_adventure_map_frame_surface,
    },
    WorkaroundStep {
        label: "patch_action_bar_button_event_fanout",
        apply: patch_action_bar_button_event_fanout,
    },
    WorkaroundStep {
        label: "patch_paging_controls_page_text",
        apply: patch_paging_controls_page_text,
    },
    WorkaroundStep {
        label: "patch_talent_edge_frame_level_sync",
        apply: patch_talent_edge_frame_level_sync,
    },
    WorkaroundStep {
        label: "patch_catalog_shop_product_card_defaults",
        apply: patch_catalog_shop_product_card_defaults,
    },
    WorkaroundStep {
        label: "patch_game_time_defaults",
        apply: patch_game_time_defaults,
    },
    WorkaroundStep {
        label: "patch_tooltip_nineslice_surface",
        apply: patch_tooltip_nineslice_surface,
    },
    WorkaroundStep {
        label: "patch_container_frame_token_tracker",
        apply: patch_container_frame_token_tracker,
    },
    WorkaroundStep {
        label: "patch_achievement_display_set_achievements",
        apply: patch_achievement_display_set_achievements,
    },
    WorkaroundStep {
        label: "patch_housing_dashboard_preload",
        apply: patch_housing_dashboard_preload_from_env,
    },
    WorkaroundStep {
        label: "patch_lfg_lock_list",
        apply: patch_lfg_lock_list,
    },
    WorkaroundStep {
        label: "patch_auction_house_browse_results_event",
        apply: patch_auction_house_browse_results_event_from_env,
    },
    WorkaroundStep {
        label: "patch_auction_house_search_context_aliases",
        apply: patch_auction_house_search_context_aliases_from_env,
    },
    WorkaroundStep {
        label: "patch_auth_challenge_frame_parent",
        apply: patch_auth_challenge_frame_parent_from_env,
    },
];

pub fn apply(env: &crate::lua_api::WowLuaEnv) {
    for step in POST_LOAD_WORKAROUNDS {
        log_step(env, step.label, || (step.apply)(env));
    }
}

fn patch_edit_mode_manager(env: &crate::lua_api::WowLuaEnv) {
    crate::lua_api::workarounds_editmode::patch_edit_mode_manager(env);
}

fn init_edit_mode_layout(env: &crate::lua_api::WowLuaEnv) {
    crate::lua_api::workarounds_editmode::init_edit_mode_layout(env);
}

fn init_chat_type_colors(env: &crate::lua_api::WowLuaEnv) {
    crate::lua_api::chat_init::init_chat_type_colors(env);
}

fn patch_housing_dashboard_preload_from_env(env: &crate::lua_api::WowLuaEnv) {
    patch_housing_dashboard_preload(&env.loader_env());
}

pub fn apply_post_event(env: &crate::lua_api::WowLuaEnv) {
    apply_post_event_bootstrap(env);
    patch_post_event_frame_layout(env);
    refresh_post_event_surfaces(env);
}

fn apply_post_event_bootstrap(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(REFRESH_ACTION_BUTTONS_LUA);
    crate::lua_api::workarounds_editmode::init_edit_mode_layout(env);
    crate::lua_api::workarounds_editmode::reapply_player_frame_anchor(env);
    crate::lua_api::chat_init::init_chat_type_colors(env);
    crate::lua_api::chat_init::show_chat_frame(env);
}

fn patch_post_event_frame_layout(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(POST_EVENT_FRAME_LAYOUT_WORKAROUND_LUA);
}

fn refresh_post_event_surfaces(env: &crate::lua_api::WowLuaEnv) {
    refresh_character_frame_surface(env);
    patch_chat_voice_button_surface(env);
    patch_objective_tracker_quest_header(env);
}

pub fn apply_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if addon_name == "Blizzard_PagedContent" {
        let _ = env.exec(PAGING_CONTROLS_PAGE_TEXT_WORKAROUND_LUA);
    }
    if matches!(
        addon_name,
        "Blizzard_SharedTalentUI" | "Blizzard_PlayerSpells"
    ) {
        let _ = env.exec(TALENT_EDGE_FRAME_LEVEL_SYNC_WORKAROUND_LUA);
    }
    patch_runtime_map_addon_surfaces(env, addon_name);
    if addon_name == "Blizzard_Collections" {
        patch_toggle_collections_journal_for_runtime_addon_load(env);
        patch_collections_journal_namespace(env);
    }
    if addon_name == "Blizzard_EncounterJournal" {
        patch_toggle_encounter_journal_for_runtime_addon_load(env);
    }
    if addon_name == "Blizzard_AdventureMap" {
        ensure_adventure_map_frame_surface_for_runtime_addon_load(env);
    }
    if matches!(addon_name, "Blizzard_ArtifactUI" | "Blizzard_Colors") {
        patch_item_quality_color_data_methods(env);
    }
    if addon_name == "Blizzard_ArtifactUI" {
        patch_artifact_ui_show_panel_guard(env);
    }
    if addon_name == "Blizzard_AuctionHouseUI" {
        patch_auction_house_runtime_surface(env);
    }
    if addon_name == "Blizzard_AuthChallengeUI" {
        patch_auth_challenge_frame_parent(env);
    }
    if addon_name == "Blizzard_AccountStore" {
        let _ = patch_account_store_set_storefront(env);
    }
    if addon_name == "Blizzard_CatalogShop" {
        let _ = env.exec(CATALOG_SHOP_PRODUCT_CARD_DEFAULTS_WORKAROUND_LUA);
    }
    if addon_name == "Blizzard_DamageMeter" {
        patch_damage_meter_initial_scrollbox_extent(env);
    }
}

fn patch_auction_house_runtime_surface(env: &crate::lua_api::LoaderEnv<'_>) {
    patch_auction_house_categories_refresh_count(env);
    patch_auction_house_browse_results_event(env);
    patch_auction_house_search_context_aliases(env);
}

fn patch_runtime_map_addon_surfaces(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if addon_name == "Blizzard_MapCanvas" {
        patch_map_canvas_scroll_container(env);
    }
    if matches!(
        addon_name,
        "Blizzard_MapCanvas"
            | "Blizzard_SharedMapDataProviders"
            | "Blizzard_WorldMap"
            | "Blizzard_BattlefieldMap"
    ) {
        patch_fog_of_war_pin_mixin_for_runtime_addon_load(env);
        patch_map_exploration_pin_mixin_for_runtime_addon_load(env);
        patch_map_canvas_data_provider_attachment_for_runtime_addon_load(env);
    }
}

pub fn apply_for_runtime_addon_preload(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if addon_name == "Blizzard_Collections" {
        patch_collections_journal_namespace(env);
    }
    if matches!(
        addon_name,
        "Blizzard_HousingDashboard" | "Blizzard_HousingHouseFinder"
    ) {
        patch_housing_dashboard_preload(env);
    }
}

fn log_with_timestamp(env: &crate::lua_api::WowLuaEnv, message: &str) {
    let start_time = env.state().borrow().start_time;
    eprintln!("{} {}", crate::logging::elapsed_prefix(start_time), message);
}

fn log_step(env: &crate::lua_api::WowLuaEnv, label: &str, apply_step: impl FnOnce()) {
    log_with_timestamp(env, &format!("[Workarounds] starting {label}"));
    let started = Instant::now();
    apply_step();
    log_with_timestamp(
        env,
        &format!(
            "[Workarounds] finished {label} in {:.2?}",
            started.elapsed()
        ),
    );
}

fn patch_ui_parent_panel_toggles(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(GETGLOBAL_HELPER_LUA);
    let _ = env.exec(TOGGLE_ACHIEVEMENT_FRAME_LUA);
    let _ = env.exec(TOGGLE_ENCOUNTER_JOURNAL_LUA);
    let _ = env.exec(TOGGLE_COLLECTIONS_JOURNAL_LUA);
    let _ = env.exec(MAIN_MENU_MICROBUTTON_CLICK_WORKAROUND_LUA);
}

fn patch_damage_meter_initial_scrollbox_extent(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(DAMAGE_METER_INITIAL_SCROLLBOX_EXTENT_LUA);
}

fn patch_housing_dashboard_preload(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(
        r#"
        HousingTutorialUtil = HousingTutorialUtil or {}
        if type(HousingTutorialUtil.BoughtHouseQuestComplete) ~= "function" then
            function HousingTutorialUtil.BoughtHouseQuestComplete()
                return true
            end
        end

        if type(C_Housing) == "table" then
            function C_Housing.GetPlayerOwnedHouses()
                if type(FireEvent) == "function" then
                    FireEvent("PLAYER_HOUSE_LIST_UPDATED", {})
                end
            end

            if type(ClearCachedActivitiesForPlayer) ~= "function" then
                function ClearCachedActivitiesForPlayer() end
            end

            local HOUSING_SIM_NEIGHBORHOODS = {
                {
                    neighborhoodGUID = "wow-ui-sim-neighborhood-dawnmeadow",
                    neighborhoodName = "Dawnmeadow",
                    neighborhoodType = Enum.NeighborhoodType.Public,
                    neighborhoodOwnerType = Enum.NeighborhoodOwnerType.None,
                    suggestionReason = Enum.HouseFinderSuggestionReason.None,
                },
                {
                    neighborhoodGUID = "wow-ui-sim-neighborhood-umber-grove",
                    neighborhoodName = "Umber Grove",
                    neighborhoodType = Enum.NeighborhoodType.Public,
                    neighborhoodOwnerType = Enum.NeighborhoodOwnerType.None,
                    suggestionReason = Enum.HouseFinderSuggestionReason.None,
                },
            }

            local HOUSING_SIM_MAP_IDS = {
                ["wow-ui-sim-neighborhood-dawnmeadow"] = 1,
                ["wow-ui-sim-neighborhood-umber-grove"] = 2248,
            }

            local HOUSING_SIM_TEXTURE_SUFFIXES = {
                ["wow-ui-sim-neighborhood-dawnmeadow"] = "elwynn",
                ["wow-ui-sim-neighborhood-umber-grove"] = "durotar",
            }

            function C_Housing.HouseFinderRequestNeighborhoods()
                if HouseFinderFrame and type(HouseFinderFrame.OnEvent) == "function" then
                    HouseFinderFrame:OnEvent("NEIGHBORHOOD_LIST_UPDATED", Enum.HousingResult.Success, HOUSING_SIM_NEIGHBORHOODS)
                end
                if type(FireEvent) == "function" then
                    FireEvent("NEIGHBORHOOD_LIST_UPDATED", Enum.HousingResult.Success, HOUSING_SIM_NEIGHBORHOODS)
                end
                local firstNeighborhood = HOUSING_SIM_NEIGHBORHOODS[1]
                if firstNeighborhood then
                    C_Housing.RequestHouseFinderNeighborhoodData(firstNeighborhood.neighborhoodGUID, firstNeighborhood.neighborhoodName)
                end
            end

            function C_Housing.GetUIMapIDForNeighborhood(neighborhoodGUID)
                return HOUSING_SIM_MAP_IDS[neighborhoodGUID]
            end

            function C_Housing.GetNeighborhoodTextureSuffix(neighborhoodGUID)
                return HOUSING_SIM_TEXTURE_SUFFIXES[neighborhoodGUID]
            end

            function C_Housing.DoesFactionMatchNeighborhood(neighborhoodGUID)
                return true
            end

            function C_Housing.RequestHouseFinderNeighborhoodData(neighborhoodGUID, neighborhoodName)
                local mapPlotData = {
                    {
                        mapPosition = { x = 0.35, y = 0.46 },
                        ownerType = Enum.HousingPlotOwnerType.None,
                        plotID = 1,
                        plotCost = 100000,
                    },
                    {
                        mapPosition = { x = 0.62, y = 0.52 },
                        ownerName = "Simfriend",
                        ownerType = Enum.HousingPlotOwnerType.Friend,
                        plotID = 2,
                    },
                }
                local function dispatchNeighborhoodData()
                    if HouseFinderFrame and type(HouseFinderFrame.OnEvent) == "function" then
                        HouseFinderFrame:OnEvent("HOUSE_FINDER_NEIGHBORHOOD_DATA_RECIEVED", mapPlotData)
                    end
                    if type(FireEvent) == "function" then
                        FireEvent("HOUSE_FINDER_NEIGHBORHOOD_DATA_RECIEVED", mapPlotData)
                    end
                end
                if type(C_Timer) == "table" and type(C_Timer.After) == "function" then
                    C_Timer.After(0, dispatchNeighborhoodData)
                else
                    dispatchNeighborhoodData()
                end
            end

            function C_Housing.StartTutorial()
                if type(C_AddOns) == "table" and type(C_AddOns.LoadAddOn) == "function" then
                    if not AUTOCOMPLETE_LIST or not AUTOCOMPLETE_LIST.HOUSE_FINDER then
                        C_AddOns.LoadAddOn("Blizzard_AutoComplete")
                    end
                end
                if not HouseFinderFrame and type(C_AddOns) == "table" and type(C_AddOns.LoadAddOn) == "function" then
                    C_AddOns.LoadAddOn("Blizzard_HousingHouseFinder")
                end
                if HouseFinderFrame and type(ShowUIPanel) == "function" then
                    ShowUIPanel(HouseFinderFrame)
                end
                if HouseFinderFrame and not HouseFinderFrame.hasNeighborhoodList then
                    C_Housing.HouseFinderRequestNeighborhoods()
                end
                if HousingDashboardFrame and type(HideUIPanel) == "function" then
                    HideUIPanel(HousingDashboardFrame)
                end
                return true
            end
        end
    "#,
    );
}

fn patch_uiparent_onupdate_worklists(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(UIPARENT_ONUPDATE_WORKLISTS_WORKAROUND_LUA);
}

fn patch_vignette_pin_template(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(VIGNETTE_PIN_TEMPLATE_WORKAROUND_LUA);
}

fn patch_character_select_selected_name(env: &crate::lua_api::WowLuaEnv) {
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

fn patch_chat_voice_button_surface(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(CHAT_VOICE_BUTTON_SURFACE_WORKAROUND_LUA);
}

fn patch_item_socketing_tooltips(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(
        r#"
        local frame = ItemSocketingFrame
        local container = frame and frame.SocketingContainer
        if type(container) ~= "table" then
            return
        end

        local function install_socket_on_enter(socket, socketIndex)
            if type(socket) ~= "table" or type(socket.SetScript) ~= "function" then
                return
            end
            socket:SetScript("OnEnter", function(self)
                if type(GameTooltip) ~= "table" then
                    return
                end
                if type(GameTooltip.SetOwner) == "function" then
                    GameTooltip:SetOwner(self, "ANCHOR_RIGHT")
                end
                if type(GameTooltip.SetSocketGem) == "function" then
                    GameTooltip:SetSocketGem(socketIndex)
                end
                if type(GameTooltip.NumLines) == "function"
                    and GameTooltip:NumLines() == 0
                    and type(GameTooltip.AddLine) == "function" then
                    GameTooltip:AddLine("Socket Gem " .. tostring(socketIndex))
                end
                if type(GameTooltip.Show) == "function" then
                    GameTooltip:Show()
                end
            end)
        end

        install_socket_on_enter(container.Socket1, 1)
        install_socket_on_enter(container.Socket2, 2)
        install_socket_on_enter(container.Socket3, 3)
        "#,
    );
}

fn patch_action_bar_button_event_fanout(env: &crate::lua_api::WowLuaEnv) {
    let trace_fanout = std::env::var_os("WOW_SIM_TRACE_ACTIONBAR_BUTTON_FANOUT").is_some();
    let script = format!(
        r##"
        if type(ActionBarButtonEventsFrameMixin) ~= "table" then
            return
        end

        local traceFanout = {trace_fanout}

        local function button_label(frame, index)
            if type(frame) ~= "table" then
                return "#" .. tostring(index)
            end
            if type(frame.GetName) == "function" then
                local name = frame:GetName()
                if name ~= nil then
                    return name
                end
            end
            if frame.action ~= nil then
                return "action:" .. tostring(frame.action)
            end
            return "#" .. tostring(index)
        end

        local function for_each_button_frame(self, func)
            local frames = self.frames
            if type(frames) ~= "table" then
                return
            end
            for i = 1, #frames do
                local frame = rawget(frames, i)
                if frame ~= nil then
                    if traceFanout then
                        print("[ActionBarFanout] begin " .. button_label(frame, i))
                    end
                    func(frame)
                    if traceFanout then
                        print("[ActionBarFanout] end " .. button_label(frame, i))
                    end
                end
            end
        end

        local function on_event(self, event, ...)
            for_each_button_frame(self, function(frame)
                frame:OnEvent(event, ...)
            end)
            if event == "ACTIONBAR_SLOT_CHANGED" or event == "ACTIONBAR_UPDATE_STATE" then
                for_each_button_frame(self, function(frame)
                    if type(frame.UpdateButtonArt) == "function" then
                        pcall(frame.UpdateButtonArt, frame)
                    end
                end)
            end
        end

        local function on_countdown_for_cooldowns_changed(self)
            for_each_button_frame(self, function(frame)
                ActionButton_UpdateCooldownNumberHidden(frame)
            end)
        end

        local function for_each_frame(self, func)
            for_each_button_frame(self, func)
        end

        ActionBarButtonEventsFrameMixin.OnEvent = on_event
        ActionBarButtonEventsFrameMixin.OnCountdownForCooldownsChanged = on_countdown_for_cooldowns_changed
        ActionBarButtonEventsFrameMixin.ForEachFrame = for_each_frame

        if type(ActionBarButtonEventsFrame) == "table" then
            ActionBarButtonEventsFrame.OnEvent = on_event
            ActionBarButtonEventsFrame.OnCountdownForCooldownsChanged = on_countdown_for_cooldowns_changed
            ActionBarButtonEventsFrame.ForEachFrame = for_each_frame
            if type(ActionBarButtonEventsFrame.SetScript) == "function" then
                ActionBarButtonEventsFrame:SetScript("OnEvent", on_event)
            end
        end
        "##,
        trace_fanout = if trace_fanout { "true" } else { "false" },
    );
    let _ = env.exec(&script);
}

fn patch_game_time_defaults(env: &crate::lua_api::WowLuaEnv) {
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
fn patch_lfg_lock_list(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(
        r#"
        if type(GetLFGLockList) == "function" and LFGLockList == nil then
            LFGLockList = GetLFGLockList()
        end
        "#,
    );
}

fn patch_tooltip_nineslice_surface(env: &crate::lua_api::WowLuaEnv) {
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

fn patch_container_frame_token_tracker(env: &crate::lua_api::WowLuaEnv) {
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

fn patch_paging_controls_page_text(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(PAGING_CONTROLS_PAGE_TEXT_WORKAROUND_LUA);
}

fn patch_achievement_display_set_achievements(env: &crate::lua_api::WowLuaEnv) {
    // Blizzard_FrameXML/AchievementDisplayFrame.lua reassigns
    // `AchievementDisplayMixin = {}` and re-defines `:SetAchievements`
    // on top of the bootstrap stub. The live body iterates through
    // a frame pool and reads `GetAchievementInfo` per criteria — both
    // out of scope for a 2D-only simulator. Reinstate the stub so the
    // AlliedRaces panel call site doesn't error.
    let _ = env.exec(ACHIEVEMENT_DISPLAY_SET_ACHIEVEMENTS_WORKAROUND_LUA);
}

const ACHIEVEMENT_DISPLAY_SET_ACHIEVEMENTS_WORKAROUND_LUA: &str = r#"
    if type(AchievementDisplayMixin) ~= "table" then
        AchievementDisplayMixin = {}
    end
    AchievementDisplayMixin.SetAchievements = function(self, achievementIds)
        self.achievementIds = achievementIds
    end
"#;

fn patch_talent_edge_frame_level_sync(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(TALENT_EDGE_FRAME_LEVEL_SYNC_WORKAROUND_LUA);
}

const PAGING_CONTROLS_PAGE_TEXT_WORKAROUND_LUA: &str = r#"
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

const TALENT_EDGE_FRAME_LEVEL_SYNC_WORKAROUND_LUA: &str = r#"
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

const OBJECTIVE_TRACKER_QUEST_HEADER_WORKAROUND_LUA: &str = r#"
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

fn patch_catalog_shop_product_card_defaults(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(CATALOG_SHOP_PRODUCT_CARD_DEFAULTS_WORKAROUND_LUA);
}

fn patch_objective_tracker_quest_header(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(OBJECTIVE_TRACKER_QUEST_HEADER_WORKAROUND_LUA);
}

fn patch_fog_of_war_pin_mixin(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(FOG_OF_WAR_PIN_WORKAROUND_LUA);
}

fn patch_map_exploration_pin_mixin(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(MAP_EXPLORATION_PIN_WORKAROUND_LUA);
}

fn patch_map_canvas_data_provider_attachment(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(MAP_CANVAS_DATA_PROVIDER_WORKAROUND_LUA);
}

fn patch_character_create_defaults(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(CHARACTER_CREATE_DEFAULTS_WORKAROUND_LUA);
}

fn patch_character_frame_title_refresh(env: &crate::lua_api::WowLuaEnv) {
    refresh_character_frame_surface(env);
}

fn refresh_character_frame_surface(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(
        r#"
        local function get_character_panel_slot_buttons()
            local slotFrameNames = {
                "CharacterHeadSlot",
                "CharacterNeckSlot",
                "CharacterShoulderSlot",
                "CharacterBackSlot",
                "CharacterChestSlot",
                "CharacterShirtSlot",
                "CharacterTabardSlot",
                "CharacterWristSlot",
                "CharacterHandsSlot",
                "CharacterWaistSlot",
                "CharacterLegsSlot",
                "CharacterFeetSlot",
                "CharacterFinger0Slot",
                "CharacterFinger1Slot",
                "CharacterTrinket0Slot",
                "CharacterTrinket1Slot",
                "CharacterMainHandSlot",
                "CharacterSecondaryHandSlot",
            }
            local buttons = {}
            for _, frameName in ipairs(slotFrameNames) do
                local button = _G[frameName]
                if type(button) == "table" then
                    table.insert(buttons, button)
                end
            end
            return buttons
        end

        if type(CharacterFrame) == "table"
            and type(CharacterFrame.GetScript) == "function"
            and type(CharacterFrame.SetScript) == "function" then
            local existing_wrapper = rawget(_G, "__wow_character_frame_onshow_wrapper")
            if CharacterFrame:GetScript("OnShow") ~= existing_wrapper then
                local original_on_show = CharacterFrame:GetScript("OnShow")
                if type(original_on_show) ~= "function" then
                    return
                end
                local wrapper = function(self, ...)
                    original_on_show(self, ...)
                    if type(self.UpdateTitle) == "function" then
                        self:UpdateTitle()
                    end
                    if type(PaperDollItemSlotButton_Update) == "function" then
                        for _, button in ipairs(get_character_panel_slot_buttons()) do
                            PaperDollItemSlotButton_Update(button)
                        end
                    end
                end
                CharacterFrame:SetScript("OnShow", wrapper)
                rawset(_G, "__wow_character_frame_onshow_wrapper", wrapper)
            end
        end

        if type(CharacterFrame) == "table" and type(CharacterFrame.RefreshDisplay) == "function" then
            local existing_wrapper = rawget(_G, "__wow_character_frame_refresh_display_wrapper")
            if CharacterFrame.RefreshDisplay ~= existing_wrapper then
                local original_refresh_display = CharacterFrame.RefreshDisplay
                local wrapper = function(self, ...)
                    original_refresh_display(self, ...)
                    if type(self.UpdateTitle) == "function" then
                        self:UpdateTitle()
                    end
                    if type(PaperDollItemSlotButton_Update) == "function" then
                        for _, button in ipairs(get_character_panel_slot_buttons()) do
                            PaperDollItemSlotButton_Update(button)
                        end
                    end
                end
                CharacterFrame.RefreshDisplay = wrapper
                rawset(_G, "__wow_character_frame_refresh_display_wrapper", wrapper)
            end
        end

        if type(CharacterFrame) == "table" then
            if type(CharacterFrame.RefreshDisplay) == "function" then
                CharacterFrame:RefreshDisplay()
            elseif type(CharacterFrame.UpdateTitle) == "function" then
                CharacterFrame:UpdateTitle()
            end
        end

        if type(PaperDollItemSlotButton_Update) == "function" then
            for _, button in ipairs(get_character_panel_slot_buttons()) do
                PaperDollItemSlotButton_Update(button)
            end
        end

        if type(CharacterFrame) == "table"
            and CharacterFrame.TitleContainer
            and CharacterFrame.TitleContainer.TitleText
            and type(CharacterFrame.TitleContainer.TitleText.SetText) == "function" then
            CharacterFrame.TitleContainer.TitleText:SetText(UnitPVPName("player"))
        end

        for _, button in ipairs(get_character_panel_slot_buttons()) do
            if type(button.icon) == "table" then
                local textureName = GetInventoryItemTexture("player", button:GetID())
                if textureName ~= nil then
                    button.icon:SetTexture(textureName)
                elseif button.backgroundTextureName ~= nil then
                    button.icon:SetTexture(button.backgroundTextureName)
                end
            end
        end
        "#,
    );
}

fn patch_fog_of_war_pin_mixin_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(FOG_OF_WAR_PIN_WORKAROUND_LUA);
}

fn patch_map_exploration_pin_mixin_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(MAP_EXPLORATION_PIN_WORKAROUND_LUA);
}

fn patch_toggle_collections_journal_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(TOGGLE_COLLECTIONS_JOURNAL_LUA);
    let _ = env.exec(MOUNT_JOURNAL_DYNAMIC_FLIGHT_POPUP_WORKAROUND_LUA);
}

fn patch_toggle_encounter_journal_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(TOGGLE_ENCOUNTER_JOURNAL_LUA);
}

fn patch_map_canvas_data_provider_attachment_for_runtime_addon_load(
    env: &crate::lua_api::LoaderEnv<'_>,
) {
    let _ = env.exec(MAP_CANVAS_DATA_PROVIDER_WORKAROUND_LUA);
}

fn ensure_adventure_map_frame_surface(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(ADVENTURE_MAP_FRAME_SURFACE_LUA);
}

fn ensure_adventure_map_frame_surface_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(ADVENTURE_MAP_FRAME_SURFACE_LUA);
}

fn patch_item_quality_color_data_methods(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(ITEM_QUALITY_COLOR_DATA_METHODS_WORKAROUND_LUA);
}

fn patch_artifact_ui_show_panel_guard(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(ARTIFACT_UI_SHOW_PANEL_GUARD_WORKAROUND_LUA);
}

fn patch_auction_house_categories_refresh_count(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(AUCTION_HOUSE_CATEGORIES_REFRESH_COUNT_WORKAROUND_LUA);
}

fn patch_auction_house_browse_results_event(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(AUCTION_HOUSE_BROWSE_RESULTS_EVENT_WORKAROUND_LUA);
}

fn patch_auction_house_browse_results_event_from_env(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(AUCTION_HOUSE_BROWSE_RESULTS_EVENT_WORKAROUND_LUA);
}

fn patch_auction_house_search_context_aliases(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(AUCTION_HOUSE_SEARCH_CONTEXT_ALIASES_WORKAROUND_LUA);
}

fn patch_auction_house_search_context_aliases_from_env(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(AUCTION_HOUSE_SEARCH_CONTEXT_ALIASES_WORKAROUND_LUA);
}

fn patch_auth_challenge_frame_parent(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(AUTH_CHALLENGE_FRAME_PARENT_WORKAROUND_LUA);
}

fn patch_auth_challenge_frame_parent_from_env(env: &crate::lua_api::WowLuaEnv) {
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

fn patch_map_canvas_scroll_container(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(
        r#"
if type(ClearCachedActivitiesForPlayer) ~= "function" then
  function ClearCachedActivitiesForPlayer() end
end

local function __wow_find_first_scroll_frame_child(parent)
  if type(parent) ~= "table" or type(parent.GetNumChildren) ~= "function" or type(parent.GetChildren) ~= "function" then
    return nil
  end
  local count = parent:GetNumChildren()
  for index = 1, count do
    local child = select(index, parent:GetChildren())
    if type(child) == "table" then
      local isScrollFrame =
        (type(child.IsObjectType) == "function" and child:IsObjectType("ScrollFrame")) or
        (type(child.GetObjectType) == "function" and child:GetObjectType() == "ScrollFrame")
      if isScrollFrame then
        return child
      end
    end
  end
  return nil
end

local function __wow_ensure_map_canvas_scroll_container(frame)
  if type(frame) ~= "table" then
    return nil
  end

  local existing = rawget(frame, "ScrollContainer")
  if existing ~= nil then
    return existing
  end

  local scroll = __wow_find_first_scroll_frame_child(frame)
  if scroll ~= nil then
    rawset(frame, "ScrollContainer", scroll)
  end
  return scroll
end

local function __wow_try_init_map_canvas(frame)
  if type(frame) ~= "table" then
    return
  end

  __wow_ensure_map_canvas_scroll_container(frame)
  if rawget(frame, "__wow_map_canvas_onload_ran") then
    return
  end

  local scroll = rawget(frame, "ScrollContainer")
  if scroll == nil then
    return
  end

  rawset(frame, "__wow_map_canvas_onload_ran", true)
  local originalOnLoad = rawget(_G, "__wow_map_canvas_original_onload")
  if type(originalOnLoad) == "function" then
    originalOnLoad(frame)
  end
end

local function __wow_refresh_map_canvas_size(frame)
  if type(frame) ~= "table" then
    return
  end

  local scroll = rawget(frame, "ScrollContainer")
  if type(scroll) ~= "table" then
    return
  end

  local child = rawget(scroll, "Child")
  local childWidth = type(child) == "table" and type(child.GetWidth) == "function" and child:GetWidth() or 0
  local childHeight = type(child) == "table" and type(child.GetHeight) == "function" and child:GetHeight() or 0
  if childWidth ~= 0 and childHeight ~= 0 then
    return
  end

  local mapID = rawget(frame, "mapID")
  if (mapID == nil or mapID == 0) and type(frame.GetMapID) == "function" then
    mapID = frame:GetMapID()
  end

  if mapID ~= nil and mapID ~= 0 and type(scroll.SetMapID) == "function" then
    scroll:SetMapID(mapID)
  elseif type(scroll.OnCanvasSizeChanged) == "function" then
    scroll:OnCanvasSizeChanged()
  end

  childWidth = type(child) == "table" and type(child.GetWidth) == "function" and child:GetWidth() or 0
  childHeight = type(child) == "table" and type(child.GetHeight) == "function" and child:GetHeight() or 0
  if childWidth ~= 0 and childHeight ~= 0 then
    return
  end

  local layers = mapID ~= nil
    and mapID ~= 0
    and C_Map ~= nil
    and type(C_Map.GetMapArtLayers) == "function"
    and C_Map.GetMapArtLayers(mapID)
    or nil
  local layer = type(layers) == "table" and layers[1] or nil
  if type(child) ~= "table" or type(layer) ~= "table" then
    return
  end

  local layerWidth = layer.layerWidth or 0
  local layerHeight = layer.layerHeight or 0
  if layerWidth == 0 or layerHeight == 0 then
    return
  end

  if type(child.SetSize) == "function" then
    child:SetSize(layerWidth, layerHeight)
  end

  local tiledBackground = rawget(child, "TiledBackground")
  if type(tiledBackground) == "table" and type(tiledBackground.SetSize) == "function" then
    tiledBackground:SetSize(layerWidth * 2, layerHeight * 2)
  end

  if type(scroll.CalculateScaleExtents) == "function" then
    scroll:CalculateScaleExtents()
  end
  if type(scroll.CalculateScrollExtents) == "function" then
    scroll:CalculateScrollExtents()
  end
  if type(frame.OnCanvasSizeChanged) == "function" then
    frame:OnCanvasSizeChanged()
  end
end

local function __wow_patch_live_map_canvas(frame)
  if type(frame) ~= "table" or type(MapCanvasMixin) ~= "table" then
    return
  end

  if type(MapCanvasMixin.SetMapID) == "function" then
    frame.SetMapID = MapCanvasMixin.SetMapID
  end
  if type(MapCanvasMixin.GetCanvas) == "function" then
    frame.GetCanvas = MapCanvasMixin.GetCanvas
  end
  if type(MapCanvasMixin.GetCanvasContainer) == "function" then
    frame.GetCanvasContainer = MapCanvasMixin.GetCanvasContainer
  end
  if type(MapCanvasMixin.OnFrameSizeChanged) == "function" then
    frame.OnFrameSizeChanged = MapCanvasMixin.OnFrameSizeChanged
  end
  if type(MapCanvasMixin.OnShow) == "function" then
    frame.OnShow = MapCanvasMixin.OnShow
  end

  __wow_try_init_map_canvas(frame)
  __wow_refresh_map_canvas_size(frame)
end

local function __wow_patch_world_map_display_state(frame)
  if type(frame) ~= "table" or type(frame.SetDisplayState) ~= "function" then
    return
  end
  if rawget(frame, "__wow_display_state_refresh_patched") then
    return
  end

  local originalSetDisplayState = frame.SetDisplayState
  frame.SetDisplayState = function(self, ...)
    local result = originalSetDisplayState(self, ...)
    __wow_try_init_map_canvas(self)
    __wow_refresh_map_canvas_size(self)
    return result
  end

  rawset(frame, "__wow_display_state_refresh_patched", true)
end

local function __wow_ensure_map_canvas_zoom_levels(scroll)
  if type(scroll) ~= "table" or type(scroll.zoomLevels) == "table" then
    return
  end

  local mapID = rawget(scroll, "mapID")
  if (mapID == nil or mapID == 0) and type(scroll.GetMap) == "function" then
    local map = scroll:GetMap()
    if type(map) == "table" and type(map.GetMapID) == "function" then
      mapID = map:GetMapID()
    end
  end

  local layers = mapID ~= nil
    and mapID ~= 0
    and C_Map ~= nil
    and type(C_Map.GetMapArtLayers) == "function"
    and C_Map.GetMapArtLayers(mapID)
    or nil
  if type(layers) ~= "table" or type(layers[1]) ~= "table" then
    scroll.zoomLevels = { { scale = 1.0, layerIndex = 1 } }
    scroll.targetScale = scroll.targetScale or 1.0
    return
  end

  local zoomLevels = {}
  for index, layer in ipairs(layers) do
    zoomLevels[index] = {
      scale = layer.minScale or 1.0,
      layerIndex = index,
    }
  end
  scroll.zoomLevels = zoomLevels
  scroll.targetScale = scroll.targetScale or zoomLevels[1].scale or 1.0
end

rawset(_G, "__wow_ensure_map_canvas_zoom_levels", __wow_ensure_map_canvas_zoom_levels)

local function __wow_refresh_world_map_canvas()
  __wow_patch_live_map_canvas(WorldMapFrame)
  __wow_patch_world_map_display_state(WorldMapFrame)
end

if type(MapCanvasMixin) == "table" and not rawget(_G, "__wow_map_canvas_scroll_container_advanced_patched") then
  if rawget(_G, "__wow_map_canvas_original_onload") == nil and type(MapCanvasMixin.OnLoad) == "function" then
    _G.__wow_map_canvas_original_onload = MapCanvasMixin.OnLoad
    MapCanvasMixin.OnLoad = function(self, ...)
      if rawget(self, "__wow_map_canvas_onload_ran") then
        return
      end
      __wow_try_init_map_canvas(self)
    end
  end

  if type(MapCanvasMixin.SetMapID) == "function" then
    local originalSetMapID = MapCanvasMixin.SetMapID
    MapCanvasMixin.SetMapID = function(self, ...)
      __wow_try_init_map_canvas(self)
      if rawget(self, "ScrollContainer") == nil then
        local mapID = ...
        self.mapID = mapID
        if C_Map and type(C_Map.GetMapArtID) == "function" then
          self.mapArtID = C_Map.GetMapArtID(mapID)
        end
        return
      end
      local result = originalSetMapID(self, ...)
      __wow_refresh_map_canvas_size(self)
      return result
    end
  end

  if type(MapCanvasMixin.OnShow) == "function" then
    local originalOnShow = MapCanvasMixin.OnShow
    MapCanvasMixin.OnShow = function(self, ...)
      __wow_try_init_map_canvas(self)
      local result = originalOnShow(self, ...)
      __wow_refresh_map_canvas_size(self)
      return result
    end
  end

  if type(MapCanvasMixin.GetCanvas) == "function" then
    MapCanvasMixin.GetCanvas = function(self, ...)
      __wow_try_init_map_canvas(self)
      local scroll = rawget(self, "ScrollContainer")
      return scroll and scroll.Child or nil
    end
  end

  if type(MapCanvasMixin.GetCanvasContainer) == "function" then
    MapCanvasMixin.GetCanvasContainer = function(self, ...)
      __wow_try_init_map_canvas(self)
      return rawget(self, "ScrollContainer")
    end
  end

  if type(MapCanvasMixin.OnFrameSizeChanged) == "function" then
    local originalOnFrameSizeChanged = MapCanvasMixin.OnFrameSizeChanged
    MapCanvasMixin.OnFrameSizeChanged = function(self, ...)
      __wow_try_init_map_canvas(self)
      if rawget(self, "ScrollContainer") == nil then
        return
      end
      return originalOnFrameSizeChanged(self, ...)
    end
  end

  if type(MapCanvasScrollControllerMixin) == "table"
    and type(MapCanvasScrollControllerMixin.GetZoomLevelIndexForScale) == "function"
  then
    local originalGetZoomLevelIndexForScale = MapCanvasScrollControllerMixin.GetZoomLevelIndexForScale
    MapCanvasScrollControllerMixin.GetZoomLevelIndexForScale = function(self, scale)
      __wow_ensure_map_canvas_zoom_levels(self)
      return originalGetZoomLevelIndexForScale(self, scale)
    end
  end

  if type(MapCanvasScrollControllerMixin) == "table"
    and type(MapCanvasScrollControllerMixin.GetCurrentLayerIndex) == "function"
  then
    local originalGetCurrentLayerIndex = MapCanvasScrollControllerMixin.GetCurrentLayerIndex
    MapCanvasScrollControllerMixin.GetCurrentLayerIndex = function(self, ...)
      __wow_ensure_map_canvas_zoom_levels(self)
      local zoomLevels = rawget(self, "zoomLevels")
      if type(zoomLevels) ~= "table" or type(zoomLevels[1]) ~= "table" then
        return 1
      end
      local ok, layerIndex = pcall(originalGetCurrentLayerIndex, self, ...)
      if ok and type(layerIndex) == "number" and layerIndex >= 1 then
        return layerIndex
      end
      return zoomLevels[1].layerIndex or 1
    end
  end

  rawset(_G, "__wow_map_canvas_scroll_container_advanced_patched", true)
  rawset(_G, "__wow_map_canvas_scroll_container_patched", true)
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
  __wow_patch_live_map_canvas(_G[mapName])
end
__wow_patch_world_map_display_state(WorldMapFrame)

if type(ToggleWorldMap) == "function" and not rawget(_G, "__wow_toggle_world_map_refresh_patched") then
  local originalToggleWorldMap = ToggleWorldMap
  ToggleWorldMap = function(...)
    local result = originalToggleWorldMap(...)
    __wow_refresh_world_map_canvas()
    return result
  end

  if type(OpenWorldMap) == "function" then
    local originalOpenWorldMap = OpenWorldMap
    OpenWorldMap = function(...)
      local result = originalOpenWorldMap(...)
      __wow_refresh_world_map_canvas()
      return result
    end
  end

  rawset(_G, "__wow_toggle_world_map_refresh_patched", true)
end
    "#,
    );
}

fn patch_collections_journal_namespace(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(
        r#"
        if type(C_MountJournal) == "table" then
            if rawget(C_MountJournal, "IsUsingDefaultFilters") == nil then
                function C_MountJournal.IsUsingDefaultFilters()
                    return true
                end
            end
            if rawget(C_MountJournal, "GetDisplayedMountID") == nil then
                function C_MountJournal.GetDisplayedMountID(_index)
                    return nil
                end
            end
        end

        if type(C_PetJournal) == "table" and rawget(C_PetJournal, "IsUsingDefaultFilters") == nil then
            function C_PetJournal.IsUsingDefaultFilters()
                return true
            end
        end

        if type(MountJournalToggleDynamicFlightFlyoutButtonMixin) == "table"
            and type(MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation) == "function"
            and not MountJournalToggleDynamicFlightFlyoutButtonMixin.__wow_popup_guard then
            local original = MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation
            MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation = function(self, ...)
                if not self.popup then
                    return
                end
                return original(self, ...)
            end
            MountJournalToggleDynamicFlightFlyoutButtonMixin.__wow_popup_guard = true
        end
        "#,
    );
}

const GETGLOBAL_HELPER_LUA: &str = r#"
local function __wow_getglobal(name)
    return getglobal(name)
end
_G.__wow_panel_getglobal = __wow_getglobal
"#;

const CHARACTER_CREATE_DEFAULTS_WORKAROUND_LUA: &str = r#"
local function __wow_character_create_defaults_frame()
    if type(CharacterCreateFrame) ~= "table" then
        return nil
    end
    return CharacterCreateFrame.RaceAndClassFrame
end

local function __wow_seed_character_create_defaults(frame)
    if type(frame) ~= "table" then
        return
    end

    local raceID = C_CharacterCreation and C_CharacterCreation.GetSelectedRace and C_CharacterCreation.GetSelectedRace() or 1
    if type(frame.selectedRaceData) ~= "table" then
        frame.selectedRaceData = C_CharacterCreation and C_CharacterCreation.GetRaceDataByID and C_CharacterCreation.GetRaceDataByID(raceID) or { enabled = true, isNeutralRace = false, factionInternalName = "Alliance" }
    end
    if type(frame.selectedClassData) ~= "table" then
        frame.selectedClassData = C_CharacterCreation and C_CharacterCreation.GetSelectedClass and C_CharacterCreation.GetSelectedClass() or { classID = 2, earlyFactionChoice = false }
    end
    if frame.selectedFaction == nil and C_CharacterCreation and C_CharacterCreation.GetFactionForRace then
        frame.selectedFaction = C_CharacterCreation.GetFactionForRace(raceID)
    end
end

local function __wow_seed_character_create_frame(frame)
    if type(frame) ~= "table" then
        return
    end

    if type(frame.BGTex) ~= "table" then
        frame.BGTex = {}
    end

    if type(frame.BackButton) == "table"
        and type(frame.BackButton.UpdateText) == "function"
        and type(frame.BackButton.GetText) == "function"
        and (frame.BackButton:GetText() == nil or frame.BackButton:GetText() == "")
    then
        frame.BackButton:UpdateText(BACK, BACKWARD_ARROW)
    end

    if type(frame.UpdateForwardButton) == "function" then
        frame:UpdateForwardButton()
    end
end

local characterCreateFrame = type(CharacterCreateFrame) == "table" and CharacterCreateFrame or nil
local raceAndClassFrame = characterCreateFrame and characterCreateFrame.RaceAndClassFrame or nil
if raceAndClassFrame ~= nil then
    __wow_seed_character_create_defaults(raceAndClassFrame)
end
if characterCreateFrame ~= nil then
    __wow_seed_character_create_frame(characterCreateFrame)
end

if type(CharacterCreateMixin) == "table" and type(CharacterCreateMixin.CreateCharacter) == "function" and not rawget(_G, "__wow_character_create_defaults_patched") then
    local originalCreateCharacter = CharacterCreateMixin.CreateCharacter
    function CharacterCreateMixin:CreateCharacter(...)
        __wow_seed_character_create_defaults(__wow_character_create_defaults_frame())
        __wow_seed_character_create_frame(self)
        if A_Admin and type(A_Admin.SetPlayerName) == "function" and type(self.GetSelectedName) == "function" then
            A_Admin.SetPlayerName(self:GetSelectedName())
        end
        return originalCreateCharacter(self, ...)
    end
    rawset(_G, "__wow_character_create_defaults_patched", true)
end

if type(CharacterCreateRaceAndClassMixin) == "table" and type(CharacterCreateRaceAndClassMixin.GetCreateCharacterFaction) == "function" and not rawget(_G, "__wow_character_create_faction_patched") then
    local originalGetCreateCharacterFaction = CharacterCreateRaceAndClassMixin.GetCreateCharacterFaction
    function CharacterCreateRaceAndClassMixin:GetCreateCharacterFaction()
        __wow_seed_character_create_defaults(self)
        return originalGetCreateCharacterFaction(self)
    end
    rawset(_G, "__wow_character_create_faction_patched", true)
end

if type(CharacterCreateRaceAndClassMixin) == "table" and type(CharacterCreateRaceAndClassMixin.UpdateState) == "function" and not rawget(_G, "__wow_character_create_update_patched") then
    local originalUpdateState = CharacterCreateRaceAndClassMixin.UpdateState
    function CharacterCreateRaceAndClassMixin:UpdateState(selectedFaction)
        __wow_seed_character_create_defaults(self)
        local result = originalUpdateState(self, selectedFaction)
        __wow_seed_character_create_frame(CharacterCreateFrame)
        return result
    end
    rawset(_G, "__wow_character_create_update_patched", true)
end

if type(CharacterCreateMixin) == "table" and type(CharacterCreateMixin.UpdateBackgroundOverlays) == "function" and not rawget(_G, "__wow_character_create_background_overlay_patched") then
    local originalUpdateBackgroundOverlays = CharacterCreateMixin.UpdateBackgroundOverlays
    function CharacterCreateMixin:UpdateBackgroundOverlays(selectedClassData, selectedRaceData)
        local ok = pcall(originalUpdateBackgroundOverlays, self, selectedClassData, selectedRaceData)
        if ok then
            return
        end

        local backgroundTextures = self and self.BGTex or nil
        if type(backgroundTextures) == "table" then
            local iter_ok, iter, state, first = pcall(ipairs, backgroundTextures)
            if iter_ok and type(iter) == "function" then
                for _, texture in iter, state, first do
                    if type(texture) == "table" and type(texture.SetAlpha) == "function" then
                        texture:SetAlpha(1)
                    end
                end
                return
            end
        end

        if type(backgroundTextures) == "table" and type(backgroundTextures.SetAlpha) == "function" then
            backgroundTextures:SetAlpha(1)
        end
    end
    rawset(_G, "__wow_character_create_background_overlay_patched", true)
end
"#;

const FOG_OF_WAR_PIN_WORKAROUND_LUA: &str = r#"
local function __wow_clear_fog_of_war_pin_assets(pin)
    if type(pin) ~= "table" then
        return
    end
    if type(pin.SetFogOfWarID) == "function" then
        pin:SetFogOfWarID(nil, true)
    end
    if type(pin.SetFogOfWarBackgroundAtlas) == "function" then
        pin:SetFogOfWarBackgroundAtlas(nil)
    end
    if type(pin.SetFogOfWarMaskAtlas) == "function" then
        pin:SetFogOfWarMaskAtlas(nil)
    end
end

local function __wow_resolve_fog_of_war_map_id(pin)
    local mapID = nil
    if type(pin) == "table" and type(pin.GetMap) == "function" then
        local map = pin:GetMap()
        if map ~= nil and type(map.GetMapID) == "function" then
            mapID = map:GetMapID()
        end
    end

    if (mapID == nil or mapID == 0) and C_Map ~= nil and type(C_Map.GetCurrentMapID) == "function" then
        mapID = C_Map.GetCurrentMapID()
    end

    return mapID or 0
end

local function __wow_refresh_fog_of_war_pin(pin, forceUpdate)
    if type(pin) ~= "table" then
        return
    end

    local mapID = __wow_resolve_fog_of_war_map_id(pin)
    if type(pin.SetUiMapID) == "function" then
        pin:SetUiMapID(mapID)
    end

    if mapID == 0 then
        __wow_clear_fog_of_war_pin_assets(pin)
        if type(pin.Hide) == "function" then
            pin:Hide()
        end
        return
    end

    local fogOfWarID = nil
    if C_FogOfWar ~= nil and type(C_FogOfWar.GetFogOfWarForMap) == "function" then
        fogOfWarID = C_FogOfWar.GetFogOfWarForMap(mapID)
    end
    if type(pin.SetFogOfWarID) == "function" then
        pin:SetFogOfWarID(fogOfWarID, forceUpdate)
    end

    local hasBackgroundAtlas =
        type(pin.GetFogOfWarBackgroundAtlas) == "function" and pin:GetFogOfWarBackgroundAtlas() ~= nil
    local hasMaskAtlas =
        type(pin.GetFogOfWarMaskAtlas) == "function" and pin:GetFogOfWarMaskAtlas() ~= nil
    if fogOfWarID == nil or (not hasBackgroundAtlas and not hasMaskAtlas) then
        __wow_clear_fog_of_war_pin_assets(pin)
        if type(pin.Hide) == "function" then
            pin:Hide()
        end
    end
end

local function __wow_apply_fog_of_war_pin_workaround(pin)
    if type(pin) ~= "table" then
        return
    end
    if type(FogOfWarPinMixin) == "table" and type(FogOfWarPinMixin.OnMapChanged) == "function" then
        pin.OnMapChanged = FogOfWarPinMixin.OnMapChanged
    end
    if type(FogOfWarFrameMixin) == "table" and type(FogOfWarFrameMixin.TryFindingBestFogOfWarID) == "function" then
        pin.TryFindingBestFogOfWarID = FogOfWarFrameMixin.TryFindingBestFogOfWarID
    end
    __wow_refresh_fog_of_war_pin(pin, true)
end

local function __wow_patch_live_fog_of_war_pins(map)
    if type(map) ~= "table" then
        return
    end

    if type(map.EnumeratePinsByTemplate) == "function" then
        for pin in map:EnumeratePinsByTemplate("FogOfWarPinTemplate") do
            __wow_apply_fog_of_war_pin_workaround(pin)
        end
    end

    if type(map.dataProviders) ~= "table" then
        return
    end

    for provider in pairs(map.dataProviders) do
        local pin = type(provider) == "table" and rawget(provider, "pin") or nil
        if type(pin) == "table" then
            __wow_apply_fog_of_war_pin_workaround(pin)
        end
    end
end

if type(FogOfWarPinMixin) == "table" and not rawget(_G, "__wow_fog_of_war_pin_methods_patched") then
    if type(FogOfWarFrameMixin) == "table" and type(FogOfWarFrameMixin.TryFindingBestFogOfWarID) == "function" then
        FogOfWarFrameMixin.TryFindingBestFogOfWarID = function(self, forceUpdate)
            __wow_refresh_fog_of_war_pin(self, forceUpdate)
        end
    end

    if type(FogOfWarPinMixin.OnMapChanged) == "function" then
        FogOfWarPinMixin.OnMapChanged = function(self)
            __wow_refresh_fog_of_war_pin(self, true)
        end
    end

    rawset(_G, "__wow_fog_of_war_pin_methods_patched", true)
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
    __wow_patch_live_fog_of_war_pins(_G[mapName])
end
"#;

const MAP_EXPLORATION_PIN_WORKAROUND_LUA: &str = r#"
local function __wow_size_map_exploration_pin(pin)
    if type(pin) ~= "table" then
        return
    end
    if type(pin.OnCanvasSizeChanged) == "function" then
        pin:OnCanvasSizeChanged()
    end
end

local function __wow_finalize_map_exploration_pin_waiting(pin)
    if type(pin) ~= "table" then
        return
    end

    if not rawget(pin, "isWaitingForLoad") then
        return
    end

    local map = type(pin.GetMap) == "function" and pin:GetMap() or nil
    local detailLayersLoaded = type(map) == "table"
        and type(map.AreDetailLayersLoaded) == "function"
        and map:AreDetailLayersLoaded()
    local textureLoadGroup = rawget(pin, "textureLoadGroup")
    local texturesLoaded = type(textureLoadGroup) == "table"
        and type(textureLoadGroup.IsFullyLoaded) == "function"
        and textureLoadGroup:IsFullyLoaded()
    local overlayPool = rawget(pin, "overlayTexturePool")
    local hasOverlayTextures = type(overlayPool) == "table"
        and type(overlayPool.GetNumActive) == "function"
        and overlayPool:GetNumActive() > 0

    if detailLayersLoaded and (texturesLoaded or hasOverlayTextures) then
        if type(pin.RefreshAlpha) == "function" then
            pin:RefreshAlpha()
        end
        pin.isWaitingForLoad = nil
        if type(textureLoadGroup) == "table" and type(textureLoadGroup.Reset) == "function" then
            textureLoadGroup:Reset()
        end
        return
    end

    if type(pin.Show) == "function" and type(pin.IsShown) == "function" and not pin:IsShown() then
        pin:Show()
    end
end

local function __wow_map_exploration_pin_overlay_count(pin)
    if type(pin) ~= "table" then
        return 0
    end
    local overlayPool = rawget(pin, "overlayTexturePool")
    if type(overlayPool) ~= "table" or type(overlayPool.GetNumActive) ~= "function" then
        return 0
    end
    return overlayPool:GetNumActive()
end

local function __wow_should_retry_map_exploration_pin_overlay_refresh(pin)
    if type(pin) ~= "table" then
        return false
    end

    local map = type(pin.GetMap) == "function" and pin:GetMap() or nil
    local mapID = type(map) == "table" and type(map.GetMapID) == "function" and map:GetMapID() or nil
    if type(mapID) ~= "number" or mapID == 0 then
        return false
    end

    if type(C_MapExplorationInfo) ~= "table" or type(C_MapExplorationInfo.GetExploredMapTextures) ~= "function" then
        return false
    end

    local exploredMapTextures = C_MapExplorationInfo.GetExploredMapTextures(mapID)
    if type(exploredMapTextures) ~= "table" or #exploredMapTextures == 0 then
        return false
    end

    return __wow_map_exploration_pin_overlay_count(pin) == 0
end

local function __wow_schedule_map_exploration_pin_finalize_retry(pin)
    if type(pin) ~= "table" or not rawget(pin, "isWaitingForLoad") then
        return
    end

    if rawget(pin, "__wow_finalize_retry_pending") then
        return
    end
    if type(C_Timer) ~= "table" or type(C_Timer.After) ~= "function" then
        return
    end

    rawset(pin, "__wow_finalize_retry_pending", true)
    C_Timer.After(0, function()
        if type(pin) ~= "table" then
            return
        end
        rawset(pin, "__wow_finalize_retry_pending", nil)
        __wow_finalize_map_exploration_pin_waiting(pin)
    end)
end

local function __wow_schedule_map_exploration_pin_overlay_retry(pin)
    if type(pin) ~= "table" then
        return
    end

    if not __wow_should_retry_map_exploration_pin_overlay_refresh(pin) then
        return
    end

    if rawget(pin, "__wow_overlay_retry_pending") then
        return
    end
    if type(C_Timer) ~= "table" or type(C_Timer.After) ~= "function" then
        return
    end

    rawset(pin, "__wow_overlay_retry_pending", true)
    C_Timer.After(0, function()
        if type(pin) ~= "table" then
            return
        end

        rawset(pin, "__wow_overlay_retry_pending", nil)
        if not __wow_should_retry_map_exploration_pin_overlay_refresh(pin) then
            return
        end

        if type(pin.RefreshOverlays) == "function" then
            pin:RefreshOverlays(true)
        end

        __wow_finalize_map_exploration_pin_waiting(pin)
        __wow_schedule_map_exploration_pin_finalize_retry(pin)
    end)
end

local function __wow_patch_live_map_exploration_pins(map)
    if type(map) ~= "table" then
        return
    end

    if type(map.EnumeratePinsByTemplate) == "function" then
        for pin in map:EnumeratePinsByTemplate("MapExplorationPinTemplate") do
            __wow_size_map_exploration_pin(pin)
            __wow_schedule_map_exploration_pin_overlay_retry(pin)
        end
    end

    if type(map.dataProviders) ~= "table" then
        return
    end

    for provider in pairs(map.dataProviders) do
        local pin = type(provider) == "table" and rawget(provider, "pin") or nil
        if type(pin) == "table"
            and type(pin.RefreshOverlays) == "function"
            and type(pin.OnCanvasSizeChanged) == "function"
        then
            if type(MapExplorationPinMixin) == "table" and type(MapExplorationPinMixin.RefreshOverlays) == "function" then
                pin.RefreshOverlays = MapExplorationPinMixin.RefreshOverlays
            end
            __wow_size_map_exploration_pin(pin)
            __wow_schedule_map_exploration_pin_overlay_retry(pin)
        end
    end
end

if type(MapExplorationPinMixin) == "table" and not rawget(_G, "__wow_map_exploration_pin_patched") then
    if type(MapExplorationPinMixin.OnAcquired) == "function" then
        local originalOnAcquired = MapExplorationPinMixin.OnAcquired
        MapExplorationPinMixin.OnAcquired = function(self, dataProvider)
            originalOnAcquired(self, dataProvider)
            __wow_size_map_exploration_pin(self)
            __wow_finalize_map_exploration_pin_waiting(self)
            __wow_schedule_map_exploration_pin_finalize_retry(self)
            __wow_schedule_map_exploration_pin_overlay_retry(self)
        end
    end

    if type(MapExplorationPinMixin.RefreshOverlays) == "function" then
        local originalRefreshOverlays = MapExplorationPinMixin.RefreshOverlays
        MapExplorationPinMixin.RefreshOverlays = function(self, fullUpdate)
            __wow_size_map_exploration_pin(self)
            local map = type(self.GetMap) == "function" and self:GetMap() or nil
            local container = type(map) == "table" and type(map.GetCanvasContainer) == "function" and map:GetCanvasContainer() or nil
            if type(container) == "table" then
                local ensureZoomLevels = rawget(_G, "__wow_ensure_map_canvas_zoom_levels")
                if type(ensureZoomLevels) == "function" then
                    ensureZoomLevels(container)
                end
                local zoomLevels = rawget(container, "zoomLevels")
                if (type(zoomLevels) ~= "table" or type(zoomLevels[1]) ~= "table")
                    and type(container.CreateZoomLevels) == "function"
                then
                    pcall(container.CreateZoomLevels, container)
                    zoomLevels = rawget(container, "zoomLevels")
                end
                if type(zoomLevels) ~= "table" or type(zoomLevels[1]) ~= "table" then
                    rawset(container, "zoomLevels", { { scale = 1.0, layerIndex = 1 } })
                end
            end
            local result = originalRefreshOverlays(self, fullUpdate)
            __wow_finalize_map_exploration_pin_waiting(self)
            __wow_schedule_map_exploration_pin_finalize_retry(self)
            __wow_schedule_map_exploration_pin_overlay_retry(self)
            return result
        end
    end

    if type(MapExplorationPinMixin.OnUpdate) == "function" then
        local originalOnUpdate = MapExplorationPinMixin.OnUpdate
        MapExplorationPinMixin.OnUpdate = function(self, elapsed)
            if rawget(self, "isWaitingForLoad")
                and type(self.Show) == "function"
                and type(self.IsShown) == "function"
                and not self:IsShown()
            then
                self:Show()
            end
            local result = originalOnUpdate(self, elapsed)
            __wow_finalize_map_exploration_pin_waiting(self)
            __wow_schedule_map_exploration_pin_overlay_retry(self)
            return result
        end
    end

    rawset(_G, "__wow_map_exploration_pin_patched", true)
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
    __wow_patch_live_map_exploration_pins(_G[mapName])
end
"#;

const MAP_CANVAS_DATA_PROVIDER_WORKAROUND_LUA: &str = r#"
local function __wow_fix_provider_pin(provider)
    if type(provider) ~= "table" then
        return
    end

    local pin = provider.pin
    if pin ~= nil then
        pin.dataProvider = provider
    end
end

if type(MapCanvasMixin) == "table" and not rawget(_G, "__wow_map_canvas_add_data_provider_patched") then
    if type(MapCanvasMixin.AddDataProvider) == "function" then
        local originalAddDataProvider = MapCanvasMixin.AddDataProvider
        MapCanvasMixin.AddDataProvider = function(self, dataProvider, ...)
            local result = originalAddDataProvider(self, dataProvider, ...)
            __wow_fix_provider_pin(dataProvider)
            return result
        end
    end

    rawset(_G, "__wow_map_canvas_add_data_provider_patched", true)
end

for _, mapName in ipairs({ "WorldMapFrame", "BattlefieldMapFrame" }) do
    local map = rawget(_G, mapName)
    if type(map) == "table" then
        if type(map.AddDataProvider) == "function" and rawget(map, "__wow_add_data_provider_patched") ~= true then
            local originalAddDataProvider = map.AddDataProvider
            map.AddDataProvider = function(self, dataProvider, ...)
                local result = originalAddDataProvider(self, dataProvider, ...)
                __wow_fix_provider_pin(dataProvider)
                return result
            end
            rawset(map, "__wow_add_data_provider_patched", true)
        end

        if type(map.dataProviders) == "table" then
            for provider in pairs(map.dataProviders) do
                __wow_fix_provider_pin(provider)
            end
        end
    end
end
"#;

const ADVENTURE_MAP_FRAME_SURFACE_LUA: &str = r#"
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

const TOGGLE_ACHIEVEMENT_FRAME_LUA: &str = r#"
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

const TOGGLE_ENCOUNTER_JOURNAL_LUA: &str = r#"
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

const MAIN_MENU_MICROBUTTON_CLICK_WORKAROUND_LUA: &str = r#"
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

const TOGGLE_COLLECTIONS_JOURNAL_LUA: &str = r#"
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

const MOUNT_JOURNAL_DYNAMIC_FLIGHT_POPUP_WORKAROUND_LUA: &str = r#"
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

const DAMAGE_METER_INITIAL_SCROLLBOX_EXTENT_LUA: &str = r#"
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

const VIGNETTE_PIN_TEMPLATE_WORKAROUND_LUA: &str = r###"
local function __wow_patch_vignette_provider(provider)
    if type(provider) ~= "table" then
        return
    end
    if type(provider.GetPinTemplate) ~= "function" then
        return
    end
    if type(provider.GetDefaultPinTemplate) ~= "function" then
        return
    end
    if provider.__wow_ui_sim_nil_safe_get_pin_template then
        return
    end
    if provider:GetDefaultPinTemplate() ~= "VignettePinTemplate" then
        return
    end

    local original = provider.GetPinTemplate
    function provider:GetPinTemplate(vignetteInfo)
        if vignetteInfo == nil then
            return self:GetDefaultPinTemplate()
        end
        return original(self, vignetteInfo)
    end
    provider.__wow_ui_sim_nil_safe_get_pin_template = true
end

__wow_patch_vignette_provider(VignetteDataProviderMixin)

for _, mapName in ipairs({"WorldMapFrame", "BattlefieldMapFrame", "FlightMapFrame"}) do
    local map = _G[mapName]
    if map and type(map.dataProviders) == "table" then
        for provider in pairs(map.dataProviders) do
            __wow_patch_vignette_provider(provider)
        end
    end
end
"###;

const UIPARENT_ONUPDATE_WORKLISTS_WORKAROUND_LUA: &str = r#"
if type(FCF_OnUpdate) == "function" and rawget(_G, "__wow_fcf_onupdate_wrapper") ~= FCF_OnUpdate then
    local original_fcf_onupdate = FCF_OnUpdate
    local wrapper = function(elapsed)
        if type(CHAT_FRAMES) == "table" and next(CHAT_FRAMES) == nil then
            return
        end
        return original_fcf_onupdate(elapsed)
    end
    FCF_OnUpdate = wrapper
    rawset(_G, "__wow_fcf_onupdate_wrapper", wrapper)
end

if type(ButtonPulse_OnUpdate) == "function" and rawget(_G, "__wow_button_pulse_onupdate_wrapper") ~= ButtonPulse_OnUpdate then
    local original_button_pulse_onupdate = ButtonPulse_OnUpdate
    local wrapper = function(elapsed)
        if type(PULSEBUTTONS) == "table" and next(PULSEBUTTONS) == nil then
            return
        end
        return original_button_pulse_onupdate(elapsed)
    end
    ButtonPulse_OnUpdate = wrapper
    rawset(_G, "__wow_button_pulse_onupdate_wrapper", wrapper)
end

if type(AnimatedShine_OnUpdate) == "function" and rawget(_G, "__wow_animated_shine_onupdate_wrapper") ~= AnimatedShine_OnUpdate then
    local original_animated_shine_onupdate = AnimatedShine_OnUpdate
    local wrapper = function(elapsed)
        if type(SHINES_TO_ANIMATE) == "table" and next(SHINES_TO_ANIMATE) == nil then
            return
        end
        return original_animated_shine_onupdate(elapsed)
    end
    AnimatedShine_OnUpdate = wrapper
    rawset(_G, "__wow_animated_shine_onupdate_wrapper", wrapper)
end

if type(UIParent) == "table"
    and type(UIParent.GetScript) == "function"
    and type(UIParent.SetScript) == "function" then
    local wrapper = rawget(_G, "__wow_ui_parent_onupdate_worklist_wrapper")
    if UIParent:GetScript("OnUpdate") ~= wrapper then
        wrapper = function(self, elapsed)
            if type(CHAT_FRAMES) ~= "table" or next(CHAT_FRAMES) ~= nil then
                FCF_OnUpdate(elapsed)
            end
            if type(PULSEBUTTONS) ~= "table" or next(PULSEBUTTONS) ~= nil then
                ButtonPulse_OnUpdate(elapsed)
            end
            if type(SHINES_TO_ANIMATE) ~= "table" or next(SHINES_TO_ANIMATE) ~= nil then
                AnimatedShine_OnUpdate(elapsed)
            end
            if type(HelpOpenWebTicketButton_OnUpdate) == "function" then
                HelpOpenWebTicketButton_OnUpdate(HelpOpenWebTicketButton, elapsed)
            end
        end
        UIParent:SetScript("OnUpdate", wrapper)
        rawset(_G, "__wow_ui_parent_onupdate_worklist_wrapper", wrapper)
    end
end
"#;

const CHAT_VOICE_BUTTON_SURFACE_WORKAROUND_LUA: &str = r#"
local defaultChatFrame = DEFAULT_CHAT_FRAME or ChatFrame1
local defaultEditBox = rawget(_G, "ChatFrame1EditBox")
if type(defaultChatFrame) == "table" and type(defaultEditBox) == "table" then
    if defaultChatFrame.editBox == nil then
        defaultChatFrame.editBox = defaultEditBox
    end
    if defaultEditBox.chatFrame == nil then
        defaultEditBox.chatFrame = defaultChatFrame
    end
    if DEFAULT_CHAT_FRAME == nil then
        DEFAULT_CHAT_FRAME = defaultChatFrame
    end
end

local channelButton = ChatFrameChannelButton
if type(channelButton) == "table" then
    local icon = channelButton.Icon
    if icon == nil and type(channelButton.CreateTexture) == "function" then
        icon = channelButton:CreateTexture(nil, "OVERLAY")
        channelButton.Icon = icon
    end

    if icon ~= nil then
        if type(icon.SetParentKey) == "function" then
            pcall(icon.SetParentKey, icon, "Icon", true)
        end
        if type(icon.GetWidth) == "function" and type(icon.GetHeight) == "function"
            and (icon:GetWidth() == 0 or icon:GetHeight() == 0)
            and type(icon.SetSize) == "function" then
            icon:SetSize(channelButton.fixedIconWidth or 15, channelButton.fixedIconHeight or 15)
        end
        if type(icon.GetNumPoints) == "function" and icon:GetNumPoints() == 0
            and type(icon.SetPoint) == "function" then
            icon:SetPoint("CENTER", channelButton, "CENTER", 0, 0)
        end
        if type(icon.SetAtlas) == "function" then
            icon:SetAtlas("chatframe-button-icon-voicechat")
        end
        if type(icon.Show) == "function" then
            icon:Show()
        end
    end
end

if QuickJoinToastButton == nil and type(CreateFrame) == "function" and UIParent ~= nil then
    QuickJoinToastButton = CreateFrame("Button", "QuickJoinToastButton", UIParent)
end
"#;

const POST_EVENT_FRAME_LAYOUT_WORKAROUND_LUA: &str = r#"
local function reanchor_objective_tracker(frame)
    frame:ClearAllPoints()
    frame:SetPoint(
        "TOPRIGHT",
        UIParentRightManagedFrameContainer,
        "TOPRIGHT",
        0,
        11
    )
    frame:SetHeight(836.5)
end

if EditModeManagerFrame then
    local partySystem = EditModeManagerFrame:GetRegisteredSystemFrame(
        Enum.EditModeSystem.UnitFrame,
        Enum.EditModeUnitFrameSystemIndices.Party
    )
    if partySystem and partySystem.systemInfo and partySystem.systemInfo.settings then
        for _, settingInfo in ipairs(partySystem.systemInfo.settings) do
            if settingInfo.setting == Enum.EditModeUnitFrameSetting.UseRaidStylePartyFrames then
                settingInfo.value = 0
            end
        end
        if partySystem.UpdateSettingMap then
            partySystem:UpdateSettingMap(true)
        end
        if partySystem.UpdateSystemSetting then
            pcall(
                partySystem.UpdateSystemSetting,
                partySystem,
                Enum.EditModeUnitFrameSetting.UseRaidStylePartyFrames,
                true
            )
        end
    end
end

if UpdateRaidAndPartyFrames then
    pcall(UpdateRaidAndPartyFrames)
end
if PartyFrame and PartyFrame.UpdatePaddingAndLayout then
    pcall(PartyFrame.UpdatePaddingAndLayout, PartyFrame)
end
if CompactPartyFrame and CompactPartyFrame.UpdateVisibility then
    pcall(CompactPartyFrame.UpdateVisibility, CompactPartyFrame)
end
if ObjectiveTrackerFrame then
    if ObjectiveTrackerFrame.Update then
        pcall(ObjectiveTrackerFrame.Update, ObjectiveTrackerFrame)
    end
    if ObjectiveTrackerFrame.UpdateHeight then
        pcall(ObjectiveTrackerFrame.UpdateHeight, ObjectiveTrackerFrame)
    end
    reanchor_objective_tracker(ObjectiveTrackerFrame)
end
if CompactPartyFrame then
    CompactPartyFrame:SetHeight(234)
end
if PlayerCastingBarFrame then
    PlayerCastingBarFrame:SetAlpha(1)
end
if not rawget(_G, "__wow_objective_tracker_update_height_wrapper")
    and ObjectiveTrackerContainerMixin
    and type(ObjectiveTrackerContainerMixin.UpdateHeight) == "function" then
    local originalUpdateHeight = ObjectiveTrackerContainerMixin.UpdateHeight
    function ObjectiveTrackerContainerMixin:UpdateHeight()
        originalUpdateHeight(self)
        if self == ObjectiveTrackerFrame then
            reanchor_objective_tracker(self)
        end
    end
    rawset(_G, "__wow_objective_tracker_update_height_wrapper", true)
end
if not rawget(_G, "__wow_compact_party_update_layout_wrapper")
    and CompactPartyFrameMixin
    and type(CompactPartyFrameMixin.UpdateLayout) == "function" then
    local originalUpdateLayout = CompactPartyFrameMixin.UpdateLayout
    function CompactPartyFrameMixin:UpdateLayout()
        originalUpdateLayout(self)
        self:SetHeight(234)
    end
    rawset(_G, "__wow_compact_party_update_layout_wrapper", true)
end
if not rawget(_G, "__wow_casting_bar_apply_alpha_wrapper")
    and CastingBarMixin
    and type(CastingBarMixin.ApplyAlpha) == "function" then
    local originalApplyAlpha = CastingBarMixin.ApplyAlpha
    function CastingBarMixin:ApplyAlpha(alpha)
        if self == PlayerCastingBarFrame then
            alpha = 1
        end
        originalApplyAlpha(self, alpha)
    end
    rawset(_G, "__wow_casting_bar_apply_alpha_wrapper", true)
end
if ChatFrame1EditBox and ChatFrame1 then
    ChatFrame1EditBox:SetWidth(447)
end
"#;

const REFRESH_ACTION_BUTTONS_LUA: &str = r###"
local function __wow_refresh_action_button(button)
    if type(button) ~= "table" then
        return
    end
    if type(button.UpdateButtonArt) == "function" then
        pcall(button.UpdateButtonArt, button)
    end
    if type(button.UpdateHotkeys) == "function" then
        pcall(button.UpdateHotkeys, button, button.buttonType)
    end
end

for i = 1, 12 do
    __wow_refresh_action_button(_G["ActionButton" .. i])
end
"###;
