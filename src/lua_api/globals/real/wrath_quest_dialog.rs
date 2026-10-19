//! Wrath dialog requests and explicit simulator offer/acceptance responses.
use crate::c_api::quest_readiness::QuestDialogPhase;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_api::state::{QuestLogEntry, SimState};
use crate::lua_bridge::FromStack;
use rilua::{LuaApiMut, LuaResult, Val, vm::state::LuaState};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    for (name, handler) in [
        (
            "SelectActiveQuest",
            select_active_quest as fn(&mut LuaState) -> LuaResult<u32>,
        ),
        ("SelectGossipActiveQuest", select_active_quest),
        ("IsQuestCompletable", is_quest_completable),
        ("CompleteQuest", complete_quest),
        ("GetQuestReward", get_quest_reward),
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
    let index = Option::<usize>::from_stack(state, 1)?
        .unwrap_or(0)
        .wrapping_sub(1);
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

fn complete_quest(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    let dialog = &mut sim.quest_log_entries.readiness;
    dialog.reward_requested = dialog.dialog.map(|(id, _)| id);
    dialog.response_sent = true;
    Ok(0)
}

fn get_quest_reward(state: &mut LuaState) -> LuaResult<u32> {
    let count = read_current_entry(state)?.map_or(0, |row| row.reward_choices.len());
    // Step A handles ordinary integer labels only; native conversion/validation is step B.
    let choice = if count == 0 {
        0
    } else {
        Option::<i32>::from_stack(state, 1)?.unwrap_or(0) as usize
    };
    let mut sim = borrow_state_mut(state)?;
    let dialog = &mut sim.quest_log_entries.readiness;
    dialog.pending_reward_choice = dialog.dialog.map(|(id, _)| (id, choice));
    dialog.response_sent = true;
    Ok(0)
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
    state.push(Val::Bool(accepted));
    Ok(1)
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
    let index = usize::from_stack(state, 2)?.wrapping_sub(1);
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
