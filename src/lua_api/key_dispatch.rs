//! Key press dispatch: Escape handling, OnKeyDown propagation, GameMenuFrame
//! toggle, and EditBox text editing.
//!
//! This is a rilua port of the master-era `key_dispatch.rs`. The public entry
//! point is `WowLuaEnv::send_key_press`.
//!
//! Architectural differences from master:
//! - rilua has no `Val::Int` — numeric values are `Val::Num(f64)`.
//! - rilua's `fire_script_handler` does not return values; a dedicated
//!   `fire_handler_returns_truthy` calls the handler directly and captures
//!   the return.
//! - Keybinding dispatch is backed by `SimState.keybindings` (not registry
//!   tables), via `dispatch_key_binding` in the keybindings module.
//! - There is no `fire_on_show_recursive`; GameMenuFrame's OnShow is fired
//!   via the existing `fire_script_handler` helper.

use std::ops::Range;

use crate::Result;
use crate::lua_api::methods::{call_function, create_string, frame_ref};
use crate::lua_api::script_helpers::get_script;
use rilua::{LuaApiMut, Val};

use super::env::WowLuaEnv;

// ── Utilities ─────────────────────────────────────────────────────────────────

enum EditboxCursorTarget {
    Delta(i32),
    Home,
    End,
}

/// Return `true` for any Lua value that is truthy (not nil and not false).
fn is_truthy(val: Val) -> bool {
    !matches!(val, Val::Nil | Val::Bool(false))
}

/// Byte range for the character at `char_index` in `s`.
fn char_byte_range(s: &str, char_index: usize) -> Range<usize> {
    let mut chars = s.char_indices();
    let (start, ch) = chars.nth(char_index).unwrap();
    start..start + ch.len_utf8()
}

fn refresh_editbox_render_text(frame: &mut crate::widget::Frame) {
    frame.text_stripped = frame.text.as_deref().map(crate::render::strip_wow_markup);
    frame.text_segments.clear();
}

fn editbox_text_char_count(frame: &crate::widget::Frame) -> i32 {
    frame
        .text
        .as_ref()
        .map(|text| text.chars().count() as i32)
        .unwrap_or(0)
}

fn printable_text_for_editbox_key<'a>(key: &'a str, text: Option<&'a str>) -> Option<String> {
    if let Some(text) = text {
        let printable: String = text.chars().filter(|c| !c.is_control()).collect();
        return (!printable.is_empty()).then_some(printable);
    }
    if key == "SPACE" {
        return Some(" ".to_string());
    }
    if key.chars().count() == 1 {
        return Some(key.to_lowercase());
    }
    None
}

// ── WowLuaEnv impl ───────────────────────────────────────────────────────────

impl WowLuaEnv {
    /// Simulate a key press through WoW's full dispatch chain.
    ///
    /// `text` is the raw Unicode character(s) for typing into a focused
    /// EditBox. Pass `None` for non-printable keys.
    pub fn send_key_press(&self, key: &str, text: Option<&str>) -> Result<()> {
        if key == "ESCAPE" {
            self.dispatch_escape()
        } else {
            self.dispatch_key(key, text)
        }
    }

    // ── Escape ────────────────────────────────────────────────────────────────

    /// Priority: focused EditBox OnEscapePressed → clear target →
    /// CloseSpecialWindows → CloseAllWindows → toggle GameMenuFrame.
    fn dispatch_escape(&self) -> Result<()> {
        let focused = self.state.borrow().focused_frame_id;
        if let Some(fid) = focused {
            if self.fire_handler_returns_truthy(fid, "OnEscapePressed")? {
                return Ok(());
            }
        }
        if self.clear_target_if_any()? {
            return Ok(());
        }
        if self.close_special_windows()? {
            return Ok(());
        }
        if self.close_all_windows()? {
            return Ok(());
        }
        self.toggle_game_menu()
    }

    /// Clear the current target (if any) by calling `ClearTarget()` in Lua.
    /// Returns `true` if a target was cleared.
    fn clear_target_if_any(&self) -> Result<bool> {
        let has_target = self.state.borrow().current_target.is_some();
        if has_target {
            self.exec("ClearTarget()")?;
            return Ok(true);
        }
        Ok(false)
    }

