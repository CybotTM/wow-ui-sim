//! Explicit scenario inputs for Wrath spellbook slot presentation.

use std::collections::HashMap;

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::{FromStack, TableBuilder};
use rilua::LuaResult;
use rilua::vm::state::LuaState;

#[derive(Clone)]
pub struct SpellSlot {
    pub spell_id: u32,
    pub name: String,
    pub rank: String,
    pub icon: String,
}

pub type SpellSlots = HashMap<(String, u32), SpellSlot>;

pub(crate) fn register_admin(builder: TableBuilder) -> LuaResult<TableBuilder> {
    if ACTIVE != ClientProfile::Wrath {
        return Ok(builder);
    }
    builder.set_function("SetWrathSpellBookSlot", set_spell_slot)
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
    };
    borrow_state_mut(state)?
        .wrath_spell_slots
        .insert((book, slot), record);
    Ok(0)
}
