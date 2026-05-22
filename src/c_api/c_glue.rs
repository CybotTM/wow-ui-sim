//! `C_Glue` screen-mode helpers.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn register_c_glue(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Glue")?;
    table_set_rust_fn_static(state, ns, "IsOnGlueScreen", c_glue_is_on_glue_screen)?;
    table_set_rust_fn_static(
        state,
        ns,
        "IsFirstLoadThisSession",
        c_glue_is_first_load_this_session,
    )?;
    Ok(())
}

fn c_glue_is_on_glue_screen(state: &mut LuaState) -> LuaResult<u32> {
    let is_glue = borrow_state(state)?.screen_kind.is_glue();
    state.push(Val::Bool(is_glue));
    Ok(1)
}

fn c_glue_is_first_load_this_session(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}
