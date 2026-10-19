//! Wrath MessageFrame XML lifetime and insertion policy.

use crate::lua_api::SimState;
use crate::xml::FrameXml;
use std::cell::RefCell;
use std::rc::Rc;

pub fn apply_xml_message_frame_attributes(
    state: &Rc<RefCell<SimState>>,
    frame_id: u64,
    frame: &FrameXml,
    inherits: &str,
) {
    if crate::client_profile::ACTIVE != crate::client_profile::ClientProfile::Wrath {
        return;
    }
    let chain = crate::xml::get_template_chain(inherits);
    let duration = frame.display_duration.or_else(|| {
        chain
            .iter()
            .rev()
            .find_map(|entry| entry.frame.display_duration)
    });
    let mode = frame.insert_mode.as_deref().or_else(|| {
        chain
            .iter()
            .rev()
            .find_map(|entry| entry.frame.insert_mode.as_deref())
    });
    let mut sim = state.borrow_mut();
    if !sim
        .widgets
        .get(frame_id)
        .is_some_and(|widget| widget.widget_type == crate::widget::WidgetType::MessageFrame)
    {
        return;
    }
    let data = sim.message_frames.entry(frame_id).or_default();
    if let Some(duration) = duration {
        data.time_visible = duration;
    }
    if let Some(mode) = mode {
        data.insert_mode = mode.to_ascii_uppercase();
    }
}
