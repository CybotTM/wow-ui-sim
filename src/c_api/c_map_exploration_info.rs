//! `C_MapExplorationInfo` surface backed by real DB2 exploration overlays.

use crate::c_api::ensure_namespace;
use crate::c_api::helpers::set_table_array;
use crate::lua_api::methods::{create_table, create_table_with_capacity, table_get, table_set};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::collections::HashSet;

const EXPLORED_OVERLAY_HASH_FIELDS: usize = 8;
const HIT_RECT_HASH_FIELDS: usize = 4;

pub(crate) fn register_c_map_exploration_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_MapExplorationInfo")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetExploredAreaIDsAtPosition",
        c_map_exploration_info_get_explored_area_ids_at_position,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetExploredMapTextures",
        c_map_exploration_info_get_explored_map_textures,
    )?;
    Ok(())
}

fn c_map_exploration_info_get_explored_map_textures(state: &mut LuaState) -> LuaResult<u32> {
    let explored = create_table(state);
    let Some(map_id) = map_id_from_stack(state, 1) else {
        state.push(explored);
        return Ok(1);
    };

    let overlays = crate::map_exploration::get_default_visible_overlays_for_map(map_id as u32);
    for (index, overlay) in overlays.into_iter().enumerate() {
        let entry = build_explored_overlay_entry(state, overlay);
        set_table_array(state, explored, index as i64 + 1, entry);
    }

    state.push(explored);
    Ok(1)
}

fn c_map_exploration_info_get_explored_area_ids_at_position(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let result = create_table(state);
    let Some(map_id) = map_id_from_stack(state, 1) else {
        state.push(result);
        return Ok(1);
    };
    let Some((x, y)) = map_point_from_stack(state, 2) else {
        state.push(result);
        return Ok(1);
    };
    let Some((layer_width, layer_height)) = map_layer_dimensions(map_id as u32) else {
        state.push(result);
        return Ok(1);
    };

    let pixel_x = x * layer_width as f64;
    let pixel_y = y * layer_height as f64;
    let overlays = crate::map_exploration::get_default_visible_overlays_for_map(map_id as u32);
    let mut seen: HashSet<u32> = HashSet::new();
    let mut write_index = 1_i64;

    for overlay in overlays {
        if !overlay.contains_pixel(pixel_x as f32, pixel_y as f32) {
            continue;
        }
        for area_id in overlay.area_ids() {
            if seen.insert(area_id) {
                set_table_array(state, result, write_index, Val::Num(area_id as f64));
                write_index += 1;
            }
        }
    }

    state.push(result);
    Ok(1)
}

fn map_id_from_stack(state: &mut LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(value) => Some(value as i32),
        _ => None,
    }
}

fn map_point_from_stack(state: &mut LuaState, index: i32) -> Option<(f64, f64)> {
    let point = stack_val(state, index);
    let x = match table_get(state, point, "x") {
        Val::Num(value) => value,
        _ => return None,
    };
    let y = match table_get(state, point, "y") {
        Val::Num(value) => value,
        _ => return None,
    };
    Some((x, y))
}

fn map_layer_dimensions(map_id: u32) -> Option<(u32, u32)> {
    let map_art = crate::map_art::get_map_art(map_id)?;
    let layer = map_art.layers.first()?;
    Some((layer.layer_width, layer.layer_height))
}

fn build_explored_overlay_entry(
    state: &mut LuaState,
    overlay: &crate::map_exploration::MapExplorationOverlay,
) -> Val {
    let entry = create_table_with_capacity(state, EXPLORED_OVERLAY_HASH_FIELDS);
    table_set(state, entry, "offsetX", Val::Num(overlay.offset_x as f64));
    table_set(state, entry, "offsetY", Val::Num(overlay.offset_y as f64));
    table_set(
        state,
        entry,
        "textureWidth",
        Val::Num(overlay.texture_width as f64),
    );
    table_set(
        state,
        entry,
        "textureHeight",
        Val::Num(overlay.texture_height as f64),
    );
    table_set(state, entry, "isShownByMouseOver", Val::Bool(false));
    table_set(state, entry, "isDrawOnTopLayer", Val::Bool(false));

    let file_data_ids = build_file_data_ids_table(state, overlay);
    table_set(state, entry, "fileDataIDs", file_data_ids);
    let hit_rect = build_hit_rect_table(state, overlay);
    table_set(state, entry, "hitRect", hit_rect);
    entry
}

fn build_file_data_ids_table(
    state: &mut LuaState,
    overlay: &crate::map_exploration::MapExplorationOverlay,
) -> Val {
    let file_data_ids = create_table(state);
    for (file_data_index, file_data_id) in overlay.file_data_ids.iter().copied().enumerate() {
        set_table_array(
            state,
            file_data_ids,
            file_data_index as i64 + 1,
            Val::Num(file_data_id as f64),
        );
    }
    file_data_ids
}

fn build_hit_rect_table(
    state: &mut LuaState,
    overlay: &crate::map_exploration::MapExplorationOverlay,
) -> Val {
    let (left, right, top, bottom) = overlay_hit_rect_bounds(overlay);
    let hit_rect = create_table_with_capacity(state, HIT_RECT_HASH_FIELDS);
    table_set(state, hit_rect, "top", Val::Num(top as f64));
    table_set(state, hit_rect, "bottom", Val::Num(bottom as f64));
    table_set(state, hit_rect, "left", Val::Num(left as f64));
    table_set(state, hit_rect, "right", Val::Num(right as f64));
    hit_rect
}

fn overlay_hit_rect_bounds(
    overlay: &crate::map_exploration::MapExplorationOverlay,
) -> (i32, i32, i32, i32) {
    let has_hit_rect = overlay.hit_rect_right > overlay.hit_rect_left
        && overlay.hit_rect_bottom > overlay.hit_rect_top;
    if has_hit_rect {
        (
            overlay.hit_rect_left,
            overlay.hit_rect_right,
            overlay.hit_rect_top,
            overlay.hit_rect_bottom,
        )
    } else {
        (
            overlay.offset_x,
            overlay.offset_x + overlay.texture_width as i32,
            overlay.offset_y,
            overlay.offset_y + overlay.texture_height as i32,
        )
    }
}
