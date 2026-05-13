use crate::common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

type AddonToc = (&'static str, &'static str);

const ACTION_BAR_ADDONS: &[AddonToc] = &[
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

fn seed_action_slot(env: &WowLuaEnv, slot: u32, spell_id: u32) {
    env.state().borrow_mut().action_bars.insert(slot, spell_id);
}

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn action_bar_toc(addon: &str, toc_name: &str) -> PathBuf {
    blizzard_ui_dir().join(addon).join(toc_name)
}

fn action_bar_addons() -> &'static [AddonToc] {
    ACTION_BAR_ADDONS
}

fn load_action_bar_addons(env: &WowLuaEnv) {
    env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];
    for (name, toc) in action_bar_addons() {
        let toc_path = action_bar_toc(name, toc);
        if toc_path.exists() {
            load_addon(&env.loader_env(), &toc_path).unwrap();
        }
    }
}

fn fire_action_bar_startup(env: &WowLuaEnv) {
    env.fire_event_with_args("ADDON_LOADED", &[env.lua_string("WoWUISim")])
        .unwrap();
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        env.fire_event(event).unwrap();
    }
    env.fire_event_with_args(
        "PLAYER_ENTERING_WORLD",
        &[rilua::Val::Bool(true), rilua::Val::Bool(false)],
    )
    .unwrap();
    env.fire_edit_mode_layouts_updated().unwrap();
    env.fire_event("ACTIONBAR_SHOWGRID").unwrap();
}

fn env_with_action_bar() -> common::LockedEnv {
    common::lock_env(|| {
        let env = WowLuaEnv::new().unwrap();
        env.set_screen_size(1024.0, 768.0);
        load_action_bar_addons(&env);
        env.apply_post_load_workarounds();
        fire_action_bar_startup(&env);
        env
    })
}

fn assert_action_button_template_has_receive_drag() {
    let chain = wow_ui_sim::xml::get_template_chain("ActionBarButtonTemplate");
    let code_template = chain
        .iter()
        .find(|entry| entry.name == "ActionBarButtonCodeTemplate")
        .unwrap();
    assert!(
        code_template
            .frame
            .scripts()
            .is_some_and(|scripts| !scripts.on_receive_drag.is_empty()),
        "template chain should include OnReceiveDrag on ActionBarButtonCodeTemplate"
    );
}

#[test]
fn pickup_action_accepts_ignore_removal_arg_and_place_restores_slot() {
    let env = WowLuaEnv::new().unwrap();
    seed_action_slot(&env, 1, 853);

    let ok: bool = env
        .eval(
            r#"
            local ok, err = pcall(function()
                PickupAction(1, false)
            end)
            TEST_PICKUP_ACTION_ERR = err
            return ok
            "#,
        )
        .unwrap();
    assert!(
        ok,
        "PickupAction(slot, ignoreRemoval) errored: {}",
        env.eval::<String>("return tostring(TEST_PICKUP_ACTION_ERR)")
            .unwrap()
    );
    assert!(!env.eval::<bool>("return HasAction(1)").unwrap());

    let (cursor_type, cursor_spell_id): (String, i32) = env.eval("return GetCursorInfo()").unwrap();
    assert_eq!(cursor_type, "spell");
    assert_eq!(cursor_spell_id, 853);

    env.exec("PlaceAction(1)").unwrap();

    assert!(env.eval::<bool>("return HasAction(1)").unwrap());
    assert!(env.eval::<bool>("return GetCursorInfo() == nil").unwrap());
    assert!(
        env.eval::<bool>("return type(C_ActionBar.GetActionTexture(1)) == 'string'")
            .unwrap()
    );
}

#[test]
fn pickup_action_updates_action_button_icon_immediately() {
    common::with_timeout(120, move || {
        let env = env_with_action_bar();
        seed_action_slot(&env, 1, 853);
        env.fire_event_with_args("ACTIONBAR_SLOT_CHANGED", &[rilua::Val::Num(1.0)])
            .unwrap();

        let before_drag: bool = env
            .eval("return ActionButton1.icon:IsShown() and HasAction(1)")
            .unwrap();
        assert!(
            before_drag,
            "action button should show its icon before PickupAction"
        );

        env.exec("PickupAction(1, false)").unwrap();

        let after_pickup: bool = env
            .eval("return (not ActionButton1.icon:IsShown()) and not HasAction(1)")
            .unwrap();
        assert!(
            after_pickup,
            "PickupAction should fire ACTIONBAR_SLOT_CHANGED so the source icon hides immediately"
        );

        env.exec("PlaceAction(1)").unwrap();

        let after_place: bool = env
            .eval("return ActionButton1.icon:IsShown() and HasAction(1)")
            .unwrap();
        assert!(
            after_place,
            "PlaceAction should fire ACTIONBAR_SLOT_CHANGED so the icon restores immediately"
        );
    });
}

#[test]
fn action_button_drag_round_trip_keeps_spell_visible() {
    common::with_timeout(120, move || {
        let env = env_with_action_bar();
        assert_action_button_template_has_receive_drag();
        let button_id = env
            .state()
            .borrow()
            .widgets
            .get_id_by_name("ActionButton1")
            .unwrap();
        seed_action_slot(&env, 1, 853);
        env.fire_event_with_args("ACTIONBAR_SLOT_CHANGED", &[rilua::Val::Num(0.0)])
            .unwrap();
        env.fire_event("ACTIONBAR_UPDATE_STATE").unwrap();
        env.exec("if ActionButton1 then ActionButton1.icon:SetTexture(GetActionTexture(1)) end")
            .unwrap();

        let before_drag: bool = env
            .eval("return type(ActionButton1.icon:GetTexture()) == 'string'")
            .unwrap();
        assert!(
            before_drag,
            "action button should show its icon before drag"
        );
        let has_receive_drag: bool = env
            .eval("return ActionButton1:GetScript('OnReceiveDrag') ~= nil")
            .unwrap();
        assert!(
            has_receive_drag,
            "action button should have an OnReceiveDrag handler"
        );

        env.fire_script_handler(button_id, "OnDragStart", vec![])
            .unwrap();
        env.fire_script_handler(button_id, "OnReceiveDrag", vec![])
            .unwrap();

        let after_drag: bool = env
            .eval("return type(ActionButton1.icon:GetTexture()) == 'string' and HasAction(1)")
            .unwrap();
        assert!(
            after_drag,
            "dragging off and back onto the same button should keep the icon"
        );
    });
}

#[test]
fn action_button_1_icon_matches_get_action_texture() {
    common::with_timeout(120, move || {
        let env = env_with_action_bar();
        seed_action_slot(&env, 1, 853);
        env.fire_event_with_args("ACTIONBAR_SLOT_CHANGED", &[rilua::Val::Num(0.0)])
            .unwrap();
        env.fire_event("ACTIONBAR_UPDATE_STATE").unwrap();
        env.exec("if ActionButton1 then ActionButton1.icon:SetTexture(GetActionTexture(1)) end")
            .unwrap();

        let result: String = env
            .eval(
                r#"
                if not ActionButton1 then
                    return "missing_action_button_1"
                end
                if not ActionButton1.icon then
                    return "missing_action_button_1_icon"
                end

                local expected = GetActionTexture(1)
                local actual = ActionButton1.icon:GetTexture()
                if actual ~= expected then
                    return string.format(
                        "icon_mismatch_expected_%s_actual_%s",
                        tostring(expected),
                        tostring(actual)
                    )
                end

                return "ok"
            "#,
            )
            .unwrap();

        assert_eq!(
            result, "ok",
            "ActionButton1 icon should match GetActionTexture(1): {result}"
        );
    });
}
