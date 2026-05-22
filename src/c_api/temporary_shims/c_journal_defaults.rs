//! Temporary journal defaults for unmodeled collections panels.
//!
//! Loot journal item sets and inspected PvP talent selection do not have
//! backing state yet. These helpers preserve empty startup behavior outside the
//! runtime bootstrap Lua.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_journal_default_shims(state: &mut LuaState) -> LuaResult<()> {
    register_loot_journal(state)?;
    register_specialization_info(state)
}

fn register_loot_journal(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_LootJournal")?;
    table_set_rust_fn_static(state, namespace, "GetItemSets", return_empty_table)?;
    table_set_rust_fn_static(state, namespace, "GetItemSetItems", return_empty_table)
}

fn register_specialization_info(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_SpecializationInfo")?;
    table_set_rust_fn_static(state, namespace, "GetInspectSelectedPvpTalent", return_nil)
}

fn return_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn return_nil(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}
