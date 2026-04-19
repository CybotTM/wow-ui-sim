//! `C_SpellBook` surface backed by spellbook data and `SimState`.

use crate::c_api::ensure_namespace;
use crate::lua_api::globals::spellbook_data;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, frame_ref, table_set,
    table_set_num,
};
use crate::lua_api::script_helpers::{
    call_error_handler_state, get_event_listeners, get_script, protected_lua_pcall_state,
};
use crate::lua_api::state_types::CursorInfo;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use crate::spells;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_spell_book(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_SpellBook")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetNumSpellBookSkillLines",
        c_spell_book_get_num_spell_book_skill_lines,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSpellBookSkillLineInfo",
        c_spell_book_get_spell_book_skill_line_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSpellBookItemName",
        c_spell_book_get_spell_book_item_name,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSpellBookItemInfo",
        c_spell_book_get_spell_book_item_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSpellBookItemType",
        c_spell_book_get_spell_book_item_type,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSpellBookItemCooldown",
        c_spell_book_get_spell_book_item_cooldown,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSpellBookItemTexture",
        c_spell_book_get_spell_book_item_texture,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSpellBookItemAutoCast",
        c_spell_book_get_spell_book_item_auto_cast,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSpellBookItemPowerCost",
        c_spell_book_get_spell_book_item_power_cost,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "FindSpellBookSlotForSpell",
        c_spell_book_find_spell_book_slot_for_spell,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "CastSpellBookItem",
        c_spell_book_cast_spell_book_item,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsSpellInSpellBook",
        c_spell_book_is_spell_in_spell_book,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSpellBookItemLossOfControlCooldownInfo",
        c_spell_book_get_spell_book_item_loss_of_control_cooldown_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetOverrideSpell",
        c_spell_book_get_override_spell,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "FindSpellOverrideByID",
        c_spell_book_get_override_spell,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsSpellKnown",
        c_spell_book_is_spell_known,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "PickupSpellBookItem",
        c_spell_book_pickup_spell_book_item,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "HasPetSpells",
        c_spell_book_has_pet_spells,
    )?;
    Ok(())
}

fn spell_power_min_cost(player_power_max: f32, cost: &crate::spell_power::SpellPowerCost) -> i32 {
    if cost.mana_cost > 0 {
        return cost.mana_cost;
    }
    if cost.cost_pct > 0.0 {
        return (player_power_max * (cost.cost_pct / 100.0)).round() as i32;
    }
    0
}

fn build_spell_power_cost_info(
    state: &mut LuaState,
    cost: &crate::spell_power::SpellPowerCost,
    player_power_max: f32,
) -> Option<Val> {
    let Val::Table(info) = create_table(state) else {
        return None;
    };

    let min_cost = spell_power_min_cost(player_power_max, cost);
    let total_cost = min_cost + cost.optional_cost.max(0);
    let power_name = create_string(state, crate::spell_power::power_type_name(cost.power_type));

    table_set(
        state,
        Val::Table(info),
        "type",
        Val::Num(cost.power_type as f64),
    );
    table_set(state, Val::Table(info), "name", power_name);
    table_set(state, Val::Table(info), "cost", Val::Num(total_cost as f64));
    table_set(
        state,
        Val::Table(info),
        "minCost",
        Val::Num(min_cost as f64),
    );
    table_set(
        state,
        Val::Table(info),
        "costPercent",
        Val::Num(cost.cost_pct as f64),
    );
    table_set(
        state,
        Val::Table(info),
        "costPerSec",
        Val::Num(cost.cost_per_sec as f64),
    );
    table_set(
        state,
        Val::Table(info),
        "requiredAuraID",
        Val::Num(cost.required_aura_id as f64),
    );
    table_set(
        state,
        Val::Table(info),
        "hasRequiredAura",
        Val::Bool(cost.required_aura_id == 0),
    );

    Some(Val::Table(info))
}

fn spell_power_costs_table(state: &mut LuaState, spell_id: u32) -> Option<Val> {
    let costs = crate::spell_power::get_spell_power(spell_id)?;
    if costs.is_empty() {
        return None;
    }

    let player_power_max = borrow_state(state).ok()?.player.power_max.max(0) as f32;
    let Val::Table(power_costs) = create_table(state) else {
        return None;
    };

    for (index, cost) in costs.iter().enumerate() {
        let Some(info) = build_spell_power_cost_info(state, cost, player_power_max) else {
            continue;
        };
        table_set_num(state, power_costs, (index + 1) as f64, info);
    }

    Some(Val::Table(power_costs))
}

fn fire_cursor_changed(state: &mut LuaState) {
    for widget_id in get_event_listeners(state, "CURSOR_CHANGED") {
        let Some(handler) = get_script(state, widget_id, "OnEvent") else {
            continue;
        };
        if !matches!(handler, Val::Function(_)) {
            continue;
        }
        let Ok(frame) = frame_ref(state, widget_id) else {
            continue;
        };
        let event_name = create_string(state, "CURSOR_CHANGED");
        if let Err(error) = protected_lua_pcall_state(state, handler, &[frame, event_name]) {
            call_error_handler_state(state, &error);
        }
    }
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

fn c_spell_book_get_spell_book_item_texture(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
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

fn c_spell_book_get_spell_book_item_power_cost(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    let Some((_, entry, _)) = spellbook_data::get_spell_at_slot(slot) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    match spell_power_costs_table(state, entry.spell_id) {
        Some(power_costs) => state.push(power_costs),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_spell_book_get_spell_book_item_loss_of_control_cooldown_info(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    if spellbook_data::get_spell_at_slot(slot).is_none() {
        state.push(Val::Nil);
        return Ok(1);
    }
    let info = create_table(state);
    table_set(state, info, "isActive", Val::Bool(false));
    table_set(state, info, "startTime", Val::Num(0.0));
    table_set(state, info, "duration", Val::Num(0.0));
    table_set(state, info, "modRate", Val::Num(1.0));
    table_set(state, info, "shouldReplaceNormalCooldown", Val::Bool(false));
    state.push(info);
    Ok(1)
}

fn c_spell_book_get_override_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Num(spell_id as f64));
    Ok(1)
}

fn c_spell_book_is_spell_known(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let known = spellbook_data::is_spell_known(spell_id);
    state.push(Val::Bool(known));
    Ok(1)
}

fn c_spell_book_pickup_spell_book_item(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    let Some((_, entry, _)) = spellbook_data::get_spell_at_slot(slot) else {
        return Ok(0);
    };
    borrow_state_mut(state)?.cursor_item = Some(CursorInfo::Spell {
        spell_id: entry.spell_id,
    });
    fire_cursor_changed(state);
    Ok(0)
}

fn c_spell_book_find_spell_book_slot_for_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    match spellbook_data::find_spell_slot(spell_id) {
        Some((slot, _spell_bank)) => state.push(Val::Num(slot as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_spell_book_cast_spell_book_item(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    let Some((_, entry, _)) = spellbook_data::get_spell_at_slot(slot) else {
        return Ok(0);
    };
    crate::lua_api::globals::combat_verbs::execute_spell_by_id(state, entry.spell_id)?;
    Ok(0)
}

fn c_spell_book_is_spell_in_spell_book(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let known = spellbook_data::is_spell_known(spell_id);
    state.push(Val::Bool(known));
    Ok(1)
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
