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
use crate::lua_api::hot_literals::{hot_metatable_key, metatable_idx};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, frame_id_from_stack, registry_get, registry_set, val_to_string,
};
use crate::lua_api::script_helpers::{call_error_handler_state, protected_lua_pcall_state};
use crate::lua_bridge::stack_val;
use crate::widget::AttributeValue;
use rilua::LuaApiMut;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};
use std::collections::HashSet;

// ── Top-level entry point ────────────────────────────────────────────────────

/// Register all security-related globals into rilua's global table.
///
/// Registers the same set as `security_api::register_security_functions` but
/// as rilua `RustFn`s. Elune C-runtime functions are excluded.
pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    LuaApiMut::register_function(lua, "securecallmethod", securecallmethod)?;
    LuaApiMut::register_function(lua, "issecurevariable", issecurevariable_override)?;
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

    let index_key = hot_metatable_key(state, metatable_idx::INDEX);
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

fn issecurevariable_override(state: &mut LuaState) -> LuaResult<u32> {
    let first = state.stack_get(state.base);
    let second = state.stack_get(state.base + 1);
    let (table_ref, key_val) = match first {
        Val::Table(table_ref) if !matches!(second, Val::Nil) => (table_ref, second),
        Val::Str(_) | Val::Num(_) => (state.global, first),
        _ => {
            state.push(Val::Bool(true));
            return Ok(1);
        }
    };

    let taint = match key_val {
        Val::Str(s) => {
            let bytes = state.gc.string_arena.get(s).map(|ls| ls.data().to_vec());
            bytes.and_then(|bytes| {
                state
                    .gc
                    .tables
                    .get(table_ref)
                    .and_then(|table| table.get_slot_taint_str(&bytes).map(str::to_string))
            })
        }
        Val::Num(n) if n.is_finite() && (n as i64) as f64 == n => state
            .gc
            .tables
            .get(table_ref)
            .and_then(|table| table.get_slot_taint_int(n as i64).map(str::to_string)),
        _ => None,
    };

    match taint {
        None => {
            state.push(Val::Bool(true));
            Ok(1)
        }
        Some(taint) => {
            let taint = Val::Str(state.gc.intern_string(taint.as_bytes()));
            state.push(Val::Bool(false));
            state.push(taint);
            Ok(2)
        }
    }
}

/// `issecretvalue(v)` — returns true for tainted values and tables containing
/// tainted slots or nested tainted values.
fn issecretvalue_fallback(state: &mut LuaState) -> LuaResult<u32> {
    let value = state.stack_get(state.base);
    let secret = value_is_secret(state, value, &mut HashSet::new());
    state.push(Val::Bool(secret));
    Ok(1)
}

/// `canaccessvalue(v)` — returns true for non-secret values.
fn canaccessvalue_fallback(state: &mut LuaState) -> LuaResult<u32> {
    let value = state.stack_get(state.base);
    let secret = value_is_secret(state, value, &mut HashSet::new());
    state.push(Val::Bool(!secret));
    Ok(1)
}

/// `canaccessallvalues(...)` — returns true only if every argument is
/// non-secret.
fn canaccessallvalues_fallback(state: &mut LuaState) -> LuaResult<u32> {
    let nargs = state.top.saturating_sub(state.base);
    let mut visited = HashSet::new();
    let mut accessible = true;
    for index in 0..nargs {
        if value_is_secret(state, state.stack_get(state.base + index), &mut visited) {
            accessible = false;
            break;
        }
    }
    state.push(Val::Bool(accessible));
    Ok(1)
}

/// `canaccesstable(t)` — returns true only for non-secret tables.
fn canaccesstable_fallback(state: &mut LuaState) -> LuaResult<u32> {
    let value = state.stack_get(state.base);
    let accessible = match value {
        Val::Table(table_ref) => !table_is_secret(state, table_ref, &mut HashSet::new()),
        _ => !value_is_secret(state, value, &mut HashSet::new()),
    };
    state.push(Val::Bool(accessible));
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

const SECRET_VALUE_REGISTRY_KEY: &str = "__sim_secret_values";
pub(crate) const SECRET_TAINT_MARKER: &str = "*** SimSecretValue ***";

pub(crate) fn mark_secret_value(state: &mut LuaState, value: Val) {
    let Some(key) = secret_registry_key(state, value) else {
        return;
    };
    let marker = Val::Str(state.gc.intern_string(SECRET_TAINT_MARKER.as_bytes()));
    let table_ref = get_or_create_secret_value_table(state);
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(key, marker, &state.gc.string_arena);
    }
}

