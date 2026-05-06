//! Panel-side getters and mutators driven by `state.viewed_artifact`.
//! Consumed by the LoD `Blizzard_ArtifactUI` panel addon. Mutators
//! fire `ARTIFACT_UPDATE` / `ARTIFACT_CLOSE` through the simulator
//! event queue, mirroring the live client's server round-trips.

use super::helpers::{
    build_artifact_art_info_table, build_power_info_table, create_int_sequence_table,
    push_appearance_info_tuple, push_artifact_info_tuple, push_int_multireturn,
    push_relic_info_tuple,
};
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_api::state::SimState;
use crate::lua_api::state_types::runtime::CursorInfo;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ── Panel-side core getters (read state.viewed_artifact) ─────────────

pub(super) fn get_artifact_info(state: &mut LuaState) -> LuaResult<u32> {
    let info = borrow_state(state)?.viewed_artifact.info.clone();
    let Some(info) = info else {
        return Ok(0);
    };
    push_artifact_info_tuple(state, &info);
    Ok(13)
}

pub(super) fn get_artifact_item_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = borrow_state(state)?
        .viewed_artifact
        .info
        .as_ref()
        .map(|info| info.item_id);
    let Some(id) = item_id else {
        return Ok(0);
    };
    state.push(Val::Num(id as f64));
    Ok(1)
}

