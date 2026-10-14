use super::fixtures::{self, GETTER, ID};
use super::helpers::{
    assert_fresh_snapshot, assert_lua, assert_replacement_snapshot, assert_selector_policy,
    install_snapshot_assertion,
};
use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn text_with_subtext_unknown_returns_exactly_one_nil_control() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        assert_lua(
            &env,
            "unknown selector control, not a populated RED",
            r#"
            local getter = C_UIWidgetManager.GetTextWithSubtextWidgetVisualizationInfo
            assert(type(getter) == 'function')
            local function oneNil(...)
                assert(select('#', ...) == 1 and select(1, ...) == nil)
            end
            oneNil(getter(762501))
            oneNil(getter(762502))
        "#,
        );
    });
}

#[test]
fn text_with_subtext_required_selectors_reject_missing_and_invalid_arguments() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::seed(&env, ID + 0.5, fixtures::supplied());
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        // Shared required-number assertions run before the populated snapshot assertion.
        assert_selector_policy(&env, ID + 0.5);
    });
}

#[test]
fn text_with_subtext_exact_27_fields_fresh_snapshots_and_all_field_replacement() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::seed(&env, ID, fixtures::supplied());
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        assert_lua(
            &env,
            "exact fixture cardinality",
            r#"
            local count = 0
            for _ in pairs(FixtureExpected) do count = count + 1 end
            assert(count == 27)
        "#,
        );
        assert_fresh_snapshot(&env, ID);
        fixtures::seed(&env, ID, fixtures::replacement());
        install_snapshot_assertion(&env, GETTER, fixtures::REPLACEMENT_EXPECTED);
        assert_replacement_snapshot(&env, ID);
    });
}

#[test]
fn text_with_subtext_two_environments_keep_records_and_replacements_isolated() {
    common::with_timeout(88, || {
        let first = WowLuaEnv::new().unwrap();
        let second = WowLuaEnv::new().unwrap();
        fixtures::seed(&first, ID, fixtures::supplied());
        assert_lua(
            &second,
            "other environment starts absent",
            r#"
            local function oneNil(...)
                assert(select('#', ...) == 1 and select(1, ...) == nil)
            end
            oneNil(C_UIWidgetManager.GetTextWithSubtextWidgetVisualizationInfo(762501))
        "#,
        );
        fixtures::seed(&second, ID, fixtures::replacement());
        install_snapshot_assertion(&first, GETTER, fixtures::EXPECTED);
        install_snapshot_assertion(&second, GETTER, fixtures::REPLACEMENT_EXPECTED);
        assert_lua(
            &first,
            "first independent record",
            "AssertSuppliedSnapshot(FixtureGetter(762501))",
        );
        assert_lua(
            &second,
            "second independent record",
            "AssertSuppliedSnapshot(FixtureGetter(762501))",
        );
        fixtures::seed(&first, ID, fixtures::replacement());
        fixtures::seed(&second, ID, fixtures::supplied());
        install_snapshot_assertion(&first, GETTER, fixtures::REPLACEMENT_EXPECTED);
        install_snapshot_assertion(&second, GETTER, fixtures::EXPECTED);
        assert_lua(
            &first,
            "first replacement",
            "AssertSuppliedSnapshot(FixtureGetter(762501))",
        );
        assert_lua(
            &second,
            "second replacement",
            "AssertSuppliedSnapshot(FixtureGetter(762501))",
        );
    });
}
