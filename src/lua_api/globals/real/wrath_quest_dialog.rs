//! Wrath dialog requests and explicit simulator offer/acceptance responses.
use crate::c_api::quest_readiness::QuestDialogPhase;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_api::state::{QuestLogEntry, SimState};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::{LuaApiMut, LuaResult, Val, vm::state::LuaState};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    for (name, handler) in [
        (
            "SelectActiveQuest",
            select_active_quest as fn(&mut LuaState) -> LuaResult<u32>,
        ),
        ("SelectGossipActiveQuest", select_active_quest),
        ("IsQuestCompletable", is_quest_completable),
        ("IsQuestFlaggedCompleted", is_quest_flagged_completed),
        ("CompleteQuest", request_reward_offer),
        ("GetQuestReward", send_reward_claim),
        ("QuestChooseRewardError", choose_reward_error),
        ("GetTitleText", get_title_text),
        ("GetQuestBackgroundMaterial", get_background_material),
        ("GetNumQuestChoices", get_num_choices),
        ("GetNumQuestRewards", get_num_rewards),
        ("GetQuestItemInfo", get_item_info),
        ("GetSuggestedGroupNum", get_suggested_group_num),
        ("GetRewardSpell", get_reward_spell),
        ("GetRewardArenaPoints", get_reward_arena_points),
        ("GetRewardTalents", get_reward_talents),
    ] {
        lua.register_function(name, handler)?;
    }
    Ok(())
}

fn current_entry(sim: &SimState) -> Option<&QuestLogEntry> {
    if let Some(offer) = &sim.pending_quest_offer {
        return Some(offer);
    }
    let (id, _) = sim.quest_log_entries.readiness.dialog?;
    sim.quest_log_entries
        .entries
        .iter()
        .find(|row| row.quest_id == id)
}

fn read_current_entry(state: &LuaState) -> LuaResult<Option<QuestLogEntry>> {
    Ok(current_entry(&*borrow_state(state)?).cloned())
}

fn select_active_quest(state: &mut LuaState) -> LuaResult<u32> {
    let supplied_index = Option::<i32>::from_stack(state, 1)?.unwrap_or(0);
    let index = (supplied_index as usize).wrapping_sub(1);
    let id = borrow_state(state)?
        .gossip
        .active_quests
        .get(index)
        .map(|row| row.quest_id);
    let Some(id) = id else { return Ok(0) };
    open_active_dialog(state, id as i32)?;
    Ok(0)
}

fn open_active_dialog(state: &mut LuaState, id: i32) -> LuaResult<()> {
    let phase = {
        let mut sim = borrow_state_mut(state)?;
        let phase = if crate::c_api::quest_readiness::is_ready(&sim, id) {
            QuestDialogPhase::Rewards
        } else {
            QuestDialogPhase::Progress
        };
        sim.selected_quest_log_id = Some(id as u32);
        sim.pending_quest_offer = None;
        let dialog = &mut sim.quest_log_entries.readiness;
        dialog.dialog = Some((id, phase));
        dialog.response_sent = false;
        dialog.reward_requested = None;
        dialog.pending_reward_choice = None;
        phase
    };
    let event = match phase {
        QuestDialogPhase::Progress => "QUEST_PROGRESS",
        QuestDialogPhase::Rewards => "QUEST_COMPLETE",
    };
    dispatch_event_now(state, event, &[])
}

fn is_quest_completable(state: &mut LuaState) -> LuaResult<u32> {
    let ready = read_current_entry(state)?.is_some_and(|row| row.is_complete && !row.is_failed);
    state.push(Val::Bool(ready));
    Ok(1)
}

fn is_quest_flagged_completed(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let completed = borrow_state(state)?
        .quest_log_entries
        .completed_quest_ids
        .contains(&id);
    state.push(Val::Bool(completed));
    Ok(1)
}

fn eligible_dialog_id(sim: &SimState, phase: QuestDialogPhase) -> Option<i32> {
    let dialog = &sim.quest_log_entries.readiness;
    if !dialog.active_player_present || dialog.response_sent {
        return None;
    }
    let (id, current_phase) = dialog.dialog?;
    (current_phase == phase).then_some(id)
}

