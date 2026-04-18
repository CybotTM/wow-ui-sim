//! `C_FogOfWar` probe surface for world-map fog pins.

use super::ensure_namespace;
use crate::lua_api::methods::{create_table, table_set};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_fog_of_war_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_FogOfWar")?;
    table_set_rust_fn_static(state, ns, "GetFogOfWarForMap", get_fog_of_war_for_map)?;
    table_set_rust_fn_static(state, ns, "GetFogOfWarInfo", get_fog_of_war_info)?;
    Ok(())
}

fn get_fog_of_war_for_map(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn get_fog_of_war_info(state: &mut LuaState) -> LuaResult<u32> {
    let info = create_table(state);
    table_set(state, info, "backgroundAtlas", Val::Nil);
    table_set(state, info, "maskAtlas", Val::Nil);
    table_set(state, info, "maskScalar", Val::Num(1.0));
    state.push(info);
    Ok(1)
}
