//! Temporary `C_CooldownViewer` fallback surface.
//!
//! Cooldown viewer category/cooldown state is not modeled yet. These defaults
//! keep the Blizzard cooldown-viewer UI loadable with empty data.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_cooldown_viewer_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_CooldownViewer")?;
    table_set_rust_fn_static(state, ns, "GetCooldownViewerCategorySet", empty_table)?;
    table_set_rust_fn_static(state, ns, "GetCooldownViewerCooldownInfo", no_results)?;
    table_set_rust_fn_static(state, ns, "GetCooldownID", no_results)?;
    Ok(())
}

fn empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn no_results(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
