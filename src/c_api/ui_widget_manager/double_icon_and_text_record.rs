//! Complete supplied DoubleIconAndTextWidgetVisualizationInfo.
//! Grounded in active cached Retail API documentation; no inferred defaults.

/// Explicit host input only. Enum scalars follow the existing u32 convention.
#[derive(Debug, Clone, PartialEq)]
pub struct DoubleIconAndTextRecord {
    pub shown_state: u32,
    pub label: String,
    pub left_text: String,
    pub left_tooltip: String,
    pub right_text: String,
    pub right_tooltip: String,
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
