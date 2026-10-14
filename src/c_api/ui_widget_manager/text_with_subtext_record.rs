//! Complete supplied TextWithSubtextWidgetVisualizationInfo; no inferred defaults.
//! Cached Retail/PTR API documentation declares all 27 fields non-nil.

/// Whole explicit host record. Enum scalars follow the existing u32 input convention.
#[derive(Debug, Clone, PartialEq)]
pub struct TextWithSubtextRecord {
    pub shown_state: u32,
    pub enabled_state: u32,
    pub text: String,
    pub widget_width: f64,
    pub tooltip: String,
    pub text_size_type: u32,
    pub font_type: u32,
    pub tooltip_loc: u32,
    pub h_align: u32,
    pub sub_text: String,
    pub sub_text_size_type: u32,
    pub sub_text_font_type: u32,
    pub sub_text_h_align: u32,
    pub sub_text_enabled_state: u32,
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
    pub spacing: f64,
}
