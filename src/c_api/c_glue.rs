//! `C_Glue` screen-mode helpers.

use crate::c_api::{ensure_namespace, permanent_shims};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn register_c_glue(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Glue")?;
    table_set_rust_fn_static(state, ns, "IsOnGlueScreen", c_glue_is_on_glue_screen)?;
    permanent_shims::c_glue::register_c_glue_defaults(state, ns)?;
    Ok(())
}

fn c_glue_is_on_glue_screen(state: &mut LuaState) -> LuaResult<u32> {
    let is_glue = borrow_state(state)?.screen_kind.is_glue();
    state.push(Val::Bool(is_glue));
    Ok(1)
}
