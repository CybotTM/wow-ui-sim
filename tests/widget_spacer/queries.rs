use super::fixtures::{self, GETTER, ID};
use super::helpers::{
    assert_fresh_snapshot, assert_lua, assert_replacement_snapshot, assert_selector_policy,
    install_snapshot_assertion,
};
use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn spacer_absent_returns_exactly_one_nil() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        assert_lua(
            &env,
            "unknown selector control, not a populated RED",
            r#"
            local getter = C_UIWidgetManager.GetSpacerVisualizationInfo
            assert(type(getter) == 'function')
            local function oneNil(...)
                assert(select('#', ...) == 1 and select(1, ...) == nil)
            end
            oneNil(getter(762201))
            oneNil(getter(762202))
        "#,
        );
    });
}

#[test]
fn spacer_required_ordinary_number_and_fraction_selectors() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::inject(&env, ID + 0.5, fixtures::supplied());
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        // Shared required-number assertions run before the populated snapshot assertion.
        assert_selector_policy(&env, ID + 0.5);
        assert_lua(
            &env,
            "fraction not truncated",
            r#"
            assert(FixtureGetter(762201) == nil)
            AssertSuppliedSnapshot(FixtureGetter(762201.5))
        "#,
        );
    });
}

#[test]
fn spacer_full15_fresh_mutation_whole_replacement_and_environment_isolation() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::inject(&env, ID, fixtures::supplied());
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        assert_lua(
            &env,
            "exact fixture cardinality",
            r#"
            local count = 0
            for _ in pairs(FixtureExpected) do count = count + 1 end
            assert(count == 15)
        "#,
        );
        assert_fresh_snapshot(&env, ID);
        fixtures::inject(&env, ID, fixtures::replacement());
        install_snapshot_assertion(&env, GETTER, fixtures::REPLACEMENT_EXPECTED);
        assert_replacement_snapshot(&env, ID);
        assert_environment_isolation();
    });
}

fn assert_environment_isolation() {
    let first = WowLuaEnv::new().unwrap();
    let second = WowLuaEnv::new().unwrap();
    fixtures::inject(&first, ID, fixtures::supplied());
    assert_lua(
        &second,
        "other environment starts absent",
        r#"
        local function oneNil(...)
            assert(select('#', ...) == 1 and select(1, ...) == nil)
        end
        oneNil(C_UIWidgetManager.GetSpacerVisualizationInfo(762201))
    "#,
    );
    fixtures::inject(&second, ID, fixtures::replacement());
    assert_environment_snapshot(&first, fixtures::EXPECTED);
    assert_environment_snapshot(&second, fixtures::REPLACEMENT_EXPECTED);
    fixtures::inject(&first, ID, fixtures::replacement());
    fixtures::inject(&second, ID, fixtures::supplied());
    assert_environment_snapshot(&first, fixtures::REPLACEMENT_EXPECTED);
    assert_environment_snapshot(&second, fixtures::EXPECTED);
}

fn assert_environment_snapshot(env: &WowLuaEnv, expected: &str) {
    install_snapshot_assertion(env, GETTER, expected);
    assert_lua(
        env,
        "independent environment snapshot",
        "AssertSuppliedSnapshot(FixtureGetter(762201))",
    );
}
