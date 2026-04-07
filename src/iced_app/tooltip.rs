//! Tooltip visual rendering — sizing, data collection, and quad emission.

use std::collections::HashMap;
use std::sync::OnceLock;

use iced::{Point, Rectangle, Size};

use crate::atlas::{NineSliceAtlasInfo, get_nine_slice_atlas_info};
use crate::lua_api::SimState;
use crate::render::QuadBatch;
use crate::render::font::WowFontSystem;
use crate::render::glyph::{GlyphAtlas, emit_text_quads};
use crate::render::shader::GLYPH_ATLAS_TEX_INDEX;
use crate::widget::{TextJustify, TextOutline};

/// Cached nine-slice atlas info for the default tooltip border.
fn tooltip_nine_slice() -> Option<&'static NineSliceAtlasInfo> {
    static CACHE: OnceLock<Option<NineSliceAtlasInfo>> = OnceLock::new();
    CACHE
        .get_or_init(|| get_nine_slice_atlas_info("Tooltip"))
        .as_ref()
}

const TOOLTIP_PADDING_H: f32 = 12.0;
const TOOLTIP_PADDING_V: f32 = 12.0;
const TOOLTIP_LINE_SPACING: f32 = 2.0;
const TOOLTIP_HEADER_FONT_SIZE: f32 = 14.0;
const TOOLTIP_BODY_FONT_SIZE: f32 = 12.0;

/// Pre-collected tooltip render data for a single tooltip frame.
pub struct TooltipRenderData {
    pub lines: Vec<TooltipLineRender>,
    pub line_spacing: f32,
}

/// A single line ready for rendering.
pub struct TooltipLineRender {
    pub left_text: String,
    pub left_color: [f32; 4],
    pub right_text: Option<String>,
    pub right_color: [f32; 4],
    pub font_size: f32,
    pub wrap: bool,
}

/// Update tooltip frame sizes based on their text content.
///
/// Must be called before layout computation so anchors resolve with correct dimensions.
pub fn update_tooltip_sizes(state: &mut SimState, font_system: &mut WowFontSystem) {
    let tooltip_ids: Vec<u64> = state.tooltips.keys().copied().collect();
    for id in tooltip_ids {
        let (lines_empty, visible) = {
            let td = match state.tooltips.get(&id) {
                Some(td) => td,
                None => continue,
            };
            let visible = state.widgets.get(id).map(|f| f.visible).unwrap_or(false);
            (td.lines.is_empty(), visible)
        };
        if lines_empty || !visible {
            continue;
        }
        let (width, height) = measure_tooltip(state, id, font_system);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.width = width;
            frame.height = height;
        }
    }
}

/// Measure a tooltip's required width and height from its lines.
fn measure_tooltip(state: &SimState, id: u64, font_system: &mut WowFontSystem) -> (f32, f32) {
    let td = match state.tooltips.get(&id) {
        Some(td) => td,
        None => return (0.0, 0.0),
    };

    let line_spacing = td.line_spacing.unwrap_or(TOOLTIP_LINE_SPACING);
    let mut max_width: f32 = td.min_width;
    let mut total_height: f32 = 0.0;

    for (i, line) in td.lines.iter().enumerate() {
        let font_size = if i == 0 {
            TOOLTIP_HEADER_FONT_SIZE
        } else {
            TOOLTIP_BODY_FONT_SIZE
        };
        let left_w = font_system.measure_text_width(&line.left_text, None, font_size);
        let right_w = line
            .right_text
            .as_ref()
            .map(|t| font_system.measure_text_width(t, None, font_size))
            .unwrap_or(0.0);

        let line_width = if right_w > 0.0 {
            left_w + right_w + 20.0 // gap between left and right
        } else {
            left_w
        };
        max_width = max_width.max(line_width);

        let line_height = (font_size * 1.2).ceil();
        if i > 0 {
            total_height += line_spacing;
        }
        total_height += line_height;
    }

    let width = max_width + TOOLTIP_PADDING_H * 2.0;
    let height = total_height + TOOLTIP_PADDING_V * 2.0;
    (width, height)
}

