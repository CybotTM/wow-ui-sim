//! C_Map namespace and related map/location API functions.
//!
//! Contains map, exploration, navigation, and location-related API functions.

use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register C_Map namespace and map-related functions.
pub fn register_c_map_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();

    globals.set("C_Map", register_c_map(lua, Rc::clone(&state))?)?;
    register_zone_text_functions(lua, state)?;
    globals.set("UiMapPoint", register_ui_map_point(lua)?)?;
    globals.set("C_MapExplorationInfo", register_c_map_exploration(lua)?)?;
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
    register_map_art_methods(lua, &t)?;
    register_map_quest_log(lua, &t, state)?;
    Ok(t)
}

/// Map info, position, and child queries.
fn register_map_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetAreaInfo", lua.create_function(get_area_info)?)?;
    t.set("GetMapInfo", lua.create_function(get_map_info)?)?;
    t.set(
        "GetBestMapForUnit",
        lua.create_function(|_, _unit: String| Ok(2274i32))?,
    )?;
    t.set("GetCurrentMapID", lua.create_function(|_, ()| Ok(2274i32))?)?;
    t.set(
        "GetPlayerMapPosition",
        lua.create_function(create_player_map_position)?,
    )?;
    t.set(
        "GetMapChildrenInfo",
        lua.create_function(
            |lua, (_map_id, _map_type, _all): (i32, Option<i32>, Option<bool>)| lua.create_table(),
        )?,
    )?;
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
fn register_map_art_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetMapArtLayers", lua.create_function(get_map_art_layers)?)?;
    t.set(
        "GetMapArtLayerTextures",
        lua.create_function(get_map_art_layer_textures)?,
    )?;
    t.set("GetMapArtID", lua.create_function(get_map_art_id)?)?;
    t.set(
        "MapHasArt",
        lua.create_function(|_, map_id: i32| {
            Ok(crate::map_art::get_map_art(map_id as u32).is_some())
        })?,
    )?;
    t.set(
        "RequestPreloadMap",
        lua.create_function(|_, _map_id: i32| Ok(()))?,
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

fn get_map_art_layers(lua: &Lua, map_id: i32) -> Result<mlua::Table> {
    let layers = lua.create_table()?;
    if let Some(info) = crate::map_art::get_map_art(map_id as u32) {
        for (i, art_layer) in info.layers.iter().enumerate() {
            let layer = lua.create_table()?;
            layer.set("layerWidth", art_layer.layer_width)?;
            layer.set("layerHeight", art_layer.layer_height)?;
            layer.set("tileWidth", art_layer.tile_width)?;
            layer.set("tileHeight", art_layer.tile_height)?;
            layer.set("minScale", art_layer.min_scale as f64)?;
            layer.set("maxScale", art_layer.max_scale as f64)?;
            layer.set("additionalZoomSteps", art_layer.additional_zoom_steps)?;
            layers.set(i + 1, layer)?;
        }
    } else {
        // Fallback for unknown maps
        let layer = lua.create_table()?;
        layer.set("layerWidth", 1002)?;
        layer.set("layerHeight", 668)?;
        layer.set("tileWidth", 256)?;
        layer.set("tileHeight", 256)?;
        layer.set("minScale", 1.0)?;
        layer.set("maxScale", 2.14)?;
        layer.set("additionalZoomSteps", 2)?;
        layers.set(1, layer)?;
    }
    Ok(layers)
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

/// UiMapPoint - map point creation helper.
fn register_ui_map_point(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;

    t.set(
        "CreateFromVector2D",
        lua.create_function(|lua, (map_id, pos): (i32, Value)| {
            let (x, y) = if let Value::Table(ref t) = pos {
                let x: f64 = t.get("x").unwrap_or(0.5);
                let y: f64 = t.get("y").unwrap_or(0.5);
                (x, y)
            } else {
                (0.5, 0.5)
            };
            let point = lua.create_table()?;
            point.set("uiMapID", map_id)?;
            point.set("x", x)?;
            point.set("y", y)?;
            Ok(point)
        })?,
    )?;
    t.set(
        "CreateFromCoordinates",
        lua.create_function(|lua, (map_id, x, y): (i32, f64, f64)| {
            let point = lua.create_table()?;
            point.set("uiMapID", map_id)?;
            point.set("x", x)?;
            point.set("y", y)?;
            Ok(point)
        })?,
    )?;

    Ok(t)
}

/// C_MapExplorationInfo namespace - map exploration data.
fn register_c_map_exploration(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;

    t.set(
        "GetExploredAreaIDsAtPosition",
        lua.create_function(|lua, (_map_id, _pos): (i32, Value)| lua.create_table())?,
    )?;
    t.set(
        "GetExploredMapTextures",
        lua.create_function(|lua, _map_id: i32| lua.create_table())?,
    )?;

    Ok(t)
}

/// C_DateAndTime namespace - date/time utilities.
fn register_c_date_and_time(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;

    t.set(
        "GetCurrentCalendarTime",
        lua.create_function(|lua, ()| {
            let info = lua.create_table()?;
            info.set("year", 2024)?;
            info.set("month", 1)?;
            info.set("monthDay", 1)?;
            info.set("weekday", 1)?;
            info.set("hour", 12)?;
            info.set("minute", 0)?;
            Ok(info)
        })?,
    )?;
    t.set("GetServerTimeLocal", lua.create_function(|_, ()| Ok(0i64))?)?;
    t.set(
        "GetSecondsUntilDailyReset",
        lua.create_function(|_, ()| Ok(86400i32))?,
    )?;
    t.set(
        "GetSecondsUntilWeeklyReset",
        lua.create_function(|_, ()| Ok(604800i32))?,
    )?;

    Ok(t)
}

/// C_Minimap namespace - minimap utilities.
fn register_c_minimap(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    register_minimap_core(lua, &t)?;
    register_minimap_tracking(lua, &t)?;
    Ok(t)
}

/// Core minimap queries and settings.
fn register_minimap_core(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "IsInsideQuestBlob",
        lua.create_function(|_, (_qid, _x, _y): (i32, f64, f64)| Ok(false))?,
    )?;
    t.set("GetViewRadius", lua.create_function(|_, ()| Ok(200.0f64))?)?;
    t.set(
        "SetPlayerTexture",
        lua.create_function(|_, (_fid, _iid): (i32, i32)| Ok(()))?,
    )?;
    t.set(
        "ShouldUseHybridMinimap",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("GetUiMapID", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set(
        "IsFilteredOut",
        lua.create_function(|_, _filter: Value| Ok(false))?,
    )?;
    Ok(())
}

/// Minimap tracking system stubs — no tracking types in the simulator.
fn register_minimap_tracking(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetNumTrackingTypes",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetTrackingInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetTrackingFilter",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    t.set("ClearAllTracking", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "SetTrackingFilterByFilterIndex",
        lua.create_function(|_, (_i, _v): (i32, bool)| Ok(()))?,
    )?;
    Ok(())
}

/// C_Navigation namespace - quest navigation waypoints.
fn register_c_navigation(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;

    t.set("GetFrame", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("GetDistance", lua.create_function(|_, ()| Ok(0.0f64))?)?;
    t.set(
        "GetDestination",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "IsAutoFollowEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "SetAutoFollowEnabled",
        lua.create_function(|_, _enabled: bool| Ok(()))?,
    )?;

    Ok(t)
}

/// C_TaxiMap namespace - flight path utilities.
fn register_c_taxi_map(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;

    t.set(
        "GetAllTaxiNodes",
        lua.create_function(|lua, _map_id: i32| lua.create_table())?,
    )?;
    t.set(
        "GetTaxiNodesForMap",
        lua.create_function(|lua, _map_id: i32| lua.create_table())?,
    )?;
    t.set(
        "ShouldMapShowTaxiNodes",
        lua.create_function(|_, _map_id: i32| Ok(true))?,
    )?;

    Ok(t)
}

/// C_InvasionInfo — legion invasion data. No active invasions in the simulator.
fn register_c_invasion_info(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set(
        "GetInvasionForUiMapID",
        lua.create_function(|_, _map_id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetInvasionInfo",
        lua.create_function(|_, _invasion_id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "AreInvasionsAvailable",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(t)
}

/// C_DeathInfo — corpse/graveyard position data.
/// Player is alive in the simulator, so all position queries return nil.
fn register_c_death_info(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set(
        "GetCorpseMapPosition",
        lua.create_function(|_, _map_id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetDeathReleasePosition",
        lua.create_function(|_, _map_id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetGraveyardsForMap",
        lua.create_function(|lua, _map_id: i32| lua.create_table())?,
    )?;
    Ok(t)
}
