use super::c_item::spell_link_for_id;
use crate::lua_api::globals::{missing_surface::ensure_namespace, spellbook_data};
use crate::lua_api::methods::borrow_state;
use crate::lua_api::methods::{create_string, create_table, table_set};
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
    table_set_rust_fn(
        state,
        table_ref,
        "GetMawPowerBorderAtlasBySpellID",
        c_spell_get_maw_power_border_atlas_by_spell_id,
    )?;
    table_set_rust_fn(state, table_ref, "GetSpellLink", c_spell_get_spell_link)?;
    table_set_rust_fn(state, table_ref, "GetSpellName", c_spell_get_spell_name)?;
    table_set_rust_fn(state, table_ref, "IsSpellPassive", c_spell_is_spell_passive)?;
    table_set_rust_fn(
        state,
        table_ref,
        "IsAutoAttackSpell",
        c_spell_is_auto_attack_spell,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "IsRangedAutoAttackSpell",
        c_spell_is_ranged_auto_attack_spell,
    )?;
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

fn c_spell_get_maw_power_border_atlas_by_spell_id(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
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

fn c_spell_is_spell_passive(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_spell_is_auto_attack_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Bool(spell_id == 6603));
    Ok(1)
}

fn c_spell_is_ranged_auto_attack_spell(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

pub(super) fn register_c_spell_book(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_SpellBook")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetNumSpellBookSkillLines",
        c_spell_book_get_num_spell_book_skill_lines,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellBookSkillLineInfo",
        c_spell_book_get_spell_book_skill_line_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellBookItemName",
        c_spell_book_get_spell_book_item_name,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellBookItemInfo",
        c_spell_book_get_spell_book_item_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellBookItemType",
        c_spell_book_get_spell_book_item_type,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellBookItemCooldown",
        c_spell_book_get_spell_book_item_cooldown,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellBookItemAutoCast",
        c_spell_book_get_spell_book_item_auto_cast,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "HasPetSpells",
        c_spell_book_has_pet_spells,
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

fn c_spell_book_get_spell_book_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    match spellbook_item_info(state, slot) {
        Some(info) => state.push(info),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_spell_book_get_spell_book_item_type(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    let Some((_, entry, _)) = spellbook_data::get_spell_at_slot(slot) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    state.push(Val::Num(1.0));
    state.push(Val::Num(entry.spell_id as f64));
    state.push(Val::Num(entry.spell_id as f64));
    Ok(3)
}

fn c_spell_book_get_spell_book_item_cooldown(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    if spellbook_data::get_spell_at_slot(slot).is_none() {
        state.push(Val::Nil);
        return Ok(1);
    }

    let cooldown = create_table(state);
    table_set(state, cooldown, "startTime", Val::Num(0.0));
    table_set(state, cooldown, "duration", Val::Num(0.0));
    table_set(state, cooldown, "isEnabled", Val::Bool(false));
    table_set(state, cooldown, "modRate", Val::Num(1.0));
    state.push(cooldown);
    Ok(1)
}

fn c_spell_book_get_spell_book_item_auto_cast(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    if spellbook_data::get_spell_at_slot(slot).is_none() {
        state.push(Val::Bool(false));
        state.push(Val::Bool(false));
        return Ok(2);
    }
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    Ok(2)
}

fn c_spell_book_get_num_spell_book_skill_lines(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(spellbook_data::num_skill_lines() as f64));
    Ok(1)
}

fn c_spell_book_get_spell_book_skill_line_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let Some(skill_line) = spellbook_data::get_skill_line(index) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info = create_table(state);
    let name = create_string(state, skill_line.name);
    table_set(state, info, "name", name);
    table_set(
        state,
        info,
        "itemIndexOffset",
        Val::Num(spellbook_data::skill_line_offset(index) as f64),
    );
    table_set(
        state,
        info,
        "numSpellBookItems",
        Val::Num(skill_line.spells.len() as f64),
    );
    table_set(
        state,
        info,
        "specID",
        skill_line
            .spec_id
            .map(|id| Val::Num(id as f64))
            .unwrap_or(Val::Nil),
    );
    table_set(
        state,
        info,
        "offSpecID",
        skill_line
            .off_spec_id
            .map(|id| Val::Num(id as f64))
            .unwrap_or(Val::Nil),
    );
    table_set(state, info, "shouldHide", Val::Bool(false));
    table_set(state, info, "iconID", Val::Num(skill_line.icon_id as f64));
    state.push(info);
    Ok(1)
}

fn c_spell_book_has_pet_spells(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.pet_spells.len();
    if count == 0 {
        state.push(Val::Bool(false));
    } else {
        state.push(Val::Num(count as f64));
    }
    Ok(1)
}

fn spellbook_item_info(state: &mut LuaState, slot: i32) -> Option<Val> {
    let (skill_line_index, entry, skill_line) = spellbook_data::get_spell_at_slot(slot)?;
    let spell = spells::get_spell(entry.spell_id);
    let name = spell.map(|spell| spell.name).unwrap_or("Unknown");
    let icon_id = spell.map(|spell| spell.icon_file_data_id).unwrap_or(136243);
    let name_val = create_string(state, name);
    let sub_name_val = create_string(state, "");

    let info = create_table(state);
    table_set(state, info, "actionID", Val::Num(entry.spell_id as f64));
    table_set(state, info, "spellID", Val::Num(entry.spell_id as f64));
    table_set(state, info, "itemType", Val::Num(1.0));
    table_set(state, info, "name", name_val);
    table_set(state, info, "subName", sub_name_val);
    table_set(state, info, "iconID", Val::Num(icon_id as f64));
    table_set(state, info, "isPassive", Val::Bool(entry.is_passive));
    table_set(
        state,
        info,
        "isOffSpec",
        Val::Bool(skill_line.off_spec_id.is_some()),
    );
    table_set(
        state,
        info,
        "skillLineIndex",
        Val::Num(skill_line_index as f64),
    );
    Some(info)
}
