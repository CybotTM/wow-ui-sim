//! Seeded quest/runtime surface restored for the rilua global registrar.
//!
//! Master used `c_quest_api.rs` for this. The rilua branch removed that file
//! without replacing the quest/watch/objective surface, which leaves the
//! objective tracker with no watched quests to render.

use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, create_string_static,
    create_table, frame_ref, table_set,
};
use crate::lua_api::script_helpers::{get_event_listeners, get_script};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use crate::quest_poi_blobs;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

type SurfaceFn = fn(&mut LuaState) -> LuaResult<u32>;

struct Objective {
    text: &'static str,
    obj_type: &'static str,
    finished: bool,
}

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

struct WorldQuest {
    quest_id: i32,
    map_id: i32,
    x: f64,
    y: f64,
    title: &'static str,
    num_objectives: i32,
}

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

static WORLD_QUESTS: &[WorldQuest] = &[
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

const QUEST_LOG_METHODS: &[(&str, SurfaceFn)] = &[
    ("GetNumQuestLogEntries", get_num_quest_log_entries),
    ("GetInfo", get_quest_log_info),
    ("GetQuestIDForLogIndex", get_quest_id_for_log_index),
    ("GetLogIndexForQuestID", get_log_index_for_quest_id),
    ("GetTitleForQuestID", get_title_for_quest_id),
    ("GetNumQuestWatches", get_num_quest_watches),
    (
        "GetQuestIDForQuestWatchIndex",
        get_quest_id_for_quest_watch_index,
    ),
    ("GetNumWorldQuestWatches", get_num_world_quest_watches),
    (
        "GetQuestIDForWorldQuestWatchIndex",
        get_quest_id_for_world_quest_watch_index,
    ),
    ("AddQuestWatch", noop),
    ("RemoveQuestWatch", noop),
    ("SortQuestWatches", noop),
    ("IsQuestFlaggedCompleted", return_false),
    ("IsComplete", return_false),
    ("ReadyForTurnIn", return_false),
    ("IsFailed", return_false),
    ("IsQuestDisabledForSession", return_false),
    ("IsPushableQuest", return_false),
    ("IsRepeatableQuest", return_false),
    ("IsImportantQuest", return_false),
    ("IsMetaQuest", return_false),
    ("IsOnMap", return_false),
    ("IsOnQuest", is_on_quest),
    ("IsWorldQuest", is_world_quest_fn),
    ("IsQuestTask", is_quest_task),
    ("IsQuestBounty", return_false),
    ("GetQuestTagInfo", get_quest_tag_info),
    ("GetRequiredMoney", get_required_money),
    ("GetNextWaypointText", get_next_waypoint_text),
    ("GetTimeAllowed", get_time_allowed),
    ("GetQuestDetailsTheme", return_nil),
    ("RequestLoadQuestByID", request_load_quest_by_id),
    ("SetSelectedQuest", set_selected_quest),
    ("GetSelectedQuest", get_selected_quest),
];

const TASK_QUEST_METHODS: &[(&str, SurfaceFn)] = &[
    ("IsActive", task_quest_is_active),
    ("GetQuestsOnMap", build_task_quest_info),
    ("GetQuestsForPlayerByMapID", build_task_quest_info),
    ("GetQuestInfoByQuestID", task_quest_get_quest_info_by_id),
    ("GetQuestLocation", task_quest_get_quest_location),
    ("GetQuestTimeLeftMinutes", task_quest_time_left_minutes),
    ("GetQuestTimeLeftSeconds", task_quest_time_left_seconds),
];

const GLOBAL_QUEST_FUNCTIONS: &[(&str, SurfaceFn)] = &[
    ("GetNumQuestLeaderBoards", get_num_quest_leaderboards),
    ("GetQuestLogLeaderBoard", get_quest_log_leaderboard),
    ("GetQuestLogQuestText", get_quest_log_quest_text),
    ("GetQuestPOIBlobCount", get_quest_poi_blob_count),
    ("HaveQuestData", have_quest_data),
    ("HaveQuestRewardData", have_quest_data),
    ("IsQuestSequenced", is_quest_sequenced),
    ("GetQuestLogCompletionText", get_quest_log_completion_text),
    ("GetQuestProgressBarPercent", get_quest_progress_bar_percent),
    (
        "QuestMapFrame_GetFocusedQuestID",
        quest_map_frame_get_focused_quest_id,
    ),
    (
        "GetQuestLogSpecialItemInfo",
        get_quest_log_special_item_info,
    ),
];

fn quest_count() -> i32 {
    QUEST_LOG
        .iter()
        .filter(|entry| matches!(entry, QuestLogEntry::Quest { .. }))
        .count() as i32
}

fn find_quest_by_id(quest_id: i32) -> Option<(i32, &'static QuestLogEntry)> {
    QUEST_LOG
        .iter()
        .enumerate()
        .find_map(|(index, entry)| match entry {
            QuestLogEntry::Quest { quest_id: id, .. } if *id == quest_id => {
                Some((index as i32 + 1, entry))
            }
            _ => None,
        })
}

fn find_world_quest(quest_id: i32) -> Option<&'static WorldQuest> {
    WORLD_QUESTS.iter().find(|quest| quest.quest_id == quest_id)
}

