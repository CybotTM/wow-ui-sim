use super::*;

#[test]
fn anchor_points_default_relative_point_and_reject_invalid_values() {
    let default_relative = AnchorXml {
        point: Some("BOTTOM".to_string()),
        relative_point: None,
        relative_to: None,
        relative_key: None,
        x: None,
        y: None,
        offset: None,
    };
    assert_eq!(
        anchor_points(&default_relative),
        Some((AnchorPoint::Bottom, AnchorPoint::Bottom))
    );

    let invalid_point = AnchorXml {
        point: Some("NOPE".to_string()),
        ..default_relative.clone()
    };
    assert!(
        anchor_points(&invalid_point).is_none(),
        "invalid primary point should reject the anchor"
    );

    let invalid_relative = AnchorXml {
        relative_point: Some("NOPE".to_string()),
        ..default_relative
    };
    assert!(
        anchor_points(&invalid_relative).is_none(),
        "invalid relative point should reject the anchor"
    );
}

#[test]
fn explicit_hidden_false_restores_frame_shown_state() {
    let state = Rc::new(RefCell::new(crate::lua_api::SimState::default()));
    let frame_id = {
        let mut state = state.borrow_mut();
        let frame = crate::widget::Frame::new(crate::widget::WidgetType::Frame, None, None);
        state.widgets.register(frame)
    };

    state.borrow_mut().set_frame_visible(frame_id, false);
    apply_xml_hidden(
        &state,
        frame_id,
        &FrameXml {
            hidden: Some(false),
            ..FrameXml::default()
        },
        "",
    );

    let is_shown = state
        .borrow()
        .widgets
        .get(frame_id)
        .expect("test frame should remain registered")
        .visible;
    assert!(
        is_shown,
        "explicit hidden=\"false\" must override an inherited hidden=true template"
    );
}
