//! Dual Lua environment: secureenv / genv.
//!
//! WoW uses two Lua environments: `genv` (addon) and `secureenv` (secure).
//! Addons with `UseSecureEnvironment: 1` in their TOC run in `secureenv`.
//! `Blizzard_EnvironmentCleanup` nils secure APIs from `genv` only —
//! `secureenv` retains them naturally via its own copies.

use mlua::{Lua, Result, Value};

/// Create the secure environment as a shallow copy of `_G`.
///
/// - Copies all current globals into `secureenv`
/// - Creates a separate `Enum` copy (EnvironmentCleanup nils Enum fields directly)
/// - Sets `__index` metatable fallback to `_G` so addon-created globals are visible
/// - Stores the table in the Lua registry as `"__secureenv"`
///
/// Must be called after `register_globals()` but before taint tracking starts.
pub fn create_secure_environment(lua: &Lua) -> Result<()> {
    let secureenv: mlua::Table = lua
        .load(
            r##"
        local genv = _G
        local secureenv = {}
        for k, v in pairs(genv) do
            secureenv[k] = v
        end
        -- Own Enum copy (EnvironmentCleanup nils Enum fields directly)
        if genv.Enum then
            local se = {}
            for k, v in pairs(genv.Enum) do se[k] = v end
            secureenv.Enum = se
        end
        secureenv._G = secureenv
        -- Fallback to genv for globals created after this point (mixins, etc.)
        setmetatable(secureenv, { __index = genv })
        return secureenv
    "##,
        )
        .eval()?;
    lua.set_named_registry_value("__secureenv", secureenv.clone())?;
    // Expose in _G so generated Lua code (mixin lookups, etc.) can resolve
    // globals that were set by UseSecureEnvironment addons.
    lua.globals().raw_set("__secureenv", secureenv)?;
    Ok(())
}

/// Apply `setfenv` to a compiled function so it runs in the secure environment.
///
/// Called before executing Lua files from `UseSecureEnvironment` addons.
pub fn apply_secure_env(lua: &Lua, func: &mlua::Function) -> Result<()> {
    refresh_secure_environment(lua)?;
    let setfenv: mlua::Function = lua.globals().get("setfenv")?;
    let secureenv: mlua::Table = lua.named_registry_value("__secureenv")?;
    setfenv.call::<()>((func.clone(), secureenv))
}

/// Refresh secureenv with the current shared globals from `_G`.
///
/// Secure addons should see the latest Blizzard globals even when `_G` later
/// replaces a table/function that existed at secureenv creation time (for
/// example `NineSliceUtil`). Shared tables are refreshed eagerly, while
/// function names already defined in secureenv are left intact so secure-only
/// implementations are not replaced by public inbound shims with the same name.
fn refresh_secure_environment(lua: &Lua) -> Result<()> {
    let genv = lua.globals();
    let secureenv: mlua::Table = lua.named_registry_value("__secureenv")?;

    for pair in genv.clone().pairs::<Value, Value>() {
        let (key, value) = pair?;
        if let Value::String(s) = &key {
            let key_name = s.to_string_lossy();
            if key_name == "_G" || key_name == "__secureenv" || key_name == "Enum" {
                continue;
            }
        }
        let current: Value = secureenv.raw_get(key.clone())?;
        let should_update = matches!(value, Value::Table(_))
            || matches!(current, Value::Nil)
            || !matches!(current, Value::Function(_));
        if should_update {
            secureenv.raw_set(key, value)?;
        }
    }

    Ok(())
}

/// Set a key/value in both `_G` (genv) and `secureenv`.
///
/// Used when registering named frames so they're accessible from both environments.
pub fn set_in_both_envs(lua: &Lua, key: &str, value: Value) -> Result<()> {
    lua.globals().raw_set(key, value.clone())?;
    if let Ok(secureenv) = lua.named_registry_value::<mlua::Table>("__secureenv") {
        secureenv.raw_set(key, value)?;
    }
    Ok(())
}
