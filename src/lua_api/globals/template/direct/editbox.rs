//! EditBox XML attributes applied base-to-derived before lifecycle scripts.

use crate::lua_api::SimState;
use crate::widget::{Frame, WidgetType};
use crate::xml::FrameXml;
use std::cell::RefCell;
use std::rc::Rc;

pub fn apply_xml_editbox_attributes(
    state: &Rc<RefCell<SimState>>,
    frame_id: u64,
    frame: &FrameXml,
    inherits: &str,
) {
    let chain = crate::xml::get_template_chain(inherits);
    let mut sim = state.borrow_mut();
    let Some(widget) = sim.widgets.get_mut_visual(frame_id) else {
        return;
    };
    for entry in chain.iter() {
        apply_declared_editbox_attributes(widget, &entry.frame);
    }
    apply_declared_editbox_attributes(widget, frame);
}

fn apply_declared_editbox_attributes(widget: &mut Frame, frame: &FrameXml) {
    if let Some(value) = frame.letters {
        widget.editbox_max_letters = value;
    }
    if let Some(value) = frame.bytes {
        widget.editbox_max_bytes = value;
    }
    if crate::client_profile::ACTIVE != crate::client_profile::ClientProfile::Wrath {
        return;
    }
    if let Some(value) = frame.password.as_deref() {
        widget.editbox_password = value == "1";
    }
    if widget.widget_type != WidgetType::EditBox {
        return;
    }
    if let Some(value) = frame.history_lines {
        widget.resize_wrath_history(value.max(0) as usize);
    }
    if let Some(value) = frame.ignore_arrows {
        widget.editbox_alt_arrow_key_mode = value;
    }
}
