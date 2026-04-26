//! `securecallmethod(obj, name, ...)` — calls `obj[name](obj, ...)` via
//! `securecall`. Permissive stub: ignores taint, delegates to the global
//! `securecall` wrapper.

use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

use crate::lua_api::hot_literals::{hot_metatable_key, metatable_idx};
use crate::lua_api::script_helpers::{call_error_handler_state, protected_lua_pcall_state};

pub(super) fn securecallmethod(state: &mut LuaState) -> LuaResult<u32> {
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
