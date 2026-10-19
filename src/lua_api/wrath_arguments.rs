//! Argument conversions shared by the Wrath region/text entry points.

use crate::lua_api::methods::val_to_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

/// Use the VM's numeric grammar without consulting a replaceable Lua global.
pub(crate) fn number(state: &mut LuaState, value: Val) -> LuaResult<Option<f64>> {
    let saved_base = state.base;
    let saved_top = state.top;
    state.base = saved_top;
    state.push(value);
    let result = rilua::stdlib::base::lua_tonumber(state);
    let converted = match state.stack_get(state.top - 1) {
        Val::Num(value) => Some(value),
        _ => None,
    };
    state.base = saved_base;
    state.top = saved_top;
    result?;
    Ok(converted)
}

pub(crate) fn string(state: &mut LuaState, value: Val) -> LuaResult<Option<String>> {
    if !matches!(value, Val::Str(_) | Val::Num(_)) {
        return Ok(None);
    }
    let saved_base = state.base;
    let saved_top = state.top;
    state.base = saved_top;
    state.push(value);
    let result = rilua::stdlib::base::lua_tostring(state);
    let converted = val_to_string(state, state.stack_get(state.top - 1));
    state.base = saved_base;
    state.top = saved_top;
    result?;
    Ok(converted)
}

pub(crate) fn text_color(state: &mut LuaState) -> LuaResult<[f32; 4]> {
    let mut color = [0.0, 0.0, 0.0, 1.0];
    for (offset, component) in color.iter_mut().enumerate() {
        let value = crate::lua_bridge::stack_val(state, offset as i32 + 2);
        *component = number(state, value)?
            .unwrap_or(*component as f64)
            .clamp(0.0, 1.0) as f32;
    }
    Ok(color)
}

pub(crate) fn justification(token: &str, horizontal: bool) -> LuaResult<&'static str> {
    let canonical = match token.to_ascii_uppercase().as_str() {
        "LEFT" => "LEFT",
        "CENTER" => "CENTER",
        "RIGHT" => "RIGHT",
        "TOP" => "TOP",
        "MIDDLE" => "MIDDLE",
        "BOTTOM" => "BOTTOM",
        _ => return Err(runtime_error("SetJustify: expected a justification token")),
    };
    let horizontal_token = matches!(canonical, "LEFT" | "CENTER" | "RIGHT");
    Ok(if horizontal_token == horizontal {
        canonical
    } else {
        "UNKNOWN"
    })
}
