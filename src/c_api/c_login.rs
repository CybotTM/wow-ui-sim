//! `C_Login` glue-screen connection state helpers.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn register_c_login(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Login")?;
    table_set_rust_fn_static(state, ns, "IsLoginReady", c_login_is_login_ready)?;
    table_set_rust_fn_static(state, ns, "GetState", c_login_get_state)?;
    table_set_rust_fn_static(state, ns, "IsLauncherLogin", c_login_false)?;
    table_set_rust_fn_static(state, ns, "IsReconnectLoginPossible", c_login_false)?;
    table_set_rust_fn_static(state, ns, "GetLastError", c_login_get_last_error)?;
    table_set_rust_fn_static(state, ns, "ClearLastError", c_login_noop)?;
    table_set_rust_fn_static(state, ns, "AttemptedLauncherLogin", c_login_false)?;
    table_set_rust_fn_static(state, ns, "IsNewPlayer", c_login_false)?;
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

fn c_login_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_login_get_last_error(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn c_login_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
