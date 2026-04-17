//! CallbackRegistryMixin equivalents: RegisterCallback, UnregisterCallback, TriggerEvent.

use crate::lua_api::methods::{
    call_function_state, frame_id_from_stack, frame_ref, val_to_string,
};
use crate::lua_api::script_helpers::call_error_handler_state;
use crate::lua_bridge::stack_val;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};

pub(super) const FRAME_CALLBACKS_KEY: &str = "__callbacks";

/// Look up (and optionally create) the per-frame/per-event callback table.
pub(super) fn callback_event_table(
    state: &mut LuaState,
    frame_id: u64,
    event: &str,
    create: bool,
) -> LuaResult<Option<GcRef<Table>>> {
    let frame = frame_ref(state, frame_id)?;
    let Val::Table(frame_ref) = frame else {
        return Ok(None);
    };
    let callbacks = get_or_create_callbacks_table(state, frame_ref, create)?;
    let Some(callbacks) = callbacks else {
        return Ok(None);
    };
    Ok(get_or_create_event_table(state, callbacks, event, create))
}

fn get_or_create_callbacks_table(
    state: &mut LuaState,
    frame_ref: GcRef<Table>,
    create: bool,
) -> LuaResult<Option<GcRef<Table>>> {
    let callbacks_key = state
        .gc
        .intern_string_static(FRAME_CALLBACKS_KEY.as_bytes());
    match state
        .gc
        .tables
        .get(frame_ref)
        .map(|t| t.get_str(callbacks_key, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
    {
        Val::Table(t) => Ok(Some(t)),
        _ if create => {
            let table_ref = state.gc.alloc_table(Table::new());
            if let Some(ft) = state.gc.tables.get_mut(frame_ref) {
                let _ = ft.raw_set(
                    Val::Str(callbacks_key),
                    Val::Table(table_ref),
                    &state.gc.string_arena,
                );
            }
            state.gc.barrier_back(frame_ref);
            Ok(Some(table_ref))
        }
        _ => Ok(None),
    }
}

fn get_or_create_event_table(
    state: &mut LuaState,
    callbacks: GcRef<Table>,
    event: &str,
    create: bool,
) -> Option<GcRef<Table>> {
    let event_key = state.gc.intern_string(event.as_bytes());
    match state
        .gc
        .tables
        .get(callbacks)
        .map(|t| t.get_str(event_key, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
    {
        Val::Table(t) => Some(t),
        _ if create => {
            let table_ref = state.gc.alloc_table(Table::new());
            if let Some(ct) = state.gc.tables.get_mut(callbacks) {
                let _ = ct.raw_set(
                    Val::Str(event_key),
                    Val::Table(table_ref),
                    &state.gc.string_arena,
                );
            }
            state.gc.barrier_back(callbacks);
            Some(table_ref)
        }
        _ => None,
    }
}

pub(super) fn callback_entries(state: &LuaState, event_table: GcRef<Table>) -> Vec<Val> {
    state
        .gc
        .tables
        .get(event_table)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default()
}

pub(super) fn callback_entry_fields(state: &mut LuaState, entry: Val) -> Option<(Val, Val)> {
    let Val::Table(entry_ref) = entry else {
        return None;
    };
    let owner_key = state.gc.intern_string_static(b"owner");
    let func_key = state.gc.intern_string_static(b"func");
    let table = state.gc.tables.get(entry_ref)?;
    Some((
        table.get_str(owner_key, &state.gc.string_arena),
        table.get_str(func_key, &state.gc.string_arena),
    ))
}

pub(super) fn rewrite_callback_entries(
    state: &mut LuaState,
    event_table: GcRef<Table>,
    entries: &[Val],
) {
    let old_len = state
        .gc
        .tables
        .get(event_table)
        .map(|t| t.array_slice().len())
        .unwrap_or(0);
    let new_len = entries.len();
    let clear_to = old_len.max(new_len);
    if let Some(table) = state.gc.tables.get_mut(event_table) {
        for (index, entry) in entries.iter().copied().enumerate() {
            let _ = table.raw_set(Val::Num((index + 1) as f64), entry, &state.gc.string_arena);
        }
        for index in new_len..clear_to {
            let _ = table.raw_set(
                Val::Num((index + 1) as f64),
                Val::Nil,
                &state.gc.string_arena,
            );
        }
    }
    state.gc.barrier_back(event_table);
}

pub(super) fn register_callback(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let event = val_to_string(state, stack_val(state, 2)).ok_or_else(|| {
        runtime_error("CallbackRegistryMixin:RegisterCallback 'event' requires string type.")
    })?;
    let func = stack_val(state, 3);
    if !matches!(func, Val::Function(_)) {
        return Err(runtime_error(
            "CallbackRegistryMixin:RegisterCallback 'func' requires function type.",
        ));
    }
    let owner = match stack_val(state, 4) {
        Val::Nil => func,
        owner => owner,
    };
    if let Some(event_table) = callback_event_table(state, frame_id, &event, true)? {
        let entries = dedup_entries_by_owner(state, event_table, owner);
        let entry_ref = build_callback_entry(state, owner, func);
        let mut new_entries = entries;
        new_entries.push(Val::Table(entry_ref));
        rewrite_callback_entries(state, event_table, &new_entries);
    }
    state.push(owner);
    Ok(1)
}

fn dedup_entries_by_owner(state: &mut LuaState, event_table: GcRef<Table>, owner: Val) -> Vec<Val> {
    callback_entries(state, event_table)
        .into_iter()
        .filter(|entry| {
            callback_entry_fields(state, *entry)
                .map(|(entry_owner, _)| entry_owner != owner)
                .unwrap_or(false)
        })
        .collect()
}

fn build_callback_entry(state: &mut LuaState, owner: Val, func: Val) -> GcRef<Table> {
    let entry_ref = state.gc.alloc_table(Table::new());
    let owner_key = state.gc.intern_string_static(b"owner");
    let func_key = state.gc.intern_string_static(b"func");
    if let Some(entry_table) = state.gc.tables.get_mut(entry_ref) {
        let _ = entry_table.raw_set(Val::Str(owner_key), owner, &state.gc.string_arena);
        let _ = entry_table.raw_set(Val::Str(func_key), func, &state.gc.string_arena);
    }
    state.gc.barrier_back(entry_ref);
    entry_ref
}

pub(super) fn unregister_callback(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let event = val_to_string(state, stack_val(state, 2)).ok_or_else(|| {
        runtime_error("CallbackRegistryMixin:UnregisterCallback 'event' requires string type.")
    })?;
    let owner = stack_val(state, 3);
    if matches!(owner, Val::Nil) {
        return Err(runtime_error(
            "CallbackRegistryMixin:UnregisterCallback 'owner' is required.",
        ));
    }
    if let Some(event_table) = callback_event_table(state, frame_id, &event, false)? {
        let entries = dedup_entries_by_owner(state, event_table, owner);
        rewrite_callback_entries(state, event_table, &entries);
    }
    Ok(0)
}

pub(super) fn trigger_callback_event(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let event = val_to_string(state, stack_val(state, 2)).ok_or_else(|| {
        runtime_error("CallbackRegistryMixin:TriggerEvent 'event' requires string type.")
    })?;
    let args = collect_trigger_args(state);
    let callbacks = callback_event_table(state, frame_id, &event, false)?
        .map(|event_table| callback_entries(state, event_table))
        .unwrap_or_default();
    dispatch_callbacks(state, &callbacks, &args);
    Ok(0)
}

fn collect_trigger_args(state: &LuaState) -> Vec<Val> {
    let arg_count = state.top.saturating_sub(state.base) as i32;
    if arg_count >= 3 {
        (3..=arg_count).map(|idx| stack_val(state, idx)).collect()
    } else {
        Vec::new()
    }
}

fn dispatch_callbacks(state: &mut LuaState, callbacks: &[Val], args: &[Val]) {
    for &entry in callbacks {
        let Some((owner, func)) = callback_entry_fields(state, entry) else {
            continue;
        };
        if matches!(func, Val::Nil) {
            continue;
        }
        let mut call_args = Vec::with_capacity(args.len() + 1);
        call_args.push(owner);
        call_args.extend_from_slice(args);
        if let Err(error) = call_function_state(state, func, &call_args) {
            call_error_handler_state(state, &error.to_string());
        }
    }
}
