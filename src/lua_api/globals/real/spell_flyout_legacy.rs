//! Legacy spell flyout globals.
//!
//! `GetFlyoutInfo` and `GetFlyoutSlotInfo` are plain global wrappers over
//! the state-backed `C_SpellBook` flyout queries. Register the globals here
//! so `c_api::c_spell_book` owns only the `C_SpellBook` namespace.

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_legacy_spell_flyout_globals(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        state.global,
        "GetFlyoutInfo",
        crate::c_api::c_spell_book::get_flyout_info,
    )?;
    table_set_rust_fn_static(
        state,
        state.global,
        "GetFlyoutSlotInfo",
        crate::c_api::c_spell_book::get_flyout_slot_info,
    )?;
    Ok(())
}
