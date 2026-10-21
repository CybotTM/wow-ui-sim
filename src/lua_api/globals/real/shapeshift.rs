//! State-backed shapeshift / stance globals consumed by `Blizzard_ActionBar/Shared/StanceBar.lua`.
//!
//! Modern profiles keep the live API shape below. Wrath returns its native
//! texture/name/numeric-active/numeric-castable projection; see
//! `docs/specs/wrath-action-callback-queries.md`.
//!
//! Modern API shape:
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

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, val_to_string};
use crate::lua_api::script_helpers::fire_named_event_state;
use crate::lua_api::state::ShapeshiftForm;
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

const WRATH_INFO: bool = cfg!(feature = "client-wrath");

fn push_form_info(state: &mut LuaState, form: &ShapeshiftForm) -> u32 {
    let texture = create_string(state, &form.texture);
    state.push(texture);
    if WRATH_INFO {
        let name = create_string(state, &form.name);
        state.push(name);
        state.push(if form.is_active {
            Val::Num(1.0)
        } else {
            Val::Nil
        });
        let castable = form.is_active || form.is_castable;
        state.push(if castable { Val::Num(1.0) } else { Val::Nil });
    } else {
        state.push(Val::Bool(form.is_active));
        state.push(Val::Bool(form.is_castable));
        state.push(Val::Num(form.spell_id as f64));
    }
    4
}

fn push_missing_form_info(state: &mut LuaState) -> u32 {
    if WRATH_INFO {
        return 0;
    }
    state.push(Val::Nil);
    1
}

fn form_info_index(state: &LuaState) -> LuaResult<Option<i32>> {
    if !WRATH_INFO {
        return Ok(stack_i32(state, 1));
    }
    let value = stack_val(state, 1);
    let number = match value {
        Val::Num(number) => Some(number),
        Val::Str(_) => val_to_string(state, value).and_then(|s| s.trim().parse::<f64>().ok()),
        _ => None,
    }
    .filter(|number| *number != 0.0)
    .ok_or_else(|| rilua::runtime_error("Usage: GetShapeshiftFormInfo(index)"))?;
    Ok(Some(number as i32))
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

/// Modern or native Wrath projection of the supplied form record.
fn get_shapeshift_form_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = form_info_index(state)? else {
        return Ok(push_missing_form_info(state));
    };
    let Some(zero_based) = form_index_to_zero_based(index) else {
        return Ok(push_missing_form_info(state));
    };
    let form = borrow_state(state)?
        .shapeshift_forms
        .get(zero_based)
        .cloned();
    let Some(form) = form else {
        return Ok(push_missing_form_info(state));
    };
    let pushed = push_form_info(state, &form);
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
