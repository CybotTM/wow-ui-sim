//! GameTooltip widget methods.

use super::shared::{opt_bool, opt_f32, opt_string, val_to_bool, val_to_f64};
use crate::lua_api::globals::create_frame::create_frame_instance;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, frame_id_from_stack,
    frame_ref, get_or_create_frame_fields, table_get, table_set, val_to_string,
};
use crate::lua_api::script_helpers::{
    call_void_function_with_fallback_state, collect_lua_error, get_script,
};
use crate::lua_api::tooltip::{
    TooltipLine, TooltipTextSegment, TooltipTexture, build_cursor_anchor,
};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn};
use crate::widget::{TextJustify, WidgetType};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub(super) fn clear_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    let td = sim.tooltips.entry(id).or_default();
    td.lines.clear();
    td.spell_id = None;
    td.unit_token = None;
    td.unit_name = None;
    td.unit_guid = None;
    drop(sim);
    fire_tooltip_script(state, id, "OnTooltipCleared");
    Ok(0)
}

pub(super) fn add_line(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::tooltip::TooltipLine;
    let id = frame_id_from_stack(state, 1)?;
    let text = opt_string(state, 2).unwrap_or_default();
    let (default_r, default_g, default_b) = normal_font_color(state);
    let r = opt_f32(state, 3).unwrap_or(default_r);
    let g = opt_f32(state, 4).unwrap_or(default_g);
    let b = opt_f32(state, 5).unwrap_or(default_b);
    let wrap = val_to_bool(stack_val(state, 6));
    let line_index = next_tooltip_line_index(state, id);
    let left_segments = processing_line_color_segments(
        state,
        id,
        line_index,
        "leftText",
        &text,
        "leftColorSegments",
    );
    let mut sim = borrow_state_mut(state)?;
    let td = sim.tooltips.entry(id).or_default();
    td.lines.push(TooltipLine {
        left_text: text,
        left_color: (r, g, b),
        left_segments,
        right_text: None,
        right_color: (1.0, 1.0, 1.0),
        right_segments: Vec::new(),
        wrap,
        texture: None,
    });
    Ok(0)
}

pub(super) fn add_double_line(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let args = double_line_args(state);
    let line_index = next_tooltip_line_index(state, id);
    let left_segments = processing_line_color_segments(
        state,
        id,
        line_index,
        "leftText",
        &args.left_text,
        "leftColorSegments",
    );
    let right_segments = processing_right_line_color_segments(state, id, line_index, &args);
    push_double_tooltip_line(state, id, args, left_segments, right_segments)?;

    Ok(0)
}

struct DoubleLineArgs {
    left_text: String,
    right_text: Option<String>,
    left_color: (f32, f32, f32),
    right_color: (f32, f32, f32),
    wrap: bool,
}

fn double_line_args(state: &mut LuaState) -> DoubleLineArgs {
    let left_text = opt_string(state, 2).unwrap_or_default();
    let right_text = opt_string(state, 3);
    let (default_r, default_g, default_b) = normal_font_color(state);
    let left_color = (
        opt_f32(state, 4).unwrap_or(default_r),
        opt_f32(state, 5).unwrap_or(default_g),
        opt_f32(state, 6).unwrap_or(default_b),
    );
    let right_color = (
        opt_f32(state, 7).unwrap_or(left_color.0),
        opt_f32(state, 8).unwrap_or(left_color.1),
        opt_f32(state, 9).unwrap_or(left_color.2),
    );
    let wrap = val_to_bool(stack_val(state, 10));
    DoubleLineArgs {
        left_text,
        right_text,
        left_color,
        right_color,
        wrap,
    }
}

fn processing_right_line_color_segments(
    state: &mut LuaState,
    tooltip_id: u64,
    line_index: usize,
    args: &DoubleLineArgs,
) -> Vec<TooltipTextSegment> {
    args.right_text
        .as_deref()
        .map(|text| {
            processing_line_color_segments(
                state,
                tooltip_id,
                line_index,
                "rightText",
                text,
                "rightColorSegments",
            )
        })
        .unwrap_or_default()
}

