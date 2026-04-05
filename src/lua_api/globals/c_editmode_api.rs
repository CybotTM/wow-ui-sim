//! C_EditMode namespace for Edit Mode layout management.
//!
//! Provides the minimum API needed for `EditModeManagerFrame:UpdateLayoutInfo()`
//! to initialize and fire `EDIT_MODE_LAYOUTS_UPDATED`, which unblocks action bar
//! positioning via `UpdateBottomActionBarPositions()`.

use mlua::{Lua, Result, Value};

/// Register the C_EditMode namespace.
pub fn register_c_editmode_api(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    register_layout_queries(lua, &t)?;
    register_account_settings(lua, &t)?;
    register_editmode_noop_stubs(lua, &t)?;
    register_editmode_conversion_stubs(lua, &t)?;
    lua.globals().set("C_EditMode", t)?;
    register_setting_display_info_manager_stub(lua)?;
    Ok(())
}

/// Register a stub EditModeSettingDisplayInfoManager so frames using EditModeSystemMixin
/// can call GetSystemSettingDisplayInfoMap during OnLoad before Blizzard_EditMode loads.
/// The real Blizzard_EditMode/Shared/EditModeSettingDisplayInfo.lua overwrites this.
fn register_setting_display_info_manager_stub(lua: &Lua) -> Result<()> {
    let manager = lua.create_table()?;
    manager.set(
        "GetSystemSettingDisplayInfoMap",
        lua.create_function(|lua, _args: mlua::MultiValue| lua.create_table())?,
    )?;
    manager.set(
        "GetSystemSettingDisplayInfo",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(Value::Nil))?,
    )?;
    manager.set(
        "GetMirroredSettings",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(Value::Nil))?,
    )?;
    lua.globals()
        .set("EditModeSettingDisplayInfoManager", manager)?;
    Ok(())
}

fn register_layout_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetLayouts",
        lua.create_function(|lua, ()| empty_layouts_info(lua))?,
    )?;
    Ok(())
}

fn register_account_settings(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetAccountSettings",
        lua.create_function(|lua, ()| build_account_settings(lua))?,
    )?;
    Ok(())
}

fn register_editmode_noop_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    for name in [
        "SaveLayouts",
        "SetActiveLayout",
        "SetAccountSetting",
        "OnEditModeExit",
        "OnLayoutAdded",
        "OnLayoutDeleted",
    ] {
        t.set(
            name,
            lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
        )?;
    }
    Ok(())
}

fn register_editmode_conversion_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "IsValidLayoutName",
        lua.create_function(|_, _name: Value| Ok(true))?,
    )?;
    t.set(
        "ConvertLayoutInfoToString",
        lua.create_function(|lua, _info: Value| lua.create_string(""))?,
    )?;
    t.set(
        "ConvertStringToLayoutInfo",
        lua.create_function(|_, _s: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "ConvertLayoutInfoToHyperlink",
        lua.create_function(|lua, _info: Value| lua.create_string(""))?,
    )?;
    Ok(())
}

fn empty_layouts_info(lua: &Lua) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("activeLayout", 1)?;
    info.set("layouts", lua.create_table()?)?;
    Ok(info)
}

fn build_account_settings(lua: &Lua) -> Result<mlua::Table> {
    let settings = lua.create_table()?;
    for i in 0..=32 {
        let entry = lua.create_table()?;
        entry.set("setting", i)?;
        entry.set("value", account_setting_default(i))?;
        settings.set(i + 1, entry)?;
    }
    Ok(settings)
}

/// Default value for an account setting enum index.
/// Enum values 0–32 map to EditMode account settings. Most "Show*" = 1 (visible).
fn account_setting_default(setting: i32) -> i32 {
    match setting {
        // ShowGrid = 0
        4 => 0,
        // GridSpacing = 100
        5 => 100,
        // EnableAdvancedOptions = 0
        8 => 0,
        // DeprecatedShowDebuffFrame = 0
        28 => 0,
        // All other Show* settings = 1, SettingsExpanded = 1, EnableSnap = 1
        _ => 1,
    }
}
