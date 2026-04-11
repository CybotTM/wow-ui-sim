use wow_ui_sim::xml::{AnimationElement, FrameChildElement, XmlElement, parse_xml};

#[test]
fn test_parse_animation_group_with_alpha() {
    let xml = r#"
        <Ui>
            <Frame name="TestFrame">
                <Animations>
                    <AnimationGroup parentKey="FadeIn" looping="NONE">
                        <Alpha fromAlpha="0" toAlpha="1" duration="0.3" order="1" smoothing="OUT"/>
                    </AnimationGroup>
                </Animations>
            </Frame>
        </Ui>
    "#;
    let ui = parse_xml(xml).expect("Failed to parse");
    let f = match &ui.elements[0] {
        XmlElement::Frame(f) => f,
        _ => panic!("Expected Frame"),
    };
    let anims: Vec<_> = f
        .children
        .iter()
        .filter_map(|c| match c {
            FrameChildElement::Animations(a) => Some(a),
            _ => None,
        })
        .collect();
    assert_eq!(anims.len(), 1);
    let group = &anims[0].animations[0];
    assert_eq!(group.parent_key.as_deref(), Some("FadeIn"));
    assert_eq!(group.looping.as_deref(), Some("NONE"));

    let alpha = group
        .elements
        .iter()
        .find_map(|e| match e {
            AnimationElement::Alpha(a) => Some(a),
            _ => None,
        })
        .expect("Expected Alpha animation");
    assert_eq!(alpha.from_alpha, Some(0.0));
    assert_eq!(alpha.to_alpha, Some(1.0));
    assert_eq!(alpha.duration, Some(0.3));
    assert_eq!(alpha.order, Some(1));
    assert_eq!(alpha.smoothing.as_deref(), Some("OUT"));
}

#[test]
fn test_parse_translation_animation() {
    let xml = r#"
        <Ui>
            <Frame name="T">
                <Animations>
                    <AnimationGroup>
                        <Translation offsetX="10" offsetY="-20" duration="0.5" order="1"/>
                        <LineTranslation offsetX="5" offsetY="5" duration="1.0"/>
                    </AnimationGroup>
                </Animations>
            </Frame>
        </Ui>
    "#;
    let ui = parse_xml(xml).expect("Failed to parse");
    let f = match &ui.elements[0] {
        XmlElement::Frame(f) => f,
        _ => panic!(),
    };
    let group = &f
        .children
        .iter()
        .find_map(|c| match c {
            FrameChildElement::Animations(a) => Some(a),
            _ => None,
        })
        .unwrap()
        .animations[0];

    let tr = group
        .elements
        .iter()
        .find_map(|e| match e {
            AnimationElement::Translation(a) => Some(a),
            _ => None,
        })
        .expect("Expected Translation");
    assert_eq!(tr.offset_x, Some(10.0));
    assert_eq!(tr.offset_y, Some(-20.0));

    let lt = group
        .elements
        .iter()
        .find_map(|e| match e {
            AnimationElement::LineTranslation(a) => Some(a),
            _ => None,
        })
        .expect("Expected LineTranslation");
    assert_eq!(lt.offset_x, Some(5.0));
}

#[test]
fn test_parse_rotation_animation() {
    let xml = r#"
        <Ui>
            <Frame name="T">
                <Animations>
                    <AnimationGroup>
                        <Rotation degrees="-180" duration="1.0" smoothing="OUT" childKey="Swirl"/>
                    </AnimationGroup>
                </Animations>
            </Frame>
        </Ui>
    "#;
    let ui = parse_xml(xml).expect("Failed to parse");
    let f = match &ui.elements[0] {
        XmlElement::Frame(f) => f,
        _ => panic!(),
    };
    let group = &f
        .children
        .iter()
        .find_map(|c| match c {
            FrameChildElement::Animations(a) => Some(a),
            _ => None,
        })
        .unwrap()
        .animations[0];

    let rot = group
        .elements
        .iter()
        .find_map(|e| match e {
            AnimationElement::Rotation(a) => Some(a),
            _ => None,
        })
        .expect("Expected Rotation");
    assert_eq!(rot.degrees, Some(-180.0));
    assert_eq!(rot.child_key.as_deref(), Some("Swirl"));
}

