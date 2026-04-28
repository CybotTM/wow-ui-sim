//! Battlefield / LFG probe globals.
//!
//! Migrates 6 entries off `GLOBAL_ZERO_STUBS`:
//!
//! - `GetBattlefieldStatus(index)`     → `(status, mapName, instanceID,
//!   lvlMin, lvlMax, teamSize, registered, eligible, waitingOther)` from
//!   `SimState.battlefield_queue`.
//! - `GetBattlefieldInstanceRunTime()` → 0. The sim doesn't track when
//!   the player entered a battleground instance; callers expect the
//!   retail `ms since entry` integer.
//! - `GetNumBattlegroundEntries()`     → 1 when the queue is active
//!   (`Queued` / `Confirm` / `Active`), else 0.
//! - `GetWorldPVPQueueStatus(index)`   → 7 values with the same status token
//!   family as battlefield queues. Default is the inert
//!   `("none", "", 0, 0, 0, 0, false)` shape Blizzard startup expects.
//! - `GetLFGDungeonInfo(dungeonID)`    → 21 values from `SimState.lfd_dungeons`.
//! - `GetLFGMode(category)`            → `(nil, nil)`. No active LFG.
//! - `GetLFGDungeonNumEncounters(id)`  → `(numEncounters, numCompleted)`.
//! - `GetLFDChoiceOrder()`             → array of dungeon IDs in seeded order.
//! - `GetNumRandomDungeons()`          → count of random-flagged LFD entries.
//! - `GetLFGRandomDungeonInfo(index)`  → `(id, name)` for 1-based random index.
//! - `GetRandomDungeonBestChoice()`    → first random dungeon id, or nil.

use crate::lua_api::methods::{borrow_state, create_string, create_table};
use crate::lua_api::state::BattlefieldStatus;
use crate::lua_api::state_types::LfdDungeonInfo;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

/// `GetBattlefieldStatus(index)` — retail returns 9 values. The sim
/// only models a single active queue, so any index other than the
/// one stored in `battlefield_queue` reports `"none"` with default
/// zeros.
fn get_battlefield_status(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(1);
    let (status, map_name) = {
        let sim = borrow_state(state)?;
        if sim.battlefield_queue.status == BattlefieldStatus::None
            || sim.battlefield_queue.index != index
        {
            (BattlefieldStatus::None, String::new())
        } else {
            (
                sim.battlefield_queue.status,
                sim.battlefield_queue.name.clone(),
            )
        }
    };
    let status_val = create_string(state, status.as_wow_str());
    let map_val = create_string(state, &map_name);
    state.push(status_val); // 1: status
    state.push(map_val); // 2: mapName
    state.push(Val::Num(0.0)); // 3: instanceID
    state.push(Val::Num(0.0)); // 4: levelRangeMin
    state.push(Val::Num(0.0)); // 5: levelRangeMax
    state.push(Val::Num(0.0)); // 6: teamSize (0 for BGs)
    state.push(Val::Bool(false)); // 7: registeredMatch
    state.push(Val::Bool(true)); // 8: eligibleInQueue (matches retail default)
    state.push(Val::Bool(false)); // 9: waitingOnOtherActivity
    Ok(9)
}

