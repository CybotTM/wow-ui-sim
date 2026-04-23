//! `C_Housing.IsHousingServiceEnabled` — SimState-backed round-trip.

use wow_ui_sim::lua_api::WowLuaEnv;

fn probe(env: &WowLuaEnv) -> bool {
    env.eval(r#"return C_Housing.IsHousingServiceEnabled()"#)
        .unwrap()
}

#[test]
fn defaults_to_false() {
    let env = WowLuaEnv::new().unwrap();
    assert!(!probe(&env));
}

#[test]
fn admin_set_enables_and_disables() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetHousingServiceEnabled(true)").unwrap();
    assert!(probe(&env));
    env.exec("A_Admin.SetHousingServiceEnabled(false)").unwrap();
    assert!(!probe(&env));
}

#[test]
fn admin_no_arg_defaults_to_true() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetHousingServiceEnabled()").unwrap();
    assert!(probe(&env));
}

#[test]
fn other_c_housing_members_still_resolve_via_metamethod_fallback() {
    // Unimplemented C_Housing.* should return the stub-namespace no-op
    // function (which returns nil), not crash with "attempt to call a nil
    // value".
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local fn = C_Housing.SomeUnimplementedMember
            if type(fn) ~= "function" then return "missing_function" end
            if fn() ~= nil then return "non_nil_return" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn dashboard_bootstrap_members_have_safe_defaults() {
    let env = WowLuaEnv::new().unwrap();
    let (max_level, cooldown_is_nil): (i32, bool) = env
        .eval(
            r#"
            return C_Housing.GetMaxHouseLevel(), C_Housing.GetVisitCooldownInfo() == nil
            "#,
        )
        .unwrap();
    assert_eq!(max_level, 0);
    assert!(
        cooldown_is_nil,
        "GetVisitCooldownInfo should default to nil when no cooldown is active"
    );
}
