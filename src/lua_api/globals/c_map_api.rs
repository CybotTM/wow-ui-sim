//! C_Map namespace and related map/location API functions.
//!
//! Contains map, exploration, navigation, and location-related API functions.

use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::LazyLock;

const DEFAULT_FOG_MASK_SCALAR: f64 = 1.0;
pub(crate) const DEFAULT_FOG_BACKGROUND_ASSET: &str = "Interface/Map/MapFogOfWar";
pub(crate) const DEFAULT_FOG_MASK_ASSET: &str = "Interface/Map/MapFogOfWarMaskSoftEdge";

static FOG_OF_WAR_ID_BY_MAP_ID: LazyLock<HashMap<i32, i32>> =
    LazyLock::new(load_fog_of_war_id_by_map_id);
static FOG_OF_WAR_INFO_BY_ID: LazyLock<HashMap<i32, FogOfWarInfo>> =
    LazyLock::new(load_fog_of_war_info_by_id);

#[derive(Clone, Copy)]
pub(crate) struct FogOfWarInfo {
    pub background_atlas: Option<&'static str>,
    pub mask_atlas: Option<&'static str>,
    pub mask_scalar: f64,
}

#[derive(Clone, Copy)]
struct FogOfWarVisualizationRow {
    background_atlas_id: u32,
    mask_atlas_id: u32,
    mask_scalar: f64,
}

/// Register C_Map namespace and map-related functions.
pub fn register_c_map_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();

    globals.set("C_Map", register_c_map(lua, Rc::clone(&state))?)?;
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
        lua.create_function(get_explored_area_ids_at_position)?,
    )?;
    t.set(
        "GetExploredMapTextures",
        lua.create_function(get_explored_map_textures)?,
    )?;

    Ok(t)
}

fn register_c_fog_of_war(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;

    t.set(
        "GetFogOfWarForMap",
        lua.create_function(|_, map_id: i32| match fog_of_war_id_for_map(map_id) {
            Some(fog_id) => Ok(Value::Integer(fog_id as i64)),
            None => Ok(Value::Nil),
        })?,
    )?;
    t.set(
        "GetFogOfWarInfo",
        lua.create_function(get_fog_of_war_info_table)?,
    )?;

    Ok(t)
}

pub(crate) fn fog_of_war_id_for_map(map_id: i32) -> Option<i32> {
    FOG_OF_WAR_ID_BY_MAP_ID.get(&map_id).copied()
}

pub(crate) fn fog_of_war_info_for_id(fog_of_war_id: i32) -> Option<FogOfWarInfo> {
    FOG_OF_WAR_INFO_BY_ID.get(&fog_of_war_id).copied()
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

fn get_fog_of_war_info_table(lua: &Lua, fog_of_war_id: Value) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    let fog_info = fog_of_war_id
        .as_integer()
        .and_then(|fog_id| fog_of_war_info_for_id(fog_id as i32));
    let Some(fog_info) = fog_info else {
        set_optional_string_field(&info, "backgroundAtlas", None)?;
        set_optional_string_field(&info, "maskAtlas", None)?;
        info.set("maskScalar", DEFAULT_FOG_MASK_SCALAR)?;
        return Ok(info);
    };

    set_optional_string_field(&info, "backgroundAtlas", fog_info.background_atlas)?;
    set_optional_string_field(&info, "maskAtlas", fog_info.mask_atlas)?;
    info.set("maskScalar", fog_info.mask_scalar)?;
    Ok(info)
}

fn set_optional_string_field(
    table: &mlua::Table,
    field_name: &str,
    value: Option<&str>,
) -> Result<()> {
    match value {
        Some(value) => table.set(field_name, value),
        None => table.set(field_name, Value::Nil),
    }
}

fn load_fog_of_war_id_by_map_id() -> HashMap<i32, i32> {
    fog_of_war_rows()
        .into_iter()
        .map(|(fog_id, map_id, _vis_id)| (map_id, fog_id))
        .collect()
}

fn load_fog_of_war_info_by_id() -> HashMap<i32, FogOfWarInfo> {
    let visualizations = fog_of_war_visualizations();
    fog_of_war_rows()
        .into_iter()
        .map(|(fog_id, _map_id, vis_id)| {
            let info = visualizations.get(&vis_id).copied().map_or(
                FogOfWarInfo {
                    background_atlas: Some(DEFAULT_FOG_BACKGROUND_ASSET),
                    mask_atlas: Some(DEFAULT_FOG_MASK_ASSET),
                    mask_scalar: DEFAULT_FOG_MASK_SCALAR,
                },
                fog_of_war_info_from_visualization,
            );
            (fog_id, info)
        })
        .collect()
}

fn fog_of_war_rows() -> Vec<(i32, i32, i32)> {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/db2/UiMapFogOfWar.csv"
    ))
    .lines()
    .skip(1)
    .filter_map(parse_fog_of_war_row)
    .collect()
}

fn fog_of_war_visualizations() -> HashMap<i32, FogOfWarVisualizationRow> {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/db2/UiMapFogOfWarVisualization.csv"
    ))
    .lines()
    .skip(1)
    .filter_map(parse_fog_of_war_visualization_row)
    .collect()
}

