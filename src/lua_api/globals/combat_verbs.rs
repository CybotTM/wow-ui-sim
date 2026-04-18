//! Combat / cast verbs that mutate `SimState.casting`.
//!
//! Migrates action/cast globals off `GLOBAL_NIL_STUBS` onto the existing cast
//! pipeline so the simulator can exercise spell-in-flight UI:
//!
//! - `AttackTarget()`         — starts an "Auto Attack" cast marker
//! - `StopAttack()`           — clears the Auto Attack marker
//! - `CastSpell(id [, unit])`         — legacy signature, forwards to CastSpellByID
//! - `CastSpellByID(id [, unit])`     — starts/executes the spell
//! - `CastSpellByName(name [, unit])` — resolves and starts/executes the spell
//! - `ClickSpecialAbility(index)`     — index 1 => Auto Attack toggle,
//!                                       2 => Extra Attack marker
//! - `SpellTargetUnit(unit)`  — no-op when no cast pending; consumes the
//!                               pending cast target when one exists
//! - `SpellIsTargeting()`     — false until the sim models spell targeting
//! - `SpellCanTargetItem()`   — false until item-targeting cursor exists
//! - `SpellCanTargetItemID()` — false until item-targeting cursor exists
//! - `SpellStopTargeting()`   — no-op companion to `SpellIsTargeting`
//! - `UseAction(slot)`        — cast/execute the spell in an action bar slot
//! - `ActionButtonDown(slot)` / `ActionButtonUp(slot)` — mirror button state
//!
//! Registered from `register_tail_globals` in `register.rs` — runs after
//! `missing_surface` so the real Rust bodies overwrite any pre-existing
//! stub_nil entries that slipped through the stubs pass.

use crate::lua_api::env::WowLuaAppData;
use crate::lua_api::game_data::{self, CastingState};
use crate::lua_api::globals::spell_api::spell_cast_time;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, frame_ref,
};
use crate::lua_api::script_helpers::{get_event_listeners, get_script};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

const DEFAULT_CAST_DURATION: f64 = 1.5;
const AUTO_ATTACK_NAME: &str = "Auto Attack";
const EXTRA_ATTACK_NAME: &str = "Extra Attack";
const DEFAULT_ICON: &str = "Interface/Icons/INV_Misc_QuestionMark";

fn fire_named_event(state: &mut LuaState, event_name: &str, args: &[Val]) {
    for widget_id in get_event_listeners(state, event_name) {
        let Some(handler) = get_script(state, widget_id, "OnEvent") else {
            continue;
        };
        let Ok(frame) = frame_ref(state, widget_id) else {
            continue;
        };
        let event_name_val = create_string(state, event_name);
        let mut call_args = Vec::with_capacity(args.len() + 2);
        call_args.push(frame);
        call_args.push(event_name_val);
        call_args.extend_from_slice(args);
        let _ = call_function_state(state, handler, &call_args);
    }
}

fn spell_name(spell_id: u32) -> String {
    crate::spells::get_spell(spell_id)
        .map(|spell| spell.name.to_string())
        .unwrap_or_else(|| format!("Spell {spell_id}"))
}

fn spell_icon(spell_id: u32) -> String {
    crate::spells::get_spell(spell_id)
        .and_then(|spell| crate::manifest_interface_data::get_texture_path(spell.icon_file_data_id))
        .unwrap_or(DEFAULT_ICON)
        .to_string()
}

