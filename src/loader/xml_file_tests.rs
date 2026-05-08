use super::*;

fn default_frame() -> FrameXml {
    FrameXml::default()
}

/// Helper: call resolve_frame_element and return (widget_type, intrinsic).
fn resolve(elem: &XmlElement) -> Option<(&'static str, Option<&'static str>)> {
    resolve_frame_element(elem).map(|(_, wt, intr)| (wt, intr))
}

#[test]
fn specialized_widget_types() {
    let f = default_frame();
    assert_eq!(
        resolve(&XmlElement::Frame(f.clone())),
        Some(("Frame", None))
    );
    assert_eq!(
        resolve(&XmlElement::Button(f.clone())),
        Some(("Button", None))
    );
    assert_eq!(
        resolve(&XmlElement::ItemButton(f.clone())),
        Some(("Button", Some("ItemButton")))
    );
    assert_eq!(
        resolve(&XmlElement::CheckButton(f.clone())),
        Some(("CheckButton", None))
    );
    assert_eq!(
        resolve(&XmlElement::EditBox(f.clone())),
        Some(("EditBox", None))
    );
    assert_eq!(
        resolve(&XmlElement::EventEditBox(f.clone())),
        Some(("EditBox", Some("EventEditBox")))
    );
    assert_eq!(
        resolve(&XmlElement::ScrollFrame(f.clone())),
        Some(("ScrollFrame", None))
    );
    assert_eq!(
        resolve(&XmlElement::EventScrollFrame(f.clone())),
        Some(("ScrollFrame", Some("EventScrollFrame")))
    );
    assert_eq!(
        resolve(&XmlElement::Slider(f.clone())),
        Some(("Slider", None))
    );
    assert_eq!(
        resolve(&XmlElement::StatusBar(f.clone())),
        Some(("StatusBar", None))
    );
    assert_eq!(
        resolve(&XmlElement::Cooldown(f.clone())),
        Some(("Cooldown", None))
    );
    assert_eq!(
        resolve(&XmlElement::GameTooltip(f.clone())),
        Some(("GameTooltip", None))
    );
    assert_eq!(
        resolve(&XmlElement::ColorSelect(f.clone())),
        Some(("ColorSelect", None))
    );
    assert_eq!(
        resolve(&XmlElement::Model(f.clone())),
        Some(("Model", None))
    );
    assert_eq!(
        resolve(&XmlElement::ModelScene(f.clone())),
        Some(("ModelScene", None))
    );
    assert_eq!(
        resolve(&XmlElement::SimpleHTML(f.clone())),
        Some(("SimpleHTML", None))
    );
    assert_eq!(
        resolve(&XmlElement::Minimap(f.clone())),
        Some(("Minimap", None))
    );
    assert_eq!(
        resolve(&XmlElement::MessageFrame(f.clone())),
        Some(("MessageFrame", None))
    );
}

#[test]
fn player_model_variants_all_map_to_player_model() {
    let f = default_frame();
    assert_eq!(
        resolve(&XmlElement::PlayerModel(f.clone())),
        Some(("PlayerModel", None))
    );
    assert_eq!(
        resolve(&XmlElement::CinematicModel(f.clone())),
        Some(("PlayerModel", None))
    );
    assert_eq!(
        resolve(&XmlElement::TabardModel(f.clone())),
        Some(("PlayerModel", None))
    );
    assert_eq!(
        resolve(&XmlElement::DressUpModel(f.clone())),
        Some(("PlayerModel", None))
    );
}

#[test]
fn button_intrinsic_variants() {
    let f = default_frame();
    // DropDownToggleButton and EventButton map to plain Button (no intrinsic) in XmlElement
    assert_eq!(
        resolve(&XmlElement::DropDownToggleButton(f.clone())),
        Some(("Button", None))
    );
    assert_eq!(
        resolve(&XmlElement::EventButton(f.clone())),
        Some(("Button", None))
    );
    // DropdownButton has intrinsic
    assert_eq!(
        resolve(&XmlElement::DropdownButton(f.clone())),
        Some(("Button", Some("DropdownButton")))
    );
    // ContainedAlertFrame has intrinsic
    assert_eq!(
        resolve(&XmlElement::ContainedAlertFrame(f.clone())),
        Some(("Button", Some("ContainedAlertFrame")))
    );
}

#[test]
fn scrolling_message_frame_has_intrinsic() {
    let f = default_frame();
    assert_eq!(
        resolve(&XmlElement::ScrollingMessageFrame(f)),
        Some(("MessageFrame", Some("ScrollingMessageFrame")))
    );
}

