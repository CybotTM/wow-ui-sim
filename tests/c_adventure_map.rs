//! Integration tests for the `C_AdventureMap` namespace registered in
//! `src/lua_api/globals/adventure_map.rs`.
//!
//! `GetMapID()` is the single entry point currently implemented; the
//! Blizzard_AdventureMap addon reads it during `AdventureMapMixin:OnShow`
//! and forwards the value to `MapCanvasMixin:SetMapID` (see
//! `Blizzard_AdventureMap.lua:45`).

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn c_adventure_map_namespace_is_a_table() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(C_AdventureMap)").unwrap();
    assert_eq!(kind, "table");
}

#[test]
fn get_map_id_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetMapID)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_map_id_defaults_to_zero() {
    let env = WowLuaEnv::new().expect("env");
    let map_id: f64 = env.eval("return C_AdventureMap.GetMapID()").unwrap();
    assert!(map_id.abs() < 1e-6);
}

#[test]
fn get_map_id_returns_seeded_value() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.map_id = 619;

    let map_id: f64 = env.eval("return C_AdventureMap.GetMapID()").unwrap();
    assert!((map_id - 619.0).abs() < 1e-6);
}

#[test]
fn get_map_id_returns_a_number_type() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.map_id = 42;

    let kind: String = env
        .eval("return type(C_AdventureMap.GetMapID())")
        .unwrap();
    assert_eq!(kind, "number");
}
