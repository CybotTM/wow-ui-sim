//! Legacy item globals.
//!
//! `GetItemID` is the plain global wrapper for `C_Item.GetItemID`. Register it
//! from the Lua globals layer so `c_api::item_spell` owns only C_* namespaces.

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_legacy_item_globals(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        state.global,
        "GetItemID",
        crate::c_api::item_spell::c_item_get_item_id,
    )
}
