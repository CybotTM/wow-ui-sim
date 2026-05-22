//! Temporary `C_DateAndTime` calendar/server-time fallback surface.
//!
//! The simulator does not model server calendar state yet. These deterministic
//! defaults preserve Blizzard UI callers that expect calendar-shaped tables.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{create_table, table_get, table_set};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const BASE_YEAR: f64 = 2026.0;
const BASE_MONTH: f64 = 4.0;
const BASE_MONTH_DAY: i64 = 14;
const BASE_WEEKDAY: f64 = 3.0;
const BASE_HOUR: i64 = 12;
const MINUTES_PER_DAY: i64 = 24 * 60;

pub(crate) fn register_c_date_and_time_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_DateAndTime")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetCurrentCalendarTime",
        get_current_calendar_time,
    )?;
    table_set_rust_fn_static(state, ns, "GetServerTimeLocal", return_zero)?;
    table_set_rust_fn_static(state, ns, "AdjustTimeByDays", adjust_time_by_days)?;
    table_set_rust_fn_static(state, ns, "AdjustTimeByMinutes", adjust_time_by_minutes)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetCalendarTimeFromEpoch",
        get_calendar_time_from_epoch,
    )?;
    table_set_rust_fn_static(state, ns, "GetWeeklyResetStartTime", return_zero)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetSecondsUntilDailyReset",
        seconds_until_daily_reset,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetSecondsUntilWeeklyReset",
        seconds_until_weekly_reset,
    )?;
    Ok(())
}

fn get_current_calendar_time(state: &mut LuaState) -> LuaResult<u32> {
    let calendar_time = make_calendar_time(state, 0, 0);
    state.push(calendar_time);
    Ok(1)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn adjust_time_by_days(state: &mut LuaState) -> LuaResult<u32> {
    let calendar_time = calendar_time_from_arg(state);
    let delta_days = i64::from_stack(state, 2).unwrap_or(0);
    let adjusted = make_calendar_time_from_parts(
        state,
        calendar_time.month_day + delta_days,
        calendar_time.hour,
        calendar_time.minute,
    );
    state.push(adjusted);
    Ok(1)
}

fn adjust_time_by_minutes(state: &mut LuaState) -> LuaResult<u32> {
    let calendar_time = calendar_time_from_arg(state);
    let delta_minutes = i64::from_stack(state, 2).unwrap_or(0);
    let base_minutes = calendar_time.hour * 60 + calendar_time.minute + delta_minutes;
    let day_delta = base_minutes.div_euclid(MINUTES_PER_DAY);
    let minute_of_day = base_minutes.rem_euclid(MINUTES_PER_DAY);
    let adjusted = make_calendar_time_from_parts(
        state,
        calendar_time.month_day + day_delta,
        minute_of_day / 60,
        minute_of_day % 60,
    );
    state.push(adjusted);
    Ok(1)
}

fn get_calendar_time_from_epoch(state: &mut LuaState) -> LuaResult<u32> {
    let mut seconds = f64::from_stack(state, 1).unwrap_or(0.0);
    if seconds > 1_000_000_000_000.0 {
        seconds /= 1_000_000.0;
    }
    let total_minutes = (seconds / 60.0).floor() as i64;
    let day_offset = total_minutes.div_euclid(MINUTES_PER_DAY).rem_euclid(30);
    let minute_offset = total_minutes.rem_euclid(MINUTES_PER_DAY);
    let calendar_time = make_calendar_time(state, day_offset, minute_offset);
    state.push(calendar_time);
    Ok(1)
}

fn seconds_until_daily_reset(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(86_400.0));
    Ok(1)
}

fn seconds_until_weekly_reset(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(604_800.0));
    Ok(1)
}

fn calendar_time_from_arg(state: &mut LuaState) -> CalendarTime {
    let table = Val::from_stack(state, 1).unwrap_or(Val::Nil);
    CalendarTime {
        month_day: table_number(state, table, "monthDay").unwrap_or(BASE_MONTH_DAY),
        hour: table_number(state, table, "hour").unwrap_or(BASE_HOUR),
        minute: table_number(state, table, "minute").unwrap_or(0),
    }
}

fn table_number(state: &mut LuaState, table: Val, key: &str) -> Option<i64> {
    match table_get(state, table, key) {
        Val::Num(value) => Some(value as i64),
        _ => None,
    }
}

fn make_calendar_time(state: &mut LuaState, day_offset: i64, minute_offset: i64) -> Val {
    let total_minutes = BASE_HOUR * 60 + minute_offset;
    let day_delta = total_minutes.div_euclid(MINUTES_PER_DAY);
    let minute_of_day = total_minutes.rem_euclid(MINUTES_PER_DAY);
    make_calendar_time_from_parts(
        state,
        BASE_MONTH_DAY + day_offset + day_delta,
        minute_of_day / 60,
        minute_of_day % 60,
    )
}

fn make_calendar_time_from_parts(
    state: &mut LuaState,
    month_day: i64,
    hour: i64,
    minute: i64,
) -> Val {
    let table = create_table(state);
    table_set(state, table, "year", Val::Num(BASE_YEAR));
    table_set(state, table, "month", Val::Num(BASE_MONTH));
    table_set(state, table, "monthDay", Val::Num(month_day as f64));
    table_set(state, table, "weekday", Val::Num(BASE_WEEKDAY));
    table_set(state, table, "hour", Val::Num(hour as f64));
    table_set(state, table, "minute", Val::Num(minute as f64));
    table
}

struct CalendarTime {
    month_day: i64,
    hour: i64,
    minute: i64,
}