/// Collect render data for all visible tooltips with lines.
pub fn collect_tooltip_data(state: &SimState) -> HashMap<u64, TooltipRenderData> {
    let mut result = HashMap::new();
    for (&id, td) in &state.tooltips {
        let Some(alpha) = tooltip_alpha(state, id, td) else {
            continue;
        };
        let lines = collect_tooltip_lines(td, alpha);
        let line_spacing = td.line_spacing.unwrap_or(TOOLTIP_LINE_SPACING);
        result.insert(
            id,
            TooltipRenderData {
                lines,
                line_spacing,
            },
        );
    }
    result
}

fn tooltip_alpha(
    state: &SimState,
    id: u64,
    td: &crate::lua_api::tooltip::TooltipData,
) -> Option<f32> {
    if td.lines.is_empty() {
        return None;
    }

    let frame = state.widgets.get(id)?;
    if !frame.visible {
        return None;
    }

    Some(frame.alpha)
}

fn collect_tooltip_lines(
    td: &crate::lua_api::tooltip::TooltipData,
    alpha: f32,
) -> Vec<TooltipLineRender> {
    td.lines
        .iter()
        .enumerate()
        .map(|(i, line)| tooltip_line_render(i, line, alpha))
        .collect()
}

fn tooltip_line_render(
    index: usize,
    line: &crate::lua_api::tooltip::TooltipLine,
    alpha: f32,
) -> TooltipLineRender {
    TooltipLineRender {
        left_text: line.left_text.clone(),
        left_color: [
            line.left_color.0,
            line.left_color.1,
            line.left_color.2,
            alpha,
        ],
        right_text: line.right_text.clone(),
        right_color: [
            line.right_color.0,
            line.right_color.1,
            line.right_color.2,
            alpha,
        ],
        font_size: tooltip_line_font_size(index),
        wrap: line.wrap,
    }
}

fn tooltip_line_font_size(index: usize) -> f32 {
    if index == 0 {
        TOOLTIP_HEADER_FONT_SIZE
    } else {
        TOOLTIP_BODY_FONT_SIZE
    }
}

/// Emit quads for a GameTooltip frame: background, border, and text lines.
pub fn build_tooltip_quads(
    tooltip: TooltipRender<'_>,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
) {
    // Only render when there are lines to display — otherwise the tooltip is
    // "owned" but has no content yet (e.g. during addon init).
    let data = tooltip.tooltip_data.and_then(|map| map.get(&tooltip.id));
    let Some(data) = data else { return };

    let alpha = tooltip.eff_alpha;

    // Tooltip border and background via nine-slice atlas (rounded corners).
    // WoW calls SetCenterColor(0, 0, 0, 1) — TOOLTIP_DEFAULT_BACKGROUND_COLOR is black.
    if let Some(ns) = tooltip_nine_slice() {
        let center = [0.0, 0.0, 0.0, alpha];
        // WoW's TooltipDefaultLayout anchors center with (-4,4,4,-4) offsets,
        // extending the fill 4px into each corner to cover transparent inner areas.
        super::nine_slice::emit_nine_slice_with_center_color(
            tooltip.batch,
            tooltip.bounds,
            ns,
            alpha,
            center,
            4.0,
        );
    } else {
        // Fallback if atlas entries are missing
        tooltip
            .batch
            .push_solid(tooltip.bounds, [0.0, 0.0, 0.0, alpha]);
        tooltip
            .batch
            .push_border(tooltip.bounds, 1.0, [0.6, 0.5, 0.15, alpha]);
    }

    let Some((font_sys, glyph_atlas)) = text_ctx else {
        return;
    };
    let mut text_renderer = TooltipTextRenderer {
        batch: tooltip.batch,
        font_sys,
        glyph_atlas,
    };

    let content_x = tooltip.bounds.x + TOOLTIP_PADDING_H;
    let content_width = tooltip.bounds.width - TOOLTIP_PADDING_H * 2.0;
    let mut y = tooltip.bounds.y + TOOLTIP_PADDING_V;

    for line in &data.lines {
        let line_height = (line.font_size * 1.2).ceil();

        emit_tooltip_line(
            &mut text_renderer,
            line,
            TooltipLinePlacement {
                x: content_x,
                y,
                width: content_width,
                height: line_height,
            },
        );

        y += line_height + data.line_spacing;
    }
}

