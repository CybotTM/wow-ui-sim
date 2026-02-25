//! Re-register globals wiped by Blizzard's environment cleanup files.
//!
//! Several Blizzard files (e.g. Blizzard_SecureLib) nil out globals as part of
//! the WoW security model after addon loading.  By the time the Wowless test
//! suite runs, these are gone.  This module re-registers them in the workarounds
//! phase so they are available to test code.

use mlua::{Lua, MultiValue, Result, Value};

/// Re-register globals wiped by Blizzard's environment cleanup during addon loading.
pub fn restore_environment_cleanup_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    restore_c_macro_extra(lua, &g)?;
    restore_create_forbidden_frame(lua, &g)?;
    restore_create_secure_delegate(lua, &g)?;
    restore_loadstring_untainted(lua, &g)?;
    restore_secretunwrap(lua, &g)?;
    // Re-register generated stubs to restore C_* namespaces cleared by Blizzard's
    // security model (e.g. C_AuthChallenge, C_SecureTransfer, C_StoreSecure,
    // C_WowTokenSecure). Each stub already checks is_nil(), so surviving globals
    // are left untouched.
    super::generated_stubs::register_generated_stubs(lua)?;
    Ok(())
}

/// Re-register C_Macro.SetMacroExecuteLineCallback if it was wiped.
fn restore_c_macro_extra(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t: mlua::Table = match g.get::<Value>("C_Macro")? {
        Value::Table(t) => t,
        _ => {
            let t = lua.create_table()?;
            g.set("C_Macro", t.clone())?;
            t
        }
    };
    if t.get::<Value>("SetMacroExecuteLineCallback")?.is_nil() {
        t.set(
            "SetMacroExecuteLineCallback",
            lua.create_function(|_, _: MultiValue| Ok(()))?,
        )?;
    }
    Ok(())
}

/// CreateForbiddenFrame — delegates to CreateFrame.
///
/// Forbidden frames are a taint/access-control concept; the simulator treats
/// them as ordinary frames.
fn restore_create_forbidden_frame(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if g.get::<Value>("CreateForbiddenFrame")?.is_nil() {
        g.set(
            "CreateForbiddenFrame",
            lua.create_function(|lua, args: MultiValue| {
                let create_frame: mlua::Function = lua.globals().get("CreateFrame")?;
                create_frame.call::<MultiValue>(args)
            })?,
        )?;
    }
    Ok(())
}

/// CreateSecureDelegate — no taint model in the simulator; return a no-op callable.
fn restore_create_secure_delegate(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if g.get::<Value>("CreateSecureDelegate")?.is_nil() {
        g.set(
            "CreateSecureDelegate",
            lua.create_function(|lua, _: MultiValue| {
                Ok(Value::Function(
                    lua.create_function(|_, _: MultiValue| Ok(()))?,
                ))
            })?,
        )?;
    }
    Ok(())
}

/// loadstring_untainted — compile Lua without marking the chunk as tainted.
///
/// The `loadstring` global in env.rs is wrapped to force taint on all
/// addon-loaded code.  This variant uses mlua's load() directly, bypassing
/// that wrapper, matching the WoW engine's untainted loadstring behaviour.
fn restore_loadstring_untainted(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if g.get::<Value>("loadstring_untainted")?.is_nil() {
        g.set(
            "loadstring_untainted",
            lua.create_function(
                |lua, (code, name): (String, Option<String>)| {
                    let chunk = lua.load(code.as_str());
                    let chunk = match name {
                        Some(n) => chunk.set_name(n),
                        None => chunk,
                    };
                    match chunk.into_function() {
                        Ok(f) => Ok((Value::Function(f), Value::Nil)),
                        Err(e) => Ok((
                            Value::Nil,
                            Value::String(lua.create_string(e.to_string().as_str())?),
                        )),
                    }
                },
            )?,
        )?;
    }
    Ok(())
}

/// secretunwrap — returns its first argument unchanged.
///
/// Used by the secure transfer system to "unwrap" a value from a secure
/// context.  No-op in the simulator.
fn restore_secretunwrap(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if g.get::<Value>("secretunwrap")?.is_nil() {
        g.set(
            "secretunwrap",
            lua.create_function(|_, mut args: MultiValue| {
                Ok(args.pop_front().unwrap_or(Value::Nil))
            })?,
        )?;
    }
    Ok(())
}