fn start_cast(
    state: &mut LuaState,
    spell_id: u32,
    spell_name: &str,
    icon_path: &str,
    duration: f64,
) {
    let Ok(mut st) = borrow_state_mut(state) else {
        return;
    };
    let now = st.start_time.elapsed().as_secs_f64();
    let cast_id = st.next_cast_id;
    st.next_cast_id = st.next_cast_id.wrapping_add(1);
    st.casting = Some(CastingState {
        spell_id,
        spell_name: spell_name.to_string(),
        icon_path: icon_path.to_string(),
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
    start_cast(state, 0, AUTO_ATTACK_NAME, DEFAULT_ICON, f64::INFINITY);
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

fn resolve_spell_id_by_name(name: &str) -> Option<u32> {
    crate::lua_api::globals::spellbook_data::find_spell_by_name(name)
}

pub(crate) fn execute_spell_by_id(state: &mut LuaState, spell_id: u32) -> LuaResult<()> {
    {
        let st = borrow_state(state)?;
        if st.casting.is_some() {
            return Ok(());
        }
    }

    let spell_name = spell_name(spell_id);
    let icon_path = spell_icon(spell_id);
    let cast_time_ms = spell_cast_time(spell_id as i32);
    if cast_time_ms > 0 {
        start_cast(
            state,
            spell_id,
            &spell_name,
            &icon_path,
            cast_time_ms as f64 / 1000.0,
        );
        let player = create_string(state, "player");
        let spell_id_val = Val::Num(spell_id as f64);
        fire_named_event(state, "UNIT_SPELLCAST_START", &[player, spell_id_val]);
        return Ok(());
    }

    apply_spell_to_target(state, spell_id);
    Ok(())
}

fn apply_spell_to_target(state: &mut LuaState, spell_id: u32) {
    let Some(app_data) = state.app_data::<WowLuaAppData>().cloned() else {
        return;
    };
    if let Some(unit_id) = game_data::apply_spell_to_state(&app_data.sim_state, spell_id) {
        let unit = create_string(state, &unit_id);
        fire_named_event(state, "UNIT_HEALTH", &[unit]);
    }
}

fn set_action_button_state(state: &mut LuaState, slot: u32, button_state: u8) {
    let Some(button_name) = slot
        .checked_sub(0)
        .map(|slot| format!("ActionButton{slot}"))
    else {
        return;
    };
    let button_id = borrow_state(state)
        .ok()
        .and_then(|sim| sim.widgets.get_id_by_name(&button_name));
    let Some(button_id) = button_id else {
        return;
    };
    if let Ok(mut sim) = borrow_state_mut(state)
        && let Some(button) = sim.widgets.get_mut_visual(button_id)
    {
        button.button_state = button_state;
    }
}

/// `CastSpellByID(spellId [, unit])` — set `SimState.casting` to the spell.
pub(crate) fn cast_spell_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let Some(spell_id) = stack_u32(state, 1) else {
        return Ok(0);
    };
    let _unit = Option::<String>::from_stack(state, 2)?;
    execute_spell_by_id(state, spell_id)?;
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
    if let Some(spell_id) = resolve_spell_id_by_name(&name) {
        let _unit = Option::<String>::from_stack(state, 2)?;
        execute_spell_by_id(state, spell_id)?;
    } else {
        start_cast(state, 0, &name, DEFAULT_ICON, DEFAULT_CAST_DURATION);
    }
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
            start_cast(state, 0, AUTO_ATTACK_NAME, DEFAULT_ICON, f64::INFINITY);
        }
        2 => {
            start_cast(
                state,
                0,
                EXTRA_ATTACK_NAME,
                DEFAULT_ICON,
                DEFAULT_CAST_DURATION,
            );
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

/// `SpellCanTargetItem()` — item-targeting cursor is not modeled yet.
fn spell_can_target_item(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// `SpellCanTargetItemID()` — item-targeting cursor is not modeled yet.
fn spell_can_target_item_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// `SpellStopTargeting()` — silent no-op until targeting cursor state exists.
fn spell_stop_targeting(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn use_action(state: &mut LuaState) -> LuaResult<u32> {
    let Some(slot) = stack_u32(state, 1) else {
        return Ok(0);
    };
    let spell_id = {
        let st = borrow_state(state)?;
        st.action_bars.get(&slot).copied()
    };
    if let Some(spell_id) = spell_id {
        execute_spell_by_id(state, spell_id)?;
    }
    Ok(0)
}

fn action_button_down(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(slot) = stack_u32(state, 1) {
        set_action_button_state(state, slot, 1);
    }
    Ok(0)
}

fn action_button_up(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(slot) = stack_u32(state, 1) {
        set_action_button_state(state, slot, 0);
    }
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "AttackTarget", attack_target)?;
    LuaApiMut::register_function(lua, "StopAttack", stop_attack)?;
    LuaApiMut::register_function(lua, "CastSpell", cast_spell)?;
    LuaApiMut::register_function(lua, "CastSpellByID", cast_spell_by_id)?;
    LuaApiMut::register_function(lua, "CastSpellByName", cast_spell_by_name)?;
    LuaApiMut::register_function(lua, "UseAction", use_action)?;
    LuaApiMut::register_function(lua, "ActionButtonDown", action_button_down)?;
    LuaApiMut::register_function(lua, "ActionButtonUp", action_button_up)?;
    LuaApiMut::register_function(lua, "ClickSpecialAbility", click_special_ability)?;
    LuaApiMut::register_function(lua, "SpellTargetUnit", spell_target_unit)?;
    LuaApiMut::register_function(lua, "SpellIsTargeting", spell_is_targeting)?;
    LuaApiMut::register_function(lua, "SpellCanTargetItem", spell_can_target_item)?;
    LuaApiMut::register_function(lua, "SpellCanTargetItemID", spell_can_target_item_id)?;
    LuaApiMut::register_function(lua, "SpellStopTargeting", spell_stop_targeting)?;
    Ok(())
}
