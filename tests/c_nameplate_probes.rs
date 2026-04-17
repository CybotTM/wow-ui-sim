//! Tests for `C_NamePlate` permissive stubs.
//!
//! The simulator has no 3D nameplate rendering. `GetNamePlateForUnit`
//! returns nil (no plate visible) and `GetNamePlates` returns an empty
//! array (no plates active). Both behaviours are what Blizzard UI
//! expects when no units are in nameplate range.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_nameplate_for_unit_target_is_nil() {
    let env = env();
    let result: Option<bool> = env
        .eval(r#"return C_NamePlate.GetNamePlateForUnit("target")"#)
        .unwrap();
    assert!(
        result.is_none(),
        "GetNamePlateForUnit(\"target\") must be nil"
    );
}

#[test]
fn get_nameplate_for_unit_player_is_nil() {
    let env = env();
    let result: Option<bool> = env
        .eval(r#"return C_NamePlate.GetNamePlateForUnit("player")"#)
        .unwrap();
    assert!(
        result.is_none(),
        "GetNamePlateForUnit(\"player\") must be nil"
    );
}

#[test]
fn get_nameplates_returns_empty_array() {
    let env = env();
    let count: i32 = env.eval(r#"return #C_NamePlate.GetNamePlates()"#).unwrap();
    assert_eq!(count, 0, "GetNamePlates() must return an empty array");
}

#[test]
fn get_nameplates_returns_table_not_nil() {
    let env = env();
    let type_name: String = env
        .eval(r#"return type(C_NamePlate.GetNamePlates())"#)
        .unwrap();
    assert_eq!(
        type_name, "table",
        "GetNamePlates() must return a table, not nil"
    );
}
