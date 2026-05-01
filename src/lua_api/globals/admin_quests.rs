//! Rilua A_Admin handlers — quest NPC / gossip dialog seeding.

use crate::lua_api::globals::quest_surface::data::{QUEST_LOG, QuestLogEntry};
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_api::sim_substates::GossipQuestRow;
use crate::lua_bridge::{FromStack, TableBuilder};
use rilua::LuaResult;
use rilua::vm::state::LuaState;

const DEFAULT_QUEST_ID: u32 = 80000;
const DEFAULT_QUEST_TITLE: &str = "The Lost Expedition";

pub(super) fn register_quests(b: TableBuilder) -> LuaResult<TableBuilder> {
    b.set_function("OpenQuestNpc", open_quest_npc)?
        .set_function("CloseQuestNpc", close_quest_npc)
}

pub(super) fn open_quest_npc(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<f64>::from_stack(state, 1)?
        .filter(|id| *id > 0.0)
        .map(|id| id as u32)
        .unwrap_or(DEFAULT_QUEST_ID);
    let title = Option::<String>::from_stack(state, 2)?
        .filter(|title| !title.is_empty())
        .unwrap_or_else(|| quest_title(quest_id));

    {
        let mut sim = borrow_state_mut(state)?;
        sim.gossip.active = true;
        sim.gossip.options.clear();
        sim.gossip.active_quests.clear();
        sim.gossip.available_quests = vec![GossipQuestRow {
            quest_id,
            quest_info_id: quest_id,
            quest_level: 80,
            title,
            is_complete: Some(false),
            ..Default::default()
        }];
        sim.gossip.num_options = sim.gossip.options.len() as i32;
        sim.gossip.num_active_quests = sim.gossip.active_quests.len() as i32;
        sim.gossip.num_available_quests = sim.gossip.available_quests.len() as i32;
    }

    dispatch_event_now(state, "GOSSIP_SHOW", &[])?;
    Ok(0)
}

pub(super) fn close_quest_npc(state: &mut LuaState) -> LuaResult<u32> {
    {
        let mut sim = borrow_state_mut(state)?;
        sim.gossip.active = false;
        sim.gossip.options.clear();
        sim.gossip.active_quests.clear();
        sim.gossip.available_quests.clear();
        sim.gossip.num_options = 0;
        sim.gossip.num_active_quests = 0;
        sim.gossip.num_available_quests = 0;
    }

    dispatch_event_now(state, "GOSSIP_CLOSED", &[])?;
    Ok(0)
}

fn quest_title(quest_id: u32) -> String {
    QUEST_LOG
        .iter()
        .find_map(|entry| match entry {
            QuestLogEntry::Quest {
                quest_id: id,
                title,
                ..
            } if *id == quest_id as i32 => Some((*title).to_string()),
            _ => None,
        })
        .unwrap_or_else(|| DEFAULT_QUEST_TITLE.to_string())
}
