//! Tooltip content population methods.

use super::super::shared::opt_string;
use super::line_frames::{table_array_get, tooltip_line_from_table};
use super::sizing::refresh_tooltip_geometry;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, frame_id_from_stack,
    frame_ref, table_get, val_to_string,
};
use crate::lua_api::script_helpers::{call_void_function_with_fallback_state, get_script};
use crate::lua_api::state::SEEDED_LOCAL_CHARACTER_GUID;
use crate::lua_api::tooltip::TooltipLine;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

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
    let word_wrap_min_width = tooltip_word_wrap_min_width(state, tooltip);
    let lines = tooltip_lines_from_table(state, lines_table);
    let allow_show_with_no_lines = tooltip_allows_showing_without_lines(state, tooltip_id)?;
    let has_lines = !lines.is_empty();

    let mut sim = borrow_state_mut(state)?;
    apply_tooltip_lines(
        &mut sim,
        tooltip_id,
        lines,
        word_wrap_min_width,
        spell_id,
        has_lines || allow_show_with_no_lines,
    );
    Ok(has_lines)
}

fn tooltip_allows_showing_without_lines(state: &LuaState, tooltip_id: u64) -> LuaResult<bool> {
    let sim = borrow_state(state)?;
    Ok(sim
        .tooltips
        .get(&tooltip_id)
        .map(|td| td.allow_show_with_no_lines)
        .unwrap_or(false))
}

fn apply_tooltip_lines(
    sim: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    lines: Vec<TooltipLine>,
    word_wrap_min_width: Option<f32>,
    spell_id: Option<u32>,
    visible: bool,
) {
    let td = sim.tooltips.entry(tooltip_id).or_default();
    td.lines = lines;
    if let Some(word_wrap_min_width) = word_wrap_min_width {
        td.custom_word_wrap_min_width = Some(word_wrap_min_width);
    }
    td.spell_id = spell_id;
    td.unit_token = None;
    td.unit_name = None;
    td.unit_guid = None;
    refresh_tooltip_geometry(sim, tooltip_id);
    sim.set_frame_visible(tooltip_id, visible);
}

fn tooltip_word_wrap_min_width(state: &mut LuaState, tooltip: Val) -> Option<f32> {
    match table_get(state, tooltip, "wordWrapMinWidth") {
        Val::Num(width) => Some(width as f32),
        _ => None,
    }
}

fn tooltip_lines_from_table(state: &mut LuaState, lines_table: Val) -> Vec<TooltipLine> {
    let mut lines = Vec::new();
    let mut index = 1;
    loop {
        let line = table_array_get(state, lines_table, index);
        if !matches!(line, Val::Table(_)) {
            return lines;
        }
        if let Some(parsed) = tooltip_line_from_table(state, line) {
            lines.push(parsed);
        }
        index += 1;
    }
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

fn unit_guid_for_token(state: &LuaState, unit_token: &str) -> Option<String> {
    let sim = borrow_state(state).ok()?;
    match unit_token {
        "player" => Some(SEEDED_LOCAL_CHARACTER_GUID.to_string()),
        "target" => Some(
            sim.current_target
                .as_ref()
                .map(|target| target.guid.clone())
                .unwrap_or_else(|| "Creature-0000-00000000".to_string()),
        ),
        "focus" => Some(
            sim.current_focus
                .as_ref()
                .map(|target| target.guid.clone())
                .unwrap_or_else(|| "Creature-0000-00000000".to_string()),
        ),
        other => crate::lua_api::globals::unit_api::parse_party_index(other).and_then(|idx| {
            (sim.party_group_active && idx < sim.party_members.len())
                .then(|| format!("Player-0000-000000{:02}", idx + 2))
        }),
    }
}

fn set_displayed_unit(state: &mut LuaState, tooltip_id: u64, unit_token: String) -> LuaResult<()> {
    let unit_guid = unit_guid_for_token(state, &unit_token);
    let mut sim = borrow_state_mut(state)?;
    let Some(td) = sim.tooltips.get_mut(&tooltip_id) else {
        return Ok(());
    };
    td.unit_name = td.lines.first().map(|line| line.left_text.clone());
    td.unit_token = Some(unit_token);
    td.unit_guid = unit_guid;
    Ok(())
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

pub(super) fn fire_tooltip_script(state: &mut LuaState, tooltip_id: u64, script_name: &str) {
    let Some(handler) = get_script(state, tooltip_id, script_name) else {
        return;
    };
    let Ok(self_ref) = frame_ref(state, tooltip_id) else {
        return;
    };
    let _ = call_void_function_with_fallback_state(state, handler, &[self_ref]);
}

pub(super) fn fire_tooltip_script_with_args(
    state: &mut LuaState,
    tooltip_id: u64,
    script_name: &str,
    args: &[Val],
) {
    let Some(handler) = get_script(state, tooltip_id, script_name) else {
        return;
    };
    let Ok(self_ref) = frame_ref(state, tooltip_id) else {
        return;
    };
    let mut call_args = Vec::with_capacity(args.len() + 1);
    call_args.push(self_ref);
    call_args.extend_from_slice(args);
    let _ = call_void_function_with_fallback_state(state, handler, &call_args);
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

pub(super) fn set_action(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let slot = stack_val(state, 2);
    let _ = populate_tooltip_from_method(state, tooltip_id, "GetAction", &[slot], None)?;
    Ok(0)
}

pub(super) fn set_currency_token(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let index = stack_val(state, 2);
    let _ = populate_tooltip_from_method(state, tooltip_id, "GetCurrencyToken", &[index], None)?;
    Ok(0)
}

pub(super) fn set_unit(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let unit = stack_val(state, 2);
    let unit_token = val_to_string(state, unit.clone());
    let has_lines = populate_tooltip_from_method(state, tooltip_id, "GetUnit", &[unit], None)?;
    if has_lines {
        if let Some(unit_token) = unit_token {
            set_displayed_unit(state, tooltip_id, unit_token)?;
        }
        fire_tooltip_script(state, tooltip_id, "OnTooltipSetUnit");
    }
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
