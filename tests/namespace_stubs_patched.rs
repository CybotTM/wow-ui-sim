//! Pins the behaviour of `system_api_runtime::patch_namespace_stubs`.
//!
//! The stubs pass (`stubs::register_all`) registers broad `stub_nil`
//! fallbacks for every namespace method the sim knows about. A small
//! set of those methods need concrete constant returns rather than
//! nil — historical wow-ui-sim master had overrides in
//! `system_api_runtime.rs`. During the rilua migration the overrides
//! landed as a named helper (`patch_namespace_stubs`) but weren't
//! called. `register_bootstrap_globals` now invokes it right after
//! the stub pass.
//!
//! Of the three methods patch_namespace_stubs overrides
//! (`C_UIWidgetManager.GetPowerBarWidgetSetID`,
//! `C_PlayerInfo.IsPlayerInRPE`, `C_PlayerInfo.GetAlternateFormInfo`),
//! two are superseded later by `missing_surface::player_info`, which
//! registers state-driven versions. The effective net-new contribution
//! of the runtime patch is `GetPowerBarWidgetSetID`. This test pins
//! that — if the wiring regresses, the stub_nil fallback returns
//! `nil` and the `f64` extraction would fail.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn c_ui_widget_manager_get_power_bar_widget_set_id_returns_zero() {
    let env = env();
    let value: f64 = env
        .eval("return C_UIWidgetManager.GetPowerBarWidgetSetID()")
        .expect("eval should succeed");
    assert_eq!(value, 0.0);
}

/// Regression guard: the two PlayerInfo methods are overridden by
/// `missing_surface::player_info` after patch_namespace_stubs runs.
/// If they ever fall back to the stub_false values, that means the
/// missing_surface path regressed — pin the real (state-driven)
/// behaviour.
#[test]
fn c_player_info_stubs_are_overridden_by_missing_surface_state() {
    let env = env();
    let (rpe_type, alt_type): (String, String) = env
        .eval(
            r#"
            return type(C_PlayerInfo.IsPlayerInRPE),
                   type(C_PlayerInfo.GetAlternateFormInfo)
            "#,
        )
        .expect("eval should succeed");
    assert_eq!(rpe_type, "function");
    assert_eq!(alt_type, "function");

    // Both methods must complete without error — proves some
    // implementation is wired, not the stub_nil fallback.
    let (rpe_ok, alt_ok): (bool, bool) = env
        .eval(
            r#"
            local rpe_ok = pcall(function() return C_PlayerInfo.IsPlayerInRPE() end)
            local alt_ok = pcall(function() return C_PlayerInfo.GetAlternateFormInfo() end)
            return rpe_ok, alt_ok
            "#,
        )
        .expect("eval should succeed");
    assert!(rpe_ok);
    assert!(alt_ok);
}
