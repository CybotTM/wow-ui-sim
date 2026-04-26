//! Loader environment (loader_env.rs / env_init.rs counterpart).
//!
//! `compile_chunk_rilua` — compiles Lua source bytes via rilua's pure-Rust
//! compiler (`compile_with_rilua`). Entry point for the rilua-side loading
//! path.

use crate::loader::LoadError;
use crate::loader::lua_file::compile_with_rilua;

use super::secure_env::mark_secure;

/// Compile Lua source bytes via rilua's pure-Rust compiler.
///
/// Equivalent to the mlua `exec` path in `LoaderEnv::exec`, but uses
/// `compile_with_rilua` instead of mlua's `Lua::load`. Returns a
/// `rilua::Function` handle that can be called with `lua.call_function`.
///
/// `chunk_name` should follow the convention: `@Interface/AddOns/Name/File.lua`
/// for file chunks or a descriptive label for inline code.
pub fn compile_chunk_rilua(
    lua: &mut rilua::Lua,
    source: &[u8],
    chunk_name: &str,
) -> Result<rilua::Function, LoadError> {
    compile_with_rilua(lua, source, chunk_name)
}

/// Execute compiled Lua source in the rilua VM, optionally applying the secure
/// environment when `use_secure_env` is true.
///
/// This is the rilua equivalent of `LoaderEnv::exec`.
pub fn exec_chunk_rilua(
    lua: &mut rilua::Lua,
    source: &[u8],
    chunk_name: &str,
    use_secure_env: bool,
) -> Result<(), LoadError> {
    let func = compile_chunk_rilua(lua, source, chunk_name)?;
    if use_secure_env {
        mark_secure(lua, &func).map_err(|e| LoadError::Lua(e.to_string()))?;
    }
    lua.call_function(&func, &[])
        .map_err(|e| LoadError::Lua(e.to_string()))?;
    Ok(())
}