/// `GetArtifactTier` returns `tier` as `Nilable = true` — `nil` when no
/// artifact is viewed, otherwise the integer tier from the info record.
pub(super) fn get_artifact_tier(state: &mut LuaState) -> LuaResult<u32> {
    let tier = borrow_state(state)?
        .viewed_artifact
        .info
        .as_ref()
        .map(|info| info.tier);
    match tier {
        Some(t) => state.push(Val::Num(t as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub(super) fn get_artifact_art_info(state: &mut LuaState) -> LuaResult<u32> {
    let viewed_present = borrow_state(state)?.viewed_artifact.info.is_some();
    if !viewed_present {
        return Ok(0);
    }
    let art = borrow_state(state)?.viewed_artifact.art_info.clone();
    let table = build_artifact_art_info_table(state, &art);
    state.push(table);
    Ok(1)
}

pub(super) fn get_points_remaining(state: &mut LuaState) -> LuaResult<u32> {
    let value = borrow_state(state)?.viewed_artifact.points_remaining;
    state.push(Val::Num(value as f64));
    Ok(1)
}

pub(super) fn get_total_purchased_ranks(state: &mut LuaState) -> LuaResult<u32> {
    let value = borrow_state(state)?.viewed_artifact.total_purchased_ranks;
    state.push(Val::Num(value as f64));
    Ok(1)
}

pub(super) fn get_num_obtained_artifacts(state: &mut LuaState) -> LuaResult<u32> {
    let value = borrow_state(state)?.viewed_artifact.num_obtained_artifacts;
    state.push(Val::Num(value as f64));
    Ok(1)
}

pub(super) fn is_artifact_disabled(state: &mut LuaState) -> LuaResult<u32> {
    let disabled = borrow_state(state)?.viewed_artifact.is_disabled;
    state.push(Val::Bool(disabled));
    Ok(1)
}

pub(super) fn is_at_forge(state: &mut LuaState) -> LuaResult<u32> {
    let at_forge = borrow_state(state)?.viewed_artifact.is_at_forge;
    state.push(Val::Bool(at_forge));
    Ok(1)
}

pub(super) fn is_maxed_by_rules_or_effect(state: &mut LuaState) -> LuaResult<u32> {
    let maxed = borrow_state(state)?.viewed_artifact.is_maxed_by_rules;
    state.push(Val::Bool(maxed));
    Ok(1)
}

pub(super) fn is_viewed_artifact_equipped(state: &mut LuaState) -> LuaResult<u32> {
    let viewed_equipped = borrow_state(state)?.viewed_artifact.is_viewed_equipped;
    state.push(Val::Bool(viewed_equipped));
    Ok(1)
}

pub(super) fn check_respec_npc(state: &mut LuaState) -> LuaResult<u32> {
    let active = borrow_state(state)?.viewed_artifact.respec_npc_active;
    state.push(Val::Bool(active));
    Ok(1)
}

// ── Panel-side power getters ─────────────────────────────────────────

pub(super) fn get_power_info(state: &mut LuaState) -> LuaResult<u32> {
    let power_id = i32::from_stack(state, 1)?;
    let power = borrow_state(state)?
        .viewed_artifact
        .powers
        .get(&power_id)
        .cloned();
    let Some(power) = power else {
        return Ok(0);
    };
    let table = build_power_info_table(state, &power);
    state.push(table);
    Ok(1)
}

pub(super) fn get_powers(state: &mut LuaState) -> LuaResult<u32> {
    let viewed_present = borrow_state(state)?.viewed_artifact.info.is_some();
    if !viewed_present {
        return Ok(0);
    }
    let mut ids: Vec<i32> = borrow_state(state)?
        .viewed_artifact
        .powers
        .keys()
        .copied()
        .collect();
    ids.sort_unstable();
    let table = create_int_sequence_table(state, &ids);
    state.push(table);
    Ok(1)
}

pub(super) fn get_power_links(state: &mut LuaState) -> LuaResult<u32> {
    let power_id = i32::from_stack(state, 1)?;
    let links = borrow_state(state)?
        .viewed_artifact
        .power_links
        .get(&power_id)
        .cloned()
        .unwrap_or_default();
    let table = create_int_sequence_table(state, &links);
    state.push(table);
    Ok(1)
}

/// `GetMetaPowerInfo` returns one stride of `(spellID, cost, currentRank)`
/// per stored meta power entry — the addon walks `select("#", ...)` in
/// strides of 3 (`Blizzard_ArtifactUI.lua:214-229`).
pub(super) fn get_meta_power_info(state: &mut LuaState) -> LuaResult<u32> {
    let entries = borrow_state(state)?.viewed_artifact.meta_powers.clone();
    for entry in &entries {
        state.push(Val::Num(entry.spell_id as f64));
        state.push(Val::Num(entry.cost as f64));
        state.push(Val::Num(entry.current_rank as f64));
    }
    Ok((entries.len() * 3) as u32)
}

pub(super) fn get_power_hyperlink(state: &mut LuaState) -> LuaResult<u32> {
    let power_id = i32::from_stack(state, 1)?;
    let link = format!("|cffff8000|Hartifactpower:{power_id}|h[Artifact Trait]|h|r");
    let val = create_string(state, &link);
    state.push(val);
    Ok(1)
}

pub(super) fn get_total_power_cost(state: &mut LuaState) -> LuaResult<u32> {
    let starting = i32::from_stack(state, 1)?;
    let count = i32::from_stack(state, 2)?;
    let tier = i32::from_stack(state, 3)?;
    let viewed_present = borrow_state(state)?.viewed_artifact.info.is_some();
    if !viewed_present {
        return Ok(0);
    }
    let cost = borrow_state(state)?
        .viewed_artifact
        .total_power_cost_table
        .get(&(starting, count, tier))
        .copied()
        .unwrap_or(0);
    state.push(Val::Num(cost as f64));
    Ok(1)
}

pub(super) fn get_powers_affected_by_relic(state: &mut LuaState) -> LuaResult<u32> {
    let slot_index = i32::from_stack(state, 1)?;
    let powers = borrow_state(state)?
        .viewed_artifact
        .powers_affected_by_relic_slot
        .get(&slot_index)
        .cloned()
        .unwrap_or_default();
    push_int_multireturn(state, &powers);
    Ok(powers.len() as u32)
}

pub(super) fn get_powers_affected_by_relic_item_link(state: &mut LuaState) -> LuaResult<u32> {
    let Ok(item_link) = String::from_stack(state, 1) else {
        return Ok(0);
    };
    let powers = borrow_state(state)?
        .viewed_artifact
        .powers_affected_by_relic_item
        .get(&item_link)
        .cloned()
        .unwrap_or_default();
    push_int_multireturn(state, &powers);
    Ok(powers.len() as u32)
}

pub(super) fn is_power_known(state: &mut LuaState) -> LuaResult<u32> {
    let power_id = i32::from_stack(state, 1)?;
    let known = {
        let sim = borrow_state(state)?;
        viewed_power_known(&sim, power_id)
    };
    state.push(Val::Bool(known));
    Ok(1)
}

fn viewed_power_known(sim: &SimState, power_id: i32) -> bool {
    sim.viewed_artifact.power_known.contains(&power_id)
}

// ── Panel-side appearance getters ────────────────────────────────────

pub(super) fn get_num_appearance_sets(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.viewed_artifact.appearance_sets.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

/// `GetAppearanceSetInfo(setIndex)` is `MayReturnNothing` — return zero
/// values when the index is out of range so the addon's `if setID and …`
/// guard at lua:140-141 takes the false branch.
pub(super) fn get_appearance_set_info(state: &mut LuaState) -> LuaResult<u32> {
    let set_index = i32::from_stack(state, 1)?;
    let set = borrow_state(state)?
        .viewed_artifact
        .appearance_sets
        .get((set_index - 1).max(0) as usize)
        .cloned();
    let Some(set) = set else {
        return Ok(0);
    };
    let name_val = create_string(state, &set.name);
    let desc_val = create_string(state, &set.description);
    state.push(Val::Num(set.set_id as f64));
    state.push(name_val);
    state.push(desc_val);
    state.push(Val::Num(set.num_appearances as f64));
    Ok(4)
}

pub(super) fn get_appearance_info(state: &mut LuaState) -> LuaResult<u32> {
    let set_index = i32::from_stack(state, 1)?;
    let appearance_index = i32::from_stack(state, 2)?;
    let appearance = borrow_state(state)?
        .viewed_artifact
        .appearances
        .get(&(set_index, appearance_index))
        .cloned();
    let Some(appearance) = appearance else {
        return Ok(0);
    };
    push_appearance_info_tuple(state, &appearance);
    Ok(13)
}

pub(super) fn get_appearance_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let appearance_id = i32::from_stack(state, 1)?;
    let appearance = borrow_state(state)?
        .viewed_artifact
        .appearances_by_id
        .get(&appearance_id)
        .cloned();
    let Some(appearance) = appearance else {
        return Ok(0);
    };
    state.push(Val::Num(appearance.set_id as f64));
    push_appearance_info_tuple(state, &appearance);
    Ok(14)
}

pub(super) fn get_preview_appearance(state: &mut LuaState) -> LuaResult<u32> {
    let preview = borrow_state(state)?.viewed_artifact.preview_appearance;
    match preview {
        Some(id) => state.push(Val::Num(id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

// ── Panel-side relic getters ─────────────────────────────────────────

pub(super) fn get_num_relic_slots(state: &mut LuaState) -> LuaResult<u32> {
    let only_unlocked = matches!(stack_val(state, 1), Val::Bool(true));
    let count = if only_unlocked {
        borrow_state(state)?
            .viewed_artifact
            .relic_slots
            .iter()
            .filter(|slot| slot.locked_reason.is_none())
            .count()
    } else {
        borrow_state(state)?.viewed_artifact.relic_slots.len()
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub(super) fn get_relic_info(state: &mut LuaState) -> LuaResult<u32> {
    let slot_index = i32::from_stack(state, 1)?;
    let slot = borrow_state(state)?
        .viewed_artifact
        .relic_slots
        .get((slot_index - 1).max(0) as usize)
        .cloned();
    let Some(slot) = slot else {
        return Ok(0);
    };
    if slot.name.is_empty() && slot.icon.is_empty() && slot.link.is_empty() {
        return Ok(0);
    }
    push_relic_info_tuple(state, &slot);
    Ok(4)
}

pub(super) fn get_relic_info_by_item_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let slot = borrow_state(state)?
        .viewed_artifact
        .relic_info_by_item_id
        .get(&item_id)
        .cloned();
    let Some(slot) = slot else {
        return Ok(0);
    };
    push_relic_info_tuple(state, &slot);
    Ok(4)
}

pub(super) fn get_relic_locked_reason(state: &mut LuaState) -> LuaResult<u32> {
    let slot_index = i32::from_stack(state, 1)?;
    let reason = borrow_state(state)?
        .viewed_artifact
        .relic_slots
        .get((slot_index - 1).max(0) as usize)
        .and_then(|slot| slot.locked_reason.clone());
    match reason {
        Some(r) => {
            let s = create_string(state, &r);
            state.push(s);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub(super) fn get_relic_slot_type(state: &mut LuaState) -> LuaResult<u32> {
    let slot_index = i32::from_stack(state, 1)?;
    let slot_type = borrow_state(state)?
        .viewed_artifact
        .relic_slots
        .get((slot_index - 1).max(0) as usize)
        .map(|slot| slot.slot_type.clone());
    let Some(slot_type) = slot_type else {
        return Ok(0);
    };
    let val = create_string(state, &slot_type);
    state.push(val);
    Ok(1)
}

/// `CanApplyCursorRelicToSlot` is true when the cursor holds a relic
/// item id registered in `state.artifact_relic_items` and the slot is
/// unlocked. Non-item cursor payloads (spell, talent, macro) and a
/// cursor with no item return false.
pub(super) fn can_apply_cursor_relic_to_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot_index = i32::from_stack(state, 1)?;
    let can_apply = {
        let sim = borrow_state(state)?;
        can_apply_cursor_relic(&sim, slot_index)
    };
    state.push(Val::Bool(can_apply));
    Ok(1)
}

pub(super) fn can_apply_relic_item_id_to_slot(state: &mut LuaState) -> LuaResult<u32> {
    let relic_item_id = i32::from_stack(state, 1)?;
    let slot_index = i32::from_stack(state, 2)?;
    let can_apply = {
        let sim = borrow_state(state)?;
        can_apply_relic_item(&sim, relic_item_id, slot_index)
    };
    state.push(Val::Bool(can_apply));
    Ok(1)
}

fn can_apply_cursor_relic(sim: &SimState, slot_index: i32) -> bool {
    cursor_item_id(&sim.cursor_item)
        .is_some_and(|item_id| can_apply_relic_item(sim, item_id, slot_index))
}

fn can_apply_relic_item(sim: &SimState, relic_item_id: i32, slot_index: i32) -> bool {
    relic_slot_unlocked(sim, slot_index) && known_artifact_relic(sim, relic_item_id)
}

fn relic_slot_unlocked(sim: &SimState, slot_index: i32) -> bool {
    let slot_pos = (slot_index - 1).max(0) as usize;
    sim.viewed_artifact
        .relic_slots
        .get(slot_pos)
        .is_some_and(|slot| slot.locked_reason.is_none())
}

fn known_artifact_relic(sim: &SimState, relic_item_id: i32) -> bool {
    sim.artifact_relic_items.contains(&relic_item_id)
}

/// Extract the cursor's carried item id when the cursor holds a
/// `CursorInfo::Item` payload. Other cursor variants (spell, talent,
/// macro) do not carry an item id.
fn cursor_item_id(cursor: &Option<CursorInfo>) -> Option<i32> {
    match cursor {
        Some(CursorInfo::Item { item_id, .. }) => Some(*item_id as i32),
        _ => None,
    }
}

// ── Panel-side forge methods ─────────────────────────────────────────

pub(super) fn get_forge_rotation(state: &mut LuaState) -> LuaResult<u32> {
    let (x, y, z) = borrow_state(state)?.viewed_artifact.forge_rotation;
    state.push(Val::Num(x as f64));
    state.push(Val::Num(y as f64));
    state.push(Val::Num(z as f64));
    Ok(3)
}

pub(super) fn should_suppress_forge_rotation(state: &mut LuaState) -> LuaResult<u32> {
    let suppress = borrow_state(state)?.viewed_artifact.suppress_forge_rotation;
    state.push(Val::Bool(suppress));
    Ok(1)
}

pub(super) fn set_forge_rotation(state: &mut LuaState) -> LuaResult<u32> {
    let x = f32::from_stack(state, 1)?;
    let y = f32::from_stack(state, 2)?;
    let z = f32::from_stack(state, 3)?;
    borrow_state_mut(state)?.viewed_artifact.forge_rotation = (x, y, z);
    Ok(0)
}

// ── Panel-side mutators ──────────────────────────────────────────────

pub(super) fn add_power(state: &mut LuaState) -> LuaResult<u32> {
    let power_id = i32::from_stack(state, 1)?;
    let success = borrow_state_mut(state)?.viewed_artifact.add_power(power_id);
    if success {
        dispatch_event_now(state, "ARTIFACT_UPDATE", &[Val::Bool(false)])?;
    }
    state.push(Val::Bool(success));
    Ok(1)
}

/// `Clear()` resets the viewed-artifact slot to `None` and fires
/// `ARTIFACT_CLOSE` — the addon's `OnHide` calls this and uses the
/// event to drop any cached panel state.
pub(super) fn clear_artifact(state: &mut LuaState) -> LuaResult<u32> {
    let was_present = {
        let mut sim = borrow_state_mut(state)?;
        let had_info = sim.viewed_artifact.info.is_some();
        sim.viewed_artifact = Default::default();
        had_info
    };
    if was_present {
        dispatch_event_now(state, "ARTIFACT_CLOSE", &[])?;
    }
    Ok(0)
}

pub(super) fn confirm_respec(state: &mut LuaState) -> LuaResult<u32> {
    let had_artifact = {
        let mut sim = borrow_state_mut(state)?;
        let had = sim.viewed_artifact.info.is_some();
        sim.viewed_artifact.confirm_respec();
        had
    };
    if had_artifact {
        dispatch_event_now(state, "ARTIFACT_UPDATE", &[Val::Bool(false)])?;
    }
    Ok(0)
}

pub(super) fn set_appearance(state: &mut LuaState) -> LuaResult<u32> {
    let appearance_id = i32::from_stack(state, 1)?;
    let had_artifact = {
        let mut sim = borrow_state_mut(state)?;
        if let Some(info) = sim.viewed_artifact.info.as_mut() {
            info.artifact_appearance_id = appearance_id;
            true
        } else {
            false
        }
    };
    if had_artifact {
        dispatch_event_now(state, "ARTIFACT_UPDATE", &[Val::Bool(false)])?;
    }
    Ok(0)
}

/// `SetPreviewAppearance(id?)` writes the preview slot. The doc says
/// "Call without an argument to clear the preview" — passing nil or
/// 0 maps to `None`.
pub(super) fn set_preview_appearance(state: &mut LuaState) -> LuaResult<u32> {
    let preview = match stack_val(state, 1) {
        Val::Num(n) if n != 0.0 => Some(n as i32),
        _ => None,
    };
    borrow_state_mut(state)?.viewed_artifact.preview_appearance = preview;
    Ok(0)
}

/// `ApplyCursorRelicToSlot(slotIndex)` writes the cursor's item into
/// the requested slot when `CanApplyCursorRelicToSlot` would have
/// returned true. No event is fired today — the live client emits
/// `ARTIFACT_UPDATE` after the server confirms; tests that need the
/// event can fire it explicitly.
pub(super) fn apply_cursor_relic_to_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot_index = i32::from_stack(state, 1)?;
    let slot_pos = (slot_index - 1).max(0) as usize;
    let mut sim = borrow_state_mut(state)?;
    let cursor_id = cursor_item_id(&sim.cursor_item);
    let Some(item_id) = cursor_id else {
        return Ok(0);
    };
    if !can_apply_relic_item(&sim, item_id, slot_index) {
        return Ok(0);
    }
    if let Some(slot) = sim.viewed_artifact.relic_slots.get_mut(slot_pos) {
        slot.link = format!("item:{item_id}");
        slot.icon = format!("Interface/Icons/inv_relic_{item_id}");
        slot.name = format!("Relic {item_id}");
    }
    Ok(0)
}
