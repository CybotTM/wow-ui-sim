//! Environment-owned widget sets and visualization inputs. No native feed inference.
use std::collections::HashMap;

use super::WidgetId;

#[derive(Debug, Clone, PartialEq)]
pub struct WidgetSetEntry {
    pub widget_id: f64,
    pub widget_type: u32,
    pub unit_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WidgetSetRecord {
    pub layout_direction: u32,
    pub vertical_padding: f64,
    pub widgets: Vec<WidgetSetEntry>,
}

/// Complete supplied record; field values are never inferred from widget IDs.
#[derive(Debug, Clone, PartialEq)]
pub struct IconAndTextRecord {
    pub state: u32,
    pub text: String,
    pub tooltip: String,
    pub dynamic_tooltip: String,
    pub tooltip_loc: u32,
    pub shift_text_type: u32,
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

#[derive(Debug, Default)]
pub struct UiWidgetInputs {
    pub sets: HashMap<WidgetId, WidgetSetRecord>,
    pub icon_and_text: HashMap<WidgetId, IconAndTextRecord>,
    pub quest_sets: HashMap<(i32, i32), f64>,
}