fn value_is_secret(
    state: &mut LuaState,
    value: Val,
    visited: &mut HashSet<rilua::vm::gc::arena::GcRef<Table>>,
) -> bool {
    if value_has_secret_marker(state, value) {
        return true;
    }

    match value {
        Val::Function(func_ref) => function_is_secret(state, func_ref),
        Val::Table(table_ref) => table_is_secret(state, table_ref, visited),
        _ => false,
    }
}

fn value_has_secret_marker(state: &mut LuaState, value: Val) -> bool {
    let Some(key) = secret_registry_key(state, value) else {
        return false;
    };
    let Val::Table(table_ref) = registry_get(state, SECRET_VALUE_REGISTRY_KEY) else {
        return false;
    };
    state.gc.tables.get(table_ref).is_some_and(|table| {
        matches!(
            table.get(key, &state.gc.string_arena),
            Val::Str(marker_ref)
                if state.gc.string_arena.get(marker_ref).is_some_and(|marker| {
                    marker.data() == SECRET_TAINT_MARKER.as_bytes()
                })
        )
    })
}

fn secret_registry_key(state: &mut LuaState, value: Val) -> Option<Val> {
    let key = match value {
        Val::Str(value_ref) => format!("str:{}", value_ref.index()),
        Val::Table(value_ref) => format!("table:{}", value_ref.index()),
        Val::Function(value_ref) => format!("func:{}", value_ref.index()),
        Val::Userdata(value_ref) => format!("userdata:{}", value_ref.index()),
        Val::Thread(value_ref) => format!("thread:{}", value_ref.index()),
        _ => return None,
    };
    Some(Val::Str(state.gc.intern_string(key.as_bytes())))
}

fn get_or_create_secret_value_table(state: &mut LuaState) -> rilua::vm::gc::arena::GcRef<Table> {
    if let Val::Table(table_ref) = registry_get(state, SECRET_VALUE_REGISTRY_KEY) {
        return table_ref;
    }
    let table_ref = state.gc.alloc_table(Table::new());
    registry_set(state, SECRET_VALUE_REGISTRY_KEY, Val::Table(table_ref));
    table_ref
}

fn function_is_secret(
    state: &mut LuaState,
    func_ref: rilua::vm::gc::arena::GcRef<rilua::vm::closure::Closure>,
) -> bool {
    let Val::Table(taint_table_ref) = registry_get(state, "__closure_taint") else {
        return false;
    };
    state.gc.tables.get(taint_table_ref).is_some_and(|table| {
        !matches!(
            table.get(Val::Num(func_ref.index() as f64), &state.gc.string_arena),
            Val::Nil
        )
    })
}

fn table_is_secret(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<Table>,
    visited: &mut HashSet<rilua::vm::gc::arena::GcRef<Table>>,
) -> bool {
    if !visited.insert(table_ref) {
        return false;
    }

    let Some(table) = state.gc.tables.get(table_ref) else {
        return false;
    };

    let mut entries = Vec::new();
    for (index, value) in table.array_slice().iter().copied().enumerate() {
        if !value.is_nil() {
            entries.push((Val::Num((index + 1) as f64), value));
        }
    }
    entries.extend(table.hash_entries());
    let tainted_slot = entries.iter().any(|(key, _)| match key {
        Val::Str(key_ref) => state
            .gc
            .string_arena
            .get(*key_ref)
            .and_then(|s| table.get_slot_taint_str(s.data()))
            .is_some(),
        Val::Num(n) if n.is_finite() && (*n as i64) as f64 == *n => {
            table.get_slot_taint_int(*n as i64).is_some()
        }
        _ => false,
    });
    if tainted_slot {
        return true;
    }

    entries.into_iter().any(|(key, value)| {
        value_is_secret(state, key, visited) || value_is_secret(state, value, visited)
    })
}

