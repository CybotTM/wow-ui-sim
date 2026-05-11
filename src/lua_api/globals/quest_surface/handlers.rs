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

const QUEST_WIDGET_SET_BASE_ID: i32 = 1_000_000;
const SEEDED_REWARD_ITEM_ID: i32 = 6948;
const QUEST_GREETING_TEXT: &str = "How can I help you, adventurer?";
const QUEST_REWARD_TEXT: &str = "You will receive:";

#[derive(Clone, Copy)]
struct StaticQuest {
    title: &'static str,
    description: &'static str,
    objectives: &'static [Objective],
}

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

fn watch_index_for_quest_id(quest_id: i32) -> Option<i32> {
    QUEST_LOG
        .iter()
        .filter_map(|entry| match entry {
            QuestLogEntry::Quest { quest_id, .. } => Some(*quest_id),
            _ => None,
        })
        .position(|watched_quest_id| watched_quest_id == quest_id)
        .map(|index| index as i32 + 1)
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

fn selected_log_index(state: &LuaState) -> LuaResult<i32> {
    Ok(find_quest_by_id(selected_quest_id(state)?)
        .map(|(index, _)| index)
        .unwrap_or(0))
}

fn set_selected_quest_id(state: &mut LuaState, quest_id: i32) -> LuaResult<()> {
    borrow_state_mut(state)?.selected_quest_log_id = (quest_id > 0).then_some(quest_id as u32);
    Ok(())
}

fn selected_criteria_spell(state: &LuaState) -> LuaResult<Option<(i32, String, String, bool)>> {
    let sim_state = borrow_state(state)?;
    let Some(quest_id) = sim_state.selected_quest_log_id.map(|id| id as i32) else {
        return Ok(None);
    };
    let Some(entry) = sim_state
        .quest_log_entries
        .entries
        .iter()
        .find(|entry| entry.quest_id == quest_id)
    else {
        return Ok(None);
    };
    let Some(spell_id) = entry.criteria_spell_id else {
        return Ok(None);
    };
    let spell_name = entry.criteria_spell_name.clone().unwrap_or_default();
    let spell_texture = entry.criteria_spell_texture.clone().unwrap_or_default();
    Ok(Some((
        spell_id,
        spell_name,
        spell_texture,
        entry.criteria_spell_finished,
    )))
}

fn quest_list_index(state: &mut LuaState) -> LuaResult<usize> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as isize;
    Ok(index.saturating_sub(1) as usize)
}

fn static_quest_by_id(quest_id: i32) -> Option<StaticQuest> {
    QUEST_LOG.iter().find_map(|entry| match entry {
        QuestLogEntry::Quest {
            quest_id: id,
            title,
            description,
            objectives,
        } if *id == quest_id => Some(StaticQuest {
            title,
            description,
            objectives,
        }),
        _ => None,
    })
}

fn selected_static_quest(state: &LuaState) -> LuaResult<Option<StaticQuest>> {
    Ok(static_quest_by_id(selected_quest_id(state)?))
}

fn join_objective_text(quest: StaticQuest) -> String {
    quest
        .objectives
        .iter()
        .map(|objective| objective.text)
        .collect::<Vec<_>>()
        .join("\n")
}

fn reward_items_for_quest(
    state: &LuaState,
    quest_id: i32,
) -> LuaResult<Vec<crate::lua_api::state::QuestRewardItem>> {
    let rewards = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .find(|entry| entry.quest_id == quest_id)
        .map(|entry| entry.reward_items.clone())
        .unwrap_or_default();
    Ok(rewards)
}

fn quest_has_rewards(state: &LuaState, quest_id: i32) -> LuaResult<bool> {
    let has_rewards = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .find(|entry| entry.quest_id == quest_id)
        .is_some_and(|entry| !entry.reward_items.is_empty() || !entry.currency_rewards.is_empty());
    Ok(has_rewards)
}

fn push_optional_i32(state: &mut LuaState, value: Option<i32>) {
    match value {
        Some(value) => state.push(Val::Num(value as f64)),
        None => state.push(Val::Nil),
    }
}

fn push_optional_bool(state: &mut LuaState, value: Option<bool>) {
    match value {
        Some(value) => state.push(Val::Bool(value)),
        None => state.push(Val::Nil),
    }
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
    let count = borrow_state(state)?.quest_log_entries.entries.len() as f64;
    state.push(Val::Num(count));
    state.push(Val::Num(count));
    Ok(2)
}

