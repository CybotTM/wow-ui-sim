use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::xml::{FrameXml, UiXml, XmlElement, parse_xml_file};

const ROOT: &str = "Blizzard_AutoComplete";
const EXPECTED_BUTTON_COUNT: usize = 5;
const EXPECTED_DEFAULT_Y_OFFSET: f64 = 3.0;
const EXPECTED_BUTTON_TOP_PADDING: f32 = 10.0;
const EXPECTED_ATTACH_THRESHOLD: f64 = 13.0;

#[test]
fn blizzard_auto_complete_button_count_and_magic_offset_stay_locked() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete button constants can be checked. \
                     Loaded set: {loaded:?}"
                );

                let (max_buttons, default_y_offset): (f64, f64) = env
                    .eval("return AUTOCOMPLETE_MAX_BUTTONS, AUTOCOMPLETE_DEFAULT_Y_OFFSET")
                    .expect("AutoComplete constants should be readable");
                assert_eq!(max_buttons, EXPECTED_BUTTON_COUNT as f64);
                assert_eq!(default_y_offset, EXPECTED_DEFAULT_Y_OFFSET);
                assert_eq!(
                    default_y_offset + f64::from(EXPECTED_BUTTON_TOP_PADDING),
                    EXPECTED_ATTACH_THRESHOLD
                );
            });
        });
    });

    let xml_path = wow_ui_sim::paths::default_blizzard_ui_addons_path()
        .expect("Blizzard UI cache should be available")
        .join("Blizzard_AutoComplete/AutoComplete.xml");
    let ui = parse_xml_file(&xml_path).expect("AutoComplete.xml should parse");
    let box_frame = find_top_level_frame(&ui, "AutoCompleteBox");
    let buttons = auto_complete_buttons(box_frame);

    assert_eq!(
        buttons.len(),
        EXPECTED_BUTTON_COUNT,
        "AutoCompleteBox XML must define exactly {EXPECTED_BUTTON_COUNT} stacked buttons"
    );

    assert_first_button_top_padding(buttons[0].0);
    for (index, (button, tag)) in buttons.iter().enumerate() {
        assert_eq!(
            *tag,
            "Button",
            "AutoCompleteButton{} must be a Button",
            index + 1
        );
        assert_eq!(
            button.inherits.as_deref(),
            Some("AutoCompleteButtonTemplate"),
            "AutoCompleteButton{} must inherit AutoCompleteButtonTemplate",
            index + 1
        );
        assert_stacked_button_anchor(index + 1, button);
    }
}

fn find_top_level_frame<'a>(ui: &'a UiXml, name: &str) -> &'a FrameXml {
    ui.elements
        .iter()
        .find_map(|element| match element {
            XmlElement::Frame(frame) if frame.name.as_deref() == Some(name) => Some(frame),
            _ => None,
        })
        .unwrap_or_else(|| panic!("{name} must exist as a top-level XML frame"))
}

fn auto_complete_buttons(box_frame: &FrameXml) -> Vec<(&FrameXml, &'static str)> {
    box_frame
        .all_frame_elements()
        .into_iter()
        .filter(|(frame, _)| {
            frame
                .name
                .as_deref()
                .is_some_and(|name| name.starts_with("AutoCompleteButton"))
        })
        .collect()
}

fn assert_first_button_top_padding(button: &FrameXml) {
    let anchor = only_anchor(button, "AutoCompleteButton1");
    assert_eq!(anchor.point.as_deref(), Some("TOPLEFT"));
    assert_eq!(anchor.relative_to.as_deref(), None);
    assert_eq!(anchor.relative_point.as_deref(), None);

    let abs_dimension = anchor
        .offset
        .as_ref()
        .and_then(|offset| offset.abs_dimension.as_ref())
        .expect("AutoCompleteButton1 TOPLEFT anchor must use AbsDimension offset");
    assert_eq!(abs_dimension.x, Some(0.0));
    assert_eq!(abs_dimension.y, Some(-EXPECTED_BUTTON_TOP_PADDING));
}

fn assert_stacked_button_anchor(index: usize, button: &FrameXml) {
    if index == 1 {
        return;
    }

    let button_name = format!("AutoCompleteButton{index}");
    let previous_button_name = format!("AutoCompleteButton{}", index - 1);
    let anchor = only_anchor(button, &button_name);

    assert_eq!(anchor.point.as_deref(), Some("TOPLEFT"));
    assert_eq!(
        anchor.relative_to.as_deref(),
        Some(previous_button_name.as_str())
    );
    assert_eq!(anchor.relative_point.as_deref(), Some("BOTTOMLEFT"));
}

fn only_anchor<'a>(button: &'a FrameXml, button_name: &str) -> &'a wow_ui_sim::xml::AnchorXml {
    let anchors = button
        .anchors()
        .unwrap_or_else(|| panic!("{button_name} must define anchors"));
    assert_eq!(
        anchors.anchors.len(),
        1,
        "{button_name} must define exactly one anchor"
    );
    &anchors.anchors[0]
}
