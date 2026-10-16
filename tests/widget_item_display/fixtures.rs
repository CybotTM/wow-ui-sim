//! Explicit supplied records and independent expected snapshots.
use super::helpers::{assert_lua, install_snapshot_assertion};
use wow_ui_sim::c_api::ui_widget_manager::{
    ItemDisplayRecord, UiWidgetItemRecord, WidgetId, WidgetSetEntry, WidgetSetRecord,
    WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 934_227.0;
pub(super) const SET_ID: f64 = 467.0;
pub(super) const GETTER: &str = "GetItemDisplayVisualizationInfo";

pub(super) fn supplied() -> ItemDisplayRecord {
    ItemDisplayRecord {
        shown_state: 1,
        tooltip_loc: 3,
        item_info: UiWidgetItemRecord {
            item_id: 6948.0,
            stack_count: Some(17.0),
            override_item_name: Some("Supplied hearth 731".into()),
            info_text: Some("Info 731".into()),
            override_tooltip: Some("Tooltip 731".into()),
            text_display_style: 1,
            tooltip_enabled: true,
            icon_size_type: 0,
            info_text_enabled_state: 4,
            show_as_earned: false,
            item_name_text_font_type: 0,
            item_name_text_size_type: 0,
            info_text_font_type: 0,
            info_text_size_type: 0,
            item_name_custom_color: 3,
            item_name_custom_color_override_state: 0,
        },
        widget_size_setting: 180.5,
        texture_kit: "".into(),
        frame_texture_kit: "".into(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "item-contract-731".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 0,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}

// Projection/feed only. Never activate these timer/animation/effect inputs in cached UI.
pub(super) fn replacement() -> ItemDisplayRecord {
    ItemDisplayRecord {
        shown_state: 0,
        tooltip_loc: 6,
        item_info: UiWidgetItemRecord {
            item_id: 19019.5,
            stack_count: Some(0.5),
            override_item_name: Some("Replaced item 947".into()),
            info_text: Some("Replaced info 947".into()),
            override_tooltip: Some("Replaced tooltip 947".into()),
            text_display_style: 2,
            tooltip_enabled: false,
            icon_size_type: 2,
            info_text_enabled_state: 2,
            show_as_earned: true,
            item_name_text_font_type: 1,
            item_name_text_size_type: 4,
            info_text_font_type: 2,
            info_text_size_type: 3,
            item_name_custom_color: 4,
            item_name_custom_color_override_state: 1,
        },
        widget_size_setting: 206.25,
        texture_kit: "projection-item-947".into(),
        frame_texture_kit: "projection-frame-947".into(),
        has_timer: true,
        order_index: -12.25,
        widget_tag: "item-contract-947".into(),
        in_anim_type: 1,
        out_anim_type: 1,
        widget_scale: 1,
        layout_direction: 2,
        model_scene_layer: 1,
        scripted_animation_effect_id: 947.5,
    }
}

pub(super) fn updated() -> ItemDisplayRecord {
    let mut record = supplied();
    record.item_info.override_item_name = Some("Updated hearth 947".into());
    record.item_info.info_text = Some("Updated info 947".into());
    record.item_info.override_tooltip = Some("Updated tooltip 947".into());
    record.item_info.stack_count = Some(23.0);
    record.item_info.icon_size_type = 2;
    record.item_info.show_as_earned = true;
    record.order_index = -12.25;
    record.widget_tag = "updated-item-tag".into();
    record
}

pub(super) fn no_optionals() -> ItemDisplayRecord {
    let mut record = supplied();
    record.item_info.stack_count = None;
    record.item_info.override_item_name = None;
    record.item_info.info_text = None;
    record.item_info.override_tooltip = None;
    record
}

pub(super) fn entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: 27,
        unit_token: None,
    }
}

pub(super) fn seed(env: &WowLuaEnv, id: f64, record: ItemDisplayRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .item_display
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
    .expect("normal empty set feed");
}

pub(super) fn supply(env: &WowLuaEnv, record: ItemDisplayRecord) {
    env.update_ui_widget(SET_ID, entry(), WidgetVisualization::ItemDisplay(record))
        .expect("normal type27 feed");
}

pub(super) fn install_assertions(env: &WowLuaEnv) {
    install_snapshot_assertion(env, GETTER, "{}");
    assert_lua(
        env,
        "independent complete nested expectations",
        include_str!("snapshots.lua"),
    );
}
