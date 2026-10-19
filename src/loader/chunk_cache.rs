//! Generated Lua chunk loading for the rilua runtime.

use crate::loader::bytecode_cache;
use crate::loader::error::LoadError;
use rilua::LuaApiMut;

/// Compile a generated Lua chunk for the active rilua VM.
pub fn load_chunk<L: LuaApiMut>(
    lua: &mut L,
    code: &str,
    tag: &str,
) -> Result<rilua::Function, LoadError> {
    let hash = bytecode_cache::content_hash(code.as_bytes(), tag);
    let profile_start = generated_chunk_profile_start();
    let chunk_name = format!("@generated/{tag}/{hash:016x}");

    if !bytecode_cache::is_disabled() {
        if let Some(result) = bytecode_cache::with_cached_bytecode(hash, |bytecode| {
            LuaApiMut::load_bytes(lua, bytecode, &chunk_name)
        }) && let Ok(func) = result
        {
            profile_generated_chunk(profile_start, tag, hash, code.len(), "replay");
            return Ok(func);
        }
    }

    let func = LuaApiMut::load_bytes(lua, code.as_bytes(), &chunk_name)
        .map_err(|e| LoadError::Lua(e.to_string()))?;
    if !bytecode_cache::is_disabled() {
        let bytecode = crate::loader::bytecode::dump_function(lua.state_mut(), &func)?;
        bytecode_cache::put(hash, &bytecode);
    }
    profile_generated_chunk(profile_start, tag, hash, code.len(), "compile");
    Ok(func)
}

pub(crate) fn load_profiled_chunk<L: LuaApiMut>(
    lua: &mut L,
    code: &str,
    tag: &str,
) -> rilua::LuaResult<rilua::Function> {
    let started = generated_chunk_profile_start();
    let result = LuaApiMut::load_bytes(lua, code.as_bytes(), "=(string)");
    if started.is_some() {
        let hash = bytecode_cache::content_hash(code.as_bytes(), tag);
        profile_generated_chunk(started, tag, hash, code.len(), "compile");
    }
    result
}

pub(crate) fn generated_chunk_profile_start() -> Option<std::time::Instant> {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    ENABLED
        .get_or_init(|| std::env::var_os("WOW_SIM_PROFILE_GENERATED_CHUNKS").is_some())
        .then(std::time::Instant::now)
}

pub(crate) fn profile_generated_chunk(
    started: Option<std::time::Instant>,
    tag: &str,
    hash: u64,
    bytes: usize,
    mode: &str,
) {
    if let Some(started) = started {
        crate::logging::eprintln_elapsed(&format!(
            "[generated-chunk] tag={tag} key={hash:016x} bytes={bytes} mode={mode} us={}",
            started.elapsed().as_micros()
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_tag() -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        format!("chunk-cache-{}-{nanos}", std::process::id())
    }

    #[test]
    fn load_chunk_stores_bytecode_for_generated_chunks() {
        let tag = unique_tag();
        let code = format!("return {:?}", tag);
        let hash = crate::loader::bytecode_cache::content_hash(code.as_bytes(), &tag);

        let mut lua = rilua::Lua::new().unwrap();
        let func = load_chunk(&mut lua, &code, &tag).expect("generated chunk should compile");
        let results = lua.call_function(&func, &[]).unwrap();
        let value = results.into_iter().next().expect("chunk returns a value");
        assert_eq!(lua.val_as_bytes(value).unwrap(), tag.as_bytes());
        let cached = crate::loader::bytecode_cache::with_cached_bytecode(hash, |_| ());
        assert!(
            cached.is_some(),
            "generated chunk should be written to bytecode cache"
        );
    }
}
