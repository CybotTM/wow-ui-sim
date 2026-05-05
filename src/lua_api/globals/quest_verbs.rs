//! Quest verbs that mutate `SimState.quest_log` + related per-session
//! quest / achievement tracking fields and dispatch the standard WoW
//! quest events.
//!
//! Migrates 8 entries off `GLOBAL_NIL_STUBS`:
//!
//! - `ConfirmAcceptQuest()`               — consumes `pending_quest_offer`,
//!                                            appends to `quest_log`,
//!                                            fires `QUEST_ACCEPTED`.
//! - `CloseQuestFrame()`                  — fires `QUEST_FINISHED`.
//! - `AcknowledgeAutoQuestPopUp(id)`      — silent dismiss. Accepts any arg.
//! - `QuestChoiceFrame_SetActiveChoice(id)` — write `quest_choice_id`.
//! - `QuestMapLogTitleButton_OnClick(id)` — write `selected_quest_log_id`.
//! - `SetAbandonQuest()`                  — snapshot the currently selected
//!                                            quest into `abandon_quest_id`.
//! - `SetTrackedAchievement(id, tracked)` — add/remove achievement id from
//!                                            `tracked_achievements`.
//! - `UntrackAchievement(id)`             — remove id (no-op when absent).
//!
//! `QUEST_REMOVED` is not fired by these verbs directly — the real WoW
//! flow would emit it from `AbandonQuest()` (not in this port). The sim
//! emits `QUEST_REMOVED` from `SetAbandonQuest` so tests can observe the
//! intent without a second confirm step; the admin API can follow up.
//!
//! Registered from `register_tail_globals` after `missing_surface`.

use crate::event::Event;
use crate::lua_api::frame::methods::methods_hierarchy::reparent_widget;
use crate::lua_api::methods::{borrow_state_mut, extract_frame_id};
use crate::lua_api::state::QuestPortraitState;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};
use std::collections::HashSet;

fn push_event(state: &mut LuaState, name: &str) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: name.to_string(),
        args: Vec::new(),
    });
    Ok(())
}

fn stack_u32(state: &mut LuaState, index: i32) -> Option<u32> {
    match stack_val(state, index) {
        Val::Num(n) if n >= 0.0 => Some(n as u32),
        _ => None,
    }
}

fn stack_i32(state: &mut LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn stack_bool(state: &mut LuaState, index: i32) -> Option<bool> {
    match stack_val(state, index) {
        Val::Bool(b) => Some(b),
        Val::Nil => None,
        Val::Num(n) => Some(n != 0.0),
        _ => None,
    }
}

/// `ConfirmAcceptQuest()` — consume `pending_quest_offer`, append to log,
/// fire QUEST_ACCEPTED. Silent no-op when no offer is pending.
fn confirm_accept_quest(state: &mut LuaState) -> LuaResult<u32> {
    let accepted = {
        let mut st = borrow_state_mut(state)?;
        let Some(id) = st.pending_quest_offer.take() else {
            return Ok(0);
        };
        append_quest_log_entry(&mut st.quest_log, id);
        true
    };
    if accepted {
        push_event(state, "QUEST_ACCEPTED")?;
    }
    Ok(0)
}

fn append_quest_log_entry(quest_log: &mut Vec<u32>, quest_id: u32) {
    let mut known_quests = quest_log.iter().copied().collect::<HashSet<_>>();
    if known_quests.insert(quest_id) {
        quest_log.push(quest_id);
    }
}

/// `CloseQuestFrame()` — fires `QUEST_FINISHED`. WoW also nils the pending
/// offer; mirror that here.
fn close_quest_frame(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.pending_quest_offer = None;
    push_event(state, "QUEST_FINISHED")?;
    Ok(0)
}

/// `AcknowledgeAutoQuestPopUp(id)` — silent dismiss. The id is accepted
/// but not recorded; retail just clears the popup.
fn acknowledge_auto_quest_popup(state: &mut LuaState) -> LuaResult<u32> {
    let _ = stack_u32(state, 1);
    Ok(0)
}

/// `QuestChoiceFrame_SetActiveChoice(id)` — write `quest_choice_id`.
fn quest_choice_set_active(state: &mut LuaState) -> LuaResult<u32> {
    let id = stack_u32(state, 1);
    borrow_state_mut(state)?.quest_choice_id = id;
    Ok(0)
}

/// `QuestMapLogTitleButton_OnClick(questId)` — write `selected_quest_log_id`.
fn quest_map_log_title_button_on_click(state: &mut LuaState) -> LuaResult<u32> {
    let id = stack_u32(state, 1);
    borrow_state_mut(state)?.selected_quest_log_id = id;
    Ok(0)
}

/// `SetAbandonQuest()` — snapshot the currently selected quest as the
/// abandon target and fire `QUEST_REMOVED` to signal intent. The sim does
/// not run a two-step confirm (`AbandonQuest` in retail); the mark is the
/// full commitment.
fn set_abandon_quest(state: &mut LuaState) -> LuaResult<u32> {
    let (removed, marked_id) = {
        let mut st = borrow_state_mut(state)?;
        let Some(id) = st.selected_quest_log_id else {
            return Ok(0);
        };
        let before = st.quest_log.len();
        st.quest_log.retain(|q| *q != id);
        let removed = st.quest_log.len() != before;
        st.abandon_quest_id = Some(id);
        if id == st.pending_quest_offer.unwrap_or(0) {
            st.pending_quest_offer = None;
        }
        (removed, id)
    };
    if removed {
        push_event(state, "QUEST_REMOVED")?;
    }
    let _ = marked_id; // kept for future admin hooks
    Ok(0)
}

/// `SetTrackedAchievement(id, tracked)` — add or remove by id.
fn set_tracked_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let Some(id) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let tracked = stack_bool(state, 2).unwrap_or(true);
    let mut st = borrow_state_mut(state)?;
    if tracked {
        st.tracked_achievements.insert(id);
    } else {
        st.tracked_achievements.remove(&id);
    }
    Ok(0)
}

