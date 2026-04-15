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
//! `create_secure_environment` and `mark_secure` mirror the old mlua
//! secureenv path: create a shallow copy of `_G`, give it its own `Enum`,
//! expose it as `__secureenv`, and retarget secure addon chunks to that env.
//!
//! # Loader environment (loader_env.rs / env_init.rs counterpart)
//!
//! `compile_chunk_rilua` — compiles Lua source bytes via rilua's pure-Rust
//! compiler (`compile_with_rilua`). Entry point for the rilua-side loading path.

use crate::loader::LoadError;
use crate::loader::lua_file::compile_with_rilua;
use crate::lua_api::rilua_methods::registry_get;
use crate::lua_api::rilua_methods::registry_set;
use crate::lua_api::rilua_script_helpers::{call_error_handler_state, protected_lua_pcall_state};
use rilua::LuaApiMut;
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
    let Some((obj, obj_ref, method_name, nargs)) = parse_securecallmethod_args(state)? else {
        return Ok(0);
    };

    let method = lookup_method_on_table(state, obj_ref, method_name);
    if method == Val::Nil {
        return Ok(0);
    }

    let self_and_extra = gather_self_and_extra_args(state, obj, nargs);
    dispatch_securecall(state, method, &self_and_extra)
}

/// Validates the incoming stack and returns `(obj, obj_ref, method_name, nargs)`.
///
/// Returns `Ok(None)` when the call is a no-op (missing args, nil receiver).
fn parse_securecallmethod_args(
    state: &LuaState,
) -> LuaResult<
    Option<(
        Val,
        rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
        rilua::vm::gc::arena::GcRef<rilua::vm::string::LuaString>,
        usize,
    )>,
> {
    let nargs = (state.top as i32 - state.base as i32) as usize;
    if nargs < 1 {
        return Ok(None);
    }

    let obj = state.stack_get(state.base);
    if obj == Val::Nil {
        return Ok(None);
    }

    let Val::Str(method_name) = state.stack_get(state.base + 1) else {
        return Err(runtime_error("Usage: securecallmethod(table, name, ...)"));
    };
    let Val::Table(obj_ref) = obj else {
        return Err(runtime_error("Usage: securecallmethod(table, name, ...)"));
    };

    Ok(Some((obj, obj_ref, method_name, nargs)))
}

