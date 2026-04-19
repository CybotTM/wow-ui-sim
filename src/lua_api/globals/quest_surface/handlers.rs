//! Quest surface handler functions registered into the Lua environment.

use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, create_string_static,
    create_table, frame_ref, table_set,
};
use crate::lua_api::script_helpers::{get_event_listeners, get_script};
use crate::lua_bridge::FromStack;
use crate::quest_poi_blobs;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

use super::data::{
    Objective, QUEST_LOG, QuestLogEntry, SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES, WORLD_QUESTS,
    WorldQuest,
};

// ---------------------------------------------------------------------------
// Data lookup helpers
// ---------------------------------------------------------------------------

pub fn quest_count() -> i32 {
    QUEST_LOG
        .iter()
        .filter(|entry| matches!(entry, QuestLogEntry::Quest { .. }))
        .count() as i32
}

pub fn find_quest_by_id(quest_id: i32) -> Option<(i32, &'static QuestLogEntry)> {
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

pub fn find_world_quest(quest_id: i32) -> Option<&'static WorldQuest> {
    WORLD_QUESTS.iter().find(|q| q.quest_id == quest_id)
}

pub fn quest_exists(quest_id: i32) -> bool {
    find_quest_by_id(quest_id).is_some()
}

pub fn is_world_quest(quest_id: i32) -> bool {
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

fn set_array_value(state: &mut LuaState, table_ref: GcRef<Table>, index: i32, value: Val) {
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(Val::Num(index as f64), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(table_ref);
}

// ---------------------------------------------------------------------------
// `get_quest_log_info` field writers
// ---------------------------------------------------------------------------

fn write_quest_header_fields(state: &mut LuaState, info: Val, title: &str) {
    let title_val = create_string(state, title);
    table_set(state, info, "title", title_val);
    table_set(state, info, "questID", Val::Num(0.0));
    table_set(state, info, "isHeader", Val::Bool(true));
    table_set(state, info, "isCollapsed", Val::Bool(false));
    table_set(state, info, "isTask", Val::Bool(false));
    table_set(state, info, "isBounty", Val::Bool(false));
    table_set(state, info, "isHidden", Val::Bool(false));
    table_set(state, info, "isOnMap", Val::Bool(false));
}

fn write_quest_entry_fields(state: &mut LuaState, info: Val, quest_id: i32, title: &str) {
    let title_val = create_string(state, title);
    table_set(state, info, "title", title_val);
    table_set(state, info, "questID", Val::Num(quest_id as f64));
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

// ---------------------------------------------------------------------------
// C_QuestLog handlers
// ---------------------------------------------------------------------------

pub fn get_num_quest_log_entries(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(QUEST_LOG.len() as f64));
    state.push(Val::Num(quest_count() as f64));
    Ok(2)
}

pub fn get_quest_log_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let Some(entry) = entry_at(index) else {
        return Ok(0);
    };
    let info = create_table(state);
    table_set(state, info, "questLogIndex", Val::Num(index as f64));
    match entry {
        QuestLogEntry::Header { title } => write_quest_header_fields(state, info, title),
        QuestLogEntry::Quest {
            quest_id, title, ..
        } => write_quest_entry_fields(state, info, *quest_id, title),
    }
    state.push(info);
    Ok(1)
}

pub fn get_quest_id_for_log_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let quest_id = match entry_at(index) {
        Some(QuestLogEntry::Quest { quest_id, .. }) => *quest_id,
        _ => 0,
    };
    state.push(Val::Num(quest_id as f64));
    Ok(1)
}

pub fn get_log_index_for_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    match find_quest_by_id(quest_id) {
        Some((index, _)) => {
            state.push(Val::Num(index as f64));
            Ok(1)
        }
        None => Ok(0),
    }
}

pub fn get_title_for_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let title = match find_quest_by_id(quest_id) {
        Some((_, QuestLogEntry::Quest { title, .. })) => *title,
        _ => "Quest",
    };
    let title_val = create_string(state, title);
    state.push(title_val);
    Ok(1)
}

pub fn get_quest_link(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let Some((_, QuestLogEntry::Quest { title, .. })) = find_quest_by_id(quest_id) else {
        return Ok(0);
    };
    let link = create_string(
        state,
        &format!("|cffffff00|Hquest:{quest_id}|h[{title}]|h|r"),
    );
    state.push(link);
    Ok(1)
}

pub fn get_num_quest_watches(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(quest_count() as f64));
    Ok(1)
}

pub fn get_quest_id_for_quest_watch_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    match watched_quest_id_at_index(index) {
        Some(quest_id) => {
            state.push(Val::Num(quest_id as f64));
            Ok(1)
        }
        None => Ok(0),
    }
}

pub fn get_num_world_quest_watches(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

pub fn get_quest_id_for_world_quest_watch_index(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

pub fn return_nil(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn is_world_quest_fn(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(is_world_quest(quest_id)));
    Ok(1)
}

pub fn is_quest_task(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(is_world_quest(quest_id)));
    Ok(1)
}

pub fn is_on_quest(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(quest_exists(quest_id)));
    Ok(1)
}

