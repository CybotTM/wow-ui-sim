//! C_EventUtils namespace.

use mlua::{Lua, Result};

/// Register the C_EventUtils namespace.
pub fn register_c_event_utils_api(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;

    t.set(
        "IsEventValid",
        lua.create_function(|_, event: String| {
            Ok(crate::event::is_valid_event(&event))
        })?,
    )?;

    t.set(
        "IsCallbackEvent",
        lua.create_function(|_, event: String| {
            Ok(crate::event::is_callback_event(&event))
        })?,
    )?;

    t.set("CanPlayerUseEventScheduler", lua.create_function(|_, ()| Ok(false))?)?;

    lua.globals().set("C_EventUtils", t)?;
    Ok(())
}
