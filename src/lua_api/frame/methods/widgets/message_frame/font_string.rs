//! FontString lifecycle: resolve, create, update for GetFontStringByID.

use crate::lua_api::message_frame::Message;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack, frame_ref};
use crate::lua_bridge::stack_val;
use crate::widget::{Frame, WidgetType};
use rilua::{LuaResult, Val};
use rilua::vm::state::LuaState;

use super::super::shared::val_to_f64;

pub(super) fn get_font_string_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let msg_id = val_to_f64(stack_val(state, 2)) as i64;
    let (font_string_id, message) = match resolve_message_font_string(state, id, msg_id) {
        Some(pair) => pair,
        None => {
            state.push(Val::Nil);
            return Ok(1);
        }
    };
    update_message_font_string(state, font_string_id, &message);
    let val = frame_ref(state, font_string_id)?;
    state.push(val);
    Ok(1)
}

fn resolve_message_font_string(
    state: &mut LuaState,
    frame_id: u64,
    message_id: i64,
) -> Option<(u64, Message)> {
    let (message, existing_fs_id) = {
        let sim = borrow_state(state).ok()?;
        let data = sim.message_frames.get(&frame_id)?;
        let message = data
            .messages
            .iter()
            .rev()
            .find(|m| m.message_id == Some(message_id))
            .cloned()?;
        let existing = data.message_font_strings.get(&message_id).copied();
        (message, existing)
    };

    let font_string_id = existing_fs_id.unwrap_or_else(|| {
        let mut sim = borrow_state_mut(state).unwrap();
        create_message_font_string(&mut sim, frame_id, message_id)
    });

    {
        let mut sim = borrow_state_mut(state).ok()?;
        sim.message_frames
            .get_mut(&frame_id)?
            .message_font_strings
            .insert(message_id, font_string_id);
    }
    Some((font_string_id, message))
}

fn create_message_font_string(
    sim: &mut crate::lua_api::SimState,
    parent_id: u64,
    message_id: i64,
) -> u64 {
    let mut font_string = Frame::new(WidgetType::FontString, None, Some(parent_id));
    font_string.visible = false;
    font_string.object_type_name = Some("FontString".to_string());
    font_string.parent_key = Some(format!("MessageID{message_id}"));
    let fs_id = font_string.id;
    sim.widgets.register(font_string);
    sim.widgets.add_child(parent_id, fs_id);
    let parent_props = sim
        .widgets
        .get(parent_id)
        .map(|p| (p.frame_strata, p.frame_level));
    if let Some((strata, level)) = parent_props
        && let Some(frame) = sim.widgets.get_mut_visual(fs_id)
    {
        frame.frame_strata = strata;
        frame.frame_level = level + 1;
    }
    fs_id
}

fn update_message_font_string(
    state: &mut LuaState,
    font_string_id: u64,
    message: &Message,
) {
    let mut sim = match borrow_state_mut(state) {
        Ok(s) => s,
        Err(_) => return,
    };
    let Some(frame) = sim.widgets.get_mut_visual(font_string_id) else {
        return;
    };
    frame.text = Some(message.text.clone());
    frame.text_stripped = Some(crate::dump::strip_wow_escapes(&message.text));
    frame.text_color = crate::widget::Color::new(message.r, message.g, message.b, message.a);
}
