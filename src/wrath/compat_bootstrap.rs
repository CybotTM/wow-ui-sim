//! Loader for the wrath compatibility Lua bootstrap.
//!
//! The bundled `compat_bootstrap.lua` supplies wrath-only globals (Lua-5.0
//! string/math aliases, frame-reference proxies, ~80 stub API functions)
//! that wrath FrameXML and addons rely on but standard Lua 5.1 / retail
//! WoW don't expose.

const WRATH_COMPAT_BOOTSTRAP_LUA: &str = include_str!("compat_bootstrap.lua");

pub fn init(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(WRATH_COMPAT_BOOTSTRAP_LUA)?;
    Ok(())
}
