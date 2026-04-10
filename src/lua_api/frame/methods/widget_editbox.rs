//! EditBox widget methods: focus, cursor, text input, history, insets.

use super::super::handle::FrameRef;
use super::widget_tooltip::{fire_tooltip_script, val_to_f32};
use crate::lua_api::frame::handle::get_sim_state;

const EDITBOX_VARIADIC_STUBS: &[&str] = &["ClearHighlightText", "SetHighlightColor"];

const EDITBOX_FALSE_GETTERS: &[&str] = &[
    "GetIndentedWordWrap",
    "HasText",
    "IsCountInvisibleLetters",
    "IsInIMECompositionMode",
];

pub fn add_editbox_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_editbox_focus_methods(methods);
    add_editbox_cursor_methods(methods);
    add_editbox_number_methods(methods);
    add_editbox_limit_methods(methods);
    add_editbox_mode_flags(methods);
    add_editbox_input_flags(methods);
    add_editbox_history_methods(methods);
    add_editbox_inset_methods(methods);
    methods.add_method("SetSecurityDisableSetText", |_, _this, ()| Ok(()));
    add_editbox_language_methods(methods);
    add_editbox_stub_methods(methods);
}

fn add_editbox_stub_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_editbox_variadic_stubs(methods, EDITBOX_VARIADIC_STUBS);
    add_editbox_false_getters(methods, EDITBOX_FALSE_GETTERS);
    add_editbox_i32_getter(methods, "GetUTF8CursorPosition", 0);
    methods.add_method("GetDisplayText", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .and_then(|frame| frame.text.clone())
            .unwrap_or_default())
    });
    methods.add_method("GetHighlightColor", |_, _this, ()| {
        Ok((1.0f64, 1.0f64, 1.0f64, 1.0f64))
    });
}

fn add_editbox_variadic_stubs<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    names: &[&'static str],
) {
    for name in names {
        methods.add_method(*name, |_, _this, _: mlua::Variadic<mlua::Value>| Ok(()));
    }
}

fn add_editbox_false_getters<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    names: &[&'static str],
) {
    for name in names {
        methods.add_method(*name, |_, _this, ()| Ok(false));
    }
}

fn add_editbox_i32_getter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    value: i32,
) {
    methods.add_method(name, move |_, _this, ()| Ok(value));
}

fn add_editbox_focus_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFocus", |lua, this, ()| {
        if let Some(old_focus) = set_editbox_focus(lua, this.0) {
            dispatch_focus_gain(lua, this.0, old_focus)?;
        }
        Ok(())
    });

    methods.add_method("ClearFocus", |lua, this, ()| {
        if clear_editbox_focus(lua, this.0) {
            fire_focus_handler(lua, this.0, "OnEditFocusLost")?;
        }
        Ok(())
    });

    methods.add_method("HasFocus", |lua, this, ()| {
        Ok(editbox_has_focus(lua, this.0))
    });
}

fn set_editbox_focus(lua: &mlua::Lua, id: u64) -> Option<Option<u64>> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let old_focus = state.focused_frame_id;
    state.focused_frame_id = Some(id);
    if old_focus == Some(id) {
        return None;
    }
    Some(old_focus)
}

fn dispatch_focus_gain(lua: &mlua::Lua, id: u64, old_focus: Option<u64>) -> mlua::Result<()> {
    if let Some(old_id) = old_focus {
        fire_focus_handler(lua, old_id, "OnEditFocusLost")?;
    }
    fire_focus_handler(lua, id, "OnEditFocusGained")?;
    Ok(())
}

fn clear_editbox_focus(lua: &mlua::Lua, id: u64) -> bool {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if state.focused_frame_id == Some(id) {
        state.focused_frame_id = None;
        true
    } else {
        false
    }
}

fn editbox_has_focus(lua: &mlua::Lua, id: u64) -> bool {
    let state_rc = get_sim_state(lua);
    if let Ok(state) = state_rc.try_borrow() {
        return state.focused_frame_id == Some(id);
    }
    false
}

fn add_editbox_cursor_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_cursor_position(methods);
    add_get_cursor_position(methods);
    methods.add_method("HighlightText", |_, _this, _args: mlua::MultiValue| Ok(()));
    add_insert_text(methods);
    add_get_num_letters(methods);
}

