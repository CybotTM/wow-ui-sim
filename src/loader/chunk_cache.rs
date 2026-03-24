//! Bytecode-backed cache for generated Lua chunks used during XML/template loading.

use super::bytecode_cache;
use mlua::{ChunkMode, Function, Lua, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Compile a generated Lua chunk, reusing cached bytecode when enabled.
pub fn load_chunk(lua: &Lua, code: &str, tag: &str) -> Result<Function> {
    let hash = tagged_hash(code.as_bytes(), tag);
    let chunk_name = format!("@generated/{tag}/{hash:016x}");

    if !bytecode_cache::is_disabled()
        && let Some(bytecode) = bytecode_cache::get(hash)
        && let Ok(func) = lua
            .load(bytecode.as_slice())
            .set_mode(ChunkMode::Binary)
            .into_function()
    {
        return Ok(func);
    }

    let func = lua.load(code).set_name(&chunk_name).into_function()?;
    if !bytecode_cache::is_disabled() {
        bytecode_cache::put(hash, &func.dump(false));
    }
    Ok(func)
}

/// Execute a generated Lua chunk using the bytecode cache.
pub fn exec(lua: &Lua, code: &str, tag: &str) -> Result<()> {
    load_chunk(lua, code, tag)?.call::<()>(())
}

fn tagged_hash(bytes: &[u8], tag: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    tag.hash(&mut hasher);
    hasher.finish()
}
