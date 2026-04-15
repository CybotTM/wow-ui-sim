//! rilua RustFn equivalents of globals from utility_api, system_api, and spell_api.
//!
//! Each `pub fn` matches the `RustFn` signature:
//!   `fn(state: &mut LuaState) -> LuaResult<u32>`
//!
//! Arguments are extracted with `stack_val(state, n)` (1-based).
//! Return values are pushed with `state.push(val)` and counted in the return.
//!
//! Complex operations (pcall, xpcall, securecall) are stubbed with TODO.

use crate::lua_bridge::{stack_val, table_set_rust_fn};
use rilua::LuaApiMut;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};

// ── Utility API ─────────────────────────────────────────────────────────────

/// wipe(t) — clear all entries from a table and return it.
///
/// TODO: rilua table iteration API needed to implement fully.
pub fn wipe(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: iterate table pairs and set each key to nil
    let t = stack_val(state, 1);
    state.push(t);
    Ok(1)
}

/// tinsert(t [, pos], value) — append or insert a value into an array table.
///
/// TODO: rilua table mutation API needed.
pub fn tinsert(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: implement via table array ops
    Ok(0)
}

/// tremove(t [, pos]) — remove and return a value from an array table.
///
/// TODO: rilua table mutation API needed.
pub fn tremove(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: implement via table array ops
    state.push(Val::Nil);
    Ok(1)
}

/// tContains(t, value) — return true if value is present in the array part of t.
///
/// TODO: rilua table iteration API needed.
pub fn t_contains(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: iterate array part and compare
    state.push(Val::Bool(false));
    Ok(1)
}

/// tIndexOf(t, value) — return the integer index of value in t, or nil.
///
/// TODO: rilua table iteration API needed.
pub fn t_index_of(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: iterate array part and compare
    state.push(Val::Nil);
    Ok(1)
}

/// tInvert(t) — return a new table with keys/values swapped.
///
/// TODO: rilua table iteration/creation API needed.
pub fn t_invert(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: build inverted table
    state.push(Val::Nil);
    Ok(1)
}

/// getglobal(name) — return the global named `name`.
pub fn getglobal(state: &mut LuaState) -> LuaResult<u32> {
    let name_val = stack_val(state, 1);
    let Val::Str(name_ref) = name_val else {
        return Err(runtime_error("getglobal: expected string argument"));
    };
    let name = {
        let lua_str = state
            .gc
            .string_arena
            .get(name_ref)
            .ok_or_else(|| runtime_error("getglobal: invalid string ref"))?;
        String::from_utf8(lua_str.data().to_vec())
            .map_err(|_| runtime_error("getglobal: non-UTF8 name"))?
    };
    let global = state.global;
    let key_ref = state.gc.intern_string(name.as_bytes());
    let val = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    state.push(val);
    Ok(1)
}

