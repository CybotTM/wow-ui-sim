//! EditBox widget methods: focus, cursor, text input, history, insets.

use super::super::handle::FrameRef;
use super::widget_tooltip::{fire_tooltip_script, val_to_f32};
use crate::lua_api::frame::handle::get_sim_state;

const EDITBOX_VARIADIC_STUBS: &[&str] = &[
    "ClearHighlightText",
    "ClearHistory",
    "ResetInputMode",
    "SetAlphabeticOnly",
    "SetAltArrowKeyMode",
    "SetHighlightColor",
    "SetNumericFullRange",
    "SetSecureText",
    "SetSecurityDisablePaste",
    "SetVisibleTextByteLimit",
    "ToggleInputLanguage",
];

const EDITBOX_FALSE_GETTERS: &[&str] = &[
    "GetAltArrowKeyMode",
    "GetIndentedWordWrap",
    "HasText",
    "IsAlphabeticOnly",
    "IsCountInvisibleLetters",
    "IsInIMECompositionMode",
    "IsNumericFullRange",
    "IsSecureText",
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
    methods.add_method("GetInputLanguage", |_, _this, ()| Ok("ROMAN"));
    add_editbox_stub_methods(methods);
}

fn add_editbox_stub_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_editbox_variadic_stubs(methods, EDITBOX_VARIADIC_STUBS);
    add_editbox_false_getters(methods, EDITBOX_FALSE_GETTERS);
    add_editbox_i32_getter(methods, "GetUTF8CursorPosition", 0);
    add_editbox_i32_getter(methods, "GetVisibleTextByteLimit", 0);
    methods.add_method("GetDisplayText", |_, _this, ()| Ok("".to_string()));
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
        let id = this.0;
        let old_focus = {
            let state_rc = get_sim_state(lua);
            let mut s = state_rc.borrow_mut();
            let old = s.focused_frame_id;
            s.focused_frame_id = Some(id);
            old
        };
        if old_focus == Some(id) {
            return Ok(());
        }
        if let Some(old_id) = old_focus {
            fire_focus_handler(lua, old_id, "OnEditFocusLost")?;
        }
        fire_focus_handler(lua, id, "OnEditFocusGained")?;
        Ok(())
    });

    methods.add_method("ClearFocus", |lua, this, ()| {
        let id = this.0;
        let had_focus = {
            let state_rc = get_sim_state(lua);
            let mut s = state_rc.borrow_mut();
            if s.focused_frame_id == Some(id) {
                s.focused_frame_id = None;
                true
            } else {
                false
            }
        };
        if had_focus {
            fire_focus_handler(lua, id, "OnEditFocusLost")?;
        }
        Ok(())
    });

    methods.add_method("HasFocus", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(s) = state_rc.try_borrow() {
            return Ok(s.focused_frame_id == Some(this.0));
        }
        Ok(false)
    });
}

fn add_editbox_cursor_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCursorPosition", |lua, this, pos: i32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let char_count = frame.text.as_deref().unwrap_or("").chars().count() as i32;
            frame.editbox_cursor_pos = pos.clamp(0, char_count);
        }
        Ok(())
    });

    methods.add_method("GetCursorPosition", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_cursor_pos)
            .unwrap_or(0))
    });

    methods.add_method("HighlightText", |_, _this, _args: mlua::MultiValue| Ok(()));

    methods.add_method("Insert", |lua, this, text: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let current = frame.text.get_or_insert_with(String::new);
            let pos = (frame.editbox_cursor_pos as usize).min(current.len());
            current.insert_str(pos, &text);
            frame.editbox_cursor_pos = (pos + text.len()) as i32;
        }
        Ok(())
    });

    methods.add_method("GetNumLetters", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let len = state
            .widgets
            .get(this.0)
            .and_then(|f| f.text.as_ref())
            .map(|t| t.chars().count())
            .unwrap_or(0);
        Ok(len as i32)
    });
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
    methods.add_method("SetMaxLetters", |lua, this, max: i32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_max_letters = max;
        }
        Ok(())
    });

    methods.add_method("GetMaxLetters", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_max_letters)
            .unwrap_or(0))
    });

    methods.add_method("SetMaxBytes", |lua, this, max: i32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_max_bytes = max;
        }
        Ok(())
    });

    methods.add_method("GetMaxBytes", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_max_bytes)
            .unwrap_or(0))
    });
}

fn add_editbox_mode_flags<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetMultiLine", |lua, this, multi: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_multi_line = multi;
        }
        Ok(())
    });
    methods.add_method("IsMultiLine", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_multi_line)
            .unwrap_or(false))
    });
    methods.add_method("SetAutoFocus", |lua, this, auto: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_auto_focus = auto;
        }
        Ok(())
    });
    methods.add_method("IsAutoFocus", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_auto_focus)
            .unwrap_or(false))
    });
    methods.add_method("SetNumeric", |lua, this, numeric: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_numeric = numeric;
        }
        Ok(())
    });
    methods.add_method("IsNumeric", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_numeric)
            .unwrap_or(false))
    });
}

fn add_editbox_input_flags<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetPassword", |lua, this, pw: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_password = pw;
        }
        Ok(())
    });
    methods.add_method("IsPassword", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_password)
            .unwrap_or(false))
    });
    methods.add_method("SetBlinkSpeed", |lua, this, speed: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_blink_speed = speed;
        }
        Ok(())
    });
    methods.add_method("GetBlinkSpeed", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.editbox_blink_speed)
            .unwrap_or(0.5))
    });
    methods.add_method("SetCountInvisibleLetters", |lua, this, count: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.editbox_count_invisible_letters = count;
        }
        Ok(())
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