/// Look up `method_name` on the table referenced by `obj_ref`.
fn lookup_method_on_table(
    state: &mut LuaState,
    obj_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
    method_name: rilua::vm::gc::arena::GcRef<rilua::vm::string::LuaString>,
) -> Val {
    let direct = state
        .gc
        .tables
        .get(obj_ref)
        .map(|t| t.get_str(method_name, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    if direct != Val::Nil {
        return direct;
    }

    let index_key = state.gc.intern_string(b"__index");
    let index_table = state
        .gc
        .tables
        .get(obj_ref)
        .and_then(|table| table.metatable())
        .and_then(|mt_ref| state.gc.tables.get(mt_ref))
        .map(|mt| mt.get_str(index_key, &state.gc.string_arena))
        .and_then(|value| match value {
            Val::Table(table_ref) => Some(table_ref),
            _ => None,
        });
    index_table
        .and_then(|table_ref| state.gc.tables.get(table_ref))
        .map(|table| table.get_str(method_name, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

/// Build the argument list `[obj, extras...]` passed through to the callee.
///
/// Stack layout: `[obj, method_name, extra_1, extra_2, ...]`. The method is
/// invoked as `method(obj, extra_1, ...)`, so we keep `obj` and drop the name.
fn gather_self_and_extra_args(state: &LuaState, obj: Val, nargs: usize) -> Vec<Val> {
    std::iter::once(obj)
        .chain((2..nargs).map(|i| state.stack_get(state.base + i)))
        .collect()
}

/// Invoke the method either through the global `securecall` wrapper or
/// directly if `securecall` was stripped from the environment.
fn dispatch_securecall(state: &mut LuaState, method: Val, args: &[Val]) -> LuaResult<u32> {
    let Val::Function(_) = method else {
        return Ok(0);
    };
    match protected_lua_pcall_state(state, method, args) {
        Ok(results) if results.is_empty() => {
            state.push(Val::Nil);
            Ok(1)
        }
        Ok(results) => {
            let count = results.len() as u32;
            for value in results {
                state.push(value);
            }
            Ok(count)
        }
        Err(error) => {
            call_error_handler_state(state, &error);
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

// ── Value-access fallbacks ───────────────────────────────────────────────────

fn register_value_access_fallbacks(lua: &mut rilua::Lua) -> LuaResult<()> {
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
    let nargs = (state.top as i32 - state.base as i32).max(0) as u32;
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
    let text = {
        let lua_str = state
            .gc
            .string_arena
            .get(s_ref)
            .ok_or_else(|| runtime_error("SecureCmdOptionParse: invalid string"))?;
        std::str::from_utf8(lua_str.data())
            .map_err(|_| runtime_error("SecureCmdOptionParse: non-UTF8 string"))?
            .to_owned()
    };
    let last = text.split(';').next_back().map(str::trim).unwrap_or("");
    let result = Val::Str(state.gc.intern_string(last.as_bytes()));
    state.push(result);
    Ok(1)
}

// ── Secure environment (secure_env.rs counterpart) ───────────────────────────

const CREATE_SECURE_ENV_LUA: &str = r##"
    local genv = _G
    local secureenv = {}
    for k, v in pairs(genv) do
        secureenv[k] = v
    end
    if genv.Enum then
        local se = {}
        for k, v in pairs(genv.Enum) do
            se[k] = v
        end
        secureenv.Enum = se
    end
    secureenv._G = secureenv
    setmetatable(secureenv, { __index = genv })
    return secureenv
"##;

fn secure_env_table(
    state: &mut LuaState,
) -> LuaResult<rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>> {
    match registry_get(state, "__secureenv") {
        Val::Table(table_ref) => Ok(table_ref),
        _ => Err(runtime_error("secure environment not initialized")),
    }
}

/// Create the secure environment as a shallow copy of `_G` with fallback.
///
/// This preserves secure APIs when `Blizzard_EnvironmentCleanup` nils them
/// from `_G`, while still seeing globals registered later through the
/// metatable `__index` fallback.
pub fn create_secure_environment(lua: &mut rilua::Lua) -> LuaResult<()> {
    if matches!(LuaApiMut::get_global_val(lua, "__secureenv"), Val::Table(_)) {
        return Ok(());
    }

    let chunk = LuaApiMut::load(lua, CREATE_SECURE_ENV_LUA)?;
    let secureenv = lua
        .call_function(&chunk, &[])?
        .into_iter()
        .next()
        .ok_or_else(|| runtime_error("create_secure_environment: missing return value"))?;
    let Val::Table(_) = secureenv else {
        return Err(runtime_error(
            "create_secure_environment: expected secureenv table",
        ));
    };

    {
        let state = lua.state_mut();
        registry_set(state, "__secureenv", secureenv);
    }
    LuaApiMut::set_global_val(lua, "__secureenv", secureenv)?;
    Ok(())
}

/// Mark a compiled function as running under the secure environment.
///
/// Swaps the function's fenv to the registry-stored `__secureenv` table so
/// the closure and every inner closure it creates see `secureenv` as their
/// globals table. Matches Blizzard's `setfenv(chunk, secureenv)` step for
/// `[LoadIntoEnvironment secure]` files — caller never passes secureenv
/// explicitly; it's looked up via the registry.
pub fn mark_secure(lua: &mut rilua::Lua, func: &rilua::Function) -> LuaResult<()> {
    mark_secure_state(lua.state_mut(), func)
}

/// Raw-state variant of [`mark_secure`] for callers holding `&mut LuaState`
/// (e.g. inside a `with_state` closure or a `RustFn`).
pub fn mark_secure_state(state: &mut LuaState, func: &rilua::Function) -> LuaResult<()> {
    let secureenv_ref = secure_env_table(state)?;
    let secureenv = rilua::Table::from_gc_ref(secureenv_ref);
    rilua::api::state_set_fenv(state, func, &secureenv)
}

/// Set a key in both the global table and secureenv.
///
/// Used for names that must stay visible in secure code even after cleanup
/// strips them from `_G`.
pub fn set_in_both_envs_rilua(lua: &mut rilua::Lua, key: &str, val: Val) -> LuaResult<()> {
    LuaApiMut::set_global_val(lua, key, val)?;
    let state = lua.state_mut();
    if let Ok(secureenv_ref) = secure_env_table(state) {
        let secureenv = rilua::Table::from_gc_ref(secureenv_ref);
        let key_ref = state.gc.intern_string(key.as_bytes());
        secureenv.raw_set(state, Val::Str(key_ref), val)?;
    }
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
    let func = compile_chunk_rilua(lua, source, chunk_name)?;
    if use_secure_env {
        mark_secure(lua, &func).map_err(|e| LoadError::Lua(e.to_string()))?;
    }
    lua.call_function(&func, &[])
        .map_err(|e| LoadError::Lua(e.to_string()))?;
    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Register a `RustFn` only if the global is currently nil.
///
/// Mirrors `security_api::set_if_missing` for the rilua path.
fn register_if_missing(lua: &mut rilua::Lua, name: &str, func: rilua::RustFn) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let existing = LuaApiMut::get_global_val(lua, name);
    if existing == Val::Nil {
        LuaApiMut::register_function(lua, name, func)?;
    }
    Ok(())
}