pub fn get_quest_log_title(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let Some(entry) = entry_at(index) else {
        return Ok(0);
    };

    match entry {
        QuestLogEntry::Header { title } => push_legacy_quest_log_title(
            state,
            LegacyQuestLogTitle {
                title,
                level: 0,
                quest_id: None,
                is_header: true,
                ..LegacyQuestLogTitle::default()
            },
        ),
        QuestLogEntry::Quest {
            quest_id, title, ..
        } => {
            let is_complete = is_completed_quest(state, *quest_id)?;
            push_legacy_quest_log_title(
                state,
                LegacyQuestLogTitle {
                    title,
                    level: 80,
                    quest_id: Some(*quest_id),
                    is_complete,
                    is_on_map: true,
                    ..LegacyQuestLogTitle::default()
                },
            )
        }
    }
}

#[derive(Default)]
struct LegacyQuestLogTitle<'a> {
    title: &'a str,
    level: i32,
    quest_id: Option<i32>,
    is_header: bool,
    is_collapsed: bool,
    is_complete: bool,
    is_on_map: bool,
    has_local_poi: bool,
    is_task: bool,
    is_bounty: bool,
    is_story: bool,
}

fn push_legacy_quest_log_title(
    state: &mut LuaState,
    row: LegacyQuestLogTitle<'_>,
) -> LuaResult<u32> {
    let title = create_string(state, row.title);
    state.push(title);
    state.push(Val::Num(row.level as f64));
    state.push(Val::Nil);
    state.push(Val::Bool(row.is_header));
    state.push(Val::Bool(row.is_collapsed));
    push_legacy_completion_flag(state, row.is_complete);
    state.push(Val::Num(0.0));
    push_optional_number(state, row.quest_id);
    state.push(Val::Bool(false));
    push_optional_number(state, row.quest_id);
    state.push(Val::Bool(row.is_on_map));
    state.push(Val::Bool(row.has_local_poi));
    state.push(Val::Bool(row.is_task));
    state.push(Val::Bool(row.is_bounty));
    state.push(Val::Bool(row.is_story));
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    Ok(17)
}

fn push_optional_number(state: &mut LuaState, value: Option<i32>) {
    match value {
        Some(value) => state.push(Val::Num(value as f64)),
        None => state.push(Val::Nil),
    }
}

fn push_legacy_completion_flag(state: &mut LuaState, is_complete: bool) {
    if is_complete {
        state.push(Val::Num(1.0));
    } else {
        state.push(Val::Nil);
    }
}

fn is_completed_quest(state: &LuaState, quest_id: i32) -> LuaResult<bool> {
    let completed = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .find(|entry| entry.quest_id == quest_id)
        .map(|entry| entry.is_complete || entry.is_flagged_completed)
        .unwrap_or(false);
    Ok(completed)
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

pub fn get_quest_log_index_by_id(state: &mut LuaState) -> LuaResult<u32> {
    get_log_index_for_quest_id(state)
}

pub fn select_quest_log_entry(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let selected_quest_id = match entry_at(index) {
        Some(QuestLogEntry::Quest { quest_id, .. }) => Some(*quest_id as u32),
        _ => None,
    };
    borrow_state_mut(state)?.selected_quest_log_id = selected_quest_id;
    Ok(0)
}

pub fn get_quest_log_selection(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(selected_log_index(state)? as f64));
    Ok(1)
}

pub fn get_quest_log_selected_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(selected_quest_id(state)? as f64));
    Ok(1)
}

pub fn get_abandon_quest_name(state: &mut LuaState) -> LuaResult<u32> {
    let Some(quest) = selected_static_quest(state)? else {
        return Ok(0);
    };
    let title = create_string(state, quest.title);
    state.push(title);
    Ok(1)
}

pub fn can_abandon_quest(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(quest_exists(quest_id)));
    Ok(1)
}