fn request_reward_offer(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    let Some(id) = eligible_dialog_id(&sim, QuestDialogPhase::Progress) else {
        return Ok(0);
    };
    let dialog = &mut sim.quest_log_entries.readiness;
    dialog.reward_requested = Some(id);
    dialog.response_sent = true;
    Ok(0)
}

fn send_reward_claim(state: &mut LuaState) -> LuaResult<u32> {
    let (id, count) = {
        let sim = borrow_state(state)?;
        let Some(id) = eligible_dialog_id(&sim, QuestDialogPhase::Rewards) else {
            return Ok(0);
        };
        let count = current_entry(&sim).map_or(0, |row| row.reward_choices.len());
        (id, count)
    };
    // Eligibility must precede even invalid argument conversion/validation.
    let choice = convert_reward_choice(state, count)?;
    let mut sim = borrow_state_mut(state)?;
    let dialog = &mut sim.quest_log_entries.readiness;
    dialog.pending_reward_choice = Some((id, choice));
    dialog.response_sent = true;
    Ok(0)
}

fn choose_reward_error(state: &mut LuaState) -> LuaResult<u32> {
    crate::c_api::c_sound::request_named_sound(state, b"igQuestFailed".to_vec(), None)?;
    let key = create_string(state, "ERR_QUEST_MUST_CHOOSE");
    let globals = Val::Table(state.global);
    let message = state.gettable(globals, key)?;
    let Val::Str(reference) = message else {
        // INFERRED: unavailable/non-string localization has no display text.
        return Ok(0);
    };
    let text = state
        .gc
        .string_arena
        .get(reference)
        .ok_or_else(|| rilua::runtime_error("ERR_QUEST_MUST_CHOOSE string was collected"))?;
    let text = String::from_utf8_lossy(text.data()).into_owned();
    let nonempty = !text.is_empty();
    borrow_state_mut(state)?.last_ui_error_message = Some(text);
    if nonempty {
        dispatch_event_now(state, "UI_ERROR_MESSAGE", &[message])?;
    }
    Ok(0)
}

fn reward_argument(state: &LuaState) -> LuaResult<f64> {
    match stack_val(state, 1) {
        Val::Num(value) => Ok(value),
        Val::Str(_) => {
            let text = String::from_stack(state, 1)?;
            Ok(text.trim().parse().unwrap_or(0.0))
        }
        _ => Ok(0.0),
    }
}

fn convert_reward_choice(state: &LuaState, count: usize) -> LuaResult<usize> {
    // Ordinary finite int32-domain inputs only: exceptional numbers/overflow
    // remain unverified, with no compatibility oracle for Rust cast behavior.
    let converted = reward_argument(state)? as i32;
    let internal = i64::from(converted) - 1;
    if count == 0 {
        return Ok(internal.max(0) as usize);
    }
    if internal < 0 || internal >= count as i64 {
        return Err(rilua::runtime_error(
            "Invalid reward choice in GetQuestReward([choice])",
        ));
    }
    // Acceptance receipts use supplied one-based choice labels, not wire indices.
    Ok(converted as usize)
}

