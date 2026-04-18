//! `StoreFrame_IsShown` + `A_Admin.SetStoreFrameShown` round-trip coverage.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn store_frame_is_shown_defaults_to_false() {
    let env = WowLuaEnv::new().expect("env");
    let shown: bool = env
        .eval("return StoreFrame_IsShown()")
        .expect("StoreFrame_IsShown should return a bool");
    assert!(!shown, "sim's Store window should default to hidden");
}

#[test]
fn admin_set_store_frame_shown_flips_the_flag() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("A_Admin.SetStoreFrameShown(true)").unwrap();
    let shown: bool = env.eval("return StoreFrame_IsShown()").unwrap();
    assert!(
        shown,
        "StoreFrame_IsShown should report true after admin flip"
    );

    env.exec("A_Admin.SetStoreFrameShown(false)").unwrap();
    let shown: bool = env.eval("return StoreFrame_IsShown()").unwrap();
    assert!(
        !shown,
        "StoreFrame_IsShown should report false after admin flip back"
    );
}

#[test]
fn admin_set_store_frame_shown_defaults_to_true_when_called_with_no_arg() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("A_Admin.SetStoreFrameShown()").unwrap();
    let shown: bool = env.eval("return StoreFrame_IsShown()").unwrap();
    assert!(
        shown,
        "A_Admin.SetStoreFrameShown() with no arg should open the store",
    );
}

#[test]
fn store_frame_set_shown_updates_the_shared_flag() {
    let env = WowLuaEnv::new().expect("env");

    let opened: bool = env
        .eval("StoreFrame_SetShown(true); return StoreFrame_IsShown()")
        .unwrap();
    assert!(
        opened,
        "StoreFrame_SetShown(true) should mark the store as visible"
    );

    let closed: bool = env
        .eval("StoreFrame_SetShown(false, 'StoreMicroButton'); return StoreFrame_IsShown()")
        .unwrap();
    assert!(
        !closed,
        "StoreFrame_SetShown(false, contextKey) should mark the store as hidden"
    );
}
