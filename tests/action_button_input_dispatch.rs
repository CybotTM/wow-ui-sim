use crate::common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::fire_startup_events;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn blizzard_toc(addon: &str, toc_name: &str) -> PathBuf {
    blizzard_ui_dir().join(addon).join(toc_name)
}

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
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }
    for (name, toc) in ACTION_BAR_ADDONS {
        let toc_path = blizzard_toc(name, toc);
        if toc_path.exists() {
            load_addon(&env.loader_env(), &toc_path)
                .unwrap_or_else(|e| panic!("[load {name}] FAILED: {e}"));
        }
    }
    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

fn seed_action_slot(env: &WowLuaEnv, slot: u32, spell_id: u32) {
    env.state().borrow_mut().action_bars.insert(slot, spell_id);
}

#[test]
fn action_button_down_and_up_toggle_state_and_fire_action() {
    let env = build_action_bar_env();
    seed_action_slot(&env, 1, 19750);
    env.fire_event("ACTIONBAR_SLOT_CHANGED").unwrap();

    let before: (String, bool) = env
        .eval(
            r#"
            return ActionButton1:GetButtonState(), HasAction(1)
        "#,
        )
        .unwrap();
    assert_eq!(before.0, "NORMAL");
    assert!(before.1);

    env.exec("ActionButtonDown(1)").unwrap();

    let pressed: (String, bool) = env
        .eval(
            r#"
            return ActionButton1:GetButtonState(), IsCurrentAction(1)
        "#,
        )
        .unwrap();
    assert_eq!(pressed.0, "PUSHED");
    assert!(pressed.1);

    env.exec("ActionButtonUp(1)").unwrap();
    let released: String = env.eval("return ActionButton1:GetButtonState()").unwrap();
    assert_eq!(released, "NORMAL");
}

#[test]
fn try_use_action_button_only_fires_from_key_down() {
    let env = build_action_bar_env();
    seed_action_slot(&env, 1, 19750);
    env.fire_event("ACTIONBAR_SLOT_CHANGED").unwrap();

    let only_down_casts: bool = env
        .eval(
            r#"
            local before = select(9, UnitCastingInfo("player"))
            TryUseActionButton(ActionButton1, false)
            local afterUp = select(9, UnitCastingInfo("player"))
            TryUseActionButton(ActionButton1, true)
            local afterDown = select(9, UnitCastingInfo("player"))
            return before == nil and afterUp == nil and afterDown == 19750
            "#,
        )
        .unwrap();
    assert!(only_down_casts);
}

#[test]
fn multi_and_extra_action_dispatch_use_the_same_button_fallthrough() {
    let env = build_action_bar_env();
    seed_action_slot(&env, 61, 19750);
    env.fire_event("ACTIONBAR_SLOT_CHANGED").unwrap();

    let multi_cast_spell_id: i32 = env
        .eval(
            r#"
            MultiActionButtonDown("MultiBarBottomLeft", 1)
            return select(9, UnitCastingInfo("player"))
        "#,
        )
        .unwrap();
    assert_eq!(multi_cast_spell_id, 19750);

    env.exec("SpellStopCasting()").unwrap();
    let extra_did_not_error: bool = env
        .eval(
            r#"
            local ok = pcall(ExtraActionButtonKey, 1, true)
            return ok
        "#,
        )
        .unwrap();
    assert!(extra_did_not_error);
}
