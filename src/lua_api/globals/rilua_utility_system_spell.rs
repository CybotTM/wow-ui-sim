//! rilua RustFn equivalents of globals from utility_api, system_api, and spell_api.
//!
//! Each `pub fn` matches the `RustFn` signature:
//!   `fn(state: &mut LuaState) -> LuaResult<u32>`
//!
//! Arguments are extracted with `stack_val(state, n)` (1-based).
//! Return values are pushed with `state.push(val)` and counted in the return.
//!
//! Complex operations (pcall, xpcall, securecall) are stubbed with TODO.

use crate::lua_api::LoaderEnv;
use crate::lua_api::rilua_methods::{
    borrow_lua, borrow_state, borrow_state_mut, create_string, create_table, frame_id_from_stack,
    registry_get, registry_set, state_handle, val_to_string,
};
use crate::lua_api::rilua_script_helpers::protected_call_state;
use crate::lua_bridge::{stack_val, table_set_rust_fn};
use rilua::LuaApiMut;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};
use std::path::PathBuf;

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
    let func = stack_val(state, 1);
    let args: Vec<_> = ((state.base + 1)..state.top)
        .map(|index| state.stack_get(index))
        .collect();
    match protected_call_state(state, func, &args) {
        Ok(results) => {
            state.push(Val::Bool(true));
            for result in &results {
                state.push(*result);
            }
            Ok(1 + results.len() as u32)
        }
        Err(error) => {
            state.push(Val::Bool(false));
            state.push(error);
            Ok(2)
        }
    }
}

/// xpcall(f, handler, ...) — protected call with error handler.
///
/// TODO: same limitation as pcall.
pub fn xpcall(state: &mut LuaState) -> LuaResult<u32> {
    let func = stack_val(state, 1);
    let handler = stack_val(state, 2);
    let args: Vec<_> = ((state.base + 2)..state.top)
        .map(|index| state.stack_get(index))
        .collect();
    match protected_call_state(state, func, &args) {
        Ok(results) => {
            state.push(Val::Bool(true));
            for result in &results {
                state.push(*result);
            }
            Ok(1 + results.len() as u32)
        }
        Err(error) => {
            let handled = if matches!(handler, Val::Function(_)) {
                protected_call_state(state, handler, &[error])
                    .ok()
                    .and_then(|results| results.into_iter().next())
                    .unwrap_or(error)
            } else {
                error
            };
            state.push(Val::Bool(false));
            state.push(handled);
            Ok(2)
        }
    }
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
const ADDON_VERSION_CHECK_KEY: &str = "__addon_version_check_enabled";

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
    register_c_addons(lua.state_mut())?;
    register_c_addon_profiler(lua.state_mut())?;
    register_legacy_addon_globals(lua.state_mut())?;
    register_widget_container_mixin(lua.state_mut())?;

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

fn register_c_addons(state: &mut LuaState) -> LuaResult<()> {
    let c_addons = create_table(state);
    let Val::Table(c_addons_ref) = c_addons else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(state, c_addons_ref, "GetNumAddOns", c_addons_get_num_addons)?;
    table_set_rust_fn(state, c_addons_ref, "GetAddOnInfo", c_addons_get_addon_info)?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "IsAddOnLoaded",
        c_addons_is_addon_loaded,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "IsAddOnLoadOnDemand",
        c_addons_is_addon_load_on_demand,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnEnableState",
        c_addons_get_addon_enable_state,
    )?;
    table_set_rust_fn(state, c_addons_ref, "EnableAddOn", c_addons_enable_addon)?;
    table_set_rust_fn(state, c_addons_ref, "DisableAddOn", c_addons_disable_addon)?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "EnableAllAddOns",
        c_addons_enable_all_addons,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "DisableAllAddOns",
        c_addons_disable_all_addons,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnMetadata",
        c_addons_get_addon_metadata,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "DoesAddOnExist",
        c_addons_does_addon_exist,
    )?;
    table_set_rust_fn(state, c_addons_ref, "GetAddOnName", c_addons_get_addon_name)?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnTitle",
        c_addons_get_addon_title,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnNotes",
        c_addons_get_addon_notes,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnSecurity",
        c_addons_get_addon_security,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "IsAddonVersionCheckEnabled",
        c_addons_is_addon_version_check_enabled,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "SetAddonVersionCheck",
        c_addons_set_addon_version_check,
    )?;
    table_set_rust_fn(state, c_addons_ref, "LoadAddOn", c_addons_load_addon)?;

    let key_ref = state.gc.intern_string(b"C_AddOns");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(
            Val::Str(key_ref),
            Val::Table(c_addons_ref),
            &state.gc.string_arena,
        );
    }
    Ok(())
}

