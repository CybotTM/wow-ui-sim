//! Rilua A_Admin handlers — Encounter.
//!
//! Extracted from rilua_admin_world.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in rilua_admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use crate::lua_api::globals::rilua_admin::opt_string_stack;
use crate::lua_api::rilua_methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ── Encounter ─────────────────────────────────────────────────────────────────

pub(super) fn simulate_boss_kill(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::{Event, EventArg};
    let encounter_id = i32::from_stack(state, 1)?;
    let name = String::from_stack(state, 2)?;
    let difficulty_id = i32::from_stack(state, 3)?;
    let group_size = i32::from_stack(state, 4)?;
    let mut st = borrow_state_mut(state)?;
    st.events.push(Event {
        name: "ENCOUNTER_END".to_string(),
        args: vec![
            EventArg::Number(encounter_id as f64),
            EventArg::String(name.clone()),
            EventArg::Number(difficulty_id as f64),
            EventArg::Number(group_size as f64),
            EventArg::Number(1.0), // success
        ],
    });
    st.events.push(Event {
        name: "BOSS_KILL".to_string(),
        args: vec![
            EventArg::Number(encounter_id as f64),
            EventArg::String(name),
        ],
    });
    Ok(0)
}

pub(super) fn start_loot_roll(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::{Event, EventArg};
    use crate::lua_api::state::LootRollInfo;
    use crate::lua_bridge::stack_val;

    let roll_id = i32::from_stack(state, 1)?;
    let roll_time = f64::from_stack(state, 2)?;
    let item_name = opt_string_stack(state, 3, "");
    let item_texture = opt_string_stack(state, 4, "");
    let item_quality = match stack_val(state, 5) {
        Val::Num(n) => n as i32,
        _ => 4,
    };
    let item_level = match stack_val(state, 6) {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    let item_link = opt_string_stack(state, 7, "");

    let info = LootRollInfo {
        roll_id,
        roll_time,
        texture: item_texture,
        name: item_name,
        count: 1,
        quality: item_quality,
        bind_on_pickup: true,
        can_need: true,
        can_greed: true,
        can_disenchant: false,
        disenchant_level: 0,
        item_level,
        item_link,
    };
    let mut st = borrow_state_mut(state)?;
    st.world.loot_rolls.insert(roll_id, info);
    st.events.push(Event {
        name: "START_LOOT_ROLL".to_string(),
        args: vec![
            EventArg::Number(roll_id as f64),
            EventArg::Number(roll_time),
        ],
    });
    Ok(0)
}

pub(super) fn end_loot_roll(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::{Event, EventArg};
    let roll_id = i32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.world.loot_rolls.remove(&roll_id);
    st.events.push(Event {
        name: "LOOT_ROLLS_COMPLETE".to_string(),
        args: vec![EventArg::Number(roll_id as f64)],
    });
    Ok(0)
}
