//! C_Quest namespaces and quest-related API functions.
//!
//! Single source of truth for all quest data. The `QUEST_LOG` array defines
//! headers and quests with objectives. All quest APIs derive from this array.

use mlua::{Lua, MultiValue, Result, Value};
use std::cell::Cell;

thread_local! {
    static SELECTED_QUEST_ID: Cell<i32> = const { Cell::new(0) };
}

/// A quest objective (leaderboard entry).
struct Objective {
    text: &'static str,
    obj_type: &'static str,
    finished: bool,
}

/// An entry in the quest log — either a zone header or a quest.
enum QuestLogEntry {
    Header {
        title: &'static str,
    },
    Quest {
        quest_id: i32,
        title: &'static str,
        description: &'static str,
        objectives: &'static [Objective],
    },
}

/// Single source of truth for the quest log.
/// Log index is the 1-based position in this array.
/// Headers group subsequent quests; the UI needs at least one header.
static QUEST_LOG: &[QuestLogEntry] = &[
    QuestLogEntry::Header {
        title: "Khaz Algar",
    },
    QuestLogEntry::Quest {
        quest_id: 80000,
        title: "The Lost Expedition",
        description: "An old journal found near the quarry entrance describes an Ironforge expedition that went missing decades ago. Scattered relics hint at their path deeper underground. Collect what remains and piece together what happened.",
        objectives: &[
            Objective {
                text: "Ironforge Relics collected: 3/5",
                obj_type: "item",
                finished: false,
            },
            Objective {
                text: "Explore the Old Quarry",
                obj_type: "event",
                finished: false,
            },
        ],
    },
    QuestLogEntry::Quest {
        quest_id: 80001,
        title: "Defending the Gates",
        description: "The Stormwind gate guards are under constant pressure from the gnoll raiders that have been sighted along the forest road. Lend your strength to the defense until reinforcements arrive from Goldshire.",
        objectives: &[Objective {
            text: "Stormwind Guards defended: 7/10",
            obj_type: "monster",
            finished: false,
        }],
    },
    QuestLogEntry::Quest {
        quest_id: 80002,
        title: "Supply Run",
        description: "The quartermaster at the forward camp is running low on provisions. Gather supplies from the nearby farmsteads and deliver them before nightfall.",
        objectives: &[
            Objective {
                text: "Supplies gathered: 5/5",
                obj_type: "item",
                finished: true,
            },
            Objective {
                text: "Deliver to Quartermaster",
                obj_type: "event",
                finished: false,
            },
        ],
    },
];

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
/// Map IDs: 2025 = Thaldraszus, 2024 = The Azure Span, 2023 = Ohn'ahran Plains,
///           2022 = The Waking Shores, 2112 = Valdrakken.
static WORLD_QUESTS: &[WorldQuest] = &[
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

/// Find a world quest by ID.
fn find_world_quest(quest_id: i32) -> Option<&'static WorldQuest> {
    WORLD_QUESTS.iter().find(|wq| wq.quest_id == quest_id)
}

/// Check if a quest ID is a seeded world quest.
pub fn is_world_quest(quest_id: i32) -> bool {
    find_world_quest(quest_id).is_some()
}

/// Number of actual quests (non-header entries).
fn quest_count() -> i32 {
    QUEST_LOG
        .iter()
        .filter(|e| matches!(e, QuestLogEntry::Quest { .. }))
        .count() as i32
}

/// Find a quest entry by quest ID, returning (log_index_1based, entry).
fn find_quest_by_id(quest_id: i32) -> Option<(i32, &'static QuestLogEntry)> {
    QUEST_LOG.iter().enumerate().find_map(|(i, e)| match e {
        QuestLogEntry::Quest { quest_id: qid, .. } if *qid == quest_id => Some((i as i32 + 1, e)),
        _ => None,
    })
}

/// Get the quest log entry at a 1-based log index.
fn entry_at(log_index: i32) -> Option<&'static QuestLogEntry> {
    QUEST_LOG.get((log_index - 1) as usize)
}

/// Public: get objective count for a quest at the given log index.
pub fn num_quest_leaderboards(log_index: i32) -> i32 {
    match entry_at(log_index) {
        Some(QuestLogEntry::Quest { objectives, .. }) => objectives.len() as i32,
        _ => 0,
    }
}

