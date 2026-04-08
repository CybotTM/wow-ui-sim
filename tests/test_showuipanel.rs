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
/// Extra addons needed for spellbook tests (loaded on demand in real WoW,
/// but we load them explicitly here for deterministic testing).
const SPELLBOOK_ADDONS: &[(&str, &str)] = &[("Blizzard_PlayerSpells", "Blizzard_PlayerSpells.toc")];

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
    let lua = env.lua();
    let _ = env.fire_event_with_args(
        "ADDON_LOADED",
        &[mlua::Value::String(lua.create_string("WoWUISim").unwrap())],
    );
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    let _ = env.fire_event_with_args(
        "PLAYER_ENTERING_WORLD",
        &[mlua::Value::Boolean(true), mlua::Value::Boolean(false)],
    );
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }
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
fn show_ui_panel_displaces_previous_occupant() {
    test_timeout! {
        let env = setup_env();
        // UIParentPanelManager manages left/center/right slots. When two panels both
        // have pushable=0 and a left slot is occupied, the new panel replaces the old.
        // CharacterFrame (pushable=3) gets pushed to center instead of replaced.
        //
        // Test 1: Pushable panel gets pushed to center (not closed)
        // CharacterFrame (pushable=3) in left, then FriendsFrame (pushable=0) opens
        // → CharacterFrame pushed to center, both visible
        let result: String = env.eval(r#"
            if not CharacterFrame or not FriendsFrame then
                return "missing_frames"
            end
            ShowUIPanel(CharacterFrame)
            if not CharacterFrame:IsShown() then return "char_not_shown" end
            ShowUIPanel(FriendsFrame)
            if not FriendsFrame:IsShown() then return "friends_not_shown" end
            -- CharacterFrame should be pushed to center, still visible
            if not CharacterFrame:IsShown() then return "char_should_be_pushed_not_closed" end
            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "Pushable panel should be pushed to center, not closed: {result}");

        // Test 2: Non-pushable panel replaces another non-pushable panel
        // When both panels have pushable=0, the old one is replaced (hidden).
        let result: String = env.eval(r#"
            -- Close everything first
            CloseAllWindows()
            -- Register two test panels with pushable=0
            local a = CreateFrame("Frame", "TestPanelA", UIParent)
            a:SetSize(300, 400)
            a:Hide()
            UIPanelWindows["TestPanelA"] = { area = "left", pushable = 0, whileDead = 1 }
            local b = CreateFrame("Frame", "TestPanelB", UIParent)
            b:SetSize(300, 400)
            b:Hide()
            UIPanelWindows["TestPanelB"] = { area = "left", pushable = 0, whileDead = 1 }

            ShowUIPanel(a)
            if not a:IsShown() then return "a_not_shown" end
            ShowUIPanel(b)
            if not b:IsShown() then return "b_not_shown" end
            if a:IsShown() then return "a_not_replaced" end
            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "Non-pushable panel should replace previous non-pushable occupant: {result}");
    }
}

#[test]
fn character_and_spellbook_coexist() {
    test_timeout! {
        let env = setup_env();

        // Load PlayerSpells addon (normally LoD, loaded on demand)
        let ui = blizzard_ui_dir();
        for (name, toc) in SPELLBOOK_ADDONS {
            let toc_path = ui.join(name).join(toc);
            if toc_path.exists() {
                if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
                    eprintln!("[load {name}] FAILED: {e}");
                }
            }
        }

        // CharacterFrame: area="left", pushable=3
        // PlayerSpellsFrame: area="centerOrLeft", pushable=3, allowOtherPanels=1
        // Both are pushable and allow other panels, so they coexist:
        // Character stays in left slot, Spellbook goes to center.
        let result: String = env.eval(r#"
            if not CharacterFrame then return "no_char_frame" end
            if not PlayerSpellsFrame then return "no_spellbook_frame" end

            ShowUIPanel(CharacterFrame)
            if not CharacterFrame:IsShown() then return "char_not_shown" end

            ShowUIPanel(PlayerSpellsFrame)
            if not PlayerSpellsFrame:IsShown() then return "spellbook_not_shown" end

            -- Both should be visible: CharacterFrame in left, PlayerSpellsFrame in center
            if not CharacterFrame:IsShown() then return "char_closed_unexpectedly" end
            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "Character and Spellbook panels should coexist (left + center): {result}");
    }
}
