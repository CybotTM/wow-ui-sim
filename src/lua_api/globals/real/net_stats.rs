//! Network status global used by the performance bar tooltip.
//!
//! WoW's real `GetNetStats()` returns `(bandwidthIn, bandwidthOut, latencyHome,
//! latencyWorld)` in (kB/s, kB/s, ms, ms). The sim has no network socket so
//! the defaults are all zeros, but tests can inject non-zero values via
//! `A_Admin.SetNetStats(...)` to drive UI paths that colour-code latency or
//! bandwidth indicators (e.g. Blizzard_MicroMenu's chat badge).
//!
//! This replaces the Lua stub that used to live in
//! `runtime_surface_bootstrap.lua` — a proper Rust registration makes the
//! admin override path authoritative (no Lua-level `if GetNetStats == nil`
//! guard to out-race).

use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// `GetNetStats() -> (bandwidthIn, bandwidthOut, latencyHome, latencyWorld)`.
pub fn get_net_stats(state: &mut LuaState) -> LuaResult<u32> {
    let stats = {
        let sim = borrow_state(state)?;
        sim.net_stats
    };
    state.push(Val::Num(stats.bandwidth_in_kbps));
    state.push(Val::Num(stats.bandwidth_out_kbps));
    state.push(Val::Num(stats.latency_home_ms));
    state.push(Val::Num(stats.latency_world_ms));
    Ok(4)
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    table_set_rust_fn_static(state, state.global, "GetNetStats", get_net_stats)?;
    Ok(())
}
