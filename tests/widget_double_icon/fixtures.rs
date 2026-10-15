//! Explicit synthetic host records, never native catalog/default observations.
use wow_ui_sim::c_api::ui_widget_manager::{
    DoubleIconAndTextRecord, WidgetId, WidgetSetEntry, WidgetSetRecord, WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 762_505.0;
pub(super) const SET_ID: f64 = 439.0;
pub(super) const TYPE: u32 = 5;
pub(super) const GETTER: &str = "GetDoubleIconAndTextWidgetVisualizationInfo";
pub(super) const EXPECTED: &str = r#"{
    shownState = 1,
    label = "Double label 731",
    leftText = "Left text 731",
    leftTooltip = "Left tooltip 731",
    rightText = "Right text 731",
    rightTooltip = "Right tooltip 731",
    tooltipLoc = 3,
    widgetSizeSetting = 180.5,
    textureKit = "",
    frameTextureKit = "",
    hasTimer = false,
    orderIndex = 37.5,
    widgetTag = "doubleicon-contract-731",
    inAnimType = 0,
    outAnimType = 0,
    widgetScale = 1,
    layoutDirection = 1,
    modelSceneLayer = 0,
    scriptedAnimationEffectID = 731.25,
}"#;
pub(super) const REPLACEMENT_EXPECTED: &str = r#"{
    shownState = 0,
    label = "Replacement label 947",
    leftText = "Replacement left 947",
    leftTooltip = "Replacement left tooltip 947",
    rightText = "Replacement right 947",
    rightTooltip = "Replacement right tooltip 947",
    tooltipLoc = 5,
    widgetSizeSetting = 206.25,
    textureKit = "replacement-foreground",
    frameTextureKit = "replacement-background",
    hasTimer = true,
    orderIndex = -12.25,
    widgetTag = "replacement-doubleicon-tag",
    inAnimType = 1,
    outAnimType = 1,
    widgetScale = 0,
    layoutDirection = 2,
    modelSceneLayer = 1,
    scriptedAnimationEffectID = 947.5,
}"#;

pub(super) fn supplied() -> DoubleIconAndTextRecord {
    DoubleIconAndTextRecord {
        shown_state: 1,
        label: "Double label 731".into(),
        left_text: "Left text 731".into(),
        left_tooltip: "Left tooltip 731".into(),
        right_text: "Right text 731".into(),
        right_tooltip: "Right tooltip 731".into(),
        tooltip_loc: 3,
        widget_size_setting: 180.5,
        texture_kit: "".into(),
        frame_texture_kit: "".into(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "doubleicon-contract-731".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 1,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 731.25,
    }
}

// Projection-only replacement: do not activate assets, timers, animations or model effects.
pub(super) fn replacement() -> DoubleIconAndTextRecord {
    DoubleIconAndTextRecord {
        shown_state: 0,
        label: "Replacement label 947".into(),
        left_text: "Replacement left 947".into(),
        left_tooltip: "Replacement left tooltip 947".into(),
        right_text: "Replacement right 947".into(),
        right_tooltip: "Replacement right tooltip 947".into(),
        tooltip_loc: 5,
        widget_size_setting: 206.25,
        texture_kit: "replacement-foreground".into(),
        frame_texture_kit: "replacement-background".into(),
        has_timer: true,
        order_index: -12.25,
        widget_tag: "replacement-doubleicon-tag".into(),
        in_anim_type: 1,
        out_anim_type: 1,
        widget_scale: 0,
        layout_direction: 2,
        model_scene_layer: 1,
        scripted_animation_effect_id: 947.5,
    }
}

pub(super) fn entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: TYPE,
        unit_token: None,
    }
}

pub(super) fn seed(env: &WowLuaEnv, id: f64, record: DoubleIconAndTextRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .double_icon_and_text
        .insert(WidgetId::from_number(id), record);
}

pub(super) fn supply_empty_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0,
            vertical_padding: 7.0,
            widgets: Vec::new(),
        }),
    )
    .expect("supply normal empty widget set");
}

pub(super) fn supply(env: &WowLuaEnv, record: DoubleIconAndTextRecord) {
    env.update_ui_widget(
        SET_ID,
        entry(),
        WidgetVisualization::DoubleIconAndText(record),
    )
    .expect("normal typed feed publishes type5 state before event");
}

pub(super) fn updated() -> DoubleIconAndTextRecord {
    let mut record = supplied();
    record.label = "Updated label 947".into();
    record.left_text = "Updated left 947".into();
    record.left_tooltip = "Updated left tooltip 947".into();
    record.right_text = "Updated right 947".into();
    record.right_tooltip = "Updated right tooltip 947".into();
    record.tooltip_loc = 5;
    record.order_index = -12.25;
    record.widget_scale = 0;
    record.widget_tag = "updated-doubleicon-tag".into();
    record
}
