use super::fixtures::{ID, assert_initial_snapshot, assert_lua, supplied_record, supply_record};
use wow_ui_sim::c_api::ui_widget_manager::WidgetId;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn bullet_supplied_fifteen_fields_and_fresh_ordered_lines() {
    let env = WowLuaEnv::new().expect("supplied bullet environment");
    supply_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    assert_lua(
        &env,
        "fresh row and fresh lines protect host input",
        r#"
        local get = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo
        local fresh = get(760401)
        assert(fresh ~= retainedBullet and fresh.lines ~= retainedBullet.lines)
        fresh.lines[1] = 'caller mutation'
        fresh.lines[2] = nil
        fresh.lines[9] = 'sparse mutation'
        fresh.enabledState = 0
        fresh.widgetTag = 'caller mutation'
        AssertBulletSnapshot(retainedBullet, expectedBullet)
        AssertBulletSnapshot(get(760401), expectedBullet)
    "#,
    );
    assert_eq!(
        env.state()
            .borrow()
            .ui_widget_inputs
            .bullet_text_lists
            .get(&WidgetId::from_number(ID)),
        Some(&supplied_record())
    );
}

#[test]
fn bullet_replacement_empty_lines_deletion_and_environment_isolation() {
    let env = WowLuaEnv::new().expect("replacement bullet environment");
    supply_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    let other = WowLuaEnv::new().expect("independent bullet environment");
    assert_lua(
        &other,
        "independent empty backing",
        r#"
        local get = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo
        assert(select('#', get(760401)) == 1 and get(760401) == nil)
    "#,
    );
    let mut replacement = supplied_record();
    replacement.lines.clear();
    replacement.enabled_state = 0;
    supply_record(&other, ID, replacement.clone());
    supply_record(&env, ID, replacement);
    assert_lua(
        &env,
        "replacement leaves retained ordered strings untouched",
        r#"
        local get = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo
        local row = get(760401)
        local expected = {}
        for key, value in pairs(expectedBullet) do expected[key] = value end
        expected.lines = {}
        expected.enabledState = 0
        AssertBulletSnapshot(row, expected)
        local fresh = get(760401)
        assert(fresh ~= row and fresh.lines ~= row.lines, 'fresh empty lines')
        retainedEmptyBullet = row
        assert(row ~= retainedBullet and row.lines ~= retainedBullet.lines)
        AssertBulletSnapshot(retainedBullet, expectedBullet)
    "#,
    );
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .bullet_text_lists
        .remove(&WidgetId::from_number(ID));
    assert_lua(
        &env,
        "record deletion retains snapshots",
        r#"
        local get = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo
        assert(select('#', get(760401)) == 1 and get(760401) == nil)
        AssertBulletSnapshot(retainedBullet, expectedBullet)
        assert(#retainedEmptyBullet.lines == 0 and retainedEmptyBullet.enabledState == 0)
    "#,
    );
    assert_lua(
        &other,
        "same ID survives deletion in other environment",
        r#"
        local row = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo(760401)
        assert(type(row) == 'table' and #row.lines == 0 and row.enabledState == 0)
    "#,
    );
}

#[test]
fn bullet_numeric_keys_do_not_truncate_and_signed_zeroes_share_a_key() {
    let env = WowLuaEnv::new().expect("numeric bullet keys");
    supply_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    for id in [-2.5, 617.25, 4294967299.0, -0.0] {
        supply_record(&env, id, supplied_record());
    }
    assert_lua(
        &env,
        "existing numeric WidgetId policy",
        r#"
        local get = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo
        for _, id in ipairs({-2.5, 617.25, 4294967299, 0}) do
            assert(select('#', get(id)) == 1)
            AssertBulletSnapshot(get(id), expectedBullet)
        end
        for _, id in ipairs({-2, 617, 3, 760402}) do
            assert(select('#', get(id)) == 1 and get(id) == nil)
        end
    "#,
    );
}
