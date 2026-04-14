//! Lua file loading functionality.

use crate::lua_api::LoaderEnv;
use crate::lua_api::globals::rilua_security::apply_secure_env_rilua;
use crate::lua_api::rilua_methods::create_string;
use crate::lua_api::rilua_script_helpers::call_error_handler;
use crate::lua_api::rilua_taint::stamp_addon_taint;
use rilua::LuaApiMut;
use std::path::Path;
use std::time::Instant;

use super::LoadTiming;
use super::addon::AddonContext;
use super::bytecode_cache;
use super::error::LoadError;

/// Load a Lua file into the environment with addon varargs.
pub fn load_lua_file(
    env: &LoaderEnv<'_>,
    path: &Path,
    ctx: &AddonContext,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let io_start = Instant::now();
    let bytes = std::fs::read(path)?;
    timing.io_time += io_start.elapsed();

    let chunk_name = wow_chunk_name(path);

    let compile_start = Instant::now();
    let mut lua = env.rilua_mut();
    let func_result = load_cached_or_compile(&mut lua, &bytes, &chunk_name, timing);
    let compile_elapsed = compile_start.elapsed();
    timing.lua_compile_time += compile_elapsed;
    timing.lua_exec_time += compile_elapsed;
    let func = func_result?;

    let call_start = Instant::now();
    // Stamp addon taint on the compiled function's GC header.
    // When the VM executes it, fixedtaint = cl->taint blocks read-propagation
    // and inner closures inherit via writetaint.
    if ctx.taint {
        set_object_taint(&mut lua, &func, ctx.name);
    }

    if ctx.use_secure_env {
        apply_secure_env_rilua(&mut lua, &func).map_err(|e| report_lua_load_error(&mut lua, e))?;
    }

    let exec_result = exec_addon_func(&mut lua, func, ctx).map_err(|e| {
        if let LoadError::Lua(msg) = &e {
            call_error_handler(&mut lua, msg);
        }
        e
    });
    let call_elapsed = call_start.elapsed();
    timing.lua_call_time += call_elapsed;
    timing.lua_exec_time += call_elapsed;
    exec_result?;

    Ok(())
}

/// Transform path to WoW-style chunk name for debugstack.
fn wow_chunk_name(path: &Path) -> String {
    let path_str = path.display().to_string();
    if let Some(pos) = path_str.find("AddOns/") {
        format!("@Interface/{}", &path_str[pos..])
    } else {
        format!("@{}", path_str)
    }
}

/// Execute a compiled addon function.
/// Taint is already stamped on the function's GC header by the caller.
fn exec_addon_func(
    lua: &mut rilua::Lua,
    func: rilua::Function,
    ctx: &AddonContext,
) -> Result<(), LoadError> {
    let name = create_string(lua.state_mut(), ctx.name);
    lua.call_function(&func, &[name, ctx.table])
        .map(|_| ())
        .map_err(|e| LoadError::Lua(e.to_string()))
}

/// Try loading from bytecode cache; compile and cache on miss.
fn load_cached_or_compile(
    lua: &mut rilua::Lua,
    bytes: &[u8],
    chunk_name: &str,
    timing: &mut LoadTiming,
) -> Result<rilua::Function, LoadError> {
    if !bytecode_cache::is_disabled() {
        timing.cache_misses += 1;
    }
    compile_from_source(lua, bytes, chunk_name)
}

/// Set taint on a Lua function's GC object header via `debug.setobjecttaint`.
fn set_object_taint(lua: &mut rilua::Lua, func: &rilua::Function, taint: &str) {
    stamp_addon_taint(lua, func, taint);
}

fn report_lua_load_error(lua: &mut rilua::Lua, err: impl ToString) -> LoadError {
    let msg = err.to_string();
    call_error_handler(lua, &msg);
    LoadError::Lua(msg)
}

/// Compile Lua source code into a function.
fn compile_from_source(
    lua: &mut rilua::Lua,
    bytes: &[u8],
    chunk_name: &str,
) -> Result<rilua::Function, LoadError> {
    compile_with_rilua(lua, bytes, chunk_name).map_err(|e| report_lua_load_error(lua, e))
}

/// Compile Lua source code using rilua's compiler (pure Rust).
///
/// This is the rilua-side equivalent of `compile_from_source`. It compiles
/// source code and returns a rilua Function handle. Used as a parallel
/// compilation path for Phase 3 migration — the mlua path remains active
/// until the full VM switch.
pub fn compile_with_rilua(
    lua: &mut rilua::Lua,
    bytes: &[u8],
    chunk_name: &str,
) -> Result<rilua::Function, LoadError> {
    use rilua::LuaApiMut;
    // Strip UTF-8 BOM if present
    let bytes = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &bytes[3..]
    } else {
        bytes
    };
    lua.load_bytes(bytes, chunk_name)
        .map_err(|e| LoadError::Lua(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rilua_compilation_matches_source_semantics() {
        let mut lua = rilua::Lua::new().unwrap();
        let func = compile_with_rilua(&mut lua, b"return 40 + 2", "@test")
            .expect("rilua should compile simple expression");
        let results = lua.call_function(&func, &[]).unwrap();
        assert_eq!(results, vec![rilua::Val::Num(42.0)]);
    }

    #[test]
    fn rilua_compilation_strips_bom() {
        let mut lua = rilua::Lua::new().unwrap();
        let source = b"\xEF\xBB\xBFreturn 1";
        let func = compile_with_rilua(&mut lua, source, "@bom_test")
            .expect("rilua should handle BOM-prefixed source");
        let results = lua.call_function(&func, &[]).unwrap();
        assert_eq!(results, vec![rilua::Val::Num(1.0)]);
    }
}
