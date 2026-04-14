//! rilua equivalents of security_api.rs, secure_env.rs, and loader_env.rs.
//!
//! # Security functions (security_api.rs counterpart)
//!
//! `securecallmethod`, taint helpers (`issecretvalue`, `canaccessvalue`,
//! `canaccessallvalues`, `canaccesstable`), scrub pass-throughs, SecureHandler
//! stubs, state/attribute driver stubs, and `SecureCmdOptionParse`.
//!
//! Functions provided by Elune's C runtime (`issecure`, `issecurevariable`,
//! `securecall`, `securecallfunction`, `forceinsecure`, `hooksecurefunc`,
//! `secureexecuterange`) are NOT registered here.
//!
//! # Secure environment (secure_env.rs counterpart)
//!
//! `create_secure_environment` and `apply_secure_env` depend on `setfenv`,
//! which is Lua 5.1 specific. rilua may expose this but taint integration is
//! not yet wired. Both are stubbed with `TODO` comments.
//!
//! # Loader environment (loader_env.rs / env_init.rs counterpart)
//!
//! `compile_chunk_rilua` — compiles Lua source bytes via rilua's pure-Rust
//! compiler (`compile_with_rilua`). Entry point for the rilua-side loading path.

use crate::loader::error::LoadError;
use crate::loader::lua_file::compile_with_rilua;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

// ── Top-level entry point ────────────────────────────────────────────────────

/// Register all security-related globals into rilua's global table.
///
/// Registers the same set as `security_api::register_security_functions` but
/// as rilua `RustFn`s. Elune C-runtime functions are excluded.
pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    LuaApiMut::register_function(lua, "securecallmethod", securecallmethod)?;
    register_value_access_fallbacks(lua)?;
    register_scrub_fallbacks(lua)?;
    register_secure_handler_stubs(lua)?;
    register_state_driver_stubs(lua)?;
    LuaApiMut::register_function(lua, "SecureCmdOptionParse", secure_cmd_option_parse)?;
    Ok(())
}

// ── securecallmethod ─────────────────────────────────────────────────────────

/// `securecallmethod(obj, name, ...)` — calls `obj[name](obj, ...)` via `securecall`.
///
/// Permissive stub: ignores taint, delegates to the global `securecall`.
fn securecallmethod(state: &mut LuaState) -> LuaResult<u32> {
    let nargs = state.top() - state.base as i32;
    if nargs < 1 {
        return Ok(0);
    }

    let obj = state.stack_get(state.base);
    if obj == Val::Nil {
        return Ok(0);
    }

    let method_name = match state.stack_get(state.base + 1) {
        Val::Str(s) => s,
        _ => {
            return Err(runtime_error(
                "Usage: securecallmethod(table, name, ...)",
            ))
        }
    };

    let Val::Table(obj_ref) = obj else {
        return Err(runtime_error(
            "Usage: securecallmethod(table, name, ...)",
        ));
    };

    let method = state
        .gc
        .tables
        .get(obj_ref)
        .map(|t| t.get_str(method_name, &state.gc.string_arena))
        .unwrap_or(Val::Nil);

    if method == Val::Nil {
        return Ok(0);
    }

    // Gather extra args (everything after `name`): obj + trailing args.
    let self_and_extra: Vec<Val> = std::iter::once(obj)
        .chain((2..nargs as usize).map(|i| state.stack_get(state.base + i as i32)))
        .collect();

    // Invoke global securecall.
    let securecall = state
        .gc
        .tables
        .get(state.globals)
        .map(|g| {
            let k = state.gc.intern_string(b"securecall");
            g.get_str(k, &state.gc.string_arena)
        })
        .unwrap_or(Val::Nil);

    let Val::Function(sc_ref) = securecall else {
        // securecall not present (stripped env) — call method directly.
        return call_direct(state, method, &self_and_extra);
    };

    drop_unused(sc_ref); // will be used below once call-from-RustFn is available
    // TODO: invoke securecall(method, obj, ...) once rilua exposes call-from-RustFn.
    // For now, call the method directly without taint wrapping.
    call_direct(state, method, &self_and_extra)
}

/// Call a function value directly and push its results onto the stack.
fn call_direct(state: &mut LuaState, func: Val, args: &[Val]) -> LuaResult<u32> {
    let Val::Function(func_ref) = func else {
        return Ok(0);
    };
    // Build a temporary rilua::Function handle and call it.
    // rilua::Function wraps the GcRef with safe lifetime extension.
    let _ = (state, func_ref, args); // suppress unused warnings until API is wired
    // TODO: wire state.call_function(func_ref, args) once available in RustFn context.
    Ok(0)
}

/// Suppress "unused" lint on variables we intentionally keep for future wiring.
#[inline(always)]
fn drop_unused<T>(_: T) {}

// ── Value-access fallbacks ───────────────────────────────────────────────────

fn register_value_access_fallbacks(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    register_if_missing(lua, "issecretvalue", issecretvalue_fallback)?;
    register_if_missing(lua, "canaccessvalue", canaccessvalue_fallback)?;
    register_if_missing(lua, "canaccessallvalues", canaccessallvalues_fallback)?;
    register_if_missing(lua, "canaccesstable", canaccesstable_fallback)?;
    Ok(())
}

/// `issecretvalue(v)` — returns false (permissive stub; no taint registry yet).
fn issecretvalue_fallback(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: check __tainted_loadstring_functions registry when rilua taint lands.
    state.push(Val::Bool(false));
    Ok(1)
}

