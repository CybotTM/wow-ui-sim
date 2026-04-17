//! EditBox and text-spacing widget methods.

use super::shared::{opt_bool, opt_string, val_to_bool, val_to_f64};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, frame_id_from_stack};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub(super) fn set_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let old_focus = {
        let mut sim = borrow_state_mut(state)?;
        let old = sim.focused_frame_id;
        sim.focused_frame_id = Some(id);
        old
    };
    if old_focus != Some(id) {
        // TODO: fire OnEditFocusLost on old_focus, OnEditFocusGained on id
    }
    Ok(0)
}

pub(super) fn clear_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let cleared = {
        let mut sim = borrow_state_mut(state)?;
        if sim.focused_frame_id == Some(id) {
            sim.focused_frame_id = None;
            true
        } else {
            false
        }
    };
    if cleared {
        // TODO: fire OnEditFocusLost
    }
    Ok(0)
}

pub(super) fn has_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.focused_frame_id == Some(id);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn has_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.text.as_ref())
        .is_some_and(|t| !t.is_empty());
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_cursor_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let pos = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        let len = f.text.as_deref().unwrap_or("").chars().count() as i32;
        f.editbox_cursor_pos = pos.clamp(0, len);
    }
    Ok(0)
}

pub(super) fn get_cursor_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_cursor_pos)
        .unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

pub(super) fn get_num_letters(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.text.as_ref())
        .map(|t| t.chars().count())
        .unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

pub(super) fn set_max_letters(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let max = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_max_letters = max;
    }
    Ok(0)
}

pub(super) fn get_max_letters(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_max_letters)
        .unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

pub(super) fn set_multi_line(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_multi_line = value;
    }
    Ok(0)
}

pub(super) fn is_multi_line(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_multi_line)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_auto_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_auto_focus = value;
    }
    Ok(0)
}

pub(super) fn is_auto_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_auto_focus)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_numeric(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_numeric = value;
    }
    Ok(0)
}

pub(super) fn is_numeric(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_numeric)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_password(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_password = value;
    }
    Ok(0)
}

pub(super) fn is_password(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_password)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_number(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let n = val_to_f64(stack_val(state, 2));
    let s = n.to_string();
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.text_stripped = Some(s.clone());
        f.text = Some(s);
    }
    Ok(0)
}

pub(super) fn get_number(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.text.as_ref())
        .and_then(|t| t.parse::<f64>().ok())
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn add_history_line(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = opt_string(state, 2).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_history.push(text);
        let max = f.editbox_history_max;
        if max > 0 && f.editbox_history.len() > max as usize {
            f.editbox_history.remove(0);
        }
    }
    Ok(0)
}

pub(super) fn get_history_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_history.len())
        .unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

pub(super) fn set_history_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let max = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_history_max = max;
    }
    Ok(0)
}

pub(super) fn clear_history(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_history.clear();
    }
    Ok(0)
}

pub(super) fn get_input_language(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let lang = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.editbox_input_language.clone())
            .unwrap_or_else(|| "ROMAN".to_string())
    };
    let val = create_string(state, &lang);
    val.into_stack(state)
}

pub(super) fn toggle_input_language(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_input_language = if f.editbox_input_language == "ROMAN" {
            "NATIVE".to_string()
        } else {
            "ROMAN".to_string()
        };
    }
    Ok(0)
}

pub(super) fn reset_input_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_input_language = "ROMAN".to_string();
    }
    Ok(0)
}

pub(super) fn set_text_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let l = val_to_f64(stack_val(state, 2)) as f32;
    let r = val_to_f64(stack_val(state, 3)) as f32;
    let t = val_to_f64(stack_val(state, 4)) as f32;
    let b = val_to_f64(stack_val(state, 5)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_text_insets = (l, r, t, b);
    }
    Ok(0)
}

pub(super) fn get_text_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (l, r, t, b) = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_text_insets)
        .unwrap_or((0.0, 0.0, 0.0, 0.0));
    drop(sim);
    (l as f64, r as f64, t as f64, b as f64).into_stack(state)
}

/// `SetSpacing(spacing)` — FontString/EditBox line spacing.
pub(super) fn set_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let spacing = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.text_line_spacing = spacing;
    }
    Ok(0)
}

