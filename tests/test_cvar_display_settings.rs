//! Coverage for the global display CVars (Brightness/Contrast/Gamma)
//! the Settings sliders bind to. These three live in `cvars.yaml` with
//! the same defaults retail ships (50/50/1.0); the SimState-backed
//! GetCVar / SetCVar Rust impls make them visible to addon code that
//! goes through `Blizzard_SharedXMLBase/CvarUtil.lua`'s wrappers.
//!
//! No "luminosity" knob — these are real WoW CVars.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn brightness_default_visible_via_get_cvar() {
    let env = env();
    let value: String = env.eval(r#"return GetCVar("Brightness")"#).unwrap();
    assert_eq!(value, "50.000000");
}

#[test]
fn contrast_default_visible_via_get_cvar() {
    let env = env();
    let value: String = env.eval(r#"return GetCVar("Contrast")"#).unwrap();
    assert_eq!(value, "50.000000");
}

#[test]
fn gamma_default_visible_via_get_cvar() {
    let env = env();
    let value: String = env.eval(r#"return GetCVar("Gamma")"#).unwrap();
    assert_eq!(value, "1.000000");
}

#[test]
fn brightness_set_then_get_round_trips_through_simstate() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SetCVar("Brightness", 75)
            return GetCVar("Brightness")
            "#,
        )
        .unwrap();
    assert_eq!(result, "75");
    env.exec(r#"SetCVar("Brightness", GetCVarDefault("Brightness"))"#)
        .unwrap();
}

#[test]
fn contrast_set_then_get_round_trips_through_simstate() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SetCVar("Contrast", 30)
            return GetCVar("Contrast")
            "#,
        )
        .unwrap();
    assert_eq!(result, "30");
    env.exec(r#"SetCVar("Contrast", GetCVarDefault("Contrast"))"#)
        .unwrap();
}

#[test]
fn cvar_lookup_is_case_insensitive_on_read() {
    let env = env();
    let lower: String = env.eval(r#"return GetCVar("brightness")"#).unwrap();
    let upper: String = env.eval(r#"return GetCVar("BRIGHTNESS")"#).unwrap();
    let mixed: String = env.eval(r#"return GetCVar("Brightness")"#).unwrap();
    assert_eq!(lower, "50.000000");
    assert_eq!(upper, "50.000000");
    assert_eq!(mixed, "50.000000");
}

#[test]
fn get_cvar_default_returns_yaml_value_after_override() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SetCVar("Brightness", 99)
            return GetCVarDefault("Brightness")
            "#,
        )
        .unwrap();
    assert_eq!(result, "50.000000");
    env.exec(r#"SetCVar("Brightness", GetCVarDefault("Brightness"))"#)
        .unwrap();
}

#[test]
fn c_cvar_namespace_mirrors_global_get_set() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            C_CVar.SetCVar("Brightness", 42)
            return C_CVar.GetCVar("Brightness")
            "#,
        )
        .unwrap();
    assert_eq!(result, "42");
    env.exec(r#"SetCVar("Brightness", GetCVarDefault("Brightness"))"#)
        .unwrap();
}

#[test]
fn unknown_cvar_returns_nil_not_empty_string() {
    let env = env();
    let result: Option<String> = env
        .eval(r#"return GetCVar("DefinitelyNotAReal_C_Var_xyz")"#)
        .unwrap();
    assert!(result.is_none(), "expected nil, got {result:?}");
}
