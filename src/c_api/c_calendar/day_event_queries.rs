//! Queries over empty-default, host-supplied Calendar day records.

use super::day_event_projection::push_day_event;
use crate::c_api::helpers::validate_supplied_relation_id;
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn query_num_day_events(state: &mut LuaState) -> LuaResult<u32> {
    let [offset, day] =
        read_required_numbers(state, "GetNumDayEvents", ["monthOffset", "monthDay"])?;
    let count = match represented_day_key(offset, day)? {
        Some(key) => borrow_state(state)?
            .calendar_day_events
            .get(&key)
            .map_or(0, Vec::len),
        None => 0,
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub(super) fn query_day_event(state: &mut LuaState) -> LuaResult<u32> {
    let [offset, day, position] =
        read_required_numbers(state, "GetDayEvent", ["monthOffset", "monthDay", "index"])?;
    let key = represented_day_key(offset, day)?;
    let index = represented_position(position);
    // End the host-state borrow before any Lua table/string allocation.
    let record = match (key, index) {
        (Some(key), Some(index)) => {
            let host = borrow_state(state)?;
            host.calendar_day_events
                .get(&key)
                .and_then(|rows| rows.get(index))
                .cloned()
        }
        _ => None,
    };
    push_day_event(state, record);
    Ok(1)
}

fn read_required_numbers<const N: usize>(
    state: &LuaState,
    method: &str,
    names: [&str; N],
) -> LuaResult<[f64; N]> {
    let mut values = [Val::Nil; N];
    // Authenticate every required slot before validating even the first type.
    for (index, value) in values.iter_mut().enumerate() {
        *value = stack_val(state, index as i32 + 1);
        #[cfg(feature = "retail-12-1-0")]
        {
            *value = rilua::table_security::unwrap_secret(state, *value)?;
        }
    }
    let mut numbers = [0.0; N];
    for ((value, name), number) in values.into_iter().zip(names).zip(numbers.iter_mut()) {
        let Val::Num(value) = value else {
            return Err(rilua::runtime_error(format!(
                "C_Calendar.{method}: {name} must be a number"
            )));
        };
        *number = value;
    }
    Ok(numbers)
}

fn represented_day_key(offset: f64, day: f64) -> LuaResult<Option<(i32, u32)>> {
    let offset = validate_supplied_relation_id(
        Val::Num(offset),
        "C_Calendar: monthOffset must be a number",
    )?;
    let day_in_range = (0.0..=f64::from(u32::MAX)).contains(&day);
    let day_is_integer = day.is_finite() && day.fract() == 0.0;
    let day = (day_is_integer && day_in_range).then(|| day as u32);
    Ok(offset.zip(day))
}

fn represented_position(position: f64) -> Option<usize> {
    let is_integer = position.is_finite() && position.fract() == 0.0;
    // usize::MAX rounds upward on 64-bit hosts; reject that rounded boundary.
    let in_range = position >= 1.0 && position < usize::MAX as f64;
    (is_integer && in_range).then(|| position as usize - 1)
}
