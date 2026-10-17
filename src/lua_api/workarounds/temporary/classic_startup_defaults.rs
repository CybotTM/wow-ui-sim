//! Temporary empty Classic product rewards. No native-default or populated claims.
//! Retire each endpoint when its corresponding supplied backing system exists.
use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let product = ensure_namespace(state, "C_ProductChoice")?;
    table_set_rust_fn_static(state, product, "GetChoices", empty_choices)?;
    Ok(())
}

fn empty_choices(state: &mut LuaState) -> LuaResult<u32> {
    let choices = create_table(state);
    state.push(choices);
    Ok(1)
}
