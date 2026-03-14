//! C_ProfSpecs and C_SettingsUtil namespace stubs.

use mlua::{Lua, Result, Value};

/// Register profession and settings namespace stubs.
pub fn register_profession_stubs(lua: &Lua) -> Result<()> {
    register_c_prof_specs(lua)?;
    register_c_settings_util(lua)?;
    Ok(())
}

/// C_ProfSpecs namespace - profession specialization data.
/// Returns nil/false/empty for all queries since the simulator has no professions.
fn register_c_prof_specs(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetStateForTab",
        lua.create_function(|_, (_tab, _cfg): (Value, Value)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetSpendCurrencyForPath",
        lua.create_function(|_, _path: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetUnlockEntryForPath",
        lua.create_function(|_, _path: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetTabInfo",
        lua.create_function(|_, _tab: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetDefaultSpecSkillLine",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "SkillLineHasSpecialization",
        lua.create_function(|_, _skill: Value| Ok(false))?,
    )?;
    t.set(
        "GetUnlockRankForPerk",
        lua.create_function(|_, _perk: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetStateForPerk",
        lua.create_function(|_, (_perk, _cfg): (Value, Value)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetStateForPath",
        lua.create_function(|_, (_path, _cfg): (Value, Value)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetSpecTabInfo",
        lua.create_function(|lua, ()| {
            let info = lua.create_table()?;
            info.set("enabled", false)?;
            info.set("errorReason", "")?;
            Ok(info)
        })?,
    )?;
    t.set(
        "GetSourceTextForPath",
        lua.create_function(|_, (_path, _cfg): (Value, Value)| Ok(""))?,
    )?;
    t.set(
        "GetPerksForPath",
        lua.create_function(|lua, _path: Value| lua.create_table())?,
    )?;
    t.set(
        "GetNewSpecReminderProfName",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetDescriptionForPerk",
        lua.create_function(|_, _perk: Value| Ok(""))?,
    )?;
    t.set("ShouldShowSpecTab", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "ShouldShowPointsReminderForSkillLine",
        lua.create_function(|_, _skill: Value| Ok(false))?,
    )?;
    t.set(
        "ShouldShowPointsReminder",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetSpendEntryForPath",
        lua.create_function(|_, _path: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetSpecTabIDsForSkillLine",
        lua.create_function(|lua, _skill: Value| lua.create_table())?,
    )?;
    t.set(
        "GetEntryIDForPerk",
        lua.create_function(|_, _perk: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetDescriptionForPath",
        lua.create_function(|_, _path: Value| Ok(""))?,
    )?;
    register_c_prof_specs_remaining(lua, &t)?;
    lua.globals().set("C_ProfSpecs", t)?;
    Ok(())
}

/// Remaining C_ProfSpecs methods (split to stay under 50 lines per function).
fn register_c_prof_specs_remaining(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetCurrencyInfoForSkillLine",
        lua.create_function(|lua, _skill: Value| {
            let info = lua.create_table()?;
            info.set("numAvailable", 0i32)?;
            info.set("numTotal", 0i32)?;
            info.set("spentPercentage", 0i32)?;
            Ok(info)
        })?,
    )?;
    t.set(
        "GetConfigIDForSkillLine",
        lua.create_function(|_, _skill: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetChildrenForPath",
        lua.create_function(|lua, _path: Value| lua.create_table())?,
    )?;
    t.set(
        "CanRefundPath",
        lua.create_function(|_, (_path, _cfg): (Value, Value)| Ok(false))?,
    )?;
    t.set(
        "CanUnlockTab",
        lua.create_function(|_, (_tab, _cfg): (Value, Value)| Ok(false))?,
    )?;
    t.set(
        "GetRootPathForTab",
        lua.create_function(|_, _tab: Value| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// C_SettingsUtil namespace - settings loading and panel management.
fn register_c_settings_util(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "NotifySettingsLoaded",
        lua.create_function(|lua, ()| {
            let fire: mlua::Function = lua.globals().get("FireEvent")?;
            fire.call::<()>(lua.create_string("SETTINGS_LOADED")?)?;
            Ok(())
        })?,
    )?;
    t.set(
        "OpenSettingsPanel",
        lua.create_function(|_, _args: mlua::Variadic<Value>| Ok(()))?,
    )?;
    lua.globals().set("C_SettingsUtil", t)?;
    Ok(())
}
