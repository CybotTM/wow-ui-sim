//! XML-template surface for `Blizzard_ArrowCalloutFrame`.

use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::xml::{
    AnimationElement, AnimationGroupXml, AnimationXml, FrameXml, LayerElement, TextureXml,
    XmlElement, parse_xml_file,
};

const ROOT: &str = "Blizzard_ArrowCalloutFrame";

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
}

struct PointerTemplateExpectation {
    template: &'static str,
    direction: &'static str,
    axis: Axis,
    offset: f32,
}

#[test]
fn arrow_callout_pointer_templates_define_atlases_and_looping_animation() {
    let ui = parse_xml_file(&arrow_callout_xml())
        .expect("ArrowCalloutFrame.xml must parse as Blizzard UI XML");

    for expectation in pointer_template_expectations() {
        let frame = find_top_level_frame(&ui.elements, expectation.template);

        assert_eq!(
            frame.is_virtual,
            Some(true),
            "`{}` must be a virtual template",
            expectation.template
        );

        assert_template_textures(frame, expectation);
        assert_template_animation(frame, expectation);
    }
}

#[test]
fn arrow_callout_container_templates_define_expected_child_surface() {
    let ui = parse_xml_file(&arrow_callout_xml())
        .expect("ArrowCalloutFrame.xml must parse as Blizzard UI XML");

    let container = find_top_level_frame(&ui.elements, "ArrowCalloutContainerTemplate");
    assert_frame_inherits(container, "ResizeLayoutFrame");
    assert_key_value(container, "widthPadding", "20", "number");
    assert_key_value(container, "heightPadding", "20", "number");

    let content = child_frame(container, "Content", "Frame");
    assert_frame_inherits(content, "GlowBoxTemplate");

    let glow = child_frame(container, "Glow", "Frame");
    assert_frame_inherits(glow, "BackdropTemplate");

    let close_container = find_top_level_frame(
        &ui.elements,
        "ArrowCalloutContainerTemplateWithCloseButtonTemplate",
    );
    assert_frame_inherits(close_container, "ArrowCalloutContainerTemplate");
    assert_key_value(close_container, "widthPadding", "40", "number");

    let close_button = child_frame(close_container, "CloseButton", "Button");
    assert_frame_inherits(close_button, "UIPanelCloseButton");
    assert_eq!(
        close_button.mixin.as_deref(),
        Some("ArrowCalloutCloseButtonMixin"),
        "`CloseButton` must mix in the ArrowCallout close-button behavior"
    );

    let widget_container = find_top_level_frame(&ui.elements, "WidgetContainerCalloutTemplate");
    assert_frame_inherits(widget_container, "UIWidgetContainerTemplate");
    assert_eq!(
        widget_container.hidden,
        Some(true),
        "`WidgetContainerCalloutTemplate` must be hidden by default"
    );
}

fn pointer_template_expectations() -> &'static [PointerTemplateExpectation] {
    &[
        PointerTemplateExpectation {
            template: "ArrowCalloutPointerUp",
            direction: "Up",
            axis: Axis::Y,
            offset: 30.0,
        },
        PointerTemplateExpectation {
            template: "ArrowCalloutPointerDown",
            direction: "Down",
            axis: Axis::Y,
            offset: -30.0,
        },
        PointerTemplateExpectation {
            template: "ArrowCalloutPointerLeft",
            direction: "Left",
            axis: Axis::X,
            offset: -30.0,
        },
        PointerTemplateExpectation {
            template: "ArrowCalloutPointerRight",
            direction: "Right",
            axis: Axis::X,
            offset: 30.0,
        },
    ]
}

fn assert_template_textures(frame: &FrameXml, expectation: &PointerTemplateExpectation) {
    let background_atlas = format!("NPE_Arrow{}", expectation.direction);
    let glow_atlas = format!("NPE_Arrow{}Glow", expectation.direction);

    let background = texture_in_layer(frame, "BACKGROUND", &background_atlas);
    assert_eq!(
        background.alpha_mode, None,
        "`{}` background arrow texture must not use additive alpha",
        expectation.template
    );

    let glow = texture_in_layer(frame, "OVERLAY", &glow_atlas);
    assert_eq!(
        glow.alpha_mode.as_deref(),
        Some("ADD"),
        "`{}` glow texture must use alphaMode=\"ADD\"",
        expectation.template
    );
}

fn assert_template_animation(frame: &FrameXml, expectation: &PointerTemplateExpectation) {
    let group = anim_group(frame, expectation.template);
    let translation = single_translation(group, expectation.template);

    assert_translation(translation, expectation);
    assert_alpha_fade_sequence(group, expectation.template);
    assert_animation_replays_on_finish(group, expectation.template);
}

fn anim_group<'a>(frame: &'a FrameXml, template: &str) -> &'a AnimationGroupXml {
    frame
        .animations()
        .and_then(|animations| {
            animations
                .animations
                .iter()
                .find(|group| group.parent_key.as_deref() == Some("Anim"))
        })
        .unwrap_or_else(|| {
            panic!("`{template}` must expose an AnimationGroup with parentKey=\"Anim\"")
        })
}

fn assert_alpha_fade_sequence(group: &AnimationGroupXml, template: &str) {
    let alphas: Vec<&AnimationXml> = group
        .elements
        .iter()
        .filter_map(|element| match element {
            AnimationElement::Alpha(animation) => Some(animation),
            _ => None,
        })
        .collect();
    assert_eq!(
        alphas.len(),
        2,
        "`{template}` Anim group must define exactly two Alpha animations"
    );
    assert_alpha(alphas[0], 0.0, 1.0, 0.1, None, template);
    assert_alpha(alphas[1], 1.0, 0.0, 0.9, Some(0.1), template);
    assert_eq!(
        alphas[1].smoothing.as_deref(),
        Some("IN"),
        "`{template}` fade-out alpha animation must smooth in"
    );
}

