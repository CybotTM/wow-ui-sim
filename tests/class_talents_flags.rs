//! Tests for `C_ClassTalents` flag probes backed by `TalentState`:
//!
//! - `C_ClassTalents.CanChangeTalents()` — `(canChange, canAdd,
//!   changeError_nilable)`
//! - `C_ClassTalents.GetHasStarterBuild()`
//! - `C_ClassTalents.IsStarterBuildActive()`
//!
//! Replaces the former `stub_false` entries in
//! `NAMESPACE_FALSE_STUBS`. Defaults: `canChange=true`,
//! `hasStarterBuild=false`, `isStarterBuildActive=false`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn can_change_talents_defaults_to_true_with_nil_error() {
    let env = env();
    let (can_change, can_add, error_is_nil): (bool, bool, bool) = env
        .eval(
            r#"
            local canChange, canAdd, err = C_ClassTalents.CanChangeTalents()
            return canChange, canAdd, err == nil
            "#,
        )
        .unwrap();
    assert!(can_change);
    assert!(can_add);
    assert!(error_is_nil, "third return is nil when canChange is true");
}

#[test]
fn can_change_talents_returns_error_string_when_blocked() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.talents.can_change_talents = false;
    }

    let (can_change, can_add, error_type): (bool, bool, String) = env
        .eval(
            r#"
            local canChange, canAdd, err = C_ClassTalents.CanChangeTalents()
            return canChange, canAdd, type(err)
            "#,
        )
        .unwrap();
    assert!(!can_change);
    assert!(!can_add);
    assert_eq!(error_type, "string");
}

#[test]
fn get_has_starter_build_defaults_to_false() {
    let env = env();
    let has_starter: bool = env
        .eval("return C_ClassTalents.GetHasStarterBuild()")
        .unwrap();
    assert!(!has_starter);
}

#[test]
fn get_has_starter_build_reflects_talent_state_field() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.talents.has_starter_build = true;
    }
    let has_starter: bool = env
        .eval("return C_ClassTalents.GetHasStarterBuild()")
        .unwrap();
    assert!(has_starter);
}

#[test]
fn is_starter_build_active_defaults_to_false() {
    let env = env();
    let active: bool = env
        .eval("return C_ClassTalents.IsStarterBuildActive()")
        .unwrap();
    assert!(!active);
}

#[test]
fn is_starter_build_active_reflects_talent_state_field() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.talents.is_starter_build_active = true;
    }
    let active: bool = env
        .eval("return C_ClassTalents.IsStarterBuildActive()")
        .unwrap();
    assert!(active);
}

#[test]
fn has_unspent_hero_talent_points_defaults_to_remaining_points_for_active_subtree() {
    let env = env();
    let (has_unspent, num_points): (bool, i32) = env
        .eval("return C_ClassTalents.HasUnspentHeroTalentPoints()")
        .unwrap();
    assert!(has_unspent);
    assert_eq!(
        num_points, 11,
        "the seeded hero subtree starts with its full budget available"
    );
}
