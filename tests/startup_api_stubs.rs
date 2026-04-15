//! Smoke tests for startup-surface stubs added to unblock Blizzard addon
//! loading. Each stub returns values that reflect the simulator's reality
//! (no network, no in-game store, no premade finder, no photo sharing)
//! rather than invented placeholders.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_net_stats_returns_four_zeros() {
    let env = env();
    let (bw_in, bw_out, latency_home, latency_world): (f64, f64, f64, f64) = env
        .eval("return GetNetStats()")
        .expect("GetNetStats should be callable");
    assert_eq!(bw_in, 0.0);
    assert_eq!(bw_out, 0.0);
    assert_eq!(latency_home, 0.0);
    assert_eq!(latency_world, 0.0);
}

#[test]
fn store_frame_is_shown_returns_false() {
    let env = env();
    let shown: bool = env.eval("return StoreFrame_IsShown()").unwrap();
    assert!(!shown, "no Store UI is ever rendered in the sim");
}

#[test]
fn c_lfg_info_can_player_use_premade_group_returns_false() {
    let env = env();
    let can_use: bool = env
        .eval("return C_LFGInfo.CanPlayerUsePremadeGroup()")
        .unwrap();
    assert!(
        !can_use,
        "premade group finder is not simulated, so the callsite takes the \
         'cannot use' branch and skips the premade promo UI"
    );
}

#[test]
fn c_photo_sharing_reports_disabled() {
    let env = env();
    let (is_enabled, is_authorized): (bool, bool) = env
        .eval("return C_PhotoSharing.IsEnabled(), C_PhotoSharing.IsAuthorized()")
        .unwrap();
    assert!(!is_enabled);
    assert!(!is_authorized);
}
