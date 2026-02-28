//! C_UnitAuras namespace — state-backed buff data implementations.
//!
//! Patches the C_UnitAuras table (created in c_misc_api.rs) with real
//! implementations that read from SimState::player_buffs.

use crate::lua_api::SimState;
use mlua::{Lua, MultiValue, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Patch C_UnitAuras with state-backed implementations.
pub fn patch_c_unit_auras(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    patch_get_aura_slots(lua, t, state.clone())?;
    patch_get_aura_data_by_slot(lua, t, state.clone())?;
    patch_get_aura_data_by_index(lua, t, state.clone())?;
    patch_get_buff_data_by_index(lua, t, state.clone())?;
    patch_get_player_aura_by_spell_id(lua, t, state.clone())?;
    patch_get_aura_data_by_spell_name(lua, t, state)?;
    Ok(())
}

/// GetAuraSlots(unit, filter, maxSlots, token) -> (token, slot1, slot2, ...).
fn patch_get_aura_slots(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetAuraSlots", lua.create_function(
        move |_, (unit, filter, _max, token): (Option<String>, Option<String>, Option<i32>, Option<i32>)| {
            if token.is_some() || unit.as_deref() != Some("player") {
                return Ok(MultiValue::from_vec(vec![Value::Nil]));
            }
            let is_harmful = filter.as_ref().map_or(false, |f| f.contains("HARMFUL"));
            if is_harmful {
                return Ok(MultiValue::from_vec(vec![Value::Nil]));
            }
            let s = state.borrow();
            let mut vals = vec![Value::Nil]; // nil continuation = all in one batch
            for aura in &s.player.buffs {
                vals.push(Value::Integer(aura.aura_instance_id as i64));
            }
            Ok(MultiValue::from_vec(vals))
        },
    )?)
}

/// GetAuraDataBySlot(unit, slot) -> AuraData table or nil.
fn patch_get_aura_data_by_slot(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetAuraDataBySlot", lua.create_function(
        move |lua, (unit, slot): (String, i32)| {
            if unit != "player" { return Ok(Value::Nil); }
            let s = state.borrow();
            match s.player.buffs.iter().find(|a| a.aura_instance_id == slot) {
                Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(lua, a)?)),
                None => Ok(Value::Nil),
            }
        },
    )?)
}

/// GetAuraDataByIndex(unit, index, filter) -> AuraData table or nil.
fn patch_get_aura_data_by_index(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetAuraDataByIndex", lua.create_function(
        move |lua, (unit, index, filter): (String, i32, Option<String>)| {
            if unit != "player" || index < 1 { return Ok(Value::Nil); }
            let dominated = filter.as_ref().map_or(false, |f| {
                f.contains("HARMFUL") || f.contains("MAW")
            });
            if dominated { return Ok(Value::Nil); }
            let s = state.borrow();
            match s.player.buffs.get((index - 1) as usize) {
                Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(lua, a)?)),
                None => Ok(Value::Nil),
            }
        },
    )?)
}

/// GetBuffDataByIndex(unit, index, filter) -> AuraData table or nil.
fn patch_get_buff_data_by_index(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetBuffDataByIndex", lua.create_function(
        move |lua, (unit, index, _filter): (String, i32, Option<String>)| {
            if unit != "player" || index < 1 { return Ok(Value::Nil); }
            let s = state.borrow();
            match s.player.buffs.get((index - 1) as usize) {
                Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(lua, a)?)),
                None => Ok(Value::Nil),
            }
        },
    )?)
}

/// GetPlayerAuraBySpellID(spellID) -> AuraData table or nil.
fn patch_get_player_aura_by_spell_id(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetPlayerAuraBySpellID", lua.create_function(
        move |lua, spell_id: i32| {
            let s = state.borrow();
            match s.player.buffs.iter().find(|a| a.spell_id == spell_id) {
                Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(lua, a)?)),
                None => Ok(Value::Nil),
            }
        },
    )?)
}

/// GetAuraDataBySpellName(unit, name, filter) -> AuraData table or nil.
fn patch_get_aura_data_by_spell_name(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetAuraDataBySpellName", lua.create_function(
        move |lua, (unit, name, _filter): (String, String, Option<String>)| {
            if unit != "player" { return Ok(Value::Nil); }
            let s = state.borrow();
            match s.player.buffs.iter().find(|a| a.name == name) {
                Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(lua, a)?)),
                None => Ok(Value::Nil),
            }
        },
    )?)
}
