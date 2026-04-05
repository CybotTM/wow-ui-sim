//! Dual Lua environment: secureenv / genv.
//!
//! WoW uses two Lua environments: `genv` (addon) and `secureenv` (secure).
//! Addons with `UseSecureEnvironment: 1` in their TOC run in `secureenv`.
//! `Blizzard_EnvironmentCleanup` nils secure APIs from `genv` only —
//! `secureenv` retains them naturally via its own copies.

use mlua::{Lua, Result, Value};

/// Create the secure environment as a shallow copy of `_G` with fallback.
///
/// Copies all current globals so EnvironmentCleanup can nil APIs from `_G`
/// without affecting secureenv. Also creates a separate Enum copy (cleanup
/// nils individual Enum fields). Sets `__index = _G` fallback so globals
/// defined after creation (addons, mixins, frame names) are visible.
///
/// No refresh is needed — the initial copy preserves base APIs, and
/// `__index` handles everything added later.
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
        -- Fallback to genv for globals created after this point
        setmetatable(secureenv, { __index = genv })
        return secureenv
    "##,
        )
        .eval()?;
    lua.set_named_registry_value("__secureenv", secureenv.clone())?;
    lua.globals().raw_set("__secureenv", secureenv)?;
    Ok(())
}

/// Apply `setfenv` to a compiled function so it runs in the secure environment.
///
/// Called before executing Lua from `UseSecureEnvironment` addons.
/// No refresh needed — secureenv uses `__index = _G` so all globals are
/// always visible through the metatable fallback.
pub fn apply_secure_env(lua: &Lua, func: &mlua::Function) -> Result<()> {
    let setfenv: mlua::Function = lua.globals().get("setfenv")?;
    let secureenv: mlua::Table = lua.named_registry_value("__secureenv")?;
    setfenv.call::<()>((func.clone(), secureenv))
}

/// Snapshot secure APIs into secureenv before EnvironmentCleanup nils them
/// from `_G`. After cleanup, `__index = _G` would return nil for these,
/// so we store them directly in secureenv.
///
/// Also snapshots Enum (EnvironmentCleanup nils individual Enum fields).
pub fn snapshot_secure_apis(lua: &Lua) -> Result<()> {
    lua.load(
        r##"
        local secureenv = __secureenv
        local genv = _G
        -- Snapshot Enum (EnvironmentCleanup nils individual fields)
        if genv.Enum then
            local se = {}
            for k, v in pairs(genv.Enum) do se[k] = v end
            rawset(secureenv, "Enum", se)
        end
        -- Snapshot capsule APIs that EnvironmentCleanup will nil
        local secure_apis = {
            "C_AuthChallenge", "C_SecureTransfer", "C_StoreSecure",
            "C_WowTokenSecure", "C_WowTokenPublic", "C_AccountStore",
            "CreateForbiddenFrame", "CreateSecureDelegate",
            "loadstring_untainted", "secretunwrap", "SecureMixin",
            "NineSliceUtil", "NineSlicePanelMixin",
        }
        for _, name in ipairs(secure_apis) do
            local v = rawget(genv, name)
            if v ~= nil then
                rawset(secureenv, name, v)
            end
        end
    "##,
    )
    .exec()?;
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
