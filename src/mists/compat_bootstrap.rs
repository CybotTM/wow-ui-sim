//! Loader for the mists compatibility Lua bootstrap.
//!
//! Mists-specific stubs for ~46 globals that mists FrameXML/AddOns reference
//! but the simulator's retail-tuned `lua_api/globals/` doesn't register
//! (mostly pre-Cata leftovers MoP kept, plus a few mists-only helpers).

const MISTS_COMPAT_BOOTSTRAP_LUA: &str = include_str!("compat_bootstrap.lua");

pub fn init(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(MISTS_COMPAT_BOOTSTRAP_LUA)?;
    Ok(())
}
