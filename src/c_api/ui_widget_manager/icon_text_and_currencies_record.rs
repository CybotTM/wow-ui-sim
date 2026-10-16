//! Complete supplied IconTextAndCurrenciesWidgetVisualizationInfo.
//! Grounded in cached Retail/PTR API documentation; no inferred defaults.

/// Explicit host input only. Enum scalars follow the existing u32 convention.
#[derive(Debug, Clone, PartialEq)]
pub struct IconTextAndCurrenciesRecord {
    pub shown_state: u32,
    pub enabled_state: u32,
    pub description_shown_state: u32,
    pub description_enabled_state: u32,
    pub text: String,
    pub description: String,
    pub currencies: Vec<UiWidgetCurrencyRecord>,
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

/// Ordered complete UIWidgetCurrencyInfo row, not a currency catalog entry.
#[derive(Debug, Clone, PartialEq)]
pub struct UiWidgetCurrencyRecord {
    pub icon_file_id: u32,
    pub leading_text: String,
    pub text: String,
    pub tooltip: String,
    pub is_currency_maxed: bool,
    pub text_font_type: u32,
    pub text_size_type: u32,
    pub text_enabled_state: u32,
    pub icon_size_type: u32,
    pub update_anim_type: u32,
}
