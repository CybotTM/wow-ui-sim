//! `C_AdventureMap` namespace — Broken Isles / Garrison-style adventure-map
//! surface consumed by the Blizzard_AdventureMap addon.
//!
//! Currently implements `GetMapID()` only. Future commits will fill in the
//! full surface (zone choices, quest offers, inset metadata, dialog hooks).

use crate::lua_api::methods::{borrow_state, create_table};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaApiMut;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const NAMESPACE: &str = "C_AdventureMap";

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let table_ref = ensure_namespace_table(state);
    table_set_rust_fn_static(state, table_ref, "GetMapID", get_map_id)?;
    Ok(())
}

fn ensure_namespace_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(NAMESPACE.as_bytes());
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(table_ref)) = existing {
        return table_ref;
    }
    let new_val = create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}

fn get_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = borrow_state(state)?.adventure_map.map_id;
    state.push(Val::Num(map_id as f64));
    Ok(1)
}
