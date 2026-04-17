//! `C_QuestLog` probe surface backed by `SimState.quest_log_entries`.
//!
//! Migrates 17 entries off the namespace stub tables:
//!
//! - `GetBountySetInfoForMapID(mapID)` — nil (no bounty board data).
//! - `GetInfo(logIndex)` — returns QuestInfo table or nil.
//! - `GetNextWaypoint(questID)` — returns (x, y) or nothing.
//! - `GetQuestDetailsTheme(questID)` — returns theme string or nil.
//! - `GetQuestTagInfo(questID)` — returns QuestTagInfo table or nil.
//! - `GetWorldQuestInfo(questID)` — returns WorldQuestInfo table or nil.
//! - `GetAllCompletedQuestIDs()` — returns array of completed quest IDs.
//! - `GetLogIndexForQuestID(questID)` — returns 1-based index or nil.
//! - `GetNumQuestLogEntries()` — returns (shownCount, totalCount).
//! - `IsComplete(questID)` — returns bool.
//! - `IsFailed(questID)` — returns bool.
//! - `IsMetaQuest(questID)` — returns bool.
//! - `IsOnMap(questID)` — returns bool.
//! - `IsOnQuest(questID)` — returns bool.
//! - `IsQuestFlaggedCompleted(questID)` — returns bool.
//! - `IsQuestReplayable(questID)` — returns bool.
//! - `IsWorldQuest(questID)` — returns bool.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_api::sim_substates::QuestLogEntry;
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_quest_log_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_QuestLog")?;
    table_set_rust_fn(
        state,
        ns,
        "GetBountySetInfoForMapID",
        get_bounty_set_info_for_map_id,
    )?;
    table_set_rust_fn(state, ns, "GetInfo", get_info)?;
    table_set_rust_fn(state, ns, "GetNextWaypoint", get_next_waypoint)?;
    table_set_rust_fn(state, ns, "GetQuestDetailsTheme", get_quest_details_theme)?;
    table_set_rust_fn(state, ns, "GetQuestTagInfo", get_quest_tag_info)?;
    table_set_rust_fn(state, ns, "GetWorldQuestInfo", get_world_quest_info)?;
    table_set_rust_fn(
        state,
        ns,
        "GetAllCompletedQuestIDs",
        get_all_completed_quest_ids,
    )?;
    table_set_rust_fn(
        state,
        ns,
        "GetLogIndexForQuestID",
        get_log_index_for_quest_id,
    )?;
    table_set_rust_fn(
        state,
        ns,
        "GetNumQuestLogEntries",
        get_num_quest_log_entries,
    )?;
    table_set_rust_fn(state, ns, "IsComplete", is_complete)?;
    table_set_rust_fn(state, ns, "IsFailed", is_failed)?;
    table_set_rust_fn(state, ns, "IsMetaQuest", is_meta_quest)?;
    table_set_rust_fn(state, ns, "IsOnMap", is_on_map)?;
    table_set_rust_fn(state, ns, "IsOnQuest", is_on_quest)?;
    table_set_rust_fn(
        state,
        ns,
        "IsQuestFlaggedCompleted",
        is_quest_flagged_completed,
    )?;
    table_set_rust_fn(state, ns, "IsQuestReplayable", is_quest_replayable)?;
    table_set_rust_fn(state, ns, "IsWorldQuest", is_world_quest)?;
    Ok(())
}

fn get_bounty_set_info_for_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let _map_id = i32::from_stack(state, 1)?;
    // No bounty board data in the sim.
    state.push(Val::Nil);
    Ok(1)
}

fn get_info(state: &mut LuaState) -> LuaResult<u32> {
    let log_index = i32::from_stack(state, 1)?;
    if log_index < 1 {
        return Ok(0);
    }
    let entry = borrow_state(state)?
        .quest_log_entries
        .entries
        .get((log_index - 1) as usize)
        .cloned();
    let Some(entry) = entry else { return Ok(0) };

    let t = create_table(state);
    write_quest_identity_fields(state, t, &entry, log_index);
    write_quest_classification_flags(state, t, &entry);
    write_quest_constant_stub_fields(state, t, &entry);
    state.push(t);
    Ok(1)
}

fn write_quest_identity_fields(
    state: &mut LuaState,
    t: Val,
    entry: &QuestLogEntry,
    log_index: i32,
) {
    table_set(state, t, "questID", Val::Num(entry.quest_id as f64));
    let title = create_string(state, &entry.title);
    table_set(state, t, "title", title);
    table_set(state, t, "level", Val::Num(entry.level as f64));
    table_set(state, t, "questLogIndex", Val::Num(log_index as f64));
    table_set(state, t, "difficultyLevel", Val::Num(entry.level as f64));
    table_set(state, t, "isComplete", Val::Bool(entry.is_complete));
    table_set(state, t, "isFailed", Val::Bool(entry.is_failed));
}

fn write_quest_classification_flags(state: &mut LuaState, t: Val, entry: &QuestLogEntry) {
    table_set(state, t, "isMeta", Val::Bool(entry.is_meta));
    table_set(state, t, "isWorldQuest", Val::Bool(entry.is_world_quest));
    table_set(state, t, "isTask", Val::Bool(entry.is_world_quest));
    table_set(state, t, "isOnMap", Val::Bool(entry.map_id.is_some()));
}

