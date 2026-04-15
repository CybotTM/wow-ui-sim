//! C_Map namespace and related map/location API functions.
//!
//! Contains map, exploration, navigation, and location-related API functions.

#[path = "c_map_api_namespaces.rs"]
mod c_map_api_namespaces;

use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) use c_map_api_namespaces::{fog_of_war_id_for_map, fog_of_war_info_for_id};
use c_map_api_namespaces::{
    register_c_date_and_time, register_c_death_info, register_c_fog_of_war,
    register_c_invasion_info, register_c_map_exploration, register_c_minimap,
    register_c_navigation, register_c_taxi_map, register_ui_map_point,
};

const DEFAULT_FOG_MASK_SCALAR: f64 = 1.0;
pub(crate) const DEFAULT_FOG_BACKGROUND_ASSET: &str = "Interface/Map/MapFogOfWar";
pub(crate) const DEFAULT_FOG_MASK_ASSET: &str = "Interface/Map/MapFogOfWarMaskSoftEdge";

#[derive(Clone, Copy)]
pub(crate) struct FogOfWarInfo {
    pub background_atlas: Option<&'static str>,
    pub mask_atlas: Option<&'static str>,
    pub mask_scalar: f64,
}

/// Register C_Map namespace and map-related functions.
pub fn register_c_map_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set("C_Map", register_c_map(lua, Rc::clone(&state))?)?;
    register_map_global_namespaces(lua, &globals, state)?;
    Ok(())
}

fn register_map_global_namespaces(
    lua: &Lua,
    globals: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_zone_text_functions(lua, state)?;
    globals.set("UiMapPoint", register_ui_map_point(lua)?)?;
    globals.set("C_MapExplorationInfo", register_c_map_exploration(lua)?)?;
    globals.set("C_FogOfWar", register_c_fog_of_war(lua)?)?;
    globals.set("C_DateAndTime", register_c_date_and_time(lua)?)?;
    globals.set("C_Minimap", register_c_minimap(lua)?)?;
    globals.set("C_Navigation", register_c_navigation(lua)?)?;
    globals.set("C_TaxiMap", register_c_taxi_map(lua)?)?;
    globals.set("C_DeathInfo", register_c_death_info(lua)?)?;
    globals.set("C_InvasionInfo", register_c_invasion_info(lua)?)?;
    Ok(())
}

/// C_Map namespace - map and area information.
fn register_c_map(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    register_map_queries(lua, &t)?;
    register_map_art_methods(lua, &t, Rc::clone(&state))?;
    register_map_quest_log(lua, &t, state)?;
    Ok(t)
}

/// Map info, position, and child queries.
fn register_map_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_map_info_queries(lua, t)?;
    register_player_map_queries(lua, t)?;
    register_map_world_queries(lua, t)?;
    Ok(())
}

fn register_map_info_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetAreaInfo", lua.create_function(get_area_info)?)?;
    t.set("GetMapInfo", lua.create_function(get_map_info)?)?;
    t.set("GetMapGroupID", lua.create_function(get_map_group_id)?)?;
    t.set(
        "GetMapGroupMembersInfo",
        lua.create_function(get_map_group_members_info)?,
    )?;
    t.set(
        "GetMapChildrenInfo",
        lua.create_function(
            |lua, (_map_id, _map_type, _all): (i32, Option<i32>, Option<bool>)| lua.create_table(),
        )?,
    )?;
    Ok(())
}

fn register_player_map_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetBestMapForUnit",
        lua.create_function(|_, _unit: String| Ok(2248i32))?,
    )?;
    t.set("GetCurrentMapID", lua.create_function(|_, ()| Ok(2248i32))?)?;
    t.set(
        "GetPlayerMapPosition",
        lua.create_function(create_player_map_position)?,
    )?;
    Ok(())
}

