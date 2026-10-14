//! Complete supplied DoubleStatusBarWidgetVisualizationInfo; no inferred defaults.
//! Fields grounded in docs/specs/ui-widget-double-status-bar.md and cached Retail/PTR docs.

/// Complete explicit host record. Enum scalars follow the existing u32 input convention.
#[derive(Debug, Clone, PartialEq)]
pub struct DoubleStatusBarRecord {
    pub shown_state: u32,                  // WidgetShownState
    pub left_bar_min: f64,                 // number
    pub left_bar_max: f64,                 // number
    pub left_bar_value: f64,               // number
    pub left_bar_tooltip: String,          // string
    pub right_bar_min: f64,                // number
    pub right_bar_max: f64,                // number
    pub right_bar_value: f64,              // number
    pub right_bar_tooltip: String,         // string
    pub bar_value_text_type: u32,          // StatusBarValueTextType
    pub text: String,                      // string
    pub left_bar_tooltip_loc: u32,         // UIWidgetTooltipLocation
    pub right_bar_tooltip_loc: u32,        // UIWidgetTooltipLocation
    pub fill_motion_type: u32,             // UIWidgetMotionType
    pub widget_size_setting: f64,          // number
    pub texture_kit: String,               // textureKit
    pub frame_texture_kit: String,         // textureKit
    pub has_timer: bool,                   // bool
    pub order_index: f64,                  // number
    pub widget_tag: String,                // string
    pub in_anim_type: u32,                 // WidgetAnimationType
    pub out_anim_type: u32,                // WidgetAnimationType
    pub widget_scale: u32,                 // UIWidgetScale
    pub layout_direction: u32,             // UIWidgetLayoutDirection
    pub model_scene_layer: u32,            // UIWidgetModelSceneLayer
    pub scripted_animation_effect_id: f64, // number
}
