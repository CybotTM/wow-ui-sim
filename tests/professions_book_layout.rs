//! Layout regression test for the classic ProfessionsBookFrame.
//!
//! Opens the professions book via ProfessionMicroButton and asserts the
//! positions of PrimaryProfession1.SpellButton1/SpellButton2 relative to the
//! primary profession row. Mirrors the same test on master so layouts can be
//! cross-checked across branches.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

const BLIZZARD_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    ("Blizzard_SharedXMLGame", "Blizzard_SharedXMLGame_Mainline.toc"),
    ("Blizzard_UIPanelTemplates", "Blizzard_UIPanelTemplates_Mainline.toc"),
    ("Blizzard_FrameXMLBase", "Blizzard_FrameXMLBase_Mainline.toc"),
    ("Blizzard_FrameEffects", "Blizzard_FrameEffects.toc"),
    ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
    ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
    ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
    ("Blizzard_AccessibilityTemplates", "Blizzard_AccessibilityTemplates.toc"),
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
    ("Blizzard_Settings_Shared", "Blizzard_Settings_Shared_Mainline.toc"),
    (
        "Blizzard_SettingsDefinitions_Shared",
        "Blizzard_SettingsDefinitions_Shared.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Frame",
        "Blizzard_SettingsDefinitions_Frame_Mainline.toc",
    ),
    ("Blizzard_FrameXMLUtil", "Blizzard_FrameXMLUtil_Mainline.toc"),
    ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
    ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    ("Blizzard_UIPanels_Game", "Blizzard_UIPanels_Game_Mainline.toc"),
    ("Blizzard_MapCanvasSecureUtil", "Blizzard_MapCanvasSecureUtil.toc"),
    ("Blizzard_MapCanvas", "Blizzard_MapCanvas.toc"),
    (
        "Blizzard_SharedMapDataProviders",
        "Blizzard_SharedMapDataProviders_Mainline.toc",
    ),
    ("Blizzard_WorldMap", "Blizzard_WorldMap_Mainline.toc"),
    ("Blizzard_ActionBar", "Blizzard_ActionBar_Mainline.toc"),
    ("Blizzard_GameMenu", "Blizzard_GameMenu_Mainline.toc"),
    ("Blizzard_UIWidgets", "Blizzard_UIWidgets_Mainline.toc"),
    ("Blizzard_Minimap", "Blizzard_Minimap_Mainline.toc"),
    ("Blizzard_AddOnList", "Blizzard_AddOnList.toc"),
    ("Blizzard_TimerunningUtil", "Blizzard_TimerunningUtil.toc"),
    ("Blizzard_Communities", "Blizzard_Communities_Mainline.toc"),
];

fn setup_env() -> WowLuaEnv {
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

fn click(env: &WowLuaEnv, name: &str) {
    env.exec(&format!(
        r#"
        local btn = {name}
        assert(btn, "{name} missing")
        local on = btn:GetScript("OnClick")
        assert(on, "{name} has no OnClick")
        on(btn, "LeftButton", false)
        "#
    ))
    .expect("click failed");
}

fn rect(env: &WowLuaEnv, expr: &str) -> (f64, f64, f64, f64) {
    env.eval::<(f64, f64, f64, f64)>(&format!(
        "local f = {expr}; return f:GetLeft() or 0, f:GetBottom() or 0, f:GetWidth() or 0, f:GetHeight() or 0"
    ))
    .unwrap_or((0.0, 0.0, 0.0, 0.0))
}

#[test]
fn professions_book_primary_spell_buttons_layout() {
    let env = setup_env();
    click(&env, "ProfessionMicroButton");

    let shown: bool = env
        .eval("return ProfessionsBookFrame ~= nil and ProfessionsBookFrame:IsShown() == true")
        .unwrap_or(false);
    assert!(shown, "ProfessionsBookFrame should be shown");

    let b1 = rect(&env, "PrimaryProfession1.SpellButton1");
    let b2 = rect(&env, "PrimaryProfession1.SpellButton2");
    let primary = rect(&env, "PrimaryProfession1");

    eprintln!("PrimaryProfession1 L={} B={} W={} H={}", primary.0, primary.1, primary.2, primary.3);
    eprintln!("SpellButton1         L={} B={} W={} H={}", b1.0, b1.1, b1.2, b1.3);
    eprintln!("SpellButton2         L={} B={} W={} H={}", b2.0, b2.1, b2.2, b2.3);

    assert!(b1.2 > 0.0 && b1.3 > 0.0, "SpellButton1 must have size");
    assert!(b2.2 > 0.0 && b2.3 > 0.0, "SpellButton2 must have size");

    let b1_top = b1.1 + b1.3;
    let b2_bottom = b2.1;
    assert!(
        (b1_top - b2_bottom).abs() < 1.0,
        "SpellButton1 top ({b1_top}) should touch SpellButton2 bottom ({b2_bottom})"
    );

    assert!(
        (b1.0 - b2.0).abs() < 1.0,
        "SpellButton1 left ({}) should match SpellButton2 left ({})",
        b1.0, b2.0
    );

    let primary_right = primary.0 + primary.2;
    let primary_top = primary.1 + primary.3;
    let b2_right = b2.0 + b2.2;
    let b2_top = b2.1 + b2.3;
    assert!(
        (primary_right - b2_right - 109.0).abs() < 1.0,
        "SpellButton2 right ({b2_right}) should be 109 px inside PrimaryProfession1 right ({primary_right})"
    );
    assert!(
        (primary_top - b2_top - 3.0).abs() < 1.0,
        "SpellButton2 top ({b2_top}) should be 3 px below PrimaryProfession1 top ({primary_top})"
    );

    // Reference values captured on master:
    // PrimaryProfession1 L=96 B=537 W=437 H=81
    // SpellButton1       L=384 B=535 W=40 H=40
    // SpellButton2       L=384 B=575 W=40 H=40
    assert_eq!(primary, (96.0, 537.0, 437.0, 81.0), "PrimaryProfession1 rect mismatch vs master");
    assert_eq!(b1, (384.0, 535.0, 40.0, 40.0), "SpellButton1 rect mismatch vs master");
    assert_eq!(b2, (384.0, 575.0, 40.0, 40.0), "SpellButton2 rect mismatch vs master");

    // Rank status bar must show formatted "<rank>/<max>", never the raw
    // "%d/%d" format string. Regression for SetFormattedText writing the
    // formatted result to the wrong stack slot when base != 0.
    let rank_text: String = env
        .eval("return tostring(PrimaryProfession1.statusBar.rankText:GetText())")
        .unwrap();
    assert!(
        !rank_text.contains("%d"),
        "statusBar.rankText should be formatted, got {rank_text:?}"
    );
    assert!(
        rank_text.contains('/'),
        "statusBar.rankText should be '<rank>/<max>', got {rank_text:?}"
    );
}
