use crate::c_api::ensure_namespace;
#[cfg(feature = "client-ptr")]
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
#[cfg(feature = "client-ptr")]
use rilua::Val;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

pub(crate) fn register_c_pvp_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_PvP")?;
    register_ptr_c_pvp_surface(state, ns)?;
    Ok(())
}

#[cfg(feature = "client-ptr")]
fn register_ptr_c_pvp_surface(state: &mut LuaState, ns: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, ns, "CanSurrenderArena", can_surrender_arena)
}

#[cfg(not(feature = "client-ptr"))]
fn register_ptr_c_pvp_surface(_state: &mut LuaState, _ns: GcRef<Table>) -> LuaResult<()> {
    Ok(())
}

#[cfg(feature = "client-ptr")]
fn can_surrender_arena(state: &mut LuaState) -> LuaResult<u32> {
    // The simulator does not model active rated-arena matches.
    state.push(Val::Bool(false));
    Ok(1)
}
