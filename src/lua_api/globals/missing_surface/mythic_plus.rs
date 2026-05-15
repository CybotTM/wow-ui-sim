//! `C_MythicPlus` probe surface backed by `SimState.mythic_plus`.
//!
//! Migrates 11 entries off the namespace stub tables:
//!
//! - `GetCurrentAffixes()` — returns array of `{id, seasonID}` tables.
//! - `GetCurrentSeason()` — returns the current season id number.
//! - `GetLastWeeklyChest()` — returns nil (no chest tracked by default).
//! - `GetRunHistory(includePrev, includeIncomplete, currentSeasonOnly)` —
//!   returns array of `MythicPlusRunInfo` tables.
//! - `GetSeasonBestAffixScoreInfoForMap(mapID)` — returns affixScores
//!   array + bestOverAllScore, or nothing if no data.
//! - `GetWeeklyChestRewardLevel()` — returns four zeros when no run done.
//! - `GetOwnedKeystoneLevel()` — returns owned keystone level or nothing.
//! - `GetWeeklyBestForMap(mapChallengeModeID)` — returns
//!   durationSec, level, completionDate=nil, fraction=0, affixIDs={},
//!   members={}, or nothing if no data.
//! - `IsMythicPlusActive()` — returns the `is_active` bool.
//! - `IsWeeklyRewardAvailable()` — returns the `is_weekly_reward_available` bool.
//! - `RequestCurrentAffixes()` / `RequestMapInfo()` / `RequestRewards()` — no-ops.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_table, table_set};
use crate::lua_api::state::MythicPlusWeeklyBest;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_mythic_plus_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_MythicPlus")?;
    table_set_rust_fn_static(state, ns, "GetCurrentAffixes", get_current_affixes)?;
    table_set_rust_fn_static(state, ns, "GetCurrentSeason", get_current_season)?;
    table_set_rust_fn_static(state, ns, "GetLastWeeklyChest", get_last_weekly_chest)?;
    table_set_rust_fn_static(state, ns, "GetRunHistory", get_run_history)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetSeasonBestAffixScoreInfoForMap",
        get_season_best_affix_score_info_for_map,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetWeeklyChestRewardLevel",
        get_weekly_chest_reward_level,
    )?;
    table_set_rust_fn_static(state, ns, "GetOwnedKeystoneLevel", get_owned_keystone_level)?;
    table_set_rust_fn_static(state, ns, "GetWeeklyBestForMap", get_weekly_best_for_map)?;
    table_set_rust_fn_static(state, ns, "IsMythicPlusActive", is_mythic_plus_active)?;
    table_set_rust_fn_static(
        state,
        ns,
        "IsWeeklyRewardAvailable",
        is_weekly_reward_available,
    )?;
    table_set_rust_fn_static(state, ns, "RequestCurrentAffixes", noop)?;
    table_set_rust_fn_static(state, ns, "RequestMapInfo", noop)?;
    table_set_rust_fn_static(state, ns, "RequestRewards", noop)?;
    let challenge_mode = ensure_namespace(state, "C_ChallengeMode")?;
    table_set_rust_fn_static(state, challenge_mode, "GetMapTable", get_map_table)?;
    table_set_rust_fn_static(
        state,
        challenge_mode,
        "GetLeaverPenaltyWarningTimeLeft",
        get_leaver_penalty_warning_time_left,
    )?;
    Ok(())
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn get_map_table(state: &mut LuaState) -> LuaResult<u32> {
    let maps = create_table(state);
    state.push(maps);
    Ok(1)
}

fn get_current_affixes(state: &mut LuaState) -> LuaResult<u32> {
    let affixes = borrow_state(state)?.mythic_plus.current_affixes.clone();
    let array = create_table(state);
    for (i, affix) in affixes.into_iter().enumerate() {
        let entry = create_table(state);
        table_set(state, entry, "id", Val::Num(affix.id as f64));
        table_set(state, entry, "seasonID", Val::Num(affix.season_id as f64));
        set_table_array(state, array, i as i64 + 1, entry);
    }
    state.push(array);
    Ok(1)
}