fn quest_exists(quest_id: i32) -> bool {
    find_quest_by_id(quest_id).is_some()
}

fn is_world_quest(quest_id: i32) -> bool {
    find_world_quest(quest_id).is_some()
}

fn entry_at(log_index: i32) -> Option<&'static QuestLogEntry> {
    QUEST_LOG.get((log_index - 1) as usize)
}

fn watched_quest_id_at_index(index: i32) -> Option<i32> {
    if index <= 0 {
        return None;
    }
    QUEST_LOG
        .iter()
        .filter_map(|entry| match entry {
            QuestLogEntry::Quest { quest_id, .. } => Some(*quest_id),
            _ => None,
        })
        .nth((index - 1) as usize)
}

fn objective_at(log_index: i32, objective_index: i32) -> Option<&'static Objective> {
    match entry_at(log_index) {
        Some(QuestLogEntry::Quest { objectives, .. }) if objective_index > 0 => {
            objectives.get((objective_index - 1) as usize)
        }
        _ => None,
    }
}

fn selected_quest_id(state: &LuaState) -> LuaResult<i32> {
    Ok(borrow_state(state)?
        .selected_quest_log_id
        .map(|id| id as i32)
        .unwrap_or(0))
}

fn set_selected_quest_id(state: &mut LuaState, quest_id: i32) -> LuaResult<()> {
    borrow_state_mut(state)?.selected_quest_log_id = (quest_id > 0).then_some(quest_id as u32);
    Ok(())
}

fn ensure_global_table(state: &mut LuaState, name: &'static str) -> GcRef<Table> {
    let key = state.gc.intern_string_static(name.as_bytes());
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|table| table.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(table_ref)) = existing {
        return table_ref;
    }

    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), table, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    table_ref
}

fn set_array_value(state: &mut LuaState, table_ref: GcRef<Table>, index: i32, value: Val) {
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(Val::Num(index as f64), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(table_ref);
}

fn fire_event_with_args(state: &mut LuaState, event_name: &'static str, args: &[Val]) {
    for widget_id in get_event_listeners(state, event_name) {
        let Some(handler) = get_script(state, widget_id, "OnEvent") else {
            continue;
        };
        let Ok(frame) = frame_ref(state, widget_id) else {
            continue;
        };

        let mut call_args = Vec::with_capacity(2 + args.len());
        call_args.push(frame);
        call_args.push(create_string_static(state, event_name));
        call_args.extend_from_slice(args);
        let _ = call_function_state(state, handler, &call_args);
    }
}

fn get_num_quest_log_entries(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(QUEST_LOG.len() as f64));
    state.push(Val::Num(quest_count() as f64));
    Ok(2)
}

fn get_quest_log_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let Some(entry) = entry_at(index) else {
        return Ok(0);
    };

    let info = create_table(state);
    table_set(state, info, "questLogIndex", Val::Num(index as f64));
    match entry {
        QuestLogEntry::Header { title } => {
            let title = create_string(state, title);
            table_set(state, info, "title", title);
            table_set(state, info, "questID", Val::Num(0.0));
            table_set(state, info, "isHeader", Val::Bool(true));
            table_set(state, info, "isCollapsed", Val::Bool(false));
            table_set(state, info, "isTask", Val::Bool(false));
            table_set(state, info, "isBounty", Val::Bool(false));
            table_set(state, info, "isHidden", Val::Bool(false));
            table_set(state, info, "isOnMap", Val::Bool(false));
        }
        QuestLogEntry::Quest {
            quest_id, title, ..
        } => {
            let title = create_string(state, title);
            table_set(state, info, "title", title);
            table_set(state, info, "questID", Val::Num(*quest_id as f64));
            table_set(state, info, "campaignID", Val::Num(0.0));
            table_set(state, info, "level", Val::Num(80.0));
            table_set(state, info, "difficultyLevel", Val::Num(80.0));
            table_set(state, info, "suggestedGroup", Val::Num(0.0));
            table_set(state, info, "isHeader", Val::Bool(false));
            table_set(state, info, "isCollapsed", Val::Bool(false));
            table_set(state, info, "isTask", Val::Bool(false));
            table_set(state, info, "isBounty", Val::Bool(false));
            table_set(state, info, "isStory", Val::Bool(false));
            table_set(state, info, "isOnMap", Val::Bool(true));
            table_set(state, info, "hasLocalPOI", Val::Bool(false));
            table_set(state, info, "isHidden", Val::Bool(false));
            table_set(state, info, "isAutoComplete", Val::Bool(false));
            table_set(state, info, "overridesSortOrder", Val::Bool(false));
            table_set(state, info, "startEvent", Val::Bool(false));
            table_set(state, info, "isScaling", Val::Bool(false));
            table_set(state, info, "readyForTranslation", Val::Bool(false));
        }
    }

    state.push(info);
    Ok(1)
}

