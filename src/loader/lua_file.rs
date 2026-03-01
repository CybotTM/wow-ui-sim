//! Lua file loading functionality.

use crate::lua_api::LoaderEnv;
use std::path::Path;
use std::time::Instant;

use super::addon::AddonContext;
use super::bytecode_cache;
use super::error::LoadError;
use super::LoadTiming;

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

    let lua_start = Instant::now();
    let func = if ctx.taint {
        // Compile via Lua with setstacktaint so the function carries addon taint.
        // Closures defined inside will inherit this taint (Elune behavior).
        tainted_compile(lua, &bytes, &chunk_name, ctx.name)?
    } else if bytecode_cache::is_disabled() {
        compile_from_source(lua, &bytes, &chunk_name)?
    } else {
        load_cached_or_compile(lua, &bytes, &chunk_name, timing)?
    };

    if ctx.use_secure_env {
        crate::lua_api::secure_env::apply_secure_env(lua, &func)
            .map_err(|e| LoadError::Lua(e.to_string()))?;
    }

    exec_addon_func(lua, func, ctx)?;
    timing.lua_exec_time += lua_start.elapsed();

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

/// Execute a compiled addon function with optional per-addon taint.
///
/// Third-party addon code runs with `debug.setstacktaint(addonName)` so
/// `issecurevariable` tracks the taint source. Blizzard base UI runs
/// securely (no taint).
fn exec_addon_func(
    lua: &mlua::Lua,
    func: mlua::Function,
    ctx: &AddonContext,
) -> Result<(), LoadError> {
    let name = ctx.name.to_string();
    let table = ctx.table.clone();

    if ctx.taint {
        if let Ok(taint_exec) = lua.named_registry_value::<mlua::Function>("__addon_taint_exec") {
            return taint_exec
                .call::<()>((func, name.clone(), name, table))
                .map_err(|e| LoadError::Lua(e.to_string()));
        }
    }

    func.call::<()>((name, table))
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
        // Bytecode found — try loading (may fail if Lua version changed)
        if let Ok(func) = lua
            .load(bytecode.as_slice())
            .set_name(chunk_name)
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

/// Compile source with `setstacktaint(addonName)` so the function carries addon taint.
fn tainted_compile(lua: &mlua::Lua, bytes: &[u8], chunk_name: &str, addon: &str) -> Result<mlua::Function, LoadError> {
    let code = String::from_utf8_lossy(bytes);
    let code = code.strip_prefix('\u{feff}').unwrap_or(&code);
    let compiler: mlua::Function = lua.named_registry_value("__tainted_compile")
        .map_err(|e| LoadError::Lua(e.to_string()))?;
    let (func, err): (Option<mlua::Function>, Option<String>) = compiler
        .call((&*code, chunk_name, addon))
        .map_err(|e| LoadError::Lua(e.to_string()))?;
    func.ok_or_else(|| LoadError::Lua(err.unwrap_or_else(|| "unknown error".into())))
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
        .map_err(|e| LoadError::Lua(e.to_string()))
}
