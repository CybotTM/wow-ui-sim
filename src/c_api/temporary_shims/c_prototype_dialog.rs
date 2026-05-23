//! Temporary `C_PrototypeDialog` dialog-state surface.
//!
//! Prototype dialog server state is not modeled yet. This shim keeps the small
//! mutable compatibility surface out of the generic runtime bootstrap until a
//! real dialog model owns active/removed dialog transitions.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{
    create_string_static, table_get_static, table_set_num, table_set_static,
};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const ACTIVE_DIALOGS_KEY: &str = "_activeDialogs";
const REMOVED_DIALOGS_KEY: &str = "_removedDialogs";
const TRANSITION_HISTORY_KEY: &str = "_transitionHistory";

pub(crate) fn register_c_prototype_dialog_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns_ref = ensure_namespace(state, "C_PrototypeDialog")?;
    let namespace = Val::Table(ns_ref);
    ensure_prototype_dialog_state(state, namespace);
    table_set_rust_fn_static(state, ns_ref, "SelectOption", select_option)?;
    table_set_rust_fn_static(state, ns_ref, "EnsureRemoved", ensure_removed)
}

fn ensure_prototype_dialog_state(state: &mut LuaState, namespace: Val) {
    ensure_table_field(state, namespace, ACTIVE_DIALOGS_KEY, 4);
    ensure_table_field(state, namespace, REMOVED_DIALOGS_KEY, 4);
    ensure_table_field(state, namespace, TRANSITION_HISTORY_KEY, 4);
}

fn select_option(state: &mut LuaState) -> LuaResult<u32> {
    let Some(dialog_id) = number_argument(state, 1) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let Some(option_id) = number_argument(state, 2) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };

    let namespace = prototype_dialog_namespace(state);
    let active_dialogs = ensure_table_field(state, namespace, ACTIVE_DIALOGS_KEY, 4);
    let removed_dialogs = ensure_table_field(state, namespace, REMOVED_DIALOGS_KEY, 4);
    let transition_history = ensure_table_field(state, namespace, TRANSITION_HISTORY_KEY, 4);
    let selection_count = next_selection_count(state, active_dialogs, dialog_id);

    let dialog_state = dialog_state_table(state, dialog_id, option_id, selection_count);
    table_set_number_key(state, active_dialogs, dialog_id, dialog_state);
    table_set_number_key(state, removed_dialogs, dialog_id, Val::Nil);
    let transition = selected_transition_table(state, dialog_id, option_id, selection_count);
    append_transition(state, transition_history, transition);

    state.push(Val::Bool(true));
    Ok(1)
}

fn ensure_removed(state: &mut LuaState) -> LuaResult<u32> {
    let Some(dialog_id) = number_argument(state, 1) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };

    let namespace = prototype_dialog_namespace(state);
    let active_dialogs = ensure_table_field(state, namespace, ACTIVE_DIALOGS_KEY, 4);
    let removed_dialogs = ensure_table_field(state, namespace, REMOVED_DIALOGS_KEY, 4);
    let transition_history = ensure_table_field(state, namespace, TRANSITION_HISTORY_KEY, 4);
    let had_active_dialog = table_get_number_key(state, active_dialogs, dialog_id) != Val::Nil;

    table_set_number_key(state, active_dialogs, dialog_id, Val::Nil);
    table_set_number_key(state, removed_dialogs, dialog_id, Val::Bool(true));
    let transition = removed_transition_table(state, dialog_id);
    append_transition(state, transition_history, transition);

    state.push(Val::Bool(had_active_dialog));
    Ok(1)
}

fn prototype_dialog_namespace(state: &mut LuaState) -> Val {
    ensure_namespace(state, "C_PrototypeDialog")
        .map(Val::Table)
        .unwrap_or(Val::Nil)
}

fn ensure_table_field(
    state: &mut LuaState,
    namespace: Val,
    key: &'static str,
    hash_capacity: usize,
) -> Val {
    let existing = table_get_static(state, namespace, key);
    if matches!(existing, Val::Table(_)) {
        return existing;
    }

    let table = Val::Table(state.gc.alloc_table(Table::with_sizes(0, hash_capacity)));
    table_set_static(state, namespace, key, table);
    table
}

fn number_argument(state: &mut LuaState, index: i32) -> Option<f64> {
    match stack_val(state, index) {
        Val::Num(value) => Some(value),
        _ => None,
    }
}

fn next_selection_count(state: &mut LuaState, active_dialogs: Val, dialog_id: f64) -> f64 {
    let prior_state = table_get_number_key(state, active_dialogs, dialog_id);
    let prior_count = table_get_static(state, prior_state, "selectionCount");
    match prior_count {
        Val::Num(count) => count + 1.0,
        _ => 1.0,
    }
}

fn dialog_state_table(
    state: &mut LuaState,
    dialog_id: f64,
    option_id: f64,
    selection_count: f64,
) -> Val {
    let table = new_table_with_hash_capacity(state, 3);
    table_set_static(state, table, "dialogID", Val::Num(dialog_id));
    table_set_static(state, table, "selectedOptionID", Val::Num(option_id));
    table_set_static(state, table, "selectionCount", Val::Num(selection_count));
    table
}

fn selected_transition_table(
    state: &mut LuaState,
    dialog_id: f64,
    option_id: f64,
    selection_count: f64,
) -> Val {
    let table = new_table_with_hash_capacity(state, 4);
    let transition = create_string_static(state, "selected");
    table_set_static(state, table, "transition", transition);
    table_set_static(state, table, "dialogID", Val::Num(dialog_id));
    table_set_static(state, table, "optionID", Val::Num(option_id));
    table_set_static(state, table, "selectionCount", Val::Num(selection_count));
    table
}

fn removed_transition_table(state: &mut LuaState, dialog_id: f64) -> Val {
    let table = new_table_with_hash_capacity(state, 2);
    let transition = create_string_static(state, "removed");
    table_set_static(state, table, "transition", transition);
    table_set_static(state, table, "dialogID", Val::Num(dialog_id));
    table
}

fn new_table_with_hash_capacity(state: &mut LuaState, hash_capacity: usize) -> Val {
    Val::Table(state.gc.alloc_table(Table::with_sizes(0, hash_capacity)))
}

fn table_get_number_key(state: &LuaState, table: Val, key: f64) -> Val {
    let Val::Table(table_ref) = table else {
        return Val::Nil;
    };
    state
        .gc
        .tables
        .get(table_ref)
        .map(|table| table.get(Val::Num(key), &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

fn table_set_number_key(state: &mut LuaState, table: Val, key: f64, value: Val) {
    let Val::Table(table_ref) = table else {
        return;
    };
    table_set_num(state, table_ref, key, value);
}

fn append_transition(state: &mut LuaState, history: Val, transition: Val) {
    let Val::Table(history_ref) = history else {
        return;
    };
    let next_index = state
        .gc
        .tables
        .get(history_ref)
        .map(|table| table.array_slice().len() + 1)
        .unwrap_or(1);
    table_set_num(state, history_ref, next_index as f64, transition);
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn preserves_existing_namespace_members() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: String = env
            .eval(
                r#"
                C_PrototypeDialog.ExistingMember = "kept"
                return C_PrototypeDialog.ExistingMember
                "#,
            )
            .expect("existing prototype dialog member should be readable");

        assert_eq!(result, "kept");
    }
}
