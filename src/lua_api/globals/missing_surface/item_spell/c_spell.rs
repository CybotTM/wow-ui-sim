use super::c_item::spell_link_for_id;
use crate::lua_api::globals::{missing_surface::ensure_namespace, spellbook_data};
use crate::lua_api::methods::create_string;
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use crate::spell_descriptions;
use crate::spells;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_c_spell(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Spell")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellDescription",
        c_spell_get_spell_description,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellTexture",
        c_spell_get_spell_texture,
    )?;
    table_set_rust_fn(state, table_ref, "GetSpellLink", c_spell_get_spell_link)?;
    table_set_rust_fn(state, table_ref, "GetSpellName", c_spell_get_spell_name)?;
    Ok(())
}

fn c_spell_get_spell_description(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let description = create_string(
        state,
        spell_descriptions::get_spell_description(spell_id).unwrap_or(""),
    );
    state.push(description);
    Ok(1)
}

fn c_spell_get_spell_texture(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let icon = spells::get_spell(spell_id)
        .map(|spell| spell.icon_file_data_id)
        .unwrap_or(136243);
    let texture = create_string(state, "Interface\\ICONS\\INV_Misc_QuestionMark");
    state.push(texture);
    state.push(Val::Num(icon as f64));
    Ok(2)
}

fn c_spell_get_spell_link(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    match spell_link_for_id(spell_id) {
        Some(link) => {
            let link = create_string(state, &link);
            state.push(link);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_spell_get_spell_name(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let name = spells::get_spell(spell_id)
        .map(|spell| spell.name)
        .unwrap_or("Unknown");
    let name = create_string(state, name);
    state.push(name);
    Ok(1)
}

pub(super) fn register_c_spell_book(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_SpellBook")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellBookItemName",
        c_spell_book_get_spell_book_item_name,
    )?;
    Ok(())
}

fn c_spell_book_get_spell_book_item_name(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    match spellbook_data::get_spell_at_slot(slot) {
        Some((_, entry, _)) => {
            let name = spells::get_spell(entry.spell_id)
                .map(|spell| spell.name)
                .unwrap_or("Unknown");
            let name = create_string(state, name);
            state.push(name);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}
