//! Complete supplied ItemDisplay records, not native catalog entries.

/// Complete ItemDisplayVisualizationInfo; every outer field is nonnullable.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemDisplayRecord {
    pub shown_state: u32,
    pub tooltip_loc: u32,
    pub item_info: UiWidgetItemRecord,
    pub widget_size_setting: f64,
    pub texture_kit: String,
    pub frame_texture_kit: String,
    pub has_timer: bool,
    pub order_index: f64,
    pub widget_tag: String,
    pub in_anim_type: u32,
    pub out_anim_type: u32,
    pub widget_scale: u32,
    pub layout_direction: u32,
    pub model_scene_layer: u32,
    pub scripted_animation_effect_id: f64,
}

/// Complete UIWidgetItemInfo; only the four documented nullable fields use Option.
#[derive(Debug, Clone, PartialEq)]
pub struct UiWidgetItemRecord {
    pub item_id: f64,
    pub stack_count: Option<f64>,
    pub override_item_name: Option<String>,
    pub info_text: Option<String>,
    pub override_tooltip: Option<String>,
    pub text_display_style: u32,
    pub tooltip_enabled: bool,
    pub icon_size_type: u32,
    pub info_text_enabled_state: u32,
    pub show_as_earned: bool,
    pub item_name_text_font_type: u32,
    pub item_name_text_size_type: u32,
    pub info_text_font_type: u32,
    pub info_text_size_type: u32,
    pub item_name_custom_color: u32,
    pub item_name_custom_color_override_state: u32,
}