/// `UntrackAchievement(id)` — remove from `tracked_achievements`.
fn untrack_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let Some(id) = stack_i32(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?.tracked_achievements.remove(&id);
    Ok(0)
}

fn stack_string(state: &mut LuaState, index: i32) -> String {
    let Val::Str(s) = stack_val(state, index) else {
        return String::new();
    };
    state
        .gc
        .string_arena
        .get(s)
        .and_then(|lua_str| std::str::from_utf8(lua_str.data()).ok())
        .map(str::to_owned)
        .unwrap_or_default()
}

fn stack_f64(state: &mut LuaState, index: i32) -> f64 {
    match stack_val(state, index) {
        Val::Num(n) => n,
        _ => 0.0,
    }
}

/// `QuestFrame_ShowQuestPortrait(parent, portraitDisplayID,
/// mountPortraitDisplayID, modelSceneID, text, name, x, y, hideModel)`
/// — records the request on `SimState.quest_portrait_state` and
/// reparents the global `QuestModelScene` frame to `parent` so the
/// dialog visually owns the portrait. Tests probe state directly; the
/// 3D model itself is intentionally not rendered.
fn quest_frame_show_quest_portrait(state: &mut LuaState) -> LuaResult<u32> {
    let parent_val = stack_val(state, 1);
    let parent_id = extract_frame_id(state, parent_val);
    let portrait_display_id = stack_i32(state, 2).unwrap_or(0);
    let mount_portrait_display_id = stack_i32(state, 3).unwrap_or(0);
    let model_scene_id = stack_i32(state, 4).unwrap_or(0);
    let text = stack_string(state, 5);
    let name = stack_string(state, 6);
    let x = stack_f64(state, 7);
    let y = stack_f64(state, 8);
    let hide_model = stack_bool(state, 9).unwrap_or(false);

    let mut sim = borrow_state_mut(state)?;
    let model_scene_id_u64 = sim.widgets.get_id_by_name("QuestModelScene");
    sim.quest_portrait_state = Some(QuestPortraitState {
        parent_frame_id: parent_id,
        portrait_display_id,
        mount_portrait_display_id,
        model_scene_id,
        text,
        name,
        x,
        y,
        hide_model,
    });
    if let Some(scene_id) = model_scene_id_u64 {
        reparent_widget(&mut sim.widgets, scene_id, parent_id);
    }
    Ok(0)
}

/// `QuestFrame_HideQuestPortrait()` — clears
/// `SimState.quest_portrait_state` and detaches the global
/// `QuestModelScene` from its parent (matching Blizzard's
/// `SetParent(nil)` step).
fn quest_frame_hide_quest_portrait(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    sim.quest_portrait_state = None;
    if let Some(scene_id) = sim.widgets.get_id_by_name("QuestModelScene") {
        reparent_widget(&mut sim.widgets, scene_id, None);
    }
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    register_quest_state_verbs(lua)?;
    register_quest_portrait_verbs(lua)?;
    Ok(())
}

fn register_quest_state_verbs(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "ConfirmAcceptQuest", confirm_accept_quest)?;
    LuaApiMut::register_function(lua, "CloseQuestFrame", close_quest_frame)?;
    LuaApiMut::register_function(
        lua,
        "AcknowledgeAutoQuestPopUp",
        acknowledge_auto_quest_popup,
    )?;
    LuaApiMut::register_function(
        lua,
        "QuestChoiceFrame_SetActiveChoice",
        quest_choice_set_active,
    )?;
    LuaApiMut::register_function(
        lua,
        "QuestMapLogTitleButton_OnClick",
        quest_map_log_title_button_on_click,
    )?;
    LuaApiMut::register_function(lua, "SetAbandonQuest", set_abandon_quest)?;
    LuaApiMut::register_function(lua, "SetTrackedAchievement", set_tracked_achievement)?;
    LuaApiMut::register_function(lua, "UntrackAchievement", untrack_achievement)?;
    Ok(())
}

fn register_quest_portrait_verbs(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(
        lua,
        "QuestFrame_ShowQuestPortrait",
        quest_frame_show_quest_portrait,
    )?;
    LuaApiMut::register_function(
        lua,
        "QuestFrame_HideQuestPortrait",
        quest_frame_hide_quest_portrait,
    )?;
    Ok(())
}
