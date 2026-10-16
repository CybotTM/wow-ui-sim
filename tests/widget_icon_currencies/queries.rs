use super::fixtures::{self, ID};
use super::helpers::assert_lua;
use crate::common;
use wow_ui_sim::c_api::ui_widget_manager::WidgetId;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn icon_currencies_complete_nested_snapshots_are_fresh_and_deep_caller_mutations_are_isolated() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::seed(&env, ID, fixtures::supplied());
        fixtures::install_assertions(&env);
        assert_lua(
            &env,
            "complete multirow snapshot and deep caller mutations",
            r#"
            local first = AssertSuppliedSnapshot(FixtureGetter(762507))
            Retained = AssertSuppliedSnapshot(FixtureGetter(762507))
            assert(first ~= Retained and first.currencies ~= Retained.currencies)
            for i=1,2 do
                assert(first.currencies[i] ~= Retained.currencies[i])
                assert(first.currencies[i] ~= first.currencies[3-i], 'independent rows')
            end
            local firstRow = first.currencies[1]
            for key in pairs(firstRow) do firstRow[key] = nil end
            firstRow.text = 'caller edit'
            first.currencies[2].tooltip = 'caller tooltip'
            table.remove(first.currencies, 1)
            table.insert(first.currencies, {text='caller inserted row'})
            first.currencies.extra = true
            first.text = 'caller outer edit'
            first.currencies = {firstRow}
            first.unexpected = 9
            AssertSuppliedSnapshot(Retained)
            local nextSnapshot = AssertSuppliedSnapshot(FixtureGetter(762507))
            assert(nextSnapshot ~= first and nextSnapshot ~= Retained)
            assert(nextSnapshot.currencies ~= Retained.currencies)
            for i=1,2 do assert(nextSnapshot.currencies[i] ~= Retained.currencies[i]) end
        "#,
        );
    });
}

#[test]
fn icon_currencies_all_fields_replace_without_mutating_retained_outer_array_or_rows() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::seed(&env, ID, fixtures::supplied());
        fixtures::install_assertions(&env);
        assert_lua(
            &env,
            "retain initial complete record",
            r#"
            Retained = AssertSuppliedSnapshot(FixtureGetter(762507))
            RetainedRows, RetainedFirst = Retained.currencies, Retained.currencies[1]
        "#,
        );
        fixtures::seed(&env, ID, fixtures::replacement());
        fixtures::expect(&env, "CurrencyReplacement");
        assert_lua(
            &env,
            "replace all twenty outer and ten per-row fields",
            r#"
            local current = AssertSuppliedSnapshot(FixtureGetter(762507))
            assert(current ~= Retained and current.currencies ~= RetainedRows)
            for i=1,2 do assert(current.currencies[i] ~= RetainedRows[i]) end
            AssertCurrencyAt(762507, CurrencyReplacement)
            FixtureExpected = CurrencyExpected
            AssertSuppliedSnapshot(Retained)
            assert(Retained.currencies == RetainedRows and RetainedRows[1] == RetainedFirst)
        "#,
        );
        // Projection only: distinguish the independently supplied animation fields.
        let mut asymmetric = fixtures::supplied();
        asymmetric.in_anim_type = 1;
        fixtures::seed(&env, ID, asymmetric);
        assert_lua(
            &env,
            "independent in/out animation projection",
            r#"
            local expected = CopyCurrencyExpected(CurrencyExpected)
            expected.inAnimType = 1
            AssertCurrencyAt(762507, expected)
            FixtureExpected = CurrencyExpected
            AssertSuppliedSnapshot(Retained)
        "#,
        );
    });
}

