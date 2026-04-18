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
    register_flat_namespace(state, "C_TrophyHall", C_TROPHY_HALL_METHODS)?;
    register_flat_namespace(state, "C_StableInfo", C_STABLE_INFO_METHODS)?;
    register_flat_namespace(state, "C_GarrisonInfo", C_GARRISON_INFO_METHODS)?;
    register_flat_namespace(state, "C_AssistedCombat", C_ASSISTED_COMBAT_METHODS)?;
    register_flat_namespace(state, "C_Map", C_MAP_METHODS)?;
    register_flat_namespace(state, "C_PvP", C_PVP_METHODS)?;
    register_flat_namespace(state, "C_LossOfControl", C_LOSS_OF_CONTROL_METHODS)?;
    register_flat_namespace(state, "C_Bank", C_BANK_METHODS)?;
    Ok(())
}

/// Look up (or create) the namespace table and register each
/// `(Lua name, RustFn)` pair on it. Keeps the main registrar a
/// per-namespace pipeline.
fn register_flat_namespace(
    state: &mut LuaState,
    namespace: &str,
    methods: &[(&str, rilua::vm::closure::RustFn)],
) -> LuaResult<()> {
    let ns = ensure_namespace(state, namespace)?;
    for (name, func) in methods {
        table_set_rust_fn(state, ns, name, *func)?;
    }
    Ok(())
}

const C_TROPHY_HALL_METHODS: &[(&str, rilua::vm::closure::RustFn)] =
    &[("GetTrophyInfo", c_trophy_hall_get_trophy_info)];

const C_STABLE_INFO_METHODS: &[(&str, rilua::vm::closure::RustFn)] =
    &[("IsAtPetStable", c_stable_info_is_at_pet_stable)];

const C_GARRISON_INFO_METHODS: &[(&str, rilua::vm::closure::RustFn)] = &[
    ("HasGarrison", c_garrison_info_has_garrison),
    ("GetGarrisonType", c_garrison_info_get_garrison_type),
];

const C_ASSISTED_COMBAT_METHODS: &[(&str, rilua::vm::closure::RustFn)] = &[
    ("GetActionSpell", c_assisted_combat_get_action_spell),
    ("GetNextCastSpell", c_assisted_combat_get_next_cast_spell),
    ("GetRotationSpells", c_assisted_combat_get_rotation_spells),
    ("IsAvailable", c_assisted_combat_is_available),
];

const C_MAP_METHODS: &[(&str, rilua::vm::closure::RustFn)] =
    &[("IsMapValidForNavigation", c_map_is_map_valid_for_navigation)];

const C_PVP_METHODS: &[(&str, rilua::vm::closure::RustFn)] = &[
    ("IsMatchConsideredArena", c_pvp_is_match_considered_arena),
    (
        "GetPvpTalentsUnlockedLevel",
        c_pvp_get_pvp_talents_unlocked_level,
    ),
    (
        "GetWarModeRewardBonusDefault",
        c_pvp_get_war_mode_reward_bonus_default,
    ),
    ("GetWarModeRewardBonus", c_pvp_get_war_mode_reward_bonus),
];

const C_LOSS_OF_CONTROL_METHODS: &[(&str, rilua::vm::closure::RustFn)] = &[
    (
        "GetActiveLossOfControlData",
        c_loc_get_active_loss_of_control_data,
    ),
    (
        "GetActiveLossOfControlDataCount",
        c_loc_get_active_loss_of_control_data_count,
    ),
];

const C_BANK_METHODS: &[(&str, rilua::vm::closure::RustFn)] =
    &[("HasFullBankAccess", c_bank_has_full_bank_access)];

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