/// Fields that the sim always reports with a constant value — headers,
/// bounty/story categories, POI flags, sort/scaling toggles, and the
/// campaign/suggestedGroup identifiers we don't model.
fn write_quest_constant_stub_fields(state: &mut LuaState, t: Val, _entry: &QuestLogEntry) {
    table_set(state, t, "isHeader", Val::Bool(false));
    table_set(state, t, "isCollapsed", Val::Bool(false));
    table_set(state, t, "isBounty", Val::Bool(false));
    table_set(state, t, "isStory", Val::Bool(false));
    table_set(state, t, "hasLocalPOI", Val::Bool(false));
    table_set(state, t, "isHidden", Val::Bool(false));
    table_set(state, t, "isAutoComplete", Val::Bool(false));
    table_set(state, t, "overridesSortOrder", Val::Bool(false));
    table_set(state, t, "startEvent", Val::Bool(false));
    table_set(state, t, "isScaling", Val::Bool(false));
    table_set(state, t, "readyForTranslation", Val::Bool(false));
    table_set(state, t, "campaignID", Val::Num(0.0));
    table_set(state, t, "suggestedGroup", Val::Num(0.0));
}

fn get_next_waypoint(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let waypoint = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .find(|e| e.quest_id == quest_id)
        .and_then(|e| e.waypoint);
    match waypoint {
        Some((x, y)) => {
            state.push(Val::Num(x));
            state.push(Val::Num(y));
            Ok(2)
        }
        None => Ok(0),
    }
}

fn get_quest_details_theme(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let theme = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .find(|e| e.quest_id == quest_id)
        .and_then(|e| e.details_theme.clone());
    match theme {
        Some(t) => {
            let s = create_string(state, &t);
            state.push(s);
            Ok(1)
        }
        None => Ok(0),
    }
}

fn get_quest_tag_info(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let entry = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .find(|e| e.quest_id == quest_id)
        .cloned();
    let Some(entry) = entry else {
        return Ok(0);
    };
    let t = create_table(state);
    let tag_id = entry.tag_id.unwrap_or(0);
    table_set(state, t, "tagID", Val::Num(tag_id as f64));
    let tag_name = if entry.is_world_quest {
        "World Quest"
    } else {
        "Quest"
    };
    let tag_name_val = create_string(state, tag_name);
    table_set(state, t, "tagName", tag_name_val);
    if entry.is_world_quest {
        table_set(state, t, "worldQuestType", Val::Num(2.0));
        table_set(state, t, "quality", Val::Num(0.0));
        table_set(state, t, "displayExpiration", Val::Bool(true));
    } else {
        table_set(state, t, "quality", Val::Num(1.0));
        table_set(state, t, "displayExpiration", Val::Bool(false));
    }
    table_set(state, t, "isElite", Val::Bool(false));
    state.push(t);
    Ok(1)
}

fn get_world_quest_info(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let entry = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .find(|e| e.quest_id == quest_id && e.is_world_quest)
        .cloned();
    let Some(entry) = entry else { return Ok(0) };

    let t = create_table(state);
    table_set(state, t, "questID", Val::Num(entry.quest_id as f64));
    let map_id = entry.map_id.unwrap_or(0);
    table_set(state, t, "mapID", Val::Num(map_id as f64));
    if let Some((x, y)) = entry.waypoint {
        table_set(state, t, "x", Val::Num(x));
        table_set(state, t, "y", Val::Num(y));
    }
    table_set(state, t, "numObjectives", Val::Num(1.0));
    table_set(state, t, "isElite", Val::Bool(false));
    table_set(state, t, "tradeskillLineIndex", Val::Num(0.0));
    state.push(t);
    Ok(1)
}

fn get_all_completed_quest_ids(state: &mut LuaState) -> LuaResult<u32> {
    let ids: Vec<i32> = borrow_state(state)?
        .quest_log_entries
        .completed_quest_ids
        .iter()
        .copied()
        .collect();
    let array = create_table(state);
    for (i, id) in ids.into_iter().enumerate() {
        set_table_array(state, array, i as i64 + 1, Val::Num(id as f64));
    }
    state.push(array);
    Ok(1)
}

fn get_log_index_for_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let idx = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .enumerate()
        .find_map(|(i, e)| {
            if e.quest_id == quest_id {
                Some(i as i32 + 1)
            } else {
                None
            }
        });
    match idx {
        Some(i) => {
            state.push(Val::Num(i as f64));
            Ok(1)
        }
        None => Ok(0),
    }
}

fn get_num_quest_log_entries(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.quest_log_entries.entries.len() as f64;
    // shownCount = totalCount (no collapsed headers in sim entries)
    state.push(Val::Num(count));
    state.push(Val::Num(count));
    Ok(2)
}

fn quest_bool_field(
    state: &mut LuaState,
    f: fn(&crate::lua_api::sim_substates::QuestLogEntry) -> bool,
) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let val = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .find(|e| e.quest_id == quest_id)
        .map(f)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

fn is_complete(state: &mut LuaState) -> LuaResult<u32> {
    quest_bool_field(state, |e| e.is_complete)
}

fn is_failed(state: &mut LuaState) -> LuaResult<u32> {
    quest_bool_field(state, |e| e.is_failed)
}

fn is_meta_quest(state: &mut LuaState) -> LuaResult<u32> {
    quest_bool_field(state, |e| e.is_meta)
}

fn is_on_map(state: &mut LuaState) -> LuaResult<u32> {
    quest_bool_field(state, |e| e.map_id.is_some())
}

fn is_on_quest(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let found = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .any(|e| e.quest_id == quest_id);
    state.push(Val::Bool(found));
    Ok(1)
}

fn is_quest_flagged_completed(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let flagged = borrow_state(state)?
        .quest_log_entries
        .completed_quest_ids
        .contains(&quest_id);
    state.push(Val::Bool(flagged));
    Ok(1)
}

fn is_quest_replayable(state: &mut LuaState) -> LuaResult<u32> {
    quest_bool_field(state, |e| e.is_replayable)
}

fn is_world_quest(state: &mut LuaState) -> LuaResult<u32> {
    quest_bool_field(state, |e| e.is_world_quest)
}
