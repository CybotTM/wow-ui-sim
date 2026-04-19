//! Tests for `C_Map` probes backed by `SimState.maps` +
//! `SimState.player_map_position`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_map_art_id_returns_seeded_art_id() {
    let env = env();
    let (stormwind, eastern_kingdoms, azeroth): (i32, i32, i32) = env
        .eval(
            r#"
            return C_Map.GetMapArtID(84),
                   C_Map.GetMapArtID(13),
                   C_Map.GetMapArtID(946)
            "#,
        )
        .unwrap();
    assert_eq!(stormwind, 104, "Stormwind City art id");
    assert_eq!(eastern_kingdoms, 62, "Eastern Kingdoms art id");
    assert_eq!(azeroth, 0, "Azeroth world map has no tileset");
}

#[test]
fn get_map_art_id_returns_nothing_for_unknown_map() {
    let env = env();
    let nret: i32 = env
        .eval("return select('#', C_Map.GetMapArtID(999999))")
        .unwrap();
    assert_eq!(nret, 0);
}

#[test]
fn get_map_children_info_returns_direct_children() {
    let env = env();
    let (count, first_id, first_name, first_type, first_parent): (i32, i32, String, i32, i32) = env
        .eval(
            r#"
            local children = C_Map.GetMapChildrenInfo(13)
            local first = children[1]
            return #children, first.mapID, first.name, first.mapType, first.parentMapID
            "#,
        )
        .unwrap();
    assert_eq!(
        count, 1,
        "Eastern Kingdoms has one seeded child (Stormwind)"
    );
    assert_eq!(first_id, 84);
    assert_eq!(first_name, "Stormwind City");
    assert_eq!(first_type, 3, "Zone");
    assert_eq!(first_parent, 13);
}

#[test]
fn get_map_children_info_with_all_descendants_walks_tree() {
    let env = env();
    let (count, ids): (i32, Vec<i32>) = env
        .eval(
            r#"
            local function array(tbl, key)
                local out = {}
                for i = 1, #tbl do out[i] = tbl[i][key] end
                return out
            end
            local children = C_Map.GetMapChildrenInfo(946, nil, true)
            return #children, array(children, "mapID")
            "#,
        )
        .unwrap();
    assert_eq!(count, 2, "Azeroth → Eastern Kingdoms → Stormwind");
    let mut sorted = ids;
    sorted.sort();
    assert_eq!(sorted, vec![13, 84]);
}

#[test]
fn get_map_children_info_filters_by_map_type() {
    let env = env();
    // Azeroth (946) has a Continent (13) and Stormwind (84, Zone).
    // Ask for Zones only with allDescendants=true — we should get 84.
    let (count, first_id, first_type): (i32, i32, i32) = env
        .eval(
            r#"
            local zones = C_Map.GetMapChildrenInfo(946, 3, true)
            return #zones, zones[1].mapID, zones[1].mapType
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(first_id, 84);
    assert_eq!(first_type, 3);
}

#[test]
fn get_map_children_info_returns_empty_array_for_childless_map() {
    let env = env();
    let count: i32 = env.eval("return #C_Map.GetMapChildrenInfo(84)").unwrap();
    assert_eq!(count, 0, "Stormwind City has no seeded children");
}

#[test]
fn get_map_children_info_returns_nothing_for_unknown_map() {
    let env = env();
    let nret: i32 = env
        .eval("return select('#', C_Map.GetMapChildrenInfo(999999))")
        .unwrap();
    assert_eq!(nret, 0);
}

#[test]
fn get_player_map_position_returns_default_center() {
    let env = env();
    let (x, y): (f64, f64) = env
        .eval(
            r#"
            local pos = C_Map.GetPlayerMapPosition(84, "player")
            return pos.x, pos.y
            "#,
        )
        .unwrap();
    assert_eq!(x, 0.5);
    assert_eq!(y, 0.5);
}

#[test]
fn get_player_map_position_reflects_sim_state_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.player_map_position = (0.25, 0.75);
    }

    let (x, y): (f64, f64) = env
        .eval(
            r#"
            local pos = C_Map.GetPlayerMapPosition(84, "player")
            return pos.x, pos.y
            "#,
        )
        .unwrap();
    assert_eq!(x, 0.25);
    assert_eq!(y, 0.75);
}

#[test]
fn get_player_map_position_returns_nil_for_unknown_map() {
    let env = env();
    let is_nil: bool = env
        .eval(r#"return C_Map.GetPlayerMapPosition(999999, "player") == nil"#)
        .unwrap();
    assert!(is_nil);
}

#[test]
fn get_player_map_position_returns_nil_for_non_player_unit() {
    let env = env();
    let is_nil: bool = env
        .eval(r#"return C_Map.GetPlayerMapPosition(84, "target") == nil"#)
        .unwrap();
    assert!(is_nil);
}

#[test]
fn set_map_for_quest_log_updates_current_map_id() {
    let env = env();
    let (before, after): (i32, i32) = env
        .eval(
            r#"
            local before = C_Map.GetCurrentMapID()
            C_Map.SetMapForQuestLog(1)
            return before, C_Map.GetCurrentMapID()
            "#,
        )
        .unwrap();
    assert_eq!(before, 2248);
    assert_eq!(after, 1);
}
