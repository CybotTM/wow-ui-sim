//! Integration tests for keybinding dispatch — world map and detailed panel interaction tests.
//!
//! Covers world map, escape menu, spellbook tooltip, and talent panel deep tests.

mod common;
#[path = "common/token_ui_fixtures.rs"]
mod token_ui_fixtures;

use std::path::PathBuf;
use token_ui_fixtures::load_token_ui;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::{fire_one_on_update_tick, process_pending_timers};

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Blizzard addons in dependency order (same as micro_menu.rs).
const BLIZZARD_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    ("Blizzard_SharedXMLGame", "Blizzard_SharedXMLGame.toc"),
    (
        "Blizzard_UIPanelTemplates",
        "Blizzard_UIPanelTemplates_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLBase",
        "Blizzard_FrameXMLBase_Mainline.toc",
    ),
    ("Blizzard_FrameEffects", "Blizzard_FrameEffects.toc"),
    ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
    ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
    ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
    (
        "Blizzard_AccessibilityTemplates",
        "Blizzard_AccessibilityTemplates.toc",
    ),
    ("Blizzard_ObjectAPI", "Blizzard_ObjectAPI_Mainline.toc"),
    ("Blizzard_UIParent", "Blizzard_UIParent_Mainline.toc"),
    ("Blizzard_TextStatusBar", "Blizzard_TextStatusBar.toc"),
    ("Blizzard_MoneyFrame", "Blizzard_MoneyFrame_Mainline.toc"),
    ("Blizzard_POIButton", "Blizzard_POIButton.toc"),
    ("Blizzard_Flyout", "Blizzard_Flyout.toc"),
    ("Blizzard_StoreUI", "Blizzard_StoreUI_Mainline.toc"),
    ("Blizzard_MicroMenu", "Blizzard_MicroMenu_Mainline.toc"),
    ("Blizzard_EditMode", "Blizzard_EditMode.toc"),
    ("Blizzard_GarrisonBase", "Blizzard_GarrisonBase.toc"),
    ("Blizzard_GameTooltip", "Blizzard_GameTooltip_Mainline.toc"),
    (
        "Blizzard_UIParentPanelManager",
        "Blizzard_UIParentPanelManager_Mainline.toc",
    ),
    (
        "Blizzard_Settings_Shared",
        "Blizzard_Settings_Shared_Mainline.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Shared",
        "Blizzard_SettingsDefinitions_Shared.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Frame",
        "Blizzard_SettingsDefinitions_Frame_Mainline.toc",
    ),
    ("Blizzard_FrameXMLUtil", "Blizzard_FrameXMLUtil.toc"),
    ("Blizzard_Menu", "Blizzard_Menu.toc"),
    ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
    ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    (
        "Blizzard_UIPanels_Game",
        "Blizzard_UIPanels_Game_Mainline.toc",
    ),
    (
        "Blizzard_MapCanvasSecureUtil",
        "Blizzard_MapCanvasSecureUtil.toc",
    ),
    ("Blizzard_MapCanvas", "Blizzard_MapCanvas.toc"),
    (
        "Blizzard_SharedMapDataProviders",
        "Blizzard_SharedMapDataProviders_Mainline.toc",
    ),
    ("Blizzard_WorldMap", "Blizzard_WorldMap_Mainline.toc"),
    ("Blizzard_ActionBar", "Blizzard_ActionBar_Mainline.toc"),
    (
        "Blizzard_ActionBarController",
        "Blizzard_ActionBarController.toc",
    ),
    ("Blizzard_GameMenu", "Blizzard_GameMenu_Mainline.toc"),
    ("Blizzard_UIWidgets", "Blizzard_UIWidgets_Mainline.toc"),
    ("Blizzard_Minimap", "Blizzard_Minimap_Mainline.toc"),
    ("Blizzard_AddOnList", "Blizzard_AddOnList.toc"),
    ("Blizzard_TimerunningUtil", "Blizzard_TimerunningUtil.toc"),
    ("Blizzard_Communities", "Blizzard_Communities_Mainline.toc"),
];