fn register_legacy_addon_globals(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn(state, state.global, "GetNumAddOns", c_addons_get_num_addons)?;
    table_set_rust_fn(
        state,
        state.global,
        "IsAddOnLoaded",
        c_addons_is_addon_loaded,
    )?;
    table_set_rust_fn(
        state,
        state.global,
        "GetAddOnMetadata",
        c_addons_get_addon_metadata,
    )?;
    table_set_rust_fn(
        state,
        state.global,
        "GetAddOnEnableState",
        legacy_get_addon_enable_state,
    )?;
    table_set_rust_fn(
        state,
        state.global,
        "IsAddOnLoadOnDemand",
        c_addons_is_addon_load_on_demand,
    )?;
    table_set_rust_fn(state, state.global, "LoadAddOn", c_addons_load_addon)?;
    let blocked = create_table(state);
    let key_ref = state.gc.intern_string(b"ADDON_ACTIONS_BLOCKED");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(Val::Str(key_ref), blocked, &state.gc.string_arena);
    }
    Ok(())
}

fn register_c_addon_profiler(state: &mut LuaState) -> LuaResult<()> {
    let profiler = create_table(state);
    let Val::Table(profiler_ref) = profiler else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(
        state,
        profiler_ref,
        "GetApplicationMetric",
        c_addon_profiler_get_application_metric,
    )?;
    table_set_rust_fn(
        state,
        profiler_ref,
        "GetOverallMetric",
        c_addon_profiler_get_overall_metric,
    )?;
    table_set_rust_fn(
        state,
        profiler_ref,
        "GetAddOnMetric",
        c_addon_profiler_get_addon_metric,
    )?;
    let key_ref = state.gc.intern_string(b"C_AddOnProfiler");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(Val::Str(key_ref), profiler, &state.gc.string_arena);
    }
    Ok(())
}

fn register_widget_container_mixin(state: &mut LuaState) -> LuaResult<()> {
    let mixin = create_table(state);
    let Val::Table(mixin_ref) = mixin else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(
        state,
        mixin_ref,
        "GetNumWidgetsShowing",
        ui_widget_container_get_num_widgets_showing,
    )?;
    let key_ref = state.gc.intern_string(b"UIWidgetContainerMixin");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(Val::Str(key_ref), mixin, &state.gc.string_arena);
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

fn registry_bool(state: &mut LuaState, key: &str) -> bool {
    matches!(registry_get(state, key), Val::Bool(true))
}

fn set_registry_bool(state: &mut LuaState, key: &str, value: bool) {
    registry_set(state, key, Val::Bool(value));
}

fn addon_index_from_value(state: &LuaState, addon: Val) -> Option<usize> {
    match addon {
        Val::Num(index) if index.is_finite() && index.fract() == 0.0 && index >= 1.0 => {
            let index = index as usize;
            (index > 0).then_some(index - 1)
        }
        Val::Str(_) => {
            let name = val_to_string(state, addon)?;
            let sim = borrow_state(state).ok()?;
            sim.addons
                .iter()
                .position(|candidate| candidate.folder_name == name)
        }
        _ => None,
    }
}

fn addon_name_from_value(state: &LuaState, addon: Val) -> Option<String> {
    match addon {
        Val::Str(_) => val_to_string(state, addon),
        other => {
            let index = addon_index_from_value(state, other)?;
            let sim = borrow_state(state).ok()?;
            sim.addons.get(index).map(|addon| addon.folder_name.clone())
        }
    }
}

fn with_addon<R>(
    state: &LuaState,
    addon: Val,
    f: impl FnOnce(&crate::lua_api::AddonInfo) -> R,
) -> Option<R> {
    let index = addon_index_from_value(state, addon)?;
    let sim = borrow_state(state).ok()?;
    sim.addons.get(index).map(f)
}

fn default_runtime_addon_bases() -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vec![
        root.join("Interface/BlizzardUI"),
        root.join("Interface/AddOns"),
    ]
}

