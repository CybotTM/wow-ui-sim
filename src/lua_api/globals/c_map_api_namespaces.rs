use super::{
    DEFAULT_FOG_BACKGROUND_ASSET, DEFAULT_FOG_MASK_ASSET, DEFAULT_FOG_MASK_SCALAR, FogOfWarInfo,
};
use mlua::{Lua, Result, Value};
use std::collections::{BTreeSet, HashMap};
use std::sync::LazyLock;

static FOG_OF_WAR_ID_BY_MAP_ID: LazyLock<HashMap<i32, i32>> =
    LazyLock::new(load_fog_of_war_id_by_map_id);
static FOG_OF_WAR_INFO_BY_ID: LazyLock<HashMap<i32, FogOfWarInfo>> =
    LazyLock::new(load_fog_of_war_info_by_id);

#[derive(Clone, Copy)]
struct FogOfWarVisualizationRow {
    background_atlas_id: u32,
    mask_atlas_id: u32,
    mask_scalar: f64,
}

pub(super) fn register_ui_map_point(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "CreateFromVector2D",
        lua.create_function(|lua, (map_id, pos): (i32, Value)| {
            let (x, y) = if let Value::Table(ref table) = pos {
                let x: f64 = table.get("x").unwrap_or(0.5);
                let y: f64 = table.get("y").unwrap_or(0.5);
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
    table.set(
        "CreateFromCoordinates",
        lua.create_function(|lua, (map_id, x, y): (i32, f64, f64)| {
            let point = lua.create_table()?;
            point.set("uiMapID", map_id)?;
            point.set("x", x)?;
            point.set("y", y)?;
            Ok(point)
        })?,
    )?;

    Ok(table)
}

pub(super) fn register_c_map_exploration(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "GetExploredAreaIDsAtPosition",
        lua.create_function(get_explored_area_ids_at_position)?,
    )?;
    table.set(
        "GetExploredMapTextures",
        lua.create_function(get_explored_map_textures)?,
    )?;

    Ok(table)
}

pub(super) fn register_c_fog_of_war(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "GetFogOfWarForMap",
        lua.create_function(|_, map_id: i32| match fog_of_war_id_for_map(map_id) {
            Some(fog_id) => Ok(Value::Integer(fog_id as i64)),
            None => Ok(Value::Nil),
        })?,
    )?;
    table.set(
        "GetFogOfWarInfo",
        lua.create_function(get_fog_of_war_info_table)?,
    )?;

    Ok(table)
}

pub(crate) fn fog_of_war_id_for_map(map_id: i32) -> Option<i32> {
    FOG_OF_WAR_ID_BY_MAP_ID.get(&map_id).copied()
}

pub(crate) fn fog_of_war_info_for_id(fog_of_war_id: i32) -> Option<FogOfWarInfo> {
    FOG_OF_WAR_INFO_BY_ID.get(&fog_of_war_id).copied()
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

pub(super) fn register_c_date_and_time(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    register_current_calendar_time(lua, &table)?;
    table.set("GetServerTimeLocal", lua.create_function(|_, ()| Ok(0i64))?)?;
    register_reset_time_queries(lua, &table)?;
    Ok(table)
}

fn register_current_calendar_time(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetCurrentCalendarTime",
        lua.create_function(create_current_calendar_time)?,
    )?;
    Ok(())
}

fn create_current_calendar_time(lua: &Lua, (): ()) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("year", 2024)?;
    info.set("month", 1)?;
    info.set("monthDay", 1)?;
    info.set("weekday", 1)?;
    info.set("hour", 12)?;
    info.set("minute", 0)?;
    Ok(info)
}

fn register_reset_time_queries(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetSecondsUntilDailyReset",
        lua.create_function(|_, ()| Ok(86400i32))?,
    )?;
    table.set(
        "GetSecondsUntilWeeklyReset",
        lua.create_function(|_, ()| Ok(604800i32))?,
    )?;
    Ok(())
}

pub(super) fn register_c_minimap(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    register_minimap_core(lua, &table)?;
    register_minimap_tracking(lua, &table)?;
    Ok(table)
}

fn register_minimap_core(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "IsInsideQuestBlob",
        lua.create_function(|_, (_qid, _x, _y): (i32, f64, f64)| Ok(false))?,
    )?;
    table.set("GetViewRadius", lua.create_function(|_, ()| Ok(200.0f64))?)?;
    table.set(
        "SetPlayerTexture",
        lua.create_function(|_, (_fid, _iid): (i32, i32)| Ok(()))?,
    )?;
    table.set(
        "ShouldUseHybridMinimap",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    table.set("GetUiMapID", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    table.set(
        "IsFilteredOut",
        lua.create_function(|_, _filter: Value| Ok(false))?,
    )?;
    Ok(())
}

fn register_minimap_tracking(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetNumTrackingTypes",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    table.set(
        "GetTrackingInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetTrackingFilter",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    table.set("ClearAllTracking", lua.create_function(|_, ()| Ok(()))?)?;
    table.set(
        "SetTrackingFilterByFilterIndex",
        lua.create_function(|_, (_i, _v): (i32, bool)| Ok(()))?,
    )?;
    Ok(())
}

pub(super) fn register_c_navigation(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("GetFrame", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    table.set("GetDistance", lua.create_function(|_, ()| Ok(0.0f64))?)?;
    table.set(
        "GetDestination",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    table.set(
        "IsAutoFollowEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    table.set(
        "SetAutoFollowEnabled",
        lua.create_function(|_, _enabled: bool| Ok(()))?,
    )?;

    Ok(table)
}

pub(super) fn register_c_taxi_map(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "GetAllTaxiNodes",
        lua.create_function(|lua, _map_id: i32| lua.create_table())?,
    )?;
    table.set(
        "GetTaxiNodesForMap",
        lua.create_function(|lua, _map_id: i32| lua.create_table())?,
    )?;
    table.set(
        "ShouldMapShowTaxiNodes",
        lua.create_function(|_, _map_id: i32| Ok(true))?,
    )?;

    Ok(table)
}

pub(super) fn register_c_invasion_info(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    table.set(
        "GetInvasionForUiMapID",
        lua.create_function(|_, _map_id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetInvasionInfo",
        lua.create_function(|_, _invasion_id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "AreInvasionsAvailable",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(table)
}

pub(super) fn register_c_death_info(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    table.set(
        "GetCorpseMapPosition",
        lua.create_function(|_, _map_id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetDeathReleasePosition",
        lua.create_function(|_, _map_id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetGraveyardsForMap",
        lua.create_function(|lua, _map_id: i32| lua.create_table())?,
    )?;
    Ok(table)
}
