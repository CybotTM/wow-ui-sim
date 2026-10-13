//! Passive complete supplied SpellDisplay records, not native catalog entries.

#[derive(Debug, Clone, PartialEq)]
pub struct WidgetSpellInfoRecord {
    pub spell_id: f64,
    pub shown_state: u32,   // WidgetShownState
    pub enabled_state: u32, // WidgetEnabledState
    pub tooltip: String,
    pub text: String,
    pub stack_display: f64,
    pub icon_size_type: u32,    // WidgetIconSizeType
    pub icon_display_type: u32, // SpellDisplayIconDisplayType
    pub text_shown_state: u32,  // SpellDisplayTextShownStateType
    pub border_color: u32,      // SpellDisplayBorderColor
    pub text_font_type: u32,    // UIWidgetFontType
    pub text_size_type: u32,    // UIWidgetTextSizeType
    pub h_align_type: u32,      // WidgetTextHorizontalAlignmentType
    pub tint: u32,              // SpellDisplayTint
    pub show_glow_state: u32,   // WidgetShowGlowState
    pub show_as_earned: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpellDisplayRecord {
    pub spell_info: WidgetSpellInfoRecord,
    pub tooltip_loc: u32, // UIWidgetTooltipLocation
    pub widget_size_setting: f64,
    pub texture_kit: String,
    pub frame_texture_kit: String,
    pub has_timer: bool,
    pub order_index: f64,
    pub widget_tag: String,
    pub in_anim_type: u32,      // WidgetAnimationType
    pub out_anim_type: u32,     // WidgetAnimationType
    pub widget_scale: u32,      // UIWidgetScale
    pub layout_direction: u32,  // UIWidgetLayoutDirection
    pub model_scene_layer: u32, // UIWidgetModelSceneLayer
    pub scripted_animation_effect_id: f64,
}
