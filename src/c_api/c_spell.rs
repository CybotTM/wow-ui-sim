//! `C_Spell` probes backed by spell data and `SimState`.
//!
//! Migrates 10 entries off the namespace stub tables:
//!
//! - `GetSpellInfo(spellID)` → `SpellInfo` table from `spells::get_spell`, or nil.
//! - `GetSpellCooldown(spellID)` → `SpellCooldownInfo` table from
//!   `SimState.spell_cooldowns` (start/duration/isEnabled/isActive/modRate).
//! - `GetSpellCastCount(spellID)` → permissive zero. The zone-ability UI
//!   only uses this as a fallback display count when charges are absent.
//! - `GetMountFromSpell(spellID)` → scans `world.mounts` for matching spell
//!   id, returns mount_id or nil.
//! - `GetVisibilityInfo(spellID, filter)` → `(false, true, false)` — most
//!   spells visible but not custom-pinned.
//! - `IsPriorityAura(spellID)` → false (permissive).
//! - `IsSelfBuff(spellID)` → true when `implicit_target == 1` (Self), else false.
//! - `IsSpellUsable(spellID)` → `(true, false)` when spell is known;
//!   `(false, false)` otherwise.
//! - `TargetSpellIsEnchanting()` → false.
//! - `TargetSpellJumpsUpgradeTrack()` → false.
//! - `TargetSpellReplacesBonusTree()` → false.

use super::helpers::ensure_namespace;
use crate::lua_api::globals::action_bar_api::spell_cooldown_times;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use crate::spells;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_spell_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Spell")?;
    table_set_rust_fn_static(state, ns, "GetSpellInfo", get_spell_info)?;
    table_set_rust_fn_static(state, ns, "GetSpellCooldown", get_spell_cooldown)?;
    table_set_rust_fn_static(state, ns, "GetSpellCastCount", get_spell_cast_count)?;
    table_set_rust_fn_static(state, ns, "GetMountFromSpell", get_mount_from_spell)?;
    table_set_rust_fn_static(state, ns, "GetVisibilityInfo", get_visibility_info)?;
    table_set_rust_fn_static(state, ns, "DoesSpellExist", does_spell_exist)?;
    table_set_rust_fn_static(state, ns, "IsSpellDataCached", is_spell_data_cached)?;
    table_set_rust_fn_static(state, ns, "IsPriorityAura", is_priority_aura)?;
    table_set_rust_fn_static(state, ns, "IsSelfBuff", is_self_buff)?;
    table_set_rust_fn_static(state, ns, "IsSpellUsable", is_spell_usable)?;
    table_set_rust_fn_static(
        state,
        ns,
        "TargetSpellIsEnchanting",
        target_spell_is_enchanting,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "TargetSpellJumpsUpgradeTrack",
        target_spell_jumps_upgrade_track,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "TargetSpellReplacesBonusTree",
        target_spell_replaces_bonus_tree,
    )?;
    Ok(())
}

/// `C_Spell.GetSpellInfo(spellID)` → `SpellInfo` table or nil.
///
/// Retail fields: `name, iconID, originalIconID, castTime, minRange, maxRange, spellID`.
fn get_spell_info(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let Some(spell) = spells::get_spell(spell_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    let name = create_string(state, spell.name);
    table_set(state, info, "name", name);
    table_set(
        state,
        info,
        "iconID",
        Val::Num(spell.icon_file_data_id as f64),
    );
    table_set(
        state,
        info,
        "originalIconID",
        Val::Num(spell.icon_file_data_id as f64),
    );
    table_set(state, info, "castTime", Val::Num(0.0));
    table_set(state, info, "minRange", Val::Num(0.0));
    table_set(state, info, "maxRange", Val::Num(0.0));
    table_set(state, info, "spellID", Val::Num(spell_id as f64));
    state.push(info);
    Ok(1)
}

/// `C_Spell.GetSpellCooldown(spellID)` → `SpellCooldownInfo` table.
///
/// Retail fields: `startTime, duration, isEnabled, isActive, modRate`.
fn get_spell_cooldown(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let (start, duration) = {
        let sim = borrow_state(state)?;
        let now = sim.start_time.elapsed().as_secs_f64();
        spell_cooldown_times(&sim, spell_id, now)
    };
    let is_active = duration > 0.0;
    let info = create_table(state);
    table_set(state, info, "startTime", Val::Num(start));
    table_set(state, info, "duration", Val::Num(duration));
    table_set(state, info, "isEnabled", Val::Bool(true));
    table_set(state, info, "isActive", Val::Bool(is_active));
    table_set(state, info, "modRate", Val::Num(1.0));
    state.push(info);
    Ok(1)
}

/// `C_Spell.GetSpellCastCount(spellID)` → `0`.
///
/// ZoneAbility treats this as a fallback count when a spell has no
/// charge table. The sim does not track spell cast counts yet, so the
/// permissive baseline is zero.
fn get_spell_cast_count(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Num(0.0));
    Ok(1)
}

