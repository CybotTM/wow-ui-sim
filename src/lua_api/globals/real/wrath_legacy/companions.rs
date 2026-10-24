//! Legacy companion projection from explicit host eligibility and creature metadata.
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::globals::real::wrath_legacy_state::CompanionMetadata;
use crate::lua_api::methods::{borrow_state, create_string, val_to_string};
use crate::lua_api::state::SimState;
use crate::lua_api::state_types::MountData;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

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

fn count_owned_critters(sim: &SimState) -> usize {
    let owned_species: std::collections::HashSet<i32> = sim
        .world
        .pets
        .iter()
        .filter(|pet| pet.is_collected)
        .map(|pet| pet.species_id as i32)
        .collect();
    sim.wrath_legacy
        .eligible_critters
        .intersection(&owned_species)
        .count()
}

fn get_num_companions(state: &mut LuaState) -> LuaResult<u32> {
    let kind = read_companion_kind(state)?;
    let count = {
        let sim = borrow_state(state)?;
        match kind {
            CompanionKind::Mount => selected_mounts(&sim).len(),
            CompanionKind::Critter => count_owned_critters(&sim),
        }
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn read_mount_info(sim: &SimState, index: usize) -> Option<(u32, CompanionMetadata)> {
    let mounts = selected_mounts(sim);
    let mount = mounts.get(index)?;
    let metadata = sim.wrath_legacy.companion_metadata.get(&mount.spell_id)?;
    Some((mount.spell_id, metadata.clone()))
}

fn get_companion_info(state: &mut LuaState) -> LuaResult<u32> {
    let kind = read_companion_kind(state)?;
    let index = f64::from_stack(state, 2)?;
    if index == 0.0 {
        return Err(runtime_error("Usage: GetCompanionInfo(type, index)"));
    }
    let row = {
        let sim = borrow_state(state)?;
        match kind {
            CompanionKind::Mount if index >= 1.0 => read_mount_info(&sim, index as usize - 1),
            // Critter spell/creature joins remain unsupplied, never alias species IDs.
            _ => None,
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

// Q5 supplies the selection paths, not arbitration/tie or hold-clock semantics.
// Latest-ending supplied intervals retain the existing simulator representation;
// see the spec's UNKNOWN ledger. Never apply an unclassified global GCD.
fn active_recovery(sim: &SimState, spell_id: u32, now: f64) -> (f64, f64, f64) {
    let Some(timer) = sim.spell_cooldowns.get(&spell_id) else {
        return (0.0, 0.0, 1.0);
    };
    if timer.start + timer.duration <= now {
        return (0.0, 0.0, 1.0);
    }
    let enable = if sim.wrath_legacy.cooldowns_on_hold.contains(&spell_id) {
        0.0
    } else {
        1.0
    };
    (timer.start, timer.duration, enable)
}

fn later_recovery(best: (f64, f64, f64), candidate: (f64, f64, f64)) -> (f64, f64, f64) {
    if candidate.0 + candidate.1 > best.0 + best.1 {
        candidate
    } else {
        best
    }
}

fn companion_cooldown_times(sim: &SimState, spell_id: u32, now: f64) -> (f64, f64, f64) {
    if crate::spell_lookup::get_spell(spell_id).is_none()
        || sim
            .wrath_legacy
            .cooldown_excluded_spells
            .contains(&spell_id)
    {
        return (0.0, 0.0, 1.0);
    }
    let own = active_recovery(sim, spell_id, now);
    let category = sim.spell_cooldown_categories.get(&spell_id);
    let same_category = sim
        .spell_cooldown_categories
        .iter()
        .filter(|(_, source_category)| Some(*source_category) == category)
        .map(|(source_spell, _)| active_recovery(sim, *source_spell, now))
        .fold(own, later_recovery);
    later_recovery(same_category, matching_start_recovery(sim, spell_id, now))
}

fn matching_start_recovery(sim: &SimState, spell_id: u32, now: f64) -> (f64, f64, f64) {
    let Some(category) = sim
        .wrath_legacy
        .spell_start_recovery_categories
        .get(&spell_id)
    else {
        return (0.0, 0.0, 1.0);
    };
    if sim.wrath_legacy.gcd_category != Some(*category) {
        return (0.0, 0.0, 1.0);
    }
    match sim.gcd {
        Some((start, duration)) if start + duration > now => (start, duration, 1.0),
        _ => (0.0, 0.0, 1.0),
    }
}

fn get_companion_cooldown(state: &mut LuaState) -> LuaResult<u32> {
    let kind = read_companion_kind(state)
        .map_err(|_| runtime_error("Usage: GetCompanionCooldown(type, index)"))?;
    let index = read_cooldown_index(state)?;
    let (start, duration, enable) = {
        let sim = borrow_state(state)?;
        let mount = match kind {
            CompanionKind::Mount if index >= 1.0 => {
                selected_mounts(&sim).get(index as usize - 1).copied()
            }
            // No critter-to-spell input exists yet; never alias species IDs.
            _ => None,
        };
        let Some(mount) = mount else {
            return Ok(0);
        };
        companion_cooldown_times(&sim, mount.spell_id, sim.start_time.elapsed().as_secs_f64())
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
