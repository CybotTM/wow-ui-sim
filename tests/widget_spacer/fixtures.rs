//! Explicit synthetic records, not native observations or catalog defaults.
use wow_ui_sim::c_api::ui_widget_manager::{
    SpacerRecord, WidgetId, WidgetSetEntry, WidgetSetRecord, WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 762_201.0;
pub(super) const SET_ID: f64 = 422.0;
pub(super) const TYPE: u32 = 22;
pub(super) const GETTER: &str = "GetSpacerVisualizationInfo";
pub(super) const EXPECTED: &str = r#"{
    shownState = 1,
    widgetWidth = 173.25,
    widgetHeight = 41.5,
    widgetSizeSetting = 180.5,
    textureKit = "",
    frameTextureKit = "",
    hasTimer = false,
    orderIndex = 37.5,
    widgetTag = "spacer-contract-731",
    inAnimType = 0,
    outAnimType = 0,
    widgetScale = 1,
    layoutDirection = 1,
    modelSceneLayer = 0,
    scriptedAnimationEffectID = 731.25,
}"#;
pub(super) const REPLACEMENT_EXPECTED: &str = r#"{
    shownState = 0,
    widgetWidth = -11.5,
    widgetHeight = 0.0,
    widgetSizeSetting = 206.25,
    textureKit = "replacement-foreground",
    frameTextureKit = "replacement-background",
    hasTimer = true,
    orderIndex = -12.25,
    widgetTag = "replacement-spacer-tag",
    inAnimType = 1,
    outAnimType = 1,
    widgetScale = 0,
    layoutDirection = 2,
    modelSceneLayer = 1,
    scriptedAnimationEffectID = 947.5,
}"#;

/// Complete supplied record; all-field replacement is query/feed-only.
pub(super) fn supplied() -> SpacerRecord {
    SpacerRecord {
        shown_state: 1,
        widget_width: 173.25,
        widget_height: 41.5,
        widget_size_setting: 180.5,
        texture_kit: "".into(),
        frame_texture_kit: "".into(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "spacer-contract-731".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 1,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 731.25,
    }
}

/// Complete replacement record; all-field replacement is query/feed-only.
pub(super) fn replacement() -> SpacerRecord {
    SpacerRecord {
        shown_state: 0,
        widget_width: -11.5,
        widget_height: 0.0,
        widget_size_setting: 206.25,
        texture_kit: "replacement-foreground".into(),
        frame_texture_kit: "replacement-background".into(),
        has_timer: true,
        order_index: -12.25,
        widget_tag: "replacement-spacer-tag".into(),
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

pub(super) fn inject(env: &WowLuaEnv, id: f64, record: SpacerRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .spacers
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

pub(super) fn supply(env: &WowLuaEnv, record: SpacerRecord) {
    env.update_ui_widget(SET_ID, entry(), WidgetVisualization::Spacer(record))
        .expect("normal typed type22 feed publishes state before event");
}

/// Consumer keeps animations/effects/timers disabled and both kits empty.
pub(super) fn updated() -> SpacerRecord {
    let mut record = supplied();
    record.widget_width = 206.25;
    record.widget_height = 73.75;
    record.widget_tag = "updated-spacer-tag".into();
    record
}
