
use super::*;

#[test]
fn intrinsic_templates_register_engine_frames_and_button_frame_parts() {
    register_intrinsic_templates();

    let scroll_box = get_template("WoWScrollBoxList").expect("scroll box list intrinsic");
    assert_eq!(scroll_box.widget_type, "Frame");
    assert_eq!(
        scroll_box.frame.inherits.as_deref(),
        Some("ScrollBoxBaseTemplate")
    );
    assert_eq!(
        scroll_box.frame.mixin.as_deref(),
        Some("ScrollBoxListMixin")
    );

    let button_frame = get_template("ButtonFrameTemplate").expect("button frame intrinsic");
    assert_eq!(button_frame.widget_type, "Frame");
    assert_eq!(button_frame.frame.children.len(), 1);
    let FrameChildElement::Frame(inset) = &button_frame.frame.children[0] else {
        panic!("ButtonFrameTemplate child should be an inset frame");
    };
    assert_eq!(inset.parent_key.as_deref(), Some("Inset"));
    assert_eq!(inset.inherits.as_deref(), Some("InsetFrameTemplate"));
}
