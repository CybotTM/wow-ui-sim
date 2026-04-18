//! GameTooltip widget methods.

use super::shared::{opt_f32, opt_string, val_to_bool, val_to_f64};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, frame_id_from_stack,
    frame_ref, get_or_create_frame_fields, table_get, table_set, val_to_string,
};
use crate::lua_api::script_helpers::{call_void_function_with_fallback_state, get_script};
use crate::lua_api::tooltip::TooltipLine;
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub(super) fn clear_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.lines.clear();
        td.spell_id = None;
    }
    drop(sim);
    fire_tooltip_script(state, id, "OnTooltipCleared");
    Ok(0)
}

pub(super) fn add_line(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::tooltip::TooltipLine;
    let id = frame_id_from_stack(state, 1)?;
    let text = opt_string(state, 2).unwrap_or_default();
    let r = val_to_f64(stack_val(state, 3)) as f32;
    let g = val_to_f64(stack_val(state, 4)) as f32;
    let b = val_to_f64(stack_val(state, 5)) as f32;
    let wrap = val_to_bool(stack_val(state, 6));
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.lines.push(TooltipLine {
            left_text: text,
            left_color: (r, g, b),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap,
            texture: None,
        });
    }
    Ok(0)
}

pub(super) fn add_double_line(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: full double-line impl (right_text / right_color parsing)
    add_line(state)
}

pub(super) fn num_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.tooltips.get(&id).map(|td| td.lines.len()).unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

pub(super) fn set_custom_line_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let spacing = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.line_spacing = Some(spacing);
    }
    Ok(0)
}

pub(super) fn get_custom_line_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .tooltips
        .get(&id)
        .and_then(|td| td.line_spacing)
        .map(|s| s as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_minimum_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let width = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.min_width = width;
    }
    Ok(0)
}

pub(super) fn get_minimum_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .tooltips
        .get(&id)
        .map(|td| td.min_width as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_allow_show_with_no_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.allow_show_with_no_lines = value;
    }
    Ok(0)
}

pub(super) fn set_custom_word_wrap_min_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let width = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.custom_word_wrap_min_width = Some(width);
    }
    Ok(0)
}

pub(super) fn set_shrink_to_fit_wrapped(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.shrink_to_fit_wrapped = value;
    }
    Ok(0)
}

pub(super) fn get_spell(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let spell_id = {
        let sim = borrow_state(state)?;
        sim.tooltips.get(&id).and_then(|td| td.spell_id)
    };
    match spell_id {
        Some(id) => {
            let name = crate::spells::get_spell(id)
                .map(|s| s.name.to_string())
                .unwrap_or_else(|| format!("Spell {}", id));
            let name_val = create_string(state, &name);
            name_val.into_stack(state)?;
            (id as f64).into_stack(state)?;
            Ok(2)
        }
        None => {
            state.push(Val::Nil);
            state.push(Val::Nil);
            Ok(2)
        }
    }
}

pub(super) fn get_unit(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

pub(super) fn get_item(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

pub(super) fn set_padding(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let padding = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.padding = padding;
    }
    Ok(0)
}

pub(super) fn get_padding(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .tooltips
        .get(&id)
        .map(|td| td.padding as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn clear_padding(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.padding = 0.0;
    }
    Ok(0)
}

pub(super) fn append_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = opt_string(state, 2).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        if let Some(last) = td.lines.last_mut() {
            last.left_text.push_str(&text);
        }
    }
    Ok(0)
}

fn table_array_get(state: &LuaState, table: Val, index: i64) -> Val {
    let Val::Table(table_ref) = table else {
        return Val::Nil;
    };
    state
        .gc
        .tables
        .get(table_ref)
        .map(|table| table.get_int(index))
        .unwrap_or(Val::Nil)
}

fn line_color_component(state: &mut LuaState, color: Val, key: &str, default: f32) -> f32 {
    match table_get(state, color, key) {
        Val::Num(value) => value as f32,
        _ => default,
    }
}

fn line_color(state: &mut LuaState, line: Val, key: &str) -> (f32, f32, f32) {
    let color = table_get(state, line, key);
    if !matches!(color, Val::Table(_)) {
        return (1.0, 1.0, 1.0);
    }
    (
        line_color_component(state, color, "r", 1.0),
        line_color_component(state, color, "g", 1.0),
        line_color_component(state, color, "b", 1.0),
    )
}

