//! C_LFGInfo temporary shim — returns empty tables / permissive booleans.
//!
//! Backed by no state model; real LFG simulation would replace this.

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use crate::c_api::ensure_global_table;
use crate::lua_api::methods::create_table;

pub fn register_c_lfg_info(state: &mut LuaState) -> LuaResult<()> {
    let t = ensure_global_table(state, "C_LFGInfo");
    let Val::Table(t_ref) = t else {
        unreachable!("C_LFGInfo must be a table");
    };
    table_set_rust_fn_static(state, t_ref, "GetDungeonInfo", empty_table_result)?;
    table_set_rust_fn_static(state, t_ref, "GetLFDLockStates", empty_table_result)?;
    table_set_rust_fn_static(state, t_ref, "GetAllEntriesForCategory", empty_table_result)?;
    table_set_rust_fn_static(state, t_ref, "CanPlayerUseLFD", can_player_use)?;
    table_set_rust_fn_static(state, t_ref, "CanPlayerUseLFR", can_player_use)?;
    Ok(())
}

fn empty_table_result(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn can_player_use(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    state.push(Val::Nil);
    Ok(2)
}