#[test]
fn icon_currencies_fresh_environments_isolate_initial_records_replacement_and_removal() {
    common::with_timeout(88, || {
        let first = WowLuaEnv::new().unwrap();
        let second = WowLuaEnv::new().unwrap();
        fixtures::install_assertions(&first);
        fixtures::install_assertions(&second);
        fixtures::seed(&first, ID, fixtures::supplied());
        assert_lua(
            &second,
            "other environment starts absent",
            r#"
            local function oneNil(...) assert(select('#', ...) == 1 and (...) == nil) end
            oneNil(FixtureGetter(762507))
        "#,
        );
        fixtures::seed(&second, ID, fixtures::replacement());
        assert_lua(
            &first,
            "first complete input",
            "AssertCurrencyAt(762507, CurrencyExpected)",
        );
        assert_lua(
            &second,
            "second independent input",
            "AssertCurrencyAt(762507, CurrencyReplacement)",
        );
        fixtures::seed(&first, ID, fixtures::replacement());
        fixtures::seed(&second, ID, fixtures::supplied());
        assert_lua(
            &first,
            "first replacement",
            "AssertCurrencyAt(762507, CurrencyReplacement)",
        );
        assert_lua(
            &second,
            "second replacement",
            "AssertCurrencyAt(762507, CurrencyExpected)",
        );
        first
            .state()
            .borrow_mut()
            .ui_widget_inputs
            .icon_text_and_currencies
            .remove(&WidgetId::from_number(ID));
        assert_lua(
            &second,
            "other environment survives first removal",
            "AssertCurrencyAt(762507, CurrencyExpected)",
        );
    });
}

#[test]
fn icon_currencies_distinct_fractional_ids_removal_and_restoration_keep_deep_snapshots() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::install_assertions(&env);
        fixtures::seed(&env, ID, fixtures::supplied());
        fixtures::seed(&env, ID + 0.5, fixtures::replacement());
        assert_lua(
            &env,
            "numeric identities do not truncate",
            r#"
            Retained = AssertCurrencyAt(762507, CurrencyExpected)
            Other = AssertCurrencyAt(762507.5, CurrencyReplacement)
            assert(Retained.currencies[1] ~= Other.currencies[1])
        "#,
        );
        env.state()
            .borrow_mut()
            .ui_widget_inputs
            .icon_text_and_currencies
            .remove(&WidgetId::from_number(ID));
        assert_lua(
            &env,
            "absence removes only explicit record",
            r#"
            local function oneNil(...) assert(select('#', ...) == 1 and (...) == nil) end
            oneNil(FixtureGetter(762507))
            AssertCurrencyAt(762507.5, CurrencyReplacement)
            FixtureExpected = CurrencyExpected
            AssertSuppliedSnapshot(Retained)
            FixtureExpected = CurrencyReplacement
            AssertSuppliedSnapshot(Other)
        "#,
        );
        fixtures::seed(&env, ID, fixtures::supplied());
        assert_lua(
            &env,
            "restored record has new nested snapshots",
            r#"
            local restored = AssertCurrencyAt(762507, CurrencyExpected)
            assert(restored ~= Retained and restored.currencies ~= Retained.currencies)
            assert(restored.currencies[1] ~= Retained.currencies[1])
            AssertCurrencyAt(762507.5, CurrencyReplacement)
        "#,
        );
    });
}

#[test]
fn icon_currencies_empty_array_is_a_fresh_public_table_and_multirows_restore_in_order() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::install_assertions(&env);
        let mut empty = fixtures::supplied();
        empty.currencies.clear();
        fixtures::seed(&env, ID, empty);
        fixtures::expect(&env, "CopyCurrencyExpected(CurrencyExpected)");
        assert_lua(
            &env,
            "empty is table not omitted or nil",
            r#"
            FixtureExpected.currencies = {}
            EmptySnapshot = AssertSuppliedSnapshot(FixtureGetter(762507))
            local second = AssertSuppliedSnapshot(FixtureGetter(762507))
            assert(EmptySnapshot.currencies ~= second.currencies)
            table.insert(second.currencies, {text='caller insertion into empty array'})
            AssertSuppliedSnapshot(EmptySnapshot)
            AssertSuppliedSnapshot(FixtureGetter(762507))
        "#,
        );
        fixtures::seed(&env, ID, fixtures::supplied());
        assert_lua(
            &env,
            "rows restore without filling old empty snapshot",
            r#"
            AssertCurrencyAt(762507, CurrencyExpected)
            AssertSuppliedSnapshot(EmptySnapshot)
        "#,
        );
    });
}