fn push_double_tooltip_line(
    state: &mut LuaState,
    tooltip_id: u64,
    args: DoubleLineArgs,
    left_segments: Vec<TooltipTextSegment>,
    right_segments: Vec<TooltipTextSegment>,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    let td = sim.tooltips.entry(tooltip_id).or_default();
    td.lines.push(TooltipLine {
        left_text: args.left_text,
        left_color: args.left_color,
        left_segments,
        right_text: args.right_text,
        right_color: args.right_color,
        right_segments,
        wrap: args.wrap,
        texture: None,
    });
    Ok(())
}

pub(super) fn num_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let is_tooltip = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .is_some_and(|frame| frame.widget_type == WidgetType::GameTooltip)
    };
    if !is_tooltip {
        return crate::lua_api::frame::methods::text_attribute_event::get_text_num_lines(state);
    }
    let line_count = {
        let sim = borrow_state(state)?;
        sim.tooltips.get(&id).map(|td| td.lines.len()).unwrap_or(0)
    };
    (line_count as f64).into_stack(state)
}

fn next_tooltip_line_index(state: &mut LuaState, tooltip_id: u64) -> usize {
    borrow_state(state)
        .ok()
        .and_then(|sim| sim.tooltips.get(&tooltip_id).map(|td| td.lines.len() + 1))
        .unwrap_or(1)
}

fn processing_line_color_segments(
    state: &mut LuaState,
    tooltip_id: u64,
    line_index: usize,
    text_key: &str,
    text: &str,
    segment_key: &str,
) -> Vec<TooltipTextSegment> {
    let Some(line) = processing_tooltip_line(state, tooltip_id, line_index, text_key, text) else {
        return Vec::new();
    };
    line_color_segments(state, line, segment_key)
}

fn processing_tooltip_line(
    state: &mut LuaState,
    tooltip_id: u64,
    line_index: usize,
    text_key: &str,
    text: &str,
) -> Option<Val> {
    let tooltip = frame_ref(state, tooltip_id).ok()?;
    let processing_info = table_get(state, tooltip, "processingInfo");
    let tooltip_data = table_get(state, processing_info, "tooltipData");
    let lines = table_get(state, tooltip_data, "lines");
    processing_tooltip_line_at(state, lines, line_index, text_key, text)
        .or_else(|| find_processing_tooltip_line(state, lines, text_key, text))
}

fn processing_tooltip_line_at(
    state: &mut LuaState,
    lines: Val,
    line_index: usize,
    text_key: &str,
    text: &str,
) -> Option<Val> {
    let line = table_array_get(state, lines, line_index as i64);
    processing_line_text_matches(state, line, text_key, text).then_some(line)
}

fn find_processing_tooltip_line(
    state: &mut LuaState,
    lines: Val,
    text_key: &str,
    text: &str,
) -> Option<Val> {
    let mut index = 1;
    loop {
        let line = table_array_get(state, lines, index);
        if !matches!(line, Val::Table(_)) {
            return None;
        }
        if processing_line_text_matches(state, line, text_key, text) {
            return Some(line);
        }
        index += 1;
    }
}

fn processing_line_text_matches(
    state: &mut LuaState,
    line: Val,
    text_key: &str,
    text: &str,
) -> bool {
    let line_text = table_get(state, line, text_key);
    val_to_string(state, line_text).as_deref() == Some(text)
}

pub(super) fn set_custom_line_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let spacing = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    sim.tooltips.entry(id).or_default().line_spacing = Some(spacing);
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
    sim.tooltips.entry(id).or_default().min_width = width;
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
    sim.tooltips.entry(id).or_default().allow_show_with_no_lines = value;
    Ok(0)
}

pub(super) fn set_custom_word_wrap_min_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let width = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    sim.tooltips
        .entry(id)
        .or_default()
        .custom_word_wrap_min_width = Some(width);
    Ok(0)
}

pub(super) fn set_shrink_to_fit_wrapped(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    sim.tooltips.entry(id).or_default().shrink_to_fit_wrapped = value;
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
    let id = frame_id_from_stack(state, 1)?;
    let unit = {
        let sim = borrow_state(state)?;
        sim.tooltips.get(&id).and_then(|td| {
            Some((
                td.unit_name.clone()?,
                td.unit_token.clone()?,
                td.unit_guid.clone()?,
            ))
        })
    };
    match unit {
        Some((name, token, guid)) => {
            let name = create_string(state, &name);
            let token = create_string(state, &token);
            let guid = create_string(state, &guid);
            state.push(name);
            state.push(token);
            state.push(guid);
        }
        None => {
            state.push(Val::Nil);
            state.push(Val::Nil);
            state.push(Val::Nil);
        }
    }
    Ok(3)
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
    sim.tooltips.entry(id).or_default().padding = padding;
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
    if let Some(last) = sim.tooltips.entry(id).or_default().lines.last_mut() {
        last.left_text.push_str(&text);
    }
    Ok(0)
}

