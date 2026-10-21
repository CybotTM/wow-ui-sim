//! Environment-local private-aura callback registration and authored dispatch.
//! No authoritative private-aura/message stream is inferred by these handles.

use crate::client_profile::{ACTIVE, ClientProfile};

const UPDATE_CALLBACKS: &str = include_str!("private_aura_callbacks.lua");

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    let modern = matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    );
    lua.exec(&format!("local modern = {modern}\n{UPDATE_CALLBACKS}"))?;
    Ok(())
}
