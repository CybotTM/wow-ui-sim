//! A_Admin encounter & loot roll simulation.
//!
//! SimulateBossKill fires ENCOUNTER_END + BOSS_KILL events.
//! StartLootRoll stores roll info in SimState and fires START_LOOT_ROLL.
//! EndLootRoll removes roll info and fires LOOT_ROLLS_COMPLETE.

use crate::lua_api::frame::get_sim_state;
use crate::lua_api::state::{LootRollInfo, SimState};
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_encounter_api(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    super::admin_api::set_fn(lua, t, "SimulateBossKill", {
        move |lua, (encounter_id, name, difficulty_id, group_size): (i32, String, i32, i32)| {
            fire_event(
                lua,
                "ENCOUNTER_END",
                &[
                    Value::Number(encounter_id as f64),
                    Value::String(lua.create_string(&name)?),
                    Value::Number(difficulty_id as f64),
                    Value::Number(group_size as f64),
                    Value::Number(1.0), // success=1
                ],
            )?;
            fire_event(
                lua,
                "BOSS_KILL",
                &[
                    Value::Number(encounter_id as f64),
                    Value::String(lua.create_string(&name)?),
                ],
            )?;
            Ok(())
        }
    })?;
    register_loot_roll_api(lua, t, Rc::clone(&state))?;
    register_loot_globals(lua)?;
    Ok(())
}

/// Fire a WoW event immediately via the registered FireEvent global.
fn fire_event(lua: &Lua, event_name: &str, args: &[Value]) -> Result<()> {
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    let mut call_args = vec![Value::String(lua.create_string(event_name)?)];
    call_args.extend(args.iter().cloned());
    fire.call(mlua::MultiValue::from_vec(call_args))
}

fn register_loot_roll_api(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::admin_api::set_fn(lua, t, "StartLootRoll", {
        let s = Rc::clone(&state);
        move |lua, args: LootRollArgs| {
            let roll_id = args.roll_id;
            let roll_time = args.roll_time;
            let info = build_loot_roll_info(&args);
            s.borrow_mut().world.loot_rolls.insert(roll_id, info);
            fire_event(
                lua,
                "START_LOOT_ROLL",
                &[Value::Number(roll_id as f64), Value::Number(roll_time)],
            )?;
            Ok(())
        }
    })?;
    super::admin_api::set_fn(lua, t, "EndLootRoll", {
        let s = Rc::clone(&state);
        move |lua, roll_id: i32| {
            s.borrow_mut().world.loot_rolls.remove(&roll_id);
            fire_event(lua, "LOOT_ROLLS_COMPLETE", &[Value::Number(roll_id as f64)])?;
            Ok(())
        }
    })?;
    Ok(())
}

struct LootRollArgs {
    roll_id: i32,
    roll_time: f64,
    item_name: String,
    item_texture: String,
    item_quality: i32,
    item_level: i32,
    item_link: String,
}

impl mlua::FromLuaMulti for LootRollArgs {
    fn from_lua_multi(values: mlua::MultiValue, _lua: &Lua) -> Result<Self> {
        let mut it = values.into_iter();
        let roll_id = it.next().and_then(|v| v.as_integer()).unwrap_or(1) as i32;
        let roll_time = it.next().and_then(as_f64).unwrap_or(30.0);
        let item_name = it
            .next()
            .and_then(|v| v.as_string_lossy())
            .unwrap_or_default();
        let item_texture = it
            .next()
            .and_then(|v| v.as_string_lossy())
            .unwrap_or_default();
        let item_quality = it.next().and_then(|v| v.as_integer()).unwrap_or(4) as i32;
        let item_level = it.next().and_then(|v| v.as_integer()).unwrap_or(0) as i32;
        let item_link = it
            .next()
            .and_then(|v| v.as_string_lossy())
            .unwrap_or_default();
        Ok(Self {
            roll_id,
            roll_time,
            item_name,
            item_texture,
            item_quality,
            item_level,
            item_link,
        })
    }
}

/// Extract f64 from Number or Integer Value.
fn as_f64(v: Value) -> Option<f64> {
    match v {
        Value::Number(n) => Some(n),
        Value::Integer(i) => Some(i as f64),
        _ => None,
    }
}

fn build_loot_roll_info(args: &LootRollArgs) -> LootRollInfo {
    LootRollInfo {
        roll_id: args.roll_id,
        roll_time: args.roll_time,
        texture: args.item_texture.clone(),
        name: args.item_name.clone(),
        count: 1,
        quality: args.item_quality,
        bind_on_pickup: true,
        can_need: true,
        can_greed: true,
        can_disenchant: false,
        disenchant_level: 0,
        item_level: args.item_level,
        item_link: args.item_link.clone(),
    }
}

// ---------------------------------------------------------------------------
// Global loot functions (overwrite generated stubs)
// ---------------------------------------------------------------------------

/// Register GetLootRollItemInfo, GetLootRollItemLink, GetActiveLootRollIDs
/// backed by SimState.world.loot_rolls.
fn register_loot_globals(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "GetLootRollItemInfo",
        lua.create_function(get_loot_roll_item_info)?,
    )?;
    g.set(
        "GetLootRollItemLink",
        lua.create_function(get_loot_roll_item_link)?,
    )?;
    g.set(
        "GetLootRollTimeLeft",
        lua.create_function(get_loot_roll_time_left)?,
    )?;
    g.set(
        "GetActiveLootRollIDs",
        lua.create_function(get_active_loot_roll_ids)?,
    )?;
    Ok(())
}

/// GetLootRollItemInfo(rollID) → texture, name, count, quality, bop, canNeed, canGreed, canDE, deLevel, ilvl, ?, ?
fn get_loot_roll_item_info(lua: &Lua, roll_id: i32) -> Result<mlua::MultiValue> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let Some(info) = state.world.loot_rolls.get(&roll_id) else {
        return Ok(mlua::MultiValue::new());
    };
    Ok(mlua::MultiValue::from_vec(vec![
        Value::String(lua.create_string(&info.texture)?),
        Value::String(lua.create_string(&info.name)?),
        Value::Integer(info.count as i64),
        Value::Integer(info.quality as i64),
        Value::Boolean(info.bind_on_pickup),
        Value::Boolean(info.can_need),
        Value::Boolean(info.can_greed),
        Value::Boolean(info.can_disenchant),
        Value::Integer(info.disenchant_level as i64),
        Value::Integer(info.item_level as i64),
        Value::Integer(0),     // encounterID (unused)
        Value::Boolean(false), // isArtifact
    ]))
}

fn get_loot_roll_item_link(lua: &Lua, roll_id: i32) -> Result<Value> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    match state.world.loot_rolls.get(&roll_id) {
        Some(info) if !info.item_link.is_empty() => {
            Ok(Value::String(lua.create_string(&info.item_link)?))
        }
        _ => Ok(Value::Nil),
    }
}

fn get_loot_roll_time_left(lua: &Lua, roll_id: i32) -> Result<f64> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    Ok(state
        .world
        .loot_rolls
        .get(&roll_id)
        .map_or(0.0, |r| r.roll_time))
}

fn get_active_loot_roll_ids(lua: &Lua, _: ()) -> Result<mlua::Table> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let t = lua.create_table()?;
    for (i, &roll_id) in state.world.loot_rolls.keys().enumerate() {
        t.raw_set(i + 1, roll_id)?;
    }
    Ok(t)
}
