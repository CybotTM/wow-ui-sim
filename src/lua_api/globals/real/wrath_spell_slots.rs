//! Explicit scenario inputs for Wrath spellbook slot presentation.

use std::collections::HashMap;

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::{FromStack, TableBuilder, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[derive(Clone)]
pub struct SpellSlot {
    pub spell_id: u32,
    pub name: String,
    pub rank: String,
    pub icon: String,
    pub selected: Option<bool>,
}

pub type SpellSlots = HashMap<(String, u32), SpellSlot>;

pub(crate) fn register_admin(builder: TableBuilder) -> LuaResult<TableBuilder> {
    if ACTIVE != ClientProfile::Wrath {
        return Ok(builder);
    }
    builder.set_function("SetWrathSpellBookSlot", set_spell_slot)
}

pub(crate) fn register_queries(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn_static(state, state.global, "GetSpellTexture", get_spell_texture)?;
    table_set_rust_fn_static(state, state.global, "GetSpellName", get_spell_name)?;
    table_set_rust_fn_static(state, state.global, "IsSelectedSpell", is_selected_spell)
}

fn find_spell_slot(state: &mut LuaState) -> LuaResult<Option<SpellSlot>> {
    let slot = match stack_val(state, 1) {
        Val::Num(slot) if (1.0..=1024.0).contains(&slot) && slot.fract() == 0.0 => slot as u32,
        _ => return Err(rilua::runtime_error("Invalid spell slot")),
    };
    let book = String::from_stack(state, 2)?;
    if !matches!(book.as_str(), "spell" | "pet") {
        return Err(rilua::runtime_error("Invalid spell slot"));
    }
    Ok(borrow_state(state)?
        .wrath_spell_slots
        .get(&(book, slot))
        .cloned())
}

fn get_spell_texture(state: &mut LuaState) -> LuaResult<u32> {
    let record = find_spell_slot(state)?;
    let texture = record
        .filter(|record| !record.icon.is_empty())
        .map_or(Val::Nil, |record| create_string(state, &record.icon));
    state.push(texture);
    Ok(1)
}

fn get_spell_name(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(record) = find_spell_slot(state)? {
        let name = create_string(state, &record.name);
        state.push(name);
        let rank = create_string(state, &record.rank);
        state.push(rank);
    } else {
        state.push(Val::Nil);
        state.push(Val::Nil);
    }
    Ok(2)
}

fn is_selected_spell(state: &mut LuaState) -> LuaResult<u32> {
    let selected = find_spell_slot(state)?.and_then(|record| record.selected);
    state.push(if selected == Some(true) {
        Val::Num(1.0)
    } else {
        Val::Nil
    });
    Ok(1)
}

fn set_spell_slot(state: &mut LuaState) -> LuaResult<u32> {
    let book = String::from_stack(state, 1)?;
    let slot = u32::from_stack(state, 2)?;
    if !matches!(book.as_str(), "spell" | "pet") || !(1..=1024).contains(&slot) {
        return Err(rilua::runtime_error(
            "SetWrathSpellBookSlot: invalid book or slot",
        ));
    }
    let record = SpellSlot {
        spell_id: u32::from_stack(state, 3)?,
        name: String::from_stack(state, 4)?,
        rank: String::from_stack(state, 5)?,
        icon: String::from_stack(state, 6)?,
        selected: Option::<bool>::from_stack(state, 7)?,
    };
    borrow_state_mut(state)?
        .wrath_spell_slots
        .insert((book, slot), record);
    Ok(0)
}
