//! Tests for MapCanvas pin infrastructure and world quest pin display.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn test_default_map_is_isle_of_dorn() {
    let env = env();
    let map_id: i32 = env.eval(r#"return C_Map.GetCurrentMapID()"#).unwrap();
    assert_eq!(map_id, 2248, "Default map should be Isle of Dorn");
}

#[test]
fn test_isle_of_dorn_has_seeded_world_quests() {
    let env = env();
    let count: i32 = env
        .eval(r#"return #C_TaskQuest.GetQuestsForPlayerByMapID(2248)"#)
        .unwrap();
    assert_eq!(
        count, 2,
        "Isle of Dorn (2248) should have 2 seeded world quests"
    );
}

#[test]
fn test_default_map_has_world_quests() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local mapID = C_Map.GetCurrentMapID()
            return #C_TaskQuest.GetQuestsForPlayerByMapID(mapID)
        "#,
        )
        .unwrap();
    assert!(
        count > 0,
        "Default map should have seeded world quests for pin display"
    );
}