fn setup_env() -> common::LockedEnv {
    common::lock_env(|| {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        {
            let mut state = env.state().borrow_mut();
            state.addon_base_paths = vec![blizzard_ui_dir()];
        }

        let ui = blizzard_ui_dir();
        for (name, toc) in BLIZZARD_ADDONS {
            let toc_path = ui.join(name).join(toc);
            if !toc_path.exists() {
                continue;
            }
            if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
                eprintln!("[load {name}] FAILED: {e}");
            } else {
                env.apply_runtime_addon_load_workarounds(name);
            }
        }

        load_token_ui(&env);
        env.apply_post_load_workarounds();
        fire_startup_events(&env);
        env
    })
}

fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::call_global_if_present(env, "RequestTimePlayed");
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
        "PLAYER_LEAVING_WORLD",
    ] {
        let _ = env.fire_event(event);
    }
}

fn frame_is_shown(env: &WowLuaEnv, frame_name: &str) -> bool {
    let code = format!("return {frame_name} ~= nil and {frame_name}:IsShown() == true");
    env.eval::<bool>(&code).unwrap_or(false)
}

fn install_test_error_handler(env: &WowLuaEnv) {
    common::install_error_collector(env, "__test_errors");
}

fn drain_test_errors(env: &WowLuaEnv) -> Vec<String> {
    common::drain_string_table(env, "__test_errors")
}

// ── M → ToggleWorldMap() ────────────────────────────────────────────────

#[test]
fn keybind_m_opens_world_map() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("M", None).expect("M keybind failed");
        assert!(
            frame_is_shown(&env, "WorldMapFrame"),
            "WorldMapFrame should be shown after pressing M"
        );
    }
}

#[test]
fn world_map_floor_dropdown_hidden_without_subzone_or_map_group() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);
        env.state().borrow_mut().world.sub_zone_name.clear();

        env.send_key_press("M", None).expect("M keybind failed");

        let result: String = env
            .eval(
                r#"
                if not (WorldMapFrame and WorldMapFrame:IsShown()) then
                    return "world_map_not_open"
                end

                local floorDropdown
                for _, frame in ipairs(WorldMapFrame.overlayFrames or {}) do
                    if type(frame.RefreshMenu) == "function" then
                        floorDropdown = frame
                        break
                    end
                end

                if not floorDropdown then
                    return "missing_floor_dropdown"
                end

                local mapID = WorldMapFrame:GetMapID()
                local groupID = C_Map.GetMapGroupID(mapID)
                local members = C_Map.GetMapGroupMembersInfo(groupID)
                local memberCount = 0
                if type(members) == "table" then
                    for _ in ipairs(members) do
                        memberCount = memberCount + 1
                    end
                end

                if floorDropdown:IsShown() then
                    return string.format(
                        "shown:subzone=%s:groupID=%s:groupType=%s:membersType=%s:members=%d",
                        tostring(GetSubZoneText()),
                        tostring(groupID),
                        type(groupID),
                        type(members),
                        memberCount
                    )
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Opening world map with no subzone produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "World map floor dropdown should stay hidden when there is no subzone or map group: {result}"
        );
    }
}

#[test]
fn keybind_m_toggles_world_map_without_errors() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("M", None).expect("first M keybind failed");

        let open_errors = drain_test_errors(&env);
        assert!(
            open_errors.is_empty(),
            "Opening world map produced {} Lua error(s):\n{}",
            open_errors.len(),
            open_errors.join("\n"),
        );
        assert!(
            frame_is_shown(&env, "WorldMapFrame"),
            "WorldMapFrame should be shown after first M press"
        );

        env.send_key_press("M", None).expect("second M keybind failed");

        let close_errors = drain_test_errors(&env);
        assert!(
            close_errors.is_empty(),
            "Closing world map produced {} Lua error(s):\n{}",
            close_errors.len(),
            close_errors.join("\n"),
        );
        assert!(
            !frame_is_shown(&env, "WorldMapFrame"),
            "WorldMapFrame should be hidden after second M press"
        );
    }
}