    // ── General key dispatch ──────────────────────────────────────────────────

    /// Dispatch a non-Escape key: special EditBox handlers → keybinding →
    /// OnKeyDown propagation → EditBox text input.
    fn dispatch_key(&self, key: &str, text: Option<&str>) -> Result<()> {
        let focused = self.state.borrow().focused_frame_id;
        if let Some(fid) = focused {
            let special = match key {
                "ENTER" => Some("OnEnterPressed"),
                "TAB" => Some("OnTabPressed"),
                "SPACE" => Some("OnSpacePressed"),
                _ => None,
            };
            if let Some(handler) = special {
                if self.fire_handler_returns_truthy(fid, handler)? {
                    return Ok(());
                }
            }
        }

        let is_editbox = self.focused_is_editbox(focused);
        if !is_editbox {
            let mut lua = self.lua.borrow_mut();
            if super::globals::keybindings::dispatch_key_binding(&mut lua, key)? {
                return Ok(());
            }
        }

        self.dispatch_on_key_down(key)?;

        if let Some(fid) = focused {
            if is_editbox {
                self.dispatch_editbox_key(fid, key, text)?;
            }
        }

        Ok(())
    }

    /// `true` if `focused` refers to an EditBox widget.
    fn focused_is_editbox(&self, focused: Option<u64>) -> bool {
        focused.is_some_and(|fid| {
            self.state
                .borrow()
                .widgets
                .get(fid)
                .map(|f| f.widget_type == crate::widget::WidgetType::EditBox)
                .unwrap_or(false)
        })
    }

    /// Dispatch a key event to a focused EditBox.
    fn dispatch_editbox_key(&self, fid: u64, key: &str, text: Option<&str>) -> Result<()> {
        match key {
            "BACKSPACE" => self.editbox_backspace(fid)?,
            "DELETE" => self.editbox_delete(fid)?,
            "LEFT" => self.editbox_move_cursor(fid, -1)?,
            "RIGHT" => self.editbox_move_cursor(fid, 1)?,
            "HOME" => self.editbox_cursor_home(fid)?,
            "END" => self.editbox_cursor_end(fid)?,
            _ => {
                if let Some(printable) = printable_text_for_editbox_key(key, text) {
                    self.editbox_insert_text(fid, &printable)?;
                }
            }
        }
        Ok(())
    }

    // ── OnKeyDown propagation ─────────────────────────────────────────────────

    /// Fire OnKeyDown starting from the focused frame (or the first
    /// keyboard-enabled visible frame if nothing is focused).
    fn dispatch_on_key_down(&self, key: &str) -> Result<()> {
        let start_id = {
            let state = self.state.borrow();
            state.focused_frame_id.or_else(|| {
                state.widgets.iter_ids().find(|&id| {
                    state
                        .widgets
                        .get(id)
                        .map(|f| f.keyboard_enabled && f.visible)
                        .unwrap_or(false)
                })
            })
        };
        let Some(frame_id) = start_id else {
            return Ok(());
        };
        self.fire_on_key_down(frame_id, key)
    }

    /// Fire OnKeyDown on a frame; if `propagate_keyboard_input` is set, walk
    /// up the parent chain.
    fn fire_on_key_down(&self, frame_id: u64, key: &str) -> Result<()> {
        let key_val = {
            let mut lua = self.lua.borrow_mut();
            create_string(lua.state_mut(), key)
        };
        self.fire_script_handler(frame_id, "OnKeyDown", vec![key_val])?;
        let propagate = self
            .state
            .borrow()
            .widgets
            .get(frame_id)
            .map(|f| f.propagate_keyboard_input)
            .unwrap_or(false);
        if propagate {
            let parent = self
                .state
                .borrow()
                .widgets
                .get(frame_id)
                .and_then(|f| f.parent_id);
            if let Some(pid) = parent {
                return self.fire_on_key_down(pid, key);
            }
        }
        Ok(())
    }

    // ── Handler-returns-truthy ────────────────────────────────────────────────

