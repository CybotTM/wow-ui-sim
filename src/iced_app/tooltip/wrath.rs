//! Wrath's ordered left-string width refinement, using simulator font metrics.

use super::{
    measure_tooltip_text_height, measure_tooltip_text_width, tooltip_line_font_size,
    tooltip_text_insets,
};
use crate::lua_api::SimState;
use crate::lua_api::tooltip::{TooltipData, TooltipLine};
use crate::render::font::WowFontSystem;

const DOUBLE_LINE_GAP: f32 = 38.400002;
const WRAP_CANDIDATE_CAP: f32 = 230.39999;
const MAX_WRAP_STARTS: usize = 30;
const ROW_GAP: f32 = 2.0;

pub(super) fn measure_tooltip(
    state: &mut SimState,
    id: u64,
    fonts: &mut WowFontSystem,
) -> (f32, f32) {
    let Some(td) = state.tooltips.get(&id).cloned() else {
        return (0.0, 0.0);
    };
    let mut width = unwrapped_width(state, &td, fonts);
    let mut height = 0.0;
    for (index, line) in td.lines.iter().enumerate() {
        let font_size = tooltip_line_font_size(index);
        let left_id = td.left_line_ids.get(index).copied();
        let left_width = refine_left_width(state, left_id, line, width, fonts, font_size);
        let extra = attachment_width(state, &td, index);
        width = width.max(left_width + extra);
        let wrap_width = line.wrap.then_some(left_width);
        height += measure_tooltip_text_height(fonts, &line.left_text, font_size, wrap_width);
        if index > 0 {
            height += td.line_spacing.unwrap_or(ROW_GAP);
        }
    }
    let insets = tooltip_text_insets();
    (
        width + insets.left + insets.right,
        height + insets.top + insets.bottom,
    )
}

fn unwrapped_width(state: &SimState, td: &TooltipData, fonts: &mut WowFontSystem) -> f32 {
    td.lines
        .iter()
        .enumerate()
        .filter(|(_, line)| !line.wrap)
        .fold(td.min_width, |width, (index, line)| {
            let font_size = tooltip_line_font_size(index);
            let left = measure_tooltip_text_width(fonts, &line.left_text, font_size);
            let right = line
                .right_text
                .as_deref()
                .filter(|text| !text.is_empty())
                .map_or(0.0, |text| {
                    measure_tooltip_text_width(fonts, text, font_size) + DOUBLE_LINE_GAP
                });
            width.max(left + right + attachment_width(state, td, index))
        })
}

fn refine_left_width(
    state: &mut SimState,
    left_id: Option<u64>,
    line: &TooltipLine,
    shared: f32,
    fonts: &mut WowFontSystem,
    font_size: f32,
) -> f32 {
    let measured = measure_tooltip_text_width(fonts, &line.left_text, font_size);
    if !line.wrap {
        return measured;
    }
    let candidate = measured.min(WRAP_CANDIDATE_CAP);
    if shared >= candidate {
        return left_id
            .and_then(|id| state.widgets.get(id))
            .map_or(shared, |frame| frame.width);
    }
    let text = crate::render::strip_wow_markup(&line.left_text);
    let starts = fonts.measure_wrap_starts(&text, font_size, candidate);
    let Some(width) = wrapped_segment_width(fonts, &text, &starts, font_size) else {
        return shared;
    };
    let width = width.max(shared);
    if let Some(id) = left_id {
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.width = width;
            frame.width_is_text_auto = false;
        }
        state.widgets.mark_rect_dirty(id);
    }
    width
}

fn wrapped_segment_width(
    fonts: &mut WowFontSystem,
    text: &str,
    starts: &[usize],
    font_size: f32,
) -> Option<f32> {
    let starts = &starts[..starts.len().min(MAX_WRAP_STARTS)];
    if starts.is_empty() {
        return None;
    }
    Some(
        starts
            .iter()
            .enumerate()
            .map(|(index, &start)| {
                let end = starts.get(index + 1).copied().unwrap_or(text.len());
                measure_tooltip_text_width(fonts, &text[start..end], font_size)
            })
            .fold(0.0, f32::max),
    )
}

fn attachment_width(state: &SimState, td: &TooltipData, row: usize) -> f32 {
    let gap = 0.005 * (state.screen_width / state.screen_height) * 1024.0;
    td.wrath_texture_rows
        .iter()
        .rev()
        .find(|(index, _)| *index == row)
        .and_then(|(_, id)| state.widgets.get(*id))
        .map_or(0.0, |frame| frame.width + gap)
}
