//! C_UnitAuras namespace — state-backed buff data implementations.
//!
//! Patches the C_UnitAuras table (created in c_misc_api.rs) with real
//! implementations that read from SimState::player_buffs.

use crate::lua_api::SimState;
use mlua::{Lua, MultiValue, Result, Value};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// Patch C_UnitAuras with state-backed implementations.
pub fn patch_c_unit_auras(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    patch_get_aura_slots(lua, t, state.clone())?;
    patch_get_aura_data_by_aura_instance_id(lua, t, state.clone())?;
    patch_get_aura_data_by_slot(lua, t, state.clone())?;
    patch_get_aura_data_by_index(lua, t, state.clone())?;
    patch_get_buff_data_by_index(lua, t, state.clone())?;
    patch_get_player_aura_by_spell_id(lua, t, state.clone())?;
    patch_blocked_aura_methods(lua, t, state.clone())?;
    patch_get_aura_data_by_spell_name(lua, t, state)?;
    patch_aura_data_provider_methods(lua, t)?;
    Ok(())
}

fn visible_player_buffs<'a>(
    state: &'a SimState,
    blocked: Option<&HashSet<i32>>,
) -> Vec<&'a crate::lua_api::state::AuraInfo> {
    state
        .player
        .buffs
        .iter()
        .filter(|aura| !blocked.is_some_and(|blocked| blocked.contains(&aura.aura_instance_id)))
        .collect()
}

fn patch_blocked_aura_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    let st = Rc::clone(&state);
    t.set(
        "AddBlockedAura",
        lua.create_function(move |_, (unit, aura_instance_id): (String, i32)| {
            st.borrow_mut()
                .blocked_auras_by_unit
                .entry(unit)
                .or_default()
                .insert(aura_instance_id);
            Ok(())
        })?,
    )?;
    let st = Rc::clone(&state);
    t.set(
        "ClearBlockedAuras",
        lua.create_function(move |_, unit: Value| {
            let Some(unit) = (match unit {
                Value::String(unit) => Some(unit.to_str()?.to_string()),
                _ => None,
            }) else {
                return Ok(());
            };
            st.borrow_mut().blocked_auras_by_unit.remove(&unit);
            Ok(())
        })?,
    )
}

/// GetAuraDataByAuraInstanceID(unit, auraInstanceID) -> AuraData table or nil.
fn patch_get_aura_data_by_aura_instance_id(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetAuraDataByAuraInstanceID",
        lua.create_function(move |lua, (unit, aura_instance_id): (String, i32)| {
            if unit != "player" {
                return Ok(Value::Nil);
            }
            let s = state.borrow();
            match s
                .player
                .buffs
                .iter()
                .find(|aura| aura.aura_instance_id == aura_instance_id)
            {
                Some(aura) => Ok(Value::Table(super::aura_api::build_aura_data_table(
                    lua, aura,
                )?)),
                None => Ok(Value::Nil),
            }
        })?,
    )
}

fn patch_aura_data_provider_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "SwitchAuraDataProvider",
        lua.create_function(move |lua, ()| switch_aura_data_provider(lua))?,
    )?;
    t.set(
        "ResetAuraDataProvider",
        lua.create_function(move |lua, ()| reset_aura_data_provider(lua))?,
    )?;
    Ok(())
}

fn switch_aura_data_provider(lua: &Lua) -> Result<()> {
    let aura_util: mlua::Table = lua.globals().get("AuraUtil")?;
    let set_provider: mlua::Function = aura_util.get("SetDataProvider")?;
    let provider = match lua.globals().get::<Value>("GetEditModeAuraDataProvider")? {
        Value::Function(get_provider) => get_provider.call::<mlua::Table>(())?,
        _ => create_empty_aura_data_provider(lua)?,
    };
    set_provider.call::<()>(provider)
}

fn reset_aura_data_provider(lua: &Lua) -> Result<()> {
    let aura_util: mlua::Table = lua.globals().get("AuraUtil")?;
    let clear_provider: mlua::Function = aura_util.get("ClearDataProvider")?;
    clear_provider.call::<()>(())
}

fn create_empty_aura_data_provider(lua: &Lua) -> Result<mlua::Table> {
    let provider = lua.create_table()?;
    provider.set(
        "GetAuraSlots",
        lua.create_function(|_, _: MultiValue| Ok(MultiValue::from_vec(vec![Value::Nil])))?,
    )?;
    provider.set(
        "GetAuraDataBySlot",
        lua.create_function(|_, _: MultiValue| Ok(Value::Nil))?,
    )?;
    provider.set(
        "GetAuraDataByIndex",
        lua.create_function(|_, _: MultiValue| Ok(Value::Nil))?,
    )?;
    provider.set(
        "GetAuraDataByAuraInstanceID",
        lua.create_function(|_, _: MultiValue| Ok(Value::Nil))?,
    )?;
    Ok(provider)
}

