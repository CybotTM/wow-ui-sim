//! Explicit synthetic host records, not catalog defaults or native observations.
use wow_ui_sim::c_api::ui_widget_manager::{
    TextWithSubtextRecord, WidgetId, WidgetSetEntry, WidgetSetRecord, WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 762_501.0;
pub(super) const SET_ID: f64 = 435.0;
pub(super) const TYPE: u32 = 25;
pub(super) const GETTER: &str = "GetTextWithSubtextWidgetVisualizationInfo";
pub(super) const EXPECTED: &str = r#"{
    shownState = 1, enabledState = 3, text = 'Subtext heading 731', widgetWidth = 913.25,
    tooltip = 'Subtext tooltip 731', textSizeType = 1, fontType = 0, tooltipLoc = 3,
    hAlign = 0, subText = 'Secondary line 731', subTextSizeType = 4,
    subTextFontType = 1, subTextHAlign = 2, subTextEnabledState = 4,
    widgetSizeSetting = 180.5, textureKit = '', frameTextureKit = '', hasTimer = false,
    orderIndex = 37.5, widgetTag = 'subtext-contract-731', inAnimType = 0, outAnimType = 0,
    widgetScale = 1, layoutDirection = 1, modelSceneLayer = 0,
    scriptedAnimationEffectID = 731.25, spacing = 3.5,
}"#;
pub(super) const REPLACEMENT_EXPECTED: &str = r#"{
    shownState = 0, enabledState = 4, text = 'Replacement heading 947', widgetWidth = -11.5,
    tooltip = 'Replacement tooltip 947', textSizeType = 4, fontType = 1, tooltipLoc = 5,
    hAlign = 2, subText = 'Replacement secondary 947', subTextSizeType = 1,
    subTextFontType = 0, subTextHAlign = 1, subTextEnabledState = 3,
    widgetSizeSetting = 206.25, textureKit = 'replacement-foreground',
    frameTextureKit = 'replacement-background', hasTimer = true, orderIndex = -12.25,
    widgetTag = 'replacement-subtext-tag', inAnimType = 1, outAnimType = 2,
    widgetScale = 0, layoutDirection = 2, modelSceneLayer = 1,
    scriptedAnimationEffectID = 947.5, spacing = 7.25,
}"#;

// Enum values grounded in both retail/AddOns and ptr/AddOns cached documentation.
pub(super) fn supplied() -> TextWithSubtextRecord {
    TextWithSubtextRecord {
        shown_state: 1,
        enabled_state: 3,
        text: "Subtext heading 731".into(),
        widget_width: 913.25,
        tooltip: "Subtext tooltip 731".into(),
        text_size_type: 1,
        font_type: 0,
        tooltip_loc: 3,
        h_align: 0,
        sub_text: "Secondary line 731".into(),
        sub_text_size_type: 4,
        sub_text_font_type: 1,
        sub_text_h_align: 2,
        sub_text_enabled_state: 4,
        widget_size_setting: 180.5,
        texture_kit: String::new(),
        frame_texture_kit: String::new(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "subtext-contract-731".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 1,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 731.25,
        spacing: 3.5,
    }
}

// All 27 fields differ; projection only, not an animation/asset activation fixture.
pub(super) fn replacement() -> TextWithSubtextRecord {
    TextWithSubtextRecord {
        shown_state: 0,
        enabled_state: 4,
        text: "Replacement heading 947".into(),
        widget_width: -11.5,
        tooltip: "Replacement tooltip 947".into(),
        text_size_type: 4,
        font_type: 1,
        tooltip_loc: 5,
        h_align: 2,
        sub_text: "Replacement secondary 947".into(),
        sub_text_size_type: 1,
        sub_text_font_type: 0,
        sub_text_h_align: 1,
        sub_text_enabled_state: 3,
        widget_size_setting: 206.25,
        texture_kit: "replacement-foreground".into(),
        frame_texture_kit: "replacement-background".into(),
        has_timer: true,
        order_index: -12.25,
        widget_tag: "replacement-subtext-tag".into(),
        in_anim_type: 1,
        out_anim_type: 2,
        widget_scale: 0,
        layout_direction: 2,
        model_scene_layer: 1,
        scripted_animation_effect_id: 947.5,
        spacing: 7.25,
    }
}

pub(super) fn entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: TYPE,
        unit_token: None,
    }
}

pub(super) fn seed(env: &WowLuaEnv, id: f64, record: TextWithSubtextRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .text_with_subtext
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
    .expect("supply existing normal empty widget set");
}

pub(super) fn supply(env: &WowLuaEnv, record: TextWithSubtextRecord) {
    env.update_ui_widget(
        SET_ID,
        entry(),
        WidgetVisualization::TextWithSubtext(record),
    )
    .expect("normal typed feed publishes type25 state before event");
}

pub(super) fn updated() -> TextWithSubtextRecord {
    let mut record = supplied();
    record.text = "Updated heading 947".into();
    record.sub_text = "Updated secondary 947".into();
    record.tooltip = "Updated tooltip 947".into();
    record.widget_size_setting = 206.25;
    record.spacing = 7.25;
    record.text_size_type = 4;
    record.font_type = 1;
    record.enabled_state = 4;
    record.h_align = 2;
    record.sub_text_size_type = 1;
    record.sub_text_font_type = 0;
    record.sub_text_enabled_state = 3;
    record.sub_text_h_align = 1;
    record.tooltip_loc = 5;
    record.order_index = -12.25;
    record.widget_scale = 0;
    record.widget_tag = "updated-subtext-tag".into();
    record
}
