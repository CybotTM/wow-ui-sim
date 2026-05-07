use super::types::{FrameXml, XmlElement};
use super::types_elements::FrameElement;

macro_rules! frame_variant_data {
    ($self:expr, $($variant:ident),+ $(,)?) => {
        match $self {
            $(Self::$variant(f) => Some((f, stringify!($variant))),)+
            _ => None,
        }
    };
}

macro_rules! impl_frame_data_for {
    ($type:ty, $non_frame_note:literal) => {
        impl $type {
            /// Extract the inner `FrameXml` and variant tag name for frame-like elements.
            #[doc = $non_frame_note]
            pub fn as_frame_data(&self) -> Option<(&FrameXml, &'static str)> {
                self.as_primary_frame_data()
                    .or_else(|| self.as_secondary_frame_data())
            }

            fn as_primary_frame_data(&self) -> Option<(&FrameXml, &'static str)> {
                frame_variant_data!(
                    self,
                    Frame,
                    Button,
                    ItemButton,
                    CheckButton,
                    EditBox,
                    ScrollFrame,
                    Slider,
                    StatusBar,
                    GameTooltip,
                    ColorSelect,
                    Model,
                    ModelScene,
                    EventFrame,
                    CinematicModel,
                    PlayerModel,
                    DressUpModel
                )
            }

            fn as_secondary_frame_data(&self) -> Option<(&FrameXml, &'static str)> {
                frame_variant_data!(
                    self,
                    Browser,
                    Minimap,
                    MessageFrame,
                    MovieFrame,
                    ScrollingMessageFrame,
                    SimpleHTML,
                    WorldFrame,
                    DropDownToggleButton,
                    DropdownButton,
                    EventButton,
                    EventEditBox,
                    Cooldown,
                    TaxiRouteFrame,
                    ModelFFX,
                    TabardModel,
                    UiCamera,
                    UnitPositionFrame,
                    OffScreenFrame,
                    Checkout,
                    FogOfWarFrame,
                    QuestPOIFrame,
                    ArchaeologyDigSiteFrame,
                    ScenarioPOIFrame,
                    UIThemeContainerFrame,
                    EventScrollFrame,
                    ContainedAlertFrame,
                    MapScene,
                    Line
                )
            }
        }
    };
}

impl_frame_data_for!(
    XmlElement,
    "Returns `None` for non-frame elements (Script, Include, Font, etc.)."
);
impl_frame_data_for!(
    FrameElement,
    "Returns `None` for `ScopedModifier` which has no `FrameXml`."
);
