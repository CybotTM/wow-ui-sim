//! Shapeshift / stance globals consumed by `Blizzard_ActionBar/Shared/StanceBar.lua`.
//!
//! Mirrors the live API shape:
//!
//! - `GetNumShapeshiftForms()` → `state.shapeshift_forms.len()`.
//! - `GetShapeshiftForm()` → active 1-based form index, or `0` when none.
//! - `GetShapeshiftFormInfo(index)` → `(texture, isActive, isCastable, spellID)`
//!   from `state.shapeshift_forms[index-1]`.
//! - `GetShapeshiftFormCooldown(index)` → `(start, duration, enable)` from
//!   `state.shapeshift_cooldowns[index]`. Missing entries return `(0, 0, 1)`
//!   like every other cooldown global.
//! - `CastShapeshiftForm(index)` → toggles `is_active` on the indexed form,
//!   clears every other form's `is_active`, and fires `UPDATE_SHAPESHIFT_FORM`
//!   so `StanceBarMixin:OnEvent` re-runs `Update`.
//!
//! The existing `runtime_surface_bootstrap.lua` `if ... == nil` guards on
//! shapeshift APIs are no-ops — registration here runs before the bootstrap
//! script.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_api::script_helpers::fire_named_event_state;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn form_index_to_zero_based(index: i32) -> Option<usize> {
    usize::try_from(index.checked_sub(1)?).ok()
}

fn push_form_info(
    state: &mut LuaState,
    texture: &str,
    is_active: bool,
    is_castable: bool,
    spell_id: u32,
) -> u32 {
    let texture_val = create_string(state, texture);
    state.push(texture_val);
    state.push(Val::Bool(is_active));
    state.push(Val::Bool(is_castable));
    state.push(Val::Num(spell_id as f64));
    4
}

/// `GetNumShapeshiftForms()` — number of currently available stance/forms.
fn get_num_shapeshift_forms(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.shapeshift_forms.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

/// `GetShapeshiftForm()` — active 1-based stance/form index, or `0` when no
/// form is active.
fn get_shapeshift_form(state: &mut LuaState) -> LuaResult<u32> {
    let active_index = borrow_state(state)?
        .shapeshift_forms
        .iter()
        .position(|form| form.is_active)
        .map(|index| index + 1)
        .unwrap_or(0);
    state.push(Val::Num(active_index as f64));
    Ok(1)
}

/// `GetShapeshiftFormInfo(index)` — `(texture, isActive, isCastable, spellID)`.
/// Out-of-range / non-numeric indexes return `nil` (matches the bootstrap
/// stub's previous shape).
fn get_shapeshift_form_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let Some(zero_based) = form_index_to_zero_based(index) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let form = borrow_state(state)?
        .shapeshift_forms
        .get(zero_based)
        .cloned();
    let Some(form) = form else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let pushed = push_form_info(
        state,
        &form.texture,
        form.is_active,
        form.is_castable,
        form.spell_id,
    );
    Ok(pushed)
}

/// `GetShapeshiftFormCooldown(index)` — `(start, duration, enable)`. Missing
/// or expired entries report `(0, 0, 1)` so the StanceBar swipe stays clear.
fn get_shapeshift_form_cooldown(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    let entry = borrow_state(state)?
        .shapeshift_cooldowns
        .get(&index)
        .cloned();
    let (start, duration) = entry
        .map(|cd| (cd.start, cd.duration))
        .unwrap_or((0.0, 0.0));
    state.push(Val::Num(start));
    state.push(Val::Num(duration));
    state.push(Val::Num(1.0));
    Ok(3)
}

/// `CastShapeshiftForm(index)` — toggles activation on the indexed form and
/// fires `UPDATE_SHAPESHIFT_FORM`. Out-of-range indexes are silent no-ops
/// (matches Blizzard: clicking a non-existent stance is harmless).
fn cast_shapeshift_form(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let Some(zero_based) = form_index_to_zero_based(index) else {
        return Ok(0);
    };
    let updated = {
        let mut sim = borrow_state_mut(state)?;
        let in_range = zero_based < sim.shapeshift_forms.len();
        if !in_range {
            return Ok(0);
        }
        let was_active = sim.shapeshift_forms[zero_based].is_active;
        for form in sim.shapeshift_forms.iter_mut() {
            form.is_active = false;
        }
        sim.shapeshift_forms[zero_based].is_active = !was_active;
        true
    };
    if updated {
        fire_named_event_state(state, "UPDATE_SHAPESHIFT_FORM", &[]);
    }
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetNumShapeshiftForms", get_num_shapeshift_forms)?;
    LuaApiMut::register_function(lua, "GetShapeshiftForm", get_shapeshift_form)?;
    LuaApiMut::register_function(lua, "GetShapeshiftFormInfo", get_shapeshift_form_info)?;
    LuaApiMut::register_function(
        lua,
        "GetShapeshiftFormCooldown",
        get_shapeshift_form_cooldown,
    )?;
    LuaApiMut::register_function(lua, "CastShapeshiftForm", cast_shapeshift_form)?;
    Ok(())
}
