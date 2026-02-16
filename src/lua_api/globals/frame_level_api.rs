//! Global frame level helpers: `RaiseFrameLevel`, `LowerFrameLevel`, `RaiseFrameLevelByTwo`.
//!
//! These are called from XML `<OnLoad>` handlers in 27 Blizzard files before the
//! Lua addon that normally defines them (`Blizzard_UIParentPanelManager`) loads.

use crate::lua_api::frame::{get_sim_state, lud_to_id, propagate_strata_level_pub};
use mlua::{LightUserData, Lua, Result};

/// Register `RaiseFrameLevel`, `LowerFrameLevel`, and `RaiseFrameLevelByTwo` globals.
pub fn register_frame_level_helpers(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set("RaiseFrameLevel", lua.create_function(|lua, ud: LightUserData| {
        adjust_frame_level(lua, ud, 1)
    })?)?;

    globals.set("LowerFrameLevel", lua.create_function(|lua, ud: LightUserData| {
        adjust_frame_level(lua, ud, -1)
    })?)?;

    globals.set("RaiseFrameLevelByTwo", lua.create_function(|lua, ud: LightUserData| {
        adjust_frame_level(lua, ud, 2)
    })?)?;

    Ok(())
}

fn adjust_frame_level(lua: &Lua, ud: LightUserData, delta: i32) -> Result<()> {
    let id = lud_to_id(ud);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.frame_level += delta;
        frame.has_fixed_frame_level = true;
    }
    propagate_strata_level_pub(&mut state.widgets, id);
    Ok(())
}
