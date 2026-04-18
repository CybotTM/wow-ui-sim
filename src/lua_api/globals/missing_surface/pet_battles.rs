//! `C_PetBattles` probe surface backed by `SimState.pet_battles`.
//!
//! Migrates 15 entries off the namespace stub tables:
//!
//! - `GetAbilityInfoByID(abilityID)` — returns placeholder name/icon for any ID.
//! - `GetActivePet(owner)` — returns the active pet slot index for that side.
//! - `GetAllEffectiveAbilityIDs(owner, petIndex)` — returns ability-id array.
//! - `GetMaxAbilityCharges(owner, petIndex, abilityIndex)` — returns 1.
//! - `GetPetAbilityInfo(owner, petIndex, abilityIndex)` — name, icon, maxCharges.
//! - `GetPetAbilityList(owner, petIndex)` — array of ability IDs + enabled flags.
//! - `GetPetInfo(owner, petIndex)` — name, texture, selected, level, maxXP, xp, ..
//! - `GetPetInfoByPetID(petID)` — returns nil (no battle-pet journal in sim).
//! - `GetPetStats(owner, petIndex)` — health, maxHealth, power, speed, petType.
//! - `GetPlayerInfo(owner)` — returns owner index (pass-through numeric stub).
//! - `GetRoundTimingInfo()` — timeRemaining, turnTime.
//! - `GetTurnResult(owner)` — last-turn result code.
//! - `GetXP(owner, petIndex)` — xp, maxXp for the pet.
//! - `IsPlayerNPC()` — always false in the sim.
//! - `StartPVPMatchmaking()` — sets `is_matchmaking = true` on state.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, create_table};
use crate::lua_api::state::PetBattlePet;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_pet_battles_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_PetBattles")?;
    table_set_rust_fn_static(state, ns, "GetAbilityInfoByID", get_ability_info_by_id)?;
    table_set_rust_fn_static(state, ns, "GetActivePet", get_active_pet)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetAllEffectiveAbilityIDs",
        get_all_effective_ability_ids,
    )?;
    table_set_rust_fn_static(state, ns, "GetMaxAbilityCharges", get_max_ability_charges)?;
    table_set_rust_fn_static(state, ns, "GetPetAbilityInfo", get_pet_ability_info)?;
    table_set_rust_fn_static(state, ns, "GetPetAbilityList", get_pet_ability_list)?;
    table_set_rust_fn_static(state, ns, "GetPetInfo", get_pet_info)?;
    table_set_rust_fn_static(state, ns, "GetPetInfoByPetID", get_pet_info_by_pet_id)?;
    table_set_rust_fn_static(state, ns, "GetPetStats", get_pet_stats)?;
    table_set_rust_fn_static(state, ns, "GetPlayerInfo", get_player_info)?;
    table_set_rust_fn_static(state, ns, "GetRoundTimingInfo", get_round_timing_info)?;
    table_set_rust_fn_static(state, ns, "GetTurnResult", get_turn_result)?;
    table_set_rust_fn_static(state, ns, "GetXP", get_xp)?;
    table_set_rust_fn_static(state, ns, "IsPlayerNPC", is_player_npc)?;
    table_set_rust_fn_static(state, ns, "StartPVPMatchmaking", start_pvp_matchmaking)?;
    Ok(())
}

/// Resolve `(owner: i32, pet_index: i32)` from the Lua stack and return
/// the matching `PetBattlePet` clone, or `None` when out of range.
fn resolve_pet(state: &mut LuaState, owner_slot: usize, idx_slot: usize) -> Option<PetBattlePet> {
    let owner = i32::from_stack(state, owner_slot as i32).ok()?;
    let pet_index = i32::from_stack(state, idx_slot as i32).ok()?;
    let sim = borrow_state(state).ok()?;
    let pets = match owner {
        1 => &sim.pet_battles.player_pets,
        2 => &sim.pet_battles.enemy_pets,
        _ => return None,
    };
    let idx = usize::try_from(pet_index - 1).ok()?;
    pets.get(idx).cloned()
}

fn get_ability_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let ability_id = i32::from_stack(state, 1)?;
    // Return a minimal placeholder: name, icon(0), maxCharges, splitDescriptionComponents
    let name = create_string(state, &format!("Ability {ability_id}"));
    state.push(name);
    state.push(Val::Num(0.0)); // icon
    state.push(Val::Num(1.0)); // maxCharges
    // splitDescriptionComponents: empty table
    let desc = create_table(state);
    state.push(desc);
    Ok(4)
}

fn get_active_pet(state: &mut LuaState) -> LuaResult<u32> {
    let owner = i32::from_stack(state, 1)?;
    let slot = {
        let sim = borrow_state(state)?;
        match owner {
            1 => sim.pet_battles.active_pet_player,
            2 => sim.pet_battles.active_pet_enemy,
            _ => 1,
        }
    };
    state.push(Val::Num(slot as f64));
    Ok(1)
}

