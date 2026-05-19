//! WoW taint integration for rilua.

use rilua::LuaApiMut;
use rilua::vm::state::LuaState;

/// Enable taint tracking on the rilua state.
pub fn enable_taint_mode(lua: &mut rilua::Lua) {
    lua.state_mut().taint_mode = true;
}

/// Stamp addon taint on a compiled function.
///
/// Stores `addon_name` in the `__closure_taint` registry table keyed by
/// the closure's GC arena index. When the VM calls this function, the
/// taint propagation system reads it back to set the call frame's taint.
pub fn stamp_addon_taint(lua: &mut rilua::Lua, func: &rilua::Function, addon_name: &str) {
    stamp_addon_taint_state(lua.state_mut(), func, addon_name);
}

pub fn stamp_addon_taint_state(state: &mut LuaState, func: &rilua::Function, addon_name: &str) {
    let cl_ref = func.gc_ref();
    let taint_table = get_or_create_closure_taint_table(state);
    let key = rilua::Val::Num(cl_ref.index() as f64);
    let val = rilua::Val::Str(state.gc.intern_string(addon_name.as_bytes()));
    if let Some(t) = state.gc.tables.get_mut(taint_table) {
        let _ = t.raw_set(key, val, &state.gc.string_arena);
    }
}

const CLOSURE_TAINT_KEY: &str = "__closure_taint";

fn get_or_create_closure_taint_table(
    state: &mut LuaState,
) -> rilua::vm::gc::arena::GcRef<rilua::vm::table::Table> {
    let key = state.gc.intern_string_static(CLOSURE_TAINT_KEY.as_bytes());
    if let Some(reg) = state.gc.tables.get(state.registry) {
        if let rilua::Val::Table(t) = reg.get_str(key, &state.gc.string_arena) {
            return t;
        }
    }
    let new_table = state.gc.alloc_table(rilua::vm::table::Table::new());
    if let Some(reg) = state.gc.tables.get_mut(state.registry) {
        let _ = reg.raw_set(
            rilua::Val::Str(key),
            rilua::Val::Table(new_table),
            &state.gc.string_arena,
        );
    }
    new_table
}

/// Set taint for the current frame before dispatching a script handler.
pub fn set_frame_taint(state: &mut LuaState, addon_name: Option<&str>) {
    if let Some(ci) = state.call_stack.get_mut(state.ci) {
        ci.taint = addon_name.map(|s| s.to_string());
    }
}

/// Clear taint for secure (Blizzard) code execution.
pub fn clear_frame_taint(state: &mut LuaState) {
    if let Some(ci) = state.call_stack.get_mut(state.ci) {
        ci.taint = None;
    }
}

pub fn clear_active_stack_taint(state: &mut LuaState) -> Vec<Option<String>> {
    let active_depth = state.ci.saturating_add(1);
    let mut saved_taints = Vec::with_capacity(active_depth);
    for call_info in state.call_stack.iter_mut().take(active_depth) {
        saved_taints.push(call_info.taint.clone());
        call_info.taint = None;
    }
    saved_taints
}

pub fn restore_active_stack_taint(state: &mut LuaState, saved_taints: Vec<Option<String>>) {
    for (call_info, taint) in state.call_stack.iter_mut().zip(saved_taints) {
        call_info.taint = taint;
    }
}
