//! Temporary reincarnation state surface.
//!
//! Reincarnation is not modeled by simulator state yet. This shim preserves the
//! small mutable Lua-side surface used by compatibility probes while keeping it
//! out of the runtime bootstrap.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{
    create_string, create_string_static, create_table_with_fields, table_get_static,
    table_set_static, val_to_string,
};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const STATE_KEY: &str = "_state";
const ACTIVE_KEY: &str = "active";
const CHARACTER_KEY: &str = "character";

pub(crate) fn register_c_reincarnation_shims(state: &mut LuaState) -> LuaResult<()> {
    let namespace_ref = ensure_namespace(state, "C_Reincarnation")?;
    let namespace = Val::Table(namespace_ref);
    ensure_reincarnation_state(state, namespace);
    table_set_rust_fn_static(state, namespace_ref, "IsReincarnating", is_reincarnating)?;
    table_set_rust_fn_static(
        state,
        namespace_ref,
        "GetReincarnatingCharacter",
        get_reincarnating_character,
    )?;
    table_set_rust_fn_static(
        state,
        namespace_ref,
        "StartReincarnation",
        start_reincarnation,
    )?;
    table_set_rust_fn_static(
        state,
        namespace_ref,
        "StopReincarnation",
        stop_reincarnation,
    )
}

fn ensure_reincarnation_state(state: &mut LuaState, namespace: Val) {
    if !matches!(table_get_static(state, namespace, STATE_KEY), Val::Nil) {
        return;
    }

    let state_table = create_table_with_fields(
        state,
        &[(ACTIVE_KEY, Val::Bool(false)), (CHARACTER_KEY, Val::Nil)],
    );
    table_set_static(state, namespace, STATE_KEY, state_table);
}

fn is_reincarnating(state: &mut LuaState) -> LuaResult<u32> {
    let active = reincarnation_state_field(state, ACTIVE_KEY) == Val::Bool(true);
    state.push(Val::Bool(active));
    Ok(1)
}

fn get_reincarnating_character(state: &mut LuaState) -> LuaResult<u32> {
    let character = reincarnation_state_field(state, CHARACTER_KEY);
    state.push(character);
    Ok(1)
}

fn start_reincarnation(state: &mut LuaState) -> LuaResult<u32> {
    if reincarnation_state_field(state, ACTIVE_KEY) == Val::Bool(true) {
        state.push(Val::Bool(false));
        return Ok(1);
    }

    let character_arg = stack_val(state, 1);
    let character = match build_reincarnating_character(state, character_arg) {
        Some(character) => character,
        None => {
            state.push(Val::Bool(false));
            return Ok(1);
        }
    };

    set_reincarnation_state_field(state, ACTIVE_KEY, Val::Bool(true));
    set_reincarnation_state_field(state, CHARACTER_KEY, character);
    state.push(Val::Bool(true));
    Ok(1)
}

fn stop_reincarnation(state: &mut LuaState) -> LuaResult<u32> {
    if reincarnation_state_field(state, ACTIVE_KEY) != Val::Bool(true) {
        state.push(Val::Bool(false));
        return Ok(1);
    }

    set_reincarnation_state_field(state, ACTIVE_KEY, Val::Bool(false));
    set_reincarnation_state_field(state, CHARACTER_KEY, Val::Nil);
    state.push(Val::Bool(true));
    Ok(1)
}

fn reincarnation_state_field(state: &mut LuaState, key: &'static str) -> Val {
    let namespace = Val::Table(ensure_namespace(state, "C_Reincarnation").expect("namespace"));
    let state_table = table_get_static(state, namespace, STATE_KEY);
    table_get_static(state, state_table, key)
}

fn set_reincarnation_state_field(state: &mut LuaState, key: &'static str, value: Val) {
    let namespace = Val::Table(ensure_namespace(state, "C_Reincarnation").expect("namespace"));
    let state_table = table_get_static(state, namespace, STATE_KEY);
    table_set_static(state, state_table, key, value);
}

fn build_reincarnating_character(state: &mut LuaState, character_arg: Val) -> Option<Val> {
    match character_arg {
        Val::Nil => Some(default_reincarnating_character(state)),
        Val::Table(_) => Some(reincarnating_character_from_table(state, character_arg)),
        _ => None,
    }
}

fn default_reincarnating_character(state: &mut LuaState) -> Val {
    let guid = create_string_static(state, "reincarnation-guid");
    let name = create_string_static(state, "Reincarnating Character");
    create_table_with_fields(state, &[("guid", guid), ("name", name)])
}

fn reincarnating_character_from_table(state: &mut LuaState, character: Val) -> Val {
    let guid = string_field_or_empty(state, character, "guid");
    let name = string_field_or_empty(state, character, "name");
    create_table_with_fields(state, &[("guid", guid), ("name", name)])
}

fn string_field_or_empty(state: &mut LuaState, table: Val, key: &'static str) -> Val {
    let field = table_get_static(state, table, key);
    let text = lua_tostring(state, field).unwrap_or_default();
    create_string(state, &text)
}

fn lua_tostring(state: &LuaState, value: Val) -> Option<String> {
    match value {
        Val::Nil => None,
        Val::Bool(true) => Some("true".to_string()),
        Val::Bool(false) => Some("false".to_string()),
        Val::Num(number) => Some(number.to_string()),
        Val::Str(_) => val_to_string(state, value),
        _ => None,
    }
}
