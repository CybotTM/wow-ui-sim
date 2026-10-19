//! Wrath template-backed tooltip image attachment.

use super::line_frames::sync_tooltip_line_frame;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, val_to_string};
use crate::lua_api::state::SimState;
use crate::lua_bridge::stack_val;
use crate::widget::{Anchor, AnchorPoint};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const MAX_TEXTURES: usize = 10;
const ROW_GAP: f32 = 2.0;

pub(super) fn attach_texture(state: &mut LuaState, tooltip_id: u64) -> LuaResult<u32> {
    let value = stack_val(state, 2);
    let Some(path) = val_to_string(state, value) else {
        return Ok(0);
    };
    let Some((row, texture_id, adjacent)) = find_texture_slot(state, tooltip_id)? else {
        return Ok(0);
    };
    let Some(left_id) = sync_tooltip_line_frame(state, tooltip_id, false, row + 1)? else {
        return Ok(0);
    };
    let uv = read_texture_coords(state)?;
    let mut sim = borrow_state_mut(state)?;
    assign_slot_texture(&mut sim, texture_id, path, value, uv);
    anchor_attachment(&mut sim, tooltip_id, row, texture_id, left_id, adjacent);
    sim.tooltips
        .entry(tooltip_id)
        .or_default()
        .wrath_texture_rows
        .push((row, texture_id));
    Ok(0)
}

fn find_texture_slot(state: &LuaState, tooltip_id: u64) -> LuaResult<Option<(usize, u64, bool)>> {
    let sim = borrow_state(state)?;
    let Some(td) = sim.tooltips.get(&tooltip_id) else {
        return Ok(None);
    };
    if td.lines.len() < 2 || td.wrath_texture_rows.len() >= MAX_TEXTURES {
        return Ok(None);
    }
    let Some(name) = sim.widgets.get(tooltip_id).and_then(|f| f.name.as_ref()) else {
        return Ok(None);
    };
    let row = td.lines.len() - 1;
    let slot_name = format!("{name}Texture{}", td.wrath_texture_rows.len() + 1);
    let adjacent = td
        .wrath_texture_rows
        .last()
        .is_some_and(|&(previous, _)| previous + 1 == row);
    Ok(sim
        .widgets
        .get_id_by_name(&slot_name)
        .map(|id| (row, id, adjacent)))
}

fn read_texture_coords(state: &mut LuaState) -> LuaResult<(f32, f32, f32, f32)> {
    let mut values = [0.0, 1.0, 0.0, 1.0];
    for (index, component) in values.iter_mut().enumerate() {
        let value = stack_val(state, index as i32 + 3);
        if !matches!(value, Val::Nil) {
            *component = crate::lua_api::wrath_arguments::convert_number(state, value)?
                .unwrap_or(0.0) as f32;
        }
    }
    Ok((values[0], values[1], values[2], values[3]))
}

fn assign_slot_texture(
    sim: &mut SimState,
    id: u64,
    path: String,
    value: Val,
    uv: (f32, f32, f32, f32),
) {
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.texture = (!path.is_empty()).then_some(path);
        frame.texture_file_data_id = match value {
            Val::Num(value) => Some(value as i64),
            _ => None,
        };
        frame.atlas = None;
        frame.color_texture = None;
        frame.atlas_tex_coords = None;
        frame.tex_coords = Some(uv);
        frame.tex_coords_quad = None;
        frame.visible = true;
    }
}

fn anchor_attachment(
    sim: &mut SimState,
    tooltip: u64,
    row: usize,
    texture: u64,
    left: u64,
    adjacent: bool,
) {
    let width = sim.widgets.get(texture).map_or(0.0, |frame| frame.width);
    let gap = crate::lua_api::tooltip::wrath_texture_gap(sim.screen_width, sim.screen_height);
    let previous = sim
        .tooltips
        .get(&tooltip)
        .and_then(|td| td.left_line_ids.get(row - 1))
        .copied();
    if let Some(previous) = previous {
        let offset = if adjacent { 0.0 } else { width + gap };
        anchor_below(sim, left, previous, offset);
    }
    replace_anchor(
        sim,
        texture,
        AnchorPoint::TopRight,
        left,
        AnchorPoint::TopLeft,
        -gap,
        0.0,
    );
    let next = sim
        .widgets
        .get(tooltip)
        .and_then(|frame| frame.name.as_ref())
        .and_then(|name| {
            sim.widgets
                .get_id_by_name(&format!("{name}TextLeft{}", row + 2))
        });
    if let Some(next) = next {
        anchor_below(sim, next, left, -(width + gap));
    }
}

fn anchor_below(sim: &mut SimState, id: u64, previous: u64, x: f32) {
    replace_anchor(
        sim,
        id,
        AnchorPoint::TopLeft,
        previous,
        AnchorPoint::BottomLeft,
        x,
        -ROW_GAP,
    );
}

fn replace_anchor(
    sim: &mut SimState,
    id: u64,
    point: AnchorPoint,
    relative: u64,
    relative_point: AnchorPoint,
    x: f32,
    y: f32,
) {
    sim.widgets.remove_all_anchor_dependents_for(id);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.anchors = vec![Anchor {
            point,
            relative_to: None,
            relative_to_id: Some(relative as usize),
            relative_point,
            x_offset: x,
            y_offset: y,
        }];
    }
    sim.widgets.add_anchor_dependent(relative, id);
    sim.widgets.mark_rect_dirty(id);
}