/// Public: get objective data (text, type, finished) for a quest.
pub fn quest_leaderboard_entry(log_index: i32, obj_index: i32) -> (String, String, bool) {
    if let Some(QuestLogEntry::Quest { objectives, .. }) = entry_at(log_index) {
        if let Some(obj) = objectives.get((obj_index - 1) as usize) {
            return (obj.text.into(), obj.obj_type.into(), obj.finished);
        }
    }
    ("Unknown objective".into(), "event".into(), false)
}

/// Register quest-related C_* namespaces.
pub fn register_c_quest_api(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set("C_QuestLog", register_c_quest_log(lua)?)?;
    globals.set("C_TaskQuest", register_c_task_quest(lua)?)?;
    globals.set("C_QuestInfoSystem", register_c_quest_info_system(lua)?)?;
    globals.set("C_QuestLine", register_c_quest_line(lua)?)?;
    globals.set("C_QuestOffer", register_c_quest_offer(lua)?)?;
    globals.set("C_QuestSession", register_c_quest_session(lua)?)?;
    register_quest_log_quest_text(lua)?;
    register_quest_poi_globals(lua)?;
    Ok(())
}

fn register_quest_poi_globals(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetQuestPOIBlobCount",
        lua.create_function(|_, quest_id: i32| {
            Ok(crate::quest_poi_blobs::get_quest_blobs(quest_id as u32).len() as i32)
        })?,
    )?;
    Ok(())
}

/// C_QuestLog namespace - quest log utilities.
fn register_c_quest_log(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    register_quest_log_queries(lua, &t)?;
    register_quest_log_info(lua, &t)?;
    register_quest_log_requests(lua, &t)?;
    register_quest_log_watch(lua, &t)?;
    register_quest_log_status(lua, &t)?;
    register_quest_log_selection(lua, &t)?;
    t.set("HasActiveThreats", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetBountySetInfoForMapID",
        lua.create_function(|_, _map_id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetBountiesForMapID",
        lua.create_function(|lua, _map_id: i32| lua.create_table())?,
    )?;
    t.set(
        "IsUnitOnQuest",
        lua.create_function(|_, (_unit, _quest_id): (String, i32)| Ok(false))?,
    )?;
    t.set(
        "IsWorldQuest",
        lua.create_function(|_, quest_id: i32| Ok(is_world_quest(quest_id)))?,
    )?;
    t.set(
        "IsQuestTask",
        lua.create_function(|_, quest_id: i32| Ok(is_world_quest(quest_id)))?,
    )?;
    Ok(t)
}

/// Quest log query methods (counts, GetInfo, objectives).
fn register_quest_log_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_quest_log_entry_queries(lua, t)?;
    register_quest_log_limit_queries(lua, t)?;
    register_quest_log_map_queries(lua, t)?;
    register_quest_log_misc_queries(lua, t)?;
    Ok(())
}

fn register_quest_log_entry_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    let num_entries = QUEST_LOG.len() as i32;
    let num_quests = quest_count();
    t.set(
        "GetNumQuestLogEntries",
        lua.create_function(move |_, ()| Ok((num_entries, num_quests)))?,
    )?;
    t.set(
        "GetInfo",
        lua.create_function(|lua, idx: i32| create_quest_info(lua, idx))?,
    )?;
    t.set(
        "GetQuestIDForLogIndex",
        lua.create_function(|_, idx: i32| Ok(quest_id_for_log_index(idx)))?,
    )?;
    t.set(
        "GetLogIndexForQuestID",
        lua.create_function(|_, quest_id: i32| Ok(find_quest_by_id(quest_id).map(|(idx, _)| idx)))?,
    )?;
    t.set(
        "GetQuestObjectives",
        lua.create_function(|lua, _id: i32| lua.create_table())?,
    )?;
    Ok(())
}

fn register_quest_log_limit_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetMaxNumQuestsCanAccept",
        lua.create_function(|_, ()| Ok(35i32))?,
    )?;
    t.set("GetMaxNumQuests", lua.create_function(|_, ()| Ok(35i32))?)?;
    Ok(())
}

