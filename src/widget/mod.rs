//! Widget system implementing WoW's UI object hierarchy.

mod anchor;
mod frame;
mod frame_enums;
mod registry;

pub use crate::atlas::NineSliceAtlasInfo;
pub use anchor::{Anchor, AnchorPoint};
pub use frame::{
    AttributeValue, Backdrop, Color, Frame, Gradient, LineAnchor, TextJustify, TextOutline,
};
pub use frame_enums::{DrawLayer, FrameStrata};
pub use registry::{RenderDirtyBatch, RenderDirtySource, WidgetRegistry};

use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_WIDGET_ID: AtomicU64 = AtomicU64::new(1);

/// Generate a unique widget ID.
pub fn next_widget_id() -> u64 {
    NEXT_WIDGET_ID.fetch_add(1, Ordering::Relaxed)
}

/// Widget types supported by the simulator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetType {
    Frame,
    Button,
    FontString,
    Texture,
    Line,
    EditBox,
    ScrollFrame,
    Slider,
    CheckButton,
    StatusBar,
    Cooldown,
    Model,
    ModelScene,
    PlayerModel,
    ColorSelect,
    MessageFrame,
    SimpleHTML,
    GameTooltip,
    Minimap,
    WorldFrame,
}

impl WidgetType {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        // WoW Lua uses both PascalCase ("Button") and ALLCAPS ("BUTTON")
        // for frame type names, so match case-insensitively.
        let lower = s.to_ascii_lowercase();
        match lower.as_str() {
            "frame" => Some(Self::Frame),
            "button" | "dropdownbutton" | "itembutton" | "containedalertframe" => {
                Some(Self::Button)
            }
            "fontstring" => Some(Self::FontString),
            "texture" => Some(Self::Texture),
            "line" => Some(Self::Line),
            "editbox" => Some(Self::EditBox),
            "scrollframe" => Some(Self::ScrollFrame),
            "slider" => Some(Self::Slider),
            "checkbutton" => Some(Self::CheckButton),
            "statusbar" => Some(Self::StatusBar),
            "cooldown" => Some(Self::Cooldown),
            "model" => Some(Self::Model),
            "modelscene" => Some(Self::ModelScene),
            "playermodel" | "cinematicmodel" | "tabardmodel" | "dressupmodel" => {
                Some(Self::PlayerModel)
            }
            "colorselect" => Some(Self::ColorSelect),
            "messageframe" | "scrollingmessageframe" => Some(Self::MessageFrame),
            "simplehtml" => Some(Self::SimpleHTML),
            "gametooltip" => Some(Self::GameTooltip),
            "minimap" => Some(Self::Minimap),
            // EventFrame is a Frame subtype for event-only handling
            "eventframe" => Some(Self::Frame),
            // Checkout is a special frame type for the in-game shop
            "checkout" => Some(Self::Frame),
            // Specialty frame types — no custom behavior needed, treat as plain Frame
            "archaeologydigsiteframe"
            | "browser"
            | "fogofwarframe"
            | "movieframe"
            | "offscreenframe"
            | "questpoiframe"
            | "scenariopoiframe"
            | "unitpositionframe" => Some(Self::Frame),
            // WorldFrame is internal only — CreateFrame("WorldFrame") should error
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Frame => "Frame",
            Self::Button => "Button",
            Self::FontString => "FontString",
            Self::Texture => "Texture",
            Self::Line => "Line",
            Self::EditBox => "EditBox",
            Self::ScrollFrame => "ScrollFrame",
            Self::Slider => "Slider",
            Self::CheckButton => "CheckButton",
            Self::StatusBar => "StatusBar",
            Self::Cooldown => "Cooldown",
            Self::Model => "Model",
            Self::ModelScene => "ModelScene",
            Self::PlayerModel => "PlayerModel",
            Self::ColorSelect => "ColorSelect",
            Self::MessageFrame => "MessageFrame",
            Self::SimpleHTML => "SimpleHTML",
            Self::GameTooltip => "GameTooltip",
            Self::Minimap => "Minimap",
            Self::WorldFrame => "Frame",
        }
    }
}
