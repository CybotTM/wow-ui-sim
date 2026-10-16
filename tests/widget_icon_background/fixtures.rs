//! Explicit synthetic records, not native catalog or asset observations.
use wow_ui_sim::c_api::ui_widget_manager::{
    IconTextAndBackgroundRecord, WidgetId, WidgetSetEntry, WidgetSetRecord, WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 762_504.0;
pub(super) const SET_ID: f64 = 439.0;
// Cached Retail/PTR UIWidgetManagerSharedDocumentation.lua: EnumValue = 4.
pub(super) const TYPE: u32 = 4;
pub(super) const GETTER: &str = "GetIconTextAndBackgroundWidgetVisualizationInfo";
pub(super) const EXPECTED: &str = r#"{
    shownState = 1,
    text = "Background text 731",
    widgetSizeSetting = 180.5,
    textureKit = "iconbackground-fixture-731",
    frameTextureKit = "iconbackground-frame-731",
    hasTimer = false,
    orderIndex = 37.5,
    widgetTag = "iconbackground-contract-731",
    inAnimType = 0,
    outAnimType = 0,
    widgetScale = 1,
    layoutDirection = 1,
    modelSceneLayer = 0,
    scriptedAnimationEffectID = 0,
}"#;
pub(super) const REPLACEMENT_EXPECTED: &str = r#"{
    shownState = 0,
    text = "Replacement background 947",
    widgetSizeSetting = 206.25,
    textureKit = "replacement-foreground",
    frameTextureKit = "replacement-background",
    hasTimer = true,
    orderIndex = -12.25,
    widgetTag = "replacement-iconbackground-tag",
    inAnimType = 1,
    outAnimType = 1,
    widgetScale = 0,
    layoutDirection = 2,
    modelSceneLayer = 1,
    scriptedAnimationEffectID = 947.5,
}"#;
pub(super) const UPDATED_EXPECTED: &str = r#"{
    shownState = 1,
    text = "Updated background 947",
    widgetSizeSetting = 180.5,
    textureKit = "iconbackground-fixture-947",
    frameTextureKit = "iconbackground-frame-731",
    hasTimer = false,
    orderIndex = -12.25,
    widgetTag = "updated-iconbackground-tag",
    inAnimType = 0,
    outAnimType = 0,
    widgetScale = 0,
    layoutDirection = 1,
    modelSceneLayer = 0,
    scriptedAnimationEffectID = 0,
}"#;

pub(super) fn supplied() -> IconTextAndBackgroundRecord {
    IconTextAndBackgroundRecord {
        shown_state: 1,
        text: "Background text 731".into(),
        widget_size_setting: 180.5,
        texture_kit: "iconbackground-fixture-731".into(),
        frame_texture_kit: "iconbackground-frame-731".into(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "iconbackground-contract-731".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 1,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}

// Projection only; never activate timers, fades, scene layers or scripted effects.
pub(super) fn replacement() -> IconTextAndBackgroundRecord {
    IconTextAndBackgroundRecord {
        shown_state: 0,
        text: "Replacement background 947".into(),
        widget_size_setting: 206.25,
        texture_kit: "replacement-foreground".into(),
        frame_texture_kit: "replacement-background".into(),
        has_timer: true,
        order_index: -12.25,
        widget_tag: "replacement-iconbackground-tag".into(),
        in_anim_type: 1,
        out_anim_type: 1,
        widget_scale: 0,
        layout_direction: 2,
        model_scene_layer: 1,
        scripted_animation_effect_id: 947.5,
    }
}

pub(super) fn updated() -> IconTextAndBackgroundRecord {
    let mut record = supplied();
    record.text = "Updated background 947".into();
    record.texture_kit = "iconbackground-fixture-947".into();
    record.order_index = -12.25;
    record.widget_scale = 0;
    record.widget_tag = "updated-iconbackground-tag".into();
    record
}

pub(super) fn entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: TYPE,
        unit_token: None,
    }
}

pub(super) fn seed(env: &WowLuaEnv, id: f64, record: IconTextAndBackgroundRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .icon_text_and_background
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

pub(super) fn supply(env: &WowLuaEnv, record: IconTextAndBackgroundRecord) {
    env.update_ui_widget(
        SET_ID,
        entry(),
        WidgetVisualization::IconTextAndBackground(record),
    )
    .expect("normal typed feed publishes type4 state before event");
}