fn tooltip_line_from_table(state: &mut LuaState, line: Val) -> Option<TooltipLine> {
    if !matches!(line, Val::Table(_)) {
        return None;
    }
    let left_text_val = table_get(state, line, "leftText");
    let right_text_val = table_get(state, line, "rightText");
    let wrap_val = table_get(state, line, "wrapText");
    let left_text = val_to_string(state, left_text_val).unwrap_or_default();
    let right_text = val_to_string(state, right_text_val);
    let wrap = matches!(wrap_val, Val::Bool(true));
    Some(TooltipLine {
        left_text,
        left_color: line_color(state, line, "leftColor"),
        right_text,
        right_color: line_color(state, line, "rightColor"),
        wrap,
        texture: None,
    })
}

fn c_tooltip_info_method(state: &mut LuaState, method: &str, args: &[Val]) -> LuaResult<Val> {
    let globals = Val::Table(state.global);
    let namespace = table_get(state, globals, "C_TooltipInfo");
    let func = table_get(state, namespace, method);
    call_function_state(state, func, args)
}

fn apply_tooltip_table(
    state: &mut LuaState,
    tooltip_id: u64,
    tooltip: Val,
    spell_id: Option<u32>,
) -> LuaResult<bool> {
    let lines_table = table_get(state, tooltip, "lines");
    let mut lines = Vec::new();
    let mut index = 1;
    loop {
        let line = table_array_get(state, lines_table, index);
        if !matches!(line, Val::Table(_)) {
            break;
        }
        if let Some(parsed) = tooltip_line_from_table(state, line) {
            lines.push(parsed);
        }
        index += 1;
    }

    let allow_show_with_no_lines = {
        let sim = borrow_state(state)?;
        sim.tooltips
            .get(&tooltip_id)
            .map(|td| td.allow_show_with_no_lines)
            .unwrap_or(false)
    };
    let has_lines = !lines.is_empty();

    let mut sim = borrow_state_mut(state)?;
    let td = sim.tooltips.entry(tooltip_id).or_default();
    td.lines = lines;
    td.spell_id = spell_id;
    sim.set_frame_visible(tooltip_id, has_lines || allow_show_with_no_lines);
    Ok(has_lines)
}

fn populate_tooltip_from_method(
    state: &mut LuaState,
    tooltip_id: u64,
    method: &str,
    args: &[Val],
    spell_id: Option<u32>,
) -> LuaResult<bool> {
    let tooltip = c_tooltip_info_method(state, method, args)?;
    apply_tooltip_table(state, tooltip_id, tooltip, spell_id)
}

fn parse_link_id(text: &str, prefix: &str) -> Option<u32> {
    if let Some(tail) = text.strip_prefix(&format!("{prefix}:")) {
        return tail
            .split(':')
            .next()
            .and_then(|digits| digits.parse::<u32>().ok());
    }
    let needle = format!("|H{prefix}:");
    let start = text.find(&needle)? + needle.len();
    text[start..]
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .ok()
}

fn fire_tooltip_script(state: &mut LuaState, tooltip_id: u64, script_name: &str) {
    let Some(handler) = get_script(state, tooltip_id, script_name) else {
        return;
    };
    let Ok(self_ref) = frame_ref(state, tooltip_id) else {
        return;
    };
    let _ = call_void_function_with_fallback_state(state, handler, &[self_ref]);
}

pub(super) fn set_spell_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let spell_id = stack_val(state, 2);
    let spell_id_num = match spell_id {
        Val::Num(value) if value > 0.0 => Some(value as u32),
        _ => None,
    };
    let has_lines =
        populate_tooltip_from_method(state, tooltip_id, "GetSpellByID", &[spell_id], spell_id_num)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetSpell");
    }
    Ok(0)
}

pub(super) fn set_item_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let item_id = stack_val(state, 2);
    let has_lines =
        populate_tooltip_from_method(state, tooltip_id, "GetItemByID", &[item_id], None)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetItem");
    }
    Ok(0)
}

pub(super) fn set_toy_by_item_id(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let item_id = stack_val(state, 2);
    let has_lines =
        populate_tooltip_from_method(state, tooltip_id, "GetToyByItemID", &[item_id], None)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetItem");
    }
    Ok(0)
}

pub(super) fn set_talent(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [
        stack_val(state, 2),
        stack_val(state, 3),
        stack_val(state, 4),
    ];
    let has_lines = populate_tooltip_from_method(state, tooltip_id, "GetTalent", &args, None)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetSpell");
    }
    Ok(0)
}

