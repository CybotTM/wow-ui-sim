use super::*;

struct DefaultMapSeed {
    ui_map_id: i32,
    name: &'static str,
    map_type: i32,
    parent_map_id: i32,
    art_id: i32,
    child_map_ids: &'static [i32],
}

const DEFAULT_MAP_SEEDS: &[DefaultMapSeed] = &[
    DefaultMapSeed {
        ui_map_id: 946,
        name: "Azeroth",
        map_type: 1,
        parent_map_id: 0,
        art_id: 0,
        child_map_ids: &[13],
    },
    DefaultMapSeed {
        ui_map_id: 13,
        name: "Eastern Kingdoms",
        map_type: 2,
        parent_map_id: 946,
        art_id: 62,
        child_map_ids: &[84],
    },
    DefaultMapSeed {
        ui_map_id: 1,
        name: "Dun Morogh",
        map_type: 3,
        parent_map_id: 0,
        art_id: 12,
        child_map_ids: &[],
    },
    DefaultMapSeed {
        ui_map_id: 84,
        name: "Stormwind City",
        map_type: 3,
        parent_map_id: 13,
        art_id: 104,
        child_map_ids: &[],
    },
    DefaultMapSeed {
        ui_map_id: 2248,
        name: "Isle of Dorn",
        map_type: 3,
        parent_map_id: 0,
        art_id: 5920,
        child_map_ids: &[],
    },
    DefaultMapSeed {
        ui_map_id: 1409,
        name: "Exile's Reach",
        map_type: 3,
        parent_map_id: 0,
        art_id: 0,
        child_map_ids: &[],
    },
    DefaultMapSeed {
        ui_map_id: 1670,
        name: "Oribos",
        map_type: 3,
        parent_map_id: 0,
        art_id: 0,
        child_map_ids: &[],
    },
];

/// Seed the `SimState.maps` table with the handful of ui-map ids
/// commonly referenced by Blizzard UI (Azeroth world map, Eastern
/// Kingdoms continent, Stormwind City zone). Retail ids from
/// wago.tools / Wowpedia.
pub(in crate::lua_api::state) fn default_maps() -> HashMap<i32, MapData> {
    DEFAULT_MAP_SEEDS.iter().map(default_map_entry).collect()
}

fn default_map_entry(seed: &DefaultMapSeed) -> (i32, MapData) {
    let map = MapData {
        ui_map_id: seed.ui_map_id,
        name: seed.name.into(),
        map_type: seed.map_type,
        parent_map_id: seed.parent_map_id,
        art_id: seed.art_id,
        flags: 0,
        child_map_ids: seed.child_map_ids.to_vec(),
        child_rects: Vec::new(),
        rect_on_parent: None,
    };

    (map.ui_map_id, map)
}
