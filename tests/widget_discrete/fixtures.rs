use wow_ui_sim::c_api::ui_widget_manager::{DiscreteProgressStepsRecord, WidgetId};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 760_301.0;
pub(super) const SET_ID: f64 = 421.0;

pub(super) fn supplied_record() -> DiscreteProgressStepsRecord {
    DiscreteProgressStepsRecord {
        shown_state: 1,
        tooltip: "Supplied discrete tooltip".into(),
        progress_min: 10.0,
        progress_max: 70.0,
        progress_val: 40.0,
        num_steps: 4.0,
        tooltip_loc: 3,
        widget_size_setting: 140.0,
        // Catalog metadata in data/atlas.rs; runtime application still unreached.
        texture_kit: "eyeofthejailer".into(),
        frame_texture_kit: String::new(),
        has_timer: false,
        order_index: 12.5,
        widget_tag: "discrete-supplied".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 0,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}

pub(super) fn seed_record(env: &WowLuaEnv, id: f64, record: DiscreteProgressStepsRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .discrete_progress_steps
        .insert(WidgetId::from_number(id), record);
}

pub(super) fn assert_lua(env: &WowLuaEnv, phase: &str, code: &str) {
    env.exec(code)
        .unwrap_or_else(|error| panic!("discrete [{phase}]: {error}"));
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        errors.is_empty(),
        "discrete [{phase}] Lua errors: {errors:?}"
    );
}

pub(super) fn assert_initial_snapshot(env: &WowLuaEnv) {
    // Always BEFORE cache loading, atlas checks, XML construction, or other prerequisites.
    assert_lua(
        env,
        "first genuine getter/record RED",
        r#"
        local get = C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo
        assert(type(get) == 'function', 'missing DiscreteProgressSteps getter')
        local first = get(760301)
        assert(type(first) == 'table', 'missing supplied DiscreteProgressSteps record')
        expectedDiscrete = {
            shownState = 1, tooltip = 'Supplied discrete tooltip',
            progressMin = 10, progressMax = 70, progressVal = 40, numSteps = 4,
            tooltipLoc = 3, widgetSizeSetting = 140, textureKit = 'eyeofthejailer',
            frameTextureKit = '', hasTimer = false, orderIndex = 12.5,
            widgetTag = 'discrete-supplied', inAnimType = 0, outAnimType = 0,
            widgetScale = 0, layoutDirection = 1, modelSceneLayer = 0,
            scriptedAnimationEffectID = 0,
        }
        function AssertDiscreteSnapshot(actual, expected)
            assert(type(actual) == 'table', 'complete supplied table')
            for key, value in pairs(expected) do
                assert(type(actual[key]) == type(value), 'field type: ' .. key)
                assert(actual[key] == value, 'field value: ' .. key)
            end
            for key in pairs(actual) do
                assert(expected[key] ~= nil, 'unexpected field: ' .. tostring(key))
            end
        end
        AssertDiscreteSnapshot(first, expectedDiscrete)
        savedDiscrete = first
    "#,
    );
}
