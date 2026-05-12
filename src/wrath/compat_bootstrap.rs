//! Loader for the wrath compatibility Lua bootstrap.
//!
//! The bundled `compat_bootstrap.lua` supplies wrath-only globals (Lua-5.0
//! string/math aliases, frame-reference proxies, ~80 stub API functions)
//! that wrath FrameXML and addons rely on but standard Lua 5.1 / retail
//! WoW don't expose.

const WRATH_COMPAT_BOOTSTRAP_LUA: &str = include_str!("compat_bootstrap.lua");
#[cfg(feature = "client-wrath")]
const WRATH_COMPAT_FRAME_PROXIES_LUA: &str = include_str!("compat_frame_proxies.lua");

/// Loads the wrath/mists-shared compat bootstrap (function stubs and Lua-5.0 aliases).
///
/// Safe to invoke under both `client-wrath` and `client-mists` — every entry
/// is `if rawget(_G, "X") == nil then ... end` so existing definitions from a
/// real `Blizzard_SharedXML` addon take precedence.
pub fn init(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(WRATH_COMPAT_BOOTSTRAP_LUA)?;
    Ok(())
}

/// Wrath-only frame proxies for `MiniMapTrackingIcon` and
/// `PlayerArrowEffectFrame`. Must NOT be loaded under mists — mists's
/// `Blizzard_SharedXML` defines real frames for these names and the proxies
/// would shadow them and cause runaway recursion in error handling.
#[cfg(feature = "client-wrath")]
pub fn init_wrath_only_proxies(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(WRATH_COMPAT_FRAME_PROXIES_LUA)?;
    Ok(())
}
