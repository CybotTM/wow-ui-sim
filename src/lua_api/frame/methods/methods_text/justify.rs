//! SetJustifyH, GetJustifyH, SetJustifyV, GetJustifyV methods.

use super::super::super::handle::{FrameRef, get_sim_state};
use super::{is_simple_html, is_text_type};
use crate::lua_api::simple_html::TextStyle;
use mlua::{Lua, Value};

/// SetJustifyH, GetJustifyH, SetJustifyV, GetJustifyV.
pub fn add_justification_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetJustifyH", |lua, this, args: mlua::MultiValue| {
        apply_set_justify_h(lua, this.0, args)
    });
    methods.add_method("SetJustifyV", |lua, this, args: mlua::MultiValue| {
        apply_set_justify_v(lua, this.0, args)
    });
    methods.add_method("GetJustifyH", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let s = state
            .widgets
            .get(this.0)
            .map(|f| f.justify_h.as_h_str())
            .unwrap_or("CENTER");
        Ok(Value::String(lua.create_string(s)?))
    });
    methods.add_method("GetJustifyV", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let s = state
            .widgets
            .get(this.0)
            .map(|f| f.justify_v.as_v_str())
            .unwrap_or("MIDDLE");
        Ok(Value::String(lua.create_string(s)?))
    });
}

/// Set horizontal justification, handling SimpleHTML per-textType and standard FontString.
fn apply_set_justify_h(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    if is_simple_html(lua, id)
        && args_vec.len() >= 2
        && let Some(Value::String(s)) = args_vec.first()
    {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            if let Some(Value::String(j)) = args_vec.get(1) {
                set_html_justify(
                    lua,
                    id,
                    type_str,
                    |style, val| style.justify_h = val,
                    j.to_string_lossy().to_string(),
                );
            }
            return Ok(());
        }
    }
    if let Some(Value::String(j)) = args_vec.first() {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.justify_h = crate::widget::TextJustify::from_wow_str(&j.to_string_lossy());
        }
    }
    Ok(())
}

/// Set vertical justification, handling SimpleHTML per-textType and standard FontString.
fn apply_set_justify_v(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    if is_simple_html(lua, id)
        && args_vec.len() >= 2
        && let Some(Value::String(s)) = args_vec.first()
    {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            if let Some(Value::String(j)) = args_vec.get(1) {
                set_html_justify(
                    lua,
                    id,
                    type_str,
                    |style, val| style.justify_v = val,
                    j.to_string_lossy().to_string(),
                );
            }
            return Ok(());
        }
    }
    if let Some(Value::String(j)) = args_vec.first() {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.justify_v = crate::widget::TextJustify::from_wow_str(&j.to_string_lossy());
        }
    }
    Ok(())
}

/// Store a justification value in a SimpleHTML text style.
fn set_html_justify(
    lua: &Lua,
    id: u64,
    type_str: String,
    setter: fn(&mut TextStyle, String),
    value: String,
) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(data) = state.simple_htmls.get_mut(&id) {
        let style = data
            .text_styles
            .entry(type_str)
            .or_insert_with(TextStyle::default);
        setter(style, value);
    }
}
