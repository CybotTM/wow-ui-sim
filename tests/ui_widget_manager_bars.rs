#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

// Test-first supplied bar records; missing/nil getters are intended behavior boundaries.
// Synthetic supplied records, NOT native fixtures. Missing/nil getters are intended RED.
use wow_ui_sim::c_api::ui_widget_manager::{
    CaptureBarRecord, StatusBarRecord, UiWidgetInputs, WidgetId,
};
use wow_ui_sim::lua_api::WowLuaEnv;

const STATUS_ID: f64 = 617.0;
const CAPTURE_ID: f64 = 829.0;

fn supplied_status() -> StatusBarRecord {
    StatusBarRecord {
        shown_state: 1,
        bar_min: -10.5,
        bar_max: 120.25,
        bar_value: 36.75,
        text: "Supplied status".into(),
        tooltip: "Status detail".into(),
        bar_value_text_type: 2,
        override_bar_text: "Explicit override".into(),
        override_bar_text_shown_type: 1,
        color_tint: 3,
        partition_values: vec![70.5, 12.25, 70.5],
        tooltip_loc: 3,
        fill_motion_type: 1,
        bar_text_enabled_state: 1,
        bar_text_font_type: 2,
        bar_text_size_type: 3,
        text_enabled_state: 0,
        text_font_type: 1,
        text_size_type: 2,
        glow_anim_type: 1,
        show_glow_state: 1,
        fill_min_opacity: 0,
        fill_max_opacity: 1,
        widget_size_setting: 6.5,
        texture_kit: "SuppliedStatusFill".into(),
        frame_texture_kit: "SuppliedStatusFrame".into(),
        has_timer: true,
        order_index: 12.5,
        widget_tag: "status-tag".into(),
        in_anim_type: 1,
        out_anim_type: 2,
        widget_scale: 1,
        layout_direction: 2,
        model_scene_layer: 1,
        scripted_animation_effect_id: 73.0,
    }
}

fn supplied_capture() -> CaptureBarRecord {
    CaptureBarRecord {
        shown_state: 1,
        bar_value: 41.75,
        bar_min_value: -20.5,
        bar_max_value: 180.25,
        neutral_zone_size: 27.5,
        neutral_zone_center: 80.25,
        tooltip: "Capture detail".into(),
        glow_anim_type: 1,
        fill_direction_type: 1,
        tooltip_loc: 2,
        widget_size_setting: 9.5,
        texture_kit: "SuppliedCaptureFill".into(),
        frame_texture_kit: "SuppliedCaptureFrame".into(),
        has_timer: false,
        order_index: 4.25,
        widget_tag: "capture-tag".into(),
        in_anim_type: 2,
        out_anim_type: 1,
        widget_scale: 2,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 91.0,
    }
}

fn supplied_env(inputs: UiWidgetInputs) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.state().borrow_mut().ui_widget_inputs = inputs;
    env.exec(
        r#"
        function AssertBarSnapshot(actual, expected, path)
            path = path or 'snapshot'
            assert(type(actual) == type(expected), 'wrong type: ' .. path)
            if type(expected) ~= 'table' then
                assert(actual == expected, 'wrong value: ' .. path)
                return
            end
            for key, value in pairs(expected) do
                AssertBarSnapshot(actual[key], value, path .. '.' .. tostring(key))
            end
            for key in pairs(actual) do
                assert(expected[key] ~= nil, 'extra field: ' .. path .. '.' .. tostring(key))
            end
        end
        "#,
    )
    .unwrap();
    env
}

