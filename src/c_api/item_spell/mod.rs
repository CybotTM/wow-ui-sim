mod c_currency;
mod c_item;
pub(crate) mod helpers;

use super::temporary_shims::item_spell::{c_container, c_spell};
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
    c_currency::register_c_equipment_set(state)?;
    c_currency::register_c_bank(state)?;
    c_spell::register_c_spell(state)?;
    c_spell::register_c_spell_book(state)?;
    Ok(())
}
