//! Small C_* namespace registrations that were previously flat stubs.
//!
//! Migrates 9 entries off the namespace stub tables:
//!
//! - `C_TrophyHall.GetTrophyInfo()` — nil (no trophy data in simulator)
//! - `C_StableInfo.IsAtPetStable()` — reads `SimState.pet_stables_open`
//! - `C_GarrisonInfo.HasGarrison()` — false (garrison not simulated)
//! - `C_GarrisonInfo.GetGarrisonType()` — 0 (no garrison type)
//! - `C_AssistedCombat.*` — empty rotation/action spell state by default
//! - `C_Map.IsMapValidForNavigation(uiMapID)` — false (no nav mesh)
//! - `C_PvP.IsMatchConsideredArena()` — false (not in arena by default)
//! - `C_LossOfControl.GetActiveLossOfControlData(index)` — nil
//! - `C_LossOfControl.GetActiveLossOfControlDataCount()` — 0
//! - `C_Bank.HasFullBankAccess()` — true (permissive default)

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string, create_table};
use crate::lua_bridge::table_set_rust_fn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_small_namespaces(state: &mut LuaState) -> LuaResult<()> {
    // C_TrophyHall
    let trophy_hall = ensure_namespace(state, "C_TrophyHall")?;
    table_set_rust_fn(
        state,
        trophy_hall,
        "GetTrophyInfo",
        c_trophy_hall_get_trophy_info,
    )?;

    // C_StableInfo
    let stable_info = ensure_namespace(state, "C_StableInfo")?;
    table_set_rust_fn(
        state,
        stable_info,
        "IsAtPetStable",
        c_stable_info_is_at_pet_stable,
    )?;

    // C_GarrisonInfo
    let garrison_info = ensure_namespace(state, "C_GarrisonInfo")?;
    table_set_rust_fn(
        state,
        garrison_info,
        "HasGarrison",
        c_garrison_info_has_garrison,
    )?;
    table_set_rust_fn(
        state,
        garrison_info,
        "GetGarrisonType",
        c_garrison_info_get_garrison_type,
    )?;

    // C_AssistedCombat
    let assisted_combat = ensure_namespace(state, "C_AssistedCombat")?;
    table_set_rust_fn(
        state,
        assisted_combat,
        "GetActionSpell",
        c_assisted_combat_get_action_spell,
    )?;
    table_set_rust_fn(
        state,
        assisted_combat,
        "GetNextCastSpell",
        c_assisted_combat_get_next_cast_spell,
    )?;
    table_set_rust_fn(
        state,
        assisted_combat,
        "GetRotationSpells",
        c_assisted_combat_get_rotation_spells,
    )?;
    table_set_rust_fn(
        state,
        assisted_combat,
        "IsAvailable",
        c_assisted_combat_is_available,
    )?;

    // C_Map
    let c_map = ensure_namespace(state, "C_Map")?;
    table_set_rust_fn(
        state,
        c_map,
        "IsMapValidForNavigation",
        c_map_is_map_valid_for_navigation,
    )?;

    // C_PvP
    let c_pvp = ensure_namespace(state, "C_PvP")?;
    table_set_rust_fn(
        state,
        c_pvp,
        "IsMatchConsideredArena",
        c_pvp_is_match_considered_arena,
    )?;
    table_set_rust_fn(
        state,
        c_pvp,
        "GetPvpTalentsUnlockedLevel",
        c_pvp_get_pvp_talents_unlocked_level,
    )?;
    table_set_rust_fn(
        state,
        c_pvp,
        "GetWarModeRewardBonusDefault",
        c_pvp_get_war_mode_reward_bonus_default,
    )?;
    table_set_rust_fn(
        state,
        c_pvp,
        "GetWarModeRewardBonus",
        c_pvp_get_war_mode_reward_bonus,
    )?;

    // C_LossOfControl
    let loc = ensure_namespace(state, "C_LossOfControl")?;
    table_set_rust_fn(
        state,
        loc,
        "GetActiveLossOfControlData",
        c_loc_get_active_loss_of_control_data,
    )?;
    table_set_rust_fn(
        state,
        loc,
        "GetActiveLossOfControlDataCount",
        c_loc_get_active_loss_of_control_data_count,
    )?;

    // C_Bank
    let c_bank = ensure_namespace(state, "C_Bank")?;
    table_set_rust_fn(
        state,
        c_bank,
        "HasFullBankAccess",
        c_bank_has_full_bank_access,
    )?;

    Ok(())
}

fn c_trophy_hall_get_trophy_info(_state: &mut LuaState) -> LuaResult<u32> {
    // No trophy-hall data in the simulator.
    Ok(0)
}

fn c_stable_info_is_at_pet_stable(state: &mut LuaState) -> LuaResult<u32> {
    let open = borrow_state(state)?.pet_stables_open;
    state.push(Val::Bool(open));
    Ok(1)
}

fn c_garrison_info_has_garrison(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_garrison_info_get_garrison_type(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_assisted_combat_get_action_spell(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn c_assisted_combat_get_next_cast_spell(state: &mut LuaState) -> LuaResult<u32> {
    let _ = state;
    state.push(Val::Nil);
    Ok(1)
}

fn c_assisted_combat_get_rotation_spells(state: &mut LuaState) -> LuaResult<u32> {
    let spells = create_table(state);
    state.push(spells);
    Ok(1)
}

fn c_assisted_combat_is_available(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    let reason = create_string(state, "Not available");
    state.push(reason);
    Ok(2)
}

fn c_map_is_map_valid_for_navigation(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_is_match_considered_arena(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_get_pvp_talents_unlocked_level(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(20.0));
    Ok(1)
}

fn c_pvp_get_war_mode_reward_bonus_default(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(10.0));
    Ok(1)
}

fn c_pvp_get_war_mode_reward_bonus(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(10.0));
    Ok(1)
}

fn c_loc_get_active_loss_of_control_data(_state: &mut LuaState) -> LuaResult<u32> {
    // No loss-of-control events in the simulator.
    Ok(0)
}

fn c_loc_get_active_loss_of_control_data_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_bank_has_full_bank_access(state: &mut LuaState) -> LuaResult<u32> {
    // Permissive default — simulator grants full bank access.
    state.push(Val::Bool(true));
    Ok(1)
}
