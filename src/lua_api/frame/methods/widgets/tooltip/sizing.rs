//! Lightweight tooltip sizing for Lua-visible geometry.
//!
//! The renderer performs the final glyph-backed measurement, but Lua queries
//! such as `GameTooltip:GetWidth()` can run before the next draw. Keep the
//! frame's stored size plausible as soon as tooltip content changes.

use crate::lua_api::tooltip::{TooltipData, TooltipLine};

const TOOLTIP_PADDING_H: f32 = 12.0;
const TOOLTIP_PADDING_V: f32 = 12.0;
const TOOLTIP_LINE_SPACING: f32 = 2.0;
const TOOLTIP_HEADER_FONT_SIZE: f32 = 14.0;
const TOOLTIP_BODY_FONT_SIZE: f32 = 12.0;
const DOUBLE_LINE_GAP: f32 = 20.0;
const APPROX_GLYPH_WIDTH: f32 = 0.55;
const TEXTURE_LINE_SIZE: f32 = 28.0;

pub(super) fn refresh_tooltip_geometry(sim: &mut crate::lua_api::state::SimState, tooltip_id: u64) {
    let Some((width, height)) = estimated_tooltip_size(sim.tooltips.get(&tooltip_id)) else {
        sim.widgets.mark_rect_dirty(tooltip_id);
        return;
    };

    let changed = if let Some(frame) = sim.widgets.get_mut_visual(tooltip_id) {
        let changed = frame.width != width || frame.height != height;
        frame.width = width;
        frame.height = height;
        changed
    } else {
        false
    };
    if changed {
        sim.widgets.mark_rect_dirty(tooltip_id);
    }
}

fn estimated_tooltip_size(td: Option<&TooltipData>) -> Option<(f32, f32)> {
    let td = td?;
    if td.lines.is_empty() {
        return None;
    }

    let mut width = td.min_width.max(wrapped_min_width(td));
    let mut height = 0.0;
    let line_spacing = td.line_spacing.unwrap_or(TOOLTIP_LINE_SPACING);
    for (index, line) in td.lines.iter().enumerate() {
        let font_size = tooltip_line_font_size(index);
        width = width.max(estimated_line_width(line, font_size));
        if index > 0 {
            height += line_spacing;
        }
        height += estimated_line_height(line, font_size);
    }

    Some((
        width + TOOLTIP_PADDING_H * 2.0,
        height + TOOLTIP_PADDING_V * 2.0,
    ))
}

fn wrapped_min_width(td: &TooltipData) -> f32 {
    if td.lines.iter().any(|line| line.wrap) {
        td.custom_word_wrap_min_width.unwrap_or(0.0)
    } else {
        0.0
    }
}

fn estimated_line_width(line: &TooltipLine, font_size: f32) -> f32 {
    let left = estimated_text_width(&line.left_text, font_size);
    let right = line
        .right_text
        .as_deref()
        .map(|text| estimated_text_width(text, font_size))
        .unwrap_or(0.0);
    let text_width = if right > 0.0 {
        left + right + DOUBLE_LINE_GAP
    } else {
        left
    };
    if line.texture.is_some() {
        text_width.max(TEXTURE_LINE_SIZE)
    } else {
        text_width
    }
}

fn estimated_text_width(text: &str, font_size: f32) -> f32 {
    crate::render::strip_wow_markup(text).chars().count() as f32 * font_size * APPROX_GLYPH_WIDTH
}

fn estimated_line_height(line: &TooltipLine, font_size: f32) -> f32 {
    if line.texture.is_some() {
        TEXTURE_LINE_SIZE
    } else {
        (font_size * 1.2).ceil()
    }
}

fn tooltip_line_font_size(index: usize) -> f32 {
    if index == 0 {
        TOOLTIP_HEADER_FONT_SIZE
    } else {
        TOOLTIP_BODY_FONT_SIZE
    }
}
