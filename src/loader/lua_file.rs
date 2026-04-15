//! Lua file loading functionality.

use crate::lua_api::LoaderEnv;
use crate::lua_api::globals::rilua_security::mark_secure_state;
use crate::lua_api::rilua_methods::create_string;
use crate::lua_api::rilua_script_helpers::call_error_handler_state;
use crate::lua_api::rilua_taint::stamp_addon_taint_state;
use rilua::LuaApiMut;
use rilua::vm::state::LuaState;
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
    let func_result =
        env.with_state(|state| load_cached_or_compile(state, &bytes, &chunk_name, timing));
    let compile_elapsed = compile_start.elapsed();
    timing.lua_compile_time += compile_elapsed;
    timing.lua_exec_time += compile_elapsed;
    let func = func_result?;

    let call_start = Instant::now();
    // Stamp addon taint on the compiled function's GC header.
    // When the VM executes it, fixedtaint = cl->taint blocks read-propagation
    // and inner closures inherit via writetaint.
    let exec_result = env.with_state(|state| {
        if ctx.taint {
            set_object_taint(state, &func, ctx.name);
        }
        if ctx.use_secure_env {
            mark_secure_state(state, &func).map_err(|e| report_lua_load_error(state, e))?;
        }
        exec_addon_func(state, func, ctx).map_err(|e| {
            if let LoadError::Lua(msg) = &e {
                call_error_handler_state(state, msg);
            }
            e
        })
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
    state: &mut LuaState,
    func: rilua::Function,
    ctx: &AddonContext,
) -> Result<(), LoadError> {
    let name = create_string(state, ctx.name);
    crate::lua_api::rilua_methods::call_function_state(
        state,
        rilua::Val::Function(func.gc_ref()),
        &[name, ctx.table],
    )
    .map(|_| ())
    .map_err(|e| LoadError::Lua(e.to_string()))
}

/// Try loading from bytecode cache; compile and cache on miss.
///
/// NOTE on secureenv: this function returns a fresh `rilua::Function` handle
/// whether the compiled body came from the cache or from source. The caller
/// (`load_lua_file`) applies `mark_secure_state` to that handle *after* this
/// function returns, so cache-replayed chunks and fresh compilations are
/// indistinguishable from secureenv's point of view — both get their fenv
/// swapped before the chunk ever runs. That ordering must be preserved if
/// anyone wires actual cache `get`/`put` calls here.
fn load_cached_or_compile(
    lua: &mut LuaState,
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
fn set_object_taint(state: &mut LuaState, func: &rilua::Function, taint: &str) {
    stamp_addon_taint_state(state, func, taint);
}

fn report_lua_load_error(state: &mut LuaState, err: impl ToString) -> LoadError {
    let msg = err.to_string();
    call_error_handler_state(state, &msg);
    LoadError::Lua(msg)
}

/// Compile Lua source code into a function.
fn compile_from_source(
    lua: &mut LuaState,
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
pub fn compile_with_rilua<L: LuaApiMut>(
    lua: &mut L,
    bytes: &[u8],
    chunk_name: &str,
) -> Result<rilua::Function, LoadError> {
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

    /// A function handle reloaded from cached bytecode must accept `setfenv`
    /// exactly like one produced by fresh compilation. This protects the
    /// invariant documented on `load_cached_or_compile`: regardless of
    /// whether `compile_with_rilua` consumed source or bytecode,
    /// `state_set_fenv` can still retarget the returned closure so
    /// downstream `mark_secure_state` works on cache hits.
    #[test]
    fn bytecode_replayed_function_accepts_setfenv() {
        use rilua::LuaApiMut;

        let mut lua = rilua::Lua::new().unwrap();
        // Fresh compile -> get a Function whose prototype we can dump.
        let func_from_source =
            compile_with_rilua(&mut lua, b"return MARK_SECURE_PROBE", "@cache_test")
                .expect("source compile should succeed");

        let proto = {
            let state = lua.state_mut();
            let closure = state
                .gc
                .closures
                .get(func_from_source.gc_ref())
                .expect("closure exists");
            match closure {
                rilua::vm::closure::Closure::Lua(cl) => cl.proto.clone(),
                rilua::vm::closure::Closure::Rust(_) => {
                    panic!("compiled source should produce a Lua closure")
                }
            }
        };

        // Dump to Lua 5.1 bytecode bytes — what bytecode_cache would store.
        let bytecode = {
            let state = lua.state_mut();
            rilua::vm::dump::dump(&proto, Some(&state.gc.string_arena), false)
        };

        // Simulate a cache hit: feed bytecode back through the same entry
        // point the loader uses on miss.
        let func_from_bytecode = compile_with_rilua(&mut lua, &bytecode, "@cache_test")
            .expect("bytecode replay should succeed");

        // Build a fresh env table, point it at a sentinel value.
        let env_table = LuaApiMut::create_table(&mut lua);
        {
            let state = lua.state_mut();
            let sentinel = rilua::Val::Str(state.gc.intern_string(b"from-secureenv"));
            let key = rilua::Val::Str(state.gc.intern_string(b"MARK_SECURE_PROBE"));
            env_table.raw_set(state, key, sentinel).unwrap();
        }

        // Swap the replayed closure's fenv — the secureenv path.
        rilua::api::state_set_fenv(lua.state_mut(), &func_from_bytecode, &env_table)
            .expect("state_set_fenv should accept bytecode-replayed closure");

        // The replayed chunk should resolve MARK_SECURE_PROBE through the
        // swapped env, not _G (where it's nil).
        let results = lua.call_function(&func_from_bytecode, &[]).unwrap();
        let resolved = results.into_iter().next().expect("chunk returns a value");
        let rilua::Val::Str(s) = resolved else {
            panic!("expected string, got {resolved:?}");
        };
        assert_eq!(
            lua.val_as_bytes(rilua::Val::Str(s)).unwrap(),
            b"from-secureenv"
        );
    }
}
