//! C_PaperDollInfo temporary stagger shim: monk stagger state is not modeled.
//!
//! Character stats and addons probe `GetStaggerPercentage` even on characters
//! that cannot stagger. Until the simulator tracks stagger amount against the
//! current target, expose WoW's inert no-stagger shape from the shim layer.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_paper_doll_stagger_shim(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_PaperDollInfo")?;
    table_set_rust_fn_static(state, ns, "GetStaggerPercentage", get_stagger_percentage)?;
    Ok(())
}

fn get_stagger_percentage(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}