fn parse_fog_of_war_row(line: &str) -> Option<(i32, i32, i32)> {
    let mut fields = line.split(',');
    let fog_id = parse_i32_field(fields.next()?);
    let map_id = parse_i32_field(fields.next()?);
    let _player_condition_id = parse_i32_field(fields.next()?);
    let vis_id = parse_i32_field(fields.next()?);
    (fog_id > 0 && map_id > 0 && vis_id > 0).then_some((fog_id, map_id, vis_id))
}

fn parse_fog_of_war_visualization_row(line: &str) -> Option<(i32, FogOfWarVisualizationRow)> {
    let mut fields = line.split(',');
    let id = parse_i32_field(fields.next()?);
    let background_atlas_id = parse_u32_field(fields.next()?);
    let mask_atlas_id = parse_u32_field(fields.next()?);
    let mask_scalar = fields.next()?.parse().ok()?;
    (id > 0).then_some((
        id,
        FogOfWarVisualizationRow {
            background_atlas_id,
            mask_atlas_id,
            mask_scalar,
        },
    ))
}

fn fog_of_war_info_from_visualization(visualization: FogOfWarVisualizationRow) -> FogOfWarInfo {
    FogOfWarInfo {
        background_atlas: resolve_fog_atlas_name(
            visualization.background_atlas_id,
            DEFAULT_FOG_BACKGROUND_ASSET,
        ),
        mask_atlas: resolve_fog_atlas_name(visualization.mask_atlas_id, DEFAULT_FOG_MASK_ASSET),
        mask_scalar: visualization.mask_scalar,
    }
}

fn resolve_fog_atlas_name(_atlas_id: u32, fallback: &'static str) -> Option<&'static str> {
    Some(fallback)
}

fn parse_i32_field(field: &str) -> i32 {
    field.parse().unwrap_or(0)
}

fn parse_u32_field(field: &str) -> u32 {
    field.parse().unwrap_or(0)
}

fn get_explored_area_ids_at_position(
    lua: &Lua,
    (map_id, pos): (i32, Value),
) -> Result<mlua::Table> {
    let areas = lua.create_table()?;
    let Some((pixel_x, pixel_y)) = map_pixel_position(map_id, &pos) else {
        return Ok(areas);
    };

    let mut area_ids = BTreeSet::new();
    for overlay in crate::map_exploration::get_default_visible_overlays_for_map(map_id as u32) {
        if !overlay.contains_pixel(pixel_x, pixel_y) {
            continue;
        }
        area_ids.extend(overlay.area_ids());
    }

    for (index, area_id) in area_ids.into_iter().enumerate() {
        areas.set(index + 1, area_id)?;
    }
    Ok(areas)
}

fn get_explored_map_textures(lua: &Lua, map_id: i32) -> Result<mlua::Table> {
    let overlays = lua.create_table()?;
    for (index, overlay_info) in
        crate::map_exploration::get_default_visible_overlays_for_map(map_id as u32)
            .into_iter()
            .enumerate()
    {
        let overlay = create_explored_overlay_table(lua, overlay_info)?;
        overlays.set(index + 1, overlay)?;
    }

    Ok(overlays)
}

fn create_explored_overlay_table(
    lua: &Lua,
    overlay_info: &crate::map_exploration::MapExplorationOverlay,
) -> Result<mlua::Table> {
    let file_data_ids = create_file_data_id_table(lua, &overlay_info.file_data_ids)?;
    let hit_rect = lua.create_table()?;
    hit_rect.set("top", overlay_info.hit_rect_top)?;
    hit_rect.set("bottom", overlay_info.hit_rect_bottom)?;
    hit_rect.set("left", overlay_info.hit_rect_left)?;
    hit_rect.set("right", overlay_info.hit_rect_right)?;

    let overlay = lua.create_table()?;
    overlay.set("textureWidth", overlay_info.texture_width)?;
    overlay.set("textureHeight", overlay_info.texture_height)?;
    overlay.set("offsetX", overlay_info.offset_x)?;
    overlay.set("offsetY", overlay_info.offset_y)?;
    overlay.set("isShownByMouseOver", false)?;
    overlay.set("isDrawOnTopLayer", false)?;
    overlay.set("fileDataIDs", file_data_ids)?;
    overlay.set("hitRect", hit_rect)?;
    Ok(overlay)
}

fn create_file_data_id_table(lua: &Lua, file_data_ids: &[u32]) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    for (index, file_data_id) in file_data_ids.iter().copied().enumerate() {
        table.set(index + 1, file_data_id)?;
    }
    Ok(table)
}

fn map_pixel_position(map_id: i32, pos: &Value) -> Option<(f32, f32)> {
    let Value::Table(table) = pos else {
        return None;
    };
    let layer = crate::map_art::get_map_art(map_id as u32)?.layers.first()?;
    let x = table.get::<f64>("x").ok()? as f32;
    let y = table.get::<f64>("y").ok()? as f32;
    Some((x * layer.layer_width as f32, y * layer.layer_height as f32))
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
