use super::fixtures::{self, ID};
use wow_ui_sim::c_api::ui_widget_manager::WidgetId;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn text_column_complete_fields_fresh_outer_array_and_each_nested_entry() {
    let env = WowLuaEnv::new().unwrap();
    fixtures::seed(&env, ID, fixtures::supplied());
    fixtures::populated_boundary(&env);
    fixtures::assert_lua(
        &env,
        "full fields and nested caller mutation",
        r#"
        local fresh = ColumnOneSnapshot(ColumnExpected, ColumnGet(763621))
        assert(fresh ~= ColumnRetained and fresh.entries ~= ColumnRetained.entries)
        assert(fresh.entries[1] ~= fresh.entries[2], 'distinct sibling entry records')
        for index, entry in ipairs(fresh.entries) do
            assert(entry ~= ColumnRetained.entries[index], 'fresh nested entry ' .. index)
            entry.text = 'caller'; entry.enabledState = 7
            entry.hAlign = 1; entry.columnWidth = -99
        end
        fresh.entries[1] = fresh.entries[2]
        fresh.entries[2] = nil
        fresh.entries[9] = {text = 'sparse caller'}
        fresh.tooltip = 'caller'; fresh.shownState = 0; fresh.extra = true
        local again = ColumnOneSnapshot(ColumnExpected, ColumnGet(763621))
        assert(again ~= fresh and again ~= ColumnRetained)
        assert(again.entries ~= fresh.entries and again.entries ~= ColumnRetained.entries)
        for index, entry in ipairs(again.entries) do
            assert(entry ~= ColumnRetained.entries[index])
        end
        ColumnSnapshot(ColumnRetained, ColumnExpected)
        ColumnOneNil(ColumnGet(763622))
    "#,
    );
    assert_eq!(
        env.state()
            .borrow()
            .ui_widget_inputs
            .text_column_rows
            .get(&WidgetId::from_number(ID)),
        Some(&fixtures::supplied())
    );
}

#[test]
fn text_column_whole_replacement_empty_entries_deletion_and_environment_isolation() {
    let env = WowLuaEnv::new().unwrap();
    fixtures::seed(&env, ID, fixtures::supplied());
    fixtures::populated_boundary(&env);
    let other = WowLuaEnv::new().unwrap();
    fixtures::install_assertions(&other);
    fixtures::assert_lua(
        &other,
        "empty independent backing",
        "ColumnOneNil(ColumnGet(763621))",
    );
    fixtures::seed(&other, ID, fixtures::replacement());
    fixtures::seed(&env, ID, fixtures::replacement());
    fixtures::assert_lua(
        &env,
        "all nineteen fields and entries replaced",
        r#"
        ColumnReplaced = ColumnOneSnapshot(ColumnReplacement, ColumnGet(763621))
        assert(ColumnReplaced ~= ColumnRetained and ColumnReplaced.entries ~= ColumnRetained.entries)
        ColumnSnapshot(ColumnRetained, ColumnExpected)
        ColumnReplaced.entries[1].text = 'mutated replacement snapshot'
        ColumnOneSnapshot(ColumnReplacement, ColumnGet(763621))
    "#,
    );
    let mut empty = fixtures::replacement();
    empty.entries.clear();
    fixtures::seed(&env, ID, empty);
    fixtures::assert_lua(
        &env,
        "fresh empty arrays and retained nested entries",
        r#"
        local expected = {}
        for key, value in pairs(ColumnReplacement) do expected[key] = value end
        expected.entries = {}
        ColumnEmpty = ColumnOneSnapshot(expected, ColumnGet(763621))
        local again = ColumnOneSnapshot(expected, ColumnGet(763621))
        assert(again ~= ColumnEmpty and again.entries ~= ColumnEmpty.entries)
        assert(next(again.entries) == nil)
        ColumnSnapshot(ColumnRetained, ColumnExpected)
        assert(ColumnReplaced.entries[1].text == 'mutated replacement snapshot')
    "#,
    );
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .text_column_rows
        .remove(&WidgetId::from_number(ID));
    fixtures::assert_lua(
        &env,
        "deletion preserves previously returned records",
        r#"
        ColumnOneNil(ColumnGet(763621))
        ColumnSnapshot(ColumnRetained, ColumnExpected)
        assert(next(ColumnEmpty.entries) == nil)
        assert(ColumnReplaced.entries[2].text == 'Epsilon 947')
    "#,
    );
    fixtures::assert_lua(
        &other,
        "same ID survives other environment deletion",
        r#"
        ColumnOneSnapshot(ColumnReplacement, ColumnGet(763621))
    "#,
    );
}

#[test]
fn text_column_numeric_keys_preserve_fractions_large_values_and_signed_zero() {
    let env = WowLuaEnv::new().unwrap();
    fixtures::seed(&env, ID, fixtures::supplied());
    fixtures::populated_boundary(&env);
    for id in [-2.5, 617.25, 4294967299.0, -0.0] {
        fixtures::seed(&env, id, fixtures::supplied());
    }
    fixtures::assert_lua(
        &env,
        "existing WidgetId numeric policy, not invented ID ranges",
        r#"
        for _, id in ipairs({-2.5, 617.25, 4294967299, 0}) do
            ColumnOneSnapshot(ColumnExpected, ColumnGet(id))
        end
        for _, id in ipairs({-2, 617, 3, 763622}) do ColumnOneNil(ColumnGet(id)) end
        local invalid = {
            function() return ColumnGet() end,
            function() return ColumnGet(nil) end,
            function() return ColumnGet('763621') end,
            function() return ColumnGet(false) end,
            function() return ColumnGet({}) end,
        }
        for index, call in ipairs(invalid) do
            local ok, message = pcall(call)
            assert(not ok and type(message) == 'string', 'invalid selector accepted: ' .. index)
            assert(string.find(message, 'must be a number', 1, true), 'numeric boundary error')
        end
        ColumnOneSnapshot(ColumnExpected, ColumnGet(763621))
    "#,
    );
}
