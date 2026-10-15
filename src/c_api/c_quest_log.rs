//! Quest queries projected from explicitly supplied simulator state.

use super::helpers::{ensure_namespace, validate_supplied_relation_id};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_QuestLog")?;
    #[cfg(any(feature = "client-retail", feature = "client-ptr"))]
    table_set_rust_fn_static(
        state,
        namespace,
        "QuestCanHaveWarModeBonus",
        quest_can_have_war_mode_bonus,
    )?;
    #[cfg(feature = "client-wowforever")]
    table_set_rust_fn_static(
        state,
        namespace,
        "GetQuestDifficultyLevel",
        get_quest_difficulty_level,
    )?;
    Ok(())
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
