//! FadingFrame_* global function stubs used by ZoneText.lua.

use mlua::{Lua, Result, Value};

/// Register FadingFrame_* globals and addframetext.
pub fn register_fading_frame_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "FadingFrame_OnLoad",
        lua.create_function(fading_frame_on_load)?,
    )?;
    g.set(
        "FadingFrame_SetFadeInTime",
        lua.create_function(|_, (_frame, _t): (Value, f64)| Ok(()))?,
    )?;
    g.set(
        "FadingFrame_SetHoldTime",
        lua.create_function(|_, (_frame, _t): (Value, f64)| Ok(()))?,
    )?;
    g.set(
        "FadingFrame_SetFadeOutTime",
        lua.create_function(|_, (_frame, _t): (Value, f64)| Ok(()))?,
    )?;
    g.set(
        "FadingFrame_Show",
        lua.create_function(|_, _frame: Value| Ok(()))?,
    )?;
    g.set(
        "GetErrorCallstackHeight",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "SetChatWindowShown",
        lua.create_function(|_, (_id, _shown): (Value, Value)| Ok(()))?,
    )?;
    // Native WoW error display function — called by Blizzard_ScriptErrors error handler.
    // Without this stub, the error handler itself crashes, causing recursive error spam.
    g.set(
        "addframetext",
        lua.create_function(|lua, msg: Value| {
            let msg_str = match &msg {
                Value::String(s) => s.to_string_lossy().to_string(),
                Value::Integer(i) => i.to_string(),
                Value::Number(n) => n.to_string(),
                _ => lua
                    .load("return tostring(...)")
                    .call::<String>(msg)
                    .unwrap_or_else(|_| "<error>".to_string()),
            };
            eprintln!("[addframetext] {msg_str}");
            super::super::script_helpers::collect_lua_error(lua, &msg_str);
            Ok(())
        })?,
    )?;
    Ok(())
}

/// FadingFrame_OnLoad: initializes fading state fields on the frame or table.
fn fading_frame_on_load(lua: &Lua, frame: Value) -> Result<()> {
    match &frame {
        Value::UserData(_) => {
            if let Some(id) = crate::lua_api::frame::extract_frame_id(&frame) {
                let fields = crate::lua_api::script_helpers::get_or_create_frame_fields(lua, id);
                fields.set("fadeInTime", 0.0f64)?;
                fields.set("fadeOutTime", 0.0f64)?;
                fields.set("holdTime", 0.0f64)?;
            }
        }
        Value::Table(t) => {
            t.set("fadeInTime", 0.0f64)?;
            t.set("fadeOutTime", 0.0f64)?;
            t.set("holdTime", 0.0f64)?;
        }
        _ => {}
    }
    Ok(())
}
