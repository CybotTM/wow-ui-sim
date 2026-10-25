use super::*;

#[cfg(feature = "profile-retail")]
#[test]
fn test_parse_objective_tracker_widget_container_xml_keeps_self_closing_frame() {
    let path = crate::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
    .join("Blizzard_ObjectiveTracker/Blizzard_ObjectiveTrackerUIWidgetContainer.xml");
    let ui = parse_xml_file(&path).expect("ObjectiveTracker widget container XML should parse");

    let frame_names = ui
        .elements
        .iter()
        .filter_map(|element| {
            element
                .as_frame_data()
                .and_then(|(frame, _)| frame.name.as_deref())
        })
        .collect::<Vec<_>>();

    assert!(
        frame_names.contains(&"UIWidgetObjectiveTracker"),
        "expected UIWidgetObjectiveTracker in parsed frame list: {frame_names:?}"
    );
    assert!(
        frame_names.contains(&"ObjectiveTrackerUIWidgetContainer"),
        "expected ObjectiveTrackerUIWidgetContainer in parsed frame list: {frame_names:?}"
    );
}

#[cfg(feature = "profile-retail")]
#[test]
fn test_parse_low_health_frame_xml_keeps_animations() {
    let path = crate::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
    .join("Blizzard_FrameXML/Mainline/LowHealthFrame.xml");
    let ui = parse_xml_file(&path).expect("LowHealthFrame XML should parse");

    let low_health_frame = ui
        .elements
        .iter()
        .filter_map(|element| element.as_frame_data())
        .find_map(|(frame, _)| (frame.name.as_deref() == Some("LowHealthFrame")).then_some(frame))
        .expect("LowHealthFrame should exist in parsed XML");

    let animations = low_health_frame
        .animations()
        .expect("LowHealthFrame should keep its Animations block");
    assert_eq!(animations.animations.len(), 1);
    assert_eq!(
        animations.animations[0].parent_key.as_deref(),
        Some("pulseAnim")
    );
}