fn get_all_effective_ability_ids(state: &mut LuaState) -> LuaResult<u32> {
    let pet = resolve_pet(state, 1, 2);
    let ids = pet.map(|p| p.ability_ids).unwrap_or_default();
    let array = create_table(state);
    for (i, id) in ids.into_iter().enumerate() {
        set_table_array(state, array, i as i64 + 1, Val::Num(id as f64));
    }
    state.push(array);
    Ok(1)
}

fn get_max_ability_charges(_state: &mut LuaState) -> LuaResult<u32> {
    _state.push(Val::Num(1.0));
    Ok(1)
}

fn get_pet_ability_info(state: &mut LuaState) -> LuaResult<u32> {
    let pet = resolve_pet(state, 1, 2);
    let ability_index = i32::from_stack(state, 3).unwrap_or(1);
    let ability_id = pet
        .as_ref()
        .and_then(|p| {
            usize::try_from(ability_index - 1)
                .ok()
                .and_then(|i| p.ability_ids.get(i).copied())
        })
        .unwrap_or(0);
    let name = create_string(state, &format!("Ability {ability_id}"));
    state.push(name);
    state.push(Val::Num(0.0)); // icon
    state.push(Val::Num(1.0)); // maxCharges
    Ok(3)
}

fn get_pet_ability_list(state: &mut LuaState) -> LuaResult<u32> {
    let pet = resolve_pet(state, 1, 2);
    let ids = pet.map(|p| p.ability_ids).unwrap_or_default();
    let id_array = create_table(state);
    let enabled_array = create_table(state);
    for (i, id) in ids.into_iter().enumerate() {
        set_table_array(state, id_array, i as i64 + 1, Val::Num(id as f64));
        set_table_array(state, enabled_array, i as i64 + 1, Val::Bool(true));
    }
    state.push(id_array);
    state.push(enabled_array);
    Ok(2)
}

fn get_pet_info(state: &mut LuaState) -> LuaResult<u32> {
    let pet = resolve_pet(state, 1, 2);
    let Some(p) = pet else {
        state.push(Val::Nil);
        return Ok(1);
    };
    // Returns: customName, speciesName, selected, level, maxXP, xp, displayID,
    //          isFavorite, name, icon, petType, creatureID, sourceText, description, isWild
    let custom_name = create_string(state, &p.name);
    let species_name = create_string(state, &p.name);
    state.push(custom_name);
    state.push(species_name);
    state.push(Val::Bool(false)); // selected
    state.push(Val::Num(p.level as f64));
    state.push(Val::Num(p.max_xp as f64));
    state.push(Val::Num(p.xp as f64));
    state.push(Val::Num(p.species_id as f64)); // displayID
    state.push(Val::Bool(false)); // isFavorite
    let name2 = create_string(state, &p.name);
    state.push(name2);
    state.push(Val::Num(0.0)); // icon
    state.push(Val::Num(p.pet_type as f64));
    state.push(Val::Num(p.species_id as f64)); // creatureID
    state.push(Val::Nil); // sourceText
    state.push(Val::Nil); // description
    state.push(Val::Bool(false)); // isWild
    Ok(15)
}

fn get_pet_info_by_pet_id(_state: &mut LuaState) -> LuaResult<u32> {
    // No battle-pet journal integration in the sim.
    Ok(0)
}

fn get_pet_stats(state: &mut LuaState) -> LuaResult<u32> {
    let pet = resolve_pet(state, 1, 2);
    let Some(p) = pet else {
        return Ok(0);
    };
    state.push(Val::Num(p.current_health as f64));
    state.push(Val::Num(p.max_health as f64));
    state.push(Val::Num(p.power as f64));
    state.push(Val::Num(p.speed as f64));
    state.push(Val::Num(p.pet_type as f64));
    Ok(5)
}

fn get_player_info(state: &mut LuaState) -> LuaResult<u32> {
    // Returns owner index as a pass-through numeric stub.
    let owner = i32::from_stack(state, 1).unwrap_or(1);
    state.push(Val::Num(owner as f64));
    Ok(1)
}

fn get_round_timing_info(state: &mut LuaState) -> LuaResult<u32> {
    let (time_remaining, turn_time) = {
        let sim = borrow_state(state)?;
        (
            sim.pet_battles.round_time_left_ms / 1000.0,
            sim.pet_battles.round_time_ms / 1000.0,
        )
    };
    state.push(Val::Num(time_remaining));
    state.push(Val::Num(turn_time));
    Ok(2)
}

fn get_turn_result(state: &mut LuaState) -> LuaResult<u32> {
    let result = borrow_state(state)?.pet_battles.turn_result;
    state.push(Val::Num(result as f64));
    Ok(1)
}

fn get_xp(state: &mut LuaState) -> LuaResult<u32> {
    let pet = resolve_pet(state, 1, 2);
    let Some(p) = pet else {
        state.push(Val::Num(0.0));
        state.push(Val::Num(0.0));
        return Ok(2);
    };
    state.push(Val::Num(p.xp as f64));
    state.push(Val::Num(p.max_xp as f64));
    Ok(2)
}

fn is_player_npc(state: &mut LuaState) -> LuaResult<u32> {
    let _ = state;
    state.push(Val::Bool(false));
    Ok(1)
}

fn start_pvp_matchmaking(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.pet_battles.is_matchmaking = true;
    Ok(0)
}
