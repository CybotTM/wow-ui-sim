//! Supplied widget sets, visualizations, and synchronous host update feeds.
use std::collections::HashMap;

mod feed;
mod fixtures;
mod inputs;
pub use feed::WidgetVisualization;
pub(crate) mod queries;
pub(crate) use fixtures::seed_demo_world_quest_widgets;
pub use inputs::{IconAndTextRecord, UiWidgetInputs, WidgetSetEntry, WidgetSetRecord};

#[cfg(any(feature = "client-retail", feature = "client-ptr"))]
pub(crate) mod button_header;

/// Modeled numeric key, not a native widget-ID range or availability claim.
/// Preserve f64 bits except that equal numeric signed zeroes share one key.
/// No integer cast, positivity rule, or invented native rejection policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    pub fn from_number(number: f64) -> Self {
        let bits = if number == 0.0 {
            0.0_f64.to_bits()
        } else {
            number.to_bits()
        };
        Self(bits)
    }
}

/// Whole record must be supplied. No Default DTO or automatic field filler.
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonHeaderRecord {
    pub shown_state: u32,
    pub header_text: String,
    pub tooltip: String,
    pub tooltip_loc: u32,
    pub buttons: Vec<WidgetSpellButtonRecord>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct WidgetSpellButtonRecord {
    pub spell_id: f64,
    pub tooltip: String,
    pub icon: u32,
    pub enabled_state: u32,
    pub cooldown_type: u32,
}

/// Default is an empty supplied-record map, never a default record.
#[derive(Debug, Default)]
pub struct ButtonHeaderInputs {
    pub widgets: HashMap<WidgetId, ButtonHeaderRecord>,
}
