//! C_Calendar temporary shim — calendar/event/invite state is not modeled.
//!
//! The Calendar UI loads against an empty calendar:
//! - `GetMonthInfo(offset)` returns a sane month skeleton so OnLoad code
//!   indexing `.month`/`.year`/`.numDays`/`.firstWeekday` doesn't crash.
//! - `EventGetTypesDisplayOrdered` returns an empty array so `ipairs` over
//!   the dropdown setup at load time is well-formed.
//! - All other commonly-called methods return safe defaults (no events,
//!   no invites, all booleans false).
//!
//! Real calendar state would replace this surface.

use crate::c_api::ensure_global_table;
use crate::lua_api::methods::{create_table, table_set};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn register_c_calendar(state: &mut LuaState) -> LuaResult<()> {
    let t = ensure_global_table(state, "C_Calendar");
    let Val::Table(t_ref) = t else {
        unreachable!("C_Calendar must be a table");
    };
    table_set_rust_fn_static(state, t_ref, "GetMonthInfo", get_month_info)?;
    table_set_rust_fn_static(state, t_ref, "GetMaxCreateDate", empty_date_info)?;
    table_set_rust_fn_static(state, t_ref, "GetMinDate", empty_date_info)?;
    table_set_rust_fn_static(
        state,
        t_ref,
        "EventGetTypesDisplayOrdered",
        empty_table_result,
    )?;
    table_set_rust_fn_static(state, t_ref, "GetNumDayEvents", return_zero)?;
    table_set_rust_fn_static(state, t_ref, "GetNumGuildEvents", return_zero)?;
    table_set_rust_fn_static(state, t_ref, "GetNumInvites", return_zero)?;
    table_set_rust_fn_static(state, t_ref, "GetNumPendingInvites", return_zero)?;
    table_set_rust_fn_static(state, t_ref, "GetClubCalendarEvents", empty_table_result)?;
    table_set_rust_fn_static(state, t_ref, "AreNamesReady", return_true)?;
    table_set_rust_fn_static(state, t_ref, "IsActionPending", return_false)?;
    table_set_rust_fn_static(state, t_ref, "CanAddEvent", return_false)?;
    table_set_rust_fn_static(state, t_ref, "CanSendInvite", return_false)?;
    table_set_rust_fn_static(state, t_ref, "OpenCalendar", noop)?;
    table_set_rust_fn_static(state, t_ref, "CloseEvent", noop)?;
    Ok(())
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn return_true(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn empty_table_result(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

/// `{month, year, numDays, firstWeekday}` placeholder for any month
/// offset. Real calendar state would compute these from the current date.
fn get_month_info(state: &mut LuaState) -> LuaResult<u32> {
    let t = create_table(state);
    table_set(state, t, "month", Val::Num(1.0));
    table_set(state, t, "year", Val::Num(2024.0));
    table_set(state, t, "numDays", Val::Num(31.0));
    table_set(state, t, "firstWeekday", Val::Num(1.0));
    state.push(t);
    Ok(1)
}

/// `{year, month, monthDay, weekday, hour, minute}` placeholder used by
/// `GetMaxCreateDate` / `GetMinDate`.
fn empty_date_info(state: &mut LuaState) -> LuaResult<u32> {
    let t = create_table(state);
    table_set(state, t, "year", Val::Num(2024.0));
    table_set(state, t, "month", Val::Num(1.0));
    table_set(state, t, "monthDay", Val::Num(1.0));
    table_set(state, t, "weekday", Val::Num(1.0));
    table_set(state, t, "hour", Val::Num(0.0));
    table_set(state, t, "minute", Val::Num(0.0));
    state.push(t);
    Ok(1)
}
