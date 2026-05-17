//! Private loader helpers for XML script binding slots.
//!
//! These are not WoW API globals. The XML loader emits calls to them when it
//! needs to install intrinsic script bindings without going through the public
//! `SetScript` normal-binding path.

use crate::lua_api::methods::{frame_id_from_stack, val_to_string};
use crate::lua_api::script_helpers::{ScriptBinding, remove_script_binding, set_script_binding};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

pub const SET_SCRIPT_BINDING_GLOBAL: &str = "__wow_uisim_set_script_binding";

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, SET_SCRIPT_BINDING_GLOBAL, set_script_binding_global)?;
    Ok(())
}

fn set_script_binding_global(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let handler_name = val_to_string(state, stack_val(state, 2))
        .ok_or_else(|| runtime_error("set script binding: handler name required"))?;
    let binding = script_binding_from_stack(state, 3)?;
    let handler = stack_val(state, 4);

    match handler {
        Val::Nil => remove_script_binding(state, frame_id, &handler_name, binding),
        Val::Function(_) => set_script_binding(state, frame_id, &handler_name, binding, handler),
        other => {
            return Err(runtime_error(format!(
                "set script binding: handler must be a function or nil, got {}",
                other.type_name()
            )));
        }
    }
    Ok(0)
}

fn script_binding_from_stack(state: &mut LuaState, index: i32) -> LuaResult<ScriptBinding> {
    match stack_val(state, index) {
        Val::Num(raw) if raw.is_finite() => {
            let binding_index = raw as i32;
            if binding_index as f64 == raw
                && let Some(binding) = ScriptBinding::from_index(binding_index)
            {
                return Ok(binding);
            }
            Err(runtime_error(format!(
                "set script binding: binding must be 0, 1, or 2, got {raw}"
            )))
        }
        other => Err(runtime_error(format!(
            "set script binding: binding must be a number, got {}",
            other.type_name()
        ))),
    }
}
