//! Integration tests for the ShowUIPanel / HideUIPanel panel slot system.
//!
//! Verifies that:
//! - UIPanelWindows is populated with panel entries after addons load
//! - ShowUIPanel shows a panel and HideUIPanel hides it
//! - Panel area attributes (left/center/right) are registered correctly

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Blizzard addons needed for the panel system (dependency order).
const PANEL_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    (
        "Blizzard_SharedXMLGame",
        "Blizzard_SharedXMLGame_Mainline.toc",
    ),
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
    (
        "Blizzard_FrameXMLUtil",
        "Blizzard_FrameXMLUtil_Mainline.toc",
    ),
    ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
    ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    (
        "Blizzard_UIPanels_Game",
        "Blizzard_UIPanels_Game_Mainline.toc",
    ),
];

fn setup_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    let ui = blizzard_ui_dir();
    for (name, toc) in PANEL_ADDONS {
        let toc_path = ui.join(name).join(toc);
        if !toc_path.exists() {
            continue;
        }
        if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }

    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }
}

fn install_test_error_handler(env: &WowLuaEnv) {
    common::install_error_collector(env, "__test_errors");
}

fn drain_test_errors(env: &WowLuaEnv) -> Vec<String> {
    common::drain_string_table(env, "__test_errors")
}

#[test]
fn uipanel_windows_has_entries() {
    test_timeout! {
        let env = setup_env();
        let count: i32 = env.eval(r#"
            local count = 0
            for _ in pairs(UIPanelWindows) do count = count + 1 end
            return count
        "#).unwrap();
        eprintln!("UIPanelWindows entries: {count}");
        assert!(count > 0, "UIPanelWindows should have entries after loading Blizzard addons");
    }
}

#[test]
fn uipanel_windows_has_area_attributes() {
    test_timeout! {
        let env = setup_env();
        // CharacterFrame is registered by Blizzard_UIPanels_Game with area info
        let has_area: bool = env.eval(r#"
            local entry = UIPanelWindows["CharacterFrame"]
            if not entry then return false end
            -- Entry should have an area field
            return entry.area ~= nil
        "#).unwrap();
        assert!(has_area, "CharacterFrame should have an area attribute in UIPanelWindows");
    }
}

#[test]
fn key_blizzard_panels_registered() {
    test_timeout! {
        let env = setup_env();
        // Core Blizzard panels loaded from UIPanelWindows.lua should all be registered.
        // LoD panels (AchievementFrame, CollectionsJournal, etc.) register when their
        // addon loads — they won't be here, and that's correct WoW behavior.
        let missing: String = env.eval(r#"
            local expected = {
                -- From Blizzard_UIParentPanelManager/Mainline/UIPanelWindows.lua
                {"CharacterFrame", "left"},
                {"FriendsFrame", "left"},
                {"GameMenuFrame", "center"},
                {"HelpFrame", "center"},
                {"ProfessionsBookFrame", "left"},
                {"PVEFrame", "left"},
                {"MailFrame", "left"},
                {"MerchantFrame", "left"},
                {"BankFrame", "left"},
                {"GossipFrame", "left"},
                {"DressUpFrame", "left"},
                {"QuestFrame", "left"},
                {"CommunitiesFrame", "left"},
            }
            local missing = {}
            for _, pair in ipairs(expected) do
                local name, area = pair[1], pair[2]
                local entry = UIPanelWindows[name]
                if not entry then
                    table.insert(missing, name .. "(not registered)")
                elseif entry.area ~= area then
                    table.insert(missing, name .. "(area=" .. tostring(entry.area) .. " expected=" .. area .. ")")
                end
            end
            return table.concat(missing, ", ")
        "#).unwrap();
        assert!(missing.is_empty(), "Missing or wrong UIPanelWindows entries: {missing}");
    }
}

#[test]
fn show_ui_panel_shows_frame() {
    test_timeout! {
        let env = setup_env();
        let result: bool = env.eval(r#"
            if not CharacterFrame then return false end
            -- Should start hidden
            if CharacterFrame:IsShown() then return false end
            ShowUIPanel(CharacterFrame)
            return CharacterFrame:IsShown() == true
        "#).unwrap();
        assert!(result, "ShowUIPanel should show CharacterFrame");
    }
}

#[test]
fn show_ui_panel_anchors_character_frame_to_ui_parent() {
    test_timeout! {
        let env = setup_env();
        let result: String = env
            .eval(
                r#"
                if not CharacterFrame then
                    return "missing_character_frame"
                end

                ShowUIPanel(CharacterFrame)
                if not CharacterFrame:IsShown() then
                    return "character_not_shown"
                end

                local numPoints = CharacterFrame:GetNumPoints()
                if numPoints ~= 1 then
                    return "num_points=" .. tostring(numPoints)
                end

                local point, relativeTo, relativePoint, x, y = CharacterFrame:GetPoint(1)
                if point ~= "TOPLEFT" then
                    return "point=" .. tostring(point)
                end
                if not relativeTo or relativeTo ~= UIParent then
                    return "relative_to=" .. tostring(relativeTo and relativeTo:GetName())
                end
                if relativePoint ~= "TOPLEFT" then
                    return "relative_point=" .. tostring(relativePoint)
                end
                if x ~= 16 or y ~= -116 then
                    return string.format("offsets=%s,%s", tostring(x), tostring(y))
                end

                local left, bottom, width, height = CharacterFrame:GetRect()
                if not (left and bottom and width and height) then
                    return "missing_rect"
                end

                return "ok"
            "#,
            )
            .unwrap();
        assert_eq!(
            result, "ok",
            "ShowUIPanel should anchor CharacterFrame to UIParent and resolve its rect: {result}"
        );
    }
}

#[test]
fn show_ui_panel_reanchors_character_frame_after_reopen() {
    test_timeout! {
        let env = setup_env();
        let result: String = env
            .eval(
                r#"
                if not CharacterFrame then
                    return "missing_character_frame"
                end

                ShowUIPanel(CharacterFrame)
                if not CharacterFrame:IsShown() then
                    return "character_not_shown_first"
                end
                HideUIPanel(CharacterFrame)
                if CharacterFrame:IsShown() then
                    return "character_not_hidden"
                end

                ShowUIPanel(CharacterFrame)
                if not CharacterFrame:IsShown() then
                    return "character_not_shown_second"
                end

                local numPoints = CharacterFrame:GetNumPoints()
                if numPoints ~= 1 then
                    return "num_points=" .. tostring(numPoints)
                end

                local point, relativeTo, relativePoint, x, y = CharacterFrame:GetPoint(1)
                if point ~= "TOPLEFT" then
                    return "point=" .. tostring(point)
                end
                if not relativeTo or relativeTo ~= UIParent then
                    return "relative_to=" .. tostring(relativeTo and relativeTo:GetName())
                end
                if relativePoint ~= "TOPLEFT" then
                    return "relative_point=" .. tostring(relativePoint)
                end
                if x ~= 16 or y ~= -116 then
                    return string.format("offsets=%s,%s", tostring(x), tostring(y))
                end

                return "ok"
            "#,
            )
            .unwrap();
        assert_eq!(
            result, "ok",
            "ShowUIPanel should restore CharacterFrame anchors after reopen: {result}"
        );
    }
}

#[test]
fn hide_ui_panel_hides_frame() {
    test_timeout! {
        let env = setup_env();
        let result: bool = env.eval(r#"
            if not CharacterFrame then return false end
            ShowUIPanel(CharacterFrame)
            if not CharacterFrame:IsShown() then return false end
            HideUIPanel(CharacterFrame)
            return CharacterFrame:IsShown() == false
        "#).unwrap();
        assert!(result, "HideUIPanel should hide CharacterFrame after ShowUIPanel");
    }
}

#[test]
fn show_and_hide_ui_panel_toggle_registered_frame_visibility() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            local panel = CreateFrame("Frame", "ShowHideUIPanelTestFrame", UIParent)
            panel:SetSize(300, 400)
            panel:Hide()
            RegisterUIPanel(panel, { area = "center", pushable = 0, whileDead = 1, allowOtherPanels = 1 })

            if panel:IsShown() then
                return "panel_started_shown"
            end

            ShowUIPanel(panel)
            if not panel:IsShown() then
                return "show_failed"
            end

            HideUIPanel(panel)
            if panel:IsShown() then
                return "hide_failed"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(
            result,
            "ok",
            "ShowUIPanel/HideUIPanel should toggle visibility for a registered panel: {result}"
        );
    }
}

#[test]
fn show_ui_panel_is_function() {
    test_timeout! {
        let env = setup_env();
        let show_type: String = env.eval("return type(ShowUIPanel)").unwrap();
        assert_eq!(show_type, "function", "ShowUIPanel should be a function");
        let hide_type: String = env.eval("return type(HideUIPanel)").unwrap();
        assert_eq!(hide_type, "function", "HideUIPanel should be a function");
    }
}

#[test]
fn register_ui_panel_populates_uipanel_windows_without_overwriting() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            local panel = CreateFrame("Frame", "RegisterUIPanelTestFrame", UIParent)
            panel:SetSize(300, 400)
            panel:Hide()

            RegisterUIPanel(panel, { area = "center", pushable = 0, whileDead = 1, allowOtherPanels = 1 })
            local entry = UIPanelWindows["RegisterUIPanelTestFrame"]
            if not entry then
                return "missing_entry"
            end
            if entry.area ~= "center" or entry.pushable ~= 0 or entry.whileDead ~= 1 then
                return "wrong_attributes"
            end

            RegisterUIPanel(panel, { area = "left", pushable = 3, whileDead = 0 })
            local entry_after_second_register = UIPanelWindows["RegisterUIPanelTestFrame"]
            if entry_after_second_register.area ~= "center" or entry_after_second_register.pushable ~= 0 or entry_after_second_register.whileDead ~= 1 then
                return "overwrote_existing_entry"
            end

            ShowUIPanel(panel)
            if not panel:IsShown() then
                return "show_failed"
            end

            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "RegisterUIPanel should populate UIPanelWindows once and enable ShowUIPanel: {result}");
    }
}

#[test]
fn startup_without_world_map_does_not_error_in_quest_map_refresh() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        {
            let mut state = env.state().borrow_mut();
            state.addon_base_paths = vec![blizzard_ui_dir()];
        }

        let ui = blizzard_ui_dir();
        for (name, toc) in PANEL_ADDONS {
            let toc_path = ui.join(name).join(toc);
            if !toc_path.exists() {
                continue;
            }
            if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
                eprintln!("[load {name}] FAILED: {e}");
            }
        }

        env.apply_post_load_workarounds();
        install_test_error_handler(&env);
        fire_startup_events(&env);

        let quest_map_parent_errors: Vec<String> = drain_test_errors(&env)
            .into_iter()
            .filter(|msg| msg.contains("QuestMapFrame.lua:"))
            .collect();

        assert!(
            quest_map_parent_errors.is_empty(),
            "QuestMapFrame startup refresh should tolerate a missing world map parent: {quest_map_parent_errors:#?}"
        );
    }
}
