//! Integration tests for main action bar visibility after startup.
//!
//! Verifies that loading Blizzard addons and firing startup events results
//! in the MainActionBar and its 12 ActionButton children being visible.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn blizzard_toc(addon: &str, toc_name: &str) -> PathBuf {
    blizzard_ui_dir().join(addon).join(toc_name)
}

/// Blizzard addons needed for the action bar, in dependency order.
const ACTION_BAR_ADDONS: &[(&str, &str)] = &[
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
];

fn build_action_bar_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    // Set addon_base_paths so runtime LoadAddOn() can find on-demand addons.
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    for (name, toc) in ACTION_BAR_ADDONS {
        let toc_path = blizzard_toc(name, toc);
        if !toc_path.exists() {
            continue;
        }
        if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("{name} failed: {e}");
        }
    }

    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

/// Load addons, fire startup events, and apply post-startup fixups.
fn env_with_action_bar() -> common::LockedEnv {
    common::lock_env(build_action_bar_env)
}

/// Replicate the startup event sequence from main.rs / app.rs.
fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::call_global_if_present(env, "RequestTimePlayed");
    common::fire_player_entering_world(env, true, false);
    let _ = env.fire_edit_mode_layouts_updated();

    // WoW's C++ engine fires ACTIONBAR_SHOWGRID on startup to show empty slots.
    let _ = env.fire_event("ACTIONBAR_SHOWGRID");

    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }
}

/// MultiBarLeft and MultiBarRight (right-side action bars) should be hidden
/// by default — they're only shown when the player enables them via
/// PROXY_SHOW_ACTIONBAR_4/5 settings (backed by GetActionBarToggles).
#[test]
fn test_right_action_bars_hidden_by_default() {
    let env = env_with_action_bar();

    let results: (bool, bool) = env
        .eval(
            r#"
            local left = MultiBarLeft and MultiBarLeft:IsVisible()
            local right = MultiBarRight and MultiBarRight:IsVisible()
            return left or false, right or false
        "#,
        )
        .unwrap();
    assert!(!results.0, "MultiBarLeft should be hidden by default");
    assert!(!results.1, "MultiBarRight should be hidden by default");
}

#[test]
fn test_main_action_bar_visible_after_startup() {
    let env = env_with_action_bar();

    let visible: bool = env
        .eval("return MainActionBar ~= nil and MainActionBar:IsVisible()")
        .unwrap();
    assert!(visible, "MainActionBar should be visible after startup");
}

#[test]
fn test_main_action_bar_end_caps_visible_after_startup() {
    let env = env_with_action_bar();

    let (shown, left_atlas, right_atlas): (bool, String, String) = env
        .eval(
            r#"
            if not MainActionBar or not MainActionBar.EndCaps then
                return false, "", ""
            end
            local left = MainActionBar.EndCaps.LeftEndCap
            local right = MainActionBar.EndCaps.RightEndCap
            return MainActionBar.EndCaps:IsShown(),
                   (left and left.GetAtlas and left:GetAtlas()) or "",
                   (right and right.GetAtlas and right:GetAtlas()) or ""
            "#,
        )
        .unwrap();

    assert!(shown, "MainActionBar.EndCaps should be shown after startup");
    assert_eq!(left_atlas, "ui-hud-actionbar-gryphon-left");
    assert_eq!(right_atlas, "ui-hud-actionbar-gryphon-right");
}

#[test]
fn test_action_buttons_visible_after_startup() {
    let env = env_with_action_bar();

    let count: i32 = env
        .eval(
            r#"
            local n = 0
            for i = 1, 12 do
                local btn = _G["ActionButton" .. i]
                if btn and btn:IsVisible() then
                    n = n + 1
                end
            end
            return n
        "#,
        )
        .unwrap();
    assert_eq!(count, 12, "All 12 ActionButtons should be visible");
}

#[test]
fn test_action_buttons_have_showgrid_attribute() {
    let env = env_with_action_bar();

    let all_have_grid: bool = env
        .eval(
            r#"
            for i = 1, 12 do
                local btn = _G["ActionButton" .. i]
                if not btn then return false end
                local grid = btn:GetAttribute("showgrid")
                if not grid or grid <= 0 then return false end
            end
            return true
        "#,
        )
        .unwrap();
    assert!(all_have_grid, "All ActionButtons should have showgrid > 0");
}

#[test]
fn test_action_button_size() {
    let env = env_with_action_bar();

    let size: (f64, f64) = env
        .eval(
            r#"
            local btn = ActionButton1
            if not btn then return 0, 0 end
            return btn:GetSize()
        "#,
        )
        .unwrap();
    assert_eq!(size, (45.0, 45.0), "ActionButton should be 45x45");
}

#[test]
fn test_action_bar_env_can_bootstrap_twice_in_same_process() {
    common::with_perf_lock(|| {
        let first = build_action_bar_env();
        let first_visible: bool = first
            .eval("return MainActionBar ~= nil and MainActionBar:IsVisible()")
            .unwrap();
        assert!(first_visible, "first action bar env should initialize");
        drop(first);

        let second = build_action_bar_env();
        let second_visible: bool = second
            .eval("return MainActionBar ~= nil and MainActionBar:IsVisible()")
            .unwrap();
        assert!(second_visible, "second action bar env should initialize");
    });
}

#[test]
fn test_main_action_bar_size() {
    let env = env_with_action_bar();

    let size: (f64, f64) = env
        .eval(
            r#"
            local bar = MainActionBar
            if not bar then return 0, 0 end
            return bar:GetSize()
        "#,
        )
        .unwrap();
    // 12 buttons (45px) + 11 gaps (2px) from the button grid layout.
    assert_eq!(
        size,
        (562.0, 45.0),
        "MainActionBar should match the button grid after layout"
    );
}

/// Verify the core EditMode enter/exit flow works without pcall.
///
/// AccountSettings:OnEditModeEnter/Exit are excluded — they call
/// Setup/Refresh on ~30 optional frames (DurabilityFrame, TargetFrame, etc.)
/// that may not be loaded in partial test environments.
#[test]
fn test_enter_exit_edit_mode_core_steps() {
    let env = env_with_action_bar();
    env.apply_post_event_workarounds();
    let failures: String = env
        .eval(
            r#"
            local emm = EditModeManagerFrame
            local fails = {}
            local function try(name, fn)
                local ok, err = pcall(fn)
                if not ok then fails[#fails+1] = name .. ": " .. tostring(err) end
            end
            emm.editModeActive = true
            try("ClearActiveChangesFlags", function() emm:ClearActiveChangesFlags() end)
            try("ShowSystemSelections", function() emm:ShowSystemSelections() end)
            try("TriggerEvent_Enter", function() EventRegistry:TriggerEvent("EditMode.Enter") end)
            emm.editModeActive = false
            try("HideSystemSelections", function() emm:HideSystemSelections() end)
            try("TriggerEvent_Exit", function() EventRegistry:TriggerEvent("EditMode.Exit") end)
            return table.concat(fails, "\n")
        "#,
        )
        .unwrap();
    assert!(
        failures.is_empty(),
        "EditMode core steps should not crash:\n{}",
        failures
    );
}
