//! WoW Enum table containing game enumerations.
//!
//! This module registers the global `Enum` table which contains various game
//! enumerations used by addons, such as item quality, quest types, UI widget
//! types, and other game constants.

use super::enum_data::{EXPLICIT_ENUMS, SEQUENTIAL_ENUMS};
use mlua::{Lua, Result};

/// Auto-generated Lua code that registers missing WoW client enums.
const MISSING_ENUMS_LUA: &str = include_str!("enum_data/missing_enums.lua");

/// Register the Enum table with all WoW game enumerations.
pub fn register_enum_api(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let enum_table = lua.create_table()?;

    // Register enums from data file
    register_sequential_enums(lua, &enum_table)?;
    register_explicit_enums(lua, &enum_table)?;

    globals.set("Enum", enum_table)?;

    // Load auto-generated missing enums from globals.yaml (1568 enums).
    // Uses `if not Enum.X then` guards so existing Rust-registered enums take priority.
    lua.load(MISSING_ENUMS_LUA).set_name("missing_enums").exec()?;

    Ok(())
}

/// Register all sequential enums (values are 0, 1, 2, ...).
fn register_sequential_enums(lua: &Lua, enum_table: &mlua::Table) -> Result<()> {
    for (name, variants) in SEQUENTIAL_ENUMS {
        let table = lua.create_table()?;
        for (i, variant) in variants.iter().enumerate() {
            table.set(*variant, i as i32)?;
        }
        enum_table.set(*name, table)?;
    }
    Ok(())
}

/// Register all explicit value enums (values are explicitly specified).
fn register_explicit_enums(lua: &Lua, enum_table: &mlua::Table) -> Result<()> {
    for (name, variants) in EXPLICIT_ENUMS {
        let table = lua.create_table()?;
        for (variant, value) in *variants {
            table.set(*variant, *value)?;
        }
        enum_table.set(*name, table)?;
    }
    Ok(())
}
