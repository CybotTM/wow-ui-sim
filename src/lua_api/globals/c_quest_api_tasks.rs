//! C_TaskQuest namespace and world quest data.
//!
//! World quest types, seeded data, and the C_TaskQuest registration
//! extracted from c_quest_api to keep file sizes manageable.

use mlua::{Lua, Result, Value};

/// A world quest (task) that appears as a map pin.
struct WorldQuest {
    quest_id: i32,
    map_id: i32,
    x: f64,
    y: f64,
    title: &'static str,
    num_objectives: i32,
}

/// Seeded world quests for map display.
/// Map IDs: 2248 = Isle of Dorn, 2215 = Hallowfall, 2214 = The Ringing Deeps,
///           2025 = Thaldraszus, 2024 = The Azure Span, 2023 = Ohn'ahran Plains,
///           2022 = The Waking Shores.
static WORLD_QUESTS: &[WorldQuest] = &[
    // TWW zones (visible on default Khaz Algar map)
    WorldQuest {
        quest_id: 90101,
        map_id: 2248,
        x: 0.45,
        y: 0.35,
        title: "Earthen Relic Recovery",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90102,
        map_id: 2248,
        x: 0.62,
        y: 0.58,
        title: "Arathi Signal Fires",
        num_objectives: 2,
    },
    WorldQuest {
        quest_id: 90103,
        map_id: 2215,
        x: 0.40,
        y: 0.50,
        title: "Crystal Shard Collection",
        num_objectives: 3,
    },
    WorldQuest {
        quest_id: 90104,
        map_id: 2214,
        x: 0.55,
        y: 0.45,
        title: "Kobold Tunnel Collapse",
        num_objectives: 1,
    },
    // Dragon Isles zones
    WorldQuest {
        quest_id: 90001,
        map_id: 2025,
        x: 0.52,
        y: 0.63,
        title: "Glittering Geodes",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90002,
        map_id: 2025,
        x: 0.38,
        y: 0.41,
        title: "Temporal Rift Collapse",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90003,
        map_id: 2024,
        x: 0.47,
        y: 0.55,
        title: "Frozen Tuskarr Supplies",
        num_objectives: 3,
    },
    WorldQuest {
        quest_id: 90004,
        map_id: 2024,
        x: 0.62,
        y: 0.32,
        title: "Brackenhide Gnolls",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90005,
        map_id: 2023,
        x: 0.71,
        y: 0.48,
        title: "Storm-Charged Hunt",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90006,
        map_id: 2023,
        x: 0.35,
        y: 0.62,
        title: "Centaur Caravan Defense",
        num_objectives: 2,
    },
    WorldQuest {
        quest_id: 90007,
        map_id: 2022,
        x: 0.58,
        y: 0.70,
        title: "Lava Surge Containment",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90008,
        map_id: 2022,
        x: 0.44,
        y: 0.35,
        title: "Djaradin Weapon Cache",
        num_objectives: 2,
    },
];

const SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES: i32 = 120;

/// Find a world quest by ID.
fn find_world_quest(quest_id: i32) -> Option<&'static WorldQuest> {
    WORLD_QUESTS.iter().find(|wq| wq.quest_id == quest_id)
}

/// Check if a quest ID is a seeded world quest.
pub fn is_world_quest(quest_id: i32) -> bool {
    find_world_quest(quest_id).is_some()
}

fn quest_time_left_minutes(quest_id: i32) -> Option<i32> {
    is_world_quest(quest_id).then_some(SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES)
}

fn quest_time_left_seconds(quest_id: i32) -> Option<i32> {
    quest_time_left_minutes(quest_id).map(|minutes| minutes * 60)
}

/// Global quest data availability functions.
pub(super) fn register_quest_data_globals(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "HaveQuestData",
        lua.create_function(|_, quest_id: i32| {
            Ok(super::c_quest_api::quest_exists(quest_id) || is_world_quest(quest_id))
        })?,
    )?;
    globals.set(
        "HaveQuestRewardData",
        lua.create_function(|_, quest_id: i32| {
            Ok(super::c_quest_api::quest_exists(quest_id) || is_world_quest(quest_id))
        })?,
    )?;
    Ok(())
}

/// C_TaskQuest namespace - world quest/task utilities.
pub(super) fn register_c_task_quest(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    register_task_quest_queries(lua, &t)?;
    register_task_quest_stubs(lua, &t)?;
    Ok(t)
}

