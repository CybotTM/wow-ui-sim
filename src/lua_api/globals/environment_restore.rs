//! Post-EnvironmentCleanup fixups.
//!
//! Blizzard_EnvironmentCleanup nil's capsule APIs (C_AuthChallenge,
//! C_SecureTransfer, C_StoreSecure, C_WowTokenSecure, CreateForbiddenFrame,
//! CreateSecureDelegate, loadstring_untainted, secretunwrap, SecureMixin, etc.)
//! from the addon environment.  These should stay nil — they only exist in
//! secureenv for UseSecureEnvironment addons.
//!
//! Previously this module re-registered them, which was wrong.

use mlua::{Lua, Result};

/// No-op — capsule APIs should stay nil'd after EnvironmentCleanup.
pub fn restore_environment_cleanup_stubs(_lua: &Lua) -> Result<()> {
    Ok(())
}
