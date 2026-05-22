//! Temporary `C_Ping` fallback surface.
//!
//! Ping target options are not modeled yet. Startup code expects the query to
//! be callable and receive a list, so return an empty option list until the
//! ping system has backing state.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_ping_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Ping")?;
    table_set_rust_fn_static(state, ns, "GetDefaultPingOptions", get_default_ping_options)
}

fn get_default_ping_options(state: &mut LuaState) -> LuaResult<u32> {
    let options = create_table(state);
    state.push(options);
    Ok(1)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn default_ping_options_are_an_empty_list() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let count: i32 = env
            .eval("local options = C_Ping.GetDefaultPingOptions(); return #options")
            .expect("default ping options should be queryable");

        assert_eq!(count, 0);
    }
}
