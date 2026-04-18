//! `C_Map` probe surface backed by `SimState.maps` +
//! `SimState.player_map_position`.
//!
//! Migrates 7 entries off `NAMESPACE_NIL_STUBS`:
//!
//! - `C_Map.GetMapArtID(uiMapID)` — returns the `art_id` for the
//!   seeded map, or nothing (retail `mayreturnnothing`).
//! - `C_Map.GetMapInfo(uiMapID)` — returns a `UiMapDetails`-shaped
//!   table for seeded maps, or nothing for unknown ids.
//! - `C_Map.GetMapChildrenInfo(uiMapID, mapType?, allDescendants?)`
//!   — returns the children as an array of `UiMapDetails` tables.
//!   `mapType` filters by the UIMapType enum; `allDescendants`
//!   recursively walks the subtree. Returns nothing when the map is
//!   unknown, an empty array when it exists but has no children
//!   matching the filter.
//! - `C_Map.GetPlayerMapPosition(uiMapID, unitToken)` — returns
//!   `{x, y}` vector2 from `SimState.player_map_position` for any
//!   known map, or `nil` for an unknown map / non-player unit.
//! - `C_Map.GetBestMapForUnit(unitToken)` — returns the seeded player
//!   map id (`2248`) for `"player"`.
//! - `C_Map.GetFallbackWorldMapID()` — returns the seeded player map
//!   id (`2248`).
//! - `C_Map.MapHasArt(uiMapID)` — true for positive map ids.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{
    borrow_state, create_string, create_table, table_set, val_to_string,
};
use crate::lua_api::state::MapData;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::collections::HashSet;

pub(super) fn register_c_map_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Map")?;
    table_set_rust_fn_static(state, table_ref, "GetMapArtID", c_map_get_map_art_id)?;
    table_set_rust_fn_static(state, table_ref, "GetMapInfo", c_map_get_map_info)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetMapChildrenInfo",
        c_map_get_map_children_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetPlayerMapPosition",
        c_map_get_player_map_position,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetBestMapForUnit",
        c_map_get_best_map_for_unit,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetFallbackWorldMapID",
        c_map_get_fallback_world_map_id,
    )?;
    table_set_rust_fn_static(state, table_ref, "MapHasArt", c_map_map_has_art)?;
    Ok(())
}

const DEFAULT_PLAYER_MAP_ID: i32 = 2248;

fn c_map_get_map_art_id(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    let art_id = borrow_state(state)?.maps.get(&ui_map_id).map(|m| m.art_id);
    let Some(art_id) = art_id else {
        return Ok(0);
    };
    state.push(Val::Num(art_id as f64));
    Ok(1)
}

fn c_map_get_map_info(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    let map = borrow_state(state)?.maps.get(&ui_map_id).cloned();
    let Some(map) = map else {
        return Ok(0);
    };
    let details = push_map_details_table(state, &map);
    state.push(details);
    Ok(1)
}

fn push_map_details_table(state: &mut LuaState, map: &MapData) -> Val {
    let t = create_table(state);
    let name = create_string(state, &map.name);
    table_set(state, t, "mapID", Val::Num(map.ui_map_id as f64));
    table_set(state, t, "name", name);
    table_set(state, t, "mapType", Val::Num(map.map_type as f64));
    table_set(state, t, "parentMapID", Val::Num(map.parent_map_id as f64));
    table_set(state, t, "flags", Val::Num(map.flags as f64));
    t
}

fn collect_children(
    maps: &std::collections::HashMap<i32, MapData>,
    root: i32,
    all_descendants: bool,
    map_type_filter: Option<i32>,
) -> Vec<MapData> {
    let Some(root_map) = maps.get(&root) else {
        return Vec::new();
    };

    let mut out: Vec<MapData> = Vec::new();
    let mut visited: HashSet<i32> = HashSet::new();
    let mut frontier: Vec<i32> = root_map.child_map_ids.clone();

    while let Some(child_id) = frontier.pop() {
        if !visited.insert(child_id) {
            continue;
        }
        let Some(child) = maps.get(&child_id) else {
            continue;
        };
        if map_type_filter.is_none_or(|filter| child.map_type == filter) {
            out.push(child.clone());
        }
        if all_descendants {
            frontier.extend(child.child_map_ids.iter().copied());
        }
    }

    out
}

fn c_map_get_map_children_info(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    let map_type_filter = match stack_val(state, 2) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    };
    let all_descendants = matches!(stack_val(state, 3), Val::Bool(true));

    let children = {
        let sim = borrow_state(state)?;
        if !sim.maps.contains_key(&ui_map_id) {
            return Ok(0);
        }
        collect_children(&sim.maps, ui_map_id, all_descendants, map_type_filter)
    };

    let array = create_table(state);
    for (index, child) in children.into_iter().enumerate() {
        let entry = push_map_details_table(state, &child);
        set_table_array(state, array, index as i64 + 1, entry);
    }
    state.push(array);
    Ok(1)
}

fn c_map_get_player_map_position(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    let unit_token = val_to_string(state, stack_val(state, 2)).unwrap_or_default();
    let is_player = matches!(unit_token.as_str(), "player" | "");

    let position = {
        let sim = borrow_state(state)?;
        if !sim.maps.contains_key(&ui_map_id) || !is_player {
            None
        } else {
            Some(sim.player_map_position)
        }
    };

    let Some(position) = position else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let t = create_table(state);
    table_set(state, t, "x", Val::Num(position.0));
    table_set(state, t, "y", Val::Num(position.1));
    state.push(t);
    Ok(1)
}

fn c_map_get_best_map_for_unit(state: &mut LuaState) -> LuaResult<u32> {
    let unit_token = val_to_string(state, stack_val(state, 1)).unwrap_or_default();
    if !matches!(unit_token.as_str(), "" | "player") {
        return Ok(0);
    }
    state.push(Val::Num(DEFAULT_PLAYER_MAP_ID as f64));
    Ok(1)
}

fn c_map_get_fallback_world_map_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(DEFAULT_PLAYER_MAP_ID as f64));
    Ok(1)
}

fn c_map_map_has_art(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    state.push(Val::Bool(ui_map_id > 0));
    Ok(1)
}
