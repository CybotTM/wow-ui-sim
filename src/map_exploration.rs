use std::collections::{BTreeMap, HashMap};
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct MapExplorationOverlay {
    pub id: u32,
    pub texture_width: u32,
    pub texture_height: u32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub hit_rect_top: i32,
    pub hit_rect_bottom: i32,
    pub hit_rect_left: i32,
    pub hit_rect_right: i32,
    pub player_condition_id: u32,
    pub area_ids: [u32; 4],
    pub file_data_ids: Vec<u32>,
}

impl MapExplorationOverlay {
    pub fn contains_pixel(&self, pixel_x: f32, pixel_y: f32) -> bool {
        let (left, right, top, bottom) = self.hit_rect_bounds();
        pixel_x >= left as f32
            && pixel_x <= right as f32
            && pixel_y >= top as f32
            && pixel_y <= bottom as f32
    }

    pub fn area_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.area_ids
            .iter()
            .copied()
            .filter(|area_id| *area_id != 0)
    }

    pub fn is_default_visible(&self) -> bool {
        self.player_condition_id == 0
    }

    pub fn is_seeded_unexplored(&self, map_id: u32) -> bool {
        seeded_unexplored_overlay_ids(map_id).contains(&self.id)
    }

    fn hit_rect_bounds(&self) -> (i32, i32, i32, i32) {
        let has_hit_rect =
            self.hit_rect_right > self.hit_rect_left && self.hit_rect_bottom > self.hit_rect_top;
        if has_hit_rect {
            (
                self.hit_rect_left,
                self.hit_rect_right,
                self.hit_rect_top,
                self.hit_rect_bottom,
            )
        } else {
            (
                self.offset_x,
                self.offset_x + self.texture_width as i32,
                self.offset_y,
                self.offset_y + self.texture_height as i32,
            )
        }
    }
}

#[derive(Debug)]
struct PendingOverlay {
    id: u32,
    art_id: u32,
    texture_width: u32,
    texture_height: u32,
    offset_x: i32,
    offset_y: i32,
    hit_rect_top: i32,
    hit_rect_bottom: i32,
    hit_rect_left: i32,
    hit_rect_right: i32,
    player_condition_id: u32,
    area_ids: [u32; 4],
}

static OVERLAYS_BY_ART_ID: LazyLock<HashMap<u32, Vec<MapExplorationOverlay>>> =
    LazyLock::new(load_overlays_by_art_id);

pub fn get_overlays_for_map(map_id: u32) -> Option<&'static [MapExplorationOverlay]> {
    let art_id = crate::map_art::get_map_art(map_id)?.art_id;
    OVERLAYS_BY_ART_ID.get(&art_id).map(Vec::as_slice)
}

pub fn get_default_visible_overlays_for_map(map_id: u32) -> Vec<&'static MapExplorationOverlay> {
    let Some(overlays) = get_overlays_for_map(map_id) else {
        return Vec::new();
    };

    overlays
        .iter()
        .filter(|overlay| overlay.is_default_visible() && !overlay.is_seeded_unexplored(map_id))
        .collect()
}

fn load_overlays_by_art_id() -> HashMap<u32, Vec<MapExplorationOverlay>> {
    let pending_overlays = load_overlay_rows();
    let overlay_tiles = load_overlay_tiles();
    let mut overlays_by_art_id: HashMap<u32, Vec<MapExplorationOverlay>> = HashMap::new();

    for pending in pending_overlays {
        let Some(file_data_ids) = overlay_tiles.get(&pending.id) else {
            continue;
        };

        let overlay = MapExplorationOverlay {
            id: pending.id,
            texture_width: pending.texture_width,
            texture_height: pending.texture_height,
            offset_x: pending.offset_x,
            offset_y: pending.offset_y,
            hit_rect_top: pending.hit_rect_top,
            hit_rect_bottom: pending.hit_rect_bottom,
            hit_rect_left: pending.hit_rect_left,
            hit_rect_right: pending.hit_rect_right,
            player_condition_id: pending.player_condition_id,
            area_ids: pending.area_ids,
            file_data_ids: file_data_ids.clone(),
        };
        overlays_by_art_id
            .entry(pending.art_id)
            .or_default()
            .push(overlay);
    }

    overlays_by_art_id
}

fn seeded_unexplored_overlay_ids(map_id: u32) -> &'static [u32] {
    match map_id {
        // Keep one real Isle of Dorn sub-zone unexplored until we model
        // character-specific exploration state. Overlay 4885 is an
        // isolated explored chunk for The Three Shields / Skolzgal Mill,
        // so omitting it keeps one genuine pocket unexplored without
        // manufacturing fog geometry.
        2248 => &[4885],
        _ => &[],
    }
}

fn load_overlay_rows() -> Vec<PendingOverlay> {
    include_str!("../data/db2/WorldMapOverlay.csv")
        .lines()
        .skip(1)
        .filter_map(parse_overlay_row)
        .collect()
}

fn parse_overlay_row(line: &str) -> Option<PendingOverlay> {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() < 16 {
        return None;
    }

    let id = parse_u32(fields[0]);
    let art_id = parse_u32(fields[1]);
    if id == 0 || art_id == 0 {
        return None;
    }

    Some(PendingOverlay {
        id,
        art_id,
        texture_width: parse_u32(fields[2]),
        texture_height: parse_u32(fields[3]),
        offset_x: parse_i32(fields[4]),
        offset_y: parse_i32(fields[5]),
        hit_rect_top: parse_i32(fields[6]),
        hit_rect_bottom: parse_i32(fields[7]),
        hit_rect_left: parse_i32(fields[8]),
        hit_rect_right: parse_i32(fields[9]),
        player_condition_id: parse_u32(fields[10]),
        area_ids: [
            parse_u32(fields[12]),
            parse_u32(fields[13]),
            parse_u32(fields[14]),
            parse_u32(fields[15]),
        ],
    })
}

fn load_overlay_tiles() -> HashMap<u32, Vec<u32>> {
    let mut tiles_by_overlay_id: HashMap<u32, BTreeMap<(u32, u32), u32>> = HashMap::new();

    for line in include_str!("../data/db2/WorldMapOverlayTile.csv")
        .lines()
        .skip(1)
    {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 6 {
            continue;
        }

        let row = parse_u32(fields[1]);
        let col = parse_u32(fields[2]);
        let file_data_id = parse_u32(fields[4]);
        let overlay_id = parse_u32(fields[5]);
        if file_data_id == 0 || overlay_id == 0 {
            continue;
        }

        tiles_by_overlay_id
            .entry(overlay_id)
            .or_default()
            .insert((row, col), file_data_id);
    }

    tiles_by_overlay_id
        .into_iter()
        .map(|(overlay_id, tiles)| (overlay_id, flatten_tiles(&tiles)))
        .collect()
}

fn flatten_tiles(tiles: &BTreeMap<(u32, u32), u32>) -> Vec<u32> {
    let max_row = tiles.keys().map(|&(row, _)| row).max().unwrap_or(0);
    let max_col = tiles.keys().map(|&(_, col)| col).max().unwrap_or(0);
    let tiles_wide = max_col + 1;
    let mut flattened = vec![0; ((max_row + 1) * tiles_wide) as usize];

    for (&(row, col), &file_data_id) in tiles {
        let index = (row * tiles_wide + col) as usize;
        flattened[index] = file_data_id;
    }

    flattened
}

fn parse_u32(field: &str) -> u32 {
    field.parse().unwrap_or(0)
}

fn parse_i32(field: &str) -> i32 {
    field.parse().unwrap_or(0)
}
