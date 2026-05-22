//! C_ScriptedAnimations temporary shim — scripted animation effects are not modeled.
//!
//! Startup consumers iterate this list while initializing animation metadata.
//! Return an empty list until scripted animation effect data has backing state.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_scripted_animations_shims(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_ScriptedAnimations")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetAllScriptedAnimationEffects",
        get_all_scripted_animation_effects,
    )
}

fn get_all_scripted_animation_effects(state: &mut LuaState) -> LuaResult<u32> {
    let effects = create_table(state);
    state.push(effects);
    Ok(1)
}