/// `GetSpacing()` — returns the spacing stored by `SetSpacing`.
pub(super) fn get_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let spacing = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.text_line_spacing)
            .unwrap_or(0.0)
    };
    (spacing as f64).into_stack(state)
}

pub(super) fn get_display_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|f| f.text.clone())
            .unwrap_or_default()
    };
    let val = create_string(state, &text);
    val.into_stack(state)
}

pub(super) fn insert(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = opt_string(state, 2).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        let pos = f.editbox_cursor_pos.max(0) as usize;
        let current = f.text.get_or_insert_with(String::new);
        let insert_at = pos.min(current.len());
        current.insert_str(insert_at, &text);
        f.editbox_cursor_pos = (insert_at + text.len()) as i32;
    }
    Ok(0)
}

pub(super) fn set_blink_speed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let speed = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_blink_speed = speed;
    }
    Ok(0)
}

pub(super) fn get_blink_speed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_blink_speed)
        .unwrap_or(0.5);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_alt_arrow_key_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_alt_arrow_key_mode = value;
    }
    Ok(0)
}

pub(super) fn get_alt_arrow_key_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_alt_arrow_key_mode)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn highlight_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let start = val_to_f64(stack_val(state, 2)) as i32;
    let end_raw = stack_val(state, 3);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        let len = f.text.as_deref().unwrap_or("").chars().count() as i32;
        let end = match end_raw {
            Val::Num(n) => n as i32,
            _ => len,
        };
        let (s, e) = normalize_highlight_range(start, end, len);
        f.editbox_highlight_range = Some((s, e));
    }
    Ok(0)
}

pub(super) fn clear_highlight_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_highlight_range = None;
    }
    Ok(0)
}

fn normalize_highlight_range(start: i32, end: i32, len: i32) -> (i32, i32) {
    let s = start.clamp(0, len);
    let e = end.clamp(0, len);
    if s <= e { (s, e) } else { (e, s) }
}

// ---------------------------------------------------------------------------
// register_editbox
// ---------------------------------------------------------------------------

const EDITBOX_METHODS: &[(&str, rilua::vm::closure::RustFn)] = &[
    // Focus state
    ("SetFocus", set_focus),
    ("ClearFocus", clear_focus),
    ("HasFocus", has_focus),
    ("HasText", has_text),
    // Cursor + length
    ("SetCursorPosition", set_cursor_position),
    ("GetCursorPosition", get_cursor_position),
    ("GetNumLetters", get_num_letters),
    ("SetMaxLetters", set_max_letters),
    ("GetMaxLetters", get_max_letters),
    // Mode flags
    ("SetMultiLine", set_multi_line),
    ("IsMultiLine", is_multi_line),
    ("SetAutoFocus", set_auto_focus),
    ("IsAutoFocus", is_auto_focus),
    ("SetNumeric", set_numeric),
    ("IsNumeric", is_numeric),
    ("SetPassword", set_password),
    ("IsPassword", is_password),
    // Numeric helpers
    ("SetNumber", set_number),
    ("GetNumber", get_number),
    // Input history
    ("AddHistoryLine", add_history_line),
    ("GetHistoryLines", get_history_lines),
    ("SetHistoryLines", set_history_lines),
    ("ClearHistory", clear_history),
    // IME / language
    ("GetInputLanguage", get_input_language),
    ("ToggleInputLanguage", toggle_input_language),
    ("ResetInputMode", reset_input_mode),
    // Layout / display
    ("SetTextInsets", set_text_insets),
    ("SetSpacing", set_spacing),
    ("GetSpacing", get_spacing),
    ("GetTextInsets", get_text_insets),
    ("GetDisplayText", get_display_text),
    ("Insert", insert),
    // Cursor blink + nav
    ("SetBlinkSpeed", set_blink_speed),
    ("GetBlinkSpeed", get_blink_speed),
    ("SetAltArrowKeyMode", set_alt_arrow_key_mode),
    ("GetAltArrowKeyMode", get_alt_arrow_key_mode),
    // Selection
    ("HighlightText", highlight_text),
    ("ClearHighlightText", clear_highlight_text),
];

pub(super) fn register_editbox(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    for (name, func) in EDITBOX_METHODS {
        table_set_rust_fn(state, metatable, name, *func)?;
    }
    Ok(())
}
