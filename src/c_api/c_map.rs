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
//! - `C_Map.RequestPreloadMap(uiMapID)` — queues map art + overlay textures.

use super::helpers::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_set, val_to_string,
};
use crate::lua_api::state::MapData;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::collections::HashSet;

pub(crate) fn register_c_map_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Map")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetMapArtBackgroundAtlas",
        c_map_get_map_art_background_atlas,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetMapArtID", c_map_get_map_art_id)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetMapArtLayerTextures",
        c_map_get_map_art_layer_textures,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetMapArtLayers",
        c_map_get_map_art_layers,
    )?;
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
        "GetCurrentMapID",
        c_map_get_current_map_id,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetFallbackWorldMapID",
        c_map_get_fallback_world_map_id,
    )?;
    table_set_rust_fn_static(state, table_ref, "MapHasArt", c_map_map_has_art)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "RequestPreloadMap",
        c_map_request_preload_map,
    )?;
    Ok(())
}

const DEFAULT_PLAYER_MAP_ID: i32 = 2248;
const DEFAULT_MAP_ART_BACKGROUND_ATLAS: &str = "AdventureMap_TileBg";

fn c_map_get_map_art_background_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    if crate::map_art::get_map_art(ui_map_id as u32).is_none() {
        return Ok(0);
    }
    let atlas = create_string(state, DEFAULT_MAP_ART_BACKGROUND_ATLAS);
    state.push(atlas);
    Ok(1)
}

fn c_map_get_map_art_id(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    let art_id = borrow_state(state)?.maps.get(&ui_map_id).map(|m| m.art_id);
    let Some(art_id) = art_id else {
        return Ok(0);
    };
    state.push(Val::Num(art_id as f64));
    Ok(1)
}

fn c_map_get_map_art_layers(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    let Some(map_art) = crate::map_art::get_map_art(ui_map_id as u32) else {
        return Ok(0);
    };

    let layers = create_table(state);
    for (index, layer) in map_art.layers.iter().enumerate() {
        let layer_info = create_table(state);
        table_set(
            state,
            layer_info,
            "layerWidth",
            Val::Num(layer.layer_width as f64),
        );
        table_set(
            state,
            layer_info,
            "layerHeight",
            Val::Num(layer.layer_height as f64),
        );
        table_set(
            state,
            layer_info,
            "tileWidth",
            Val::Num(layer.tile_width as f64),
        );
        table_set(
            state,
            layer_info,
            "tileHeight",
            Val::Num(layer.tile_height as f64),
        );
        table_set(
            state,
            layer_info,
            "minScale",
            Val::Num(layer.min_scale as f64),
        );
        table_set(
            state,
            layer_info,
            "maxScale",
            Val::Num(layer.max_scale as f64),
        );
        table_set(
            state,
            layer_info,
            "additionalZoomSteps",
            Val::Num(layer.additional_zoom_steps as f64),
        );
        set_table_array(state, layers, index as i64 + 1, layer_info);
    }

    state.push(layers);
    Ok(1)
}

fn c_map_get_map_art_layer_textures(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    let layer_index = i32::from_stack(state, 2)?;
    if layer_index < 1 {
        return Ok(0);
    }

    let Some(map_art) = crate::map_art::get_map_art(ui_map_id as u32) else {
        return Ok(0);
    };
    let Some(textures) = map_art.tiles.get((layer_index - 1) as usize) else {
        return Ok(0);
    };

    let texture_ids = create_table(state);
    for (index, file_data_id) in textures.iter().copied().enumerate() {
        set_table_array(
            state,
            texture_ids,
            index as i64 + 1,
            Val::Num(file_data_id as f64),
        );
    }
    state.push(texture_ids);
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

fn c_map_get_current_map_id(state: &mut LuaState) -> LuaResult<u32> {
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

fn c_map_request_preload_map(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = i32::from_stack(state, 1)?;
    let queued_paths = collect_preload_paths_for_map(ui_map_id);
    borrow_state_mut(state)?.enqueue_texture_preloads(queued_paths);
    Ok(0)
}

fn collect_preload_paths_for_map(ui_map_id: i32) -> Vec<String> {
    let mut paths = Vec::new();
    if let Some(art_info) = crate::map_art::get_map_art(ui_map_id as u32) {
        for file_data_id in art_info
            .tiles
            .iter()
            .flat_map(|tiles| tiles.iter().copied())
        {
            if let Some(path) = file_data_id_to_wow_path(file_data_id) {
                paths.push(path);
            }
        }
    }
    if let Some(overlays) = crate::map_exploration::get_overlays_for_map(ui_map_id as u32) {
        for file_data_id in overlays
            .iter()
            .flat_map(|overlay| overlay.file_data_ids.iter().copied())
        {
            if let Some(path) = file_data_id_to_wow_path(file_data_id) {
                paths.push(path);
            }
        }
    }
    paths
}

fn file_data_id_to_wow_path(file_data_id: u32) -> Option<String> {
    let path = crate::manifest_interface_data::get_texture_path(file_data_id)?;
    Some(format!("Interface\\{}", path.replace('/', "\\")))
}
