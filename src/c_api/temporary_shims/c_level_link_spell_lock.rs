//! Temporary spell lock state for `C_LevelLink`.
//!
//! Action locks are backed by simulator state. Spell locks still use a small
//! Lua-visible state table so tests and compatibility probes can seed locks
//! until spell-level progression is modeled.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{
    create_table, create_table_with_fields, table_get_static, table_set_static, val_to_string,
};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const STATE_KEY: &str = "_state";
const LOCKED_SPELLS_KEY: &str = "lockedSpells";
const LAST_SPELL_QUERY_KEY: &str = "lastSpellQuery";
const LOCKED_KEY: &str = "locked";

pub(crate) fn register_c_level_link_spell_lock_shim(state: &mut LuaState) -> LuaResult<()> {
    let namespace_ref = ensure_namespace(state, "C_LevelLink")?;
    let namespace = Val::Table(namespace_ref);
    ensure_level_link_state(state, namespace);
    table_set_rust_fn_static(state, namespace_ref, "IsSpellLocked", is_spell_locked)
}

fn ensure_level_link_state(state: &mut LuaState, namespace: Val) {
    if !matches!(table_get_static(state, namespace, STATE_KEY), Val::Nil) {
        return;
    }

    let locked_spells = create_table(state);
    let state_table = create_table_with_fields(
        state,
        &[
            (LOCKED_SPELLS_KEY, locked_spells),
            (LAST_SPELL_QUERY_KEY, Val::Nil),
        ],
    );
    table_set_static(state, namespace, STATE_KEY, state_table);
}

fn is_spell_locked(state: &mut LuaState) -> LuaResult<u32> {
    let Some(spell_id) = normalize_spell_id(state, stack_val(state, 1)) else {
        set_last_spell_query(state, Val::Nil);
        state.push(Val::Bool(false));
        return Ok(1);
    };

    let lock_entry = spell_lock_entry(state, spell_id);
    set_last_spell_query(state, Val::Num(spell_id));
    let is_locked = lock_entry_is_locked(state, lock_entry);
    state.push(Val::Bool(is_locked));
    Ok(1)
}

fn normalize_spell_id(state: &LuaState, value: Val) -> Option<f64> {
    match value {
        Val::Num(number) => Some(number),
        Val::Str(_) => val_to_string(state, value)?.parse::<f64>().ok(),
        _ => None,
    }
}

fn spell_lock_entry(state: &mut LuaState, spell_id: f64) -> Val {
    let locked_spells = level_link_state_field(state, LOCKED_SPELLS_KEY);
    let Val::Table(locked_spells_ref) = locked_spells else {
        return Val::Nil;
    };
    state
        .gc
        .tables
        .get(locked_spells_ref)
        .map(|table| table.get(Val::Num(spell_id), &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

fn lock_entry_is_locked(state: &mut LuaState, entry: Val) -> bool {
    match entry {
        Val::Bool(locked) => locked,
        Val::Table(_) => table_get_static(state, entry, LOCKED_KEY) == Val::Bool(true),
        _ => false,
    }
}

fn level_link_state_field(state: &mut LuaState, key: &'static str) -> Val {
    let namespace = Val::Table(ensure_namespace(state, "C_LevelLink").expect("namespace"));
    let state_table = table_get_static(state, namespace, STATE_KEY);
    table_get_static(state, state_table, key)
}

fn set_last_spell_query(state: &mut LuaState, value: Val) {
    let namespace = Val::Table(ensure_namespace(state, "C_LevelLink").expect("namespace"));
    let state_table = table_get_static(state, namespace, STATE_KEY);
    table_set_static(state, state_table, LAST_SPELL_QUERY_KEY, value);
}