fn get_quest_id_for_log_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let quest_id = match entry_at(index) {
        Some(QuestLogEntry::Quest { quest_id, .. }) => *quest_id,
        _ => 0,
    };
    state.push(Val::Num(quest_id as f64));
    Ok(1)
}

fn get_log_index_for_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    match find_quest_by_id(quest_id) {
        Some((index, _)) => {
            state.push(Val::Num(index as f64));
            Ok(1)
        }
        None => Ok(0),
    }
}

fn get_title_for_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let title = match find_quest_by_id(quest_id) {
        Some((_, QuestLogEntry::Quest { title, .. })) => *title,
        _ => "Quest",
    };
    let title = create_string(state, title);
    state.push(title);
    Ok(1)
}

fn get_num_quest_watches(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(quest_count() as f64));
    Ok(1)
}

fn get_quest_id_for_quest_watch_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    match watched_quest_id_at_index(index) {
        Some(quest_id) => {
            state.push(Val::Num(quest_id as f64));
            Ok(1)
        }
        None => Ok(0),
    }
}

fn get_num_world_quest_watches(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_quest_id_for_world_quest_watch_index(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn return_nil(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn is_world_quest_fn(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(is_world_quest(quest_id)));
    Ok(1)
}

fn is_quest_task(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(is_world_quest(quest_id)));
    Ok(1)
}

fn is_on_quest(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(quest_exists(quest_id)));
    Ok(1)
}

fn get_quest_tag_info(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let info = create_table(state);
    if is_world_quest(quest_id) {
        table_set(state, info, "tagID", Val::Num(2.0));
        let tag_name = create_string_static(state, "World Quest");
        table_set(state, info, "tagName", tag_name);
        table_set(state, info, "worldQuestType", Val::Num(2.0));
        table_set(state, info, "quality", Val::Num(0.0));
        table_set(state, info, "isElite", Val::Bool(false));
        table_set(state, info, "displayExpiration", Val::Bool(true));
    } else {
        table_set(state, info, "tagID", Val::Num(0.0));
        let tag_name = create_string_static(state, "Quest");
        table_set(state, info, "tagName", tag_name);
        table_set(state, info, "quality", Val::Num(1.0));
        table_set(state, info, "isElite", Val::Bool(false));
        table_set(state, info, "displayExpiration", Val::Bool(false));
    }
    state.push(info);
    Ok(1)
}

