//! Supplied RAF version catalog and shared summon cooldown input (seconds).
//! Empty catalogs, zero timing and disabled timers are simulator defaults, not native parity.

use super::helpers::set_table_array;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_table, table_set};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[derive(Debug, Clone, Copy, Default)]
pub struct SummonFriendCooldown {
    pub start_time_seconds: f64,
    pub duration_seconds: f64,
    pub enable_cooldown_timer: bool,
}

/// Supplied no-reward RAF version. Ordering and numeric values are caller inputs.
#[derive(Debug, Clone)]
pub struct RafVersionInfo {
    pub raf_version: i32,
    pub lifetime_months: i32,
    pub spent_months: i32,
    pub available_months: i32,
    pub num_recruits: i32,
    pub num_affordable_rewards: i32,
}

pub(crate) fn add_version_input(state: &mut LuaState) -> LuaResult<u32> {
    let version = RafVersionInfo {
        raf_version: i32::from_stack(state, 1)?,
        lifetime_months: i32::from_stack(state, 2)?,
        spent_months: i32::from_stack(state, 3)?,
        available_months: i32::from_stack(state, 4)?,
        num_recruits: i32::from_stack(state, 5)?,
        num_affordable_rewards: i32::from_stack(state, 6)?,
    };
    borrow_state_mut(state)?.raf_versions.push(version);
    Ok(0)
}

pub(crate) fn clear_version_inputs(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.raf_versions.clear();
    Ok(0)
}

pub(crate) fn get_raf_info(state: &mut LuaState) -> LuaResult<u32> {
    let versions = borrow_state(state)?.raf_versions.clone();
    let info = create_table(state);
    let rows = create_table(state);
    for (index, version) in versions.iter().enumerate() {
        let row = encode_version(state, version);
        set_table_array(state, rows, index as i64 + 1, row);
    }
    table_set(state, info, "versions", rows);
    table_set(state, info, "recruitmentInfo", Val::Nil);
    let recruits = create_table(state);
    table_set(state, info, "recruits", recruits);
    table_set(state, info, "claimInProgress", Val::Bool(false));
    state.push(info);
    Ok(1)
}

fn encode_version(state: &mut LuaState, version: &RafVersionInfo) -> Val {
    let row = create_table(state);
    for (name, value) in [
        ("rafVersion", version.raf_version),
        ("numRecruits", version.num_recruits),
        ("numAffordableRewards", version.num_affordable_rewards),
    ] {
        table_set(state, row, name, Val::Num(value as f64));
    }
    let months = encode_month_count(state, version);
    table_set(state, row, "monthCount", months);
    let rewards = create_table(state);
    table_set(state, row, "rewards", rewards);
    table_set(state, row, "nextReward", Val::Nil);
    row
}

fn encode_month_count(state: &mut LuaState, version: &RafVersionInfo) -> Val {
    let months = create_table(state);
    for (name, value) in [
        ("lifetimeMonths", version.lifetime_months),
        ("spentMonths", version.spent_months),
        ("availableMonths", version.available_months),
    ] {
        table_set(state, months, name, Val::Num(value as f64));
    }
    months
}

pub(crate) fn get_summon_friend_cooldown(state: &mut LuaState) -> LuaResult<u32> {
    let cooldown = borrow_state(state)?.summon_friend_cooldown;
    state.push(Val::Num(cooldown.start_time_seconds));
    state.push(Val::Num(cooldown.duration_seconds));
    state.push(Val::Bool(cooldown.enable_cooldown_timer));
    Ok(3)
}