/// `canaccessvalue(v)` — returns true (permissive stub).
fn canaccessvalue_fallback(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

/// `canaccessallvalues(...)` — returns true (permissive stub).
fn canaccessallvalues_fallback(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

/// `canaccesstable(t)` — returns true (permissive stub).
fn canaccesstable_fallback(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

// ── Scrub pass-throughs ──────────────────────────────────────────────────────

fn register_scrub_fallbacks(lua: &mut rilua::Lua) -> LuaResult<()> {
    register_if_missing(lua, "scrub", scrub_passthrough)?;
    register_if_missing(lua, "scrubsecretvalues", scrub_passthrough)?;
    Ok(())
}

/// `scrub(...)` / `scrubsecretvalues(...)` — return args unchanged.
fn scrub_passthrough(state: &mut LuaState) -> LuaResult<u32> {
    // All args are already on the stack; return them as-is.
    let nargs = (state.top() - state.base as i32).max(0) as u32;
    Ok(nargs)
}

// ── SecureHandler stubs ──────────────────────────────────────────────────────

fn register_secure_handler_stubs(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    LuaApiMut::register_function(lua, "SecureHandlerSetFrameRef", stub_noop)?;
    LuaApiMut::register_function(lua, "SecureHandlerExecute", stub_noop)?;
    LuaApiMut::register_function(lua, "SecureHandlerWrapScript", stub_noop)?;
    Ok(())
}

// ── State/attribute driver stubs ─────────────────────────────────────────────

fn register_state_driver_stubs(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    LuaApiMut::register_function(lua, "RegisterStateDriver", stub_noop)?;
    LuaApiMut::register_function(lua, "UnregisterStateDriver", stub_noop)?;
    LuaApiMut::register_function(lua, "RegisterAttributeDriver", stub_noop)?;
    LuaApiMut::register_function(lua, "UnregisterAttributeDriver", stub_noop)?;
    Ok(())
}

/// No-op stub: accepts any args, returns nothing.
fn stub_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// ── SecureCmdOptionParse ─────────────────────────────────────────────────────

/// `SecureCmdOptionParse(options)` — returns the last semicolon-delimited option.
fn secure_cmd_option_parse(state: &mut LuaState) -> LuaResult<u32> {
    let arg = state.stack_get(state.base);
    let Val::Str(s_ref) = arg else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let lua_str = state
        .gc
        .string_arena
        .get(s_ref)
        .ok_or_else(|| runtime_error("SecureCmdOptionParse: invalid string"))?;
    let text = std::str::from_utf8(lua_str.data())
        .map_err(|_| runtime_error("SecureCmdOptionParse: non-UTF8 string"))?;
    let last = text.split(';').next_back().map(str::trim).unwrap_or("");
    let result = Val::Str(state.gc.intern_string(last.as_bytes()));
    state.push(result);
    Ok(1)
}

// ── Secure environment (secure_env.rs counterpart) ───────────────────────────

/// Create the secure environment (rilua stub).
///
/// In the mlua path, `create_secure_environment` copies `_G` into a `secureenv`
/// table with `__index = _G` fallback so `Blizzard_EnvironmentCleanup` can nil
/// APIs from `_G` without affecting secure addons.
///
/// TODO: implement once rilua exposes `setfenv` / environment APIs and the
/// taint system is wired. For now this is a no-op; all code runs in the single
/// global env.
pub fn create_secure_environment(_lua: &mut rilua::Lua) -> LuaResult<()> {
    // TODO: clone globals table into secureenv, store in registry as
    // "__secureenv", set __index = globals metatable fallback.
    Ok(())
}

/// Apply the secure environment to a compiled function (rilua stub).
///
/// In the mlua path, `apply_secure_env` calls `setfenv(func, secureenv)` so
/// `UseSecureEnvironment` addon code runs in the isolated secure table.
///
/// TODO: implement via rilua's upvalue/env API once available.
pub fn apply_secure_env_rilua(
    _lua: &mut rilua::Lua,
    _func: &rilua::Function,
) -> LuaResult<()> {
    // TODO: call rilua equivalent of setfenv(func, secureenv).
    Ok(())
}

/// Set a key in both the global table and secureenv (rilua stub).
///
/// TODO: once `create_secure_environment` is implemented, also write to the
/// secureenv table stored in the registry.
pub fn set_in_both_envs_rilua(
    lua: &mut rilua::Lua,
    key: &str,
    val: Val,
) -> LuaResult<()> {
    use rilua::LuaApiMut;
    LuaApiMut::set_global_val(lua, key, val)?;
    // TODO: also write to registry["__secureenv"][key].
    Ok(())
}

// ── Loader environment (loader_env.rs / env_init.rs counterpart) ─────────────

/// Compile Lua source bytes via rilua's pure-Rust compiler.
///
/// Equivalent to the mlua `exec` path in `LoaderEnv::exec`, but uses
/// `compile_with_rilua` instead of mlua's `Lua::load`. Returns a `rilua::Function`
/// handle that can be called with `lua.call_function`.
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
    use rilua::LuaApiMut;
    let func = compile_chunk_rilua(lua, source, chunk_name)?;
    if use_secure_env {
        apply_secure_env_rilua(lua, &func)
            .map_err(|e| LoadError::Lua(e.to_string()))?;
    }
    LuaApiMut::call_function(lua, &func, &[])
        .map_err(|e| LoadError::Lua(e.to_string()))?;
    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Register a `RustFn` only if the global is currently nil.
///
/// Mirrors `security_api::set_if_missing` for the rilua path.
fn register_if_missing(
    lua: &mut rilua::Lua,
    name: &str,
    func: rilua::RustFn,
) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let existing = LuaApiMut::get_global_val(lua, name);
    if existing == Val::Nil {
        LuaApiMut::register_function(lua, name, func)?;
    }
    Ok(())
}
