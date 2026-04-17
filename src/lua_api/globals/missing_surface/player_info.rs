//! `C_PlayerInfo` probe surface backed by `SimState.player`.
//!
//! Migrates 6 entries off the namespace stub tables:
//!
//! - `GetAlternateFormInfo()` — returns `(hasAlternateForm, inAlternateForm)` bools.
//! - `GetContentDifficultyCreatureForPlayer(unitToken)` — returns an
//!   `Enum.RelativeContentDifficulty` number (default 2 = equal).
//! - `GetPlayerMythicPlusRatingSummary(playerToken)` — returns a
//!   `MythicPlusRatingSummary` table or nothing if no data.
//! - `IsPlayerEligibleForNPE()` — returns `(isEligible, failureReason)`.
//! - `IsPlayerNPERestricted()` — returns the `is_npe_restricted` bool.
//! - `IsPlayerInRPE()` — returns the `is_in_rpe` bool.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::table_set_rust_fn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// Relative content difficulty: 2 = equal level.
const RELATIVE_CONTENT_DIFFICULTY_EQUAL: f64 = 2.0;

pub(super) fn register_player_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_PlayerInfo")?;
    table_set_rust_fn(state, ns, "GetAlternateFormInfo", get_alternate_form_info)?;
    table_set_rust_fn(
        state,
        ns,
        "GetContentDifficultyCreatureForPlayer",
        get_content_difficulty_creature,
    )?;
    table_set_rust_fn(
        state,
        ns,
        "GetPlayerMythicPlusRatingSummary",
        get_player_mythic_plus_rating_summary,
    )?;
    table_set_rust_fn(state, ns, "IsPlayerEligibleForNPE", is_player_eligible_for_npe)?;
    table_set_rust_fn(state, ns, "IsPlayerNPERestricted", is_player_npe_restricted)?;
    table_set_rust_fn(state, ns, "IsPlayerInRPE", is_player_in_rpe)?;
    Ok(())
}

fn get_alternate_form_info(state: &mut LuaState) -> LuaResult<u32> {
    let sim = borrow_state(state)?;
    let has = sim.player.is_alternate_form;
    let in_form = sim.player.alternate_form_is_default;
    drop(sim);
    state.push(Val::Bool(has));
    state.push(Val::Bool(in_form));
    Ok(2)
}

fn get_content_difficulty_creature(state: &mut LuaState) -> LuaResult<u32> {
    // Ignores unitToken — returns equal difficulty for the simulated player.
    let _ = state;
    state.push(Val::Num(RELATIVE_CONTENT_DIFFICULTY_EQUAL));
    Ok(1)
}

fn get_player_mythic_plus_rating_summary(state: &mut LuaState) -> LuaResult<u32> {
    // Ignores playerToken — always returns the local player's data.
    let summary = borrow_state(state)?.player.mythic_plus_rating_summary.clone();
    let Some(summary) = summary else {
        return Ok(0); // mayreturnnothing
    };
    let result = create_table(state);
    table_set(
        state,
        result,
        "currentSeasonScore",
        Val::Num(summary.current_season_score),
    );
    let runs_table = create_table(state);
    for (i, run) in summary.runs.into_iter().enumerate() {
        let entry = create_table(state);
        table_set(
            state,
            entry,
            "challengeModeID",
            Val::Num(run.challenge_mode_id as f64),
        );
        table_set(state, entry, "mapScore", Val::Num(run.map_score));
        table_set(
            state,
            entry,
            "bestRunLevel",
            Val::Num(run.best_run_level as f64),
        );
        table_set(
            state,
            entry,
            "bestRunDurationMS",
            Val::Num(run.best_run_duration_ms as f64),
        );
        table_set(state, entry, "finishedSuccess", Val::Bool(run.finished_success));
        set_table_array(state, runs_table, i as i64 + 1, entry);
    }
    table_set(state, result, "runs", runs_table);
    state.push(result);
    Ok(1)
}

fn is_player_eligible_for_npe(state: &mut LuaState) -> LuaResult<u32> {
    let eligible = borrow_state(state)?.player.is_npe_eligible;
    let reason = if eligible { "" } else { "level" };
    state.push(Val::Bool(eligible));
    let reason_str = create_string(state, reason);
    state.push(reason_str);
    Ok(2)
}

fn is_player_npe_restricted(state: &mut LuaState) -> LuaResult<u32> {
    let restricted = borrow_state(state)?.player.is_npe_restricted;
    state.push(Val::Bool(restricted));
    Ok(1)
}

fn is_player_in_rpe(state: &mut LuaState) -> LuaResult<u32> {
    let in_rpe = borrow_state(state)?.player.is_in_rpe;
    state.push(Val::Bool(in_rpe));
    Ok(1)
}
