//! Quest queries projected from explicitly supplied simulator state.

use super::helpers::{ensure_namespace, validate_supplied_relation_id};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[cfg(feature = "client-wowforever")]
use crate::{lua_api::sim_substates::QuestLogEntry, lua_bridge::FromStack};

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    #[cfg(any(feature = "client-retail", feature = "client-ptr"))]
    {
        let namespace = ensure_namespace(state, "C_QuestLog")?;
        table_set_rust_fn_static(
            state,
            namespace,
            "QuestCanHaveWarModeBonus",
            quest_can_have_war_mode_bonus,
        )?;
    }
    #[cfg(feature = "client-wowforever")]
    register_forever_queries(state)?;
    Ok(())
}

#[cfg(feature = "client-wowforever")]
fn register_forever_queries(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_QuestLog")?;
    let methods: &[(&str, rilua::vm::closure::RustFn)] = &[
        ("GetQuestDifficultyLevel", get_quest_difficulty_level),
        ("GetNumQuestWatches", get_num_quest_watches),
        ("GetQuestIDForQuestWatchIndex", get_quest_id_for_watch_index),
    ];
    for &(name, function) in methods {
        table_set_rust_fn_static(state, namespace, name, function)?;
    }
    let globals = state.global;
    table_set_rust_fn_static(state, globals, "GetNumQuestWatches", get_num_quest_watches)
}

#[cfg(any(feature = "client-retail", feature = "client-ptr"))]
fn quest_can_have_war_mode_bonus(state: &mut LuaState) -> LuaResult<u32> {
    let value = stack_val(state, 1);
    #[cfg(feature = "retail-12-1-0")]
    let value = rilua::table_security::unwrap_secret(state, value)?;
    let quest_id = validate_supplied_relation_id(
        value,
        "C_QuestLog.QuestCanHaveWarModeBonus: questID must be a number",
    )?;
    let eligible = {
        let sim = borrow_state(state)?;
        quest_id.is_some_and(|id| sim.quest_war_mode_eligibility.contains(&id))
    };
    state.push(Val::Bool(eligible));
    Ok(1)
}

/// Simulator policy: difficulty is the rich row's level; absent data is an error.
/// This does not establish native unknown-ID, error, or secrecy behavior.
#[cfg(feature = "client-wowforever")]
fn get_quest_difficulty_level(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = validate_supplied_relation_id(
        stack_val(state, 1),
        "C_QuestLog.GetQuestDifficultyLevel: questID must be a number",
    )?
    .ok_or_else(|| {
        rilua::runtime_error(
            "C_QuestLog.GetQuestDifficultyLevel: questID must be a finite i32 integer",
        )
    })?;
    let level = {
        let sim = borrow_state(state)?;
        sim.quest_log_entries
            .entries
            .iter()
            .find(|entry| entry.quest_id == quest_id)
            .map(|entry| entry.level)
    }
    .ok_or_else(|| {
        rilua::runtime_error(format!(
            "C_QuestLog.GetQuestDifficultyLevel: no supplied quest-log entry for questID {quest_id}"
        ))
    })?;
    state.push(Val::Num(f64::from(level)));
    Ok(1)
}

/// Existing simulator policy: all ordinary quests are watched in log order.
/// Watch-subset membership and its mutators are not modeled by this projection.
#[cfg(feature = "client-wowforever")]
fn ordinary_quest_ids(entries: &[QuestLogEntry]) -> impl Iterator<Item = i32> + '_ {
    entries
        .iter()
        .filter(|entry| !entry.is_world_quest)
        .map(|entry| entry.quest_id)
}

#[cfg(feature = "client-wowforever")]
fn get_num_quest_watches(state: &mut LuaState) -> LuaResult<u32> {
    let count = {
        let sim = borrow_state(state)?;
        ordinary_quest_ids(&sim.quest_log_entries.entries).count()
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

#[cfg(feature = "client-wowforever")]
fn get_quest_id_for_watch_index(state: &mut LuaState) -> LuaResult<u32> {
    // Preserve the existing watch index conversion, including numeric truncation.
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    if index < 1 {
        return Ok(0);
    }
    let quest_id = {
        let sim = borrow_state(state)?;
        let mut ids = ordinary_quest_ids(&sim.quest_log_entries.entries);
        ids.nth((index - 1) as usize)
    };
    let Some(quest_id) = quest_id else {
        return Ok(0);
    };
    state.push(Val::Num(f64::from(quest_id)));
    Ok(1)
}
