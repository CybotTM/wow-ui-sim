use super::discrete_fixtures::assert_lua;
use super::fixtures::{ID, assert_initial_snapshot, seed_record, supplied_record};
use wow_ui_sim::c_api::ui_widget_manager::WidgetId;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn spell_display_complete_nested_fresh_replace_delete() {
    let env = WowLuaEnv::new().expect("SpellDisplay environment");
    let original = supplied_record();
    seed_record(&env, ID, original.clone());
    assert_initial_snapshot(&env);
    assert_lua(
        &env,
        "nested fresh snapshots",
        r#"
        local get = C_UIWidgetManager.GetSpellDisplayVisualizationInfo
        local copy = get(761301)
        assert(copy ~= savedSpell and copy.spellInfo ~= savedSpell.spellInfo)
        copy.spellInfo.text = 'caller mutation'
        copy.spellInfo.tint = 1
        copy.spellInfo.extra = true
        copy.widgetSizeSetting = -999
        AssertSpellSnapshot(savedSpell, expectedSpell)
        AssertSpellSnapshot(get(761301), expectedSpell)
        assert(select('#', get(761302)) == 1 and get(761302) == nil)
    "#,
    );
    assert_eq!(
        env.state().borrow().ui_widget_inputs.spell_display[&WidgetId::from_number(ID)],
        original
    );
    let mut replacement = original;
    replacement.spell_info.text = "Replaced spell".into();
    replacement.spell_info.tint = 1; // SpellDisplayTint.Red, both schemas
    replacement.spell_info.show_as_earned = true;
    replacement.spell_info.stack_display = 6.75;
    replacement.order_index = -2.25;
    replacement.widget_size_setting = 206.25;
    seed_record(&env, ID, replacement);
    assert_lua(
        &env,
        "nested replacement retains old snapshot",
        r#"
        AssertSpellSnapshot(savedSpell, expectedSpell)
        expectedSpell.spellInfo.text = 'Replaced spell'
        expectedSpell.spellInfo.tint = 1
        expectedSpell.spellInfo.showAsEarned = true
        expectedSpell.spellInfo.stackDisplay = 6.75
        expectedSpell.orderIndex = -2.25
        expectedSpell.widgetSizeSetting = 206.25
        local fresh = C_UIWidgetManager.GetSpellDisplayVisualizationInfo(761301)
        assert(fresh ~= savedSpell and fresh.spellInfo ~= savedSpell.spellInfo)
        AssertSpellSnapshot(fresh, expectedSpell)
    "#,
    );
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .spell_display
        .remove(&WidgetId::from_number(ID));
    assert_lua(
        &env,
        "explicit input deletion",
        r#"
        local get = C_UIWidgetManager.GetSpellDisplayVisualizationInfo
        assert(select('#', get(761301)) == 1 and get(761301) == nil)
        assert(savedSpell.spellInfo.text == 'Supplied spell' and savedSpell.spellInfo.tint == 0)
    "#,
    );
}

#[test]
fn spell_display_strict_numeric_ids_without_integer_range_inference() {
    let env = WowLuaEnv::new().expect("numeric environment");
    seed_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    for id in [-2.5, 0.0, 617.25, 4_294_967_299.0] {
        let mut record = supplied_record();
        record.order_index = id;
        seed_record(&env, id, record);
    }
    assert_lua(
        &env,
        "existing strict numeric key policy",
        r#"
        local get = C_UIWidgetManager.GetSpellDisplayVisualizationInfo
        for _, id in ipairs({-2.5, 0, 617.25, 4294967299}) do
            assert(get(id).orderIndex == id)
        end
        assert(get(-0.0).orderIndex == 0)
        assert(get(-2) == nil and get(617) == nil and get(3) == nil)
        local function reject(...)
            local ok, message = pcall(get, ...)
            assert(not ok and type(message) == 'string')
            assert(string.find(message, 'widgetID must be a number', 1, true), tostring(message))
        end
        reject(); reject(nil); reject('761301'); reject(true); reject({}); reject(function() end)
    "#,
    );
}

#[test]
fn spell_display_same_id_environment_isolation() {
    let first = WowLuaEnv::new().expect("first environment");
    seed_record(&first, ID, supplied_record());
    assert_initial_snapshot(&first);
    let second = WowLuaEnv::new().expect("second environment");
    assert_lua(
        &second,
        "empty independent map",
        "assert(C_UIWidgetManager.GetSpellDisplayVisualizationInfo(761301) == nil)",
    );
    let mut record = supplied_record();
    record.spell_info.text = "Second environment".into();
    seed_record(&second, ID, record);
    assert_lua(
        &second,
        "second nested record",
        "assert(C_UIWidgetManager.GetSpellDisplayVisualizationInfo(761301).spellInfo.text == 'Second environment')",
    );
    assert_lua(
        &first,
        "first nested record unchanged",
        "AssertSpellSnapshot(C_UIWidgetManager.GetSpellDisplayVisualizationInfo(761301), expectedSpell)",
    );
}