/// `GetBattlefieldInstanceRunTime()` — retail returns ms since the
/// player entered a battleground instance. The sim doesn't track that,
/// so always 0.
fn get_battlefield_instance_run_time(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// `GetNumBattlegroundEntries()` — 1 while the queue holds any active
/// state, 0 otherwise.
fn get_num_battleground_entries(state: &mut LuaState) -> LuaResult<u32> {
    let count = if borrow_state(state)?.battlefield_queue.status == BattlefieldStatus::None {
        0.0
    } else {
        1.0
    };
    state.push(Val::Num(count));
    Ok(1)
}

/// `GetWorldPVPQueueStatus(index)` — retail returns 7 values. The sim
/// doesn't seed world PvP queues, so every index reports the inert queue
/// shape Blizzard UI uses for "not queued".
fn get_world_pvp_queue_status(state: &mut LuaState) -> LuaResult<u32> {
    let status = create_string(state, "none");
    let map_name = create_string(state, "");
    state.push(status); // 1: status
    state.push(map_name); // 2: mapName
    state.push(Val::Num(0.0)); // 3: queueID / battleID
    state.push(Val::Num(0.0)); // 4: expireTime
    state.push(Val::Num(0.0)); // 5: averageWaitTime
    state.push(Val::Num(0.0)); // 6: queuedTime
    state.push(Val::Bool(false)); // 7: suspended
    Ok(7)
}

fn push_dungeon_info(state: &mut LuaState, d: &LfdDungeonInfo) {
    let name = create_string(state, &d.name);
    let texture = create_string(state, &d.texture_filename);
    let description = create_string(state, &d.description);
    let map_name = create_string(state, &d.map_name);
    state.push(name);                                    // 1: name
    state.push(Val::Num(d.type_id as f64));              // 2: typeID
    state.push(Val::Num(d.subtype_id as f64));           // 3: subtypeID
    state.push(Val::Num(d.min_level as f64));            // 4: minLevel
    state.push(Val::Num(d.max_level as f64));            // 5: maxLevel
    state.push(Val::Num(d.rec_level as f64));            // 6: recLevel
    state.push(Val::Num(d.min_rec_level as f64));        // 7: minRecLevel
    state.push(Val::Num(d.max_rec_level as f64));        // 8: maxRecLevel
    state.push(Val::Num(d.expansion_level as f64));      // 9: expansionLevel
    state.push(Val::Num(d.group_id as f64));             // 10: groupID
    state.push(texture);                                 // 11: textureFilename
    state.push(Val::Num(d.difficulty as f64));           // 12: difficulty
    state.push(Val::Num(d.max_players as f64));          // 13: maxPlayers
    state.push(description);                             // 14: description
    state.push(Val::Bool(d.is_holiday));                 // 15: isHoliday
    state.push(Val::Num(d.min_players as f64));          // 16: minPlayers
    state.push(map_name);                                // 17: mapName
    state.push(Val::Num(d.min_gear as f64));             // 18: minGear
    state.push(Val::Bool(d.is_scaling_dungeon));         // 19: isScalingDungeon
    state.push(Val::Num(d.dungeon_id as f64));           // 20: dungeonID (echo)
    state.push(Val::Bool(d.is_follower_dungeon));        // 21: isFollowerDungeon
}

/// `GetLFGDungeonInfo(dungeonID)` → 21 values from `lfd_dungeons`, or nil.
fn get_lfg_dungeon_info(state: &mut LuaState) -> LuaResult<u32> {
    let dungeon_id = match stack_val(state, 1) {
        Val::Num(n) => n as i32,
        _ => {
            state.push(Val::Nil);
            return Ok(1);
        }
    };
    let dungeon = borrow_state(state)?
        .lfd_dungeons
        .iter()
        .find(|d| d.dungeon_id == dungeon_id)
        .cloned();
    let Some(d) = dungeon else {
        state.push(Val::Nil);
        return Ok(1);
    };
    push_dungeon_info(state, &d);
    Ok(21)
}

/// `GetLFGMode(category[, queueID])` — retail returns `(mode, submode)`.
/// No active LFG in the sim, so `(nil, nil)`.
fn get_lfg_mode(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

/// `GetLFGDungeonNumEncounters(dungeonID)` → `(numEncounters, numCompleted)`.
fn get_lfg_dungeon_num_encounters(state: &mut LuaState) -> LuaResult<u32> {
    let dungeon_id = stack_i32(state, 1).unwrap_or(0);
    let known = borrow_state(state)?
        .lfd_dungeons
        .iter()
        .any(|d| d.dungeon_id == dungeon_id && dungeon_id > 0);
    if known {
        state.push(Val::Num(3.0)); // numEncounters
        state.push(Val::Num(0.0)); // numCompleted
    } else {
        state.push(Val::Num(0.0));
        state.push(Val::Num(0.0));
    }
    Ok(2)
}

/// `GetLFDChoiceOrder()` → array of dungeon IDs in seeded order.
fn get_lfd_choice_order(state: &mut LuaState) -> LuaResult<u32> {
    let ids = borrow_state(state)?
        .lfd_dungeons
        .iter()
        .map(|d| d.dungeon_id)
        .collect::<Vec<_>>();
    let result = create_table(state);
    if let Val::Table(table_ref) = result {
        for (index, id) in ids.iter().enumerate() {
            if let Some(table) = state.gc.tables.get_mut(table_ref) {
                let _ = table.raw_set(
                    Val::Num(index as f64 + 1.0),
                    Val::Num(*id as f64),
                    &state.gc.string_arena,
                );
            }
        }
        state.gc.barrier_back(table_ref);
    }
    state.push(result);
    Ok(1)
}

/// `GetNumRandomDungeons()` → count of lfd_dungeons where is_random=true.
fn get_num_random_dungeons(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?
        .lfd_dungeons
        .iter()
        .filter(|d| d.is_random)
        .count();
    state.push(Val::Num(count as f64));
    Ok(1)
}

/// `GetLFGRandomDungeonInfo(index)` → `(id, name)` for 1-based random subset index.
fn get_lfg_random_dungeon_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(1) as usize;
    let found = borrow_state(state)?
        .lfd_dungeons
        .iter()
        .filter(|d| d.is_random)
        .nth(index.saturating_sub(1))
        .map(|d| (d.dungeon_id, d.name.clone()));
    let Some((id, name)) = found else {
        state.push(Val::Nil);
        return Ok(1);
    };
    state.push(Val::Num(id as f64));
    let name_val = create_string(state, &name);
    state.push(name_val);
    Ok(2)
}

/// `GetRandomDungeonBestChoice()` → first random dungeon id, or nil.
fn get_random_dungeon_best_choice(state: &mut LuaState) -> LuaResult<u32> {
    let found = borrow_state(state)?
        .lfd_dungeons
        .iter()
        .find(|d| d.is_random)
        .map(|d| d.dungeon_id);
    match found {
        Some(id) => {
            state.push(Val::Num(id as f64));
            Ok(1)
        }
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetBattlefieldStatus", get_battlefield_status)?;
    LuaApiMut::register_function(
        lua,
        "GetBattlefieldInstanceRunTime",
        get_battlefield_instance_run_time,
    )?;
    LuaApiMut::register_function(
        lua,
        "GetNumBattlegroundEntries",
        get_num_battleground_entries,
    )?;
    LuaApiMut::register_function(lua, "GetWorldPVPQueueStatus", get_world_pvp_queue_status)?;
    LuaApiMut::register_function(lua, "GetLFGDungeonInfo", get_lfg_dungeon_info)?;
    LuaApiMut::register_function(lua, "GetLFGMode", get_lfg_mode)?;
    LuaApiMut::register_function(
        lua,
        "GetLFGDungeonNumEncounters",
        get_lfg_dungeon_num_encounters,
    )?;
    LuaApiMut::register_function(lua, "GetLFDChoiceOrder", get_lfd_choice_order)?;
    LuaApiMut::register_function(lua, "GetNumRandomDungeons", get_num_random_dungeons)?;
    LuaApiMut::register_function(
        lua,
        "GetLFGRandomDungeonInfo",
        get_lfg_random_dungeon_info,
    )?;
    LuaApiMut::register_function(
        lua,
        "GetRandomDungeonBestChoice",
        get_random_dungeon_best_choice,
    )?;
    Ok(())
}