fn assert_translation(animation: &AnimationXml, expectation: &PointerTemplateExpectation) {
    assert_eq!(
        animation.duration,
        Some(1.0),
        "`{}` translation duration must match the Blizzard template",
        expectation.template
    );
    assert_eq!(
        animation.order,
        Some(1),
        "`{}` translation order must match the Blizzard template",
        expectation.template
    );
    assert_eq!(
        animation.smoothing.as_deref(),
        Some("OUT"),
        "`{}` translation must smooth out",
        expectation.template
    );

    match expectation.axis {
        Axis::X => {
            assert_eq!(animation.offset_x, Some(expectation.offset));
            assert_eq!(animation.offset_y, None);
        }
        Axis::Y => {
            assert_eq!(animation.offset_x, None);
            assert_eq!(animation.offset_y, Some(expectation.offset));
        }
    }
}

fn assert_alpha(
    animation: &AnimationXml,
    from_alpha: f32,
    to_alpha: f32,
    duration: f32,
    start_delay: Option<f32>,
    template: &str,
) {
    assert_eq!(
        animation.from_alpha,
        Some(from_alpha),
        "`{template}` alpha animation must start at the expected opacity"
    );
    assert_eq!(
        animation.to_alpha,
        Some(to_alpha),
        "`{template}` alpha animation must end at the expected opacity"
    );
    assert_eq!(
        animation.duration,
        Some(duration),
        "`{template}` alpha animation must have the expected duration"
    );
    assert_eq!(
        animation.start_delay, start_delay,
        "`{template}` alpha animation must have the expected start delay"
    );
    assert_eq!(
        animation.order,
        Some(1),
        "`{template}` alpha animation order must match the Blizzard template"
    );
}

fn assert_animation_replays_on_finish(group: &AnimationGroupXml, template: &str) {
    let on_finished = group.elements.iter().find_map(|element| match element {
        AnimationElement::Scripts(scripts) => scripts.on_finished.first(),
        _ => None,
    });

    let body = on_finished
        .and_then(|script| script.body.as_deref())
        .unwrap_or_else(|| panic!("`{template}` Anim group must define an OnFinished script"));

    assert!(
        body.contains("self:Play();"),
        "`{template}` OnFinished script must replay the pointer animation"
    );
}

fn single_translation<'a>(group: &'a AnimationGroupXml, template: &str) -> &'a AnimationXml {
    let translations: Vec<&AnimationXml> = group
        .elements
        .iter()
        .filter_map(|element| match element {
            AnimationElement::Translation(animation) => Some(animation),
            _ => None,
        })
        .collect();

    assert_eq!(
        translations.len(),
        1,
        "`{template}` Anim group must define exactly one Translation animation"
    );

    translations[0]
}

fn texture_in_layer<'a>(frame: &'a FrameXml, layer: &str, atlas: &str) -> &'a TextureXml {
    frame
        .layers()
        .flat_map(|layers| layers.layers.iter())
        .filter(|entry| entry.level.as_deref() == Some(layer))
        .flat_map(|entry| entry.elements.iter())
        .find_map(|element| match element {
            LayerElement::Texture(texture) if texture.atlas.as_deref() == Some(atlas) => {
                Some(texture)
            }
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!(
                "`{}` must define `{atlas}` in the `{layer}` layer",
                frame_name(frame)
            )
        })
}

fn find_top_level_frame<'a>(elements: &'a [XmlElement], name: &str) -> &'a FrameXml {
    elements
        .iter()
        .find_map(|element| match element {
            XmlElement::Frame(frame) if frame.name.as_deref() == Some(name) => Some(frame),
            _ => None,
        })
        .unwrap_or_else(|| panic!("`{name}` must exist as a top-level frame template"))
}

fn child_frame<'a>(parent: &'a FrameXml, parent_key: &str, tag: &str) -> &'a FrameXml {
    parent
        .all_frame_elements()
        .into_iter()
        .find_map(|(frame, frame_tag)| {
            let matches_key = frame.parent_key.as_deref() == Some(parent_key);
            let matches_tag = frame_tag == tag;
            if matches_key && matches_tag {
                Some(frame)
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            panic!(
                "`{}` must expose `{parent_key}` as a `{tag}` child",
                frame_name(parent)
            )
        })
}

fn assert_frame_inherits(frame: &FrameXml, expected_inherits: &str) {
    assert_eq!(
        frame.inherits.as_deref(),
        Some(expected_inherits),
        "`{}` must inherit `{expected_inherits}`",
        frame_name(frame)
    );
}

fn assert_key_value(frame: &FrameXml, key: &str, expected_value: &str, expected_type: &str) {
    let key_value = frame
        .all_key_values()
        .flat_map(|key_values| key_values.values.iter())
        .find(|key_value| key_value.key == key)
        .unwrap_or_else(|| panic!("`{}` must define KeyValue `{key}`", frame_name(frame)));

    assert_eq!(
        key_value.value,
        expected_value,
        "`{}` must set KeyValue `{key}` to `{expected_value}`",
        frame_name(frame)
    );
    assert_eq!(
        key_value.value_type.as_deref(),
        Some(expected_type),
        "`{}` must type KeyValue `{key}` as `{expected_type}`",
        frame_name(frame)
    );
}

fn frame_name(frame: &FrameXml) -> &str {
    frame.name.as_deref().unwrap_or("<unnamed frame>")
}

fn arrow_callout_xml() -> std::path::PathBuf {
    blizzard_ui_dir().join(ROOT).join("ArrowCalloutFrame.xml")
}
