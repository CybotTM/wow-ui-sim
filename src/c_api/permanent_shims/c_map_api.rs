//! Fog-of-war lookup data shared by map-facing C_* shims.
//!
//! The world-map fog pins need stable atlas lookup even though the richer map
//! helper namespaces are still shimmed elsewhere.

use std::collections::HashMap;
use std::sync::LazyLock;

const DEFAULT_FOG_BACKGROUND_ASSET: &str = "worldmap-wardisplay-background";
const DEFAULT_FOG_MASK_ASSET: &str = "worldmap-wardisplay-mask";
const DEFAULT_FOG_MASK_SCALAR: f64 = 1.0;

static FOG_OF_WAR_ID_BY_MAP_ID: LazyLock<HashMap<i32, i32>> =
    LazyLock::new(load_fog_of_war_id_by_map_id);
static FOG_OF_WAR_INFO_BY_ID: LazyLock<HashMap<i32, FogOfWarInfo>> =
    LazyLock::new(load_fog_of_war_info_by_id);

#[derive(Clone, Copy)]
pub(crate) struct FogOfWarInfo {
    pub(crate) background_atlas: Option<&'static str>,
    pub(crate) mask_atlas: Option<&'static str>,
    pub(crate) mask_scalar: f64,
}

#[derive(Clone, Copy)]
struct FogOfWarVisualizationRow {
    background_atlas_id: u32,
    mask_atlas_id: u32,
    mask_scalar: f64,
}

pub(crate) fn fog_of_war_id_for_map(map_id: i32) -> Option<i32> {
    FOG_OF_WAR_ID_BY_MAP_ID.get(&map_id).copied()
}

pub(crate) fn fog_of_war_info_for_id(fog_of_war_id: i32) -> Option<FogOfWarInfo> {
    FOG_OF_WAR_INFO_BY_ID.get(&fog_of_war_id).copied()
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
