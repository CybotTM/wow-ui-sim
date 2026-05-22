//! Temporary color/PvP fallback surface.
//!
//! These methods are no-state compatibility defaults. Other `C_PvP` methods
//! remain registered by the state-backed PvP/world systems.

use crate::c_api::{ensure_namespace, global_val};
use crate::lua_api::methods::{call_function_state, create_table_with_fields};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_color_and_pvp_default_shims(state: &mut LuaState) -> LuaResult<()> {
    register_color_overrides(state)?;
    register_pvp_defaults(state)
}

fn register_color_overrides(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_ColorOverrides")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetColorForQuality",
        get_color_for_quality,
    )
}

fn register_pvp_defaults(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_PvP")?;
    table_set_rust_fn_static(state, namespace, "IsInBrawl", return_false)?;
    table_set_rust_fn_static(state, namespace, "IsSoloShuffle", return_false)?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetArenaCrowdControlInfo",
        get_arena_crowd_control_info,
    )
}

fn get_color_for_quality(state: &mut LuaState) -> LuaResult<u32> {
    let color = create_white_color(state);
    state.push(color);
    Ok(1)
}

fn create_white_color(state: &mut LuaState) -> Val {
    let args = &[Val::Num(1.0), Val::Num(1.0), Val::Num(1.0), Val::Num(1.0)];
    let create_color = global_val(state, "CreateColor");
    match call_function_state(state, create_color, args) {
        Ok(color) => color,
        Err(_) => create_table_with_fields(
            state,
            &[
                ("r", Val::Num(1.0)),
                ("g", Val::Num(1.0)),
                ("b", Val::Num(1.0)),
                ("a", Val::Num(1.0)),
            ],
        ),
    }
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn get_arena_crowd_control_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(3)
}