    /// Fire a script handler and return whether it returned a truthy value.
    /// Uses a direct call (not through `call_widget_handler`) to capture
    /// the return value.
    pub(crate) fn fire_handler_returns_truthy(
        &self,
        widget_id: u64,
        handler_name: &str,
    ) -> Result<bool> {
        let mut lua = self.lua.borrow_mut();
        let handler = get_script(lua.state_mut(), widget_id, handler_name);
        let Some(handler) = handler else {
            return Ok(false);
        };
        let frame_arg = frame_ref(lua.state_mut(), widget_id)?;
        let result = call_function(&mut lua, handler, &[frame_arg])?;
        Ok(is_truthy(result))
    }

    // ── Special-window helpers ────────────────────────────────────────────────

    /// Iterate `UISpecialFrames` and hide any visible frames. Returns `true`
    /// if at least one was closed.
    fn close_special_windows(&self) -> Result<bool> {
        let names = self.read_special_frame_names()?;
        Ok(self.hide_visible_frames(&names))
    }

    /// Read the `UISpecialFrames` sequence from Lua globals.
    fn read_special_frame_names(&self) -> Result<Vec<String>> {
        let mut lua = self.lua.borrow_mut();
        let state = lua.state_mut();
        let global_key = state.gc.intern_string_static(b"UISpecialFrames");
        let global = state
            .gc
            .tables
            .get(state.global)
            .map(|t| t.get_str(global_key, &state.gc.string_arena))
            .unwrap_or(Val::Nil);
        let Val::Table(tref) = global else {
            return Ok(vec![]);
        };
        let Some(table) = state.gc.tables.get(tref) else {
            return Ok(vec![]);
        };
        let len = table.len(&state.gc.string_arena);
        let mut out = Vec::with_capacity(len);
        for i in 1..=len {
            let v = table.get_int(i as i64);
            if let Val::Str(s) = v {
                if let Some(ls) = state.gc.string_arena.get(s) {
                    out.push(String::from_utf8_lossy(ls.data()).into_owned());
                }
            }
        }
        Ok(out)
    }

    /// Hide each named frame that is currently visible. Returns `true` if any
    /// frame was hidden.
    fn hide_visible_frames(&self, names: &[String]) -> bool {
        let mut closed = false;
        for name in names {
            let id = self.state.borrow().widgets.get_id_by_name(name);
            if let Some(id) = id {
                let is_visible = self
                    .state
                    .borrow()
                    .widgets
                    .get(id)
                    .map(|f| f.visible)
                    .unwrap_or(false);
                if is_visible {
                    self.state.borrow_mut().set_frame_visible(id, false);
                    closed = true;
                }
            }
        }
        closed
    }

    /// Call the Lua `CloseAllWindows()` global. Returns `false` if it is not
    /// defined (e.g. in unit tests).
    fn close_all_windows(&self) -> Result<bool> {
        let func: Val = {
            let mut lua = self.lua.borrow_mut();
            let state = lua.state_mut();
            let key = state.gc.intern_string_static(b"CloseAllWindows");
            state
                .gc
                .tables
                .get(state.global)
                .map(|t| t.get_str(key, &state.gc.string_arena))
                .unwrap_or(Val::Nil)
        };
        if matches!(func, Val::Nil) {
            return Ok(false);
        }
        let mut lua = self.lua.borrow_mut();
        let result = call_function(&mut lua, func, &[])?;
        Ok(is_truthy(result))
    }

    // ── GameMenuFrame toggle ──────────────────────────────────────────────────

    /// Toggle the GameMenuFrame: show it if hidden, hide it if visible.
    fn toggle_game_menu(&self) -> Result<()> {
        let id = self.state.borrow().widgets.get_id_by_name("GameMenuFrame");
        let Some(id) = id else {
            return Ok(());
        };
        let is_visible = self
            .state
            .borrow()
            .widgets
            .get(id)
            .map(|f| f.visible)
            .unwrap_or(false);
        if is_visible {
            self.state.borrow_mut().set_frame_visible(id, false);
        } else {
            self.state.borrow_mut().set_frame_visible(id, true);
            self.fire_script_handler(id, "OnShow", vec![])?;
        }
        Ok(())
    }

    // ── EditBox text editing ──────────────────────────────────────────────────