fn add_set_cursor_position<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCursorPosition", |lua, this, pos: i32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_cursor_pos = clamp_cursor_position(frame, pos);
        }
        Ok(())
    });
}

fn add_get_cursor_position<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetCursorPosition", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_cursor_pos)
            .unwrap_or(0))
    });
}

fn add_insert_text<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Insert", |lua, this, text: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            insert_editbox_text(frame, &text);
        }
        Ok(())
    });
}

fn add_get_num_letters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetNumLetters", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(editbox_text_len(&state, this.0) as i32)
    });
}

fn clamp_cursor_position(frame: &crate::widget::Frame, pos: i32) -> i32 {
    pos.clamp(0, editbox_char_count(frame))
}

fn editbox_char_count(frame: &crate::widget::Frame) -> i32 {
    frame.text.as_deref().unwrap_or("").chars().count() as i32
}

fn insert_editbox_text(frame: &mut crate::widget::Frame, text: &str) {
    let pos = frame.editbox_cursor_pos.max(0) as usize;
    let current = frame.text.get_or_insert_with(String::new);
    let insert_at = pos.min(current.len());
    current.insert_str(insert_at, text);
    frame.editbox_cursor_pos = (insert_at + text.len()) as i32;
}

fn editbox_text_len(state: &crate::lua_api::SimState, id: u64) -> usize {
    state
        .widgets
        .get(id)
        .and_then(|frame| frame.text.as_ref())
        .map(|text| text.chars().count())
        .unwrap_or(0)
}

fn add_editbox_number_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetNumber", |lua, this, n: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let s = n.to_string();
            frame.text_stripped = Some(s.clone());
            frame.text = Some(s);
        }
        Ok(())
    });

    methods.add_method("GetNumber", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0)
            && let Some(text) = &frame.text
        {
            return Ok(text.parse::<f64>().unwrap_or(0.0));
        }
        Ok(0.0)
    });
}

fn add_editbox_limit_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_editbox_i32_setter(methods, "SetMaxLetters", |frame, max| {
        frame.editbox_max_letters = max
    });
    add_editbox_i32_getter_from_frame(methods, "GetMaxLetters", |frame| frame.editbox_max_letters);
    add_editbox_i32_setter(methods, "SetMaxBytes", |frame, max| {
        frame.editbox_max_bytes = max
    });
    add_editbox_i32_getter_from_frame(methods, "GetMaxBytes", |frame| frame.editbox_max_bytes);
}

fn add_editbox_i32_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::widget::Frame, i32) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, value: i32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            setter(frame, value);
        }
        Ok(())
    });
}

fn add_editbox_i32_getter_from_frame<M, F>(methods: &mut M, name: &'static str, getter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::widget::Frame) -> i32 + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(getter).unwrap_or(0))
    });
}

fn add_editbox_mode_flags<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_editbox_bool_setter(methods, "SetMultiLine", |frame, value| {
        frame.editbox_multi_line = value
    });
    add_editbox_bool_getter(methods, "IsMultiLine", |frame| frame.editbox_multi_line);
    add_editbox_bool_setter(methods, "SetAutoFocus", |frame, value| {
        frame.editbox_auto_focus = value
    });
    add_editbox_bool_getter(methods, "IsAutoFocus", |frame| frame.editbox_auto_focus);
    add_editbox_bool_setter(methods, "SetNumeric", |frame, value| {
        frame.editbox_numeric = value
    });
    add_editbox_bool_getter(methods, "IsNumeric", |frame| frame.editbox_numeric);
    add_editbox_bool_setter(methods, "SetAlphabeticOnly", |frame, value| {
        frame.editbox_alphabetic_only = value
    });
    add_editbox_bool_getter(methods, "IsAlphabeticOnly", |frame| {
        frame.editbox_alphabetic_only
    });
    add_editbox_bool_setter(methods, "SetAltArrowKeyMode", |frame, value| {
        frame.editbox_alt_arrow_key_mode = value
    });
    add_editbox_bool_getter(methods, "GetAltArrowKeyMode", |frame| {
        frame.editbox_alt_arrow_key_mode
    });
    add_editbox_bool_setter(methods, "SetNumericFullRange", |frame, value| {
        frame.editbox_numeric_full_range = value
    });
    add_editbox_bool_getter(methods, "IsNumericFullRange", |frame| {
        frame.editbox_numeric_full_range
    });
}

