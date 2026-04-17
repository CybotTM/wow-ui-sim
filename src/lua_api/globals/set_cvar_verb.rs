//! `SetCVar(name, value)` global. Routes through `SimState.cvars` so
//! `GetCVar` returns the updated value in the same session.
//!
//! Previously registered only on the `A_Admin` path; the retail-facing
//! global was `stub_nil`. Addons that call `SetCVar("nameplateShowAll", 1)`
//! now actually persist the change for the session.

use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn required_string(state: &mut LuaState, index: i32) -> Option<String> {
    Option::<String>::from_stack(state, index)
        .ok()
        .flatten()
        .filter(|s| !s.is_empty())
}

fn value_to_string(state: &mut LuaState, index: i32) -> String {
    match stack_val(state, index) {
        Val::Nil => String::new(),
        Val::Bool(true) => "1".to_string(),
        Val::Bool(false) => "0".to_string(),
        Val::Num(n) => {
            if n.fract() == 0.0 {
                format!("{}", n as i64)
            } else {
                format!("{n}")
            }
        }
        Val::Str(_) => Option::<String>::from_stack(state, index)
            .ok()
            .flatten()
            .unwrap_or_default(),
        _ => String::new(),
    }
}

/// `SetCVar(name, value)` — write `value` (stringified) to
/// `SimState.cvars[name]`. Returns true on success, matching retail.
fn set_cvar(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = required_string(state, 1) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let value = value_to_string(state, 2);
    let accepted = borrow_state_mut(state)?.cvars.set(&name, &value);
    state.push(Val::Bool(accepted));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "SetCVar", set_cvar)?;
    Ok(())
}