pub(super) fn set_mount_by_spell_id(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [stack_val(state, 2), stack_val(state, 3)];
    let spell_id = match args[0] {
        Val::Num(value) if value > 0.0 => Some(value as u32),
        _ => None,
    };
    let has_lines =
        populate_tooltip_from_method(state, tooltip_id, "GetMountBySpellID", &args, spell_id)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetSpell");
    }
    Ok(0)
}

pub(super) fn set_hyperlink(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let Some(link) = opt_string(state, 2) else {
        return Ok(0);
    };
    if let Some(item_id) = parse_link_id(&link, "item") {
        let has_lines = populate_tooltip_from_method(
            state,
            tooltip_id,
            "GetItemByID",
            &[Val::Num(item_id as f64)],
            None,
        )?;
        if has_lines {
            fire_tooltip_script(state, tooltip_id, "OnTooltipSetItem");
        }
        return Ok(0);
    }
    if let Some(spell_id) = parse_link_id(&link, "spell") {
        let has_lines = populate_tooltip_from_method(
            state,
            tooltip_id,
            "GetSpellByID",
            &[Val::Num(spell_id as f64)],
            Some(spell_id),
        )?;
        if has_lines {
            fire_tooltip_script(state, tooltip_id, "OnTooltipSetSpell");
        }
        return Ok(0);
    }
    let link_val = create_string(state, &link);
    let _ = populate_tooltip_from_method(state, tooltip_id, "GetHyperlink", &[link_val], None)?;
    Ok(0)
}

pub(super) fn set_unit(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let unit = stack_val(state, 2);
    let has_lines = populate_tooltip_from_method(state, tooltip_id, "GetUnit", &[unit], None)?;
    state.push(Val::Bool(has_lines));
    Ok(1)
}

pub(super) fn set_unit_buff(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [
        stack_val(state, 2),
        stack_val(state, 3),
        stack_val(state, 4),
    ];
    let _ = populate_tooltip_from_method(state, tooltip_id, "GetUnitBuff", &args, None)?;
    Ok(0)
}

pub(super) fn set_unit_buff_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [
        stack_val(state, 2),
        stack_val(state, 3),
        stack_val(state, 4),
    ];
    let _ = populate_tooltip_from_method(
        state,
        tooltip_id,
        "GetUnitBuffByAuraInstanceID",
        &args,
        None,
    )?;
    Ok(0)
}

pub(super) fn set_unit_debuff(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [
        stack_val(state, 2),
        stack_val(state, 3),
        stack_val(state, 4),
    ];
    let _ = populate_tooltip_from_method(state, tooltip_id, "GetUnitDebuff", &args, None)?;
    Ok(0)
}

pub(super) fn set_unit_debuff_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [
        stack_val(state, 2),
        stack_val(state, 3),
        stack_val(state, 4),
    ];
    let _ = populate_tooltip_from_method(
        state,
        tooltip_id,
        "GetUnitDebuffByAuraInstanceID",
        &args,
        None,
    )?;
    Ok(0)
}

pub(super) fn set_unit_aura(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [
        stack_val(state, 2),
        stack_val(state, 3),
        stack_val(state, 4),
    ];
    let _ = populate_tooltip_from_method(state, tooltip_id, "GetUnitAura", &args, None)?;
    Ok(0)
}

pub(super) fn set_unit_aura_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [stack_val(state, 2), stack_val(state, 3)];
    let _ = populate_tooltip_from_method(
        state,
        tooltip_id,
        "GetUnitAuraByAuraInstanceID",
        &args,
        None,
    )?;
    Ok(0)
}

pub(super) fn set_inventory_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [stack_val(state, 2), stack_val(state, 3)];
    let has_lines =
        populate_tooltip_from_method(state, tooltip_id, "GetInventoryItem", &args, None)?;
    state.push(Val::Bool(has_lines));
    Ok(1)
}

pub(super) fn set_spell_book_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let slot = stack_val(state, 2);
    let book_type = stack_val(state, 3);
    let has_lines = populate_tooltip_from_method(
        state,
        tooltip_id,
        "GetSpellBookItem",
        &[slot, book_type],
        None,
    )?;
    state.push(Val::Bool(has_lines));
    Ok(1)
}

pub(super) fn set_socketed_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let _ = populate_tooltip_from_method(state, tooltip_id, "GetSocketedItem", &[], None)?;
    Ok(0)
}