fn find_runtime_addon_toc(state: &LuaState, addon_name: &str) -> Option<PathBuf> {
    let bases = {
        let sim = borrow_state(state).ok()?;
        if sim.addon_base_paths.is_empty() {
            default_runtime_addon_bases()
        } else {
            sim.addon_base_paths.clone()
        }
    };
    for base in bases {
        let addon_dir = base.join(addon_name);
        if let Some(toc_path) = crate::loader::find_toc_file(&addon_dir) {
            return Some(toc_path);
        }
    }
    None
}

fn addon_exists(state: &LuaState, addon_name: &str) -> bool {
    let registered = {
        let sim = match borrow_state(state) {
            Ok(sim) => sim,
            Err(_) => return false,
        };
        sim.addons
            .iter()
            .any(|addon| addon.folder_name == addon_name)
    };
    registered || find_runtime_addon_toc(state, addon_name).is_some()
}

fn addon_metadata(addon: &crate::lua_api::AddonInfo, field: &str) -> Option<String> {
    match field {
        "Title" => Some(addon.title.clone()),
        "Notes" => (!addon.notes.is_empty()).then(|| addon.notes.clone()),
        "Version" => Some("@project-version@".to_string()),
        _ => None,
    }
}

fn push_addon_info(state: &mut LuaState, addon: &crate::lua_api::AddonInfo) -> u32 {
    let folder_name = create_string(state, &addon.folder_name);
    let title = create_string(state, &addon.title);
    let notes = (!addon.notes.is_empty()).then(|| create_string(state, &addon.notes));
    state.push(folder_name);
    state.push(title);
    if addon.notes.is_empty() {
        state.push(Val::Nil);
    } else {
        state.push(notes.unwrap_or(Val::Nil));
    }
    state.push(Val::Bool(addon.enabled));
    4
}

