//! Rilua A_Admin handlers — Transmog & collections.
//!
//! Extracted from rilua_admin_extras.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in rilua_admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use crate::lua_api::rilua_methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ── Transmog & collections ────────────────────────────────────────────────────

pub(super) fn add_transmog(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?
        .world
        .collected_transmogs
        .insert(id);
    Ok(0)
}

pub(super) fn remove_transmog(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?
        .world
        .collected_transmogs
        .remove(&id);
    Ok(0)
}

pub(super) fn add_transmog_appearance(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::state_types::TransmogAppearance;
    let source_id = i32::from_stack(state, 1)?;
    let category_id = i32::from_stack(state, 2)?;
    let item_id = i32::from_stack(state, 3)?;
    let mut st = borrow_state_mut(state)?;
    let visual_id = st
        .world
        .transmog_appearances
        .iter()
        .map(|a| a.visual_id)
        .max()
        .unwrap_or(0)
        + 1;
    st.world.transmog_appearances.push(TransmogAppearance {
        source_id,
        visual_id,
        category_id,
        item_id,
        is_collected: true,
        source_type: 0,
        item_mod_id: 0,
    });
    Ok(0)
}

pub(super) fn set_transmog_for_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot_id = i32::from_stack(state, 1)?;
    let source_id = i32::from_stack(state, 2)?;
    borrow_state_mut(state)?
        .world
        .applied_transmog_slots
        .insert(slot_id, source_id);
    Ok(0)
}

pub(super) fn collect_heirloom(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?
        .world
        .collected_heirlooms
        .insert(item_id as u32);
    Ok(0)
}

pub(super) fn uncollect_heirloom(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?
        .world
        .collected_heirlooms
        .remove(&(item_id as u32));
    Ok(0)
}

pub(super) fn set_mount_collected(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let collected = bool::from_stack(state, 2)?;
    let mut st = borrow_state_mut(state)?;
    if collected {
        st.world.collected_mounts.insert(id);
    } else {
        st.world.collected_mounts.remove(&id);
    }
    Ok(0)
}

pub(super) fn set_pet_collected(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let collected = bool::from_stack(state, 2)?;
    let mut st = borrow_state_mut(state)?;
    if collected {
        st.world.collected_pets.insert(id);
    } else {
        st.world.collected_pets.remove(&id);
    }
    Ok(0)
}

pub(super) fn set_toy_collected(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let collected = bool::from_stack(state, 2)?;
    let mut st = borrow_state_mut(state)?;
    if collected {
        st.world.collected_toys.insert(id);
    } else {
        st.world.collected_toys.remove(&id);
    }
    Ok(0)
}

pub(super) fn set_achievement_earned(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let collected = bool::from_stack(state, 2)?;
    let mut st = borrow_state_mut(state)?;
    if collected {
        st.world.earned_achievements.insert(id);
    } else {
        st.world.earned_achievements.remove(&id);
    }
    Ok(0)
}

pub(super) fn has_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let result = borrow_state(state)?.world.earned_achievements.contains(&id);
    state.push(Val::Bool(result));
    Ok(1)
}

pub(super) fn earn_achievement(state: &mut LuaState) -> LuaResult<u32> {
    use crate::event::{Event, EventArg};
    let id = i32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.world.earned_achievements.insert(id);
    st.events.push(Event {
        name: "ACHIEVEMENT_EARNED".to_string(),
        args: vec![EventArg::Number(id as f64)],
    });
    Ok(0)
}

pub(super) fn collect_mount(state: &mut LuaState) -> LuaResult<u32> {
    let mount_id = u32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.world.collected_mounts.insert(mount_id as i32);
    if let Some(m) = st.world.mounts.iter_mut().find(|m| m.mount_id == mount_id) {
        m.is_collected = true;
        m.is_usable = true;
    }
    Ok(0)
}

pub(super) fn uncollect_mount(state: &mut LuaState) -> LuaResult<u32> {
    let mount_id = u32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.world.collected_mounts.remove(&(mount_id as i32));
    if let Some(m) = st.world.mounts.iter_mut().find(|m| m.mount_id == mount_id) {
        m.is_collected = false;
        m.is_usable = false;
    }
    Ok(0)
}

pub(super) fn collect_pet(state: &mut LuaState) -> LuaResult<u32> {
    let species_id = u32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.world.collected_pets.insert(species_id as i32);
    if let Some(p) = st
        .world
        .pets
        .iter_mut()
        .find(|p| p.species_id == species_id)
    {
        p.is_collected = true;
    }
    Ok(0)
}

pub(super) fn uncollect_pet(state: &mut LuaState) -> LuaResult<u32> {
    let species_id = u32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.world.collected_pets.remove(&(species_id as i32));
    if let Some(p) = st
        .world
        .pets
        .iter_mut()
        .find(|p| p.species_id == species_id)
    {
        p.is_collected = false;
    }
    Ok(0)
}

pub(super) fn collect_toy(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.world.collected_toys.insert(item_id as i32);
    if let Some(toy) = st.world.toys.iter_mut().find(|t| t.item_id == item_id) {
        toy.is_collected = true;
        toy.is_usable = true;
    }
    Ok(0)
}

pub(super) fn uncollect_toy(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.world.collected_toys.remove(&(item_id as i32));
    if let Some(toy) = st.world.toys.iter_mut().find(|t| t.item_id == item_id) {
        toy.is_collected = false;
        toy.is_usable = false;
    }
    Ok(0)
}