fn tooltip_line_exists(state: &LuaState, tooltip_id: u64, line_index: usize) -> bool {
    if line_index == 0 {
        return false;
    }
    let Ok(sim) = borrow_state(state) else {
        return false;
    };
    let line_count = sim.tooltips.get(&tooltip_id).map(|td| td.lines.len());
    line_count.is_some_and(|count| line_index <= count)
}

fn tooltip_line_name(
    state: &LuaState,
    tooltip_id: u64,
    right_side: bool,
    line_index: usize,
) -> Option<String> {
    let sim = borrow_state(state).ok()?;
    let tooltip_name = sim.widgets.get(tooltip_id)?.name.as_ref()?;
    let suffix = if right_side { "Right" } else { "Left" };
    Some(format!("{tooltip_name}Text{suffix}{line_index}"))
}

fn find_existing_tooltip_line_id(
    state: &LuaState,
    tooltip_id: u64,
    right_side: bool,
    line_index: usize,
) -> Option<u64> {
    let sim = borrow_state(state).ok()?;
    let td = sim.tooltips.get(&tooltip_id)?;
    let existing = if right_side {
        td.right_line_ids.get(line_index - 1).copied()
    } else {
        td.left_line_ids.get(line_index - 1).copied()
    };
    if let Some(id) = existing
        && sim.widgets.get(id).is_some_and(|frame| {
            frame.parent_id == Some(tooltip_id) && frame.widget_type == WidgetType::FontString
        })
    {
        return Some(id);
    }

    let name = tooltip_line_name(state, tooltip_id, right_side, line_index)?;
    let child_id = sim.widgets.get_id_by_name(&name)?;
    sim.widgets.get(child_id).and_then(|frame| {
        (frame.parent_id == Some(tooltip_id) && frame.widget_type == WidgetType::FontString)
            .then_some(child_id)
    })
}

fn ensure_tooltip_line_id(
    state: &mut LuaState,
    tooltip_id: u64,
    right_side: bool,
    line_index: usize,
) -> LuaResult<Option<u64>> {
    if !tooltip_line_exists(state, tooltip_id, line_index) {
        return Ok(None);
    }
    if let Some(existing_id) =
        find_existing_tooltip_line_id(state, tooltip_id, right_side, line_index)
    {
        return Ok(Some(existing_id));
    }

    let name = tooltip_line_name(state, tooltip_id, right_side, line_index);
    let child_id = create_frame_instance(
        state,
        WidgetType::FontString,
        "FontString",
        name,
        Some(tooltip_id),
        true,
        None,
    )?;
    {
        let mut sim = borrow_state_mut(state)?;
        configure_tooltip_line_child(&mut sim, child_id, right_side, line_index);
        record_tooltip_line_child_id(&mut sim, tooltip_id, child_id, right_side, line_index);
    }
    Ok(Some(child_id))
}

fn configure_tooltip_line_child(
    sim: &mut crate::lua_api::state::SimState,
    child_id: u64,
    right_side: bool,
    line_index: usize,
) {
    let Some(child) = sim.widgets.get_mut_visual(child_id) else {
        return;
    };
    child.parent_key = Some(tooltip_line_parent_key(right_side, line_index));
    child.justify_h = tooltip_line_justify(right_side);
}

fn tooltip_line_parent_key(right_side: bool, line_index: usize) -> String {
    if right_side {
        format!("TextRight{line_index}")
    } else {
        format!("TextLeft{line_index}")
    }
}

fn tooltip_line_justify(right_side: bool) -> TextJustify {
    if right_side {
        TextJustify::Right
    } else {
        TextJustify::Left
    }
}