fn c_addons_get_num_addons(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.addons.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn c_addons_get_addon_info(state: &mut LuaState) -> LuaResult<u32> {
    let addon = stack_val(state, 1);
    let Some(index) = addon_index_from_value(state, addon) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = {
        let sim = borrow_state(state)?;
        sim.addons.get(index).cloned()
    };
    let Some(info) = info else {
        state.push(Val::Nil);
        return Ok(1);
    };
    Ok(push_addon_info(state, &info))
}

fn c_addons_is_addon_loaded(state: &mut LuaState) -> LuaResult<u32> {
    let loaded = with_addon(state, stack_val(state, 1), |addon| addon.loaded).unwrap_or(false);
    state.push(Val::Bool(loaded));
    Ok(1)
}

fn c_addons_is_addon_load_on_demand(state: &mut LuaState) -> LuaResult<u32> {
    let load_on_demand =
        with_addon(state, stack_val(state, 1), |addon| addon.load_on_demand).unwrap_or(false);
    state.push(Val::Bool(load_on_demand));
    Ok(1)
}

fn c_addons_get_addon_enable_state(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = with_addon(state, stack_val(state, 1), |addon| addon.enabled).unwrap_or(false);
    state.push(Val::Num(if enabled { 2.0 } else { 0.0 }));
    Ok(1)
}

fn c_addons_enable_addon(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(index) = addon_index_from_value(state, stack_val(state, 1))
        && let Some(addon) = borrow_state_mut(state)?.addons.get_mut(index)
    {
        addon.enabled = true;
    }
    Ok(0)
}

fn c_addons_disable_addon(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(index) = addon_index_from_value(state, stack_val(state, 1))
        && let Some(addon) = borrow_state_mut(state)?.addons.get_mut(index)
        && addon.folder_name != "__BuiltIn"
    {
        addon.enabled = false;
    }
    Ok(0)
}

fn c_addons_enable_all_addons(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    for addon in &mut sim.addons {
        addon.enabled = true;
    }
    Ok(0)
}

fn c_addons_disable_all_addons(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    for addon in &mut sim.addons {
        if addon.folder_name != "__BuiltIn" {
            addon.enabled = false;
        }
    }
    Ok(0)
}

fn c_addons_get_addon_metadata(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = addon_name_from_value(state, stack_val(state, 1)).unwrap_or_default();
    let field = val_to_string(state, stack_val(state, 2)).unwrap_or_default();
    let value = with_addon(state, stack_val(state, 1), |addon| {
        addon_metadata(addon, &field)
    })
    .flatten()
    .or_else(|| (field == "Title").then_some(addon_name));
    match value {
        Some(value) => {
            let value = create_string(state, &value);
            state.push(value);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_does_addon_exist(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = addon_name_from_value(state, stack_val(state, 1)).unwrap_or_default();
    state.push(Val::Bool(
        !addon_name.is_empty() && addon_exists(state, &addon_name),
    ));
    Ok(1)
}

fn c_addons_get_addon_name(state: &mut LuaState) -> LuaResult<u32> {
    let name = with_addon(state, stack_val(state, 1), |addon| {
        addon.folder_name.clone()
    });
    match name {
        Some(name) => {
            let name = create_string(state, &name);
            state.push(name);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_get_addon_title(state: &mut LuaState) -> LuaResult<u32> {
    let title = with_addon(state, stack_val(state, 1), |addon| addon.title.clone());
    match title {
        Some(title) => {
            let title = create_string(state, &title);
            state.push(title);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_get_addon_notes(state: &mut LuaState) -> LuaResult<u32> {
    let notes = with_addon(state, stack_val(state, 1), |addon| addon.notes.clone());
    match notes.filter(|notes| !notes.is_empty()) {
        Some(notes) => {
            let notes = create_string(state, &notes);
            state.push(notes);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_get_addon_security(state: &mut LuaState) -> LuaResult<u32> {
    let security = with_addon(state, stack_val(state, 1), |addon| {
        if addon.folder_name == "__BuiltIn" || addon.folder_name.starts_with("Blizzard_") {
            "SECURE"
        } else {
            "INSECURE"
        }
    })
    .unwrap_or("INSECURE");
    let security = create_string(state, security);
    state.push(security);
    Ok(1)
}

fn c_addons_is_addon_version_check_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = registry_bool(state, ADDON_VERSION_CHECK_KEY);
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn c_addons_set_addon_version_check(state: &mut LuaState) -> LuaResult<u32> {
    set_registry_bool(
        state,
        ADDON_VERSION_CHECK_KEY,
        !matches!(stack_val(state, 1), Val::Nil | Val::Bool(false)),
    );
    Ok(0)
}

fn c_addons_load_addon(state: &mut LuaState) -> LuaResult<u32> {
    let Some(addon_name) = addon_name_from_value(state, stack_val(state, 1)) else {
        let missing = create_string(state, "MISSING");
        state.push(Val::Bool(false));
        state.push(missing);
        return Ok(2);
    };

    if with_addon(state, stack_val(state, 1), |addon| addon.loaded).unwrap_or(false) {
        state.push(Val::Bool(true));
        state.push(Val::Nil);
        return Ok(2);
    }

    let Some(toc_path) = find_runtime_addon_toc(state, &addon_name) else {
        let missing = create_string(state, "MISSING");
        state.push(Val::Bool(false));
        state.push(missing);
        return Ok(2);
    };

    let loader_env = LoaderEnv::from_parts(borrow_lua(state)?, state_handle(state)?);
    match crate::loader::load_addon(&loader_env, &toc_path) {
        Ok(_) => {
            {
                let mut sim = loader_env.state().borrow_mut();
                if let Some(addon) = sim
                    .addons
                    .iter_mut()
                    .find(|addon| addon.folder_name == addon_name)
                {
                    addon.loaded = true;
                    addon.enabled = true;
                }
            }
            let addon_name_val = create_string(state, &addon_name);
            let _ = loader_env.fire_event_with_args("ADDON_LOADED", &[addon_name_val]);
            state.push(Val::Bool(true));
            state.push(Val::Nil);
        }
        Err(error) => {
            let error = create_string(state, &error.to_string());
            state.push(Val::Bool(false));
            state.push(error);
        }
    }
    Ok(2)
}

fn legacy_get_addon_enable_state(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(2.0));
    Ok(1)
}

fn ui_widget_container_get_num_widgets_showing(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(frame_id)
            .map(|frame| {
                frame
                    .children
                    .iter()
                    .filter(|&&child_id| {
                        sim.widgets
                            .get(child_id)
                            .map(|child| child.visible)
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0) as f64
    };
    state.push(Val::Num(count));
    Ok(1)
}

fn profiler_metric_kind(state: &LuaState, metric: Val) -> Option<i32> {
    match metric {
        Val::Num(value) if value.is_finite() && value.fract() == 0.0 => Some(value as i32),
        _ => None,
    }
}

fn average(values: impl Iterator<Item = f64>, count: usize) -> f64 {
    if count == 0 {
        0.0
    } else {
        values.sum::<f64>() / count as f64
    }
}

fn addon_metric_value(addon: &crate::lua_api::AddonInfo, metric: i32) -> f64 {
    match metric {
        0 => {
            if addon.runtime.session_frame_count == 0 {
                0.0
            } else {
                addon.runtime.session_total_ms / addon.runtime.session_frame_count as f64
            }
        }
        1 => average(
            addon.runtime.recent_frames.iter().copied(),
            addon.runtime.recent_frames.len(),
        ),
        4 => addon.runtime.peak_ms,
        _ => 0.0,
    }
}

fn application_metric_value(state: &crate::lua_api::SimState, metric: i32) -> f64 {
    match metric {
        0 => {
            if state.app_frame_metrics.session_frame_count == 0 {
                0.0
            } else {
                state.app_frame_metrics.session_total_ms
                    / state.app_frame_metrics.session_frame_count as f64
            }
        }
        1 => average(
            state.app_frame_metrics.recent_frame_ms.iter().copied(),
            state.app_frame_metrics.recent_frame_ms.len(),
        ),
        4 => state.app_frame_metrics.peak_ms,
        _ => 0.0,
    }
}

fn c_addon_profiler_get_application_metric(state: &mut LuaState) -> LuaResult<u32> {
    let metric = profiler_metric_kind(state, stack_val(state, 1)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        application_metric_value(&sim, metric)
    };
    state.push(Val::Num(value));
    Ok(1)
}

fn c_addon_profiler_get_overall_metric(state: &mut LuaState) -> LuaResult<u32> {
    let metric = profiler_metric_kind(state, stack_val(state, 1)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        sim.addons
            .iter()
            .filter(|addon| addon.folder_name != "__BuiltIn")
            .map(|addon| addon_metric_value(addon, metric))
            .sum::<f64>()
    };
    state.push(Val::Num(value));
    Ok(1)
}

fn c_addon_profiler_get_addon_metric(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = addon_name_from_value(state, stack_val(state, 1)).unwrap_or_default();
    let metric = profiler_metric_kind(state, stack_val(state, 2)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        sim.addons
            .iter()
            .find(|addon| addon.folder_name == addon_name)
            .map(|addon| addon_metric_value(addon, metric))
            .unwrap_or(0.0)
    };
    state.push(Val::Num(value));
    Ok(1)
}
