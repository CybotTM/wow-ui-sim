use super::*;
use crate::xml::{FrameElement, FrameXml};

fn default_frame() -> FrameXml {
    FrameXml::default()
}

/// Helper: call frame_element_to_type and return (widget_type, intrinsic).
fn resolve(elem: &FrameElement) -> Option<(&'static str, Option<&'static str>)> {
    frame_element_to_type(elem).map(|(_, wt, intr)| (wt, intr))
}

#[test]
fn specialized_widget_types() {
    let f = default_frame();
    assert_eq!(
        resolve(&FrameElement::Frame(f.clone())),
        Some(("Frame", None))
    );
    assert_eq!(
        resolve(&FrameElement::Button(f.clone())),
        Some(("Button", None))
    );
    assert_eq!(
        resolve(&FrameElement::ItemButton(f.clone())),
        Some(("ItemButton", None))
    );
    assert_eq!(
        resolve(&FrameElement::CheckButton(f.clone())),
        Some(("CheckButton", None))
    );
    assert_eq!(
        resolve(&FrameElement::EditBox(f.clone())),
        Some(("EditBox", None))
    );
    assert_eq!(
        resolve(&FrameElement::EventEditBox(f.clone())),
        Some(("EditBox", None))
    );
    assert_eq!(
        resolve(&FrameElement::ScrollFrame(f.clone())),
        Some(("ScrollFrame", None))
    );
    assert_eq!(
        resolve(&FrameElement::EventScrollFrame(f.clone())),
        Some(("ScrollFrame", None))
    );
    assert_eq!(
        resolve(&FrameElement::Slider(f.clone())),
        Some(("Slider", None))
    );
    assert_eq!(
        resolve(&FrameElement::StatusBar(f.clone())),
        Some(("StatusBar", None))
    );
    assert_eq!(
        resolve(&FrameElement::Cooldown(f.clone())),
        Some(("Cooldown", None))
    );
    assert_eq!(
        resolve(&FrameElement::GameTooltip(f.clone())),
        Some(("GameTooltip", None))
    );
    assert_eq!(
        resolve(&FrameElement::ColorSelect(f.clone())),
        Some(("ColorSelect", None))
    );
    assert_eq!(
        resolve(&FrameElement::Model(f.clone())),
        Some(("Model", None))
    );
    assert_eq!(
        resolve(&FrameElement::ModelScene(f.clone())),
        Some(("ModelScene", None))
    );
    assert_eq!(
        resolve(&FrameElement::SimpleHTML(f.clone())),
        Some(("SimpleHTML", None))
    );
    assert_eq!(
        resolve(&FrameElement::Minimap(f.clone())),
        Some(("Minimap", None))
    );
    assert_eq!(
        resolve(&FrameElement::MessageFrame(f.clone())),
        Some(("MessageFrame", None))
    );
}

#[test]
fn player_model_variants_all_map_to_player_model() {
    let f = default_frame();
    assert_eq!(
        resolve(&FrameElement::PlayerModel(f.clone())),
        Some(("PlayerModel", None))
    );
    assert_eq!(
        resolve(&FrameElement::CinematicModel(f.clone())),
        Some(("PlayerModel", None))
    );
    assert_eq!(
        resolve(&FrameElement::TabardModel(f.clone())),
        Some(("PlayerModel", None))
    );
    assert_eq!(
        resolve(&FrameElement::DressUpModel(f.clone())),
        Some(("PlayerModel", None))
    );
}

#[test]
fn button_intrinsic_variants() {
    let f = default_frame();
    assert_eq!(
        resolve(&FrameElement::DropdownButton(f.clone())),
        Some(("Button", Some("DropdownButton")))
    );
    assert_eq!(
        resolve(&FrameElement::DropDownToggleButton(f.clone())),
        Some(("Button", Some("DropDownToggleButton")))
    );
    assert_eq!(
        resolve(&FrameElement::EventButton(f.clone())),
        Some(("Button", Some("EventButton")))
    );
    assert_eq!(
        resolve(&FrameElement::ContainedAlertFrame(f.clone())),
        Some(("Button", Some("ContainedAlertFrame")))
    );
}

#[test]
fn scrolling_message_frame_has_intrinsic() {
    let f = default_frame();
    assert_eq!(
        resolve(&FrameElement::ScrollingMessageFrame(f)),
        Some(("MessageFrame", Some("ScrollingMessageFrame")))
    );
}

#[test]
fn frame_like_elements_preserve_supported_alias_types() {
    let f = default_frame();
    let preserved_aliases = [
        FrameElement::EventFrame(f.clone()),
        FrameElement::UnitPositionFrame(f.clone()),
        FrameElement::OffScreenFrame(f.clone()),
        FrameElement::Checkout(f.clone()),
        FrameElement::FogOfWarFrame(f.clone()),
        FrameElement::QuestPOIFrame(f.clone()),
        FrameElement::ArchaeologyDigSiteFrame(f.clone()),
        FrameElement::ScenarioPOIFrame(f.clone()),
        FrameElement::Browser(f.clone()),
        FrameElement::MovieFrame(f.clone()),
    ];
    for elem in &preserved_aliases {
        let (_, tag) = elem.as_frame_data().unwrap();
        assert_eq!(
            resolve(elem),
            Some((tag, None)),
            "Expected preserved type for {:?}",
            std::mem::discriminant(elem)
        );
    }
}

#[test]
fn unsupported_frame_like_elements_still_map_to_frame() {
    let f = default_frame();
    let frame_likes = [
        FrameElement::TaxiRouteFrame(f.clone()),
        FrameElement::ModelFFX(f.clone()),
        FrameElement::UiCamera(f.clone()),
        FrameElement::UIThemeContainerFrame(f.clone()),
        FrameElement::MapScene(f.clone()),
        FrameElement::Line(f.clone()),
        FrameElement::WorldFrame(f.clone()),
    ];
    for elem in &frame_likes {
        assert_eq!(
            resolve(elem),
            Some(("Frame", None)),
            "Expected Frame for {:?}",
            std::mem::discriminant(elem)
        );
    }
}

#[test]
fn scoped_modifier_returns_none() {
    use crate::xml::ScopedModifierXml;
    let sm = ScopedModifierXml {
        forbidden: None,
        elements: vec![],
    };
    assert_eq!(resolve(&FrameElement::ScopedModifier(sm)), None);
}