/// `C_Spell.GetMountFromSpell(spellID)` → mountID or nil.
///
/// Scans `world.mounts` for a matching spell_id. Returns the mount_id or nil.
fn get_mount_from_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let mount_id = sim
        .world
        .mounts
        .iter()
        .find(|m| m.spell_id == spell_id)
        .map(|m| m.mount_id);
    drop(sim);
    match mount_id {
        Some(id) => state.push(Val::Num(id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

/// `C_Spell.GetVisibilityInfo(spellID, filter)` → `(hasCustom, alwaysShowMine, showForMySpec)`.
///
/// Permissive: most spells are shown for the current spec but not custom-pinned.
fn get_visibility_info(state: &mut LuaState) -> LuaResult<u32> {
    // hasCustom=false: no user-defined custom visibility overrides.
    // alwaysShowMine=true: always show if cast by the player.
    // showForMySpec=false: not spec-pinned.
    state.push(Val::Bool(false));
    state.push(Val::Bool(true));
    state.push(Val::Bool(false));
    Ok(3)
}

/// `C_Spell.DoesSpellExist(spellID)` -> `bool`.
///
/// Permissive for addon/UI probes: any non-zero spell ID is treated as existing.
fn does_spell_exist(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Bool(spell_id != 0));
    Ok(1)
}

/// `C_Spell.IsSpellDataCached(spellID)` -> `true` for non-zero spell IDs.
fn is_spell_data_cached(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Bool(spell_id != 0));
    Ok(1)
}

/// `C_Spell.IsPriorityAura(spellID)` → `false`.
///
/// The sim does not model priority aura ordering; always false.
fn is_priority_aura(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

/// `C_Spell.IsSelfBuff(spellID)` → `bool`.
///
/// Returns true when `implicit_target == 1` (TARGET_UNIT_CASTER / Self).
fn is_self_buff(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    // implicit_target == 1 is TARGET_UNIT_CASTER (self-only cast target).
    let is_self = spells::get_spell(spell_id)
        .map(|s| s.implicit_target == 1)
        .unwrap_or(false);
    state.push(Val::Bool(is_self));
    Ok(1)
}

/// `C_Spell.IsSpellUsable(spellID)` → `(isUsable, insufficientPower)`.
///
/// Returns `(true, false)` for known spells; `(false, false)` otherwise.
fn is_spell_usable(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let known = borrow_state(state)
        .map(|sim| sim.known_spells.contains(&spell_id))
        .unwrap_or(false);
    state.push(Val::Bool(known));
    state.push(Val::Bool(false));
    Ok(2)
}

/// `C_Spell.TargetSpellIsEnchanting()` → `false`.
fn target_spell_is_enchanting(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// `C_Spell.TargetSpellJumpsUpgradeTrack()` → `false`.
fn target_spell_jumps_upgrade_track(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// `C_Spell.TargetSpellReplacesBonusTree()` → `false`.
fn target_spell_replaces_bonus_tree(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}
