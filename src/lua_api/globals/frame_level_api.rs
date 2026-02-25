//! Global frame level helpers: `RaiseFrameLevel`, `LowerFrameLevel`, `RaiseFrameLevelByTwo`.
//!
//! These are called from XML `<OnLoad>` handlers in 27 Blizzard files before the
//! Lua addon that normally defines them (`Blizzard_UIParentPanelManager`) loads.

use crate::lua_api::frame::extract_frame_id;
use crate::lua_api::frame::{get_sim_state, propagate_strata_level_pub};
use mlua::{Lua, Result, Value};

/// Register `RaiseFrameLevel`, `LowerFrameLevel`, and `RaiseFrameLevelByTwo` globals.
pub fn register_frame_level_helpers(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set("RaiseFrameLevel", lua.create_function(|lua, v: Value| {
        adjust_frame_level(lua, v, 1)
    })?)?;

    globals.set("LowerFrameLevel", lua.create_function(|lua, v: Value| {
        adjust_frame_level(lua, v, -1)
    })?)?;

    globals.set("RaiseFrameLevelByTwo", lua.create_function(|lua, v: Value| {
        adjust_frame_level(lua, v, 2)
    })?)?;

    Ok(())
}

fn adjust_frame_level(lua: &Lua, value: Value, delta: i32) -> Result<()> {
    let id = extract_frame_id(&value).ok_or_else(|| {
        mlua::Error::runtime("RaiseFrameLevel/LowerFrameLevel: expected frame")
    })?;
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.frame_level += delta;
        frame.has_fixed_frame_level = true;
    }
    propagate_strata_level_pub(&mut state.widgets, id);
    Ok(())
}