fn record_tooltip_line_child_id(
    sim: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    child_id: u64,
    right_side: bool,
    line_index: usize,
) {
    let td = sim.tooltips.entry(tooltip_id).or_default();
    let ids = if right_side {
        &mut td.right_line_ids
    } else {
        &mut td.left_line_ids
    };
    if ids.len() < line_index {
        ids.resize(line_index, 0);
    }
    ids[line_index - 1] = child_id;
}

fn sync_tooltip_line_frame(
    state: &mut LuaState,
    tooltip_id: u64,
    right_side: bool,
    line_index: usize,
) -> LuaResult<Option<u64>> {
    let Some(line_id) = ensure_tooltip_line_id(state, tooltip_id, right_side, line_index)? else {
        return Ok(None);
    };
    let line = {
        let sim = borrow_state(state)?;
        sim.tooltips
            .get(&tooltip_id)
            .and_then(|td| td.lines.get(line_index - 1).cloned())
    };
    let Some(line) = line else {
        return Ok(None);
    };
    let (text, color, text_segments) = tooltip_line_frame_content(line, right_side);
    let stripped = text
        .as_ref()
        .map(|value| crate::render::strip_wow_markup(value));
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(line_id) {
        frame.text = text;
        frame.text_stripped = stripped;
        frame.text_color = color;
        frame.text_segments = text_segments;
    }
    Ok(Some(line_id))
}

fn tooltip_line_frame_content(
    line: TooltipLine,
    right_side: bool,
) -> (
    Option<String>,
    crate::widget::Color,
    Vec<crate::widget::TextSegment>,
) {
    if right_side {
        (
            line.right_text,
            tooltip_line_frame_color(line.right_color),
            tooltip_line_frame_segments(line.right_segments),
        )
    } else {
        (
            Some(line.left_text),
            tooltip_line_frame_color(line.left_color),
            tooltip_line_frame_segments(line.left_segments),
        )
    }
}

fn tooltip_line_frame_color(color: (f32, f32, f32)) -> crate::widget::Color {
    let (r, g, b) = color;
    crate::widget::Color::new(r, g, b, 1.0)
}

fn tooltip_line_frame_segments(
    segments: Vec<TooltipTextSegment>,
) -> Vec<crate::widget::TextSegment> {
    segments
        .into_iter()
        .map(|segment| crate::widget::TextSegment {
            text: segment.text,
            color: crate::widget::Color::new(
                segment.color.0,
                segment.color.1,
                segment.color.2,
                1.0,
            ),
        })
        .collect()
}

fn push_tooltip_line_ref(
    state: &mut LuaState,
    tooltip_id: u64,
    right_side: bool,
    line_index: usize,
) -> LuaResult<u32> {
    if line_index == 0 {
        state.push(Val::Nil);
        return Ok(1);
    }
    let line_id = sync_tooltip_line_frame(state, tooltip_id, right_side, line_index)?;
    let Some(line_id) = line_id else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let line_ref = frame_ref(state, line_id)?;
    state.push(line_ref);
    Ok(1)
}

fn add_texture_line(
    state: &mut LuaState,
    tooltip_id: u64,
    texture: TooltipTexture,
) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    sim.tooltips
        .entry(tooltip_id)
        .or_default()
        .lines
        .push(TooltipLine {
            left_text: String::new(),
            left_color: (1.0, 1.0, 1.0),
            left_segments: Vec::new(),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            right_segments: Vec::new(),
            wrap: false,
            texture: Some(texture),
        });
    Ok(0)
}

pub(super) fn add_texture(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let texture = match stack_val(state, 2) {
        Val::Num(value) if value >= 0.0 => Some(TooltipTexture::FileDataId(value as u32)),
        value => val_to_string(state, value)
            .and_then(|text| text.parse::<u32>().ok())
            .map(TooltipTexture::FileDataId),
    };
    let Some(texture) = texture else {
        return Ok(0);
    };
    add_texture_line(state, tooltip_id, texture)
}

pub(super) fn add_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let Some(atlas) = opt_string(state, 2) else {
        return Ok(0);
    };
    add_texture_line(state, tooltip_id, TooltipTexture::Atlas(atlas))
}

pub(super) fn get_left_line(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let line_index = opt_f32(state, 2).unwrap_or(0.0).max(0.0) as usize;
    push_tooltip_line_ref(state, tooltip_id, false, line_index)
}

