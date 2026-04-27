//! Legacy archaeology probes consumed by `Blizzard_ArchaeologyUI`.
//!
//! These globals predate the `C_*` namespacing pass; they live at the
//! top-level Lua surface and are not exposed under `C_ResearchInfo`.
//!
//! - `GetArchaeologyInfo() → professionName: string`
//! - `GetNumArchaeologyRaces() → number`
//! - `GetArchaeologyRaceInfo(raceIndex: number, getCurrentArtifact: bool?) →
//!   name: string, texture: number, raceItemID: number, currencyAmount: number,
//!   projectAmount: number` (returns nothing for out-of-range indices)
//! - `GetNumArtifactsByRace(raceIndex: number) → number` (0 for out-of-range)
//!
//! Without `GetArchaeologyInfo` the addon errors out at
//! `Blizzard_ArchaeologyUI.lua:102` during `OnLoad` and `ArchaeologyFrame`
//! becomes a half-initialized table.

use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn get_archaeology_info(state: &mut LuaState) -> LuaResult<u32> {
    let profession = borrow_state(state)?.archaeology.profession_name.clone();
    let val = create_string(state, &profession);
    state.push(val);
    Ok(1)
}

fn get_num_archaeology_races(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.archaeology.races.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

/// Race-row tuple pushed onto the Lua stack by `GetArchaeologyRaceInfo`.
struct RaceRow {
    name: String,
    texture: u32,
    race_item_id: u32,
    currency_amount: i32,
    project_amount: i32,
}

fn race_row_for(state: &mut LuaState, race_index: i32) -> LuaResult<Option<RaceRow>> {
    let sim = borrow_state(state)?;
    let Some(race) = sim.archaeology.race_at(race_index) else {
        return Ok(None);
    };
    Ok(Some(RaceRow {
        name: race.name.clone(),
        texture: race.texture,
        race_item_id: race.race_item_id,
        currency_amount: race.currency_amount,
        project_amount: race.project_amount,
    }))
}

fn push_race_row(state: &mut LuaState, row: RaceRow) -> LuaResult<u32> {
    let name_val = create_string(state, &row.name);
    state.push(name_val);
    state.push(Val::Num(row.texture as f64));
    state.push(Val::Num(row.race_item_id as f64));
    state.push(Val::Num(row.currency_amount as f64));
    state.push(Val::Num(row.project_amount as f64));
    Ok(5)
}

fn get_archaeology_race_info(state: &mut LuaState) -> LuaResult<u32> {
    let race_index = i32::from_stack(state, 1)?;
    // Arg 2 (`getCurrentArtifact: bool?`) selects an alternate set of returns
    // for the artifact page; the race-summary surface ignores it. The
    // active-artifact branch is filed as a separate priority entry.
    match race_row_for(state, race_index)? {
        Some(row) => push_race_row(state, row),
        None => Ok(0),
    }
}

fn get_num_artifacts_by_race(state: &mut LuaState) -> LuaResult<u32> {
    let race_index = i32::from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.archaeology
            .race_at(race_index)
            .map(|r| r.artifacts.len() as f64)
            .unwrap_or(0.0)
    };
    state.push(Val::Num(count));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetArchaeologyInfo", get_archaeology_info)?;
    LuaApiMut::register_function(lua, "GetNumArchaeologyRaces", get_num_archaeology_races)?;
    LuaApiMut::register_function(lua, "GetArchaeologyRaceInfo", get_archaeology_race_info)?;
    LuaApiMut::register_function(lua, "GetNumArtifactsByRace", get_num_artifacts_by_race)?;
    Ok(())
}
