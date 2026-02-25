//! Locale, region, and build info WoW API functions.

use mlua::{Lua, Result, Value};

/// Register locale, region, and build-related global functions.
pub fn register_locale_api(lua: &Lua) -> Result<()> {
    register_build_info(lua)?;
    register_realm_functions(lua)?;
    register_locale_and_region(lua)?;
    register_client_type_checks(lua)?;
    register_expansion_functions(lua)?;
    register_expansion_constants(lua)?;
    register_glue_functions(lua)?;
    Ok(())
}

/// Register `GetBuildInfo()` - game version info.
fn register_build_info(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    let get_build_info = lua.create_function(|lua, ()| {
        Ok(mlua::MultiValue::from_vec(vec![
            Value::String(lua.create_string("12.0.0")?),
            Value::String(lua.create_string("65655")?),
            Value::String(lua.create_string("Jan 28 2026")?),
            Value::Integer(120000),
            Value::String(lua.create_string("")?),
            Value::String(lua.create_string(" ")?),
        ]))
    })?;
    globals.set("GetBuildInfo", get_build_info)?;

    Ok(())
}

/// Register realm-related functions.
fn register_realm_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "GetRealmName",
        lua.create_function(|lua, ()| Ok(Value::String(lua.create_string("SimulatedRealm")?)))?,
    )?;
    globals.set(
        "GetNormalizedRealmName",
        lua.create_function(|lua, ()| Ok(Value::String(lua.create_string("SimulatedRealm")?)))?,
    )?;
    globals.set("GetRealmID", lua.create_function(|_, ()| Ok(1i32))?)?;

    Ok(())
}

/// Register locale and region functions.
fn register_locale_and_region(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "GetLocale",
        lua.create_function(|lua, ()| Ok(Value::String(lua.create_string("enUS")?)))?,
    )?;
    globals.set("GetCurrentRegion", lua.create_function(|_, ()| Ok(1i32))?)?;
    globals.set(
        "GetCurrentRegionName",
        lua.create_function(|lua, ()| Ok(Value::String(lua.create_string("US")?)))?,
    )?;

    Ok(())
}

/// Register client type check functions.
fn register_client_type_checks(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set("IsMacClient", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsWindowsClient", lua.create_function(|_, ()| Ok(true))?)?;
    globals.set("IsLinuxClient", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsTestBuild", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsBetaBuild", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsPTRClient", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsTrialAccount", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsVeteranTrialAccount", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsPublicTestClient", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsPublicBuild", lua.create_function(|_, ()| Ok(true))?)?;

    Ok(())
}

/// Max player level per expansion index.
fn max_level_for_expansion(expansion: i32) -> i32 {
    match expansion {
        0 => 60,  // Classic
        1 => 70,  // TBC
        2 => 80,  // WotLK
        3 => 85,  // Cata
        4 => 90,  // MoP
        5 => 100, // WoD
        6 => 110, // Legion
        7 => 120, // BfA
        8 => 60,  // Shadowlands (level squish)
        9 => 70,  // Dragonflight
        10 => 80, // The War Within
        _ => 80,
    }
}

/// Validate ClassicExpansionAtLeast/AtMost arg: must be a number in [0, 4294967295].
fn is_valid_expansion_level(level: &Value) -> bool {
    match level {
        Value::Number(n) => *n >= 0.0 && *n <= 4_294_967_295.0,
        Value::Integer(n) => *n >= 0 && *n <= 4_294_967_295,
        _ => false,
    }
}

/// Register expansion level query functions.
fn register_expansion_functions(lua: &Lua) -> Result<()> {
    register_expansion_level_stubs(lua)?;
    register_classic_expansion_checks(lua)
}

/// GetExpansionLevel, GetMaxLevel*, GetServerExpansionLevel, etc.
fn register_expansion_level_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("GetExpansionLevel", lua.create_function(|_, ()| Ok(10))?)?;
    g.set("GetMaxLevelForPlayerExpansion", lua.create_function(|_, ()| Ok(80))?)?;
    g.set("GetMaxPlayerLevel", lua.create_function(|_, ()| Ok(80))?)?;
    g.set("GetMaxLevelForExpansionLevel",
        lua.create_function(|_, expansion: i32| Ok(max_level_for_expansion(expansion)))?)?;
    g.set("GetServerExpansionLevel", lua.create_function(|_, ()| Ok(10))?)?;
    g.set("GetClientDisplayExpansionLevel", lua.create_function(|_, ()| Ok(10))?)?;
    g.set("GetMinimumExpansionLevel", lua.create_function(|_, ()| Ok(0))?)?;
    g.set("GetMaximumExpansionLevel", lua.create_function(|_, ()| Ok(10))?)?;
    g.set("GetAccountExpansionLevel", lua.create_function(|_, ()| Ok(10))?)?;
    g.set("GetAutoCompleteRealms", lua.create_function(|lua, ()| lua.create_table())?)?;
    Ok(())
}

/// ClassicExpansionAtLeast / ClassicExpansionAtMost.
/// Errors on invalid arg; simulates retail WoW (AtLeast=true, AtMost=false).
fn register_classic_expansion_checks(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("ClassicExpansionAtLeast", lua.create_function(|_, level: Value| {
        if !is_valid_expansion_level(&level) {
            return Err(mlua::Error::RuntimeError("assertion failed!".into()));
        }
        Ok(true)
    })?)?;
    g.set("ClassicExpansionAtMost", lua.create_function(|_, level: Value| {
        if !is_valid_expansion_level(&level) {
            return Err(mlua::Error::RuntimeError("assertion failed!".into()));
        }
        Ok(false)
    })?)?;
    Ok(())
}

/// Register expansion level constants (LE_EXPANSION_*).
fn register_expansion_constants(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set("LE_EXPANSION_CLASSIC", 0)?;
    globals.set("LE_EXPANSION_BURNING_CRUSADE", 1)?;
    globals.set("LE_EXPANSION_WRATH_OF_THE_LICH_KING", 2)?;
    globals.set("LE_EXPANSION_CATACLYSM", 3)?;
    globals.set("LE_EXPANSION_MISTS_OF_PANDARIA", 4)?;
    globals.set("LE_EXPANSION_WARLORDS_OF_DRAENOR", 5)?;
    globals.set("LE_EXPANSION_LEGION", 6)?;
    globals.set("LE_EXPANSION_BATTLE_FOR_AZEROTH", 7)?;
    globals.set("LE_EXPANSION_SHADOWLANDS", 8)?;
    globals.set("LE_EXPANSION_DRAGONFLIGHT", 9)?;
    globals.set("LE_EXPANSION_WAR_WITHIN", 10)?;
    globals.set("LE_EXPANSION_LEVEL_CURRENT", 10)?;

    Ok(())
}

/// Register glue screen and login state functions.
fn register_glue_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    let c_glue = lua.create_table()?;
    c_glue.set("IsOnGlueScreen", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("C_Glue", c_glue)?;

    globals.set("InGlue", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsLoggedIn", lua.create_function(|_, ()| Ok(false))?)?;

    Ok(())
}
