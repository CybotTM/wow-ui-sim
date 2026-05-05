//! `C_LFGInfo.CanPlayerUsePremadeGroup` — SimState-backed round-trip.

use wow_ui_sim::lua_api::WowLuaEnv;

fn probe(env: &WowLuaEnv) -> bool {
    env.eval(r#"return C_LFGInfo.CanPlayerUsePremadeGroup()"#)
        .unwrap()
}

#[test]
fn defaults_to_true() {
    let env = WowLuaEnv::new().unwrap();
    assert!(probe(&env));
}

#[test]
fn admin_enables_and_disables() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetCanUsePremadeGroup(true)").unwrap();
    assert!(probe(&env));
    env.exec("A_Admin.SetCanUsePremadeGroup(false)").unwrap();
    assert!(!probe(&env));
}

#[test]
fn no_arg_defaults_to_true() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetCanUsePremadeGroup()").unwrap();
    assert!(probe(&env));
}

#[test]
fn sibling_c_lfg_info_members_still_resolve() {
    // Other C_LFGInfo members (IsLFGModeActiveForCategory via namespace
    // false-stubs, CanPlayerUseLFD etc.) should still be callable.
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            if type(C_LFGInfo.IsLFGModeActiveForCategory) ~= "function" then
                return "missing_is_lfg_mode_active"
            end
            if C_LFGInfo.IsLFGModeActiveForCategory(1) ~= false then
                return "expected_false_for_category_1"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn lfg_mode_active_reads_state_set() {
    let env = WowLuaEnv::new().unwrap();
    env.state().borrow_mut().lfg_active_categories.insert(3);
    let (active, inactive): (bool, bool) = env
        .eval(
            r#"
            return C_LFGInfo.IsLFGModeActiveForCategory(3),
                   C_LFGInfo.IsLFGModeActiveForCategory(2)
            "#,
        )
        .unwrap();

    assert!(active);
    assert!(!inactive);
}
