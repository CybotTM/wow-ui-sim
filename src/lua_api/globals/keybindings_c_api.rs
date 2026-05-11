use crate::lua_api::methods::{borrow_state, create_string, create_table};
use crate::lua_bridge::{FromStack, IntoStack, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

use super::{binding_registry_action_index, default_action_for_key};

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

fn get_binding_index(state: &mut LuaState) -> LuaResult<u32> {
    let action = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let user_index = {
        let sim = borrow_state(state)?;
        sim.keybindings
            .base
            .iter()
            .position(|(_, existing_action)| existing_action == &action)
            .map(|index| index + 1)
    };
    let index = user_index.or_else(|| {
        let user_bindings = borrow_state(state).ok()?.keybindings.base.len();
        binding_registry_action_index(&action).map(|registry_index| user_bindings + registry_index)
    });
    match index {
        Some(index) => (index as i32).into_stack(state),
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

fn get_binding_context_for_action(state: &mut LuaState) -> LuaResult<u32> {
    let _ = state;
    Ok(0)
}

fn get_custom_binding_type(state: &mut LuaState) -> LuaResult<u32> {
    let _ = state;
    Ok(0)
}

fn get_search_tags_for_action(state: &mut LuaState) -> LuaResult<u32> {
    create_table(state).into_stack(state)
}

fn get_binding_by_key(state: &mut LuaState) -> LuaResult<u32> {
    let key = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let action = {
        let sim = borrow_state(state)?;
        sim.keybindings.action_for_key(&key)
    };
    if action.is_empty() {
        if let Some(default_action) = default_action_for_key(&key) {
            return create_string(state, default_action).into_stack(state);
        }
    }
    create_string(state, &action).into_stack(state)
}

pub(super) fn register_c_keybindings_namespace(lua: &mut rilua::Lua) -> LuaResult<()> {
    let ns = ensure_namespace_table(lua.state_mut(), "C_KeyBindings");
    table_set_rust_fn_static(lua.state_mut(), ns, "GetBindingIndex", get_binding_index)?;
    table_set_rust_fn_static(
        lua.state_mut(),
        ns,
        "GetBindingContextForAction",
        get_binding_context_for_action,
    )?;
    table_set_rust_fn_static(
        lua.state_mut(),
        ns,
        "GetCustomBindingType",
        get_custom_binding_type,
    )?;
    table_set_rust_fn_static(
        lua.state_mut(),
        ns,
        "GetSearchTagsForAction",
        get_search_tags_for_action,
    )?;
    table_set_rust_fn_static(lua.state_mut(), ns, "GetBindingByKey", get_binding_by_key)?;
    Ok(())
}
