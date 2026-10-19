//! Owner-serviced UI lifecycle requests, shared with the legacy ReloadUI global.

use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = super::ensure_namespace(state, "C_UI")?;
    table_set_rust_fn_static(state, namespace, "Reload", request_reload)
}

pub(crate) fn request_reload(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.ui_reload_requested = true;
    Ok(0)
}
