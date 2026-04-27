//! `C_AdventureMap` namespace — Broken Isles / Garrison-style adventure-map
//! surface consumed by the Blizzard_AdventureMap addon.
//!
//! Currently implements `GetMapID()`, `Close()`, `GetNumMapInsets()`,
//! `GetMapInsetInfo()`, `GetMapInsetDetailTileInfo()`, and
//! `GetNumZoneChoices()`. Future commits will fill in the rest of the
//! surface (per-zone-choice descriptors, quest offers, dialog hooks).

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, create_table};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::LuaApiMut;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const NAMESPACE: &str = "C_AdventureMap";

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let table_ref = ensure_namespace_table(state);
    table_set_rust_fn_static(state, table_ref, "GetMapID", get_map_id)?;
    table_set_rust_fn_static(state, table_ref, "Close", close)?;
    table_set_rust_fn_static(state, table_ref, "GetNumMapInsets", get_num_map_insets)?;
    table_set_rust_fn_static(state, table_ref, "GetMapInsetInfo", get_map_inset_info)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetMapInsetDetailTileInfo",
        get_map_inset_detail_tile_info,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetNumZoneChoices", get_num_zone_choices)?;
    Ok(())
}

fn ensure_namespace_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(NAMESPACE.as_bytes());
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(table_ref)) = existing {
        return table_ref;
    }
    let new_val = create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}

fn get_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = borrow_state(state)?.adventure_map.map_id;
    state.push(Val::Num(map_id as f64));
    Ok(1)
}

fn close(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    let elapsed = sim.start_time.elapsed().as_secs_f64();
    sim.adventure_map.last_closed = Some(elapsed);
    Ok(0)
}

fn get_num_map_insets(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?
        .adventure_map
        .insets
        .as_ref()
        .map(|insets| insets.len());
    match count {
        Some(n) => state.push(Val::Num(n as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_map_inset_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(slot_index) = lua_one_based_index_to_slot(stack_val(state, 1)) else {
        return Ok(0);
    };
    let descriptor = {
        let sim = borrow_state(state)?;
        sim.adventure_map
            .insets
            .as_ref()
            .and_then(|insets| insets.get(slot_index).cloned())
    };
    let Some(inset) = descriptor else {
        return Ok(0);
    };
    let title = create_string(state, &inset.title);
    let description = create_string(state, &inset.description);
    let collapsed_icon = create_string(state, &inset.collapsed_icon);
    state.push(Val::Num(inset.map_id as f64));
    state.push(title);
    state.push(description);
    state.push(collapsed_icon);
    state.push(Val::Num(inset.area_table_id as f64));
    state.push(Val::Num(inset.num_detail_tiles as f64));
    state.push(Val::Num(inset.normalized_x));
    state.push(Val::Num(inset.normalized_y));
    Ok(8)
}

fn get_map_inset_detail_tile_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(inset_slot) = lua_one_based_index_to_slot(stack_val(state, 1)) else {
        return Ok(0);
    };
    let Some(tile_slot) = lua_one_based_index_to_slot(stack_val(state, 2)) else {
        return Ok(0);
    };
    let file_data_id = {
        let sim = borrow_state(state)?;
        sim.adventure_map
            .insets
            .as_ref()
            .and_then(|insets| insets.get(inset_slot))
            .and_then(|inset| inset.detail_tiles.get(tile_slot).copied())
    };
    let Some(id) = file_data_id else {
        return Ok(0);
    };
    state.push(Val::Num(id as f64));
    Ok(1)
}

fn get_num_zone_choices(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.adventure_map.zone_choices.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

/// Convert a Lua-facing 1-based index to a 0-based slot index. Returns
/// `None` for non-numeric or non-positive arguments so the caller can
/// short-circuit with no return values, matching WoW's "unknown index"
/// path.
fn lua_one_based_index_to_slot(arg: Val) -> Option<usize> {
    let Val::Num(index) = arg else {
        return None;
    };
    if index < 1.0 {
        return None;
    }
    Some(index as usize - 1)
}
