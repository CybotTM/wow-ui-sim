//! `SetCVar` / `GetCVar` globals + `C_CVar.{Set,Get,GetDefault}CVar`.
//! All routes through `SimState.cvars` (which is seeded from
//! `cvars.yaml` and persists overrides to disk) so the in-game CVar
//! surface, the in-Lua bootstrap defaults, and the simulator's
//! Rust-side knobs (e.g. `Brightness=50`, `Contrast=50`, `Gamma=1.0`
//! from `cvars.yaml`) all read the same store.
//!
//! Previously `SetCVar` was registered only on the `A_Admin` path and
//! the retail-facing global was `stub_nil`; `GetCVar` came from
//! `Blizzard_SharedXMLBase/CvarUtil.lua` which routed through the
//! Lua-only `__cvars` table — so `SimState.cvars` defaults like the
//! display sliders were invisible to addon code. Now they're not.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

fn required_string(state: &mut LuaState, index: i32) -> Option<String> {
    Option::<String>::from_stack(state, index)
        .ok()
        .flatten()
        .filter(|s| !s.is_empty())
}

fn value_to_string(state: &mut LuaState, index: i32) -> String {
    match stack_val(state, index) {
        Val::Nil => String::new(),
        Val::Bool(true) => "1".to_string(),
        Val::Bool(false) => "0".to_string(),
        Val::Num(n) => {
            if n.fract() == 0.0 {
                format!("{}", n as i64)
            } else {
                format!("{n}")
            }
        }
        Val::Str(_) => Option::<String>::from_stack(state, index)
            .ok()
            .flatten()
            .unwrap_or_default(),
        _ => String::new(),
    }
}

/// `SetCVar(name, value)` — write `value` (stringified) to
/// `SimState.cvars[name]`. Returns true on success, matching retail.
fn set_cvar(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = required_string(state, 1) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let value = value_to_string(state, 2);
    let accepted = borrow_state_mut(state)?.cvars.set(&name, &value);
    state.push(Val::Bool(accepted));
    Ok(1)
}

/// `GetCVar(name)` — read `SimState.cvars[name]` (override → default).
/// Returns nil for unknown names, matching retail's behaviour for
/// CVars that haven't been registered.
fn get_cvar(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = required_string(state, 1) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let value = borrow_state(state)?.cvars.get(&name);
    push_optional_string(state, value);
    Ok(1)
}

/// `GetCVarDefault(name)` — read the YAML/factory default for `name`,
/// ignoring any session override.
fn get_cvar_default(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = required_string(state, 1) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let value = borrow_state(state)?.cvars.get_default(&name);
    push_optional_string(state, value);
    Ok(1)
}

/// `GetCVarBool(name)` — `true` iff the stored value is `"1"`.
fn get_cvar_bool(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = required_string(state, 1) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let value = borrow_state(state)?.cvars.get_bool(&name);
    state.push(Val::Bool(value));
    Ok(1)
}

fn push_optional_string(state: &mut LuaState, value: Option<String>) {
    match value {
        Some(value) => {
            let val = create_string(state, &value);
            state.push(val);
        }
        None => state.push(Val::Nil),
    }
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "SetCVar", set_cvar)?;
    LuaApiMut::register_function(lua, "GetCVar", get_cvar)?;
    LuaApiMut::register_function(lua, "GetCVarDefault", get_cvar_default)?;
    LuaApiMut::register_function(lua, "GetCVarBool", get_cvar_bool)?;
    install_c_cvar_namespace(lua)?;
    Ok(())
}

/// Override the bootstrap-defined `C_CVar.{Set,Get,GetBool,GetDefault}CVar`
/// with Rust impls that route through `SimState.cvars`. The bootstrap's
/// fallbacks read/write a Lua-only `__cvars` table — that table doesn't
/// know about the YAML defaults (Brightness/Contrast/Gamma/etc.), so
/// the in-game settings sliders would otherwise see nil for them.
fn install_c_cvar_namespace(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let table_ref = ensure_c_cvar_table(state);
    crate::lua_bridge::table_set_rust_fn_static(state, table_ref, "GetCVar", get_cvar)?;
    crate::lua_bridge::table_set_rust_fn_static(state, table_ref, "SetCVar", set_cvar)?;
    crate::lua_bridge::table_set_rust_fn_static(state, table_ref, "GetCVarBool", get_cvar_bool)?;
    crate::lua_bridge::table_set_rust_fn_static(
        state,
        table_ref,
        "GetCVarDefault",
        get_cvar_default,
    )?;
    Ok(())
}

fn ensure_c_cvar_table(state: &mut LuaState) -> rilua::vm::gc::arena::GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_CVar");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(r)) = existing {
        return r;
    }
    let new_val = crate::lua_api::methods::create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}
