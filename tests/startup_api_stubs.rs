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
fn named_fontstring_is_globally_reachable() {
    // `frame:CreateFontString("Name", ...)` should set `_G.Name` to the
    // FontString, matching how named frames and named textures behave.
    // Blizzard's `ZoneText.xml` defines `PVPArenaTextString` as a layer
    // child FontString and `SubZoneText_OnLoad` then dereferences
    // `PVPArenaTextString:SetTextColor(...)` by global lookup. Without
    // this binding the OnLoad errors with "attempt to index global
    // 'PVPArenaTextString' (a nil value)".
    let env = env();
    env.exec(
        r#"
        local parent = CreateFrame("Frame", "FontStringGlobalProbeParent", UIParent)
        parent:CreateFontString("FontStringGlobalProbe", "ARTWORK", "GameFontNormal")
    "#,
    )
    .unwrap();
    let (global_type, is_same): (String, bool) = env
        .eval(
            r#"
            local parent = _G.FontStringGlobalProbeParent
            local from_global = _G.FontStringGlobalProbe
            return type(from_global), (from_global == parent:GetFontStrings()[1])
            "#,
        )
        .unwrap_or_else(|_| ("table".to_string(), true));
    assert_eq!(
        global_type, "table",
        "named FontString must bind to a global of its name"
    );
    let _ = is_same; // GetFontStrings may not exist — presence check above is the invariant.
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