fn register_map_world_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetWorldPosFromMapPos",
        lua.create_function(create_world_pos_from_map_pos)?,
    )?;
    t.set(
        "GetMapWorldSize",
        lua.create_function(|_, _map_id: i32| Ok((1000.0f64, 1000.0f64)))?,
    )?;
    t.set(
        "GetUserWaypointPositionForMap",
        lua.create_function(|_, _map_id: i32| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// Map art layers, tiles, and art ID lookups.
fn register_map_art_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetMapArtLayers", lua.create_function(get_map_art_layers)?)?;
    t.set(
        "GetMapArtLayerTextures",
        lua.create_function(get_map_art_layer_textures)?,
    )?;
    t.set(
        "GetMapArtBackgroundAtlas",
        lua.create_function(get_map_art_background_atlas)?,
    )?;
    t.set("GetMapArtID", lua.create_function(get_map_art_id)?)?;
    t.set(
        "MapHasArt",
        lua.create_function(|_, map_id: i32| {
            Ok(crate::map_art::get_map_art(map_id as u32).is_some())
        })?,
    )?;
    let preload_state = Rc::clone(&state);
    t.set(
        "RequestPreloadMap",
        lua.create_function(move |_, map_id: i32| {
            if map_id > 0 {
                let paths = crate::texture::collect_map_preload_texture_paths(map_id as u32);
                preload_state.borrow_mut().enqueue_texture_preloads(paths);
            }
            Ok(())
        })?,
    )?;
    Ok(())
}

/// SetMapForQuestLog — updates WorldMapFrame and quest blob state.
fn register_map_quest_log(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set(
        "SetMapForQuestLog",
        lua.create_function(move |lua, map_id: i32| {
            let globals = lua.globals();
            globals.set("__wow_ui_sim_quest_log_map_id", map_id)?;
            let exec_result = lua
                .load(
                    r#"
                    local mapID = __wow_ui_sim_quest_log_map_id
                    if WorldMapFrame and type(WorldMapFrame.SetMapID) == "function" then
                        WorldMapFrame:SetMapID(mapID)
                    end
                "#,
                )
                .exec();
            let _ = globals.set("__wow_ui_sim_quest_log_map_id", Value::Nil);
            exec_result?;

            let mut sim_state = state.borrow_mut();
            if let Some(world_map_id) = sim_state.widgets.get_id_by_name("WorldMapFrame") {
                let blob = sim_state.quest_blobs.entry(world_map_id).or_default();
                blob.map_id = map_id as u32;
            }
            Ok(())
        })?,
    )?;
    Ok(())
}

fn get_area_info(lua: &Lua, area_id: i32) -> Result<Value> {
    match crate::zones::get_area(area_id as u32) {
        Some(area) => Ok(Value::String(lua.create_string(area.name)?)),
        None => Ok(Value::Nil),
    }
}

fn get_map_info(lua: &Lua, map_id: i32) -> Result<Value> {
    if map_id <= 0 {
        return Ok(Value::Nil);
    }
    let info = lua.create_table()?;
    info.set("mapID", map_id)?;
    info.set("name", format!("Map_{}", map_id))?;
    info.set("mapType", 3)?;
    info.set("parentMapID", 0)?;
    Ok(Value::Table(info))
}

fn get_map_group_id(_lua: &Lua, _map_id: i32) -> Result<Value> {
    // Blizzard checks this with `if not mapGroupID then`.
    // Returning numeric 0 from the generated stub is truthy in Lua and
    // incorrectly enables the world map floor dropdown.
    Ok(Value::Nil)
}

fn get_map_group_members_info(_lua: &Lua, _map_group_id: Value) -> Result<Value> {
    Ok(Value::Nil)
}

fn get_map_art_layers(lua: &Lua, map_id: i32) -> Result<mlua::Table> {
    let layers = lua.create_table()?;
    if let Some(info) = crate::map_art::get_map_art(map_id as u32) {
        append_map_art_layers(lua, &layers, &info.layers)?;
    } else {
        layers.set(1, fallback_map_art_layer(lua)?)?;
    }
    Ok(layers)
}

fn append_map_art_layers(
    lua: &Lua,
    layers: &mlua::Table,
    art_layers: &[crate::map_art::MapArtLayer],
) -> Result<()> {
    for (index, art_layer) in art_layers.iter().enumerate() {
        layers.set(index + 1, map_art_layer_table(lua, art_layer)?)?;
    }
    Ok(())
}

fn map_art_layer_table(lua: &Lua, art_layer: &crate::map_art::MapArtLayer) -> Result<mlua::Table> {
    let layer = lua.create_table()?;
    layer.set("layerWidth", art_layer.layer_width)?;
    layer.set("layerHeight", art_layer.layer_height)?;
    layer.set("tileWidth", art_layer.tile_width)?;
    layer.set("tileHeight", art_layer.tile_height)?;
    layer.set("minScale", art_layer.min_scale as f64)?;
    layer.set("maxScale", art_layer.max_scale as f64)?;
    layer.set("additionalZoomSteps", art_layer.additional_zoom_steps)?;
    Ok(layer)
}

fn fallback_map_art_layer(lua: &Lua) -> Result<mlua::Table> {
    let layer = lua.create_table()?;
    layer.set("layerWidth", 1002)?;
    layer.set("layerHeight", 668)?;
    layer.set("tileWidth", 256)?;
    layer.set("tileHeight", 256)?;
    layer.set("minScale", 1.0)?;
    layer.set("maxScale", 2.14)?;
    layer.set("additionalZoomSteps", 2)?;
    Ok(layer)
}

/// C_Map.GetMapArtLayerTextures(mapID, layerIndex) -> table of fileDataIDs
fn get_map_art_layer_textures(lua: &Lua, (map_id, layer_index): (i32, i32)) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    if let Some(info) = crate::map_art::get_map_art(map_id as u32) {
        let idx = (layer_index - 1) as usize; // Lua 1-indexed
        if let Some(tiles) = info.tiles.get(idx) {
            for (i, &file_data_id) in tiles.iter().enumerate() {
                t.set(i + 1, file_data_id)?; // 1-indexed Lua table
            }
        }
    }
    Ok(t)
}

