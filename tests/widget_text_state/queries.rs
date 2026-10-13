use super::discrete_fixtures::assert_lua;
use super::fixtures::{ID, assert_initial_snapshot, seed_record, supplied_record};
use wow_ui_sim::c_api::ui_widget_manager::WidgetId;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn text_state_complete21_fresh_snapshots_unknown_nil_replace_remove() {
    let env = WowLuaEnv::new().expect("text-state environment");
    let record = supplied_record();
    seed_record(&env, ID, record.clone());
    assert_initial_snapshot(&env);
    assert_lua(
        &env,
        "TextWithState freshness and exactly one unknown nil",
        r#"
        local get = C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo
        local copy = get(760801)
        assert(copy ~= savedTextState)
        copy.text = 'caller mutation'
        copy.bottomPadding = -999
        copy.extra = true
        AssertTextStateSnapshot(savedTextState, expectedTextState)
        AssertTextStateSnapshot(get(760801), expectedTextState)
        assert(select('#', get(760802)) == 1 and get(760802) == nil)
        assert(select('#', get(1760801)) == 1 and get(1760801) == nil)
    "#,
    );
    assert_eq!(
        env.state().borrow().ui_widget_inputs.text_with_state[&WidgetId::from_number(ID)],
        record
    );
    let mut replacement = record;
    replacement.text = "Replaced state text".into();
    replacement.tooltip = "Replaced tooltip".into();
    replacement.bottom_padding = -3.75;
    replacement.widget_size_setting = 206.25;
    replacement.order_index = -2.25;
    replacement.scripted_animation_effect_id = 7.5;
    // Discriminators are lookup policy, not consumer/native admissibility claims.
    replacement.texture_kit = "query-only-kit".into();
    replacement.frame_texture_kit = "query-only-frame-kit".into();
    replacement.has_timer = true;
    seed_record(&env, ID, replacement);
    assert_lua(
        &env,
        "TextWithState replacement without consumer clamping",
        r#"
        AssertTextStateSnapshot(savedTextState, expectedTextState)
        expectedTextState.text = 'Replaced state text'
        expectedTextState.tooltip = 'Replaced tooltip'
        expectedTextState.bottomPadding = -3.75
        expectedTextState.widgetSizeSetting = 206.25
        expectedTextState.orderIndex = -2.25
        expectedTextState.scriptedAnimationEffectID = 7.5
        expectedTextState.textureKit = 'query-only-kit'
        expectedTextState.frameTextureKit = 'query-only-frame-kit'
        expectedTextState.hasTimer = true
        local fresh = C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(760801)
        assert(fresh ~= savedTextState)
        AssertTextStateSnapshot(fresh, expectedTextState)
    "#,
    );
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .text_with_state
        .remove(&WidgetId::from_number(ID));
    assert_lua(
        &env,
        "TextWithState explicit input deletion",
        r#"
        local get = C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo
        assert(select('#', get(760801)) == 1 and get(760801) == nil)
        assert(savedTextState.text == 'Supplied state text' and savedTextState.bottomPadding == 2.5)
    "#,
    );
}

#[test]
fn text_state_numeric_ids_and_strict_widget_id_arguments() {
    let env = WowLuaEnv::new().expect("text-state numeric environment");
    seed_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    for id in [-2.5, 0.0, 617.25, 4_294_967_299.0] {
        let mut record = supplied_record();
        record.order_index = id;
        seed_record(&env, id, record);
    }
    assert_lua(
        &env,
        "TextWithState common read_numeric_id contract",
        r#"
        local get = C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo
        for _, id in ipairs({-2.5, 0, 617.25, 4294967299}) do
            assert(get(id).orderIndex == id, 'no truncation or native range policy')
        end
        assert(get(-0.0).orderIndex == 0)
        assert(get(-2) == nil and get(617) == nil and get(3) == nil)
        local function reject(...)
            local ok, message = pcall(get, ...)
            assert(not ok and type(message) == 'string')
            assert(string.find(message, 'widgetID must be a number', 1, true), tostring(message))
        end
        reject()
        reject(nil)
        reject('760801')
        reject(true)
        reject({})
        reject(function() end)
    "#,
    );
}

#[test]
fn text_state_same_id_inputs_are_environment_local() {
    let first = WowLuaEnv::new().expect("first text-state environment");
    seed_record(&first, ID, supplied_record());
    assert_initial_snapshot(&first);
    let second = WowLuaEnv::new().expect("second text-state environment");
    assert_lua(
        &second,
        "TextWithState empty independent map",
        "assert(C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(760801) == nil)",
    );
    let mut record = supplied_record();
    record.text = "Second environment".into();
    record.bottom_padding = 6.75;
    seed_record(&second, ID, record);
    assert_lua(
        &second,
        "TextWithState independent supplied map",
        r#"
        local row = C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(760801)
        assert(row.text == 'Second environment' and row.bottomPadding == 6.75)
    "#,
    );
    assert_lua(
        &first,
        "TextWithState first environment retained",
        r#"
        AssertTextStateSnapshot(C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(760801),
            expectedTextState)
    "#,
    );
}
