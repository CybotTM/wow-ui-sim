//! Aura/buff API functions.
//!
//! Implements UnitBuff, UnitDebuff, UnitAura, GetPlayerAuraBySpellID,
//! and the AuraUtil namespace stubs.

use crate::lua_api::SimState;
use crate::lua_api::state::AuraInfo;
use mlua::{Lua, MultiValue, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register all aura-related global functions.
pub fn register_aura_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_unit_buff(lua, state.clone())?;
    register_unit_debuff(lua)?;
    register_unit_aura(lua, state.clone())?;
    register_get_player_aura_by_spell_id(lua, state.clone())?;
    lua.globals()
        .set("AuraUtil", register_aura_util(lua, state)?)?;
    Ok(())
}

/// Check if a filter string includes "HARMFUL".
fn filter_is_harmful(filter: &Option<String>) -> bool {
    filter.as_ref().map_or(false, |f| f.contains("HARMFUL"))
}

/// Get the nth player buff (1-based index), or None.
fn get_player_buff(state: &SimState, index: i32) -> Option<&AuraInfo> {
    if index < 1 {
        return None;
    }
    state.player.buffs.get((index - 1) as usize)
}

/// Build the old-style multi-return values for UnitBuff/UnitAura.
pub(super) fn build_aura_multi_value(lua: &Lua, aura: &AuraInfo) -> Result<MultiValue> {
    Ok(MultiValue::from_vec(vec![
        Value::String(lua.create_string(aura.name.as_str())?),
        Value::Integer(aura.icon as i64),
        Value::Integer(aura.applications as i64),
        Value::Nil, // dispelName (buffs have none)
        Value::Number(aura.duration),
        Value::Number(aura.expiration_time),
        Value::String(lua.create_string(aura.source_unit.as_str())?),
        Value::Boolean(aura.is_stealable),
        Value::Boolean(false), // nameplateShowPersonal
        Value::Integer(aura.spell_id as i64),
        Value::Boolean(aura.can_apply_aura),
        Value::Boolean(false), // isBossAura
        Value::Boolean(aura.is_from_player_or_player_pet),
        Value::Boolean(false), // nameplateShowAll
        Value::Number(1.0),    // timeMod
    ]))
}

/// Build an AuraData Lua table from an AuraInfo.
pub(super) fn build_aura_data_table(lua: &Lua, aura: &AuraInfo) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    set_aura_data_core_fields(lua, &t, aura)?;
    set_aura_data_extra_fields(lua, &t, aura)?;
    Ok(t)
}

/// Set the core AuraData fields (name, icon, duration, etc.).
fn set_aura_data_core_fields(lua: &Lua, t: &mlua::Table, aura: &AuraInfo) -> Result<()> {
    t.set("name", lua.create_string(aura.name.as_str())?)?;
    t.set("icon", aura.icon)?;
    t.set("applications", aura.applications)?;
    t.set("dispelName", Value::Nil)?;
    t.set("duration", aura.duration)?;
    t.set("expirationTime", aura.expiration_time)?;
    t.set("sourceUnit", lua.create_string(aura.source_unit.as_str())?)?;
    t.set("isStealable", aura.is_stealable)?;
    t.set("nameplateShowPersonal", false)?;
    t.set("spellId", aura.spell_id)?;
    Ok(())
}

/// Set the extra AuraData fields (boolean flags, instance ID, points).
fn set_aura_data_extra_fields(lua: &Lua, t: &mlua::Table, aura: &AuraInfo) -> Result<()> {
    t.set("canApplyAura", aura.can_apply_aura)?;
    t.set("isBossAura", false)?;
    t.set("isFromPlayerOrPlayerPet", aura.is_from_player_or_player_pet)?;
    t.set("nameplateShowAll", false)?;
    t.set("timeMod", 1.0)?;
    t.set("points", lua.create_table()?)?;
    t.set("auraInstanceID", aura.aura_instance_id)?;
    t.set("isHelpful", aura.is_helpful)?;
    t.set("isHarmful", false)?;
    t.set("isRaid", false)?;
    t.set("isNameplateOnly", false)?;
    Ok(())
}