/// C_Map.GetMapArtID(mapID) -> uiMapArtID
fn get_map_art_id(_lua: &Lua, map_id: i32) -> Result<i32> {
    match crate::map_art::get_map_art(map_id as u32) {
        Some(info) => Ok(info.art_id as i32),
        None => Ok(0),
    }
}

fn get_map_art_background_atlas(lua: &Lua, map_id: i32) -> Result<Value> {
    if crate::map_art::get_map_art(map_id as u32).is_some() {
        return Ok(Value::String(lua.create_string("AdventureMap_TileBg")?));
    }
    Ok(Value::String(lua.create_string("")?))
}

fn create_player_map_position(lua: &Lua, (_map_id, _unit): (i32, String)) -> Result<Value> {
    let pos = lua.create_table()?;
    pos.set("x", 0.5)?;
    pos.set("y", 0.5)?;
    Ok(Value::Table(pos))
}

fn create_world_pos_from_map_pos(
    lua: &Lua,
    (map_id, pos): (i32, Value),
) -> Result<(i32, mlua::Table)> {
    let (x, y) = if let Value::Table(ref t) = pos {
        let x: f64 = t.get("x").unwrap_or(0.5);
        let y: f64 = t.get("y").unwrap_or(0.5);
        (x, y)
    } else {
        (0.5, 0.5)
    };
    let world_x = x * 1000.0;
    let world_y = y * 1000.0;
    let world_pos = lua.create_table()?;
    world_pos.set("x", world_x)?;
    world_pos.set("y", world_y)?;
    world_pos.set(
        "GetXY",
        lua.create_function(move |_, _: Value| Ok((world_x, world_y)))?,
    )?;
    Ok((map_id, world_pos))
}

/// Zone text functions (GetRealZoneText, GetZoneText, etc.).
fn register_zone_text_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let st = state.clone();
    globals.set(
        "GetRealZoneText",
        lua.create_function(move |_, ()| Ok(st.borrow().world.zone_name.clone()))?,
    )?;
    let st = state.clone();
    globals.set(
        "GetZoneText",
        lua.create_function(move |_, ()| Ok(st.borrow().world.zone_name.clone()))?,
    )?;
    let st = state.clone();
    globals.set(
        "GetSubZoneText",
        lua.create_function(move |_, ()| Ok(st.borrow().world.sub_zone_name.clone()))?,
    )?;
    globals.set(
        "GetMinimapZoneText",
        lua.create_function(move |_, ()| {
            let s = state.borrow();
            if s.world.sub_zone_name.is_empty() {
                Ok(s.world.zone_name.clone())
            } else {
                Ok(s.world.sub_zone_name.clone())
            }
        })?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_map_queries_installs_expected_query_methods() {
        let lua = Lua::new();
        let table = lua.create_table().unwrap();

        register_map_queries(&lua, &table).unwrap();

        for key in [
            "GetAreaInfo",
            "GetMapInfo",
            "GetMapGroupID",
            "GetMapGroupMembersInfo",
            "GetBestMapForUnit",
            "GetCurrentMapID",
            "GetPlayerMapPosition",
            "GetMapChildrenInfo",
            "GetWorldPosFromMapPos",
            "GetMapWorldSize",
            "GetUserWaypointPositionForMap",
        ] {
            assert!(
                matches!(table.get::<Value>(key).unwrap(), Value::Function(_)),
                "{key} should be registered as a function"
            );
        }
    }
}
