//! Explicit absolute-millisecond input; zero default is simulator policy.

use crate::lua_api::methods::{borrow_state, create_table, table_set_static};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[derive(Default)]
pub struct PingCooldownInterval {
    pub start_time_ms: f64,
    pub end_time_ms: f64,
}

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = super::ensure_namespace(state, "C_Ping")?;
    table_set_rust_fn_static(state, namespace, "GetCooldownInfo", get_cooldown_info)
}

fn get_cooldown_info(state: &mut LuaState) -> LuaResult<u32> {
    let (start_time_ms, end_time_ms) = {
        let sim = borrow_state(state)?;
        let interval = &sim.ping_cooldown_interval;
        (interval.start_time_ms, interval.end_time_ms)
    };
    // Release the provider borrow before allocating; root the fresh DTO before keys.
    let info = create_table(state);
    state.push(info);
    table_set_static(state, info, "startTimeMs", Val::Num(start_time_ms));
    table_set_static(state, info, "endTimeMs", Val::Num(end_time_ms));
    Ok(1)
}