pub struct TooltipRender<'a> {
    pub batch: &'a mut QuadBatch,
    pub bounds: Rectangle,
    pub tooltip_data: Option<&'a HashMap<u64, TooltipRenderData>>,
    pub id: u64,
    pub eff_alpha: f32,
}

struct TooltipTextRenderer<'a> {
    batch: &'a mut QuadBatch,
    font_sys: &'a mut WowFontSystem,
    glyph_atlas: &'a mut GlyphAtlas,
}

/// Emit quads for a single tooltip line (left text, optional right text).
fn emit_tooltip_line(
    text_renderer: &mut TooltipTextRenderer<'_>,
    line: &TooltipLineRender,
    placement: TooltipLinePlacement,
) {
    let bounds = tooltip_line_bounds(placement.x, placement.y, placement.width, placement.height);
    emit_tooltip_text_run(
        text_renderer.batch,
        text_renderer.font_sys,
        text_renderer.glyph_atlas,
        &line.left_text,
        bounds,
        TextJustify::Left,
        line.font_size,
        line.left_color,
        line.wrap,
    );

    if let Some(ref right_text) = line.right_text {
        emit_tooltip_text_run(
            text_renderer.batch,
            text_renderer.font_sys,
            text_renderer.glyph_atlas,
            right_text,
            bounds,
            TextJustify::Right,
            line.font_size,
            line.right_color,
            false,
        );
    }
}

struct TooltipLinePlacement {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn tooltip_line_bounds(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
    Rectangle::new(Point::new(x, y), Size::new(width, height))
}

#[allow(clippy::too_many_arguments)]
fn emit_tooltip_text_run(
    batch: &mut QuadBatch,
    font_sys: &mut WowFontSystem,
    glyph_atlas: &mut GlyphAtlas,
    text: &str,
    bounds: Rectangle,
    horiz_justify: TextJustify,
    font_size: f32,
    color: [f32; 4],
    wrap: bool,
) {
    emit_text_quads(
        batch,
        font_sys,
        glyph_atlas,
        text,
        bounds,
        None,
        font_size,
        color,
        horiz_justify,
        TextJustify::Center,
        GLYPH_ATLAS_TEX_INDEX,
        None,
        (0.0, 0.0),
        TextOutline::None,
        wrap,
        0,
        None,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::tooltip::{TooltipData, TooltipLine};
    use crate::widget::{Frame, WidgetType};

    #[test]
    fn collect_tooltip_data_applies_alpha_and_font_sizes() {
        let mut state = SimState::default();
        let mut frame = Frame::new(WidgetType::Frame, Some("GameTooltip".to_string()), None);
        frame.id = 42;
        frame.visible = true;
        frame.alpha = 0.35;
        state.widgets.register(frame);
        state.tooltips.insert(
            42,
            TooltipData {
                lines: vec![
                    TooltipLine {
                        left_text: "Header".to_string(),
                        left_color: (1.0, 0.5, 0.25),
                        right_text: Some("Right".to_string()),
                        right_color: (0.2, 0.3, 0.4),
                        wrap: false,
                        texture: None,
                    },
                    TooltipLine {
                        left_text: "Body".to_string(),
                        left_color: (0.1, 0.2, 0.3),
                        right_text: None,
                        right_color: (0.0, 0.0, 0.0),
                        wrap: true,
                        texture: None,
                    },
                ],
                ..TooltipData::default()
            },
        );

        let data = collect_tooltip_data(&state);
        let lines = &data.get(&42).unwrap().lines;

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].font_size, TOOLTIP_HEADER_FONT_SIZE);
        assert_eq!(lines[1].font_size, TOOLTIP_BODY_FONT_SIZE);
        assert_eq!(lines[0].left_color, [1.0, 0.5, 0.25, 0.35]);
        assert_eq!(lines[0].right_color, [0.2, 0.3, 0.4, 0.35]);
        assert_eq!(lines[1].left_color, [0.1, 0.2, 0.3, 0.35]);
        assert!(lines[1].wrap);
    }
}
