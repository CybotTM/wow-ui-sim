//! PTR surname visibility projected from an explicit player policy input.

use super::helpers::ensure_namespace;
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_PlayerInfo")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "ShouldDisplaySurname",
        should_display_surname,
    )
}

fn should_display_surname(state: &mut LuaState) -> LuaResult<u32> {
    let display = borrow_state(state)?
        .player
        .should_display_surname
        .ok_or_else(|| {
            rilua::runtime_error(
                "C_PlayerInfo.ShouldDisplaySurname: surname visibility has not been supplied",
            )
        })?;
    state.push(Val::Bool(display));
    Ok(1)
}