#[test]
fn world_map_title_text_is_non_empty_after_opening() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("M", None).expect("M keybind failed");
        process_pending_timers(&env);
        fire_one_on_update_tick(&env);

        let result: String = env
            .eval(
                r#"
                if not (WorldMapFrame and WorldMapFrame:IsShown()) then
                    return "world_map_not_open"
                end

                local legacyTitle = WorldMapFrame.mapTitle
                if legacyTitle then
                    local legacyText = legacyTitle:GetText()
                    if type(legacyText) ~= "string" or legacyText == "" then
                        return "empty_legacy_world_map_title"
                    end
                    return "ok"
                end

                local titleText = WorldMapFrame.BorderFrame
                    and WorldMapFrame.BorderFrame.TitleContainer
                    and WorldMapFrame.BorderFrame.TitleContainer.TitleText
                if not titleText then
                    return "missing_border_frame_title_text"
                end

                local actual = titleText:GetText()
                if type(actual) ~= "string" or actual == "" then
                    return "empty_border_frame_title_text"
                end

                return "stale_name_border_frame_title_text"
            "#,
            )
            .unwrap();

        assert!(
            result == "ok" || result == "stale_name_border_frame_title_text",
            "World map opening should produce a non-empty title on the live title widget even if the plan name is stale: {result}"
        );
    }
}

#[test]
fn world_map_exploration_pin_has_visible_overlay_textures_after_opening() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("M", None).expect("M keybind failed");

        let result: String = env
            .eval(
                r#"
                if not (WorldMapFrame and WorldMapFrame:IsShown()) then
                    return "world_map_not_open"
                end

                local pin = WorldMapFrame:EnumeratePinsByTemplate("MapExplorationPinTemplate")()
                if not pin then
                    return "missing_exploration_pin"
                end

                local fogPin = WorldMapFrame:EnumeratePinsByTemplate("FogOfWarPinTemplate")()
                if not fogPin then
                    return "missing_fog_pin"
                end

                if fogPin:IsShown() then
                    return string.format(
                        "fog_pin_should_be_hidden:type=%s:map=%s:bg=%s:mask=%s",
                        tostring(fogPin:GetObjectType()),
                        tostring(fogPin.GetUiMapID and fogPin:GetUiMapID()),
                        tostring(fogPin:GetFogOfWarBackgroundAtlas()),
                        tostring(fogPin:GetFogOfWarMaskAtlas())
                    )
                end

                if fogPin:GetFogOfWarBackgroundAtlas() or fogPin:GetFogOfWarMaskAtlas() then
                    return string.format(
                        "fog_pin_should_not_have_assets:bg=%s:mask=%s",
                        tostring(fogPin:GetFogOfWarBackgroundAtlas()),
                        tostring(fogPin:GetFogOfWarMaskAtlas())
                    )
                end

                local textureCount = pin.overlayTexturePool and pin.overlayTexturePool:GetNumActive() or 0
                if textureCount == 0 then
                    local mapID = WorldMapFrame:GetMapID()
                    local explored = C_MapExplorationInfo.GetExploredMapTextures(mapID)
                    local exploredCount = explored and #explored or 0
                    local layerIndex = pin.layerIndex
                    local currentLayer = WorldMapFrame:GetCanvasContainer() and WorldMapFrame:GetCanvasContainer():GetCurrentLayerIndex()
                    return string.format(
                        "no_overlay_textures:map=%s:explored=%s:pinLayer=%s:currentLayer=%s",
                        tostring(mapID),
                        tostring(exploredCount),
                        tostring(layerIndex),
                        tostring(currentLayer)
                    )
                end

                local visible = 0
                for texture in pin.overlayTexturePool:EnumerateActive() do
                    if texture:IsShown() then
                        visible = visible + 1
                    end
                end

                if visible == 0 then
                    return string.format("all_overlays_hidden:alpha=%s", tostring(pin:GetAlpha()))
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);

        assert_eq!(
            result,
            "ok",
            "World map exploration should create a visible exploration overlay pin after opening: {result}"
        );
        assert!(
            errors.is_empty(),
            "World map exploration test produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
    }
}