fn register_quest_log_map_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "SetMapForQuestPOIs",
        lua.create_function(|_, _map_id: i32| Ok(()))?,
    )?;
    t.set(
        "GetZoneStoryInfo",
        lua.create_function(|_, _map_id: i32| Ok((Value::Nil, Value::Nil)))?,
    )?;
    t.set(
        "GetQuestsOnMap",
        lua.create_function(|lua, _map_id: i32| lua.create_table())?,
    )?;
    Ok(())
}

fn register_quest_log_misc_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetQuestAdditionalHighlights",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "IsQuestReplayable",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set(
        "GetQuestWatchType",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    Ok(())
}

fn quest_id_for_log_index(idx: i32) -> i32 {
    match entry_at(idx) {
        Some(QuestLogEntry::Quest { quest_id, .. }) => *quest_id,
        _ => 0,
    }
}

/// Create a quest info table for a given log index.
fn create_quest_info(lua: &Lua, idx: i32) -> Result<Value> {
    let Some(entry) = entry_at(idx) else {
        return Ok(Value::Nil);
    };
    let info = lua.create_table()?;
    info.set("questLogIndex", idx)?;
    populate_quest_info_fields(&info, entry)?;
    Ok(Value::Table(info))
}

fn populate_quest_info_fields(info: &mlua::Table, entry: &QuestLogEntry) -> Result<()> {
    match entry {
        QuestLogEntry::Header { title } => populate_header_quest_info(info, title),
        QuestLogEntry::Quest {
            quest_id, title, ..
        } => populate_log_quest_info(info, *quest_id, title),
    }
}

fn populate_header_quest_info(info: &mlua::Table, title: &str) -> Result<()> {
    info.set("title", title)?;
    info.set("questID", 0)?;
    info.set("isHeader", true)?;
    info.set("isCollapsed", false)?;
    info.set("isTask", false)?;
    info.set("isBounty", false)?;
    info.set("isHidden", false)?;
    info.set("isOnMap", false)?;
    Ok(())
}

fn populate_log_quest_info(info: &mlua::Table, quest_id: i32, title: &str) -> Result<()> {
    info.set("title", title)?;
    info.set("questID", quest_id)?;
    info.set("campaignID", 0)?;
    info.set("level", 80)?;
    info.set("difficultyLevel", 80)?;
    info.set("suggestedGroup", 0)?;
    info.set("isHeader", false)?;
    info.set("isCollapsed", false)?;
    info.set("isTask", false)?;
    info.set("isBounty", false)?;
    info.set("isStory", false)?;
    info.set("isOnMap", true)?;
    info.set("hasLocalPOI", false)?;
    info.set("isHidden", false)?;
    info.set("isAutoComplete", false)?;
    info.set("overridesSortOrder", false)?;
    info.set("startEvent", false)?;
    info.set("isScaling", false)?;
    info.set("readyForTranslation", false)?;
    Ok(())
}

/// Quest data request stubs (async data loading).
/// In WoW, these trigger server requests. We stub them as no-ops.
fn register_quest_log_requests(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "RequestLoadQuestByID",
        lua.create_function(|lua, id: i32| {
            let success = find_quest_by_id(id).is_some();
            fire_event(
                lua,
                "QUEST_DATA_LOAD_RESULT",
                &[Value::Integer(i64::from(id)), Value::Boolean(success)],
            )
        })?,
    )?;
    t.set(
        "UpdateCampaignHeaders",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    Ok(())
}

fn fire_event(lua: &Lua, event_name: &str, args: &[Value]) -> Result<()> {
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    let mut call_args = vec![Value::String(lua.create_string(event_name)?)];
    call_args.extend(args.iter().cloned());
    fire.call(MultiValue::from_vec(call_args))
}

