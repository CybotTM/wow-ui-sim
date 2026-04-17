//! Instance + mirror-timer probe globals backed by `SimState.world`.
//!
//! Migrates 3 entries off `GLOBAL_ZERO_STUBS`:
//!
//! - `GetInstanceInfo()`          → 10 values assembled from
//!   `WorldState.instance_*` (name, type, difficulty id + name, max
//!   players, dynamic-difficulty flags, instance id, group size, LFG
//!   dungeon id).
//! - `GetMirrorTimerInfo(index)`  → 7 values from
//!   `WorldState.mirror_timers[index-1]` (name, startValue, maxValue,
//!   scale, paused, label, spellID). Returns nothing for out-of-range
//!   indices (matches `mayreturnnothing: true` in apis.yaml).
//! - `GetMirrorTimerProgress(name)` → current progress for the timer
//!   with the matching `name`, or nil when absent.

use crate::lua_api::methods::{borrow_state, create_string, val_to_string};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn stack_string(state: &LuaState, index: i32) -> Option<String> {
    val_to_string(state, stack_val(state, index))
}

fn get_instance_info(state: &mut LuaState) -> LuaResult<u32> {
    let (
        name,
        instance_type,
        difficulty_id,
        difficulty_name,
        max_players,
        dynamic_difficulty,
        is_dynamic,
        instance_id,
        group_size,
        lfg_dungeon_id,
    ) = {
        let w = &borrow_state(state)?.world;
        (
            w.instance_name.clone(),
            w.instance_type.clone(),
            w.instance_difficulty,
            w.instance_difficulty_name.clone(),
            w.instance_max_players,
            w.instance_dynamic_difficulty,
            w.instance_is_dynamic,
            w.instance_id,
            w.instance_group_size,
            w.instance_lfg_dungeon_id,
        )
    };

    let name_val = create_string(state, &name);
    state.push(name_val);
    let type_val = create_string(state, &instance_type);
    state.push(type_val);
    state.push(Val::Num(difficulty_id as f64));
    let diff_name_val = create_string(state, &difficulty_name);
    state.push(diff_name_val);
    state.push(Val::Num(max_players as f64));
    state.push(Val::Num(dynamic_difficulty as f64));
    state.push(Val::Bool(is_dynamic));
    state.push(Val::Num(instance_id as f64));
    state.push(Val::Num(group_size as f64));
    match lfg_dungeon_id {
        Some(id) => state.push(Val::Num(id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(10)
}

fn get_mirror_timer_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    let timer = usize::try_from(index.saturating_sub(1))
        .ok()
        .and_then(|idx| borrow_state(state).ok()?.world.mirror_timers.get(idx).cloned());
    let Some(t) = timer else {
        return Ok(0);
    };

    let name_val = create_string(state, &t.name);
    state.push(name_val);
    state.push(Val::Num(t.start_value));
    state.push(Val::Num(t.max_value));
    state.push(Val::Num(t.scale));
    state.push(Val::Num(t.paused as f64));
    let label_val = create_string(state, &t.label);
    state.push(label_val);
    state.push(Val::Num(t.spell_id as f64));
    Ok(7)
}

fn get_mirror_timer_progress(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = stack_string(state, 1) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let progress = borrow_state(state)?
        .world
        .mirror_timers
        .iter()
        .find(|t| t.name == name)
        .map(|t| t.progress);
    match progress {
        Some(value) => state.push(Val::Num(value)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetInstanceInfo", get_instance_info)?;
    LuaApiMut::register_function(lua, "GetMirrorTimerInfo", get_mirror_timer_info)?;
    LuaApiMut::register_function(lua, "GetMirrorTimerProgress", get_mirror_timer_progress)?;
    Ok(())
}
