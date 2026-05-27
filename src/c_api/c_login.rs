//! `C_Login` glue-screen connection state helpers.

use crate::c_api::{ensure_namespace, permanent_shims};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn register_c_login(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Login")?;
    table_set_rust_fn_static(state, ns, "IsLoginReady", c_login_is_login_ready)?;
    table_set_rust_fn_static(state, ns, "GetState", c_login_get_state)?;
    permanent_shims::c_login::register_c_login_defaults(state, ns)?;
    Ok(())
}

fn c_login_is_login_ready(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_login_get_state(state: &mut LuaState) -> LuaResult<u32> {
    let screen_kind = borrow_state(state)?.screen_kind;
    let (aurora_state, connected_to_wow, wow_connection_state, has_realm_list) =
        screen_kind.login_state();
    state.push(Val::Num(f64::from(aurora_state)));
    state.push(Val::Bool(connected_to_wow));
    state.push(Val::Num(f64::from(wow_connection_state)));
    state.push(Val::Bool(has_realm_list));
    state.push(Val::Bool(false));
    Ok(5)
}