fn get_current_season(state: &mut LuaState) -> LuaResult<u32> {
    let season = borrow_state(state)?.mythic_plus.current_season;
    state.push(Val::Num(season as f64));
    Ok(1)
}

fn get_last_weekly_chest(state: &mut LuaState) -> LuaResult<u32> {
    // No weekly chest tracked by default — return nothing (mayreturnnothing).
    let _ = state;
    Ok(0)
}

fn get_run_history(state: &mut LuaState) -> LuaResult<u32> {
    // Args: includePreviousWeeks, includeIncompleteRuns, currentSeasonOnly
    // We ignore the filter args and return all seeded runs.
    let runs = borrow_state(state)?.mythic_plus.run_history.clone();
    let array = create_table(state);
    for (i, run) in runs.into_iter().enumerate() {
        let entry = create_table(state);
        table_set(
            state,
            entry,
            "mapChallengeModeID",
            Val::Num(run.map_challenge_mode_id as f64),
        );
        table_set(state, entry, "level", Val::Num(run.level as f64));
        table_set(state, entry, "completed", Val::Bool(run.completed));
        table_set(state, entry, "season", Val::Num(run.season as f64));
        table_set(state, entry, "runScore", Val::Num(run.run_score));
        table_set(state, entry, "thisWeek", Val::Bool(run.this_week));
        table_set(
            state,
            entry,
            "durationSec",
            Val::Num(run.duration_sec as f64),
        );
        // completionDate omitted (nil by default in Lua tables)
        set_table_array(state, array, i as i64 + 1, entry);
    }
    state.push(array);
    Ok(1)
}

fn get_season_best_affix_score_info_for_map(state: &mut LuaState) -> LuaResult<u32> {
    // No season-best data seeded by default — return nothing
    // (mayreturnnothing per API spec).
    let _map_id = i32::from_stack(state, 1)?;
    Ok(0)
}

fn get_weekly_chest_reward_level(state: &mut LuaState) -> LuaResult<u32> {
    // currentWeekBestLevel, weeklyRewardLevel, nextDifficultyWeeklyRewardLevel, nextBestLevel
    // Default all zeros — no run completed this week.
    let _ = state;
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(4)
}

fn get_owned_keystone_level(state: &mut LuaState) -> LuaResult<u32> {
    let level = borrow_state(state)?.mythic_plus.owned_keystone_level;
    if level == 0 {
        // mayreturnnothing: return nothing when player has no key.
        return Ok(0);
    }
    state.push(Val::Num(level as f64));
    Ok(1)
}

fn get_weekly_best_for_map(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = i32::from_stack(state, 1)?;
    let best: Option<MythicPlusWeeklyBest> = borrow_state(state)?
        .mythic_plus
        .weekly_best_per_map
        .get(&map_id)
        .cloned();
    let Some(b) = best else {
        // mayreturnnothing: no data for this map.
        return Ok(0);
    };
    state.push(Val::Num(b.duration_sec as f64));
    state.push(Val::Num(b.level as f64));
    state.push(Val::Nil); // completionDate nilable
    state.push(Val::Num(0.0)); // fraction
    let affix_ids = create_table(state);
    state.push(affix_ids);
    let members = create_table(state);
    state.push(members);
    Ok(6)
}

fn is_mythic_plus_active(state: &mut LuaState) -> LuaResult<u32> {
    let active = borrow_state(state)?.mythic_plus.is_active;
    state.push(Val::Bool(active));
    Ok(1)
}

fn is_weekly_reward_available(state: &mut LuaState) -> LuaResult<u32> {
    let available = borrow_state(state)?.mythic_plus.is_weekly_reward_available;
    state.push(Val::Bool(available));
    Ok(1)
}

fn get_leaver_penalty_warning_time_left(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}
