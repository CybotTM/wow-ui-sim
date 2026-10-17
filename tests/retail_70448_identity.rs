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
fn retail_70448_build_info_matches_selected_installed_identity() {
    let env = WowLuaEnv::new().unwrap();
    let identity: (String, String, i32) = env
        .eval(
            "local version, build, _, interface = GetBuildInfo(); return version, build, interface",
        )
        .unwrap();
    assert_eq!(identity, ("12.1.5".into(), "70448".into(), 120105));
}
