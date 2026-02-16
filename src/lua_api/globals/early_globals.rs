//! Early global stubs for functions called during addon loading before
//! the addon that defines them has loaded.
//!
//! These are minimal no-op implementations that prevent nil errors.
//! The real implementations (in workarounds_editmode.rs or the Blizzard
//! Lua addon itself) overwrite these once they load.

use mlua::{Lua, Result, Value};

/// Register early stubs for globals needed during addon XML/Lua loading.
pub fn register_early_globals(lua: &Lua) -> Result<()> {
    register_panel_stubs(lua)?;
    register_mixin_stubs(lua)?;
    Ok(())
}

/// ShowUIPanel / HideUIPanel / CloseAllWindows — called by ChatFrame OnLoad
/// before Blizzard_UIParentPanelManager loads. The workaround code in
/// workarounds_editmode.rs overwrites these with full implementations.
fn register_panel_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set("ShowUIPanel", lua.create_function(|_, _args: mlua::MultiValue| {
        Ok(())
    })?)?;

    globals.set("HideUIPanel", lua.create_function(|_, _args: mlua::MultiValue| {
        Ok(())
    })?)?;

    globals.set("CloseAllWindows", lua.create_function(|_, _args: mlua::MultiValue| {
        Ok(false)
    })?)?;

    // UIPanelWindows registry table — addons register panels here before
    // ShowUIPanel/HideUIPanel reference it.
    if globals.get::<Value>("UIPanelWindows")? == Value::Nil {
        globals.set("UIPanelWindows", lua.create_table()?)?;
    }

    Ok(())
}

/// Pre-register mixin tables with no-op methods for mixins referenced
/// by XML templates before their defining Lua file loads.
fn register_mixin_stubs(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        -- AnimatedStatusBarMixin.OnLoad — called by AnimatedStatusBarTemplate
        -- <OnLoad>self:OnLoad()</OnLoad>. Overwritten by AnimatedStatusBar.lua.
        if not AnimatedStatusBarMixin then
            AnimatedStatusBarMixin = { OnLoad = function() end }
        end

        -- StoreTooltipBackdropMixin.StoreTooltipOnLoad — called by
        -- StoreTooltipBackdrop template. Overwritten by
        -- Blizzard_Shared_StoreUITemplates.lua.
        if not StoreTooltipBackdropMixin then
            StoreTooltipBackdropMixin = { StoreTooltipOnLoad = function() end }
        end
    "#,
    )
    .exec()
}
