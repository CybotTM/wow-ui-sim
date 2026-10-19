//! QL-2 authored readiness/dialog state. No reward grants or native timing claims.
use std::collections::HashSet;

use crate::lua_api::SimState;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestDialogPhase {
    Progress,
    Rewards,
}

#[derive(Debug, Clone, Default)]
pub struct QuestReadinessState {
    pub auto_complete: HashSet<i32>,
    pub notified: HashSet<i32>,
    pub dialog: Option<(i32, QuestDialogPhase)>,
    /// Supplied Wrath active-player context; fixtures explicitly set presence.
    pub active_player_present: bool,
    /// Wrath response latch; SetState resets it, CloseQuest preserves it.
    pub response_sent: bool,
    /// Concrete supplied next quest ID; zero means no continuation.
    pub next_quest_id: i32,
    /// Supplied simulator response policy: acceptance completes a cache-hit callback inline.
    /// False does not specify when a native cache-miss callback will complete.
    pub completion_callback_cache_hit: bool,
    /// CompleteQuest request awaiting simulator-supplied offer data.
    pub reward_requested: Option<i32>,
    /// GetQuestReward request awaiting simulator acceptance, not a receipt.
    pub pending_reward_choice: Option<(i32, usize)>,
    pub popups: Vec<(i32, String)>,
}

impl QuestReadinessState {
    pub fn remove_quest(&mut self, quest_id: i32) {
        self.auto_complete.remove(&quest_id);
        self.notified.remove(&quest_id);
        self.popups.retain(|(id, _)| *id != quest_id);
        if self.dialog.is_some_and(|(id, _)| id == quest_id) {
            self.dialog = None;
        }
    }
}

pub(crate) fn is_ready(sim: &SimState, quest_id: i32) -> bool {
    sim.quest_log_entries
        .entries
        .iter()
        .any(|entry| entry.quest_id == quest_id && entry.is_complete && !entry.is_failed)
}

pub(crate) fn ready_for_turn_in(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = Option::<i32>::from_stack(state, 1)?.unwrap_or(0);
    let ready = {
        let sim = borrow_state(state)?;
        is_ready(&sim, quest_id)
    };
    state.push(Val::Bool(ready));
    Ok(1)
}

/// Called inside QL-1's mutation scope, before its first notification.
/// Returns whether an opted-in first-ready notification was committed.
pub(crate) fn commit_objective_readiness(sim: &mut SimState, quest_id: i32) -> bool {
    let complete = sim
        .quest_log_entries
        .objective_progress
        .get(&quest_id)
        .filter(|progress| !progress.objectives.is_empty())
        .map(|progress| progress.objectives.iter().all(|row| row.finished));
    if let Some(complete) = complete {
        if let Some(entry) = sim
            .quest_log_entries
            .entries
            .iter_mut()
            .find(|entry| entry.quest_id == quest_id)
        {
            entry.is_complete = complete && !entry.is_failed;
        }
    }
    sync_gossip_readiness(sim, quest_id);
    commit_autocomplete_notification(sim, quest_id)
}

pub(crate) fn sync_gossip_readiness(sim: &mut SimState, quest_id: i32) {
    let ready = is_ready(sim, quest_id);
    for row in &mut sim.gossip.active_quests {
        if i32::try_from(row.quest_id).ok() == Some(quest_id) {
            row.is_complete = Some(ready);
        }
    }
}

fn commit_autocomplete_notification(sim: &mut SimState, quest_id: i32) -> bool {
    let opted_in = sim
        .quest_log_entries
        .readiness
        .auto_complete
        .contains(&quest_id);
    if !opted_in || !is_ready(sim, quest_id) {
        return false;
    }
    sim.quest_log_entries.readiness.notified.insert(quest_id)
}