/// Quest log info methods (titles, tags).
fn register_quest_log_info(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetTitleForQuestID",
        lua.create_function(|lua, id: i32| {
            let title = find_quest_by_id(id).map_or("Quest", |(_, e)| match e {
                QuestLogEntry::Quest { title, .. } => title,
                _ => "Quest",
            });
            Ok(Value::String(lua.create_string(title)?))
        })?,
    )?;
    t.set(
        "GetQuestTagInfo",
        lua.create_function(|lua, id: i32| {
            let info = lua.create_table()?;
            if is_world_quest(id) {
                // Enum.QuestTagType.Normal = 2
                info.set("tagID", 2)?;
                info.set("tagName", "World Quest")?;
                info.set("worldQuestType", 2)?;
                // Enum.WorldQuestQuality.Common = 0
                info.set("quality", 0)?;
                info.set("isElite", false)?;
                info.set("displayExpiration", true)?;
            } else {
                info.set("tagID", 0)?;
                info.set("tagName", "Quest")?;
                info.set("worldQuestType", Value::Nil)?;
                info.set("quality", 1)?;
                info.set("isElite", false)?;
                info.set("displayExpiration", false)?;
            }
            Ok(info)
        })?,
    )?;
    t.set(
        "GetRequiredMoney",
        lua.create_function(|_, _id: i32| Ok(0i32))?,
    )?;
    Ok(())
}

/// Quest watch list methods (tracked quests for ObjectiveTracker).
fn register_quest_log_watch(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_quest_watch_count_queries(lua, t)?;
    register_quest_watch_mutations(lua, t)?;
    register_world_quest_watch_queries(lua, t)?;
    Ok(())
}

fn register_quest_watch_count_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    let num_watches = quest_count();
    t.set(
        "GetNumQuestWatches",
        lua.create_function(move |_, ()| Ok(num_watches))?,
    )?;
    t.set(
        "GetQuestIDForQuestWatchIndex",
        lua.create_function(|_, idx: i32| Ok(watched_quest_id_at_index(idx)))?,
    )?;
    Ok(())
}

fn register_quest_watch_mutations(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("AddQuestWatch", lua.create_function(|_, _id: i32| Ok(()))?)?;
    t.set(
        "RemoveQuestWatch",
        lua.create_function(|_, _id: i32| Ok(()))?,
    )?;
    t.set("SortQuestWatches", lua.create_function(|_, ()| Ok(()))?)?;
    Ok(())
}

fn register_world_quest_watch_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetNumWorldQuestWatches",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetQuestIDForWorldQuestWatchIndex",
        lua.create_function(|_, _idx: i32| Ok(Value::Nil))?,
    )?;
    Ok(())
}

fn watched_quest_id_at_index(idx: i32) -> Option<i32> {
    if idx <= 0 {
        return None;
    }
    QUEST_LOG
        .iter()
        .filter_map(|entry| match entry {
            QuestLogEntry::Quest { quest_id, .. } => Some(*quest_id),
            _ => None,
        })
        .nth((idx - 1) as usize)
}

/// Quest status check methods.
fn register_quest_log_status(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_quest_completion_status(lua, t)?;
    register_quest_membership_status(lua, t)?;
    register_quest_status_metadata(lua, t)?;
    Ok(())
}

