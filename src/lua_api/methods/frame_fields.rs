//! Per-frame debug field table helpers.

use rilua::Val;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

use super::{registry_get, registry_table_or_create, table_get_num};

/// Get or create the per-frame fields table in the `__rilua_frame_fields`
/// registry entry.
pub fn get_or_create_frame_fields(state: &mut LuaState, frame_id: u64) -> Val {
    let fields_registry = registry_table_or_create(state, "__rilua_frame_fields");
    let Val::Table(fields_reg_ref) = fields_registry else {
        return Val::Nil;
    };

    let existing = get_frame_fields_from_registry(state, fields_reg_ref, frame_id);
    if let Val::Table(_) = existing {
        return existing;
    }

    let created = Val::Table(state.gc.alloc_table(Table::with_sizes(0, 4)));
    copy_frame_table_fields_into_fields(state, frame_id, created);
    if let Some(reg) = state.gc.tables.get_mut(fields_reg_ref) {
        let _ = reg.raw_set(Val::Num(frame_id as f64), created, &state.gc.string_arena);
    }
    state.gc.barrier_back(fields_reg_ref);
    created
}

/// Return an existing per-frame fields table without creating one.
pub fn get_existing_frame_fields(state: &mut LuaState, frame_id: u64) -> Val {
    let fields_registry = registry_get(state, "__rilua_frame_fields");
    let Val::Table(fields_reg_ref) = fields_registry else {
        return Val::Nil;
    };
    get_frame_fields_from_registry(state, fields_reg_ref, frame_id)
}

fn get_frame_fields_from_registry(
    state: &LuaState,
    fields_reg_ref: GcRef<Table>,
    frame_id: u64,
) -> Val {
    state
        .gc
        .tables
        .get(fields_reg_ref)
        .map(|t| {
            let int_val = t.get_int(frame_id as i64);
            if int_val != Val::Nil {
                int_val
            } else {
                t.get(Val::Num(frame_id as f64), &state.gc.string_arena)
            }
        })
        .unwrap_or(Val::Nil)
}

fn copy_frame_table_fields_into_fields(state: &mut LuaState, frame_id: u64, fields: Val) {
    let Some(Val::Table(frame_ref)) = existing_frame_ref(state, frame_id) else {
        return;
    };
    let Val::Table(fields_ref) = fields else {
        return;
    };

    let array_values = state
        .gc
        .tables
        .get(frame_ref)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default();
    let hash_entries = state
        .gc
        .tables
        .get(frame_ref)
        .map(|table| table.hash_entries())
        .unwrap_or_default();

    if let Some(fields_table) = state.gc.tables.get_mut(fields_ref) {
        for (index, value) in array_values.into_iter().enumerate() {
            if value != Val::Nil {
                let _ = fields_table.raw_set(
                    Val::Num((index + 1) as f64),
                    value,
                    &state.gc.string_arena,
                );
            }
        }
        for (key, value) in hash_entries {
            let _ = fields_table.raw_set(key, value, &state.gc.string_arena);
        }
    }
    state.gc.barrier_back(fields_ref);
}

fn existing_frame_ref(state: &mut LuaState, frame_id: u64) -> Option<Val> {
    let cache = registry_get(state, "__rilua_frame_refs");
    let Val::Table(cache_ref) = cache else {
        return None;
    };
    let existing = table_get_num(state, cache_ref, frame_id as f64);
    if existing == Val::Nil {
        None
    } else {
        Some(existing)
    }
}