pub fn get_abandon_quest_items(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

pub fn get_quest_log_pushable(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

pub fn get_quest_ui_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let map_id = borrow_state(state)?
        .quest_log_entries
        .entries
        .iter()
        .find(|entry| entry.quest_id == quest_id)
        .and_then(|entry| entry.map_id)
        .unwrap_or(2248);
    state.push(Val::Num(map_id as f64));
    Ok(1)
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

pub fn get_quest_index_for_watch(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let log_index = watched_quest_id_at_index(index)
        .and_then(|quest_id| find_quest_by_id(quest_id).map(|(log_index, _)| log_index))
        .unwrap_or(0);
    state.push(Val::Num(log_index as f64));
    Ok(1)
}

pub fn quest_poi_get_quest_id_by_visible_index(state: &mut LuaState) -> LuaResult<u32> {
    let visible_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let (quest_id, log_index) = visible_quest_at_index(visible_index).unwrap_or((0, 0));
    state.push(Val::Num(quest_id as f64));
    state.push(Val::Num(log_index as f64));
    Ok(2)
}

fn visible_quest_at_index(visible_index: i32) -> Option<(i32, i32)> {
    if visible_index <= 0 {
        return None;
    }

    if let Some(QuestLogEntry::Quest { quest_id, .. }) = entry_at(visible_index) {
        return Some((*quest_id, visible_index));
    }

    watched_quest_id_at_index(visible_index)
        .and_then(|quest_id| find_quest_by_id(quest_id).map(|(log_index, _)| (quest_id, log_index)))
}

pub fn get_quest_watch_index(state: &mut LuaState) -> LuaResult<u32> {
    let log_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let watch_index = entry_at(log_index)
        .and_then(|entry| match entry {
            QuestLogEntry::Quest { quest_id, .. } => watch_index_for_quest_id(*quest_id),
            QuestLogEntry::Header { .. } => None,
        })
        .unwrap_or(0);
    state.push(Val::Num(watch_index as f64));
    Ok(1)
}

pub fn get_quest_sort_index(state: &mut LuaState) -> LuaResult<u32> {
    get_quest_watch_index(state)
}

pub fn is_quest_watched(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let watched = matches!(entry_at(index), Some(QuestLogEntry::Quest { .. }));
    state.push(Val::Bool(watched));
    Ok(1)
}

pub fn is_quest_complete(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(is_completed_quest(state, quest_id)?));
    Ok(1)
}

pub fn is_unit_on_quest(state: &mut LuaState) -> LuaResult<u32> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let on_quest = matches!(entry_at(index), Some(QuestLogEntry::Quest { .. }));
    state.push(Val::Bool(on_quest));
    Ok(1)
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

pub fn should_show_quest_rewards(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    state.push(Val::Bool(quest_has_rewards(state, quest_id)?));
    Ok(1)
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

pub fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

pub fn get_quest_reward_currencies(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let currencies = {
        let sim = borrow_state(state)?;
        sim.quest_log_entries
            .entries
            .iter()
            .find(|entry| entry.quest_id == quest_id)
            .map(|entry| entry.currency_rewards.clone())
            .unwrap_or_default()
    };
    let list_val = create_table(state);
    let Val::Table(list_ref) = list_val else {
        return Ok(0);
    };
    for (zero_based_index, currency) in currencies.into_iter().enumerate() {
        let entry_val = build_currency_reward_table(state, &currency);
        set_array_value(state, list_ref, (zero_based_index + 1) as i32, entry_val);
    }
    state.push(list_val);
    Ok(1)
}

fn build_currency_reward_table(
    state: &mut LuaState,
    currency: &crate::lua_api::state::QuestRewardCurrency,
) -> Val {
    let entry = create_table(state);
    let name_val = create_string(state, &currency.name);
    let texture_val = create_string(state, &currency.texture);
    table_set(
        state,
        entry,
        "currencyID",
        Val::Num(currency.currency_id as f64),
    );
    table_set(state, entry, "name", name_val);
    table_set(state, entry, "texture", texture_val);
    table_set(
        state,
        entry,
        "totalRewardAmount",
        Val::Num(currency.total_reward_amount as f64),
    );
    if let Some(base) = currency.base_reward_amount {
        table_set(state, entry, "baseRewardAmount", Val::Num(base as f64));
    }
    entry
}

pub fn get_quest_log_reward_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_index = i32::from_stack(state, 1)?;
    let quest_id = i32::from_stack(state, 2)?;
    if item_index < 1 {
        return Ok(0);
    }
    let item = {
        let sim = borrow_state(state)?;
        sim.quest_log_entries
            .entries
            .iter()
            .find(|entry| entry.quest_id == quest_id)
            .and_then(|entry| entry.reward_items.get((item_index - 1) as usize).cloned())
    };
    let Some(item) = item else {
        return Ok(0);
    };
    let name_val = create_string(state, &item.name);
    let texture_val = create_string(state, &item.texture);
    state.push(name_val);
    state.push(texture_val);
    state.push(Val::Num(item.count as f64));
    state.push(Val::Num(item.quality as f64));
    state.push(Val::Bool(item.is_usable));
    Ok(5)
}

pub fn get_suggested_group_size(state: &mut LuaState) -> LuaResult<u32> {
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

pub fn get_greeting_text(state: &mut LuaState) -> LuaResult<u32> {
    let text = create_string_static(state, QUEST_GREETING_TEXT);
    state.push(text);
    Ok(1)
}

pub fn get_num_active_quests(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.gossip.active_quests.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub fn get_num_available_quests(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.gossip.available_quests.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub fn get_active_title(state: &mut LuaState) -> LuaResult<u32> {
    let index = quest_list_index(state)?;
    let row = borrow_state(state)?
        .gossip
        .active_quests
        .get(index)
        .cloned();
    let Some(row) = row else {
        return Ok(0);
    };
    let title = create_string(state, &row.title);
    state.push(title);
    state.push(Val::Bool(row.is_complete.unwrap_or(false)));
    Ok(2)
}

pub fn get_active_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let index = quest_list_index(state)?;
    let quest_id = borrow_state(state)?
        .gossip
        .active_quests
        .get(index)
        .map(|row| row.quest_id)
        .unwrap_or(0);
    state.push(Val::Num(quest_id as f64));
    Ok(1)
}

pub fn is_active_quest_trivial(state: &mut LuaState) -> LuaResult<u32> {
    let index = quest_list_index(state)?;
    let is_trivial = borrow_state(state)?
        .gossip
        .active_quests
        .get(index)
        .is_some_and(|row| row.is_trivial);
    state.push(Val::Bool(is_trivial));
    Ok(1)
}

pub fn get_available_title(state: &mut LuaState) -> LuaResult<u32> {
    let index = quest_list_index(state)?;
    let title = borrow_state(state)?
        .gossip
        .available_quests
        .get(index)
        .map(|row| row.title.clone())
        .unwrap_or_default();
    let title = create_string(state, &title);
    state.push(title);
    Ok(1)
}

pub fn get_available_quest_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = quest_list_index(state)?;
    let row = borrow_state(state)?
        .gossip
        .available_quests
        .get(index)
        .cloned();
    let Some(row) = row else {
        return Ok(0);
    };
    state.push(Val::Bool(row.is_trivial));
    push_optional_i32(state, row.frequency);
    push_optional_bool(state, row.repeatable);
    state.push(Val::Bool(row.is_legendary));
    state.push(Val::Num(row.quest_id as f64));
    state.push(Val::Bool(row.is_important));
    state.push(Val::Bool(row.is_meta));
    state.push(Val::Num(row.quest_info_id as f64));
    Ok(8)
}

pub fn select_active_quest(state: &mut LuaState) -> LuaResult<u32> {
    let index = quest_list_index(state)?;
    let row = borrow_state(state)?
        .gossip
        .active_quests
        .get(index)
        .cloned();
    let Some(row) = row else {
        return Ok(0);
    };
    set_selected_quest_id(state, row.quest_id as i32)?;
    let event = if row.is_complete.unwrap_or(false) {
        "QUEST_COMPLETE"
    } else {
        "QUEST_PROGRESS"
    };
    fire_event_with_args(state, event, &[]);
    Ok(0)
}

pub fn select_available_quest(state: &mut LuaState) -> LuaResult<u32> {
    let index = quest_list_index(state)?;
    let row = borrow_state(state)?
        .gossip
        .available_quests
        .get(index)
        .cloned();
    let Some(row) = row else {
        return Ok(0);
    };
    {
        let mut sim = borrow_state_mut(state)?;
        sim.pending_quest_offer = Some(row.quest_id);
        sim.selected_quest_log_id = Some(row.quest_id);
    }
    fire_event_with_args(state, "QUEST_DETAIL", &[]);
    Ok(0)
}

pub fn get_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = selected_quest_id(state)?;
    state.push(Val::Num(quest_id as f64));
    Ok(1)
}

pub fn get_title_text(state: &mut LuaState) -> LuaResult<u32> {
    let title = selected_static_quest(state)?
        .map(|quest| quest.title)
        .unwrap_or("Quest");
    let title = create_string(state, title);
    state.push(title);
    Ok(1)
}

pub fn get_quest_text(state: &mut LuaState) -> LuaResult<u32> {
    let description = selected_static_quest(state)?
        .map(|quest| quest.description)
        .unwrap_or("");
    let description = create_string(state, description);
    state.push(description);
    Ok(1)
}

pub fn get_objective_text(state: &mut LuaState) -> LuaResult<u32> {
    let objective_text = selected_static_quest(state)?
        .map(join_objective_text)
        .unwrap_or_default();
    let objective_text = create_string(state, &objective_text);
    state.push(objective_text);
    Ok(1)
}

pub fn get_reward_text(state: &mut LuaState) -> LuaResult<u32> {
    let reward_text = create_string_static(state, QUEST_REWARD_TEXT);
    state.push(reward_text);
    Ok(1)
}

pub fn get_num_quest_rewards(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = selected_quest_id(state)?;
    let count = reward_items_for_quest(state, quest_id)?.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub fn get_quest_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let reward_type = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let reward_index = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as usize;
    if reward_type != "reward" || reward_index == 0 {
        return Ok(0);
    }
    let quest_id = selected_quest_id(state)?;
    let item = reward_items_for_quest(state, quest_id)?
        .get(reward_index - 1)
        .cloned();
    let Some(item) = item else {
        return Ok(0);
    };
    let name = create_string(state, &item.name);
    let texture = create_string(state, &item.texture);
    state.push(name);
    state.push(texture);
    state.push(Val::Num(item.count as f64));
    state.push(Val::Num(item.quality as f64));
    state.push(Val::Bool(item.is_usable));
    state.push(Val::Num(SEEDED_REWARD_ITEM_ID as f64));
    Ok(6)
}

pub fn get_quest_item_info_loot_type(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

pub fn get_num_quest_leaderboards(state: &mut LuaState) -> LuaResult<u32> {
    let log_index = Option::<f64>::from_stack(state, 1)?
        .map(|index| index as i32)
        .unwrap_or(selected_log_index(state)?);
    let count = match entry_at(log_index) {
        Some(QuestLogEntry::Quest { objectives, .. }) => objectives.len() as i32,
        _ => 0,
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub fn get_quest_log_leaderboard(state: &mut LuaState) -> LuaResult<u32> {
    let objective_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let log_index = Option::<f64>::from_stack(state, 2)?
        .map(|index| index as i32)
        .unwrap_or(selected_log_index(state)?);
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

pub fn is_current_quest_failed(state: &mut LuaState) -> LuaResult<u32> {
    let is_failed = {
        let sim_state = borrow_state(state)?;
        sim_state
            .selected_quest_log_id
            .map(|id| id as i32)
            .and_then(|quest_id| {
                sim_state
                    .quest_log_entries
                    .entries
                    .iter()
                    .find(|entry| entry.quest_id == quest_id)
                    .map(|entry| entry.is_failed)
            })
            .unwrap_or(false)
    };
    state.push(Val::Bool(is_failed));
    Ok(1)
}

pub fn is_quest_sequenced(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

pub fn get_quest_log_completion_text(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn get_quest_log_criteria_spell(state: &mut LuaState) -> LuaResult<u32> {
    push_selected_criteria_spell(state)
}

pub fn get_criteria_spell(state: &mut LuaState) -> LuaResult<u32> {
    push_selected_criteria_spell(state)
}

fn push_selected_criteria_spell(state: &mut LuaState) -> LuaResult<u32> {
    let Some((spell_id, spell_name, spell_texture, finished)) = selected_criteria_spell(state)?
    else {
        return Ok(0);
    };
    let spell_name = create_string(state, &spell_name);
    let spell_texture = create_string(state, &spell_texture);
    state.push(Val::Num(spell_id as f64));
    state.push(spell_name);
    state.push(spell_texture);
    state.push(Val::Bool(finished));
    Ok(4)
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
/// were found on the current world map. The sim mirrors the total quest-log
/// entry count so the seeded quest-map tests stay aligned with the same
/// surface that backs `C_QuestLog.GetNumQuestLogEntries()`.
pub fn quest_map_update_all_quests(state: &mut LuaState) -> LuaResult<u32> {
    let entry_count = borrow_state(state)?.quest_log_entries.entries.len() as f64;
    state.push(Val::Num(entry_count));
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

pub fn get_reward_skill_points_nil_triplet(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(3)
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

pub fn resolve_quest_ui_widget_set_by_type(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    if !is_world_quest(quest_id) {
        return Ok(0);
    }

    let widget_type = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as i32;
    let widget_set_id = QUEST_WIDGET_SET_BASE_ID + (quest_id * 10) + widget_type;
    state.push(Val::Num(widget_set_id as f64));
    Ok(1)
}
