#![cfg(feature = "client-ptr")]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::c_api::c_input_interface_style::InputInterfaceStyle;
use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::screen::ScreenKind;

#[test]
fn ptr_input_util_observes_supplied_styles_and_initializes_matching_subscribers() {
    let ui = blizzard_ui_dir();
    let discovered = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    assert!(
        discovered
            .iter()
            .any(|(name, _)| name == "Blizzard_SharedXML")
    );
    let env = new_blizzard_addon_env(&ui);
    // Supply a model input, not an assertion about the native client's default.
    env.state()
        .borrow_mut()
        .set_input_interface_style(InputInterfaceStyle::Mkb);
    let loaded = load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_SharedXML"], &[]);
    assert!(loaded.iter().any(|name| name == "Blizzard_SharedXML"));
    let listener_id = {
        let state = env.state().borrow();
        assert!(state.lua_errors.is_empty(), "{:?}", state.lua_errors);
        let listeners: Vec<_> = state
            .widgets
            .iter_ids()
            .filter(|&id| {
                state
                    .widgets
                    .get(id)
                    .unwrap()
                    .registered_events
                    .contains("INPUT_DEVICE_INTERFACE_TRANSITION")
            })
            .collect();
        assert_eq!(
            listeners.len(),
            1,
            "original InputUtil listener must register"
        );
        listeners[0]
    };
    // The original listener is forbidden, so EnumerateFrames deliberately hides it.
    env.exec(&format!(
        "inputTransitionListener = debug.getregistry().__frame_refs[{listener_id}]"
    ))
    .unwrap();
    env.exec(
        r#"
        assert(inputTransitionListener:IsEventRegistered("INPUT_DEVICE_INTERFACE_TRANSITION"))
        assert(not inputTransitionListener:IsEventRegistered("PLAYER_LOGIN"))
        local unknown = "WOW_SIM_UNKNOWN_INPUT_INTERFACE_EVENT"
        assert(not pcall(inputTransitionListener.RegisterEvent, inputTransitionListener, unknown))
        assert(not inputTransitionListener:IsEventRegistered(unknown))
        local handler = inputTransitionListener:GetScript("OnEvent")
        assert(type(handler) == "function")
        local source = debug.getinfo(handler, "S").source:gsub("\\", "/")
        assert(source:find("Blizzard_SharedXML/InputUtil.lua", 1, true))
        "#,
    )
    .expect("original InputUtil top-level listener must retain its registration and handler");

    for (style, expected) in [
        (InputInterfaceStyle::Mkb, 0),
        (InputInterfaceStyle::Gamepad, 1),
        (InputInterfaceStyle::Mkb, 0),
    ] {
        env.state().borrow_mut().set_input_interface_style(style);
        env.exec(&format!("expectedStyle = {expected}")).unwrap();
        env.exec(
            r#"
            assert(InputUtil.GetCurrentInterfaceStyle() == expectedStyle)
            assert(Enum.InputDeviceInterfaceType.Mkb == 0)
            assert(Enum.InputDeviceInterfaceType.Gamepad == 1)
            local isMkb = expectedStyle == 0
            assert(InputUtil.IsMKBUIEnabled() == isMkb)
            assert(InputUtil.IsGamepadUIEnabled() == not isMkb)
            local subscriber = {}
            local mkbSetup, mkbInit, gamepadSetup, gamepadInit = 0, 0, 0, 0
            InputUtil.RegisterForInterfaceTransitions(subscriber)
            InputUtil.RegisterMKBSetup(subscriber, function() mkbSetup = mkbSetup + 1 end)
            InputUtil.RegisterMKBInit(subscriber, function() mkbInit = mkbInit + 1 end)
            InputUtil.RegisterGamepadSetup(subscriber, function() gamepadSetup = gamepadSetup + 1 end)
            InputUtil.RegisterGamepadInit(subscriber, function() gamepadInit = gamepadInit + 1 end)
            assert(mkbSetup == (isMkb and 1 or 0))
            assert(mkbInit == (isMkb and 1 or 0))
            assert(gamepadSetup == (isMkb and 0 or 1))
            assert(gamepadInit == (isMkb and 0 or 1))
            "#,
        )
        .unwrap();
    }
}
