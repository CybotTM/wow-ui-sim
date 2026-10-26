//! Legacy companion projection from explicit host eligibility and creature metadata.
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::globals::real::wrath_legacy_state::CompanionMetadata;
use crate::lua_api::methods::{borrow_state, create_string, val_to_string};
use crate::lua_api::state::SimState;
use crate::lua_api::state_types::MountData;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

mod recovery;

#[derive(Clone, Copy)]
enum CompanionKind {
    Critter,
    Mount,
}

fn read_companion_kind(state: &LuaState) -> LuaResult<CompanionKind> {
    match String::from_stack(state, 1)?.to_ascii_uppercase().as_str() {
        "MOUNT" => Ok(CompanionKind::Mount),
        "CRITTER" => Ok(CompanionKind::Critter),
        _ => Err(runtime_error("companion type must be CRITTER or MOUNT")),
    }
}

fn selected_mounts(sim: &SimState) -> Vec<&MountData> {
    let mut mounts: Vec<_> = sim
        .world
        .mounts
        .iter()
        .filter(|mount| mount.is_collected)
        .filter(|mount| {
            sim.wrath_legacy
                .eligible_mounts
                .contains(&(mount.mount_id as i32))
        })
        .collect();
    // Only resolved names have an evidenced ordering. Missing-name order is unspecified.
    mounts.sort_by_cached_key(|mount| {
        sim.wrath_legacy
            .companion_metadata
            .get(&mount.spell_id)
            .and_then(|row| row.name.as_ref())
            .map(|name| name.to_lowercase())
    });
    mounts
}

fn selected_critters(sim: &SimState) -> Vec<i32> {
    let owned_species: std::collections::BTreeSet<i32> = sim
        .world
        .pets
        .iter()
        .filter(|pet| pet.is_collected)
        .map(|pet| pet.species_id as i32)
        .filter(|species| sim.wrath_legacy.eligible_critters.contains(species))
        .collect();
    let mut species: Vec<_> = owned_species.into_iter().collect();
    // Resolved creature-name ordering; species order breaks unresolved/equal names
    // deterministically as simulator policy, not a native tie rule.
    species.sort_by_cached_key(|id| {
        sim.wrath_legacy
            .critter_spells
            .get(id)
            .and_then(|spell| sim.wrath_legacy.companion_metadata.get(spell))
            .and_then(|row| row.name.as_ref())
            .map(|name| name.to_lowercase())
    });
    species
}

fn selected_companion_spell(sim: &SimState, kind: CompanionKind, index: usize) -> Option<u32> {
    match kind {
        CompanionKind::Mount => selected_mounts(sim).get(index).map(|mount| mount.spell_id),
        CompanionKind::Critter => selected_critters(sim)
            .get(index)
            .and_then(|species| sim.wrath_legacy.critter_spells.get(species))
            .copied(),
    }
}

fn get_num_companions(state: &mut LuaState) -> LuaResult<u32> {
    let kind = read_companion_kind(state)?;
    let count = {
        let sim = borrow_state(state)?;
        match kind {
            CompanionKind::Mount => selected_mounts(&sim).len(),
            CompanionKind::Critter => selected_critters(&sim).len(),
        }
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn read_companion_info(
    sim: &SimState,
    kind: CompanionKind,
    index: usize,
) -> Option<(u32, CompanionMetadata)> {
    let spell_id = selected_companion_spell(sim, kind, index)?;
    let metadata = sim.wrath_legacy.companion_metadata.get(&spell_id)?;
    Some((spell_id, metadata.clone()))
}

fn get_companion_info(state: &mut LuaState) -> LuaResult<u32> {
    let kind = read_companion_kind(state)?;
    let index = f64::from_stack(state, 2)?;
    if index == 0.0 {
        return Err(runtime_error("Usage: GetCompanionInfo(type, index)"));
    }
    let row = {
        let sim = borrow_state(state)?;
        if index >= 1.0 {
            read_companion_info(&sim, kind, index as usize - 1)
        } else {
            None
        }
    };
    let Some((spell_id, metadata)) = row else {
        return Ok(0);
    };
    push_companion_info(state, spell_id, &metadata);
    Ok(5)
}

fn push_companion_info(state: &mut LuaState, spell_id: u32, row: &CompanionMetadata) {
    let name = row
        .name
        .as_ref()
        .map_or(Val::Nil, |name| create_string(state, name));
    let icon = row
        .icon_path
        .as_ref()
        .map_or(Val::Nil, |path| create_string(state, path));
    state.push(Val::Num(f64::from(row.creature_id)));
    state.push(name);
    state.push(Val::Num(f64::from(spell_id)));
    state.push(icon);
    state.push(if row.active { Val::Num(1.0) } else { Val::Nil });
}

// Request-11 Q5: this wrapper queries the companion spell, not creature metadata.
fn read_cooldown_index(state: &LuaState) -> LuaResult<f64> {
    let value = stack_val(state, 2);
    let index = match value {
        Val::Num(index) => Some(index),
        Val::Str(_) => val_to_string(state, value).and_then(|s| s.trim().parse::<f64>().ok()),
        _ => None,
    };
    match index {
        Some(index) if index != 0.0 => Ok(index),
        _ => Err(runtime_error("Usage: GetCompanionCooldown(type, index)")),
    }
}

fn get_companion_cooldown(state: &mut LuaState) -> LuaResult<u32> {
    let kind = read_companion_kind(state)
        .map_err(|_| runtime_error("Usage: GetCompanionCooldown(type, index)"))?;
    let index = read_cooldown_index(state)?;
    let (start, duration, enable) = {
        let sim = borrow_state(state)?;
        if sim.wrath_legacy.companion_player_absent || index < 1.0 {
            return Ok(0);
        }
        let Some(spell_id) = selected_companion_spell(&sim, kind, index as usize - 1) else {
            return Ok(0);
        };
        recovery::companion_cooldown_times(&sim, spell_id, sim.clock_elapsed().as_millis() as u32)
    };
    state.push(Val::Num(start));
    state.push(Val::Num(duration));
    state.push(Val::Num(enable));
    Ok(3)
}

pub(super) fn register(lua: &mut rilua::Lua) -> crate::Result<()> {
    if ACTIVE != ClientProfile::Wrath {
        return Ok(());
    }
    LuaApiMut::register_function(lua, "GetNumCompanions", get_num_companions)?;
    LuaApiMut::register_function(lua, "GetCompanionInfo", get_companion_info)?;
    LuaApiMut::register_function(lua, "GetCompanionCooldown", get_companion_cooldown)?;
    Ok(())
}