/// Register UnitBuff: returns unpacked aura data for the nth player buff.
fn register_unit_buff(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "UnitBuff",
        lua.create_function(
            move |lua, (unit, index, _filter): (String, i32, Option<String>)| {
                if unit != "player" {
                    return Ok(MultiValue::new());
                }
                let s = state.borrow();
                match get_player_buff(&s, index) {
                    Some(aura) => build_aura_multi_value(lua, aura),
                    None => Ok(MultiValue::new()),
                }
            },
        )?,
    )
}

/// Register UnitDebuff: returns nil (no debuffs in sim).
fn register_unit_debuff(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "UnitDebuff",
        lua.create_function(
            |_, (_unit, _index, _filter): (String, i32, Option<String>)| Ok(Value::Nil),
        )?,
    )
}

/// Register UnitAura: returns unpacked aura data filtered by HELPFUL/HARMFUL.
fn register_unit_aura(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "UnitAura",
        lua.create_function(
            move |lua, (unit, index, filter): (String, i32, Option<String>)| {
                if unit != "player" || filter_is_harmful(&filter) {
                    return Ok(MultiValue::new());
                }
                let s = state.borrow();
                match get_player_buff(&s, index) {
                    Some(aura) => build_aura_multi_value(lua, aura),
                    None => Ok(MultiValue::new()),
                }
            },
        )?,
    )
}

/// Register GetPlayerAuraBySpellID: looks up a buff by spell ID.
fn register_get_player_aura_by_spell_id(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "GetPlayerAuraBySpellID",
        lua.create_function(move |lua, spell_id: i32| {
            let s = state.borrow();
            let aura = s.player.buffs.iter().find(|a| a.spell_id == spell_id);
            match aura {
                Some(a) => Ok(Value::Table(build_aura_data_table(lua, a)?)),
                None => Ok(Value::Nil),
            }
        })?,
    )
}

/// ForEachAura: iterate player buffs/debuffs and call the callback for each.
///
/// Signature: ForEachAura(unit, filter, maxCount, callback, usePackedAura)
/// If usePackedAura is true, passes the AuraData table directly.
/// Otherwise passes it through AuraUtil.UnpackAuraData (multi-return).
/// If the callback returns true, iteration stops early.
fn register_for_each_aura(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set(
        "ForEachAura",
        lua.create_function(
            move |lua,
                  (unit, filter, max, cb, use_packed): (
                String,
                String,
                Option<i32>,
                mlua::Function,
                Option<bool>,
            )| {
                if should_skip_for_each_aura(&unit, &filter) {
                    return Ok(());
                }
                let use_packed = use_packed.unwrap_or(false);
                let limit = for_each_aura_limit(max);
                let state = state.borrow();
                for (i, aura) in state.player.buffs.iter().enumerate() {
                    if i >= limit || invoke_for_each_aura_callback(lua, &cb, aura, use_packed)? {
                        break;
                    }
                }
                Ok(())
            },
        )?,
    )
}

fn should_skip_for_each_aura(unit: &str, filter: &str) -> bool {
    if unit != "player" {
        return true;
    }
    let is_harmful = filter.contains("HARMFUL");
    let is_helpful = filter.contains("HELPFUL");
    !is_helpful || is_harmful
}

fn for_each_aura_limit(max: Option<i32>) -> usize {
    max.unwrap_or(i32::MAX) as usize
}

fn invoke_for_each_aura_callback(
    lua: &Lua,
    callback: &mlua::Function,
    aura: &crate::lua_api::state::AuraInfo,
    use_packed: bool,
) -> Result<bool> {
    let done: Option<bool> = if use_packed {
        callback.call(Value::Table(build_aura_data_table(lua, aura)?))?
    } else {
        callback.call(build_aura_multi_value(lua, aura)?)?
    };
    Ok(done == Some(true))
}