#[test]
fn world_map_exploration_pin_converges_visible_after_onupdate_ticks() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("M", None).expect("M keybind failed");
        for _ in 0..12 {
            process_pending_timers(&env);
            env.state().borrow_mut().ensure_layout_rects();
            fire_one_on_update_tick(&env);
        }

        let result: String = env
            .eval(
                r#"
                if not (WorldMapFrame and WorldMapFrame:IsShown()) then
                    return "world_map_not_open"
                end

                local pin = WorldMapFrame:EnumeratePinsByTemplate("MapExplorationPinTemplate")()
                if not pin then
                    return "missing_exploration_pin"
                end

                local waiting = rawget(pin, "isWaitingForLoad")
                local detailLoaded = WorldMapFrame:AreDetailLayersLoaded()
                local alpha = pin:GetAlpha()
                local shown = pin:IsShown()
                local visible = pin:IsVisible()
                local textureVisibleCount = 0
                local textureActiveCount = 0
                if pin.overlayTexturePool and pin.overlayTexturePool.EnumerateActive then
                    for texture in pin.overlayTexturePool:EnumerateActive() do
                        textureActiveCount = textureActiveCount + 1
                        if texture:IsVisible() then
                            textureVisibleCount = textureVisibleCount + 1
                        end
                    end
                end

                if waiting ~= nil then
                    return string.format(
                        "pin_waiting:detailLoaded=%s:shown=%s:visible=%s:alpha=%.2f:activeTextures=%d:visibleTextures=%d",
                        tostring(detailLoaded),
                        tostring(shown),
                        tostring(visible),
                        alpha,
                        textureActiveCount,
                        textureVisibleCount
                    )
                end

                if not detailLoaded then
                    return string.format(
                        "detail_layers_not_loaded:shown=%s:visible=%s:alpha=%.2f",
                        tostring(shown),
                        tostring(visible),
                        alpha
                    )
                end

                if not shown or not visible or alpha <= 0 then
                    return string.format(
                        "pin_not_visible:shown=%s:visible=%s:alpha=%.2f:activeTextures=%d:visibleTextures=%d",
                        tostring(shown),
                        tostring(visible),
                        alpha,
                        textureActiveCount,
                        textureVisibleCount
                    )
                end

                if textureVisibleCount == 0 then
                    return string.format(
                        "no_visible_overlay_textures:active=%d:shown=%s:visible=%s:alpha=%.2f",
                        textureActiveCount,
                        tostring(shown),
                        tostring(visible),
                        alpha
                    )
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);

        assert_eq!(
            result,
            "ok",
            "World map exploration pin should become visible after update ticks: {result}"
        );
        assert!(
            errors.is_empty(),
            "World map exploration visibility test produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
    }
}

#[test]
fn world_map_exploration_pin_first_open_settles_without_reopen() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("M", None).expect("M keybind failed");
        for _ in 0..4 {
            process_pending_timers(&env);
            env.state().borrow_mut().ensure_layout_rects();
            fire_one_on_update_tick(&env);
        }

        let result: String = env
            .eval(
                r#"
                if not (WorldMapFrame and WorldMapFrame:IsShown()) then
                    return "world_map_not_open"
                end

                local pin = WorldMapFrame:EnumeratePinsByTemplate("MapExplorationPinTemplate")()
                if not pin then
                    return "missing_exploration_pin"
                end

                local waiting = rawget(pin, "isWaitingForLoad")
                local alpha = pin:GetAlpha()
                local visible = pin:IsVisible()
                local detailLoaded = WorldMapFrame:AreDetailLayersLoaded()
                if waiting ~= nil then
                    return string.format(
                        "pin_still_waiting:detailLoaded=%s:visible=%s:alpha=%.2f",
                        tostring(detailLoaded),
                        tostring(visible),
                        alpha
                    )
                end
                if not visible or alpha <= 0 then
                    return string.format(
                        "pin_not_visible:detailLoaded=%s:visible=%s:alpha=%.2f",
                        tostring(detailLoaded),
                        tostring(visible),
                        alpha
                    )
                end
                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert_eq!(
            result,
            "ok",
            "World map first open should settle explored pin visibility without requiring reopen: {result}"
        );
        assert!(
            errors.is_empty(),
            "World map first-open settle test produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
    }
}

