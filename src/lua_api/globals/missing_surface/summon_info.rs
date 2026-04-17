//! `C_SummonInfo` and `C_IncomingSummon` probe surface backed by
//! `SimState.summon_request`.
//!
//! Migrates 5 entries off the namespace stub tables:
//!
//! - `C_SummonInfo.GetSummonReason()` — returns the numeric summon-reason
//!   code, or nil when no summon is active.
//! - `C_SummonInfo.GetSummonConfirmTimeLeft()` — returns `time_left_ms`.
//! - `C_SummonInfo.IsSummonSkippingStartExperience()` — returns bool flag.
//! - `C_IncomingSummon.HasIncomingSummon(unitToken)` — true when the
//!   summon is active and `unitToken == "player"`.
//! - `C_IncomingSummon.IncomingSummonStatus(unitToken)` — returns 1
//!   (pending) when active and unit is "player", 0 otherwise.

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, val_to_string};
use crate::lua_bridge::stack_val;
use crate::lua_bridge::table_set_rust_fn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_summon_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let summon_info = ensure_namespace(state, "C_SummonInfo")?;
    table_set_rust_fn(
        state,
        summon_info,
        "GetSummonReason",
        c_summon_info_get_reason,
    )?;
    table_set_rust_fn(
        state,
        summon_info,
        "GetSummonConfirmTimeLeft",
        c_summon_info_get_confirm_time_left,
    )?;
    table_set_rust_fn(
        state,
        summon_info,
        "IsSummonSkippingStartExperience",
        c_summon_info_is_skipping_start_experience,
    )?;

    let incoming = ensure_namespace(state, "C_IncomingSummon")?;
    table_set_rust_fn(
        state,
        incoming,
        "HasIncomingSummon",
        c_incoming_summon_has_incoming_summon,
    )?;
    table_set_rust_fn(
        state,
        incoming,
        "IncomingSummonStatus",
        c_incoming_summon_status,
    )?;
    Ok(())
}

/// `GetSummonReason()` — returns the numeric reason code when a summon is
/// active, or nil when idle.
fn c_summon_info_get_reason(state: &mut LuaState) -> LuaResult<u32> {
    let reason = {
        let sim = borrow_state(state)?;
        sim.summon_request
            .active
            .then_some(sim.summon_request.reason as f64)
    };
    if let Some(reason) = reason {
        state.push(Val::Num(reason));
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

/// `GetSummonConfirmTimeLeft()` — returns the milliseconds remaining on the
/// confirm timer (0 when no summon is active).
fn c_summon_info_get_confirm_time_left(state: &mut LuaState) -> LuaResult<u32> {
    let ms = borrow_state(state)?.summon_request.time_left_ms;
    state.push(Val::Num(ms as f64));
    Ok(1)
}

/// `IsSummonSkippingStartExperience()` — true when the summon bypasses the
/// start-experience flow.
fn c_summon_info_is_skipping_start_experience(state: &mut LuaState) -> LuaResult<u32> {
    let flag = borrow_state(state)?.summon_request.skips_start_experience;
    state.push(Val::Bool(flag));
    Ok(1)
}

/// `HasIncomingSummon(unitToken)` — true when the summon is active and the
/// queried unit is "player".
fn c_incoming_summon_has_incoming_summon(state: &mut LuaState) -> LuaResult<u32> {
    let unit = val_to_string(state, stack_val(state, 1)).unwrap_or_default();
    let active = borrow_state(state)?.summon_request.active;
    state.push(Val::Bool(active && unit == "player"));
    Ok(1)
}

/// `IncomingSummonStatus(unitToken)` — returns 1 (pending) when active and
/// unit is "player", 0 otherwise.  Status codes: 0=none, 1=pending,
/// 2=accepted, 3=declined.
fn c_incoming_summon_status(state: &mut LuaState) -> LuaResult<u32> {
    let unit = val_to_string(state, stack_val(state, 1)).unwrap_or_default();
    let active = borrow_state(state)?.summon_request.active;
    let status = if active && unit == "player" { 1.0 } else { 0.0 };
    state.push(Val::Num(status));
    Ok(1)
}
