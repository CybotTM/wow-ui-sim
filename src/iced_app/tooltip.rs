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
const DOUBLE_LINE_GAP: f32 = 20.0;

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
    /// Measured height for this line (accounts for word-wrap).
    pub measured_height: f32,
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
///
/// Two-pass approach: first determine width from non-wrapping lines,
/// then measure wrapped lines' heights against that width.
fn measure_tooltip(state: &SimState, id: u64, font_system: &mut WowFontSystem) -> (f32, f32) {
    let td = match state.tooltips.get(&id) {
        Some(td) => td,
        None => return (0.0, 0.0),
    };

    let line_spacing = td.line_spacing.unwrap_or(TOOLTIP_LINE_SPACING);

    // Pass 1: determine content width from non-wrapping lines.
    let content_width = measure_tooltip_content_width(td, font_system);

    // Pass 2: sum line heights, using text measurement for wrapped lines.
    let mut total_height: f32 = 0.0;
    for (i, line) in td.lines.iter().enumerate() {
        let font_size = tooltip_line_font_size(i);
        let line_height = if line.wrap && !line.left_text.is_empty() {
            font_system.measure_text_height(&line.left_text, None, font_size, Some(content_width))
        } else {
            (font_size * 1.2).ceil()
        };
        if i > 0 {
            total_height += line_spacing;
        }
        total_height += line_height;
    }

    let width = content_width + TOOLTIP_PADDING_H * 2.0;
    let height = total_height + TOOLTIP_PADDING_V * 2.0;
    (width, height)
}

/// Measure the content width from non-wrapping tooltip lines.
/// Wrapping lines don't contribute to width — they wrap within it.
fn measure_tooltip_content_width(
    td: &crate::lua_api::tooltip::TooltipData,
    font_system: &mut WowFontSystem,
) -> f32 {
    let mut max_width: f32 = td.min_width;
    for (i, line) in td.lines.iter().enumerate() {
        if line.wrap {
            continue;
        }
        let font_size = tooltip_line_font_size(i);
        let left_w = font_system.measure_text_width(&line.left_text, None, font_size);
        let right_w = line
            .right_text
            .as_ref()
            .map(|t| font_system.measure_text_width(t, None, font_size))
            .unwrap_or(0.0);
        let line_width = if right_w > 0.0 {
            left_w + right_w + DOUBLE_LINE_GAP
        } else {
            left_w
        };
        max_width = max_width.max(line_width);
    }
    max_width
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
        measured_height: (tooltip_line_font_size(index) * 1.2).ceil(),
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
    let data = tooltip.tooltip_data.and_then(|map| map.get(&tooltip.id));
    let Some(data) = data else { return };

    emit_tooltip_background(tooltip.batch, tooltip.bounds, tooltip.eff_alpha);

    let Some((font_sys, glyph_atlas)) = text_ctx else {
        return;
    };
    let mut text_renderer = TooltipTextRenderer {
        batch: tooltip.batch,
        font_sys,
        glyph_atlas,
    };
    emit_tooltip_lines(&mut text_renderer, data, tooltip.bounds);
}

/// Render tooltip background: nine-slice border with black fill, or solid fallback.
fn emit_tooltip_background(batch: &mut QuadBatch, bounds: Rectangle, alpha: f32) {
    if let Some(ns) = tooltip_nine_slice() {
        let center = [0.0, 0.0, 0.0, alpha];
        super::nine_slice::emit_nine_slice_with_center_color(batch, bounds, ns, alpha, center, 4.0);
    } else {
        batch.push_solid(bounds, [0.0, 0.0, 0.0, alpha]);
        batch.push_border(bounds, 1.0, [0.6, 0.5, 0.15, alpha]);
    }
}

/// Render tooltip text lines with wrapping-aware height measurement.
fn emit_tooltip_lines(
    text_renderer: &mut TooltipTextRenderer<'_>,
    data: &TooltipRenderData,
    bounds: Rectangle,
) {
    let content_x = bounds.x + TOOLTIP_PADDING_H;
    let content_width = bounds.width - TOOLTIP_PADDING_H * 2.0;
    let mut y = bounds.y + TOOLTIP_PADDING_V;

    for line in &data.lines {
        let line_height = if line.wrap && !line.left_text.is_empty() {
            text_renderer.font_sys.measure_text_height(
                &line.left_text,
                None,
                line.font_size,
                Some(content_width),
            )
        } else {
            line.measured_height
        };

        emit_tooltip_line(
            text_renderer,
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

impl TooltipTextRenderer<'_> {
    fn emit(
        &mut self,
        text: &str,
        bounds: Rectangle,
        justify: TextJustify,
        font_size: f32,
        color: [f32; 4],
        wrap: bool,
    ) {
        emit_tooltip_text_run(
            self.batch,
            self.font_sys,
            self.glyph_atlas,
            text,
            bounds,
            justify,
            font_size,
            color,
            wrap,
        );
    }
}

/// Emit quads for a single tooltip line (left text, optional right text).
fn emit_tooltip_line(
    tr: &mut TooltipTextRenderer<'_>,
    line: &TooltipLineRender,
    placement: TooltipLinePlacement,
) {
    let right_width = line
        .right_text
        .as_ref()
        .map(|t| tr.font_sys.measure_text_width(t, None, line.font_size));

    let left_width = match right_width {
        Some(rw) if rw > 0.0 => (placement.width - rw - DOUBLE_LINE_GAP).max(0.0),
        _ => placement.width,
    };
    let left_bounds = tooltip_line_bounds(placement.x, placement.y, left_width, placement.height);
    tr.emit(
        &line.left_text,
        left_bounds,
        TextJustify::Left,
        line.font_size,
        line.left_color,
        line.wrap,
    );

    if let Some(ref right_text) = line.right_text {
        let right_x = placement.x + left_width + DOUBLE_LINE_GAP;
        let right_w = (placement.width - left_width - DOUBLE_LINE_GAP).max(0.0);
        let right_bounds = tooltip_line_bounds(right_x, placement.y, right_w, placement.height);
        tr.emit(
            right_text,
            right_bounds,
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
