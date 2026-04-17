//! `C_AchievementInfo` probe surface backed by `SimState.achievements`
//! and `WorldState.earned_achievements`.
//!
//! Migrates 3 entries off the namespace stub tables:
//!
//! - `C_AchievementInfo.GetAchievementInfo(achievementID)` — returns
//!   the 15-value retail multiret for a seeded row (id, name, points,
//!   completed, month, day, year, description, flags, icon,
//!   rewardText, isGuild, wasEarnedByMe, earnedBy, isStatistic).
//!   `completed` / `wasEarnedByMe` are derived from
//!   `WorldState.earned_achievements` at read time.
//! - `C_AchievementInfo.GetRewardItemID(achievementID)` — returns the
//!   `reward_item_id` or nil when no reward is seeded.
//! - `C_AchievementInfo.IsValidAchievement(achievementID)` — true when
//!   the id is present in the seeded map, false otherwise.

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_api::state::AchievementInfo;
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_achievement_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_AchievementInfo")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetAchievementInfo",
        c_achievement_info_get_achievement_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetRewardItemID",
        c_achievement_info_get_reward_item_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "IsValidAchievement",
        c_achievement_info_is_valid_achievement,
    )?;
    Ok(())
}

fn c_achievement_info_get_achievement_info(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    let row = {
        let sim = borrow_state(state)?;
        let Some(info) = sim.achievements.get(&achievement_id).cloned() else {
            return Ok(0);
        };
        let completed = sim.world.earned_achievements.contains(&achievement_id);
        (info, completed)
    };
    push_achievement_multiret(state, &row.0, row.1);
    Ok(15)
}

fn c_achievement_info_get_reward_item_id(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    let reward = borrow_state(state)?
        .achievements
        .get(&achievement_id)
        .and_then(|a| a.reward_item_id);
    match reward {
        Some(id) => state.push(Val::Num(id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_achievement_info_is_valid_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    let valid = borrow_state(state)?
        .achievements
        .contains_key(&achievement_id);
    state.push(Val::Bool(valid));
    Ok(1)
}

fn push_achievement_multiret(state: &mut LuaState, info: &AchievementInfo, completed: bool) {
    let name = create_string(state, &info.name);
    let description = create_string(state, &info.description);
    let reward_text = create_string(state, &info.reward_text);
    let earned_by = create_string(state, if completed { "player" } else { "" });

    state.push(Val::Num(info.achievement_id as f64));
    state.push(name);
    state.push(Val::Num(info.points as f64));
    state.push(Val::Bool(completed));
    state.push(Val::Num(0.0)); // month
    state.push(Val::Num(0.0)); // day
    state.push(Val::Num(0.0)); // year
    state.push(description);
    state.push(Val::Num(info.flags as f64));
    state.push(Val::Num(info.icon as f64));
    state.push(reward_text);
    state.push(Val::Bool(info.is_guild));
    state.push(Val::Bool(completed));
    state.push(earned_by);
    state.push(Val::Bool(info.is_statistic));
}
