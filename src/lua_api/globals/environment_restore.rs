//! Post-EnvironmentCleanup fixups.
//!
//! Blizzard_EnvironmentCleanup nil's capsule APIs (C_AuthChallenge,
//! C_SecureTransfer, C_StoreSecure, C_WowTokenSecure, CreateForbiddenFrame,
//! loadstring_untainted, secretunwrap, SecureMixin, etc.) from `_G`.
//! Most should stay nil — they only exist in secureenv for
//! UseSecureEnvironment addons.
//!
//! Exception: `CreateSecureDelegate` is used by non-secure Blizzard addons
//! that load after cleanup (Blizzard_Menu, Blizzard_SharedXMLGame/
//! TooltipDataHandler). In real WoW this is a C engine function available
//! to all Blizzard code; we restore it as an identity function (no taint
//! system).

use mlua::{Lua, Result};

/// Restore globals that non-secure Blizzard addons need after cleanup.
///
/// Must be called after Blizzard_EnvironmentCleanup loads, before any
/// addon that depends on these globals.
pub fn restore_post_cleanup_globals(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "CreateSecureDelegate",
        lua.create_function(|_, func: mlua::Function| Ok(func))?,
    )?;
    Ok(())
}