pub(super) fn set_socket_gem(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let index = stack_val(state, 2);
    let _ = populate_tooltip_from_method(state, tooltip_id, "GetSocketGem", &[index], None)?;
    Ok(0)
}

pub(super) fn set_existing_socket_gem(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let index = stack_val(state, 2);
    let _ =
        populate_tooltip_from_method(state, tooltip_id, "GetExistingSocketGem", &[index], None)?;
    Ok(0)
}

pub(super) fn set_trade_player_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let slot = stack_val(state, 2);
    let has_lines =
        populate_tooltip_from_method(state, tooltip_id, "GetTradePlayerItem", &[slot], None)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetItem");
    }
    Ok(0)
}

pub(super) fn set_trade_target_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let slot = stack_val(state, 2);
    let has_lines =
        populate_tooltip_from_method(state, tooltip_id, "GetTradeTargetItem", &[slot], None)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetItem");
    }
    Ok(0)
}

pub(super) fn set_inbox_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [stack_val(state, 2), stack_val(state, 3)];
    let has_lines = populate_tooltip_from_method(state, tooltip_id, "GetInboxItem", &args, None)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetItem");
    }
    Ok(0)
}

pub(super) fn set_send_mail_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [stack_val(state, 2)];
    let has_lines =
        populate_tooltip_from_method(state, tooltip_id, "GetSendMailItem", &args, None)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetItem");
    }
    Ok(0)
}

pub(super) fn set_trade_skill_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = [stack_val(state, 2), stack_val(state, 3)];
    let has_lines =
        populate_tooltip_from_method(state, tooltip_id, "GetTradeSkillItem", &args, None)?;
    if has_lines {
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetItem");
    }
    Ok(0)
}

/// `Tooltip:SetOwner(frame, anchor, xOffset, yOffset)`
pub(super) fn set_owner(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let owner_id = frame_id_from_stack(state, 2).ok();
    let anchor_kind = opt_string(state, 3).unwrap_or_else(|| "ANCHOR_NONE".into());
    let x_offset = opt_f32(state, 4).unwrap_or(0.0);
    let y_offset = opt_f32(state, 5).unwrap_or(0.0);
    let mut sim = borrow_state_mut(state)?;
    let Some(tooltip) = sim.widgets.get_mut_visual(tooltip_id) else {
        return Ok(0);
    };
    tooltip.tooltip_owner_id = owner_id;
    apply_tooltip_anchor(tooltip, &anchor_kind, owner_id, x_offset, y_offset);
    if let Some(td) = sim.tooltips.get_mut(&tooltip_id) {
        td.owner_id = owner_id;
        td.anchor_type = anchor_kind.clone();
        td.anchor_x_offset = x_offset;
        td.anchor_y_offset = y_offset;
        td.lines.clear();
        td.spell_id = None;
    }
    sim.set_frame_visible(tooltip_id, false);
    drop(sim);
    let fields = get_or_create_frame_fields(state, tooltip_id);
    let anchor_value = create_string(state, &anchor_kind);
    table_set(state, fields, "anchor", anchor_value);
    fire_tooltip_script(state, tooltip_id, "OnTooltipCleared");
    Ok(0)
}

fn apply_tooltip_anchor(
    tooltip: &mut crate::widget::Frame,
    anchor_kind: &str,
    owner_id: Option<u64>,
    x_offset: f32,
    y_offset: f32,
) {
    use crate::widget::AnchorPoint::{
        Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight,
    };
    if anchor_kind == "ANCHOR_PRESERVE" {
        return;
    }
    tooltip.anchors.clear();
    let Some(owner_id) = owner_id else {
        return;
    };
    let points = match anchor_kind {
        "ANCHOR_RIGHT" => Some((Left, Right)),
        "ANCHOR_LEFT" => Some((Right, Left)),
        "ANCHOR_TOP" => Some((Bottom, Top)),
        "ANCHOR_BOTTOM" => Some((Top, Bottom)),
        "ANCHOR_TOPRIGHT" => Some((BottomRight, TopRight)),
        "ANCHOR_TOPLEFT" => Some((BottomLeft, TopLeft)),
        "ANCHOR_BOTTOMRIGHT" => Some((TopRight, BottomRight)),
        "ANCHOR_BOTTOMLEFT" => Some((TopLeft, BottomLeft)),
        _ => None,
    };
    if let Some((point, relative_point)) = points {
        tooltip.anchors.push(crate::widget::Anchor {
            point,
            relative_to: None,
            relative_to_id: Some(owner_id as usize),
            relative_point,
            x_offset,
            y_offset,
        });
    }
}

