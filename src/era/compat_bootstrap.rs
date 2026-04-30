//! Loader for the era / anniversary compatibility Lua bootstrap.
//!
//! Era and Anniversary both serve vanilla content. The bundled
//! `compat_bootstrap.lua` stubs the ~30 globals the vanilla source repos
//! reference but the simulator's retail-tuned `lua_api/globals/` doesn't
//! register. Loaded by `init_lua_state` under both `client-era` and
//! `client-anniversary`.

const ERA_COMPAT_BOOTSTRAP_LUA: &str = include_str!("compat_bootstrap.lua");

pub fn init(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ERA_COMPAT_BOOTSTRAP_LUA)?;
    Ok(())
}