    /// Insert `text` at the cursor position, fire OnChar and OnTextChanged.
    fn editbox_insert_text(&self, fid: u64, text: &str) -> Result<()> {
        let numeric = self
            .state
            .borrow()
            .widgets
            .get(fid)
            .map(|f| f.editbox_numeric)
            .unwrap_or(false);
        let is_valid_numeric = text
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == '-');
        if numeric && !is_valid_numeric {
            return Ok(());
        }
        self.splice_text_at_cursor(fid, text);
        self.fire_char_events(fid, text)?;
        self.fire_script_handler(fid, "OnTextChanged", vec![Val::Bool(true)])?;
        Ok(())
    }

    /// Write `text` into the frame's text buffer at the cursor position and
    /// advance the cursor by the number of characters inserted.
    fn splice_text_at_cursor(&self, fid: u64, text: &str) {
        let mut state = self.state.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(fid) {
            let current = frame.text.get_or_insert_with(String::new);
            let char_pos = frame.editbox_cursor_pos as usize;
            let byte_pos = current
                .char_indices()
                .nth(char_pos)
                .map(|(i, _)| i)
                .unwrap_or(current.len());
            current.insert_str(byte_pos, text);
            frame.editbox_cursor_pos += text.chars().count() as i32;
            refresh_editbox_render_text(frame);
        }
    }

    /// Fire `OnChar` for each character in `text`.
    fn fire_char_events(&self, fid: u64, text: &str) -> Result<()> {
        for ch in text.chars() {
            let ch_str = ch.to_string();
            let char_val = {
                let mut lua = self.lua.borrow_mut();
                create_string(lua.state_mut(), &ch_str)
            };
            self.fire_script_handler(fid, "OnChar", vec![char_val])?;
        }
        Ok(())
    }

    /// Delete the character before the cursor (Backspace).
    fn editbox_backspace(&self, fid: u64) -> Result<()> {
        let changed = {
            let mut state = self.state.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(fid) {
                let current = frame.text.get_or_insert_with(String::new);
                let char_pos = frame.editbox_cursor_pos as usize;
                if char_pos > 0 {
                    let byte_range = char_byte_range(current, char_pos - 1);
                    current.drain(byte_range);
                    frame.editbox_cursor_pos -= 1;
                    refresh_editbox_render_text(frame);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if changed {
            self.fire_script_handler(fid, "OnTextChanged", vec![Val::Bool(true)])?;
        }
        Ok(())
    }

    /// Delete the character after the cursor (Delete key).
    fn editbox_delete(&self, fid: u64) -> Result<()> {
        let changed = {
            let mut state = self.state.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(fid) {
                let current = frame.text.get_or_insert_with(String::new);
                let char_pos = frame.editbox_cursor_pos as usize;
                let char_count = current.chars().count();
                if char_pos < char_count {
                    let byte_range = char_byte_range(current, char_pos);
                    current.drain(byte_range);
                    refresh_editbox_render_text(frame);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if changed {
            self.fire_script_handler(fid, "OnTextChanged", vec![Val::Bool(true)])?;
        }
        Ok(())
    }

    /// Move the cursor by `delta` characters (negative = left, positive = right).
    fn editbox_move_cursor(&self, fid: u64, delta: i32) -> Result<()> {
        self.set_editbox_cursor(fid, EditboxCursorTarget::Delta(delta))
    }

    fn set_editbox_cursor(&self, fid: u64, target: EditboxCursorTarget) -> Result<()> {
        let mut state = self.state.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(fid) {
            let char_count = editbox_text_char_count(frame);
            frame.editbox_cursor_pos = match target {
                EditboxCursorTarget::Delta(delta) => {
                    (frame.editbox_cursor_pos + delta).clamp(0, char_count)
                }
                EditboxCursorTarget::Home => 0,
                EditboxCursorTarget::End => char_count,
            };
        }
        Ok(())
    }

    /// Move the cursor to the start of text (Home key).
    fn editbox_cursor_home(&self, fid: u64) -> Result<()> {
        self.set_editbox_cursor(fid, EditboxCursorTarget::Home)
    }

    /// Move the cursor to the end of text (End key).
    fn editbox_cursor_end(&self, fid: u64) -> Result<()> {
        self.set_editbox_cursor(fid, EditboxCursorTarget::End)
    }
}