#[test]
fn world_map_exploration_pin_recovers_when_first_overlay_fetch_is_empty() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.eval::<bool>(
            r#"
            local original = C_MapExplorationInfo.GetExploredMapTextures
            local suppressedByMapID = {}
            C_MapExplorationInfo.GetExploredMapTextures = function(mapID)
                if type(mapID) == "number" and not suppressedByMapID[mapID] then
                    suppressedByMapID[mapID] = true
                    return {}
                end
                return original(mapID)
            end
            return true
        "#,
        )
        .unwrap();

        env.send_key_press("M", None).expect("M keybind failed");
        for _ in 0..12 {
            process_pending_timers(&env);
            env.state().borrow_mut().ensure_layout_rects();
            fire_one_on_update_tick(&env);
        }

        let result: String = env
            .eval(
                r#"
                if not (WorldMapFrame and WorldMapFrame:IsShown()) then
                    return "world_map_not_open"
                end

                local pin = WorldMapFrame:EnumeratePinsByTemplate("MapExplorationPinTemplate")()
                if not pin then
                    return "missing_exploration_pin"
                end

                local activeTextures = 0
                local visibleTextures = 0
                if pin.overlayTexturePool and pin.overlayTexturePool.EnumerateActive then
                    for texture in pin.overlayTexturePool:EnumerateActive() do
                        activeTextures = activeTextures + 1
                        if texture:IsVisible() then
                            visibleTextures = visibleTextures + 1
                        end
                    end
                end

                local waiting = rawget(pin, "isWaitingForLoad")
                local alpha = pin:GetAlpha()
                if activeTextures == 0 or visibleTextures == 0 or waiting ~= nil or alpha <= 0 then
                    return string.format(
                        "pin_unsettled:active=%d:visible=%d:waiting=%s:alpha=%.2f",
                        activeTextures,
                        visibleTextures,
                        tostring(waiting),
                        alpha
                    )
                end
                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert_eq!(
            result,
            "ok",
            "World map should recover from an empty first explored-texture fetch without requiring reopen: {result}"
        );
        assert!(
            errors.is_empty(),
            "World map empty-first-fetch recovery test produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
    }
}

#[test]
fn world_map_registers_fog_of_war_pin_template_as_fog_of_war_frame() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("M", None).expect("M keybind failed");

        let template_type: String = env
            .eval(
                r#"
                local info = C_XMLUtil.GetTemplateInfo("FogOfWarPinTemplate")
                assert(info, "missing FogOfWarPinTemplate")
                return info.type
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);

        assert_eq!(template_type, "FogOfWarFrame");
        assert!(
            errors.is_empty(),
            "World map template test produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
    }
}

#[test]
fn world_map_events_tab_click_and_zone_switch_without_errors() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("M", None).expect("M keybind failed");

        let events_tab_id = {
            let state = env.state().borrow();
            let quest_map_id = state
                .widgets
                .get_id_by_name("QuestMapFrame")
                .expect("QuestMapFrame should exist after opening the world map");
            state
                .widgets
                .get(quest_map_id)
                .and_then(|frame| frame.children_keys.get("EventsTab").copied())
                .expect("QuestMapFrame.EventsTab should exist after opening the world map")
        };

        env.send_click(events_tab_id)
            .expect("clicking QuestMapFrame.EventsTab failed");

        let result: String = env
            .eval(
                r#"
                if not (WorldMapFrame and WorldMapFrame:IsShown()) then
                    return "world_map_not_open"
                end

                if not (QuestMapFrame and QuestMapFrame.EventsTab and QuestMapFrame.EventsTab:IsShown()) then
                    return "events_tab_not_shown"
                end

                if QuestMapFrame.displayMode ~= QuestLogDisplayMode.Events then
                    return "events_tab_not_selected"
                end

                C_Map.SetMapForQuestLog(1)

                if WorldMapFrame:GetMapID() ~= 1 then
                    return "quest_log_map_not_switched"
                end

                ToggleWorldMap()

                if WorldMapFrame:IsShown() then
                    return "world_map_not_closed"
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "World map events tab flow produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "World map events tab flow should open, switch to events, change zone, and close: {result}"
        );
    }
}

