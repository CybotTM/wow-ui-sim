use super::fixtures::{ID, assert_initial_snapshot, assert_lua, seed_record, supplied_record};
use wow_ui_sim::c_api::ui_widget_manager::WidgetId;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn discrete_snapshot_is_complete_fresh_and_observes_supplied_replace_remove() {
    let env = WowLuaEnv::new().expect("isolated discrete environment");
    let record = supplied_record();
    seed_record(&env, ID, record.clone());
    assert_initial_snapshot(&env);
    // All assertions below are UNREACHED until genuine getter/record RED passes.
    assert_lua(
        &env,
        "fresh isolation and absence",
        r#"
        local get = C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo
        local copy = get(760301)
        assert(copy ~= savedDiscrete, 'fresh snapshot')
        copy.progressVal = -999
        copy.tooltip = 'caller mutation'
        copy.extra = true
        AssertDiscreteSnapshot(savedDiscrete, expectedDiscrete)
        AssertDiscreteSnapshot(get(760301), expectedDiscrete)
        assert(get(760302) == nil and get(1760301) == nil, 'no synthesized record')
    "#,
    );
    assert_eq!(
        env.state()
            .borrow()
            .ui_widget_inputs
            .discrete_progress_steps[&WidgetId::from_number(ID)],
        record
    );
    let mut replacement = record;
    replacement.progress_min = -10.5;
    replacement.progress_max = 120.25;
    replacement.progress_val = 36.75;
    replacement.num_steps = 6.5;
    replacement.tooltip = "Replaced discrete tooltip".into();
    replacement.order_index = 2.75;
    seed_record(&env, ID, replacement);
    assert_lua(
        &env,
        "replace preserves supplied numbers without consumer clamp",
        r#"
        AssertDiscreteSnapshot(savedDiscrete, expectedDiscrete)
        expectedDiscrete.progressMin = -10.5
        expectedDiscrete.progressMax = 120.25
        expectedDiscrete.progressVal = 36.75
        expectedDiscrete.numSteps = 6.5
        expectedDiscrete.tooltip = 'Replaced discrete tooltip'
        expectedDiscrete.orderIndex = 2.75
        local latest = C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(760301)
        assert(latest ~= savedDiscrete)
        AssertDiscreteSnapshot(latest, expectedDiscrete)
    "#,
    );
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .discrete_progress_steps
        .remove(&WidgetId::from_number(ID));
    assert_lua(
        &env,
        "supplied removal",
        r#"
        assert(C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(760301) == nil)
        assert(savedDiscrete.progressVal == 40 and savedDiscrete.numSteps == 4)
    "#,
    );
    let other = WowLuaEnv::new().expect("independent discrete environment");
    assert_lua(
        &other,
        "empty environment",
        r#"
        assert(C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(760301) == nil)
    "#,
    );
}

#[test]
fn discrete_numeric_ids_preserve_fractional_negative_and_signed_zero_keys() {
    let env = WowLuaEnv::new().expect("numeric discrete environment");
    seed_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    for id in [-2.5, 0.0, 617.25, 4_294_967_299.0] {
        let mut record = supplied_record();
        record.progress_val = id;
        seed_record(&env, id, record);
    }
    assert_lua(
        &env,
        "numeric contract like existing bars",
        r#"
        local get = C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo
        for _, id in ipairs({-2.5, 0, 617.25, 4294967299}) do
            assert(get(id).progressVal == id, 'no integer truncation or range policy')
        end
        assert(get(-0.0).progressVal == 0, 'signed zero shares numeric key')
        assert(get(-2) == nil and get(617) == nil and get(3) == nil)
        local function reject(...)
            local ok, message = pcall(get, ...)
            assert(not ok and type(message) == 'string')
            assert(string.find(message, 'widgetID must be a number', 1, true),
                'reject non-number rather than coerce it: ' .. tostring(message))
        end
        reject()
        reject(nil)
        reject('760301')
        reject(true)
        reject({})
        reject(function() end)
    "#,
    );
}

#[test]
fn discrete_same_ids_are_environment_owned() {
    let first = WowLuaEnv::new().expect("first environment");
    seed_record(&first, ID, supplied_record());
    assert_initial_snapshot(&first);
    let second = WowLuaEnv::new().expect("second environment");
    let mut record = supplied_record();
    record.progress_val = 62.25;
    record.tooltip = "Second environment".into();
    seed_record(&second, ID, record);
    assert_lua(
        &second,
        "second inputs",
        r#"
        local row = C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(760301)
        assert(row.progressVal == 62.25 and row.tooltip == 'Second environment')
    "#,
    );
    assert_lua(
        &first,
        "first inputs retained",
        r#"
        AssertDiscreteSnapshot(C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(760301),
            expectedDiscrete)
    "#,
    );
}
