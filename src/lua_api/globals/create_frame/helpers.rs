//! Low-level helpers shared by the dropdown, template, and top-level modules.

use crate::lua_api::methods::{
    create_string, create_table, extract_frame_id, frame_ref, get_or_create_frame_fields,
    registry_get, table_get, table_set,
};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ---------------------------------------------------------------------------
// Global table helpers
// ---------------------------------------------------------------------------

/// Get a named global value from the Lua global table.
pub(super) fn get_global(state: &mut LuaState, name: &str) -> Val {
    let key = state.gc.intern_string(name.as_bytes());
    state
        .gc
        .tables
        .get(state.global)
        .map(|g| g.get_str(key, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

/// Set a named global to a numeric value.
pub(super) fn set_global_num(state: &mut LuaState, name: &str, value: f64) {
    set_global_raw(state, name, Val::Num(value));
}

/// Set a named global to any Val.
pub(super) fn set_global_raw(state: &mut LuaState, name: &str, value: Val) {
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    if let Some(g) = state.gc.tables.get_mut(global) {
        let _ = g.raw_set(Val::Str(key), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
}

// ---------------------------------------------------------------------------
// Frame field helpers
// ---------------------------------------------------------------------------

/// Set a named field (arg 2) on the frame (arg 1)'s fields table.
pub(super) fn set_frame_field(state: &mut LuaState, field_name: &str) -> LuaResult<u32> {
    let frame: Val = crate::lua_bridge::FromStack::from_stack(state, 1)?;
    let value: Val = crate::lua_bridge::FromStack::from_stack(state, 2)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let fields = get_or_create_frame_fields(state, id);
        let key = create_string(state, field_name);
        if let Val::Table(fields_ref) = fields {
            if let Some(t) = state.gc.tables.get_mut(fields_ref) {
                let _ = t.raw_set(key, value, &state.gc.string_arena);
            }
            state.gc.barrier_back(fields_ref);
        }
    }
    Ok(0)
}

/// Get a named field from the frame (arg 1)'s fields table; push result, return 1.
pub(super) fn get_frame_field(state: &mut LuaState, field_name: &str) -> LuaResult<u32> {
    let frame: Val = crate::lua_bridge::FromStack::from_stack(state, 1)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let fields_registry = registry_get(state, "__rilua_frame_fields");
        if let Val::Table(reg_ref) = fields_registry {
            let frame_fields = state
                .gc
                .tables
                .get(reg_ref)
                .map(|t| t.get_int(id as i64))
                .unwrap_or(Val::Nil);
            if let Val::Table(ff_ref) = frame_fields {
                let key_ref = state.gc.intern_string(field_name.as_bytes());
                let val = state
                    .gc
                    .tables
                    .get(ff_ref)
                    .map(|t| t.get_str(key_ref, &state.gc.string_arena))
                    .unwrap_or(Val::Nil);
                state.push(val);
                return Ok(1);
            }
        }
    }
    state.push(Val::Nil);
    Ok(1)
}

/// Get a string-keyed value from a `Val::Table`.
pub(super) fn table_get_str(state: &mut LuaState, table: Val, key: &str) -> Val {
    let Val::Table(table_ref) = table else {
        return Val::Nil;
    };
    let key_ref = state.gc.intern_string(key.as_bytes());
    state
        .gc
        .tables
        .get(table_ref)
        .map(|t| t.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

// ---------------------------------------------------------------------------
// Frame strata parsing
// ---------------------------------------------------------------------------

pub(super) fn parse_frame_strata(strata: &str) -> crate::widget::FrameStrata {
    match strata.to_uppercase().as_str() {
        "WORLD" | "BACKGROUND" => crate::widget::FrameStrata::Background,
        "LOW" => crate::widget::FrameStrata::Low,
        "MEDIUM" => crate::widget::FrameStrata::Medium,
        "HIGH" => crate::widget::FrameStrata::High,
        "DIALOG" => crate::widget::FrameStrata::Dialog,
        "FULLSCREEN" => crate::widget::FrameStrata::Fullscreen,
        "FULLSCREEN_DIALOG" => crate::widget::FrameStrata::FullscreenDialog,
        "TOOLTIP" => crate::widget::FrameStrata::Tooltip,
        _ => crate::widget::FrameStrata::Medium,
    }
}

// ---------------------------------------------------------------------------
// Mixin helpers
// ---------------------------------------------------------------------------

pub(crate) fn apply_frame_mixins(state: &mut LuaState, frame_id: u64, mixins: Option<&str>) {
    let Some(mixins) = mixins else {
        return;
    };

    for mixin_name in mixins
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
    {
        let mixin_val = resolve_global_path(state, mixin_name);
        copy_table_into_frame(state, frame_id, mixin_val);
    }
}

pub(super) fn resolve_global_path(state: &mut LuaState, path: &str) -> Val {
    let current = resolve_table_path(state, Val::Table(state.global), path);
    if current != Val::Nil {
        return current;
    }
    let secureenv = registry_get(state, "__secureenv");
    resolve_table_path(state, secureenv, path)
}

fn resolve_table_path(state: &mut LuaState, root: Val, path: &str) -> Val {
    let mut current = root;
    for segment in path
        .split('.')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
    {
        let Val::Table(table_ref) = current else {
            return Val::Nil;
        };
        let key = state.gc.intern_string(segment.as_bytes());
        current = state
            .gc
            .tables
            .get(table_ref)
            .map(|table| table.get_str(key, &state.gc.string_arena))
            .unwrap_or(Val::Nil);
    }
    current
}

fn copy_table_into_frame(state: &mut LuaState, frame_id: u64, source: Val) {
    let Val::Table(source_ref) = source else {
        return;
    };
    let frame = frame_ref(state, frame_id).ok();
    let Some(Val::Table(frame_ref_val)) = frame else {
        return;
    };

    copy_table_entries_into_frame(state, frame_ref_val, source_ref);
    let index_key = state.gc.intern_string(b"__index");
    let index_table = state
        .gc
        .tables
        .get(source_ref)
        .and_then(|table| table.metatable())
        .and_then(|mt_ref| state.gc.tables.get(mt_ref))
        .map(|mt| mt.get_str(index_key, &state.gc.string_arena))
        .and_then(|value| match value {
            Val::Table(table_ref) => Some(table_ref),
            _ => None,
        });
    if let Some(index_ref) = index_table {
        copy_table_entries_into_frame(state, frame_ref_val, index_ref);
    }
}

fn copy_table_entries_into_frame(
    state: &mut LuaState,
    frame_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
    source_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) {
    let array_values = state
        .gc
        .tables
        .get(source_ref)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default();
    let hash_entries = collect_filtered_hash_entries(state, source_ref);
    if let Some(fields_table) = state.gc.tables.get_mut(frame_ref) {
        for (index, value) in array_values.into_iter().enumerate() {
            let _ =
                fields_table.raw_set(Val::Num((index + 1) as f64), value, &state.gc.string_arena);
        }
        for (key, value) in hash_entries {
            let _ = fields_table.raw_set(key, value, &state.gc.string_arena);
        }
    }
    state.gc.barrier_back(frame_ref);
}

fn collect_filtered_hash_entries(
    state: &LuaState,
    source_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> Vec<(Val, Val)> {
    state
        .gc
        .tables
        .get(source_ref)
        .map(|table| table.hash_entries())
        .unwrap_or_default()
        .into_iter()
        .filter(|(key, _)| {
            let Val::Str(str_ref) = key else { return true };
            !state
                .gc
                .string_arena
                .get(*str_ref)
                .and_then(|name| name.as_str())
                .is_some_and(|s| {
                    matches!(
                        s,
                        "RegisterCallback" | "UnregisterCallback" | "TriggerEvent"
                    )
                })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Parent array helpers
// ---------------------------------------------------------------------------

pub(super) fn append_parent_array_entry(
    state: &mut LuaState,
    parent_id: u64,
    key: &str,
    child_id: u64,
) {
    let Ok(parent) = frame_ref(state, parent_id) else {
        return;
    };
    let Ok(child) = frame_ref(state, child_id) else {
        return;
    };
    let array = match table_get(state, parent, key) {
        Val::Table(existing) => Val::Table(existing),
        _ => {
            let created = create_table(state);
            table_set(state, parent, key, created);
            created
        }
    };
    let Val::Table(array_ref) = array else {
        return;
    };
    let next_index = state
        .gc
        .tables
        .get(array_ref)
        .map(|table| table.array_slice().len() + 1)
        .unwrap_or(1);
    if let Some(table) = state.gc.tables.get_mut(array_ref) {
        let _ = table.raw_set(Val::Num(next_index as f64), child, &state.gc.string_arena);
    }
    state.gc.barrier_back(array_ref);
}
