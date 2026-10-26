//! Supplied active effects, separate from spell loss-of-control cooldowns.
//! See docs/specs/active-loss-of-control.md for pinned declaration citations.

use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_get, table_set,
};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// LossOfControlData from each supported profile's pinned documentation.
#[derive(Clone, Debug)]
pub struct ActiveLossOfControlData {
    pub loc_type: String,
    pub spell_id: f64,
    pub display_text: String,
    pub icon_texture: f64,
    pub start_time: Option<f64>,
    pub time_remaining: Option<f64>,
    pub duration: Option<f64>,
    pub lockout_school: f64,
    pub priority: f64,
    pub display_type: f64,
    pub aura_instance_id: Option<f64>,
}

pub(crate) fn query_player_data(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    push_entry(state, "player", index)
}

pub(crate) fn query_unit_data(state: &mut LuaState) -> LuaResult<u32> {
    let unit = String::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    push_entry(state, &unit, index)
}

pub(crate) fn query_player_count(state: &mut LuaState) -> LuaResult<u32> {
    push_count(state, "player")
}

pub(crate) fn query_unit_count(state: &mut LuaState) -> LuaResult<u32> {
    let unit = String::from_stack(state, 1)?;
    push_count(state, &unit)
}

fn push_count(state: &mut LuaState, unit: &str) -> LuaResult<u32> {
    let count = borrow_state(state)?
        .active_loss_of_control
        .get(unit)
        .map_or(0, Vec::len);
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn read_entry(
    state: &LuaState,
    unit: &str,
    index: i32,
) -> LuaResult<Option<ActiveLossOfControlData>> {
    if index <= 0 {
        return Ok(None);
    }
    Ok(borrow_state(state)?
        .active_loss_of_control
        .get(unit)
        .and_then(|entries| entries.get(index as usize - 1))
        .cloned())
}

fn push_entry(state: &mut LuaState, unit: &str, index: i32) -> LuaResult<u32> {
    let Some(entry) = read_entry(state, unit, index)? else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let data = create_table(state);
    for (key, text) in [
        ("locType", &entry.loc_type),
        ("displayText", &entry.display_text),
    ] {
        let value = create_string(state, text);
        table_set(state, data, key, value);
    }
    for (key, value) in numeric_fields(&entry) {
        table_set(state, data, key, value.map_or(Val::Nil, Val::Num));
    }
    state.push(data);
    Ok(1)
}

fn numeric_fields(entry: &ActiveLossOfControlData) -> [(&'static str, Option<f64>); 9] {
    [
        ("spellID", Some(entry.spell_id)),
        ("iconTexture", Some(entry.icon_texture)),
        ("startTime", entry.start_time),
        ("timeRemaining", entry.time_remaining),
        ("duration", entry.duration),
        ("lockoutSchool", Some(entry.lockout_school)),
        ("priority", Some(entry.priority)),
        ("displayType", Some(entry.display_type)),
        ("auraInstanceID", entry.aura_instance_id),
    ]
}

pub(crate) fn query_duration(state: &mut LuaState) -> LuaResult<u32> {
    let unit = String::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    let timing = read_entry(state, &unit, index)?
        .and_then(|entry| Some((entry.start_time?, entry.duration?)));
    let Some((start, duration)) = timing else {
        state.push(Val::Nil);
        return Ok(1);
    };
    crate::lua_api::globals::lua_duration_object::push_timed_duration_object(state, start, duration)
}

/// Explicit fixture input only: native acquisition and event policy are not inferred.
pub(crate) fn add_supplied_entry(state: &mut LuaState) -> LuaResult<u32> {
    let unit = String::from_stack(state, 1)?;
    let row = stack_val(state, 2);
    if !matches!(row, Val::Table(_)) {
        return Err(rilua::runtime_error(
            "AddLossOfControl: data must be a table",
        ));
    }
    let entry = decode_entry(state, row)?;
    let mut sim = borrow_state_mut(state)?;
    let entries = sim.active_loss_of_control.entry(unit).or_default();
    entries.push(entry);
    let index = entries.len();
    drop(sim);
    state.push(Val::Num(index as f64));
    Ok(1)
}

pub(crate) fn clear_supplied_entries(state: &mut LuaState) -> LuaResult<u32> {
    let unit = String::from_stack(state, 1)?;
    borrow_state_mut(state)?
        .active_loss_of_control
        .remove(&unit);
    Ok(0)
}

fn decode_entry(state: &mut LuaState, row: Val) -> LuaResult<ActiveLossOfControlData> {
    Ok(ActiveLossOfControlData {
        loc_type: read_field(state, row, "locType")?,
        spell_id: read_field(state, row, "spellID")?,
        display_text: read_field(state, row, "displayText")?,
        icon_texture: read_field(state, row, "iconTexture")?,
        start_time: read_field(state, row, "startTime")?,
        time_remaining: read_field(state, row, "timeRemaining")?,
        duration: read_field(state, row, "duration")?,
        lockout_school: read_field(state, row, "lockoutSchool")?,
        priority: read_field(state, row, "priority")?,
        display_type: read_field(state, row, "displayType")?,
        aura_instance_id: read_field(state, row, "auraInstanceID")?,
    })
}

fn read_field<T: FromStack>(state: &mut LuaState, row: Val, key: &str) -> LuaResult<T> {
    let value = table_get(state, row, key);
    let top = state.top;
    state.push(value);
    let result = T::from_stack(state, -1);
    state.top = top;
    result
}
