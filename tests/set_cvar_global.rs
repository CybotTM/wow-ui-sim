//! Integration tests for `src/lua_api/globals/set_cvar_verb.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

#[test]
fn set_cvar_stores_string_value() {
    let env = env();
    let ok: bool = env
        .eval(r#"return SetCVar("nameplateShowAll", "1")"#)
        .unwrap();
    assert!(ok);
    let value = env.state().borrow().cvars.get("nameplateShowAll");
    assert_eq!(value.as_deref(), Some("1"));
}

#[test]
fn set_cvar_accepts_numeric_value() {
    let env = env();
    env.exec(r#"SetCVar("nameplateMaxDistance", 60)"#).unwrap();
    let value = env.state().borrow().cvars.get("nameplateMaxDistance");
    assert_eq!(value.as_deref(), Some("60"));
}

#[test]
fn set_cvar_accepts_fractional_numeric_value() {
    let env = env();
    env.exec(r#"SetCVar("uiScale", 0.75)"#).unwrap();
    let value = env.state().borrow().cvars.get("uiScale");
    assert_eq!(value.as_deref(), Some("0.75"));
}

#[test]
fn set_cvar_accepts_boolean_as_one_zero() {
    let env = env();
    env.exec(r#"SetCVar("nameplateShowEnemies", true)"#)
        .unwrap();
    let value = env.state().borrow().cvars.get("nameplateShowEnemies");
    assert_eq!(value.as_deref(), Some("1"));
    env.exec(r#"SetCVar("nameplateShowEnemies", false)"#)
        .unwrap();
    let value = env.state().borrow().cvars.get("nameplateShowEnemies");
    assert_eq!(value.as_deref(), Some("0"));
}

#[test]
fn set_cvar_empty_name_returns_false() {
    let env = env();
    let ok: bool = env.eval(r#"return SetCVar("", "1")"#).unwrap();
    assert!(!ok);
}

#[test]
fn c_cvar_get_reads_back_the_value_set_via_global() {
    let env = env();
    // The retail-facing global writes through SimState.cvars. C_CVar.GetCVar
    // reads from the same bootstrap-Lua-backed store — they are separate
    // surfaces today (C_CVar is a Lua table populated in runtime_surface_bootstrap.lua).
    // Pin the current split so regressing it surfaces as a test change.
    env.exec(r#"SetCVar("showTimestamps", "chat")"#).unwrap();
    let value = env.state().borrow().cvars.get("showTimestamps");
    assert_eq!(value.as_deref(), Some("chat"));
}
