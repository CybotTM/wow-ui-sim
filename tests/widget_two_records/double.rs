//! Explicit synthetic records, not native catalog defaults.
use wow_ui_sim::c_api::ui_widget_manager::DoubleStatusBarRecord;

pub const GETTER: &str = "GetDoubleStatusBarWidgetVisualizationInfo";
pub const EXPECTED: &str = r#"{
    shownState = 1, leftBarMin = 10, leftBarMax = 110, leftBarValue = 40.25,
    leftBarTooltip = 'Left supplied tooltip', rightBarMin = -20, rightBarMax = 80,
    rightBarValue = 35.5, rightBarTooltip = 'Right supplied tooltip',
    barValueTextType = 2, text = 'Double supplied label', leftBarTooltipLoc = 3,
    rightBarTooltipLoc = 4, fillMotionType = 0, widgetSizeSetting = 105,
    textureKit = 'white', frameTextureKit = 'widgetstatusbar', hasTimer = false,
    orderIndex = 10.25, widgetTag = 'double-status-fixture', inAnimType = 0,
    outAnimType = 0, widgetScale = 0, layoutDirection = 1, modelSceneLayer = 0,
    scriptedAnimationEffectID = 0,
}"#;

pub fn supplied() -> DoubleStatusBarRecord {
    DoubleStatusBarRecord {
        shown_state: 1,
        left_bar_min: 10.0,
        left_bar_max: 110.0,
        left_bar_value: 40.25,
        left_bar_tooltip: "Left supplied tooltip".into(),
        right_bar_min: -20.0,
        right_bar_max: 80.0,
        right_bar_value: 35.5,
        right_bar_tooltip: "Right supplied tooltip".into(),
        bar_value_text_type: 2,
        text: "Double supplied label".into(),
        left_bar_tooltip_loc: 3,
        right_bar_tooltip_loc: 4,
        fill_motion_type: 0,
        widget_size_setting: 105.0,
        texture_kit: "white".into(),
        frame_texture_kit: "widgetstatusbar".into(),
        has_timer: false,
        order_index: 10.25,
        widget_tag: "double-status-fixture".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 0,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}

// Changes every field for projection-only tests; no cached Setup sees these arbitrary enums/kits.
pub fn replacement() -> DoubleStatusBarRecord {
    DoubleStatusBarRecord {
        shown_state: 0,
        left_bar_min: -7.5,
        left_bar_max: 200.25,
        left_bar_value: 91.5,
        left_bar_tooltip: "Replacement left".into(),
        right_bar_min: -50.25,
        right_bar_max: 150.5,
        right_bar_value: -2.25,
        right_bar_tooltip: "Replacement right".into(),
        bar_value_text_type: 1,
        text: "Replacement double".into(),
        left_bar_tooltip_loc: 5,
        right_bar_tooltip_loc: 3,
        fill_motion_type: 1,
        widget_size_setting: 127.5,
        texture_kit: "replacement-fill".into(),
        frame_texture_kit: "replacement-frame".into(),
        has_timer: true,
        order_index: -4.5,
        widget_tag: "replacement-double-tag".into(),
        in_anim_type: 1,
        out_anim_type: 2,
        widget_scale: 1,
        layout_direction: 2,
        model_scene_layer: 1,
        scripted_animation_effect_id: 813.5,
    }
}

pub const REPLACEMENT_EXPECTED: &str = r#"{
    shownState = 0, leftBarMin = -7.5, leftBarMax = 200.25, leftBarValue = 91.5,
    leftBarTooltip = 'Replacement left', rightBarMin = -50.25, rightBarMax = 150.5,
    rightBarValue = -2.25, rightBarTooltip = 'Replacement right',
    barValueTextType = 1, text = 'Replacement double', leftBarTooltipLoc = 5,
    rightBarTooltipLoc = 3, fillMotionType = 1, widgetSizeSetting = 127.5,
    textureKit = 'replacement-fill', frameTextureKit = 'replacement-frame', hasTimer = true,
    orderIndex = -4.5, widgetTag = 'replacement-double-tag', inAnimType = 1,
    outAnimType = 2, widgetScale = 1, layoutDirection = 2, modelSceneLayer = 1,
    scriptedAnimationEffectID = 813.5,
}"#;