// ── SecureHandler fallback ───────────────────────────────────────────────────

/// Minimal Lua-side fallback for the `SecureHandler*` API surface. Replaces
/// the old no-op stubs with real snippet storage + pcall-protected execution
/// so protected frames can wire click-cast / state-driver actions before
/// `Blizzard_RestrictedAddOnEnvironment` loads (that addon registers the full
/// retail implementation, which shadows this fallback once it runs).
///
/// Semantics:
/// - `SecureHandlerSetFrameRef(frame, label, refFrame)` stores `refFrame` in a
///   weak-keyed registry at `_G.__secure_handler_frame_refs[frame][label]`.
///   `SecureHandlerGetFrameRef(frame, label)` is the companion lookup helper.
///   Weak keys so per-frame refs drop when the frame is GC'd.
/// - `SecureHandlerExecute(frame, body, ...)` compiles `body` with a
///   `local self = ...;` prelude and runs it under `pcall` with `frame` as
///   `self` plus any extra varargs. Errors are swallowed (same policy as
///   retail, which routes through `securecall`).
/// - `SecureHandlerWrapScript(frame, script, header, preBody, postBody)`
///   installs a wrapping script handler: `preBody` (if any) runs first with
///   `self = header`, the prior handler runs next, and `postBody` runs last.
///   Every step is `pcall`-isolated so a bad snippet can't prevent the others
///   from firing.
/// - `SecureHandlerUnwrapScript(frame, script)` restores the handler that was
///   active before the first fallback wrap for that frame/script pair.
fn register_secure_handler_stubs(lua: &mut rilua::Lua) -> LuaResult<()> {
    lua.exec(SECURE_HANDLER_FALLBACK_LUA)
        .map_err(|e| runtime_error(format!("secure-handler fallback: {e}")))?;
    Ok(())
}

const SECURE_HANDLER_FALLBACK_LUA: &str = r#"
-- Weak-keyed registries so per-frame state GCs with the owner.
if _G.__secure_handler_frame_refs == nil then
    _G.__secure_handler_frame_refs = setmetatable({}, { __mode = "k" })
end
if _G.__secure_handler_original_scripts == nil then
    _G.__secure_handler_original_scripts = setmetatable({}, { __mode = "k" })
end

function SecureHandlerSetFrameRef(frame, label, refFrame)
    if frame == nil or type(label) ~= "string" or refFrame == nil then
        return
    end
    local refs = _G.__secure_handler_frame_refs[frame]
    if refs == nil then
        refs = {}
        _G.__secure_handler_frame_refs[frame] = refs
    end
    refs[label] = refFrame
end

function SecureHandlerGetFrameRef(frame, label)
    if frame == nil or type(label) ~= "string" then
        return nil
    end
    local refs = _G.__secure_handler_frame_refs[frame]
    if refs == nil then
        return nil
    end
    return refs[label]
end

local function readonly_copy(source)
    local copy = {}
    for key, value in pairs(source) do
        copy[key] = value
    end
    return setmetatable(copy, {
        __newindex = function()
            error("restricted table is read-only")
        end,
        __metatable = false,
    })
end

local restricted_env = setmetatable({
    assert = assert,
    error = error,
    ipairs = ipairs,
    math = readonly_copy(math),
    next = next,
    pairs = pairs,
    print = print,
    select = select,
    string = readonly_copy(string),
    tonumber = tonumber,
    tostring = tostring,
    type = type,
    unpack = unpack,
}, {
    __newindex = function()
        error("restricted environment is read-only")
    end,
    __metatable = false,
})

-- Compile `body` as a closure `function(self, ...) <body> end`. Returning the
-- closure through an outer loadstring wrapper keeps `self` and the varargs
-- cleanly separated (plain `local self = ...` would consume from the same
-- vararg list and mis-index subsequent destructures). The closure runs in a
-- restricted environment: frame refs arrive through `self` and globals are
-- limited to safe utility tables/functions.
local function compile_snippet(body, chunk_name)
    local loader, err = loadstring("return function(self, ...) " .. body .. " end", chunk_name)
    if not loader then return nil end
    local ok, closure = pcall(loader)
    if not ok or type(closure) ~= "function" then return nil end
    setfenv(closure, restricted_env)
    return closure
