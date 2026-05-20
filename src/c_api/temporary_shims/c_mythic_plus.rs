//! C_MythicPlus temporary shims — weekly-chest cache and refresh requests are
//! not modeled.
//!
//! The main Mythic+ surface is backed by `SimState.mythic_plus`. These helpers
//! represent server/cache behavior that the simulator does not model yet.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_mythic_plus_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_MythicPlus")?;
    table_set_rust_fn_static(state, ns, "GetLastWeeklyChest", get_last_weekly_chest)?;
    table_set_rust_fn_static(state, ns, "RequestCurrentAffixes", noop)?;
    table_set_rust_fn_static(state, ns, "RequestMapInfo", noop)?;
    table_set_rust_fn_static(state, ns, "RequestRewards", noop)?;
    Ok(())
}

fn get_last_weekly_chest(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
