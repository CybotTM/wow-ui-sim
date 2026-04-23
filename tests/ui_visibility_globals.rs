//! Integration tests for `SetUIVisibility` / `SetInWorldUIVisibility`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

#[test]
fn ui_visibility_globals_are_registered() {
    let env = env();
    let (set_ui_ty, set_in_world_ty): (String, String) = env
        .eval("return type(SetUIVisibility), type(SetInWorldUIVisibility)")
        .unwrap();
    assert_eq!(set_ui_ty, "function");
    assert_eq!(set_in_world_ty, "function");
}

#[test]
fn set_ui_visibility_toggles_ui_parent_visibility() {
    let env = env();

    env.exec("SetUIVisibility(false)").unwrap();
    let hidden: bool = env.eval("return not UIParent:IsShown()").unwrap();
    assert!(
        hidden,
        "UIParent should be hidden after SetUIVisibility(false)"
    );

    env.exec("SetUIVisibility(true)").unwrap();
    let shown: bool = env.eval("return UIParent:IsShown()").unwrap();
    assert!(
        shown,
        "UIParent should be shown after SetUIVisibility(true)"
    );
}

#[test]
fn open_world_map_maximized_does_not_error_on_set_ui_visibility() {
    let env = env();
    env.exec("SetCVar('miniWorldMap', 0)").unwrap();
    env.exec("OpenWorldMap()")
        .expect("OpenWorldMap maximize path should not error on SetUIVisibility");
}