pub(super) fn get_right_line(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let line_index = opt_f32(state, 2).unwrap_or(0.0).max(0.0) as usize;
    push_tooltip_line_ref(state, tooltip_id, true, line_index)
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

fn normal_font_color(state: &mut LuaState) -> (f32, f32, f32) {
    let globals = Val::Table(state.global);
    let normal = table_get(state, globals, "NORMAL_FONT_COLOR");
    if !matches!(normal, Val::Table(_)) {
        return (1.0, 0.82, 0.0);
    }
    (
        line_color_component(state, normal, "r", 1.0),
        line_color_component(state, normal, "g", 0.82),
        line_color_component(state, normal, "b", 0.0),
    )
}

fn line_color(state: &mut LuaState, line: Val, key: &str) -> (f32, f32, f32) {
    let color = table_get(state, line, key);
    if !matches!(color, Val::Table(_)) {
        return normal_font_color(state);
    }
    (
        line_color_component(state, color, "r", 1.0),
        line_color_component(state, color, "g", 1.0),
        line_color_component(state, color, "b", 1.0),
    )
}

fn line_color_segments(state: &mut LuaState, line: Val, key: &str) -> Vec<TooltipTextSegment> {
    let segments_table = table_get(state, line, key);
    let mut segments = Vec::new();
    let mut index = 1;

    loop {
        let segment = table_array_get(state, segments_table, index);
        if !matches!(segment, Val::Table(_)) {
            break;
        }
        let text_val = table_get(state, segment, "text");
        let text = val_to_string(state, text_val).unwrap_or_default();
        let color = line_color(state, segment, "color");
        segments.push(TooltipTextSegment { text, color });
        index += 1;
    }

    segments
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
        left_segments: line_color_segments(state, line, "leftColorSegments"),
        right_text,
        right_color: line_color(state, line, "rightColor"),
        right_segments: line_color_segments(state, line, "rightColorSegments"),
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
        "player" => Some("Player-0000-00000001".to_string()),
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

fn fire_tooltip_script(state: &mut LuaState, tooltip_id: u64, script_name: &str) {
    let Some(handler) = get_script(state, tooltip_id, script_name) else {
        return;
    };
    let Ok(self_ref) = frame_ref(state, tooltip_id) else {
        return;
    };
    let _ = call_void_function_with_fallback_state(state, handler, &[self_ref]);
}

fn fire_tooltip_script_with_args(
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

/// `Tooltip:SetOwner(frame, anchor, xOffset, yOffset)`
pub(super) fn set_owner(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let args = tooltip_owner_args(state);
    let mut sim = borrow_state_mut(state)?;
    if !apply_tooltip_owner(&mut sim, tooltip_id, &args) {
        return Ok(0);
    }
    drop(sim);
    let fields = get_or_create_frame_fields(state, tooltip_id);
    let anchor_value = create_string(state, &args.anchor_kind);
    table_set(state, fields, "anchor", anchor_value);
    fire_tooltip_script(state, tooltip_id, "OnTooltipCleared");
    Ok(0)
}

struct TooltipOwnerArgs {
    owner_id: Option<u64>,
    anchor_kind: String,
    x_offset: f32,
    y_offset: f32,
}

fn tooltip_owner_args(state: &mut LuaState) -> TooltipOwnerArgs {
    TooltipOwnerArgs {
        owner_id: frame_id_from_stack(state, 2).ok(),
        anchor_kind: tooltip_anchor_kind_arg(state),
        x_offset: opt_f32(state, 4).unwrap_or(0.0),
        y_offset: opt_f32(state, 5).unwrap_or(0.0),
    }
}

fn tooltip_anchor_kind_arg(state: &mut LuaState) -> String {
    let anchor_kind = opt_string(state, 3).unwrap_or_else(|| "ANCHOR_NONE".to_string());
    if is_valid_tooltip_anchor(&anchor_kind) {
        return anchor_kind;
    }

    let _ = collect_lua_error(
        state,
        &format!("invalid anchor type: {anchor_kind}; defaulting to ANCHOR_LEFT"),
    );
    "ANCHOR_LEFT".to_string()
}

fn apply_tooltip_owner(
    sim: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    args: &TooltipOwnerArgs,
) -> bool {
    let mouse_position = sim.mouse_position;
    let Some(tooltip) = sim.widgets.get_mut_visual(tooltip_id) else {
        return false;
    };
    tooltip.tooltip_owner_id = args.owner_id;
    apply_tooltip_anchor(
        tooltip,
        &args.anchor_kind,
        args.owner_id,
        mouse_position,
        args.x_offset,
        args.y_offset,
    );
    record_tooltip_owner(sim, tooltip_id, args);
    true
}

fn record_tooltip_owner(
    sim: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    args: &TooltipOwnerArgs,
) {
    let td = sim.tooltips.entry(tooltip_id).or_default();
    td.owner_id = args.owner_id;
    td.anchor_type = args.anchor_kind.clone();
    td.anchor_x_offset = args.x_offset;
    td.anchor_y_offset = args.y_offset;
    td.lines.clear();
    td.spell_id = None;
    sim.widgets.mark_rect_dirty(tooltip_id);
    // Tooltip owners commonly reapply SetOwner during periodic refreshes.
    // Keep the tooltip shown so identical refreshes don't churn show/hide state.
    sim.set_frame_visible(tooltip_id, true);
}

pub(super) fn set_object_tooltip_position(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Anchor, AnchorPoint};

    let tooltip_id = frame_id_from_stack(state, 1)?;
    let owner_id = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(tooltip_id)
            .and_then(|tooltip| tooltip.tooltip_owner_id)
            .or_else(|| sim.tooltips.get(&tooltip_id).and_then(|td| td.owner_id))
    };
    let Some(owner_id) = owner_id else {
        return Ok(0);
    };

    let mut sim = borrow_state_mut(state)?;
    let Some(tooltip) = sim.widgets.get_mut_visual(tooltip_id) else {
        return Ok(0);
    };
    tooltip.anchors.clear();
    tooltip.anchors.push(Anchor {
        point: AnchorPoint::Bottom,
        relative_to: None,
        relative_to_id: Some(owner_id as usize),
        relative_point: AnchorPoint::Top,
        x_offset: 0.0,
        y_offset: 0.0,
    });
    sim.widgets.mark_rect_dirty(tooltip_id);
    Ok(0)
}

fn apply_tooltip_anchor(
    tooltip: &mut crate::widget::Frame,
    anchor_kind: &str,
    owner_id: Option<u64>,
    mouse_position: Option<(f32, f32)>,
    x_offset: f32,
    y_offset: f32,
) {
    if anchor_kind == "ANCHOR_PRESERVE" {
        return;
    }
    tooltip.anchors.clear();
    if is_cursor_tooltip_anchor(anchor_kind) {
        if let Some((mx, my)) = mouse_position {
            tooltip
                .anchors
                .push(build_cursor_anchor(mx, my, x_offset, y_offset));
        }
        return;
    }

    if let Some(owner_id) = owner_id {
        push_owner_tooltip_anchor(tooltip, anchor_kind, owner_id, x_offset, y_offset);
    }
}

fn push_owner_tooltip_anchor(
    tooltip: &mut crate::widget::Frame,
    anchor_kind: &str,
    owner_id: u64,
    x_offset: f32,
    y_offset: f32,
) {
    let Some((point, relative_point)) = owner_tooltip_anchor_points(anchor_kind) else {
        return;
    };
    tooltip.anchors.push(crate::widget::Anchor {
        point,
        relative_to: None,
        relative_to_id: Some(owner_id as usize),
        relative_point,
        x_offset,
        y_offset,
    });
}

fn owner_tooltip_anchor_points(
    anchor_kind: &str,
) -> Option<(crate::widget::AnchorPoint, crate::widget::AnchorPoint)> {
    use crate::widget::AnchorPoint::{
        Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight,
    };

    match anchor_kind {
        "ANCHOR_RIGHT" => Some((Left, Right)),
        "ANCHOR_LEFT" => Some((Right, Left)),
        "ANCHOR_TOP" => Some((Bottom, Top)),
        "ANCHOR_BOTTOM" => Some((Top, Bottom)),
        "ANCHOR_TOPRIGHT" => Some((BottomRight, TopRight)),
        "ANCHOR_TOPLEFT" => Some((BottomLeft, TopLeft)),
        "ANCHOR_BOTTOMRIGHT" => Some((TopRight, BottomRight)),
        "ANCHOR_BOTTOMLEFT" => Some((TopLeft, BottomLeft)),
        _ => None,
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

pub(super) fn fade_out(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(tooltip) = sim.widgets.get_mut_visual(tooltip_id) {
            tooltip.tooltip_owner_id = None;
        }
        if let Some(td) = sim.tooltips.get_mut(&tooltip_id) {
            td.owner_id = None;
        }
    }
    crate::lua_api::frame::methods::core_state::hide(state)
}

pub(super) fn get_anchor_type(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let anchor = {
        let sim = borrow_state(state)?;
        sim.tooltips
            .get(&tooltip_id)
            .map(|td| td.anchor_type.clone())
            .unwrap_or_else(|| "ANCHOR_NONE".to_string())
    };
    let anchor_val = create_string(state, &anchor);
    state.push(anchor_val);
    Ok(1)
}

pub(super) fn set_anchor_type(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let anchor_kind = {
        let anchor_kind = opt_string(state, 2).unwrap_or_else(|| "ANCHOR_NONE".to_string());
        if is_valid_tooltip_anchor(&anchor_kind) {
            anchor_kind
        } else {
            let _ = collect_lua_error(
                state,
                &format!("invalid anchor type: {anchor_kind}; defaulting to ANCHOR_LEFT"),
            );
            "ANCHOR_LEFT".to_string()
        }
    };
    let x_offset = opt_f32(state, 3).unwrap_or(0.0);
    let y_offset = opt_f32(state, 4).unwrap_or(0.0);
    let owner_id = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(tooltip_id)
            .and_then(|tooltip| tooltip.tooltip_owner_id)
            .or_else(|| sim.tooltips.get(&tooltip_id).and_then(|td| td.owner_id))
    };
    let mut sim = borrow_state_mut(state)?;
    let mouse_position = sim.mouse_position;
    let Some(tooltip) = sim.widgets.get_mut_visual(tooltip_id) else {
        return Ok(0);
    };
    apply_tooltip_anchor(
        tooltip,
        &anchor_kind,
        owner_id,
        mouse_position,
        x_offset,
        y_offset,
    );
    let td = sim.tooltips.entry(tooltip_id).or_default();
    td.anchor_type = anchor_kind.clone();
    td.anchor_x_offset = x_offset;
    td.anchor_y_offset = y_offset;
    sim.widgets.mark_rect_dirty(tooltip_id);
    drop(sim);
    let fields = get_or_create_frame_fields(state, tooltip_id);
    let anchor_value = create_string(state, &anchor_kind);
    table_set(state, fields, "anchor", anchor_value);
    Ok(0)
}

fn is_valid_tooltip_anchor(anchor_kind: &str) -> bool {
    matches!(
        anchor_kind,
        "ANCHOR_NONE"
            | "ANCHOR_PRESERVE"
            | "ANCHOR_RIGHT"
            | "ANCHOR_LEFT"
            | "ANCHOR_TOP"
            | "ANCHOR_BOTTOM"
            | "ANCHOR_TOPRIGHT"
            | "ANCHOR_TOPLEFT"
            | "ANCHOR_BOTTOMRIGHT"
            | "ANCHOR_BOTTOMLEFT"
            | "ANCHOR_CURSOR"
            | "ANCHOR_CURSOR_RIGHT"
            | "ANCHOR_CURSOR_LEFT"
    )
}

fn is_cursor_tooltip_anchor(anchor_kind: &str) -> bool {
    matches!(
        anchor_kind,
        "ANCHOR_CURSOR" | "ANCHOR_CURSOR_RIGHT" | "ANCHOR_CURSOR_LEFT"
    )
}

pub(super) fn copy_tooltip(state: &mut LuaState) -> LuaResult<u32> {
    let target_id = frame_id_from_stack(state, 1)?;
    let Ok(source_id) = frame_id_from_stack(state, 2) else {
        return Ok(0);
    };
    let source = {
        let sim = borrow_state(state)?;
        sim.tooltips.get(&source_id).cloned()
    };
    let Some(source) = source else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    let target = sim.tooltips.entry(target_id).or_default();
    let preserved_owner = target.owner_id;
    let preserved_anchor = target.anchor_type.clone();
    let preserved_x = target.anchor_x_offset;
    let preserved_y = target.anchor_y_offset;
    *target = source;
    target.owner_id = preserved_owner;
    target.anchor_type = preserved_anchor;
    target.anchor_x_offset = preserved_x;
    target.anchor_y_offset = preserved_y;
    Ok(0)
}

pub(super) fn set_frame_stack(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(state, 1)?;
    let _show_hidden = opt_bool(state, 2).unwrap_or(false);
    let _show_regions = opt_bool(state, 3).unwrap_or(false);
    let frame_stack_index = opt_f32(state, 4).unwrap_or(0.0).max(0.0) as usize;
    let highlight_id = {
        let sim = borrow_state(state)?;
        sim.hovered_frame
    };
    let Some(highlight_id) = highlight_id else {
        clear_tooltip_lines(state, tooltip_id)?;
        state.push(Val::Nil);
        return Ok(1);
    };

    let frame_info = {
        let sim = borrow_state(state)?;
        sim.widgets.get(highlight_id).map(|frame| {
            let primary = frame.name.clone().unwrap_or_else(|| {
                frame
                    .object_type_name
                    .clone()
                    .unwrap_or_else(|| "Frame".into())
            });
            let parent_label = frame
                .parent_id
                .and_then(|pid| sim.widgets.get(pid))
                .and_then(|parent| parent.name.clone())
                .unwrap_or_else(|| frame.widget_type.as_str().to_string());
            (primary, parent_label)
        })
    };
    let Some((primary, parent_label)) = frame_info else {
        clear_tooltip_lines(state, tooltip_id)?;
        state.push(Val::Nil);
        return Ok(1);
    };
    let highlight = frame_global_or_ref_local(state, highlight_id)?;

    {
        let mut sim = borrow_state_mut(state)?;
        let td = sim.tooltips.entry(tooltip_id).or_default();
        td.frame_stack_index = frame_stack_index;
        td.lines.clear();
        td.lines.push(TooltipLine {
            left_text: primary,
            left_color: (1.0, 1.0, 1.0),
            left_segments: Vec::new(),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            right_segments: Vec::new(),
            wrap: false,
            texture: None,
        });
        td.lines.push(TooltipLine {
            left_text: format!("Parent: {parent_label}"),
            left_color: (0.8, 0.8, 0.8),
            left_segments: Vec::new(),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            right_segments: Vec::new(),
            wrap: false,
            texture: None,
        });
        sim.set_frame_visible(tooltip_id, true);
    }
    fire_tooltip_script_with_args(
        state,
        tooltip_id,
        "OnTooltipSetFramestack",
        &[highlight.clone()],
    );
    state.push(highlight);
    Ok(1)
}

fn clear_tooltip_lines(state: &mut LuaState, tooltip_id: u64) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    sim.tooltips.entry(tooltip_id).or_default().lines.clear();
    Ok(())
}

