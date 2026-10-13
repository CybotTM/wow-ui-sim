//! Explicit ordered spell lists keyed by (specialization ID, item ID).

use std::collections::HashMap;

use super::helpers::{ensure_namespace, validate_supplied_relation_id};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::table_security::unwrap_secret;
use rilua::vm::{state::LuaState, table::Table};
use rilua::{LuaResult, Val};

pub type ItemSetBonuses = HashMap<(i32, i32), Vec<i32>>;

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_Item")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetSetBonusesForSpecializationByItemID",
        get_set_bonuses,
    )
}

fn get_set_bonuses(state: &mut LuaState) -> LuaResult<u32> {
    // Authenticate both originals before parsing either; wrappers stay on the stack.
    let spec_argument = unwrap_secret(state, stack_val(state, 1))?;
    let item_argument = unwrap_secret(state, stack_val(state, 2))?;
    let spec_id = validate_supplied_relation_id(
        spec_argument,
        "C_Item.GetSetBonusesForSpecializationByItemID: specID must be a number",
    )?;
    let item_id = validate_supplied_relation_id(
        item_argument,
        "C_Item.GetSetBonusesForSpecializationByItemID: itemID must be a number",
    )?;
    let spell_ids = {
        let sim = borrow_state(state)?;
        spec_id
            .zip(item_id)
            .and_then(|key| sim.item_set_bonuses.get(&key).cloned())
    };
    // The state borrow ends before allocating the caller-owned result.
    match spell_ids {
        Some(ids) => push_spell_ids(state, &ids)?,
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn push_spell_ids(state: &mut LuaState, spell_ids: &[i32]) -> LuaResult<()> {
    let result = state.gc.alloc_table(Table::new());
    state.push(Val::Table(result));
    let table = state.gc.tables.get_mut(result).ok_or_else(|| {
        rilua::runtime_error("C_Item.GetSetBonusesForSpecializationByItemID: result table missing")
    })?;
    for (index, &spell_id) in spell_ids.iter().enumerate() {
        table.raw_set(
            Val::Num((index + 1) as f64),
            Val::Num(f64::from(spell_id)),
            &state.gc.string_arena,
        )?;
    }
    state.gc.barrier_back(result);
    Ok(())
}
