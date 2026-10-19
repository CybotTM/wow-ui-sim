//! Wrath engine-side arrows: history and ordinary newline cursor motion.
//! Wrapped visual-line/sticky-x behavior is not established by this slice.

use super::{WowLuaEnv, editbox_byte_range};
use crate::Result;
use crate::lua_api::frame::methods::text_attribute_event::set_frame_text_value;
use rilua::LuaApiMut;
use std::ops::Range;

impl WowLuaEnv {
    pub(super) fn dispatch_wrath_editbox_arrow(&self, fid: u64, arrow: &str) -> Result<()> {
        match arrow {
            "LEFT" => self.editbox_move_cursor(fid, -1),
            "RIGHT" => self.editbox_move_cursor(fid, 1),
            "UP" | "DOWN" => self.dispatch_wrath_vertical_arrow(fid, arrow == "UP"),
            _ => Ok(()),
        }
    }

    fn dispatch_wrath_vertical_arrow(&self, fid: u64, backward: bool) -> Result<()> {
        let recalled = {
            let mut sim = self.state.borrow_mut();
            let Some(frame) = sim.widgets.get_mut_visual(fid) else {
                return Ok(());
            };
            if frame.editbox_multi_line {
                let text = frame.text.as_deref().unwrap_or_default();
                let cursor = frame.editbox_cursor_pos.max(0) as usize;
                frame.editbox_cursor_pos = line_cursor_target(text, cursor, backward) as i32;
                None
            } else {
                frame.recall_wrath_history(backward)
            }
        };
        if let Some(text) = recalled {
            set_frame_text_value(self.rilua_mut().state_mut(), fid, Some(text), false)?;
        }
        Ok(())
    }
}

fn adjacent_line(text: &str, current: Range<usize>, backward: bool) -> Option<Range<usize>> {
    if backward {
        let end = current.start.checked_sub(1)?;
        let start = text[..end].rfind('\n').map_or(0, |position| position + 1);
        return Some(start..end);
    }
    if current.end == text.len() {
        return None;
    }
    let start = current.end + 1;
    let end = text[start..]
        .find('\n')
        .map_or(text.len(), |position| start + position);
    Some(start..end)
}

fn line_cursor_target(text: &str, cursor: usize, backward: bool) -> usize {
    let byte = editbox_byte_range(text, cursor..cursor).start;
    let start = text[..byte].rfind('\n').map_or(0, |position| position + 1);
    let end = text[byte..]
        .find('\n')
        .map_or(text.len(), |position| byte + position);
    let column = text[start..byte].chars().count();
    let Some(target) = adjacent_line(text, start..end, backward) else {
        return cursor;
    };
    let target_byte = text[target.clone()]
        .char_indices()
        .nth(column)
        .map_or(target.end, |(offset, _)| target.start + offset);
    text[..target_byte].chars().count()
}