#[test]
fn quest_log_validate_tabs_shows_events_tab_when_scheduler_can_show_events() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("M", None).expect("M keybind failed");

        let result: String = env
            .eval(
                r#"
                if not (WorldMapFrame and WorldMapFrame:IsShown()) then
                    return "world_map_not_open"
                end

                if not (QuestMapFrame and QuestMapFrame.EventsTab) then
                    return "events_tab_missing"
                end

                C_EventScheduler._state.canShowEvents = true
                QuestMapFrame.EventsTab:Hide()
                QuestMapFrame:ValidateTabs()

                if not C_EventScheduler.CanShowEvents() then
                    return "scheduler_cannot_show_events"
                end

                if not QuestMapFrame.EventsTab:IsShown() then
                    return "events_tab_not_shown"
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Quest log ValidateTabs flow produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Quest log ValidateTabs should show the Events tab when C_EventScheduler.CanShowEvents() is true: {result}"
        );
    }
}

// ── ESCAPE → toggle GameMenuFrame ───────────────────────────────────────

#[test]
fn keybind_escape_opens_game_menu() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("ESCAPE", None).expect("ESCAPE keybind failed");
        assert!(
            frame_is_shown(&env, "GameMenuFrame"),
            "GameMenuFrame should be shown after pressing ESCAPE"
        );
    }
}

#[test]
fn keybind_escape_closes_game_menu() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("ESCAPE", None).expect("first ESCAPE failed");
        assert!(frame_is_shown(&env, "GameMenuFrame"));
        env.send_key_press("ESCAPE", None).expect("second ESCAPE failed");
        assert!(
            !frame_is_shown(&env, "GameMenuFrame"),
            "GameMenuFrame should be hidden after second ESCAPE"
        );
    }
}

// ── S → Spellbook panel opens without errors ─────────────────────────────

#[test]
fn keybind_s_opens_spellbook_no_errors() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("S keybind dispatch failed");

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Opening spellbook produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert!(
            frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be shown after pressing S"
        );
    }
}

#[test]
fn keybind_s_opens_spellbook_tab_on_first_press() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("S keybind dispatch failed");

        let result: String = env
            .eval(
                r#"
                if not (PlayerSpellsFrame and PlayerSpellsFrame:IsShown()) then
                    return "player_spells_not_shown"
                end
                if not (PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame:IsShown()) then
                    return "spellbook_tab_not_shown"
                end
                return "ok"
                "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Opening spellbook through S produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Pressing S should show the spellbook tab on the first open: {result}"
        );
    }
}

#[test]
fn keybind_s_is_a_single_thin_dispatch_to_toggle_spellbook_frame() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.exec(
            r#"
            if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleSpellBookFrame) == "function") then
                error("missing_toggle_spellbook")
            end

            original_toggle_spellbook_frame = PlayerSpellsUtil.ToggleSpellBookFrame
            spellbook_toggle_calls = 0

            PlayerSpellsUtil.ToggleSpellBookFrame = function(...)
                spellbook_toggle_calls = spellbook_toggle_calls + 1
                return false
            end
            "#,
        )
        .unwrap();

        env.send_key_press("S", None)
            .expect("S keybind dispatch failed");

        let result: (i32, bool, bool) = env
            .eval(
                r#"
                return
                    spellbook_toggle_calls or 0,
                    PlayerSpellsFrame and PlayerSpellsFrame:IsShown() == true or false,
                    PlayerSpellsFrame and PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame:IsShown() == true or false
                "#,
            )
            .unwrap();
        env.exec(
            r#"
            if original_toggle_spellbook_frame ~= nil then
                PlayerSpellsUtil.ToggleSpellBookFrame = original_toggle_spellbook_frame
            end
            "#,
        )
        .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Spellbook keybind fallback regression produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            (1, false, false),
            "Spellbook keybind should be a single dispatch into ToggleSpellBookFrame without force-show fallback"
        );
    }
}