end

function SecureHandlerExecute(frame, body, ...)
    if frame == nil or type(body) ~= "string" then
        return
    end
    local closure = compile_snippet(body, "SecureHandlerExecute")
    if closure == nil then return end
    pcall(closure, frame, ...)
end

local function original_scripts_for_frame(frame)
    local scripts = _G.__secure_handler_original_scripts[frame]
    if scripts == nil then
        scripts = {}
        _G.__secure_handler_original_scripts[frame] = scripts
    end
    return scripts
end

function SecureHandlerWrapScript(frame, script, header, preBody, postBody)
    if frame == nil or type(script) ~= "string" or type(preBody) ~= "string" then
        return
    end
    local owner = header or frame
    local pre_closure = compile_snippet(preBody, "SecureHandlerWrapScript-pre")
    local post_closure
    if type(postBody) == "string" then
        post_closure = compile_snippet(postBody, "SecureHandlerWrapScript-post")
    end
    local original = frame.GetScript and frame:GetScript(script) or nil
    local scripts = original_scripts_for_frame(frame)
    if scripts[script] == nil then
        scripts[script] = original or false
    end
    frame:SetScript(script, function(self, ...)
        if pre_closure then
            pcall(pre_closure, owner, ...)
        end
        if original then
            pcall(original, self, ...)
        end
        if post_closure then
            pcall(post_closure, owner, ...)
        end
    end)
end

function SecureHandlerUnwrapScript(frame, script)
    if frame == nil or type(script) ~= "string" then
        return
    end
    local scripts = _G.__secure_handler_original_scripts[frame]
    if scripts == nil or scripts[script] == nil then
        return
    end
    local original = scripts[script]
    scripts[script] = nil
    if original == false then
        original = nil
    end
    frame:SetScript(script, original)
end
"#;

// ── State/attribute drivers ──────────────────────────────────────────────────

fn register_state_driver_stubs(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    LuaApiMut::register_function(lua, "RegisterStateDriver", register_state_driver)?;
    LuaApiMut::register_function(lua, "UnregisterStateDriver", unregister_state_driver)?;
    LuaApiMut::register_function(lua, "RegisterAttributeDriver", register_attribute_driver)?;
    LuaApiMut::register_function(
        lua,
        "UnregisterAttributeDriver",
        unregister_attribute_driver,
    )?;
    Ok(())
}

fn register_state_driver(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(name) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    let Some(values) = val_to_string(state, stack_val(state, 3)) else {
        return Ok(0);
    };
    register_driver(state, id, &format!("state-{name}"), values)?;
    Ok(0)
}

fn unregister_state_driver(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(name) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    unregister_driver(state, id, &format!("state-{name}"))
}

fn register_attribute_driver(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(attribute) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    let Some(values) = val_to_string(state, stack_val(state, 3)) else {
        return Ok(0);
    };
    register_driver(state, id, &attribute, values)
}

fn unregister_attribute_driver(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(attribute) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    unregister_driver(state, id, &attribute)
}

fn register_driver(
    state: &mut LuaState,
    id: u64,
    attribute: &str,
    values: String,
) -> LuaResult<u32> {
    if attribute.starts_with('_') {
        return Ok(0);
    }
    {
        let mut sim = borrow_state_mut(state)?;
        sim.secure_attribute_drivers
            .entry(id)
            .or_default()
            .insert(attribute.to_string(), values.clone());
    }
    apply_driver(state, id, attribute, &values)?;
    Ok(0)
}

fn unregister_driver(state: &mut LuaState, id: u64, attribute: &str) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    let Some(drivers) = sim.secure_attribute_drivers.get_mut(&id) else {
        return Ok(0);
    };
    drivers.remove(attribute);
    if drivers.is_empty() {
        sim.secure_attribute_drivers.remove(&id);
    }
    Ok(0)
}