fn get_required_money(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_next_waypoint_text(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn get_time_allowed(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

fn set_selected_quest(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    set_selected_quest_id(state, quest_id)?;
    Ok(0)
}

fn get_selected_quest(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(selected_quest_id(state)? as f64));
    Ok(1)
}

fn request_load_quest_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let success = quest_exists(quest_id) || is_world_quest(quest_id);
    fire_event_with_args(
        state,
        "QUEST_DATA_LOAD_RESULT",
        &[Val::Num(quest_id as f64), Val::Bool(success)],
    );
    Ok(0)
}

fn get_num_quest_leaderboards(state: &mut LuaState) -> LuaResult<u32> {
    let log_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let count = match entry_at(log_index) {
        Some(QuestLogEntry::Quest { objectives, .. }) => objectives.len() as i32,
        _ => 0,
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_quest_log_leaderboard(state: &mut LuaState) -> LuaResult<u32> {
    let objective_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let log_index = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as i32;
    let Some(objective) = objective_at(log_index, objective_index) else {
        return Ok(0);
    };

    let text = create_string(state, objective.text);
    let obj_type = create_string(state, objective.obj_type);
    state.push(text);
    state.push(obj_type);
    state.push(Val::Bool(objective.finished));
    Ok(3)
}

fn get_quest_log_quest_text(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = selected_quest_id(state)?;
    let Some((
        _,
        QuestLogEntry::Quest {
            description,
            objectives,
            ..
        },
    )) = find_quest_by_id(quest_id)
    else {
        let empty = create_string_static(state, "");
        state.push(empty);
        let empty = create_string_static(state, "");
        state.push(empty);
        return Ok(2);
    };

    let objective_lines = objectives
        .iter()
        .map(|objective| objective.text)
        .collect::<Vec<_>>()
        .join("\n");
    let description = create_string(state, description);
    let objective_lines = create_string(state, &objective_lines);
    state.push(description);
    state.push(objective_lines);
    Ok(2)
}

fn get_quest_poi_blob_count(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    state.push(Val::Num(
        quest_poi_blobs::get_quest_blobs(quest_id).len() as f64
    ));
    Ok(1)
}

fn have_quest_data(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(
        quest_exists(quest_id) || is_world_quest(quest_id),
    ));
    Ok(1)
}

fn is_quest_sequenced(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn get_quest_log_completion_text(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn get_quest_progress_bar_percent(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn quest_map_frame_get_focused_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_quest_log_special_item_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn build_task_quest_info(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let result = create_table(state);
    let Val::Table(result_ref) = result else {
        unreachable!("create_table must return a table");
    };

    let mut out_index = 1;
    for quest in WORLD_QUESTS.iter().filter(|quest| quest.map_id == map_id) {
        let info = create_table(state);
        table_set(state, info, "questID", Val::Num(quest.quest_id as f64));
        table_set(state, info, "x", Val::Num(quest.x));
        table_set(state, info, "y", Val::Num(quest.y));
        table_set(state, info, "mapID", Val::Num(quest.map_id as f64));
        table_set(
            state,
            info,
            "numObjectives",
            Val::Num(quest.num_objectives as f64),
        );
        table_set(state, info, "isMapIndicatorQuest", Val::Bool(false));
        set_array_value(state, result_ref, out_index, info);
        out_index += 1;
    }

    state.push(result);
    Ok(1)
}

fn task_quest_is_active(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(is_world_quest(quest_id)));
    Ok(1)
}

fn task_quest_get_quest_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let Some(quest) = find_world_quest(quest_id) else {
        return Ok(0);
    };

    let title = create_string(state, quest.title);
    state.push(title);
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    Ok(4)
}

fn task_quest_get_quest_location(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let (x, y) = match find_world_quest(quest_id) {
        Some(quest) => (quest.x, quest.y),
        None => (0.0, 0.0),
    };
    state.push(Val::Num(x));
    state.push(Val::Num(y));
    Ok(2)
}

fn task_quest_time_left_minutes(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    if is_world_quest(quest_id) {
        state.push(Val::Num(SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES as f64));
        return Ok(1);
    }
    Ok(0)
}

fn task_quest_time_left_seconds(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    if is_world_quest(quest_id) {
        state.push(Val::Num((SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES * 60) as f64));
        return Ok(1);
    }
    Ok(0)
}

fn register_c_quest_log(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_global_table(state, "C_QuestLog");
    for (name, func) in QUEST_LOG_METHODS {
        table_set_rust_fn(state, table_ref, name, *func)?;
    }
    Ok(())
}

fn register_c_task_quest(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_global_table(state, "C_TaskQuest");
    for (name, func) in TASK_QUEST_METHODS {
        table_set_rust_fn(state, table_ref, name, *func)?;
    }
    Ok(())
}

fn register_c_quest_info_system(state: &mut LuaState) -> LuaResult<()> {
    fn get_quest_classification(state: &mut LuaState) -> LuaResult<u32> {
        let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
        let classification = if is_world_quest(quest_id) { 10.0 } else { 7.0 };
        state.push(Val::Num(classification));
        Ok(1)
    }

    let table_ref = ensure_global_table(state, "C_QuestInfoSystem");
    table_set_rust_fn(
        state,
        table_ref,
        "GetQuestClassification",
        get_quest_classification,
    )?;
    Ok(())
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    for (name, func) in GLOBAL_QUEST_FUNCTIONS {
        LuaApiMut::register_function(lua, name, *func)?;
    }

    let state = lua.state_mut();
    register_c_quest_log(state)?;
    register_c_task_quest(state)?;
    register_c_quest_info_system(state)?;
    Ok(())
}
