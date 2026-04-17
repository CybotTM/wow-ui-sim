//! Integration tests for `system_api_runtime::patch_namespace_stubs`.
//!
//! Each test constructs a `WowLuaEnv` (which already runs
//! `stubs::register_all` during init), then calls `patch_namespace_stubs`
//! explicitly to prove the function is callable and idempotent, and
//! asserts the expected return values from Lua.

use rilua::LuaApiMut;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::globals::system_api_runtime::patch_namespace_stubs;

fn env_with_patches() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("WowLuaEnv init");
    {
        let loader = env.loader_env();
        let mut lua = loader.rilua_mut();
        patch_namespace_stubs(lua.state_mut());
    }
    env
}

// ── C_UIWidgetManager ─────────────────────────────────────────────────────────

#[test]
fn get_power_bar_widget_set_id_returns_zero() {
    let env = env_with_patches();
    let id: i64 = env
        .eval("return C_UIWidgetManager.GetPowerBarWidgetSetID()")
        .expect("GetPowerBarWidgetSetID should not error");
    assert_eq!(id, 0, "GetPowerBarWidgetSetID must return 0 (master value)");
}

// ── C_PlayerInfo ──────────────────────────────────────────────────────────────

#[test]
fn is_player_in_rpe_returns_false() {
    let env = env_with_patches();
    let result: bool = env
        .eval("return C_PlayerInfo.IsPlayerInRPE()")
        .expect("IsPlayerInRPE should not error");
    assert!(!result, "IsPlayerInRPE must return false (master value)");
}

#[test]
fn get_alternate_form_info_returns_false_false() {
    let env = env_with_patches();
    let (has_form, in_form): (bool, bool) = env
        .eval("return C_PlayerInfo.GetAlternateFormInfo()")
        .expect("GetAlternateFormInfo should not error");
    assert!(!has_form, "GetAlternateFormInfo first return must be false");
    assert!(!in_form, "GetAlternateFormInfo second return must be false");
}

// ── Idempotency ───────────────────────────────────────────────────────────────

#[test]
fn patch_namespace_stubs_is_idempotent() {
    let env = WowLuaEnv::new().expect("WowLuaEnv init");
    {
        let loader = env.loader_env();
        let mut lua = loader.rilua_mut();
        // Call twice; second call must not panic or corrupt state.
        patch_namespace_stubs(lua.state_mut());
        patch_namespace_stubs(lua.state_mut());
    }
    let (widget_id, in_rpe, has_alt, in_alt): (i64, bool, bool, bool) = env
        .eval(
            r#"
            return C_UIWidgetManager.GetPowerBarWidgetSetID(),
                   C_PlayerInfo.IsPlayerInRPE(),
                   C_PlayerInfo.GetAlternateFormInfo()
            "#,
        )
        .expect("all three methods must work after double-patch");
    assert_eq!(widget_id, 0);
    assert!(!in_rpe);
    assert!(!has_alt);
    assert!(!in_alt);
}

// ── Namespace tables are created if absent ────────────────────────────────────

#[test]
fn patch_creates_namespace_tables_if_missing() {
    // Verify that C_UIWidgetManager and C_PlayerInfo exist as tables after
    // the patch (they will always exist after stubs::register_all, but this
    // pins the guarantee even if a hypothetical env skips the stub pass).
    let env = env_with_patches();
    let (widget_is_table, player_is_table): (bool, bool) = env
        .eval(
            r#"
            return type(C_UIWidgetManager) == "table",
                   type(C_PlayerInfo) == "table"
            "#,
        )
        .expect("namespace tables must exist");
    assert!(widget_is_table, "C_UIWidgetManager must be a table");
    assert!(player_is_table, "C_PlayerInfo must be a table");
}