fn apply_driver(state: &mut LuaState, id: u64, attribute: &str, values: &str) -> LuaResult<()> {
    let Some(resolved) = resolve_driver_value(values) else {
        return Ok(());
    };

    if attribute == "state-visibility" {
        apply_visibility_driver(state, id, resolved);
        return Ok(());
    }

    let attr = coerce_driver_attribute(resolved);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        match attr {
            AttributeValue::Nil => {
                frame.attributes.remove(attribute);
            }
            value => {
                frame.attributes.insert(attribute.to_string(), value);
            }
        }
    }
    Ok(())
}

fn resolve_driver_value(values: &str) -> Option<&str> {
    values
        .split(';')
        .next_back()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn apply_visibility_driver(state: &mut LuaState, id: u64, resolved: &str) {
    let mut sim = match borrow_state_mut(state) {
        Ok(sim) => sim,
        Err(_) => return,
    };
    let Some(frame) = sim.widgets.get_mut(id) else {
        return;
    };
    match resolved {
        "show" => {
            frame.attributes.remove("statehidden");
            sim.set_frame_visible(id, true);
        }
        "hide" => {
            frame
                .attributes
                .insert("statehidden".into(), AttributeValue::Boolean(true));
            sim.set_frame_visible(id, false);
        }
        _ => {}
    }
}

fn coerce_driver_attribute(resolved: &str) -> AttributeValue {
    if resolved == "nil" {
        AttributeValue::Nil
    } else if let Ok(number) = resolved.parse::<f64>() {
        AttributeValue::Number(number)
    } else {
        AttributeValue::String(resolved.to_string())
    }
}

// ── SecureCmdOptionParse ─────────────────────────────────────────────────────

/// `SecureCmdOptionParse(options)` — returns the first option whose bracketed
/// condition list matches current simulator state.
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
    let selected = {
        let sim = borrow_state(state)?;
        resolve_cmd_option(&text, &sim).map(str::to_string)
    };
    match selected {
        Some(value) => {
            let result = Val::Str(state.gc.intern_string(value.as_bytes()));
            state.push(result);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn resolve_cmd_option<'a>(text: &'a str, sim: &crate::lua_api::SimState) -> Option<&'a str> {
    text.split(';')
        .filter_map(parse_cmd_option_clause)
        .find_map(|clause| clause.matches(sim).then_some(clause.value))
}

struct CmdOptionClause<'a> {
    conditions: Option<&'a str>,
    value: &'a str,
}

impl<'a> CmdOptionClause<'a> {
    fn matches(&self, sim: &crate::lua_api::SimState) -> bool {
        self.conditions
            .is_none_or(|conditions| condition_list_matches(conditions, sim))
    }
}

fn parse_cmd_option_clause(clause: &str) -> Option<CmdOptionClause<'_>> {
    let trimmed = clause.trim();
    if trimmed.is_empty() {
        return None;
    }
    let Some(rest) = trimmed.strip_prefix('[') else {
        return Some(CmdOptionClause {
            conditions: None,
            value: trimmed,
        });
    };
    let Some(close_index) = rest.find(']') else {
        return Some(CmdOptionClause {
            conditions: None,
            value: trimmed,
        });
    };
    let conditions = &rest[..close_index];
    let value = rest[close_index + 1..].trim();
    (!value.is_empty()).then_some(CmdOptionClause {
        conditions: Some(conditions),
        value,
    })
}

fn condition_list_matches(conditions: &str, sim: &crate::lua_api::SimState) -> bool {
    let mut unit = "target";
    for condition in conditions.split(',').map(str::trim) {
        if let Some(unit_override) = parse_unit_override(condition) {
            unit = unit_override;
        }
    }
    conditions
        .split(',')
        .map(str::trim)
        .all(|condition| condition_matches(condition, unit, sim))
}

fn parse_unit_override(condition: &str) -> Option<&str> {
    condition
        .strip_prefix('@')
        .or_else(|| condition.strip_prefix("target="))
        .or_else(|| condition.strip_prefix("unit="))
        .map(str::trim)
        .filter(|unit| !unit.is_empty())
}