#[test]
fn test_parse_scale_animations() {
    let xml = r#"
        <Ui>
            <Frame name="T">
                <Animations>
                    <AnimationGroup>
                        <Scale fromScaleX="0" fromScaleY="0" toScaleX="1" toScaleY="1" duration="0.4"/>
                        <LineScale scaleX="2.0" scaleY="2.0" duration="0.2"/>
                    </AnimationGroup>
                </Animations>
            </Frame>
        </Ui>
    "#;
    let ui = parse_xml(xml).expect("Failed to parse");
    let f = match &ui.elements[0] {
        XmlElement::Frame(f) => f,
        _ => panic!(),
    };
    let group = &f
        .children
        .iter()
        .find_map(|c| match c {
            FrameChildElement::Animations(a) => Some(a),
            _ => None,
        })
        .unwrap()
        .animations[0];

    let scale = group
        .elements
        .iter()
        .find_map(|e| match e {
            AnimationElement::Scale(a) => Some(a),
            _ => None,
        })
        .expect("Expected Scale");
    assert_eq!(scale.from_scale_x, Some(0.0));
    assert_eq!(scale.to_scale_x, Some(1.0));

    assert!(
        group
            .elements
            .iter()
            .any(|e| matches!(e, AnimationElement::LineScale(_)))
    );
}

#[test]
fn test_parse_path_flipbook_vertexcolor_texcoord() {
    let xml = r#"
        <Ui>
            <Frame name="T">
                <Animations>
                    <AnimationGroup>
                        <Path curve="SMOOTH" duration="1.0"/>
                        <FlipBook flipBookRows="4" flipBookColumns="4" flipBookFrames="16" duration="2.0"/>
                        <VertexColor duration="0.5"/>
                        <TextureCoordTranslation offsetU="0.5" offsetV="-0.5" duration="1.0"/>
                        <Animation duration="0.1" order="1"/>
                    </AnimationGroup>
                </Animations>
            </Frame>
        </Ui>
    "#;
    let ui = parse_xml(xml).expect("Failed to parse");
    let f = match &ui.elements[0] {
        XmlElement::Frame(f) => f,
        _ => panic!(),
    };
    let group = &f
        .children
        .iter()
        .find_map(|c| match c {
            FrameChildElement::Animations(a) => Some(a),
            _ => None,
        })
        .unwrap()
        .animations[0];

    let path = group
        .elements
        .iter()
        .find_map(|e| match e {
            AnimationElement::Path(a) => Some(a),
            _ => None,
        })
        .expect("Expected Path");
    assert_eq!(path.curve.as_deref(), Some("SMOOTH"));

    let fb = group
        .elements
        .iter()
        .find_map(|e| match e {
            AnimationElement::FlipBook(a) => Some(a),
            _ => None,
        })
        .expect("Expected FlipBook");
    assert_eq!(fb.flip_book_rows, Some(4));
    assert_eq!(fb.flip_book_columns, Some(4));
    assert_eq!(fb.flip_book_frames, Some(16));

    assert!(
        group
            .elements
            .iter()
            .any(|e| matches!(e, AnimationElement::VertexColor(_)))
    );

    let tc = group
        .elements
        .iter()
        .find_map(|e| match e {
            AnimationElement::TextureCoordTranslation(a) => Some(a),
            _ => None,
        })
        .expect("Expected TextureCoordTranslation");
    assert_eq!(tc.offset_u, Some(0.5));
    assert_eq!(tc.offset_v, Some(-0.5));

    assert!(
        group
            .elements
            .iter()
            .any(|e| matches!(e, AnimationElement::Animation(_)))
    );
}