/// GetAuraSlots(unit, filter, maxSlots, token) -> (token, slot1, slot2, ...).
fn patch_get_aura_slots(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set(
        "GetAuraSlots",
        lua.create_function(
            move |_,
                  (unit, filter, _max, token): (
                Option<String>,
                Option<String>,
                Option<i32>,
                Option<i32>,
            )| {
                if token.is_some() || unit.as_deref() != Some("player") {
                    return Ok(MultiValue::from_vec(vec![Value::Nil]));
                }
                let is_harmful = filter.as_ref().map_or(false, |f| f.contains("HARMFUL"));
                if is_harmful {
                    return Ok(MultiValue::from_vec(vec![Value::Nil]));
                }
                let s = state.borrow();
                let blocked = s.blocked_auras_by_unit.get("player");
                let mut vals = vec![Value::Nil]; // nil continuation = all in one batch
                for aura in visible_player_buffs(&s, blocked) {
                    vals.push(Value::Integer(aura.aura_instance_id as i64));
                }
                Ok(MultiValue::from_vec(vals))
            },
        )?,
    )
}

/// GetAuraDataBySlot(unit, slot) -> AuraData table or nil.
fn patch_get_aura_data_by_slot(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetAuraDataBySlot",
        lua.create_function(move |lua, (unit, slot): (String, i32)| {
            if unit != "player" {
                return Ok(Value::Nil);
            }
            let s = state.borrow();
            match s.player.buffs.iter().find(|a| a.aura_instance_id == slot) {
                Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(
                    lua, a,
                )?)),
                None => Ok(Value::Nil),
            }
        })?,
    )
}

/// GetAuraDataByIndex(unit, index, filter) -> AuraData table or nil.
fn patch_get_aura_data_by_index(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetAuraDataByIndex",
        lua.create_function(
            move |lua, (unit, index, filter): (String, i32, Option<String>)| {
                if unit != "player" || index < 1 {
                    return Ok(Value::Nil);
                }
                let dominated = filter
                    .as_ref()
                    .map_or(false, |f| f.contains("HARMFUL") || f.contains("MAW"));
                if dominated {
                    return Ok(Value::Nil);
                }
                let s = state.borrow();
                let blocked = s.blocked_auras_by_unit.get("player");
                match visible_player_buffs(&s, blocked).get((index - 1) as usize) {
                    Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(
                        lua, a,
                    )?)),
                    None => Ok(Value::Nil),
                }
            },
        )?,
    )
}

/// GetBuffDataByIndex(unit, index, filter) -> AuraData table or nil.
fn patch_get_buff_data_by_index(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetBuffDataByIndex",
        lua.create_function(
            move |lua, (unit, index, _filter): (String, i32, Option<String>)| {
                if unit != "player" || index < 1 {
                    return Ok(Value::Nil);
                }
                let s = state.borrow();
                let blocked = s.blocked_auras_by_unit.get("player");
                match visible_player_buffs(&s, blocked).get((index - 1) as usize) {
                    Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(
                        lua, a,
                    )?)),
                    None => Ok(Value::Nil),
                }
            },
        )?,
    )
}

/// GetPlayerAuraBySpellID(spellID) -> AuraData table or nil.
fn patch_get_player_aura_by_spell_id(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetPlayerAuraBySpellID",
        lua.create_function(move |lua, spell_id: i32| {
            let s = state.borrow();
            match s.player.buffs.iter().find(|a| a.spell_id == spell_id) {
                Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(
                    lua, a,
                )?)),
                None => Ok(Value::Nil),
            }
        })?,
    )
}

/// GetAuraDataBySpellName(unit, name, filter) -> AuraData table or nil.
fn patch_get_aura_data_by_spell_name(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetAuraDataBySpellName",
        lua.create_function(
            move |lua, (unit, name, _filter): (String, String, Option<String>)| {
                if unit != "player" {
                    return Ok(Value::Nil);
                }
                let s = state.borrow();
                match s.player.buffs.iter().find(|a| a.name == name) {
                    Some(a) => Ok(Value::Table(super::aura_api::build_aura_data_table(
                        lua, a,
                    )?)),
                    None => Ok(Value::Nil),
                }
            },
        )?,
    )
}
