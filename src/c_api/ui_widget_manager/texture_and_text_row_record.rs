//! Complete supplied TextureAndTextRowVisualizationInfo; no inferred defaults.

/// Ordered row entry: exactly the documented text and tooltip fields.
#[derive(Debug, Clone, PartialEq)]
pub struct TextureAndTextEntryRecord {
    pub text: String,
    pub tooltip: String,
}

/// All eighteen outer fields must be supplied; only fixed_width is nullable.
#[derive(Debug, Clone, PartialEq)]
pub struct TextureAndTextRowRecord {
    pub shown_state: u32,
    pub entries: Vec<TextureAndTextEntryRecord>,
    pub text_size_type: u32,
    pub group_alignment: u32,
    pub fixed_width: Option<f64>,
    pub tooltip_loc: u32,
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
