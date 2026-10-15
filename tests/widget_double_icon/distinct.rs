use super::fixtures::{self, GETTER, ID};
use super::helpers::{assert_lua, install_snapshot_assertion};
use crate::common;
use wow_ui_sim::c_api::ui_widget_manager::WidgetId;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn double_icon_and_text_distinct_fractional_key_and_record_removal_preserve_other_snapshot() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::seed(&env, ID, fixtures::supplied());
        fixtures::seed(&env, ID + 0.5, fixtures::replacement());
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        assert_lua(
            &env,
            "first exact supplied key",
            "RetainedFirst = AssertSuppliedSnapshot(FixtureGetter(762505))",
        );
        install_snapshot_assertion(&env, GETTER, fixtures::REPLACEMENT_EXPECTED);
        assert_lua(
            &env,
            "distinct fractional key does not truncate to integer",
            r#"
            RetainedOther = AssertSuppliedSnapshot(FixtureGetter(762505.5))
            assert(RetainedOther ~= RetainedFirst)
            "#,
        );
        // Remove the explicit host input, not just set membership (covered by feed tests).
        env.state()
            .borrow_mut()
            .ui_widget_inputs
            .double_icon_and_text
            .remove(&WidgetId::from_number(ID));
        assert_lua(
            &env,
            "removed input is exactly one nil; other input and retained tables survive",
            r#"
            local function oneNil(...)
                assert(select('#', ...) == 1 and select(1, ...) == nil)
            end
            oneNil(FixtureGetter(762505))
            AssertSuppliedSnapshot(FixtureGetter(762505.5))
            AssertSuppliedSnapshot(RetainedOther)
            "#,
        );
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        assert_lua(
            &env,
            "retained removed input",
            "AssertSuppliedSnapshot(RetainedFirst)",
        );
        fixtures::seed(&env, ID, fixtures::supplied());
        assert_lua(
            &env,
            "explicit restoration creates another fresh snapshot",
            r#"
            local restored = AssertSuppliedSnapshot(FixtureGetter(762505))
            assert(restored ~= RetainedFirst and restored ~= RetainedOther)
            "#,
        );
    });
}