#[test]
fn status_bar_queries_complete_ordered_fresh_snapshots_and_supplied_updates() {
    let record = supplied_status();
    let mut inputs = UiWidgetInputs::default();
    inputs
        .status_bars
        .insert(WidgetId::from_number(STATUS_ID), record.clone());
    let env = supplied_env(inputs);
    env.exec(
        r#"
        expectedStatus = {
            shownState = 1, barMin = -10.5, barMax = 120.25, barValue = 36.75,
            text = 'Supplied status', tooltip = 'Status detail', barValueTextType = 2,
            overrideBarText = 'Explicit override', overrideBarTextShownType = 1,
            colorTint = 3, partitionValues = {70.5, 12.25, 70.5}, tooltipLoc = 3,
            fillMotionType = 1, barTextEnabledState = 1, barTextFontType = 2,
            barTextSizeType = 3, textEnabledState = 0, textFontType = 1,
            textSizeType = 2, glowAnimType = 1, showGlowState = 1,
            fillMinOpacity = 0, fillMaxOpacity = 1, widgetSizeSetting = 6.5,
            textureKit = 'SuppliedStatusFill', frameTextureKit = 'SuppliedStatusFrame',
            hasTimer = true, orderIndex = 12.5, widgetTag = 'status-tag',
            inAnimType = 1, outAnimType = 2, widgetScale = 1, layoutDirection = 2,
            modelSceneLayer = 1, scriptedAnimationEffectID = 73,
        }
        firstStatus = C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(617)
        AssertBarSnapshot(firstStatus, expectedStatus)
        savedStatus = C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(617)
        assert(savedStatus ~= firstStatus)
        assert(savedStatus.partitionValues ~= firstStatus.partitionValues)
        firstStatus.text = 'Caller mutation'
        firstStatus.partitionValues[1] = -999
        firstStatus.partitionValues[4] = 1000
        firstStatus.extra = true
        AssertBarSnapshot(savedStatus, expectedStatus)
        AssertBarSnapshot(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(617), expectedStatus)
        assert(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(618) == nil)
        assert(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(1000617) == nil)
        assert(C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(617) == nil)
        "#,
    )
    .unwrap();
    assert_eq!(
        env.state().borrow().ui_widget_inputs.status_bars[&WidgetId::from_number(STATUS_ID)],
        record
    );
    let mut updated = record;
    updated.bar_value = 88.5;
    updated.partition_values = vec![99.25, 1.5];
    updated.order_index = 2.75;
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .status_bars
        .insert(WidgetId::from_number(STATUS_ID), updated);
    env.exec(
        r#"
        AssertBarSnapshot(savedStatus, expectedStatus)
        expectedStatus.barValue = 88.5
        expectedStatus.partitionValues = {99.25, 1.5}
        expectedStatus.orderIndex = 2.75
        local latest = C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(617)
        assert(latest ~= savedStatus and latest.partitionValues ~= savedStatus.partitionValues)
        AssertBarSnapshot(latest, expectedStatus)
        "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .status_bars
        .get_mut(&WidgetId::from_number(STATUS_ID))
        .unwrap()
        .partition_values
        .clear();
    env.exec(
        r#"
        expectedStatus.partitionValues = {}
        AssertBarSnapshot(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(617), expectedStatus)
        assert(savedStatus.partitionValues[1] == 70.5 and #savedStatus.partitionValues == 3)
        "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .status_bars
        .remove(&WidgetId::from_number(STATUS_ID));
    env.exec("assert(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(617) == nil)")
        .unwrap();
    supplied_env(UiWidgetInputs::default())
        .exec("assert(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(617) == nil)")
        .unwrap();
}

#[test]
fn capture_bar_queries_complete_fresh_snapshots_and_supplied_updates() {
    let record = supplied_capture();
    let mut inputs = UiWidgetInputs::default();
    inputs
        .capture_bars
        .insert(WidgetId::from_number(CAPTURE_ID), record.clone());
    let env = supplied_env(inputs);
    env.exec(
        r#"
        expectedCapture = {
            shownState = 1, barValue = 41.75, barMinValue = -20.5, barMaxValue = 180.25,
            neutralZoneSize = 27.5, neutralZoneCenter = 80.25, tooltip = 'Capture detail',
            glowAnimType = 1, fillDirectionType = 1, tooltipLoc = 2,
            widgetSizeSetting = 9.5, textureKit = 'SuppliedCaptureFill',
            frameTextureKit = 'SuppliedCaptureFrame', hasTimer = false,
            orderIndex = 4.25, widgetTag = 'capture-tag', inAnimType = 2,
            outAnimType = 1, widgetScale = 2, layoutDirection = 1,
            modelSceneLayer = 0, scriptedAnimationEffectID = 91,
        }
        firstCapture = C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(829)
        AssertBarSnapshot(firstCapture, expectedCapture)
        savedCapture = C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(829)
        assert(savedCapture ~= firstCapture)
        firstCapture.neutralZoneCenter = -999
        firstCapture.tooltip = 'Caller mutation'
        firstCapture.extra = true
        AssertBarSnapshot(savedCapture, expectedCapture)
        AssertBarSnapshot(C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(829), expectedCapture)
        assert(C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(830) == nil)
        assert(C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(1000829) == nil)
        assert(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(829) == nil)
        "#,
    )
    .unwrap();
    assert_eq!(
        env.state().borrow().ui_widget_inputs.capture_bars[&WidgetId::from_number(CAPTURE_ID)],
        record
    );
    let mut updated = record;
    updated.bar_value = 99.5;
    updated.neutral_zone_size = 13.25;
    updated.neutral_zone_center = 66.75;
    updated.order_index = 18.5;
    updated.has_timer = true;
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .capture_bars
        .insert(WidgetId::from_number(CAPTURE_ID), updated);
    env.exec(
        r#"
        AssertBarSnapshot(savedCapture, expectedCapture)
        expectedCapture.barValue = 99.5
        expectedCapture.neutralZoneSize = 13.25
        expectedCapture.neutralZoneCenter = 66.75
        expectedCapture.orderIndex = 18.5
        expectedCapture.hasTimer = true
        local latest = C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(829)
        assert(latest ~= savedCapture)
        AssertBarSnapshot(latest, expectedCapture)
        "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .capture_bars
        .remove(&WidgetId::from_number(CAPTURE_ID));
    env.exec("assert(C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(829) == nil)")
        .unwrap();
    supplied_env(UiWidgetInputs::default())
        .exec("assert(C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(829) == nil)")
        .unwrap();
}