fn condition_matches(condition: &str, unit: &str, sim: &crate::lua_api::SimState) -> bool {
    if condition.is_empty() || parse_unit_override(condition).is_some() {
        return true;
    }
    let condition = condition.to_ascii_lowercase();
    let (name, argument) = condition
        .split_once(':')
        .map_or((condition.as_str(), ""), |(name, argument)| {
            (name, argument)
        });
    match name {
        "combat" => sim.player.in_combat,
        "nocombat" => !sim.player.in_combat,
        "mod" => modifier_condition_matches(argument, sim, true),
        "nomod" => modifier_condition_matches(argument, sim, false),
        "harm" => unit_is_harmful(unit, sim),
        "noharm" => !unit_is_harmful(unit, sim),
        "help" => unit_is_helpful(unit, sim),
        "nohelp" => !unit_is_helpful(unit, sim),
        "exists" => unit_exists_for_option(unit, sim),
        "noexists" => !unit_exists_for_option(unit, sim),
        "dead" => unit_is_dead(unit, sim),
        "nodead" => !unit_is_dead(unit, sim),
        "group" => group_condition_matches(argument, sim),
        "nogroup" => !group_condition_matches(argument, sim),
        _ => false,
    }
}

fn modifier_condition_matches(
    argument: &str,
    sim: &crate::lua_api::SimState,
    expected: bool,
) -> bool {
    let actual = if argument.is_empty() {
        sim.modifier_keys.any_modifier()
    } else {
        argument.split('/').any(|key| modifier_key_down(key, sim))
    };
    actual == expected
}

fn modifier_key_down(key: &str, sim: &crate::lua_api::SimState) -> bool {
    match key.trim() {
        "shift" => sim.modifier_keys.shift,
        "ctrl" | "control" => sim.modifier_keys.control,
        "alt" => sim.modifier_keys.alt,
        "meta" => sim.modifier_keys.meta,
        _ => false,
    }
}

fn group_condition_matches(argument: &str, sim: &crate::lua_api::SimState) -> bool {
    match argument {
        "" | "party" | "raid" => sim.party_group_active,
        _ => false,
    }
}

fn unit_exists_for_option(unit: &str, sim: &crate::lua_api::SimState) -> bool {
    match unit {
        "player" | "pet" | "vehicle" => true,
        "target" => sim.current_target.is_some(),
        "focus" => sim.current_focus.is_some(),
        other => visible_party_member_for_option(other, sim).is_some(),
    }
}

fn unit_is_harmful(unit: &str, sim: &crate::lua_api::SimState) -> bool {
    match unit_target_info(unit, sim) {
        Some(target) => target.is_enemy || target.reaction < 4,
        None => false,
    }
}

fn unit_is_helpful(unit: &str, sim: &crate::lua_api::SimState) -> bool {
    match unit_target_info(unit, sim) {
        Some(target) => !target.is_enemy || target.reaction >= 4,
        None => {
            visible_party_member_for_option(unit, sim).is_some()
                || matches!(unit, "player" | "pet" | "vehicle")
        }
    }
}

fn unit_is_dead(unit: &str, sim: &crate::lua_api::SimState) -> bool {
    match unit_target_info(unit, sim) {
        Some(target) => target.health <= 0,
        None => visible_party_member_for_option(unit, sim)
            .is_some_and(|member| member.dead_since.is_some()),
    }
}

fn unit_target_info<'a>(
    unit: &str,
    sim: &'a crate::lua_api::SimState,
) -> Option<&'a crate::lua_api::game_data::TargetInfo> {
    match unit {
        "target" => sim.current_target.as_ref(),
        "focus" => sim.current_focus.as_ref(),
        _ => None,
    }
}

fn visible_party_member_for_option<'a>(
    unit: &str,
    sim: &'a crate::lua_api::SimState,
) -> Option<&'a crate::lua_api::state::PartyMember> {
    if !sim.party_group_active {
        return None;
    }
    if let Some(idx) = crate::lua_api::globals::unit_api::parse_party_index(unit) {
        return sim.party_members.get(idx);
    }
    unit.strip_prefix("raid")
        .and_then(|rest| rest.parse::<usize>().ok())
        .and_then(|n| n.checked_sub(1))
        .and_then(|idx| sim.party_members.get(idx))
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
