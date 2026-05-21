mod c_container;
mod c_currency;
mod c_equipment_set;
mod c_item;
pub(crate) mod helpers;

use super::c_spell;
use super::c_spell_book;
use super::temporary_shims::c_spell_book_call_pet;
use super::temporary_shims::c_spell_static_fallbacks;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) use c_item::{
    item_link_for_id, parse_item_guid, parse_item_id_from_val, parse_prefixed_id, push_item_info,
    spell_link_for_id,
};
pub(crate) use helpers::current_item_upgrade_location;
pub(crate) use helpers::item_class_name;

pub(crate) fn register_item_and_spell_surfaces(state: &mut LuaState) -> LuaResult<()> {
    c_item::register_c_item(state)?;
    c_container::register_c_item_upgrade(state)?;
    c_container::register_c_container(state)?;
    c_currency::register_c_currency_info(state)?;
    c_equipment_set::register_c_equipment_set(state)?;
    c_currency::register_c_bank(state)?;
    c_spell::register_c_spell_surface(state)?;
    c_spell_static_fallbacks::register_c_spell_static_fallbacks(state)?;
    c_spell_book::register_c_spell_book(state)?;
    c_spell_book_call_pet::register_spell_book_call_pet_shim(state)?;
    Ok(())
}