/// setglobal(name, value) — set the global named `name` to `value`.
pub fn setglobal(state: &mut LuaState) -> LuaResult<u32> {
    let name_val = stack_val(state, 1);
    let value = stack_val(state, 2);
    let Val::Str(name_ref) = name_val else {
        return Err(runtime_error(
            "setglobal: expected string as first argument",
        ));
    };
    let name = {
        let lua_str = state
            .gc
            .string_arena
            .get(name_ref)
            .ok_or_else(|| runtime_error("setglobal: invalid string ref"))?;
        String::from_utf8(lua_str.data().to_vec())
            .map_err(|_| runtime_error("setglobal: non-UTF8 name"))?
    };
    let global = state.global;
    let key_ref = state.gc.intern_string(name.as_bytes());
    if let Some(t) = state.gc.tables.get_mut(global) {
        let _ = t.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
    Ok(0)
}

/// nop(...) — no-operation, discards all arguments.
pub fn nop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// strsplit(delimiter, str [, limit]) — split str on delimiter, return multiple values.
///
/// TODO: full varargs return requires pushing multiple values.
pub fn strsplit(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: implement multi-return string split
    let input = stack_val(state, 2);
    state.push(input);
    Ok(1)
}

/// strjoin(delimiter, ...) — join variadic string args with delimiter.
///
/// TODO: full varargs collection.
pub fn strjoin(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: collect all variadic args and join
    let empty = state.gc.intern_string(b"");
    state.push(Val::Str(empty));
    Ok(1)
}

// ── System API ───────────────────────────────────────────────────────────────

/// type(v) — return the Lua type name of v as a string, reporting frame UserData as "table".
///
/// Note: in rilua, FrameRef is a backed table (Val::Table), so no special case
/// is needed — Val::Table already covers frame-backed tables.
pub fn type_fn(state: &mut LuaState) -> LuaResult<u32> {
    let val = stack_val(state, 1);
    let type_name: &str = match val {
        Val::Nil => "nil",
        Val::Bool(_) => "boolean",
        Val::Num(_) => "number",
        Val::Str(_) => "string",
        Val::Table(_) => "table",
        Val::Function(_) => "function",
        Val::Userdata(_) | Val::LightUserdata(_) | Val::Thread(_) => "userdata",
    };
    let s = state.gc.intern_string(type_name.as_bytes());
    state.push(Val::Str(s));
    Ok(1)
}

/// IsPublicTestClient() — always false in the simulator.
pub fn is_public_test_client(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// IsBetaBuild() — always false in the simulator.
pub fn is_beta_build(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// IsPublicBuild() — always true in the simulator.
pub fn is_public_build(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

/// BNFeaturesEnabled() — always false (no Battle.net in sim).
pub fn bn_features_enabled(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// BNFeaturesEnabledAndConnected() — always false.
pub fn bn_features_enabled_and_connected(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// BNConnected() — always true (sim pretends connected).
pub fn bn_connected(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

/// IsGMClient() — always false.
pub fn is_gm_client(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// RegisterStaticConstants(t) — no-op stub.
pub fn register_static_constants(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// pcall(f, ...) — protected call.
///
/// TODO: rilua does not expose a pcall surface from RustFn context; stub returns false.
pub fn pcall(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: implement pcall semantics via rilua's error boundary API
    state.push(Val::Bool(false));
    let msg = state
        .gc
        .intern_string(b"pcall: not implemented in rilua path");
    state.push(Val::Str(msg));
    Ok(2)
}

/// xpcall(f, handler, ...) — protected call with error handler.
///
/// TODO: same limitation as pcall.
pub fn xpcall(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: implement xpcall semantics via rilua's error boundary API
    state.push(Val::Bool(false));
    let msg = state
        .gc
        .intern_string(b"xpcall: not implemented in rilua path");
    state.push(Val::Str(msg));
    Ok(2)
}

/// securecall(name_or_func, ...) — call a function by name in a secure context.
///
/// TODO: taint-aware dispatch not yet implemented in rilua path.
pub fn securecall(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: resolve function by name or Val::Function, call with taint cleared
    state.push(Val::Nil);
    Ok(1)
}

const ERROR_HANDLER_KEY: &str = "__error_handler";

pub fn seterrorhandler(state: &mut LuaState) -> LuaResult<u32> {
    let handler = stack_val(state, 1);
    if !matches!(handler, Val::Function(_)) {
        return Err(runtime_error("seterrorhandler: expected function"));
    }
    let previous = registry_value(state, ERROR_HANDLER_KEY);
    set_registry_value(state, ERROR_HANDLER_KEY, handler);
    state.push(previous);
    Ok(1)
}

pub fn geterrorhandler(state: &mut LuaState) -> LuaResult<u32> {
    let handler = ensure_error_handler(state)?;
    state.push(handler);
    Ok(1)
}

pub fn table_util_find_indexed_mismatch(state: &mut LuaState) -> LuaResult<u32> {
    let left = stack_val(state, 1);
    let right = stack_val(state, 2);
    let comparator = stack_val(state, 3);
    let (Val::Table(left_ref), Val::Table(right_ref)) = (left, right) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let left_values = state
        .gc
        .tables
        .get(left_ref)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default();
    let right_values = state
        .gc
        .tables
        .get(right_ref)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default();
    let count = left_values.len().max(right_values.len());

    for index in 0..count {
        let left_val = left_values.get(index).copied().unwrap_or(Val::Nil);
        let right_val = right_values.get(index).copied().unwrap_or(Val::Nil);
        let equal = if matches!(comparator, Val::Function(_)) {
            call_table_util_comparator(state, comparator, left_val, right_val, index + 1)?
        } else {
            left_val == right_val
        };
        if !equal {
            state.push(Val::Num((index + 1) as f64));
            return Ok(1);
        }
    }

    state.push(Val::Nil);
    Ok(1)
}

// ── Spell API ────────────────────────────────────────────────────────────────

/// CastSpellByID(spellId [, unit]) — cast a spell by ID.
///
/// TODO: cast_spell_by_id requires Rc<RefCell<SimState>> — use borrow_state_mut helper.
pub fn cast_spell_by_id(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: extract spell_id from arg 1, call cast logic via borrow_state_mut
    Ok(0)
}

/// CastSpellByName(name [, unit]) — cast a spell by name.
///
/// TODO: same dependency as CastSpellByID.
pub fn cast_spell_by_name(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: extract name from arg 1, look up spell_id, call cast logic
    Ok(0)
}

// ── Registration ─────────────────────────────────────────────────────────────

/// Register all functions in this module as rilua globals.
///
/// Call this once after the rilua `Lua` instance is created.
pub fn register_all(lua: &mut rilua::Lua) -> rilua::LuaResult<()> {
    use rilua::LuaApiMut;

    // Utility: table functions
    LuaApiMut::register_function(lua, "wipe", wipe)?;
    LuaApiMut::register_function(lua, "tinsert", tinsert)?;
    LuaApiMut::register_function(lua, "tremove", tremove)?;
    LuaApiMut::register_function(lua, "tContains", t_contains)?;
    LuaApiMut::register_function(lua, "tIndexOf", t_index_of)?;
    LuaApiMut::register_function(lua, "tInvert", t_invert)?;

    // Utility: global access
    LuaApiMut::register_function(lua, "getglobal", getglobal)?;
    LuaApiMut::register_function(lua, "setglobal", setglobal)?;

    // Utility: misc
    LuaApiMut::register_function(lua, "nop", nop)?;

    // Utility: string functions
    LuaApiMut::register_function(lua, "strsplit", strsplit)?;
    LuaApiMut::register_function(lua, "strjoin", strjoin)?;

    // System: type override
    LuaApiMut::register_function(lua, "type", type_fn)?;

    // System: build type checks
    LuaApiMut::register_function(lua, "IsPublicTestClient", is_public_test_client)?;
    LuaApiMut::register_function(lua, "IsBetaBuild", is_beta_build)?;
    LuaApiMut::register_function(lua, "IsPublicBuild", is_public_build)?;

    // System: Battle.net stubs
    LuaApiMut::register_function(lua, "BNFeaturesEnabled", bn_features_enabled)?;
    LuaApiMut::register_function(
        lua,
        "BNFeaturesEnabledAndConnected",
        bn_features_enabled_and_connected,
    )?;
    LuaApiMut::register_function(lua, "BNConnected", bn_connected)?;

    // System: secure stubs
    LuaApiMut::register_function(lua, "IsGMClient", is_gm_client)?;
    LuaApiMut::register_function(lua, "RegisterStaticConstants", register_static_constants)?;

    // System: protected calls (stubbed)
    LuaApiMut::register_function(lua, "pcall", pcall)?;
    LuaApiMut::register_function(lua, "xpcall", xpcall)?;
    LuaApiMut::register_function(lua, "securecall", securecall)?;
    LuaApiMut::register_function(lua, "seterrorhandler", seterrorhandler)?;
    LuaApiMut::register_function(lua, "geterrorhandler", geterrorhandler)?;

    // Spell: cast globals (stubbed)
    LuaApiMut::register_function(lua, "CastSpellByID", cast_spell_by_id)?;
    LuaApiMut::register_function(lua, "CastSpellByName", cast_spell_by_name)?;
    register_table_util(lua.state_mut())?;

    Ok(())
}

fn register_table_util(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = state.gc.alloc_table(Table::new());
    table_set_rust_fn(
        state,
        table_ref,
        "FindIndexedMismatch",
        table_util_find_indexed_mismatch,
    )?;
    let key_ref = state.gc.intern_string(b"C_TableUtil");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(
            Val::Str(key_ref),
            Val::Table(table_ref),
            &state.gc.string_arena,
        );
    }
    Ok(())
}

fn ensure_error_handler(state: &mut LuaState) -> LuaResult<Val> {
    let existing = registry_value(state, ERROR_HANDLER_KEY);
    if existing != Val::Nil {
        return Ok(existing);
    }
    let default_handler = build_default_error_handler(state)?;
    set_registry_value(state, ERROR_HANDLER_KEY, default_handler);
    Ok(default_handler)
}

fn build_default_error_handler(state: &mut LuaState) -> LuaResult<Val> {
    let func = state.load("return function(_msg) end")?;
    let call_base = state.top;
    state.ensure_stack(call_base + 2);
    state.stack_set(call_base, Val::Function(func.gc_ref()));
    state.top = call_base + 1;
    state.call_function(call_base, 1)?;
    let result = state.stack_get(call_base);
    state.top = call_base;
    Ok(result)
}

fn registry_value(state: &mut LuaState, key: &str) -> Val {
    let key_ref = state.gc.intern_string(key.as_bytes());
    state
        .gc
        .tables
        .get(state.registry)
        .map(|table| table.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

fn set_registry_value(state: &mut LuaState, key: &str, value: Val) {
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(table) = state.gc.tables.get_mut(state.registry) {
        let _ = table.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
}

fn call_table_util_comparator(
    state: &mut LuaState,
    comparator: Val,
    left: Val,
    right: Val,
    index: usize,
) -> LuaResult<bool> {
    let call_base = state.top;
    state.ensure_stack(call_base + 5);
    state.stack_set(call_base, comparator);
    state.stack_set(call_base + 1, left);
    state.stack_set(call_base + 2, right);
    state.stack_set(call_base + 3, Val::Num(index as f64));
    state.top = call_base + 4;
    state.call_function(call_base, 1)?;
    let result = state.stack_get(call_base);
    state.top = call_base;
    Ok(!matches!(result, Val::Nil | Val::Bool(false)))
}
