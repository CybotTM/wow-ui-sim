//! Lua file loading functionality.

use crate::lua_api::LoaderEnv;
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
    let lua = env.lua();

    let compile_start = Instant::now();
    let func_result = if bytecode_cache::is_disabled() {
        compile_from_source(lua, &bytes, &chunk_name)
    } else {
        load_cached_or_compile(lua, &bytes, &chunk_name, timing)
    };
    let compile_elapsed = compile_start.elapsed();
    timing.lua_compile_time += compile_elapsed;
    timing.lua_exec_time += compile_elapsed;
    let func = func_result?;

    let call_start = Instant::now();
    // Stamp addon taint on the compiled function's GC header.
    // When the VM executes it, fixedtaint = cl->taint blocks read-propagation
    // and inner closures inherit via writetaint.
    if ctx.taint {
        set_object_taint(lua, &func, ctx.name);
    }

    if ctx.use_secure_env {
        crate::lua_api::secure_env::apply_secure_env(lua, &func)
            .map_err(|e| report_lua_load_error(lua, e))?;
    }

    let exec_result = exec_addon_func(lua, func, ctx).inspect_err(|e| {
        if let LoadError::Lua(msg) = &e {
            crate::lua_api::script_helpers::call_error_handler(lua, msg);
        }
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
    _lua: &mlua::Lua,
    func: mlua::Function,
    ctx: &AddonContext,
) -> Result<(), LoadError> {
    let table = ctx.table.clone();
    func.call::<()>((ctx.lua_name.clone(), table))
        .map_err(|e| LoadError::Lua(e.to_string()))
}

/// Try loading from bytecode cache; compile and cache on miss.
fn load_cached_or_compile(
    lua: &mlua::Lua,
    bytes: &[u8],
    chunk_name: &str,
    timing: &mut LoadTiming,
) -> Result<mlua::Function, LoadError> {
    let hash = bytecode_cache::content_hash(bytes, chunk_name);

    if let Some(bytecode) = bytecode_cache::get(hash) {
        // Bytecode already embeds its original chunk name. Reassigning a name here
        // causes mlua/Lua 5.1 binary loads to fail, so cached chunks must be loaded
        // as-is.
        if let Ok(func) = lua
            .load(bytecode.as_slice())
            .set_mode(mlua::ChunkMode::Binary)
            .into_function()
        {
            timing.cache_hits += 1;
            return Ok(func);
        }
    }

    // Cache miss or invalid bytecode — compile from source
    timing.cache_misses += 1;
    let func = compile_from_source(lua, bytes, chunk_name)?;
    let bc = func.dump(false);
    bytecode_cache::put(hash, &bc);
    Ok(func)
}

/// Set taint on a Lua function's GC object header via `debug.setobjecttaint`.
fn set_object_taint(lua: &mlua::Lua, func: &mlua::Function, taint: &str) {
    if let Ok(sot) = lua.named_registry_value::<mlua::Function>("__setobjecttaint") {
        let _ = sot.call::<()>((func.clone(), taint));
    }
}

fn report_lua_load_error(lua: &mlua::Lua, err: impl ToString) -> LoadError {
    let msg = err.to_string();
    crate::lua_api::script_helpers::call_error_handler(lua, &msg);
    LoadError::Lua(msg)
}

/// Compile Lua source code into a function.
fn compile_from_source(
    lua: &mlua::Lua,
    bytes: &[u8],
    chunk_name: &str,
) -> Result<mlua::Function, LoadError> {
    let code = String::from_utf8_lossy(bytes);
    // Strip UTF-8 BOM if present (common in Windows-edited files)
    let code = code.strip_prefix('\u{feff}').unwrap_or(&code);
    lua.load(code)
        .set_name(chunk_name)
        .into_function()
        .map_err(|e| report_lua_load_error(lua, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dumped_bytecode_round_trips_with_same_lua_state() {
        let lua = unsafe { mlua::Lua::unsafe_new() };
        let chunk_name = "@Interface/AddOns/TestAddon/Test.lua";
        let func = compile_from_source(&lua, b"return 40 + 2", chunk_name)
            .expect("source chunk should compile");
        let bytecode = func.dump(false);

        let loaded = lua
            .load(bytecode.as_slice())
            .set_mode(mlua::ChunkMode::Binary)
            .into_function();
        if let Err(err) = loaded {
            panic!("bytecode should load in same Lua state: {err}");
        }
    }

    #[test]
    fn dumped_bytecode_round_trips_with_fresh_lua_state() {
        let source_lua = unsafe { mlua::Lua::unsafe_new() };
        let chunk_name = "@Interface/AddOns/TestAddon/Test.lua";
        let func = compile_from_source(&source_lua, b"return 40 + 2", chunk_name)
            .expect("source chunk should compile");
        let bytecode = func.dump(false);

        let fresh_lua = unsafe { mlua::Lua::unsafe_new() };
        let loaded = fresh_lua
            .load(bytecode.as_slice())
            .set_mode(mlua::ChunkMode::Binary)
            .into_function();
        if let Err(err) = loaded {
            panic!("bytecode should load in fresh Lua state: {err}");
        }
    }
}
