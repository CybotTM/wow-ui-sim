mod c_container;
mod c_currency;
mod c_item;
mod c_spell;
mod helpers;

pub(super) use c_item::{parse_item_guid, parse_prefixed_id, spell_link_for_id};
pub(super) use helpers::current_item_upgrade_location;

use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(super) fn register_item_and_spell_surfaces(state: &mut LuaState) -> LuaResult<()> {
    c_item::register_c_item(state)?;
    c_container::register_c_item_upgrade(state)?;
    c_container::register_c_container(state)?;
    c_currency::register_c_currency_info(state)?;
    c_currency::register_c_equipment_set(state)?;
    c_currency::register_c_bank(state)?;
    c_spell::register_c_spell(state)?;
    c_spell::register_c_spell_book(state)?;
    Ok(())
}

pub(crate) fn item_link_for_id(item_id: u32) -> Option<String> {
    c_item::item_link_for_id(item_id)
}
