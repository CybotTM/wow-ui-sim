//! Temporary empty Classic product rewards and loot history. No native-default or populated claims.
//! Retire each endpoint when its corresponding supplied backing system exists.
use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let product = ensure_namespace(state, "C_ProductChoice")?;
    table_set_rust_fn_static(state, product, "GetChoices", empty_choices)?;
    let loot = ensure_namespace(state, "C_LootHistory")?;
    table_set_rust_fn_static(state, loot, "GetNumItems", empty_loot_count)?;
    Ok(())
}

fn empty_loot_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn empty_choices(state: &mut LuaState) -> LuaResult<u32> {
    let choices = create_table(state);
    state.push(choices);
    Ok(1)
}