/// AuraUtil namespace (may be overridden by Blizzard AuraUtil.lua in full sim).
fn register_aura_util(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Table> {
    let aura_util = lua.create_table()?;
    register_for_each_aura(lua, &aura_util, state)?;
    register_aura_util_provider_methods(lua, &aura_util)?;
    if let Ok(provider) = lua.globals().get::<mlua::Table>("C_UnitAuras") {
        aura_util.raw_set("__provider", provider)?;
    }
    aura_util.set(
        "FindAura",
        lua.create_function(
            |_,
             (_pred, _unit, _filter, _spell, _caster): (
                mlua::Function,
                String,
                String,
                Option<i32>,
                Option<String>,
            )| Ok(Value::Nil),
        )?,
    )?;
    aura_util.set("UnpackAuraData", lua.create_function(unpack_aura_data)?)?;
    let aura_util_ref = aura_util.clone();
    aura_util.set(
        "FindAuraByName",
        lua.create_function(move |lua, (name, unit, filter): (String, String, String)| {
            find_aura_by_name(lua, &aura_util_ref, &name, &unit, &filter)
        })?,
    )?;
    let aura_util_ref = aura_util.clone();
    aura_util.set(
        "GetAuraDataByAuraInstanceID",
        lua.create_function(move |_, (unit, aura_instance_id): (String, i32)| {
            call_aura_provider_method(
                &aura_util_ref,
                "GetAuraDataByAuraInstanceID",
                (unit, aura_instance_id),
            )
        })?,
    )?;
    Ok(aura_util)
}

fn register_aura_util_provider_methods(lua: &Lua, aura_util: &mlua::Table) -> Result<()> {
    let aura_util_ref = aura_util.clone();
    aura_util.set(
        "SetDataProvider",
        lua.create_function(move |_, provider: mlua::Table| {
            aura_util_ref.raw_set("__provider", provider)?;
            Ok(())
        })?,
    )?;
    let aura_util_ref = aura_util.clone();
    aura_util.set(
        "ClearDataProvider",
        lua.create_function(move |lua, ()| {
            let provider: Value = lua.globals().get("C_UnitAuras")?;
            aura_util_ref.raw_set("__provider", provider)?;
            Ok(())
        })?,
    )?;
    Ok(())
}

fn current_aura_provider(aura_util: &mlua::Table) -> Result<mlua::Table> {
    match aura_util.raw_get::<Value>("__provider")? {
        Value::Table(provider) => Ok(provider),
        _ => Err(mlua::Error::RuntimeError(
            "AuraUtil provider is not initialized".to_string(),
        )),
    }
}

fn call_aura_provider_method<A>(
    aura_util: &mlua::Table,
    method_name: &str,
    args: A,
) -> Result<Value>
where
    A: mlua::IntoLuaMulti,
{
    let provider = current_aura_provider(aura_util)?;
    match provider.get::<Value>(method_name)? {
        Value::Function(method) => method.call(args),
        _ => Ok(Value::Nil),
    }
}

fn unpack_aura_data(lua: &Lua, aura_data: Value) -> Result<MultiValue> {
    let Value::Table(aura_data) = aura_data else {
        return Ok(MultiValue::new());
    };
    Ok(MultiValue::from_vec(vec![
        aura_data.get("name")?,
        aura_data.get("icon")?,
        aura_data.get("applications")?,
        aura_data.get("dispelName")?,
        aura_data.get("duration")?,
        aura_data.get("expirationTime")?,
        aura_data.get("sourceUnit")?,
        aura_data.get("isStealable")?,
        aura_data.get("nameplateShowPersonal")?,
        aura_data.get("spellId")?,
        aura_data.get("canApplyAura")?,
        aura_data.get("isBossAura")?,
        aura_data.get("isFromPlayerOrPlayerPet")?,
        aura_data.get("nameplateShowAll")?,
        aura_data.get("timeMod")?,
        aura_data
            .get("points")
            .unwrap_or(Value::Table(lua.create_table()?)),
    ]))
}

fn find_aura_by_name(
    lua: &Lua,
    aura_util: &mlua::Table,
    name: &str,
    unit: &str,
    filter: &str,
) -> Result<MultiValue> {
    let aura_data = call_aura_provider_method(
        aura_util,
        "GetAuraDataBySpellName",
        (unit.to_string(), name.to_string(), Some(filter.to_string())),
    )?;
    unpack_aura_data(lua, aura_data)
}
