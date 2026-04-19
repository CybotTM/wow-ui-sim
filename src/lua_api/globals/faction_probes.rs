//! Faction / reputation probe + selection globals.
//!
//! Migrates 5 entries off stubs:
//!
//! - `GetFactionInfoByID(factionID)` → 16-value retail shape from a
//!   matching `SimState.factions` entry, or nil when unknown.
//! - `GetGuildFactionInfo()`         → 6-value shape for the synthetic
//!   guild faction: name from `world.guild_name`, other fields fixed
//!   to Exalted / Friendly defaults until the sim models guild rep.
//! - `GetSelectedFaction()`          → `SimState.selected_faction_index`.
//! - `SetSelectedFaction(index)`     → write it back + fire
//!   `UPDATE_FACTION`.
//! - `SetWatchedFaction(index)`      → write
//!   `SimState.watched_faction_index` and flip the matching entry's
//!   `is_watched` flag; fires `UPDATE_FACTION`.

use crate::event::Event;
use crate::lua_api::globals::reputation_data;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_set,
};
use crate::lua_bridge::stack_val;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn stack_u32(state: &LuaState, index: i32) -> Option<u32> {
    match stack_val(state, index) {
        Val::Num(n) if n >= 0.0 => Some(n as u32),
        _ => None,
    }
}

fn push_update_faction_event(state: &mut LuaState) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: "UPDATE_FACTION".to_string(),
        args: Vec::new(),
    });
    Ok(())
}

/// `GetFactionInfoByID(factionID)` — retail: 16 values per row. When
/// the id isn't in `SimState.factions` we return nothing (matches
/// retail's "unknown faction" behaviour).
fn get_faction_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let Some(faction_id) = stack_u32(state, 1) else {
        return Ok(0);
    };
    let Some(entry) = borrow_state(state)?
        .factions
        .iter()
        .find(|e| e.faction_id == faction_id)
        .cloned()
    else {
        return Ok(0);
    };
    let name = create_string(state, &entry.name);
    let description = create_string(state, &entry.description);
    state.push(name); // 1: name
    state.push(description); // 2: description
    state.push(Val::Num(entry.standing as f64)); // 3: standingID
    state.push(Val::Num(entry.bottom as f64)); // 4: bottomValue
    state.push(Val::Num(entry.top as f64)); // 5: topValue
    state.push(Val::Num(entry.earned as f64)); // 6: earnedValue
    state.push(Val::Bool(entry.at_war)); // 7: atWarWith
    state.push(Val::Bool(entry.can_toggle_at_war)); // 8: canToggleAtWar
    state.push(Val::Bool(entry.is_header)); // 9: isHeader
    state.push(Val::Bool(entry.is_collapsed)); // 10: isCollapsed
    state.push(Val::Bool(entry.has_rep)); // 11: hasRep
    state.push(Val::Bool(entry.is_watched)); // 12: isWatched
    state.push(Val::Bool(entry.is_child)); // 13: isChild
    state.push(Val::Num(entry.faction_id as f64)); // 14: factionID
    state.push(Val::Bool(entry.has_bonus_rep_gain)); // 15: hasBonusRepGain
    state.push(Val::Bool(entry.can_be_lfg_bonus)); // 16: canBeLFGBonus
    Ok(16)
}

/// `GetGuildFactionInfo()` — retail: `(name, description, standingID,
/// bottomValue, topValue, earnedValue)`. We synthesise a guild faction
/// from `WorldState.guild_name` at Exalted (8) standing; the sim
/// doesn't model guild reputation progression.
fn get_guild_faction_info(state: &mut LuaState) -> LuaResult<u32> {
    let guild_name = borrow_state(state)?.world.guild_name.clone();
    let Some(guild_name) = guild_name else {
        return Ok(0);
    };
    let name = create_string(state, &guild_name);
    let description = create_string(state, "Guild");
    state.push(name);
    state.push(description);
    state.push(Val::Num(8.0)); // standingID: Exalted
    state.push(Val::Num(0.0)); // bottomValue
    state.push(Val::Num(1000.0)); // topValue
    state.push(Val::Num(1000.0)); // earnedValue (capped)
    Ok(6)
}

/// `GetSelectedFaction()` — 1-based reputation-window index.
fn get_selected_faction(state: &mut LuaState) -> LuaResult<u32> {
    let idx = borrow_state(state)?.selected_faction_index;
    state.push(Val::Num(idx as f64));
    Ok(1)
}

/// `SetSelectedFaction(index)` — select a reputation-window row.
/// Retail clamps to the valid range silently; we do the same.
fn set_selected_faction(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    {
        let mut sim = borrow_state_mut(state)?;
        let len = sim.factions.len() as i32;
        sim.selected_faction_index = index.clamp(0, len);
    }
    push_update_faction_event(state)?;
    Ok(0)
}

