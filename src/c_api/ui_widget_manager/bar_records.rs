//! Complete supplied bar visualization records; no native catalog/default inference.
/// Complete supplied StatusBarWidgetVisualizationInfo. Every cached field is nonnil.
/// Enum scalars use the existing u32 input convention; no enum validity inference.
#[derive(Debug, Clone, PartialEq)]
pub struct StatusBarRecord {
    pub shown_state: u32,                  // WidgetShownState
    pub bar_min: f64,                      // number
    pub bar_max: f64,                      // number
    pub bar_value: f64,                    // number
    pub text: String,                      // string
    pub tooltip: String,                   // string
    pub bar_value_text_type: u32,          // StatusBarValueTextType
    pub override_bar_text: String,         // string
    pub override_bar_text_shown_type: u32, // StatusBarOverrideBarTextShownType
    pub color_tint: u32,                   // StatusBarColorTintValue (enum, not an RGBA structure)
    pub partition_values: Vec<f64>,        // table, InnerType = number; ordered numeric array
    pub tooltip_loc: u32,                  // UIWidgetTooltipLocation
    pub fill_motion_type: u32,             // UIWidgetMotionType
    pub bar_text_enabled_state: u32,       // WidgetEnabledState
    pub bar_text_font_type: u32,           // UIWidgetFontType
    pub bar_text_size_type: u32,           // UIWidgetTextSizeType
    pub text_enabled_state: u32,           // WidgetEnabledState
    pub text_font_type: u32,               // UIWidgetFontType
    pub text_size_type: u32,               // UIWidgetTextSizeType
    pub glow_anim_type: u32,               // WidgetGlowAnimType
    pub show_glow_state: u32,              // WidgetShowGlowState
    pub fill_min_opacity: u32,             // WidgetOpacityType (enum, not a fractional number)
    pub fill_max_opacity: u32,             // WidgetOpacityType
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

/// Complete supplied CaptureBarWidgetVisualizationInfo. Every cached field is nonnil.
#[derive(Debug, Clone, PartialEq)]
pub struct CaptureBarRecord {
    pub shown_state: u32,                  // WidgetShownState
    pub bar_value: f64,                    // number
    pub bar_min_value: f64,                // number (not StatusBar's barMin)
    pub bar_max_value: f64,                // number (not StatusBar's barMax)
    pub neutral_zone_size: f64,            // number
    pub neutral_zone_center: f64,          // number
    pub tooltip: String,                   // string
    pub glow_anim_type: u32,               // WidgetGlowAnimType
    pub fill_direction_type: u32,          // CaptureBarWidgetFillDirectionType
    pub tooltip_loc: u32,                  // UIWidgetTooltipLocation
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
