use wow_ui_sim::iced_app::frame_collect::frame_accepts_mouse_button;
use wow_ui_sim::widget::{Frame, WidgetType};

#[test]
fn decorative_regions_do_not_accept_mouse_button_targets() {
    let texture = Frame::new(
        WidgetType::Texture,
        Some("DecorativeTexture".to_string()),
        None,
    );

    assert!(
        !frame_accepts_mouse_button(&texture, "LeftButton"),
        "decorative regions must not become final click targets"
    );
}

#[test]
fn mouse_enabled_buttons_accept_non_passthrough_mouse_buttons() {
    let button = Frame::new(
        WidgetType::Button,
        Some("ClickableButton".to_string()),
        None,
    );

    assert!(
        frame_accepts_mouse_button(&button, "LeftButton"),
        "mouse-enabled buttons should accept normal click targets"
    );
}

#[test]
fn pass_through_buttons_do_not_accept_matching_mouse_buttons() {
    let mut button = Frame::new(
        WidgetType::Button,
        Some("PassThroughButton".to_string()),
        None,
    );
    button
        .pass_through_buttons
        .insert("rightbutton".to_string());

    assert!(
        !frame_accepts_mouse_button(&button, "RightButton"),
        "pass-through buttons should let matching clicks fall through"
    );
}