/// `SetWatchedFaction(index)` — show the faction on the XP bar and
/// clear the previous watch flag. `index == 0` stops watching.
fn set_watched_faction(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    {
        let mut sim = borrow_state_mut(state)?;
        let len = sim.factions.len() as i32;
        let clamped = index.clamp(0, len);
        for entry in sim.factions.iter_mut() {
            entry.is_watched = false;
        }
        if clamped > 0 {
            if let Some(entry) = sim.factions.get_mut((clamped - 1) as usize) {
                entry.is_watched = true;
            }
        }
        sim.watched_faction_index = clamped;
    }
    push_update_faction_event(state)?;
    Ok(0)
}

fn reputation_entry_table(
    state: &mut LuaState,
    entry: &reputation_data::FactionEntry,
    _faction_id: i32,
) -> Val {
    let table = create_table(state);
    let name = create_string(state, entry.name);
    let description = create_string(state, entry.description);
    table_set(state, table, "name", name);
    table_set(state, table, "description", description);
    table_set(state, table, "factionID", Val::Num(entry.faction_id as f64));
    table_set(state, table, "reaction", Val::Num(entry.reaction as f64));
    table_set(state, table, "standing", Val::Num(entry.standing as f64));
    table_set(state, table, "currentReactionThreshold", Val::Num(0.0));
    table_set(
        state,
        table,
        "nextReactionThreshold",
        Val::Num(entry.top_value as f64),
    );
    table_set(
        state,
        table,
        "currentStanding",
        Val::Num(entry.standing as f64),
    );
    table_set(state, table, "topValue", Val::Num(entry.top_value as f64));
    table_set(state, table, "isHeader", Val::Bool(entry.is_header));
    table_set(state, table, "isCollapsed", Val::Bool(entry.is_collapsed));
    table_set(state, table, "isChild", Val::Bool(entry.is_child));
    table_set(
        state,
        table,
        "isAccountWide",
        Val::Bool(entry.is_account_wide),
    );
    table_set(state, table, "currentReactionThreshold", Val::Num(0.0));
    table_set(
        state,
        table,
        "nextReactionThreshold",
        Val::Num(entry.top_value as f64),
    );
    table_set(
        state,
        table,
        "currentStanding",
        Val::Num(entry.standing as f64),
    );
    table
}

fn register_reputation_namespace(lua: &mut rilua::Lua) -> crate::Result<()> {
    let existing_reputation = LuaApiMut::get_global_val(lua, "C_Reputation");
    let state = lua.state_mut();
    let reputation = match existing_reputation {
        Val::Table(table) => Val::Table(table),
        _ => create_table(state),
    };
    let Val::Table(reputation_ref) = reputation else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn_static(
        state,
        reputation_ref,
        "GetFactionDataByID",
        reputation_get_faction_data_by_id,
    )?;
    table_set_rust_fn_static(
        state,
        reputation_ref,
        "IsFactionParagon",
        reputation_is_faction_paragon,
    )?;
    table_set_rust_fn_static(
        state,
        reputation_ref,
        "GetFactionParagonInfo",
        reputation_get_faction_paragon_info,
    )?;
    table_set_rust_fn_static(
        state,
        reputation_ref,
        "GetNumFactions",
        reputation_get_num_factions,
    )?;
    table_set_rust_fn_static(
        state,
        reputation_ref,
        "GetFactionInfo",
        reputation_get_faction_info,
    )?;
    table_set_rust_fn_static(
        state,
        reputation_ref,
        "GetWatchedFactionData",
        reputation_get_watched_faction_data,
    )?;
    table_set_rust_fn_static(
        state,
        reputation_ref,
        "SetWatchedFactionByID",
        reputation_set_watched_faction_by_id,
    )?;
    LuaApiMut::set_global_val(lua, "C_Reputation", reputation)?;
    Ok(())
}

fn reputation_get_faction_data_by_id(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn reputation_is_faction_paragon(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn reputation_get_faction_paragon_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn reputation_get_num_factions(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(reputation_data::num_factions() as f64));
    Ok(1)
}

fn reputation_get_faction_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    let Some(entry) = reputation_data::get_faction_by_index(index) else {
        return Ok(0);
    };
    let table = reputation_entry_table(state, entry, index);
    state.push(table);
    Ok(1)
}

fn reputation_get_watched_faction_data(state: &mut LuaState) -> LuaResult<u32> {
    let Some(entry) = reputation_data::watched_faction() else {
        return Ok(0);
    };
    let table = reputation_entry_table(state, entry, entry.faction_id);
    state.push(table);
    Ok(1)
}

fn reputation_set_watched_faction_by_id(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetFactionInfoByID", get_faction_info_by_id)?;
    LuaApiMut::register_function(lua, "GetGuildFactionInfo", get_guild_faction_info)?;
    LuaApiMut::register_function(lua, "GetSelectedFaction", get_selected_faction)?;
    LuaApiMut::register_function(lua, "SetSelectedFaction", set_selected_faction)?;
    LuaApiMut::register_function(lua, "SetWatchedFaction", set_watched_faction)?;
    register_reputation_namespace(lua)?;
    Ok(())
}