pub fn get_quest_tag_info(state: &mut LuaState) -> LuaResult<u32> {
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

pub fn get_required_money(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

pub fn get_next_waypoint_text(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn get_time_allowed(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

pub fn set_selected_quest(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    set_selected_quest_id(state, quest_id)?;
    Ok(0)
}

pub fn get_selected_quest(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(selected_quest_id(state)? as f64));
    Ok(1)
}

pub fn request_load_quest_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let success = quest_exists(quest_id) || is_world_quest(quest_id);
    fire_event_with_args(
        state,
        "QUEST_DATA_LOAD_RESULT",
        &[Val::Num(quest_id as f64), Val::Bool(success)],
    );
    Ok(0)
}

// ---------------------------------------------------------------------------
// Global quest function handlers
// ---------------------------------------------------------------------------

pub fn get_num_quest_leaderboards(state: &mut LuaState) -> LuaResult<u32> {
    let log_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let count = match entry_at(log_index) {
        Some(QuestLogEntry::Quest { objectives, .. }) => objectives.len() as i32,
        _ => 0,
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub fn get_quest_log_leaderboard(state: &mut LuaState) -> LuaResult<u32> {
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

pub fn get_quest_log_quest_text(state: &mut LuaState) -> LuaResult<u32> {
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
        let empty1 = create_string_static(state, "");
        state.push(empty1);
        let empty2 = create_string_static(state, "");
        state.push(empty2);
        return Ok(2);
    };

    let objective_lines = objectives
        .iter()
        .map(|o| o.text)
        .collect::<Vec<_>>()
        .join("\n");
    let description_val = create_string(state, description);
    let objectives_val = create_string(state, &objective_lines);
    state.push(description_val);
    state.push(objectives_val);
    Ok(2)
}

pub fn get_quest_poi_blob_count(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as u32;
    state.push(Val::Num(
        quest_poi_blobs::get_quest_blobs(quest_id).len() as f64
    ));
    Ok(1)
}

pub fn have_quest_data(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(
        quest_exists(quest_id) || is_world_quest(quest_id),
    ));
    Ok(1)
}

pub fn is_quest_sequenced(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

pub fn get_quest_log_completion_text(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn get_quest_progress_bar_percent(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

pub fn quest_map_frame_get_focused_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let selected_quest_log_id = borrow_state(state)?.selected_quest_log_id;
    match selected_quest_log_id {
        Some(quest_id) => {
            state.push(Val::Num(quest_id as f64));
            Ok(1)
        }
        None => Ok(0),
    }
}

/// `QuestMapUpdateAllQuests()` — retail returns the number of POIs that
/// were found on the current world map. The sim approximates POI count as
/// the number of quest-log entries that are actual quests (not headers).
pub fn quest_map_update_all_quests(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(quest_count() as f64));
    Ok(1)
}

/// `GetQuestLogTimeLeft()` — retail returns seconds remaining on a
/// time-limited quest, or `nil` when the currently selected quest isn't
/// timed. The seeded quest-log has no time-limited entries, so we only
/// return a value when the selected quest resolves to one of the seeded
/// world quests.
pub fn get_quest_log_time_left(state: &mut LuaState) -> LuaResult<u32> {
    let selected = borrow_state(state)?.selected_quest_log_id;
    match selected {
        Some(quest_id) if is_world_quest(quest_id as i32) => {
            state.push(Val::Num(SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES as f64 * 60.0));
            Ok(1)
        }
        _ => Ok(0),
    }
}

pub fn get_quest_log_special_item_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// ---------------------------------------------------------------------------
// C_TaskQuest handlers
// ---------------------------------------------------------------------------

pub fn build_task_quest_info(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let result = create_table(state);
    let Val::Table(result_ref) = result else {
        unreachable!("create_table must return a table");
    };

    let mut out_index = 1;
    for quest in WORLD_QUESTS.iter().filter(|q| q.map_id == map_id) {
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

pub fn task_quest_is_active(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(is_world_quest(quest_id)));
    Ok(1)
}

pub fn does_map_show_task_quest_objectives(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let shows_objectives = WORLD_QUESTS.iter().any(|quest| quest.map_id == map_id);
    state.push(Val::Bool(shows_objectives));
    Ok(1)
}

pub fn task_quest_get_quest_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
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

pub fn task_quest_get_quest_location(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let (x, y) = find_world_quest(quest_id)
        .map(|q| (q.x, q.y))
        .unwrap_or((0.0, 0.0));
    state.push(Val::Num(x));
    state.push(Val::Num(y));
    Ok(2)
}

pub fn task_quest_time_left_minutes(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    if is_world_quest(quest_id) {
        state.push(Val::Num(SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES as f64));
        return Ok(1);
    }
    Ok(0)
}

pub fn task_quest_time_left_seconds(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    if is_world_quest(quest_id) {
        state.push(Val::Num((SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES * 60) as f64));
        return Ok(1);
    }
    Ok(0)
}