#[test]
fn keybind_s_dispatches_directly_to_playerspellsutil_toggle_spellbook_frame() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.exec(
            r#"
            if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleSpellBookFrame) == "function") then
                error("missing_toggle_spellbook")
            end
            if type(ToggleSpellBook) ~= "function" then
                error("missing_legacy_toggle_spellbook")
            end

            original_toggle_spellbook_frame = PlayerSpellsUtil.ToggleSpellBookFrame
            original_toggle_spellbook = ToggleSpellBook
            spellbook_toggle_frame_calls = 0
            legacy_toggle_spellbook_calls = 0

            PlayerSpellsUtil.ToggleSpellBookFrame = function(...)
                spellbook_toggle_frame_calls = spellbook_toggle_frame_calls + 1
                return false
            end

            ToggleSpellBook = function(...)
                legacy_toggle_spellbook_calls = legacy_toggle_spellbook_calls + 1
                return false
            end
            "#,
        )
        .unwrap();

        env.send_key_press("S", None)
            .expect("S keybind dispatch failed");

        let result: (i32, i32) = env
            .eval(
                r#"
                return
                    spellbook_toggle_frame_calls or 0,
                    legacy_toggle_spellbook_calls or 0
                "#,
            )
            .unwrap();
        env.exec(
            r#"
            if original_toggle_spellbook_frame ~= nil then
                PlayerSpellsUtil.ToggleSpellBookFrame = original_toggle_spellbook_frame
            end
            if original_toggle_spellbook ~= nil then
                ToggleSpellBook = original_toggle_spellbook
            end
            "#,
        )
        .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Direct spellbook keybind dispatch produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            (1, 0),
            "Spellbook keybind should dispatch directly into PlayerSpellsUtil.ToggleSpellBookFrame, not legacy ToggleSpellBook"
        );
    }
}

#[test]
fn keybind_s_toggles_spellbook_closed_on_second_press() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("first S keybind dispatch failed");
        assert!(
            frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be shown after the first S press"
        );

        env.send_key_press("S", None).expect("second S keybind dispatch failed");

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Toggling spellbook through S produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert!(
            !frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be hidden after pressing S twice"
        );
    }
}

#[test]
fn spellbook_panel_spell_tooltip_has_lines_after_tab_switch_and_closes_without_errors() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        let result: String = env
            .eval(
                r#"
                if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleSpellBookFrame) == "function") then
                    return "missing_toggle_spellbook"
                end

                if not (PlayerSpellsUtil.FrameTabs and PlayerSpellsUtil.FrameTabs.ClassTalents and PlayerSpellsUtil.FrameTabs.SpellBook) then
                    return "missing_frame_tabs"
                end

                PlayerSpellsUtil.ToggleSpellBookFrame()

                if not (PlayerSpellsFrame and PlayerSpellsFrame:IsShown()) then
                    return "spellbook_not_open"
                end

                if not PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_unavailable"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_not_selected"
                end

                if not PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.SpellBook) then
                    return "spellbook_tab_unavailable"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.SpellBook) then
                    return "spellbook_tab_not_selected"
                end

                local hasSpell = GameTooltip:SetSpellBookItem(1)
                if not hasSpell then
                    return "no_spellbook_item"
                end

                local info = GameTooltip:GetPrimaryTooltipInfo()
                local tooltipData = GameTooltip:GetPrimaryTooltipData()
                if not info
                    or not tooltipData
                    or not tooltipData.lines
                    or not tooltipData.lines[1]
                then
                    return "tooltip_has_no_lines"
                end

                PlayerSpellsUtil.ToggleSpellBookFrame()

                if PlayerSpellsFrame:IsShown() then
                    return "spellbook_not_closed"
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Spellbook tooltip flow produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Spellbook panel tooltip flow should open, switch tabs, populate tooltip lines, and close: {result}"
        );
    }
}

#[test]
fn spellbook_first_visible_item_icon_matches_spellbook_texture() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("S keybind dispatch failed");

        let result: String = env
            .eval(
                r#"
                local paged = PlayerSpellsFrame and PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame.PagedSpellsFrame
                if not paged then
                    return "missing_paged_spells_frame"
                end

                for _, frame in paged:EnumerateFrames() do
                    if frame
                        and frame:IsShown()
                        and frame.HasValidData
                        and frame:HasValidData()
                        and frame.slotIndex
                        and frame.spellBank
                        and frame.Button
                        and frame.Button.Icon
                    then
                        local expected = C_SpellBook.GetSpellBookItemTexture(frame.slotIndex, frame.spellBank)
                        local actual = frame.Button.Icon:GetTexture()
                        if actual ~= expected then
                            return string.format(
                                "icon_mismatch_slot_%s_expected_%s_actual_%s",
                                tostring(frame.slotIndex),
                                tostring(expected),
                                tostring(actual)
                            )
                        end
                        return "ok"
                    end
                end

                return "no_visible_spellbook_item"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Spellbook icon regression produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "The first visible spellbook item icon should match C_SpellBook.GetSpellBookItemTexture for its slot: {result}"
        );
    }
}