#[test]
fn frame_like_elements_preserve_supported_alias_types() {
    let f = default_frame();
    let preserved_aliases = [
        XmlElement::EventFrame(f.clone()),
        XmlElement::UnitPositionFrame(f.clone()),
        XmlElement::OffScreenFrame(f.clone()),
        XmlElement::Checkout(f.clone()),
        XmlElement::FogOfWarFrame(f.clone()),
        XmlElement::QuestPOIFrame(f.clone()),
        XmlElement::ArchaeologyDigSiteFrame(f.clone()),
        XmlElement::ScenarioPOIFrame(f.clone()),
        XmlElement::Browser(f.clone()),
        XmlElement::MovieFrame(f.clone()),
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
fn unsupported_frame_like_elements_still_fall_back_to_frame() {
    let f = default_frame();
    let frame_likes = [
        XmlElement::TaxiRouteFrame(f.clone()),
        XmlElement::ModelFFX(f.clone()),
        XmlElement::UiCamera(f.clone()),
        XmlElement::UIThemeContainerFrame(f.clone()),
        XmlElement::MapScene(f.clone()),
        XmlElement::Line(f.clone()),
        XmlElement::WorldFrame(f.clone()),
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
fn non_frame_elements_return_none() {
    use crate::xml::ScriptXml;
    assert_eq!(
        resolve(&XmlElement::Script(ScriptXml {
            file: None,
            inline: None
        })),
        None
    );
    assert_eq!(resolve(&XmlElement::Text("hello".into())), None);
    assert_eq!(resolve(&XmlElement::Unknown), None);
}

/// Document the differences between XmlElement and FrameElement mappings.
/// ItemButton resolves as a Button with the ItemButton intrinsic base here,
/// while FrameElement preserves the raw alias and inherits are resolved later.
/// DropDownToggleButton/EventButton have no intrinsic here but do in FrameElement.
#[test]
fn xml_vs_frame_element_divergences() {
    let f = default_frame();
    // XmlElement::ItemButton -> ("Button", Some("ItemButton"))
    assert_eq!(
        resolve(&XmlElement::ItemButton(f.clone())),
        Some(("Button", Some("ItemButton")))
    );
    // XmlElement::DropDownToggleButton -> ("Button", None) — no intrinsic
    assert_eq!(
        resolve(&XmlElement::DropDownToggleButton(f.clone())),
        Some(("Button", None))
    );
    // XmlElement::EventButton -> ("Button", None) — no intrinsic
    assert_eq!(
        resolve(&XmlElement::EventButton(f.clone())),
        Some(("Button", None))
    );
}

#[test]
fn roman_font_overrides_with_all_fields() {
    let ff = crate::xml::FontFamilyXml {
        name: Some("TestFont".to_string()),
        is_virtual: None,
        members: vec![crate::xml::FontFamilyMemberXml {
            alphabet: Some("roman".to_string()),
            font: Some(crate::xml::FontXml {
                font: Some("Fonts\\Test.TTF".to_string()),
                height: Some(14.0),
                outline: Some("OUTLINE".to_string()),
                ..Default::default()
            }),
        }],
    };
    let code = build_roman_font_overrides("TestFont", &ff);
    assert!(code.contains("TestFont.__font = \"Fonts/Test.TTF\""));
    assert!(code.contains("TestFont.__height = 14"));
    assert!(code.contains("TestFont.__outline = \"OUTLINE\""));
}

#[test]
fn roman_font_overrides_no_roman_member() {
    let ff = crate::xml::FontFamilyXml {
        name: Some("TestFont".to_string()),
        is_virtual: None,
        members: vec![crate::xml::FontFamilyMemberXml {
            alphabet: Some("hangul".to_string()),
            font: Some(crate::xml::FontXml::default()),
        }],
    };
    let code = build_roman_font_overrides("TestFont", &ff);
    assert!(code.is_empty());
}

#[test]
fn roman_font_overrides_partial_fields() {
    let ff = crate::xml::FontFamilyXml {
        name: Some("TestFont".to_string()),
        is_virtual: None,
        members: vec![crate::xml::FontFamilyMemberXml {
            alphabet: Some("roman".to_string()),
            font: Some(crate::xml::FontXml {
                height: Some(16.0),
                ..Default::default()
            }),
        }],
    };
    let code = build_roman_font_overrides("TestFont", &ff);
    assert!(!code.contains("__font"));
    assert!(code.contains("TestFont.__height = 16"));
    assert!(!code.contains("__outline"));
}
