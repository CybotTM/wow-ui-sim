//! Generated Lua chunk loading for the rilua runtime.

use crate::loader::error::LoadError;
use rilua::LuaApiMut;

/// Compile a generated Lua chunk for the active rilua VM.
pub fn load_chunk<L: LuaApiMut>(
    lua: &mut L,
    code: &str,
    tag: &str,
) -> Result<rilua::Function, LoadError> {
    let chunk_name = format!(
        "@generated/{tag}/{:016x}",
        tagged_hash(code.as_bytes(), tag)
    );
    LuaApiMut::load_bytes(lua, code.as_bytes(), &chunk_name)
        .map_err(|e| LoadError::Lua(e.to_string()))
}

fn tagged_hash(bytes: &[u8], tag: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    tag.hash(&mut hasher);
    hasher.finish()
}
