//! Player combo points: Retail owns the pool; Wrath/Forever assign it to a target.

use crate::client_profile::{ACTIVE, ClientProfile};

use crate::lua_api::globals::targeting_verbs::resolve_unit_snapshot;
use crate::lua_api::globals::unit_misc::guid_for_unit;
use crate::lua_api::methods::borrow_state;
use crate::lua_api::state::SimState;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

pub(crate) const COMBO_POINTS_POWER_TYPE: i32 = 4;

fn uses_target_bound_pool() -> bool {
    matches!(ACTIVE, ClientProfile::Wrath | ClientProfile::WowForever)
}

/// Target-bound simulator policy: power input assigns the pool to the selected target.
/// Selection changes alone leave UnitPower's snapshot and this assignment untouched.
pub(crate) fn bind_player_combo_points(sim: &mut SimState) {
    if !uses_target_bound_pool() {
        return;
    }
    let has_points = sim
        .player
        .secondary_powers
        .get(&COMBO_POINTS_POWER_TYPE)
        .is_some_and(|pool| pool.current != 0);
    sim.player.combo_points_target_guid = if has_points {
        sim.current_target
            .as_ref()
            .map(|target| target.guid.clone())
    } else {
        None
    };
}

fn read_player_combo_points(sim: &SimState, unit: &str, target: &str) -> LuaResult<i32> {
    let Some(owner) = resolve_unit_snapshot(sim, unit) else {
        return Ok(0);
    };
    let target = if uses_target_bound_pool() {
        let Some(target) = resolve_unit_snapshot(sim, target) else {
            return Ok(0);
        };
        Some(target)
    } else {
        None
    };
    if owner.guid != guid_for_unit(sim, "player") {
        if ACTIVE == ClientProfile::Wrath {
            // No supplied nonplayer combo pool; Wrath's empty-state policy is zero.
            return Ok(0);
        }
        return Err(runtime_error(
            "GetComboPoints currently supports the modeled player owner only",
        ));
    }
    if let Some(target) = target
        && sim.player.combo_points_target_guid.as_deref() != Some(target.guid.as_str())
    {
        return Ok(0);
    }
    Ok(sim
        .player
        .secondary_powers
        .get(&COMBO_POINTS_POWER_TYPE)
        .map_or(0, |pool| pool.current))
}

fn get_combo_points(state: &mut LuaState) -> LuaResult<u32> {
    let unit = String::from_stack(state, 1)?;
    let target = String::from_stack(state, 2)?;
    let sim = borrow_state(state)?;
    let points = read_player_combo_points(&sim, &unit, &target)?;
    drop(sim);
    state.push(Val::Num(points as f64));
    Ok(1)
}

pub(crate) fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    if matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::WowForever | ClientProfile::Wrath
    ) {
        LuaApiMut::register_function(lua, "GetComboPoints", get_combo_points)?;
    }
    Ok(())
}