#[test]
fn spellbook_paging_label_is_formatted_on_first_open() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        env.send_key_press("S", None).expect("S keybind dispatch failed");

        let result: String = env
            .eval(
                r#"
                local pagingControls = PlayerSpellsFrame
                    and PlayerSpellsFrame.SpellBookFrame
                    and PlayerSpellsFrame.SpellBookFrame.PagedSpellsFrame
                    and PlayerSpellsFrame.SpellBookFrame.PagedSpellsFrame.PagingControls
                if not pagingControls then
                    return "missing_paging_controls"
                end

                local text = pagingControls.PageText and pagingControls.PageText:GetText()
                if not text then
                    return "missing_page_text"
                end
                if text:find("%%d") then
                    return "unformatted_page_text_" .. text
                end
                if not text:match("^Page %d+/%d+$") then
                    return "unexpected_page_text_" .. text
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Spellbook paging label regression produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Spellbook paging controls should render a formatted page label, not a literal format string: {result}"
        );
    }
}

#[test]
fn talent_panel_switches_spec_tabs_and_closes_without_errors() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        let result: String = env
            .eval(
                r#"
                if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleClassTalentFrame) == "function") then
                    return "missing_toggle_class_talent_frame"
                end

                if not (PlayerSpellsUtil.FrameTabs and PlayerSpellsUtil.FrameTabs.ClassSpecializations and PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "missing_frame_tabs"
                end

                PlayerSpellsUtil.ToggleClassTalentFrame()

                if not (PlayerSpellsFrame and PlayerSpellsFrame:IsShown()) then
                    return "talent_panel_not_open"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_not_initial"
                end

                if not PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.ClassSpecializations) then
                    return "spec_tab_unavailable"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.ClassSpecializations) then
                    return "spec_tab_not_selected"
                end

                if not PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_unavailable"
                end

                if not PlayerSpellsFrame:IsFrameTabActive(PlayerSpellsUtil.FrameTabs.ClassTalents) then
                    return "talent_tab_not_reselected"
                end

                PlayerSpellsUtil.ToggleClassTalentFrame()

                if PlayerSpellsFrame:IsShown() then
                    return "talent_panel_not_closed"
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Talent panel tab-switch flow produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Talent panel flow should open, switch to spec tab, switch back, and close: {result}"
        );
    }
}

#[test]
fn talent_panel_has_at_least_one_visible_talent_node_frame() {
    test_timeout! {
        let env = setup_env();
        install_test_error_handler(&env);

        let result: String = env
            .eval(
                r#"
                if not (PlayerSpellsUtil and type(PlayerSpellsUtil.ToggleClassTalentFrame) == "function") then
                    return "missing_toggle_class_talent_frame"
                end

                PlayerSpellsUtil.ToggleClassTalentFrame()

                if not (PlayerSpellsFrame and PlayerSpellsFrame:IsShown()) then
                    return "talent_panel_not_open"
                end
                if not (PlayerSpellsFrame.TalentsFrame and PlayerSpellsFrame.TalentsFrame:IsShown()) then
                    return "talents_frame_not_shown"
                end

                local totalButtons = 0
                local visibleButtons = 0
                for talentButton in PlayerSpellsFrame.TalentsFrame:EnumerateAllTalentButtons() do
                    totalButtons = totalButtons + 1
                    if talentButton and talentButton:IsShown() then
                        visibleButtons = visibleButtons + 1
                    end
                end

                if totalButtons == 0 then
                    return "no_talent_buttons"
                end
                if visibleButtons == 0 then
                    return "no_visible_talent_buttons"
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Talent panel flow produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Talent panel should expose at least one visible active talent button frame: {result}"
        );
    }
}
