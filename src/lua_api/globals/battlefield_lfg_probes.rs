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
//! - `GetLFGDungeonInfo(dungeonID)`    → nil. The sim doesn't seed a
//!   dungeon list.
//! - `GetLFGMode(category)`            → `(nil, nil)`. No active LFG.
//! - `GetLFGDungeonNumEncounters(id)`  → 0 (same rationale as
//!   `GetLFGDungeonInfo`).

use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_api::state::BattlefieldStatus;
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

/// `GetLFGDungeonInfo(dungeonID)` — retail returns ~18 values; we
/// don't seed a dungeon list yet, so always nil.
fn get_lfg_dungeon_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

/// `GetLFGMode(category[, queueID])` — retail returns `(mode, submode)`.
/// No active LFG in the sim, so `(nil, nil)`.
fn get_lfg_mode(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

/// `GetLFGDungeonNumEncounters(dungeonID)` — 0 for every id since no
/// dungeons are seeded.
fn get_lfg_dungeon_num_encounters(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
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
    Ok(())
}