fn add_editbox_bool_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::widget::Frame, bool) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, value: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            setter(frame, value);
        }
        Ok(())
    });
}

fn add_editbox_bool_getter<M, F>(methods: &mut M, name: &'static str, getter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::widget::Frame) -> bool + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(getter).unwrap_or(false))
    });
}

fn add_editbox_input_flags<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_editbox_bool_setter(methods, "SetPassword", |frame, value| {
        frame.editbox_password = value
    });
    add_editbox_bool_getter(methods, "IsPassword", |frame| frame.editbox_password);
    add_editbox_f64_setter(methods, "SetBlinkSpeed", |frame, value| {
        frame.editbox_blink_speed = value
    });
    add_editbox_f64_getter(methods, "GetBlinkSpeed", 0.5, |frame| {
        frame.editbox_blink_speed
    });
    add_editbox_bool_setter(methods, "SetCountInvisibleLetters", |frame, value| {
        frame.editbox_count_invisible_letters = value
    });
    add_editbox_bool_setter(methods, "SetSecureText", |frame, value| {
        frame.editbox_secure_text = value
    });
    add_editbox_bool_getter(methods, "IsSecureText", |frame| frame.editbox_secure_text);
    methods.add_method("SetSecurityDisablePaste", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_security_disable_paste = true;
        }
        Ok(())
    });
    add_editbox_i32_setter(methods, "SetVisibleTextByteLimit", |frame, max| {
        frame.editbox_visible_text_byte_limit = max
    });
    add_editbox_i32_getter_from_frame(methods, "GetVisibleTextByteLimit", |frame| {
        frame.editbox_visible_text_byte_limit
    });
}

fn add_editbox_f64_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::widget::Frame, f64) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, value: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            setter(frame, value);
        }
        Ok(())
    });
}

fn add_editbox_f64_getter<M, F>(methods: &mut M, name: &'static str, default: f64, getter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::widget::Frame) -> f64 + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(getter).unwrap_or(default))
    });
}

fn add_editbox_history_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AddHistoryLine", |lua, this, text: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_history.push(text);
            let max = frame.editbox_history_max;
            if max > 0 && frame.editbox_history.len() > max as usize {
                frame.editbox_history.remove(0);
            }
        }
        Ok(())
    });

    methods.add_method("GetHistoryLines", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let count = state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_history.len())
            .unwrap_or(0);
        Ok(count as i32)
    });

    methods.add_method("SetHistoryLines", |lua, this, max: i32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_history_max = max;
        }
        Ok(())
    });

    methods.add_method("ClearHistory", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_history.clear();
        }
        Ok(())
    });
}

fn add_editbox_language_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetInputLanguage", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.editbox_input_language.clone())
            .unwrap_or_else(|| "ROMAN".to_string()))
    });

    methods.add_method("ToggleInputLanguage", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_input_language = if frame.editbox_input_language == "ROMAN" {
                "NATIVE".to_string()
            } else {
                "ROMAN".to_string()
            };
        }
        Ok(())
    });

    methods.add_method("ResetInputMode", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_input_language = "ROMAN".to_string();
        }
        Ok(())
    });
}

fn add_editbox_inset_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetTextInsets", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let l = val_to_f32(it.next(), 0.0);
        let r = val_to_f32(it.next(), 0.0);
        let t = val_to_f32(it.next(), 0.0);
        let b = val_to_f32(it.next(), 0.0);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_text_insets = (l, r, t, b);
        }
        Ok(())
    });

    methods.add_method("GetTextInsets", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            return Ok(frame.editbox_text_insets);
        }
        Ok((0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32))
    });
}

fn fire_focus_handler(lua: &mlua::Lua, frame_id: u64, handler: &str) -> mlua::Result<()> {
    fire_tooltip_script(lua, frame_id, handler)
}
