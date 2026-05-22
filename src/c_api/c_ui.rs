//! `C_UI` display-safe-area helpers.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn register_c_ui(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_UI")?;
    table_set_rust_fn_static(
        state,
        ns,
        "ShouldUIParentAvoidNotch",
        c_ui_should_ui_parent_avoid_notch,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "DoesAnyDisplayHaveNotch",
        c_ui_does_any_display_have_notch,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetTopLeftNotchSafeRegion",
        c_ui_get_top_left_notch_safe_region,
    )?;
    Ok(())
}

fn c_ui_should_ui_parent_avoid_notch(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_ui_does_any_display_have_notch(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_ui_get_top_left_notch_safe_region(state: &mut LuaState) -> LuaResult<u32> {
    for _ in 0..4 {
        state.push(Val::Num(0.0));
    }
    Ok(4)
}
