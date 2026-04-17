//! GameTooltip widget methods.

use super::shared::{opt_bool, opt_f32, opt_string, val_to_bool, val_to_f64};
use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack, frame_ref,
};
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
    // TODO: fire OnTooltipCleared script
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

pub(super) fn set_spell_by_id(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: populate_spell_tooltip + fire OnTooltipSetSpell
    Ok(0)
}

pub(super) fn set_item_by_id(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: populate_item_tooltip + fire OnTooltipSetItem
    Ok(0)
}

pub(super) fn set_hyperlink(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: parse item/spell hyperlink and populate
    Ok(0)
}

pub(super) fn set_unit(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: set_unit_for_tooltip
    Ok(0)
}

pub(super) fn set_unit_buff(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: lookup_aura + populate_aura_tooltip
    Ok(0)
}

pub(super) fn set_unit_debuff(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub(super) fn set_unit_aura(_state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn register_tooltip(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, metatable, "ClearLines", clear_lines)?;
    table_set_rust_fn(state, metatable, "AddLine", add_line)?;
    table_set_rust_fn(state, metatable, "AddDoubleLine", add_double_line)?;
    table_set_rust_fn(state, metatable, "NumLines", num_lines)?;
    table_set_rust_fn(state, metatable, "GetNumLines", num_lines)?;
    table_set_rust_fn(
        state,
        metatable,
        "SetCustomLineSpacing",
        set_custom_line_spacing,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "GetCustomLineSpacing",
        get_custom_line_spacing,
    )?;
    table_set_rust_fn(state, metatable, "SetMinimumWidth", set_minimum_width)?;
    table_set_rust_fn(state, metatable, "GetMinimumWidth", get_minimum_width)?;
    table_set_rust_fn(
        state,
        metatable,
        "SetAllowShowWithNoLines",
        set_allow_show_with_no_lines,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "SetCustomWordWrapMinWidth",
        set_custom_word_wrap_min_width,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "SetShrinkToFitWrapped",
        set_shrink_to_fit_wrapped,
    )?;
    table_set_rust_fn(state, metatable, "GetSpell", get_spell)?;
    table_set_rust_fn(state, metatable, "GetUnit", get_unit)?;
    table_set_rust_fn(state, metatable, "GetItem", get_item)?;
    table_set_rust_fn(state, metatable, "SetPadding", set_padding)?;
    table_set_rust_fn(state, metatable, "GetPadding", get_padding)?;
    table_set_rust_fn(state, metatable, "ClearPadding", clear_padding)?;
    table_set_rust_fn(state, metatable, "AppendText", append_text)?;
    table_set_rust_fn(state, metatable, "SetSpellByID", set_spell_by_id)?;
    table_set_rust_fn(state, metatable, "SetItemByID", set_item_by_id)?;
    table_set_rust_fn(state, metatable, "SetHyperlink", set_hyperlink)?;
    table_set_rust_fn(state, metatable, "SetUnit", set_unit)?;
    table_set_rust_fn(state, metatable, "SetUnitBuff", set_unit_buff)?;
    table_set_rust_fn(state, metatable, "SetUnitDebuff", set_unit_debuff)?;
    table_set_rust_fn(state, metatable, "SetUnitAura", set_unit_aura)?;
    table_set_rust_fn(state, metatable, "SetOwner", set_owner)?;
    table_set_rust_fn(state, metatable, "GetOwner", get_owner)?;
    table_set_rust_fn(state, metatable, "IsOwned", is_owned)?;
    table_set_rust_fn(state, metatable, "SetAnchorType", set_anchor_type)?;
    table_set_rust_fn(state, metatable, "CopyTooltip", copy_tooltip)?;
    table_set_rust_fn(state, metatable, "SetFrameStack", set_frame_stack)?;
    table_set_rust_fn(state, metatable, "AddFontStrings", add_font_strings)?;
    Ok(())
}
