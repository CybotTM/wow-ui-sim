//! `C_Debug` debug-window helpers.

use crate::lua_api::methods::{borrow_state_mut, create_table, val_to_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

fn ensure_namespace_table(state: &mut LuaState, namespace: &'static str) -> GcRef<Table> {
    let key = state.gc.intern_string_static(namespace.as_bytes());
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|table| table.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(table_ref)) = existing {
        return table_ref;
    }

    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), table, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    table_ref
}

fn append_console_line(state: &mut LuaState, line: String) {
    if let Ok(mut sim) = borrow_state_mut(state) {
        sim.console_output.push(line);
    }
}

fn format_stack_values(state: &LuaState, values: &[Val]) -> String {
    let mut output = String::new();
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push('\t');
        }
        output.push_str(&format_stack_value(state, *value));
    }
    output
}

fn format_stack_value(state: &LuaState, value: Val) -> String {
    match value {
        Val::Nil => "nil".to_string(),
        Val::Bool(true) => "true".to_string(),
        Val::Bool(false) => "false".to_string(),
        Val::Num(number) => format_stack_number(number),
        Val::Str(_) => val_to_string(state, value).unwrap_or_default(),
        _ => format!("{value:?}"),
    }
}

fn format_stack_number(number: f64) -> String {
    if number.fract() == 0.0 && number.abs() < 1e15 {
        format!("{}", number as i64)
    } else {
        format!("{number}")
    }
}

fn collect_stack_values(state: &LuaState) -> Vec<Val> {
    let nargs = (state.top as i32 - state.base as i32).max(0) as usize;
    (0..nargs)
        .map(|index| stack_val(state, (index + 1) as i32))
        .collect()
}

fn print_to_debug_window(state: &mut LuaState) -> LuaResult<u32> {
    let values = collect_stack_values(state);
    append_console_line(state, format_stack_values(state, &values));
    Ok(0)
}

fn view_in_debug_window(state: &mut LuaState) -> LuaResult<u32> {
    let values = collect_stack_values(state);
    append_console_line(state, format_stack_values(state, &values));
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    let state = lua.state_mut();
    let ns = ensure_namespace_table(state, "C_Debug");
    table_set_rust_fn_static(state, ns, "PrintToDebugWindow", print_to_debug_window)?;
    table_set_rust_fn_static(state, ns, "ViewInDebugWindow", view_in_debug_window)?;
    Ok(())
}