pub(crate) fn offer_response(state: &mut LuaState) -> LuaResult<u32> {
    let id = borrow_state_mut(state)?
        .quest_log_entries
        .readiness
        .reward_requested
        .take();
    let Some(id) = id else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    {
        let mut sim = borrow_state_mut(state)?;
        let dialog = &mut sim.quest_log_entries.readiness;
        dialog.dialog = Some((id, QuestDialogPhase::Rewards));
        dialog.response_sent = false;
    }
    dispatch_event_now(state, "QUEST_COMPLETE", &[])?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn accepted_choice_count(sim: &SimState, id: i32, choice: usize) -> Option<usize> {
    let row = sim
        .quest_log_entries
        .entries
        .iter()
        .find(|row| row.quest_id == id)?;
    let choices = row.reward_choices.len();
    let accepted = if choices == 0 {
        choice == 0
    } else {
        (1..=choices).contains(&choice)
    };
    accepted.then_some(choices)
}

fn commit_pending_claim(sim: &mut SimState) -> bool {
    let Some((id, choice)) = sim.quest_log_entries.readiness.pending_reward_choice.take() else {
        return false;
    };
    // Response policy, not a native send guard or inventory grant.
    let Some(choices) = accepted_choice_count(sim, id, choice) else {
        return false;
    };
    if !sim.quest_log_entries.turn_in(id) {
        return false;
    }
    if choices > 0 {
        sim.quest_log_entries.turn_in_choices.insert(id, choice);
    }
    sim.quest_log.retain(|entry| *entry != id as u32);
    crate::lua_api::globals::admin_quests::clear_turned_in_quest_dialog(sim, id);
    true
}

pub(crate) fn accept_response(state: &mut LuaState) -> LuaResult<u32> {
    let accepted = commit_pending_claim(&mut *borrow_state_mut(state)?);
    if accepted {
        crate::lua_api::globals::quest_verbs::notify_wrath_log_changed(state)?;
        finish_completed_dialog(state)?;
    }
    state.push(Val::Bool(accepted));
    Ok(1)
}

fn finish_completed_dialog(state: &mut LuaState) -> LuaResult<()> {
    {
        let sim = borrow_state(state)?;
        let dialog = &sim.quest_log_entries.readiness;
        if !dialog.completion_callback_cache_hit || dialog.next_quest_id != 0 {
            return Ok(());
        }
    }
    // The supplied inline callback and mirror notification are separate paths.
    // Their invocation here is simulator scheduling, not a native total order.
    crate::lua_api::globals::quest_verbs::finish_wrath_quest_dialog(state)
}

fn get_title_text(state: &mut LuaState) -> LuaResult<u32> {
    let title = read_current_entry(state)?.map_or_else(String::new, |row| row.title);
    let title = create_string(state, &title);
    state.push(title);
    Ok(1)
}

fn get_background_material(state: &mut LuaState) -> LuaResult<u32> {
    let material = {
        let sim = borrow_state(state)?;
        current_entry(&sim)
            .and_then(|row| {
                sim.quest_log_entries
                    .background_materials
                    .get(&row.quest_id)
            })
            .cloned()
    };
    let value = match material {
        Some(material) => create_string(state, &material),
        None => Val::Nil,
    };
    state.push(value);
    Ok(1)
}

fn get_num_choices(state: &mut LuaState) -> LuaResult<u32> {
    let count = read_current_entry(state)?.map_or(0, |row| row.reward_choices.len());
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_num_rewards(state: &mut LuaState) -> LuaResult<u32> {
    let count = read_current_entry(state)?.map_or(0, |row| row.reward_items.len());
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let kind = String::from_stack(state, 1)?;
    let supplied_index = i32::from_stack(state, 2)?;
    let index = (supplied_index as usize).wrapping_sub(1);
    let item = read_current_entry(state)?.and_then(|row| match kind.as_str() {
        "choice" => row.reward_choices.get(index).cloned(),
        "reward" => row.reward_items.get(index).cloned(),
        _ => None,
    });
    let Some(item) = item else { return Ok(0) };
    let name = create_string(state, &item.name);
    state.push(name);
    let texture = create_string(state, &item.texture);
    state.push(texture);
    state.push(Val::Num(item.count as f64));
    state.push(Val::Num(item.quality as f64));
    state.push(Val::Bool(item.is_usable));
    Ok(5)
}

fn get_suggested_group_num(state: &mut LuaState) -> LuaResult<u32> {
    let count = read_current_entry(state)?
        .and_then(|row| row.suggested_group_size)
        .unwrap_or(0);
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_reward_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell = read_current_entry(state)?.and_then(|row| row.reward_spell);
    let Some(spell) = spell else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let texture = create_string(state, &spell.texture);
    state.push(texture);
    let name = create_string(state, &spell.name);
    state.push(name);
    state.push(Val::Bool(spell.is_trade_skill));
    state.push(Val::Bool(spell.is_learned));
    Ok(4)
}

fn get_reward_arena_points(state: &mut LuaState) -> LuaResult<u32> {
    let points = read_current_entry(state)?.map_or(0, |row| row.reward_arena_points);
    state.push(Val::Num(points as f64));
    Ok(1)
}

fn get_reward_talents(state: &mut LuaState) -> LuaResult<u32> {
    let talents = read_current_entry(state)?.map_or(0, |row| row.reward_talents);
    state.push(Val::Num(talents as f64));
    Ok(1)
}