pub(super) fn get_owner(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let owner_id = {
        let sim = borrow_state(state)?;
        sim.widgets.get(tooltip_id).and_then(|f| f.tooltip_owner_id)
    };
    let val = match owner_id {
        Some(id) => frame_ref(state, id)?,
        None => Val::Nil,
    };
    state.push(val);
    Ok(1)
}

pub(super) fn is_owned(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let candidate_id = frame_id_from_stack(state, 2).ok();
    let matched = {
        let sim = borrow_state(state)?;
        let tooltip_owner = sim.widgets.get(tooltip_id).and_then(|f| f.tooltip_owner_id);
        match (tooltip_owner, candidate_id) {
            (Some(owner), Some(candidate)) => owner == candidate,
            _ => false,
        }
    };
    state.push(Val::Bool(matched));
    Ok(1)
}

pub(super) fn set_anchor_type(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub(super) fn copy_tooltip(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub(super) fn set_frame_stack(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub(super) fn add_font_strings(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// ---------------------------------------------------------------------------
// register_tooltip
// ---------------------------------------------------------------------------

const TOOLTIP_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    // Lines
    ("ClearLines", clear_lines),
    ("AddLine", add_line),
    ("AddDoubleLine", add_double_line),
    ("NumLines", num_lines),
    ("GetNumLines", num_lines),
    // Layout (spacing, width, padding)
    ("SetCustomLineSpacing", set_custom_line_spacing),
    ("GetCustomLineSpacing", get_custom_line_spacing),
    ("SetMinimumWidth", set_minimum_width),
    ("GetMinimumWidth", get_minimum_width),
    ("SetAllowShowWithNoLines", set_allow_show_with_no_lines),
    ("SetCustomWordWrapMinWidth", set_custom_word_wrap_min_width),
    ("SetShrinkToFitWrapped", set_shrink_to_fit_wrapped),
    ("SetPadding", set_padding),
    ("GetPadding", get_padding),
    ("ClearPadding", clear_padding),
    ("AppendText", append_text),
    // Content — spell/unit/item getters + setters
    ("GetSpell", get_spell),
    ("GetUnit", get_unit),
    ("GetItem", get_item),
    ("SetSpellByID", set_spell_by_id),
    ("SetSpellBookItem", set_spell_book_item),
    ("SetItemByID", set_item_by_id),
    ("SetMountBySpellID", set_mount_by_spell_id),
    ("SetTalent", set_talent),
    ("SetToyByItemID", set_toy_by_item_id),
    ("SetHyperlink", set_hyperlink),
    ("SetInventoryItem", set_inventory_item),
    ("SetSocketedItem", set_socketed_item),
    ("SetSocketGem", set_socket_gem),
    ("SetExistingSocketGem", set_existing_socket_gem),
    ("SetTradePlayerItem", set_trade_player_item),
    ("SetTradeTargetItem", set_trade_target_item),
    ("SetInboxItem", set_inbox_item),
    ("SetSendMailItem", set_send_mail_item),
    ("SetTradeSkillItem", set_trade_skill_item),
    ("SetUnit", set_unit),
    ("SetUnitBuff", set_unit_buff),
    (
        "SetUnitBuffByAuraInstanceID",
        set_unit_buff_by_aura_instance_id,
    ),
    ("SetUnitDebuff", set_unit_debuff),
    (
        "SetUnitDebuffByAuraInstanceID",
        set_unit_debuff_by_aura_instance_id,
    ),
    ("SetUnitAura", set_unit_aura),
    (
        "SetUnitAuraByAuraInstanceID",
        set_unit_aura_by_aura_instance_id,
    ),
    // Ownership + anchoring
    ("SetOwner", set_owner),
    ("GetOwner", get_owner),
    ("IsOwned", is_owned),
    ("SetAnchorType", set_anchor_type),
    // Misc
    ("CopyTooltip", copy_tooltip),
    ("SetFrameStack", set_frame_stack),
    ("AddFontStrings", add_font_strings),
];

pub(super) fn register_tooltip(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    for (name, func) in TOOLTIP_METHODS {
        table_set_rust_fn(state, metatable, name, *func)?;
    }
    Ok(())
}
