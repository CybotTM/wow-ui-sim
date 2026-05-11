//! Legacy spellbook globals backed by `spellbook_data`.
//!
//! Older Blizzard frames call global spellbook functions instead of the
//! namespaced `C_SpellBook` API. These wrappers keep the legacy return shapes
//! while sharing the same data source as the C API.

use crate::lua_api::globals::spellbook_data;
use crate::lua_api::methods::create_string;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use crate::spells;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_legacy_spell_book_globals(state: &mut LuaState) -> LuaResult<()> {
    register_legacy_spell_book_identity_globals(state)?;
    register_legacy_spell_book_state_globals(state)?;
    register_legacy_spell_book_runtime_globals(state)?;
    Ok(())
}

fn register_legacy_spell_book_identity_globals(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        state.global,
        "GetSpellBookItemName",
        get_spell_book_item_name,
    )?;
    table_set_rust_fn_static(
        state,
        state.global,
        "GetSpellBookItemInfo",
        get_spell_book_item_info,
    )?;
    table_set_rust_fn_static(
        state,
        state.global,
        "GetSpellBookItemTexture",
        get_spell_book_item_texture,
    )?;
    Ok(())
}

fn register_legacy_spell_book_state_globals(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn_static(state, state.global, "IsPassiveSpell", is_passive_spell)?;
    table_set_rust_fn_static(state, state.global, "GetSpecsForSpell", get_specs_for_spell)?;
    Ok(())
}

fn register_legacy_spell_book_runtime_globals(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        state.global,
        "GetSpellBookItemCooldown",
        get_spell_book_item_cooldown,
    )?;
    Ok(())
}

fn get_spell_book_item_name(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let Some((_, entry, _)) = spellbook_data::get_spell_at_slot(slot) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let spell = spells::get_spell(entry.spell_id);
    let name = spell.map(|spell| spell.name).unwrap_or("Unknown");
    let name = create_string(state, name);
    let sub_name = create_string(state, "");
    state.push(name);
    state.push(sub_name);
    state.push(Val::Num(entry.spell_id as f64));
    Ok(3)
}

fn get_spell_book_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let Some((_, entry, _)) = spellbook_data::get_spell_at_slot(slot) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let slot_type = create_string(state, "SPELL");
    state.push(slot_type);
    state.push(Val::Num(entry.spell_id as f64));
    state.push(Val::Num(entry.spell_id as f64));
    state.push(Val::Bool(entry.is_passive));
    Ok(4)
}

fn get_spell_book_item_texture(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let Some((_, entry, _)) = spellbook_data::get_spell_at_slot(slot) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let icon_id = spells::get_spell(entry.spell_id)
        .map(|spell| spell.icon_file_data_id)
        .unwrap_or(136243);
    state.push(Val::Num(icon_id as f64));
    Ok(1)
}

fn is_passive_spell(state: &mut LuaState) -> LuaResult<u32> {
    let first_arg = i32::from_stack(state, 1)?;
    let spell_id = legacy_spell_id_from_arg(first_arg, has_book_type_arg(state));
    let is_passive = spell_id.and_then(passive_spellbook_entry).unwrap_or(false);
    state.push(Val::Bool(is_passive));
    Ok(1)
}

fn get_specs_for_spell(state: &mut LuaState) -> LuaResult<u32> {
    let first_arg = i32::from_stack(state, 1)?;
    let spell_id = legacy_spell_id_from_arg(first_arg, has_book_type_arg(state));
    let Some(skill_line) = spell_id.and_then(spell_skill_line) else {
        return Ok(0);
    };
    if skill_line.spec_id.is_none() && skill_line.off_spec_id.is_none() {
        return Ok(0);
    }
    let spec_name = create_string(state, skill_line.name);
    state.push(spec_name);
    Ok(1)
}

fn get_spell_book_item_cooldown(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    if spellbook_data::get_spell_at_slot(slot).is_none() {
        state.push(Val::Nil);
        return Ok(1);
    }

    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(1.0));
    state.push(Val::Num(1.0));
    Ok(4)
}

fn legacy_spell_id_from_arg(first_arg: i32, has_book_type_arg: bool) -> Option<u32> {
    if has_book_type_arg {
        return spellbook_data::get_spell_at_slot(first_arg).map(|(_, entry, _)| entry.spell_id);
    }
    u32::try_from(first_arg).ok()
}

fn has_book_type_arg(state: &LuaState) -> bool {
    !matches!(stack_val(state, 2), Val::Nil)
}

fn passive_spellbook_entry(spell_id: u32) -> Option<bool> {
    let (slot, _) = spellbook_data::find_spell_slot(spell_id)?;
    spellbook_data::get_spell_at_slot(slot).map(|(_, entry, _)| entry.is_passive)
}

fn spell_skill_line(spell_id: u32) -> Option<&'static spellbook_data::SkillLineData> {
    let (slot, _) = spellbook_data::find_spell_slot(spell_id)?;
    spellbook_data::get_spell_at_slot(slot).map(|(_, _, skill_line)| skill_line)
}