fn register_task_quest_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_task_quest_core_queries(lua, t)?;
    register_task_quest_time_queries(lua, t)?;
    Ok(())
}

fn register_task_quest_core_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "IsActive",
        lua.create_function(|_, quest_id: i32| Ok(is_world_quest(quest_id)))?,
    )?;
    t.set("GetQuestsOnMap", lua.create_function(build_quests_on_map)?)?;
    t.set(
        "GetQuestInfoByQuestID",
        lua.create_function(build_quest_info_by_id)?,
    )?;
    t.set(
        "GetQuestLocation",
        lua.create_function(build_quest_location)?,
    )?;
    t.set(
        "GetQuestsForPlayerByMapID",
        lua.create_function(build_quests_on_map)?,
    )?;
    Ok(())
}

fn register_task_quest_time_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetQuestTimeLeftMinutes",
        lua.create_function(|_, quest_id: i32| {
            Ok(quest_time_left_value(quest_time_left_minutes(quest_id)))
        })?,
    )?;
    t.set(
        "GetQuestTimeLeftSeconds",
        lua.create_function(|_, quest_id: i32| {
            Ok(quest_time_left_value(quest_time_left_seconds(quest_id)))
        })?,
    )?;
    Ok(())
}

fn quest_time_left_value(time_left: Option<i32>) -> Value {
    time_left
        .map(|value| Value::Integer(i64::from(value)))
        .unwrap_or(Value::Nil)
}

fn register_task_quest_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetQuestUIWidgetSetByType",
        lua.create_function(|_, (_quest_id, _set_type): (i32, i32)| Ok(Value::Nil))?,
    )?;
    t.set(
        "RequestPreloadRewardData",
        lua.create_function(|_, _quest_id: i32| Ok(()))?,
    )?;
    Ok(())
}

fn build_quest_location(_lua: &Lua, (quest_id, _map_id): (i32, i32)) -> Result<(f64, f64)> {
    match find_world_quest(quest_id) {
        Some(wq) => Ok((wq.x, wq.y)),
        None => Ok((0.0, 0.0)),
    }
}

/// Build the task info array for a given map ID from seeded world quests.
fn build_quests_on_map(lua: &Lua, map_id: i32) -> Result<mlua::Table> {
    let result = lua.create_table()?;
    let mut idx = 1;
    for wq in WORLD_QUESTS.iter().filter(|wq| wq.map_id == map_id) {
        let info = lua.create_table()?;
        info.set("questID", wq.quest_id)?;
        info.set("x", wq.x)?;
        info.set("y", wq.y)?;
        info.set("mapID", wq.map_id)?;
        info.set("numObjectives", wq.num_objectives)?;
        info.set("isMapIndicatorQuest", false)?;
        result.set(idx, info)?;
        idx += 1;
    }
    Ok(result)
}

/// Return (title, factionID, capped, displayAsObjective) for a world quest.
fn build_quest_info_by_id(_lua: &Lua, quest_id: i32) -> Result<mlua::MultiValue> {
    match find_world_quest(quest_id) {
        Some(wq) => Ok(mlua::MultiValue::from_vec(vec![
            Value::String(_lua.create_string(wq.title)?),
            Value::Integer(0),     // factionID
            Value::Boolean(false), // capped
            Value::Boolean(false), // displayAsObjective
        ])),
        None => Ok(mlua::MultiValue::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::{SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES, WORLD_QUESTS, quest_time_left_value};
    use crate::lua_api::WowLuaEnv;
    use mlua::Value;

    #[test]
    fn quest_time_left_value_converts_some_and_none() {
        assert!(matches!(quest_time_left_value(None), Value::Nil));
        assert!(matches!(
            quest_time_left_value(Some(45)),
            Value::Integer(45)
        ));
    }

    #[test]
    fn task_quest_time_left_queries_return_minutes_seconds_and_nil() {
        let quest_id = WORLD_QUESTS[0].quest_id;
        let env = WowLuaEnv::new().expect("failed to create lua env");
        let result: (i64, i64, bool) = env
            .eval(&format!(
                r#"
                return C_TaskQuest.GetQuestTimeLeftMinutes({quest_id}),
                    C_TaskQuest.GetQuestTimeLeftSeconds({quest_id}),
                    C_TaskQuest.GetQuestTimeLeftMinutes(999999) == nil
                "#
            ))
            .expect("task quest time queries should run");

        assert_eq!(
            result,
            (
                i64::from(SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES),
                i64::from(SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES * 60),
                true,
            )
        );
    }
}