fn frame_global_or_ref_local(state: &mut LuaState, id: u64) -> LuaResult<Val> {
    let frame_name = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|frame| frame.name.clone())
    };
    if let Some(name) = frame_name {
        let key = state.gc.intern_string(name.as_bytes());
        let global = state
            .gc
            .tables
            .get(state.global)
            .map(|table| table.get_str(key, &state.gc.string_arena))
            .unwrap_or(Val::Nil);
        if global != Val::Nil {
            return Ok(global);
        }
    }
    frame_ref(state, id)
}

pub(super) fn add_font_strings(_state: &mut LuaState) -> LuaResult<u32> {
    let tooltip_id = frame_id_from_stack(_state, 1)?;
    let line_count = {
        let sim = borrow_state(_state)?;
        sim.tooltips
            .get(&tooltip_id)
            .map(|td| td.lines.len())
            .unwrap_or(0)
    };
    for line_index in 1..=line_count {
        let _ = sync_tooltip_line_frame(_state, tooltip_id, false, line_index)?;
        let _ = sync_tooltip_line_frame(_state, tooltip_id, true, line_index)?;
    }
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
    ("AddTexture", add_texture),
    ("AddAtlas", add_atlas),
    ("NumLines", num_lines),
    ("GetNumLines", num_lines),
    ("GetLeftLine", get_left_line),
    ("GetRightLine", get_right_line),
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
    ("SetObjectTooltipPosition", set_object_tooltip_position),
    ("GetOwner", get_owner),
    ("IsOwned", is_owned),
    ("FadeOut", fade_out),
    ("GetAnchorType", get_anchor_type),
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
