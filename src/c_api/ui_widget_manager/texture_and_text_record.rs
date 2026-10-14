//! Complete supplied TextureAndTextVisualizationInfo; no inferred defaults.
//! Fields grounded in docs/specs/ui-widget-texture-and-text.md and cached Retail/PTR docs.

/// Complete explicit host record. Enum scalars follow the existing u32 input convention.
#[derive(Debug, Clone, PartialEq)]
pub struct TextureAndTextRecord {
    pub shown_state: u32,                  // WidgetShownState
    pub text: String,                      // string
    pub tooltip: String,                   // string
    pub tooltip_loc: u32,                  // UIWidgetTooltipLocation
    pub text_size_type: u32,               // UIWidgetTextureAndTextSizeType
    pub text_format_type: u32,             // UIWidgetTextFormatType
    pub update_anim_type: u32,             // UIWidgetUpdateAnimType
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
