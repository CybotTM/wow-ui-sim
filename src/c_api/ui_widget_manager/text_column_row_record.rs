//! Supplied TextColumnRow contract data; no defaults, catalog, or Lua projection.

/// Literal cached Retail/PTR structure name. All nineteen fields are required.
#[derive(Debug, Clone, PartialEq)]
pub struct TextColumnRowVisualizationInfo {
    pub shown_state: u32,
    pub entries: Vec<TextColumnRowEntryInfo>,
    pub text_size_type: u32,
    pub font_type: u32,
    pub tooltip: String,
    pub tooltip_loc: u32,
    pub bottom_padding: f64,
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

/// Ordered nested record, not a frame or optional placeholder.
#[derive(Debug, Clone, PartialEq)]
pub struct TextColumnRowEntryInfo {
    pub text: String,
    pub enabled_state: u32,
    pub h_align: u32,
    pub column_width: f64,
}
