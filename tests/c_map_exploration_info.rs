//! Tests for `C_MapExplorationInfo` behavior across current and non-current maps.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::{map_art, map_exploration};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn first_non_current_map_with_explored_overlays(current_map_id: u32) -> Option<u32> {
    (1..=10_000).find(|&map_id| {
        map_id != current_map_id
            && !map_exploration::get_default_visible_overlays_for_map(map_id).is_empty()
    })
}

fn first_non_current_map_sample_with_area_ids(current_map_id: u32) -> Option<(u32, f64, f64)> {
    for map_id in 1..=10_000 {
        if map_id == current_map_id {
            continue;
        }
        let Some(layer) = map_art::get_map_art(map_id).and_then(|info| info.layers.first()) else {
            continue;
        };
        for overlay in map_exploration::get_default_visible_overlays_for_map(map_id) {
            if overlay.area_ids().next().is_none() {
                continue;
            }
            let x = (overlay.offset_x as f64 + (overlay.texture_width as f64 / 2.0))
                / layer.layer_width as f64;
            let y = (overlay.offset_y as f64 + (overlay.texture_height as f64 / 2.0))
                / layer.layer_height as f64;
            return Some((map_id, x, y));
        }
    }
    None
}

#[test]
fn test_get_explored_map_textures_support_non_current_maps() {
    let env = env();
    let current_map_id: i32 = env.eval("return C_Map.GetCurrentMapID()").unwrap();
    let target_map_id = first_non_current_map_with_explored_overlays(current_map_id as u32)
        .expect("expected at least one non-current map with explored overlays");

    let overlay_count: i32 = env
        .eval(&format!(
            r#"
        local explored = C_MapExplorationInfo.GetExploredMapTextures({target_map_id})
        local count = 0
        for _ in ipairs(explored or {{}}) do
            count = count + 1
        end
        return count
    "#
        ))
        .unwrap();

    assert!(
        overlay_count > 0,
        "non-current maps with overlay DB rows should also return explored overlay textures"
    );
}

#[test]
fn test_get_explored_area_ids_support_non_current_maps() {
    let env = env();
    let current_map_id: i32 = env.eval("return C_Map.GetCurrentMapID()").unwrap();
    let (target_map_id, sample_x, sample_y) =
        first_non_current_map_sample_with_area_ids(current_map_id as u32)
            .expect("expected at least one non-current map overlay sample with area IDs");

    let area_count: i32 = env
        .eval(&format!(
            r#"
        local areaIDs = C_MapExplorationInfo.GetExploredAreaIDsAtPosition(
            {target_map_id},
            {{ x = {sample_x}, y = {sample_y} }}
        )
        local count = 0
        for _ in ipairs(areaIDs or {{}}) do
            count = count + 1
        end
        return count
    "#
        ))
        .unwrap();

    assert!(
        area_count > 0,
        "non-current maps should report explored area IDs from real overlay hit regions"
    );
}
