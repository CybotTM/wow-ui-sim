//! Per-widget-type method allow-list derived from WoW discovery data.
//!
//! GLOBAL_METHODS contains methods present on all widget types.
//! Per-type sets contain only type-specific additions.

mod controls;
mod edit;
mod frame;
pub(crate) mod global;
mod leaf;
mod minimap;
mod misc;
mod model;
mod scroll;
mod tooltip;

use controls::{COOLDOWN_METHODS, SLIDER_METHODS, STATUSBAR_METHODS};
use edit::EDITBOX_METHODS;
use frame::{BUTTON_METHODS, CHECKBUTTON_METHODS, FRAME_METHODS};
use leaf::{FONTSTRING_METHODS, TEXTURE_METHODS};
use minimap::MINIMAP_METHODS;
use misc::{MESSAGEFRAME_METHODS, SIMPLEHTML_METHODS};
use model::{MODEL_METHODS, PLAYERMODEL_METHODS};
use scroll::SCROLLFRAME_METHODS;
use tooltip::{COLORSELECT_METHODS, GAMETOOLTIP_METHODS};

use crate::widget::WidgetType;
use std::collections::HashSet;
use std::sync::LazyLock;

pub static ALL_METHODS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    global::GLOBAL_METHODS
        .iter()
        .chain(BUTTON_METHODS.iter())
        .chain(CHECKBUTTON_METHODS.iter())
        .chain(COOLDOWN_METHODS.iter())
        .chain(COLORSELECT_METHODS.iter())
        .chain(EDITBOX_METHODS.iter())
        .chain(FONTSTRING_METHODS.iter())
        .chain(FRAME_METHODS.iter())
        .chain(GAMETOOLTIP_METHODS.iter())
        .chain(MESSAGEFRAME_METHODS.iter())
        .chain(MINIMAP_METHODS.iter())
        .chain(MODEL_METHODS.iter())
        .chain(PLAYERMODEL_METHODS.iter())
        .chain(SCROLLFRAME_METHODS.iter())
        .chain(SIMPLEHTML_METHODS.iter())
        .chain(SLIDER_METHODS.iter())
        .chain(STATUSBAR_METHODS.iter())
        .chain(TEXTURE_METHODS.iter())
        .copied()
        .collect()
});

static HIDDEN_SHARED_METHODS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "GetStatusBarDesaturated",
        "GetTitle",
        "SetStatusBarAtlas",
        "SetTitle",
    ]
    .into_iter()
    .collect()
});

pub fn methods_for_type(widget_type: WidgetType) -> &'static HashSet<&'static str> {
    match widget_type {
        WidgetType::Frame | WidgetType::WorldFrame => &FRAME_METHODS,
        WidgetType::Line => &TEXTURE_METHODS,
        WidgetType::ModelScene => &MODEL_METHODS,
        WidgetType::Button => &BUTTON_METHODS,
        WidgetType::CheckButton => &CHECKBUTTON_METHODS,
        WidgetType::Texture => &TEXTURE_METHODS,
        WidgetType::FontString => &FONTSTRING_METHODS,
        WidgetType::EditBox => &EDITBOX_METHODS,
        WidgetType::ScrollFrame => &SCROLLFRAME_METHODS,
        WidgetType::Slider => &SLIDER_METHODS,
        WidgetType::StatusBar => &STATUSBAR_METHODS,
        WidgetType::Cooldown => &COOLDOWN_METHODS,
        WidgetType::Model => &MODEL_METHODS,
        WidgetType::PlayerModel => &PLAYERMODEL_METHODS,
        WidgetType::SimpleHTML => &SIMPLEHTML_METHODS,
        WidgetType::MessageFrame => &MESSAGEFRAME_METHODS,
        WidgetType::GameTooltip => &GAMETOOLTIP_METHODS,
        WidgetType::ColorSelect => &COLORSELECT_METHODS,
        WidgetType::Minimap => &MINIMAP_METHODS,
    }
}

pub fn is_registered_method(name: &str) -> bool {
    ALL_METHODS.contains(name) || HIDDEN_SHARED_METHODS.contains(name)
}

pub fn is_method_allowed(widget_type: WidgetType, name: &str) -> bool {
    global::GLOBAL_METHODS.contains(name) || methods_for_type(widget_type).contains(name)
}
