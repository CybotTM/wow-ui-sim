//! Per-widget-type method allow-list derived from WoW discovery data.
//!
//! Methods discovered on WoW frame metatables are organized by widget type.
//! Methods NOT in any type's list are Mixin/sim-specific and pass through.

mod controls;
mod edit;
mod frame;
mod leaf;
mod misc;
mod model;
mod scroll;
mod tooltip;

use controls::{COOLDOWN_METHODS, SLIDER_METHODS, STATUSBAR_METHODS};
use edit::EDITBOX_METHODS;
use frame::{BUTTON_METHODS, CHECKBUTTON_METHODS, FRAME_METHODS};
use leaf::{FONTSTRING_METHODS, TEXTURE_METHODS};
use misc::{MESSAGEFRAME_METHODS, SIMPLEHTML_METHODS};
use model::{MODEL_METHODS, PLAYERMODEL_METHODS};
use scroll::SCROLLFRAME_METHODS;
use tooltip::{COLORSELECT_METHODS, GAMETOOLTIP_METHODS};

use crate::widget::WidgetType;
use std::collections::HashSet;
use std::sync::LazyLock;

/// Check if a method should be allowed for this widget type.
///
/// Returns true if the method should be accessible:
/// - The method is NOT in any type's metatable list (Mixin/sim-specific → always pass through), OR
/// - The method IS in this type's metatable list.
pub fn is_method_allowed(widget_type: WidgetType, method: &str) -> bool {
    if !ALL_KNOWN_METHODS.contains(method) {
        return true;
    }
    let list = methods_for_type(widget_type);
    list.binary_search(&method).is_ok()
}

fn methods_for_type(widget_type: WidgetType) -> &'static [&'static str] {
    match widget_type {
        WidgetType::Frame | WidgetType::WorldFrame | WidgetType::Line => FRAME_METHODS,
        WidgetType::ModelScene => MODEL_METHODS,
        WidgetType::Button => BUTTON_METHODS,
        WidgetType::CheckButton => CHECKBUTTON_METHODS,
        WidgetType::Texture => TEXTURE_METHODS,
        WidgetType::FontString => FONTSTRING_METHODS,
        WidgetType::EditBox => EDITBOX_METHODS,
        WidgetType::ScrollFrame => SCROLLFRAME_METHODS,
        WidgetType::Slider => SLIDER_METHODS,
        WidgetType::StatusBar => STATUSBAR_METHODS,
        WidgetType::Cooldown => COOLDOWN_METHODS,
        WidgetType::Model => MODEL_METHODS,
        WidgetType::PlayerModel => PLAYERMODEL_METHODS,
        WidgetType::SimpleHTML => SIMPLEHTML_METHODS,
        WidgetType::MessageFrame => MESSAGEFRAME_METHODS,
        WidgetType::GameTooltip => GAMETOOLTIP_METHODS,
        WidgetType::ColorSelect => COLORSELECT_METHODS,
        WidgetType::Minimap => FRAME_METHODS,
    }
}

/// Union of all per-type method lists. Methods outside this set are Mixin/sim-specific.
static ALL_KNOWN_METHODS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    for list in [
        FRAME_METHODS,
        BUTTON_METHODS,
        CHECKBUTTON_METHODS,
        TEXTURE_METHODS,
        FONTSTRING_METHODS,
        EDITBOX_METHODS,
        SCROLLFRAME_METHODS,
        SLIDER_METHODS,
        STATUSBAR_METHODS,
        COOLDOWN_METHODS,
        MODEL_METHODS,
        PLAYERMODEL_METHODS,
        SIMPLEHTML_METHODS,
        MESSAGEFRAME_METHODS,
        GAMETOOLTIP_METHODS,
        COLORSELECT_METHODS,
    ] {
        for &m in list {
            set.insert(m);
        }
    }
    set
});
