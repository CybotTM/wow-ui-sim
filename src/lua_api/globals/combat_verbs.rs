//! Combat / cast verbs that mutate `SimState.casting`.
//!
//! Migrates 7 entries off `GLOBAL_NIL_STUBS` onto the existing cast
//! pipeline so the simulator can exercise spell-in-flight UI:
//!
//! - `AttackTarget()`         — starts an "Auto Attack" cast marker
//! - `StopAttack()`           — clears the Auto Attack marker
//! - `CastSpell(id [, unit])` — legacy signature, forwards to CastSpellByID
//! - `CastSpellByID(id [, unit])`     — sets casting to the spell
//! - `CastSpellByName(name [, unit])` — sets casting to the spell
//! - `ClickSpecialAbility(index)`     — index 1 => Auto Attack toggle,
//!                                       2 => Extra Attack marker
//! - `SpellTargetUnit(unit)`  — no-op when no cast pending; consumes the
//!                               pending cast target when one exists
//! - `SpellIsTargeting()`     — false until the sim models spell targeting
//! - `SpellStopTargeting()`   — no-op companion to `SpellIsTargeting`
//!
//! The sim has no spell DB, so CastSpellByID synthesizes
//! `spell_name = "Spell <id>"` and reuses `"Interface/Icons/INV_Misc_QuestionMark"`.
//! Duration defaults to 1.5s (GCD). Callers that need a real spell
//! duration should drive it via `A_Admin.StartCasting` instead.
//!
//! Registered from `register_tail_globals` in `register.rs` — runs after
//! `missing_surface` so the real Rust bodies overwrite any pre-existing
//! stub_nil entries that slipped through the stubs pass.

use crate::lua_api::game_data::CastingState;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

const DEFAULT_CAST_DURATION: f64 = 1.5;
const AUTO_ATTACK_NAME: &str = "Auto Attack";
const EXTRA_ATTACK_NAME: &str = "Extra Attack";
const DEFAULT_ICON: &str = "Interface/Icons/INV_Misc_QuestionMark";

fn start_cast(state: &mut LuaState, spell_id: u32, spell_name: &str, duration: f64) {
    let Ok(mut st) = borrow_state_mut(state) else {
        return;
    };
    let now = st.start_time.elapsed().as_secs_f64();
    let cast_id = st.next_cast_id;
    st.next_cast_id = st.next_cast_id.wrapping_add(1);
    st.casting = Some(CastingState {
        spell_id,
        spell_name: spell_name.to_string(),
        icon_path: DEFAULT_ICON.to_string(),
        start_time: now,
        end_time: now + duration,
        cast_id,
    });
}

fn clear_cast_if_named(state: &mut LuaState, expected_name: &str) {
    let Ok(mut st) = borrow_state_mut(state) else {
        return;
    };
    if st
        .casting
        .as_ref()
        .is_some_and(|c| c.spell_name == expected_name)
    {
        st.casting = None;
    }
}

/// `AttackTarget()` — engage auto-attack on the current target.
fn attack_target(state: &mut LuaState) -> LuaResult<u32> {
    start_cast(state, 0, AUTO_ATTACK_NAME, f64::INFINITY);
    Ok(0)
}

/// `StopAttack()` — drop the auto-attack marker if present.
fn stop_attack(state: &mut LuaState) -> LuaResult<u32> {
    clear_cast_if_named(state, AUTO_ATTACK_NAME);
    Ok(0)
}

fn stack_u32(state: &mut LuaState, index: i32) -> Option<u32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as u32),
        _ => None,
    }
}

/// `CastSpellByID(spellId [, unit])` — set `SimState.casting` to the spell.
pub(crate) fn cast_spell_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let Some(spell_id) = stack_u32(state, 1) else {
        return Ok(0);
    };
    let name = format!("Spell {spell_id}");
    start_cast(state, spell_id, &name, DEFAULT_CAST_DURATION);
    Ok(0)
}

/// `CastSpell(spellId [, unit])` — legacy entry; same effect as CastSpellByID.
fn cast_spell(state: &mut LuaState) -> LuaResult<u32> {
    cast_spell_by_id(state)
}

/// `CastSpellByName(name [, unit])` — set `SimState.casting` to the named spell.
pub(crate) fn cast_spell_by_name(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = Option::<String>::from_stack(state, 1)? else {
        return Ok(0);
    };
    if name.is_empty() {
        return Ok(0);
    }
    start_cast(state, 0, &name, DEFAULT_CAST_DURATION);
    Ok(0)
}

/// `ClickSpecialAbility(index)` — 1 = auto-attack toggle, 2 = extra attack,
/// other indices are silent no-ops.
fn click_special_ability(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_u32(state, 1) else {
        return Ok(0);
    };
    match index {
        1 => {
            start_cast(state, 0, AUTO_ATTACK_NAME, f64::INFINITY);
        }
        2 => {
            start_cast(state, 0, EXTRA_ATTACK_NAME, DEFAULT_CAST_DURATION);
        }
        _ => {}
    }
    Ok(0)
}

/// `SpellTargetUnit(unit)` — consume the pending cast target when a cast
/// is in flight. Silent no-op otherwise, matching retail behaviour where
/// `SpellTargetUnit` is only meaningful for a pending spell.
fn spell_target_unit(state: &mut LuaState) -> LuaResult<u32> {
    let Ok(st) = borrow_state_mut(state) else {
        return Ok(0);
    };
    if st.casting.is_none() {
        return Ok(0);
    }
    drop(st);
    let _ = Option::<String>::from_stack(state, 1);
    Ok(0)
}

/// `SpellIsTargeting()` — targeting cursor is not modeled yet.
fn spell_is_targeting(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// `SpellStopTargeting()` — silent no-op until targeting cursor state exists.
fn spell_stop_targeting(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "AttackTarget", attack_target)?;
    LuaApiMut::register_function(lua, "StopAttack", stop_attack)?;
    LuaApiMut::register_function(lua, "CastSpell", cast_spell)?;
    LuaApiMut::register_function(lua, "CastSpellByID", cast_spell_by_id)?;
    LuaApiMut::register_function(lua, "CastSpellByName", cast_spell_by_name)?;
    LuaApiMut::register_function(lua, "ClickSpecialAbility", click_special_ability)?;
    LuaApiMut::register_function(lua, "SpellTargetUnit", spell_target_unit)?;
    LuaApiMut::register_function(lua, "SpellIsTargeting", spell_is_targeting)?;
    LuaApiMut::register_function(lua, "SpellStopTargeting", spell_stop_targeting)?;
    Ok(())
}
