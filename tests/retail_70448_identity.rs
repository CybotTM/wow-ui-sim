//! Retail 12.1.5.70448 selected installed build and cumulative API epoch.
#![cfg(feature = "client-retail")]

use wow_ui_sim::client_profile::{ACTIVE, ACTIVE_INTERFACE_VERSION, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn retail_70448_selects_current_api_epoch_without_changing_channel() {
    assert_eq!(ACTIVE, ClientProfile::Retail);
    assert_eq!(ACTIVE.cache_subdir(), "retail");
    assert_eq!(ACTIVE_INTERFACE_VERSION, 120105);
    assert_eq!(ACTIVE.interface_version(), 120105);
}

#[test]
fn retail_70448_build_info_matches_selected_api_epoch() {
    let env = WowLuaEnv::new().unwrap();
    let identity: (String, i32) = env
        .eval("local version, _, _, interface = GetBuildInfo(); return version, interface")
        .unwrap();
    assert_eq!(identity, ("12.1.5".into(), 120105));
}

fn load_shared_base(env: &WowLuaEnv, file: &str) {
    let root = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    let path = root.join("Blizzard_SharedXMLBase").join(file);
    let code = std::fs::read_to_string(&path).unwrap();
    env.exec(&code)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
}

#[test]
fn retail_70448_original_math_aliases_use_native_epoch_producers() {
    let env = WowLuaEnv::new().unwrap();
    load_shared_base(&env, "MathUtil.lua");
    let output: (f64, f64, f64) = env
        .eval("return Round(2.5), Lerp(10, 20, .25), Clamp(12, 1, 10)")
        .unwrap();
    assert_eq!(output, (3.0, 12.5, 10.0));
}

#[test]
fn retail_70448_original_timer_util_dispatches_registered_callback() {
    let env = WowLuaEnv::new().unwrap();
    load_shared_base(&env, "MathUtil.lua");
    load_shared_base(&env, "TimerUtil.lua");
    env.exec(
        "callbacks = 0; local map = TimerUtil.CreateTimedSignalCallbackMap(); \
         local key = map:RegisterCallback(function() callbacks = callbacks + 1 end); \
         map:SignalAfter(key, 0); retained_map = map",
    )
    .unwrap();
    assert_eq!(env.process_timers().unwrap(), 1);
    assert_eq!(env.eval::<i32>("return callbacks").unwrap(), 1);
}