pub(crate) fn commit_supplied_readiness(
    sim: &mut SimState,
    quest_id: i32,
    complete: bool,
    auto_complete: bool,
) -> Option<bool> {
    let entry = sim
        .quest_log_entries
        .entries
        .iter_mut()
        .find(|entry| entry.quest_id == quest_id && !entry.is_failed)?;
    let was_opted_in = sim
        .quest_log_entries
        .readiness
        .auto_complete
        .contains(&quest_id);
    if entry.is_complete == complete && was_opted_in == auto_complete {
        return None;
    }
    entry.is_complete = complete;
    if auto_complete {
        sim.quest_log_entries
            .readiness
            .auto_complete
            .insert(quest_id);
    } else {
        sim.quest_log_entries
            .readiness
            .auto_complete
            .remove(&quest_id);
    }
    sync_gossip_readiness(sim, quest_id);
    Some(commit_autocomplete_notification(sim, quest_id))
}

pub(crate) fn notify_readiness(
    state: &mut LuaState,
    quest_id: i32,
    autocomplete: bool,
) -> LuaResult<()> {
    let refresh_progress = {
        let sim = borrow_state(state)?;
        sim.quest_log_entries.readiness.dialog == Some((quest_id, QuestDialogPhase::Progress))
            && sim.selected_quest_log_id == u32::try_from(quest_id).ok()
            && has_active_npc_quest(&sim, quest_id)
    };
    if refresh_progress {
        dispatch_event_now(state, "QUEST_PROGRESS", &[])?;
    }
    if autocomplete {
        dispatch_event_now(
            state,
            "QUEST_AUTOCOMPLETE",
            &[Val::Num(f64::from(quest_id))],
        )?;
    }
    Ok(())
}

pub(crate) fn select_active_dialog(state: &mut LuaState, quest_id: u32) -> LuaResult<u32> {
    let Ok(id) = i32::try_from(quest_id) else {
        return Ok(0);
    };
    let event = {
        let mut sim = borrow_state_mut(state)?;
        if !has_active_npc_quest(&sim, id) {
            return Ok(0);
        }
        let phase = if is_ready(&sim, id) {
            QuestDialogPhase::Rewards
        } else {
            QuestDialogPhase::Progress
        };
        sim.selected_quest_log_id = Some(quest_id);
        sim.quest_log_entries.readiness.dialog = Some((id, phase));
        if phase == QuestDialogPhase::Rewards {
            "QUEST_COMPLETE"
        } else {
            "QUEST_PROGRESS"
        }
    };
    dispatch_event_now(state, event, &[])?;
    Ok(0)
}

fn has_active_npc_quest(sim: &SimState, quest_id: i32) -> bool {
    sim.gossip.active
        && sim
            .gossip
            .active_quests
            .iter()
            .any(|row| i32::try_from(row.quest_id).ok() == Some(quest_id))
        && sim
            .quest_log_entries
            .entries
            .iter()
            .any(|entry| entry.quest_id == quest_id)
}

pub(crate) fn is_quest_completable(state: &mut LuaState) -> LuaResult<u32> {
    let completable = {
        let sim = borrow_state(state)?;
        sim.quest_log_entries
            .readiness
            .dialog
            .is_some_and(|(id, _)| {
                sim.selected_quest_log_id == u32::try_from(id).ok()
                    && has_active_npc_quest(&sim, id)
                    && is_ready(&sim, id)
            })
    };
    state.push(Val::Bool(completable));
    Ok(1)
}

pub(crate) fn complete_quest(state: &mut LuaState) -> LuaResult<u32> {
    {
        let mut sim = borrow_state_mut(state)?;
        let Some((id, QuestDialogPhase::Progress)) = sim.quest_log_entries.readiness.dialog else {
            return Ok(0);
        };
        let selected = sim.selected_quest_log_id == u32::try_from(id).ok();
        if !selected || !has_active_npc_quest(&sim, id) || !is_ready(&sim, id) {
            return Ok(0);
        }
        sim.quest_log_entries.readiness.dialog = Some((id, QuestDialogPhase::Rewards));
    }
    dispatch_event_now(state, "QUEST_COMPLETE", &[])?;
    Ok(0)
}