fn register_quest_completion_status(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "IsQuestFlaggedCompleted",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set("IsComplete", lua.create_function(|_, _id: i32| Ok(false))?)?;
    t.set(
        "ReadyForTurnIn",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set("IsFailed", lua.create_function(|_, _id: i32| Ok(false))?)?;
    t.set(
        "IsQuestDisabledForSession",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    Ok(())
}

fn register_quest_membership_status(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "IsOnQuest",
        lua.create_function(|_, id: i32| Ok(find_quest_by_id(id).is_some()))?,
    )?;
    t.set(
        "IsPushableQuest",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set(
        "IsRepeatableQuest",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set(
        "IsImportantQuest",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set("IsMetaQuest", lua.create_function(|_, _id: i32| Ok(false))?)?;
    t.set("IsOnMap", lua.create_function(|_, _id: i32| Ok(false))?)?;
    t.set(
        "IsAccountQuest",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set(
        "IsQuestCalling",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    Ok(())
}

fn register_quest_status_metadata(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetNextWaypointText",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetTimeAllowed",
        lua.create_function(|_, _id: i32| Ok((Value::Nil, Value::Nil)))?,
    )?;
    t.set(
        "GetQuestDetailsTheme",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// Selected quest state and GetQuestLogQuestText.
fn register_quest_log_selection(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "SetSelectedQuest",
        lua.create_function(|_, id: i32| {
            SELECTED_QUEST_ID.with(|c| c.set(id));
            Ok(())
        })?,
    )?;
    t.set(
        "GetSelectedQuest",
        lua.create_function(|_, ()| Ok(SELECTED_QUEST_ID.with(|c| c.get())))?,
    )?;
    Ok(())
}

/// Register GetQuestLogQuestText global.
///
/// Returns (description, objectives) for the currently selected quest.
/// Called by QuestInfo_ShowDescriptionText and QuestInfo_ShowObjectivesText.
pub fn register_quest_log_quest_text(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "GetQuestLogQuestText",
        lua.create_function(|_, ()| {
            let quest_id = SELECTED_QUEST_ID.with(|c| c.get());
            let Some((_, entry)) = find_quest_by_id(quest_id) else {
                return Ok(("".to_string(), "".to_string()));
            };
            let QuestLogEntry::Quest {
                description,
                objectives,
                ..
            } = entry
            else {
                return Ok(("".to_string(), "".to_string()));
            };
            let obj_text = objectives
                .iter()
                .map(|o| o.text)
                .collect::<Vec<_>>()
                .join("\n");
            Ok((description.to_string(), obj_text))
        })?,
    )?;
    Ok(())
}

/// C_TaskQuest namespace - world quest/task utilities.
fn register_c_task_quest(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
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
        lua.create_function(|_, (quest_id, _map_id): (i32, i32)| {
            match find_world_quest(quest_id) {
                Some(wq) => Ok((wq.x, wq.y)),
                None => Ok((0.0f64, 0.0f64)),
            }
        })?,
    )?;
    t.set(
        "GetQuestsForPlayerByMapID",
        lua.create_function(build_quests_on_map)?,
    )?;
    t.set(
        "GetQuestTimeLeftMinutes",
        lua.create_function(|_, quest_id: i32| {
            Ok(if is_world_quest(quest_id) { 120 } else { 0 })
        })?,
    )?;
    t.set(
        "GetQuestUIWidgetSetByType",
        lua.create_function(|_, (_quest_id, _set_type): (i32, i32)| Ok(Value::Nil))?,
    )?;
    t.set(
        "RequestPreloadRewardData",
        lua.create_function(|_, _quest_id: i32| Ok(()))?,
    )?;
    Ok(t)
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
            Value::Integer(0), // factionID
            Value::Boolean(false), // capped
            Value::Boolean(false), // displayAsObjective
        ])),
        None => Ok(mlua::MultiValue::new()),
    }
}

/// C_QuestInfoSystem namespace - quest classification info.
fn register_c_quest_info_system(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    // Returns Enum.QuestClassification.Normal (0)
    t.set(
        "GetQuestClassification",
        lua.create_function(|_, _quest_id: i32| Ok(0i32))?,
    )?;
    t.set(
        "HasQuestRewardCurrencies",
        lua.create_function(|_, _quest_id: i32| Ok(false))?,
    )?;
    Ok(t)
}

/// C_QuestLine namespace - questline information.
fn register_c_quest_line(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set(
        "GetQuestLineInfo",
        lua.create_function(|_, (_qid, _mid): (i32, Option<i32>)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetQuestLineQuests",
        lua.create_function(|lua, _id: i32| lua.create_table())?,
    )?;
    t.set(
        "GetAvailableQuestLines",
        lua.create_function(|lua, _id: i32| lua.create_table())?,
    )?;
    t.set("IsComplete", lua.create_function(|_, _id: i32| Ok(false))?)?;
    t.set(
        "RequestQuestLinesForMap",
        lua.create_function(|_, _id: i32| Ok(()))?,
    )?;
    Ok(t)
}

/// C_QuestSession namespace - quest session/party sync system.
fn register_c_quest_session(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set("Exists", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("HasJoined", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetAvailableSessionCommand",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set("HasPendingCommand", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetPendingCommand",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetSessionBeginDetails",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set("CanStart", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("CanStop", lua.create_function(|_, ()| Ok(false))?)?;
    Ok(t)
}

/// C_QuestOffer namespace - quest offer/reward info.
fn register_c_quest_offer(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set(
        "GetQuestOfferMajorFactionReputationRewards",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    Ok(t)
}
