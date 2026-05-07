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
const TOOLTIP_CENTER_OVERLAP: f32 = 4.0;

/// Pre-collected tooltip render data for a single tooltip frame.
pub struct TooltipRenderData {
    pub lines: Vec<TooltipLineRender>,
    pub line_spacing: f32,
}

/// A single line ready for rendering.
pub struct TooltipLineRender {
    pub left_text: String,
    pub left_color: [f32; 4],
    pub left_segments: Vec<TooltipTextSegmentRender>,
    pub right_text: Option<String>,
    pub right_color: [f32; 4],
    pub right_segments: Vec<TooltipTextSegmentRender>,
    pub font_size: f32,
    pub wrap: bool,
    /// Measured height for this line (accounts for word-wrap).
    pub measured_height: f32,
}

pub struct TooltipTextSegmentRender {
    pub text: String,
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TooltipTextInsets {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

/// Update tooltip frame sizes based on their text content.
///
/// Must be called before layout computation so anchors resolve with correct dimensions.
pub fn update_tooltip_sizes(state: &mut SimState, font_system: &mut WowFontSystem) {
    let tooltip_ids: Vec<u64> = state.tooltips.keys().copied().collect();
    for id in tooltip_ids {
        let (has_render_content, visible) = {
            let td = match state.tooltips.get(&id) {
                Some(td) => td,
                None => continue,
            };
            let visible = state.widgets.get(id).map(|f| f.visible).unwrap_or(false);
            (tooltip_has_render_content(td), visible)
        };
        if !has_render_content || !visible {
            continue;
        }
        let (width, height) = measure_tooltip(state, id, font_system);
        let changed = if let Some(frame) = state.widgets.get_mut_visual(id) {
            let changed = frame.width != width || frame.height != height;
            frame.width = width;
            frame.height = height;
            changed
        } else {
            false
        };
        if changed {
            state.widgets.mark_rect_dirty(id);
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

    let insets = tooltip_text_insets();
    let width = content_width + insets.left + insets.right;
    let height = total_height + insets.top + insets.bottom;
    (width, height)
}

/// Measure the content width from non-wrapping tooltip lines.
/// Wrapping lines don't contribute to width — they wrap within it.
fn measure_tooltip_content_width(
    td: &crate::lua_api::tooltip::TooltipData,
    font_system: &mut WowFontSystem,
) -> f32 {
    let mut max_width: f32 = td.min_width;
    let mut wrapped_only_max_width: f32 = 0.0;
    let mut measured_non_wrapped_line = false;
    if tooltip_has_wrapped_lines(td) {
        max_width = max_width.max(tooltip_wrapped_min_width(td));
    }
    for (i, line) in td.lines.iter().enumerate() {
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
        let has_width_text = tooltip_line_has_width_text(line);
        if line.wrap && td.shrink_to_fit_wrapped {
            if has_width_text {
                wrapped_only_max_width = wrapped_only_max_width.max(line_width);
            }
            continue;
        }
        if !has_width_text {
            continue;
        }
        measured_non_wrapped_line = true;
        max_width = max_width.max(line_width);
    }
    if !measured_non_wrapped_line {
        max_width = max_width.max(wrapped_only_max_width);
    }
    max_width
}

fn tooltip_wrapped_min_width(td: &crate::lua_api::tooltip::TooltipData) -> f32 {
    td.custom_word_wrap_min_width.unwrap_or(0.0)
}

fn tooltip_has_wrapped_lines(td: &crate::lua_api::tooltip::TooltipData) -> bool {
    td.lines.iter().any(|line| line.wrap)
}

fn tooltip_line_has_width_text(line: &crate::lua_api::tooltip::TooltipLine) -> bool {
    !line.left_text.trim().is_empty()
        || line
            .right_text
            .as_ref()
            .is_some_and(|text| !text.trim().is_empty())
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
    if !tooltip_has_render_content(td) {
        return None;
    }

    let frame = state.widgets.get(id)?;
    if !frame.visible {
        return None;
    }

    Some(frame.alpha)
}

fn tooltip_has_render_content(td: &crate::lua_api::tooltip::TooltipData) -> bool {
    !td.lines.is_empty() || td.allow_show_with_no_lines
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
    let left_color = [
        line.left_color.0,
        line.left_color.1,
        line.left_color.2,
        alpha,
    ];
    let right_color = [
        line.right_color.0,
        line.right_color.1,
        line.right_color.2,
        alpha,
    ];
    TooltipLineRender {
        left_text: line.left_text.clone(),
        left_color,
        left_segments: tooltip_text_segment_renders(&line.left_segments, alpha),
        right_text: line.right_text.clone(),
        right_color,
        right_segments: tooltip_text_segment_renders(&line.right_segments, alpha),
        font_size: tooltip_line_font_size(index),
        wrap: line.wrap,
        measured_height: (tooltip_line_font_size(index) * 1.2).ceil(),
    }
}

fn tooltip_text_segment_renders(
    segments: &[crate::lua_api::tooltip::TooltipTextSegment],
    alpha: f32,
) -> Vec<TooltipTextSegmentRender> {
    segments
        .iter()
        .map(|segment| TooltipTextSegmentRender {
            text: segment.text.clone(),
            color: [segment.color.0, segment.color.1, segment.color.2, alpha],
        })
        .collect()
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

    if tooltip.draw_background {
        emit_tooltip_background(tooltip.batch, tooltip.bounds, tooltip.eff_alpha);
    } else {
        emit_tooltip_center_fill(tooltip.batch, tooltip.bounds, tooltip.eff_alpha);
    }

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

fn emit_tooltip_center_fill(batch: &mut QuadBatch, bounds: Rectangle, alpha: f32) {
    let center_color = [0.0, 0.0, 0.0, alpha];
    let Some(ns) = tooltip_nine_slice() else {
        batch.push_solid(bounds, center_color);
        return;
    };

    let x = bounds.x + ns.corner_tl.width as f32;
    let y = bounds.y + ns.corner_tl.height as f32;
    let width = bounds.width - ns.corner_tl.width as f32 - ns.corner_tr.width as f32;
    let height = bounds.height - ns.corner_tl.height as f32 - ns.corner_bl.height as f32;
    if width > 0.0 && height > 0.0 {
        batch.push_solid(
            Rectangle::new(Point::new(x, y), Size::new(width, height)),
            center_color,
        );
    }
}

/// Render tooltip text lines with wrapping-aware height measurement.
fn emit_tooltip_lines(
    text_renderer: &mut TooltipTextRenderer<'_>,
    data: &TooltipRenderData,
    bounds: Rectangle,
) {
    let insets = tooltip_text_insets();
    let content_x = bounds.x + insets.left;
    let content_width = bounds.width - insets.left - insets.right;
    let mut y = bounds.y + insets.top;

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

fn tooltip_text_insets() -> TooltipTextInsets {
    match tooltip_nine_slice() {
        Some(ns) => TooltipTextInsets {
            left: TOOLTIP_PADDING_H + tooltip_center_inset(ns.corner_tl.width),
            right: TOOLTIP_PADDING_H + tooltip_center_inset(ns.corner_tr.width),
            top: TOOLTIP_PADDING_V + tooltip_center_inset(ns.corner_tl.height),
            bottom: TOOLTIP_PADDING_V + tooltip_center_inset(ns.corner_bl.height),
        },
        None => TooltipTextInsets {
            left: TOOLTIP_PADDING_H,
            right: TOOLTIP_PADDING_H,
            top: TOOLTIP_PADDING_V,
            bottom: TOOLTIP_PADDING_V,
        },
    }
}

fn tooltip_center_inset(piece_extent: u32) -> f32 {
    (piece_extent as f32 - TOOLTIP_CENTER_OVERLAP).max(0.0)
}

pub struct TooltipRender<'a> {
    pub batch: &'a mut QuadBatch,
    pub bounds: Rectangle,
    pub tooltip_data: Option<&'a HashMap<u64, TooltipRenderData>>,
    pub id: u64,
    pub eff_alpha: f32,
    pub draw_background: bool,
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
        let run = TooltipTextRun {
            text,
            bounds,
            justify,
            font_size,
            color,
            wrap,
        };
        emit_tooltip_text_run(self.batch, self.font_sys, self.glyph_atlas, run);
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
    emit_left_tooltip_text(tr, line, placement, left_width);

    if let Some(ref right_text) = line.right_text {
        emit_right_tooltip_text(tr, line, placement, left_width, right_text);
    }
}

fn emit_left_tooltip_text(
    tr: &mut TooltipTextRenderer<'_>,
    line: &TooltipLineRender,
    placement: TooltipLinePlacement,
    left_width: f32,
) {
    let left_bounds = tooltip_line_bounds(placement.x, placement.y, left_width, placement.height);
    emit_tooltip_text_segments(
        tr,
        TooltipTextSegments {
            text: &line.left_text,
            segments: &line.left_segments,
            bounds: left_bounds,
            justify: TextJustify::Left,
            font_size: line.font_size,
            color: line.left_color,
            wrap: line.wrap,
        },
    );
}

fn emit_right_tooltip_text(
    tr: &mut TooltipTextRenderer<'_>,
    line: &TooltipLineRender,
    placement: TooltipLinePlacement,
    left_width: f32,
    right_text: &str,
) {
    let right_x = placement.x + left_width + DOUBLE_LINE_GAP;
    let right_w = (placement.width - left_width - DOUBLE_LINE_GAP).max(0.0);
    let right_bounds = tooltip_line_bounds(right_x, placement.y, right_w, placement.height);
    emit_tooltip_text_segments(
        tr,
        TooltipTextSegments {
            text: right_text,
            segments: &line.right_segments,
            bounds: right_bounds,
            justify: TextJustify::Right,
            font_size: line.font_size,
            color: line.right_color,
            wrap: false,
        },
    );
}

#[derive(Clone, Copy)]
struct TooltipTextSegments<'a> {
    text: &'a str,
    segments: &'a [TooltipTextSegmentRender],
    bounds: Rectangle,
    justify: TextJustify,
    font_size: f32,
    color: [f32; 4],
    wrap: bool,
}

struct TooltipSegmentFlow {
    x: f32,
    y: f32,
    line_height: f32,
    right: f32,
}

fn emit_tooltip_text_segments(
    tr: &mut TooltipTextRenderer<'_>,
    segment_run: TooltipTextSegments<'_>,
) {
    if segment_run.segments.is_empty() {
        tr.emit(
            segment_run.text,
            segment_run.bounds,
            segment_run.justify,
            segment_run.font_size,
            segment_run.color,
            segment_run.wrap,
        );
        return;
    }

    let line_height = (segment_run.font_size * 1.2).ceil();
    let total_width =
        measure_tooltip_segments_width(tr, segment_run.segments, segment_run.font_size);
    let mut flow = TooltipSegmentFlow {
        x: tooltip_segment_start_x(
            segment_run.bounds,
            segment_run.justify,
            total_width,
            segment_run.wrap,
        ),
        y: segment_run.bounds.y,
        line_height,
        right: segment_run.bounds.x + segment_run.bounds.width,
    };

    for segment in segment_run.segments {
        emit_tooltip_segment_chunks(tr, segment_run, segment, &mut flow);
    }
}

fn emit_tooltip_segment_chunks(
    tr: &mut TooltipTextRenderer<'_>,
    segment_run: TooltipTextSegments<'_>,
    segment: &TooltipTextSegmentRender,
    flow: &mut TooltipSegmentFlow,
) {
    for chunk in tooltip_segment_chunks(&segment.text, segment_run.wrap) {
        let chunk_width = tr
            .font_sys
            .measure_text_width(chunk, None, segment_run.font_size);
        if starts_wrapped_tooltip_segment_line(
            segment_run.wrap,
            chunk,
            flow.x,
            chunk_width,
            segment_run.bounds.x,
            flow.right,
        ) {
            flow.x = segment_run.bounds.x;
            flow.y += flow.line_height;
        }
        let chunk_bounds = tooltip_line_bounds(
            flow.x,
            flow.y,
            chunk_width.max(1.0),
            flow.line_height.min(segment_run.bounds.height),
        );
        tr.emit(
            chunk,
            chunk_bounds,
            TextJustify::Left,
            segment_run.font_size,
            segment.color,
            false,
        );
        flow.x += chunk_width;
    }
}

fn measure_tooltip_segments_width(
    tr: &mut TooltipTextRenderer<'_>,
    segments: &[TooltipTextSegmentRender],
    font_size: f32,
) -> f32 {
    segments
        .iter()
        .map(|segment| {
            tr.font_sys
                .measure_text_width(&segment.text, None, font_size)
        })
        .sum()
}

fn tooltip_segment_start_x(
    bounds: Rectangle,
    justify: TextJustify,
    total_width: f32,
    wrap: bool,
) -> f32 {
    if wrap {
        return bounds.x;
    }
    match justify {
        TextJustify::Center => bounds.x + ((bounds.width - total_width) / 2.0).max(0.0),
        TextJustify::Right => bounds.x + (bounds.width - total_width).max(0.0),
        _ => bounds.x,
    }
}

fn tooltip_segment_chunks(text: &str, wrap: bool) -> Box<dyn Iterator<Item = &str> + '_> {
    if wrap {
        Box::new(text.split_inclusive(char::is_whitespace))
    } else {
        Box::new(std::iter::once(text))
    }
}

fn starts_wrapped_tooltip_segment_line(
    wrap: bool,
    chunk: &str,
    x: f32,
    chunk_width: f32,
    left: f32,
    right: f32,
) -> bool {
    wrap && x > left && x + chunk_width > right && !chunk.trim().is_empty()
}

#[derive(Clone, Copy)]
struct TooltipLinePlacement {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct TooltipTextRun<'a> {
    text: &'a str,
    bounds: Rectangle,
    justify: TextJustify,
    font_size: f32,
    color: [f32; 4],
    wrap: bool,
}

fn tooltip_line_bounds(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
    Rectangle::new(Point::new(x, y), Size::new(width, height))
}

fn emit_tooltip_text_run(
    batch: &mut QuadBatch,
    font_sys: &mut WowFontSystem,
    glyph_atlas: &mut GlyphAtlas,
    run: TooltipTextRun<'_>,
) {
    emit_text_quads(
        batch,
        font_sys,
        glyph_atlas,
        run.text,
        run.bounds,
        None,
        run.font_size,
        run.color,
        run.justify,
        TextJustify::Center,
        GLYPH_ATLAS_TEX_INDEX,
        None,
        (0.0, 0.0),
        TextOutline::None,
        run.wrap,
        0,
        None,
    );
}

#[cfg(test)]
mod tests {
    use iced::{Point, Rectangle, Size};

    use super::*;
    use crate::lua_api::tooltip::{TooltipData, TooltipLine, TooltipTextSegment};
    use crate::render::{GlyphAtlas, QuadBatch, TextureRequest, WowFontSystem};
    use crate::widget::{Frame, WidgetType};

    fn request_bounds(batch: &QuadBatch, request: &TextureRequest) -> (f32, f32, f32, f32) {
        let start = request.vertex_start as usize;
        let end = start + request.vertex_count as usize;
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for vertex in &batch.vertices[start..end] {
            min_x = min_x.min(vertex.position[0]);
            min_y = min_y.min(vertex.position[1]);
            max_x = max_x.max(vertex.position[0]);
            max_y = max_y.max(vertex.position[1]);
        }

        (min_x, min_y, max_x, max_y)
    }

    fn union_bounds(
        bounds: impl Iterator<Item = (f32, f32, f32, f32)>,
    ) -> Option<(f32, f32, f32, f32)> {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut found = false;

        for (x0, y0, x1, y1) in bounds {
            min_x = min_x.min(x0);
            min_y = min_y.min(y0);
            max_x = max_x.max(x1);
            max_y = max_y.max(y1);
            found = true;
        }

        found.then_some((min_x, min_y, max_x, max_y))
    }

    fn glyph_bounds(batch: &QuadBatch) -> Option<(f32, f32, f32, f32)> {
        let glyph_tex_index = GLYPH_ATLAS_TEX_INDEX;
        union_bounds(batch.vertices.iter().filter_map(|vertex| {
            (vertex.tex_index == glyph_tex_index).then_some((
                vertex.position[0],
                vertex.position[1],
                vertex.position[0],
                vertex.position[1],
            ))
        }))
    }

    fn has_glyph_color(batch: &QuadBatch, color: [f32; 4]) -> bool {
        batch
            .vertices
            .iter()
            .any(|vertex| vertex.tex_index == GLYPH_ATLAS_TEX_INDEX && vertex.color == color)
    }

    fn has_solid_color(batch: &QuadBatch, color: [f32; 4]) -> bool {
        batch
            .vertices
            .iter()
            .any(|vertex| vertex.tex_index == -1 && vertex.color == color)
    }

    fn glyph_bounds_for_color(batch: &QuadBatch, color: [f32; 4]) -> Option<(f32, f32, f32, f32)> {
        union_bounds(batch.vertices.iter().filter_map(|vertex| {
            (vertex.tex_index == GLYPH_ATLAS_TEX_INDEX && vertex.color == color).then_some((
                vertex.position[0],
                vertex.position[1],
                vertex.position[0],
                vertex.position[1],
            ))
        }))
    }

    fn request_bounds_by_base_path(
        batch: &QuadBatch,
        base_path: &str,
    ) -> Option<(f32, f32, f32, f32)> {
        batch
            .texture_requests
            .iter()
            .find(|request| request.path.starts_with(base_path))
            .map(|request| request_bounds(batch, request))
    }

    fn render_single_line_tooltip_batch(
        bounds: Rectangle,
    ) -> (QuadBatch, (f32, f32, f32, f32), (f32, f32, f32, f32)) {
        let batch = build_single_line_tooltip_batch(bounds);
        let border_bounds = tooltip_border_bounds(&batch);
        let glyph_bounds = glyph_bounds(&batch).expect("tooltip text should emit glyph vertices");

        (batch, border_bounds, glyph_bounds)
    }

    fn build_single_line_tooltip_batch(bounds: Rectangle) -> QuadBatch {
        let data = TooltipRenderData {
            lines: vec![single_header_tooltip_line()],
            line_spacing: TOOLTIP_LINE_SPACING,
        };
        let mut batch = QuadBatch::new();
        let mut font_sys = WowFontSystem::new();
        let mut glyph_atlas = GlyphAtlas::new();
        let tooltip_data = HashMap::from([(42_u64, data)]);
        let mut text_ctx = Some((&mut font_sys, &mut glyph_atlas));

        build_tooltip_quads(
            TooltipRender {
                batch: &mut batch,
                bounds,
                tooltip_data: Some(&tooltip_data),
                id: 42,
                eff_alpha: 1.0,
                draw_background: true,
            },
            &mut text_ctx,
        );

        batch
    }

    fn single_header_tooltip_line() -> TooltipLineRender {
        TooltipLineRender {
            left_text: "Header".to_string(),
            left_color: [1.0, 1.0, 1.0, 1.0],
            left_segments: Vec::new(),
            right_text: None,
            right_color: [1.0, 1.0, 1.0, 1.0],
            right_segments: Vec::new(),
            font_size: TOOLTIP_HEADER_FONT_SIZE,
            wrap: false,
            measured_height: (TOOLTIP_HEADER_FONT_SIZE * 1.2).ceil(),
        }
    }

    fn tooltip_border_bounds(batch: &QuadBatch) -> (f32, f32, f32, f32) {
        union_bounds(
            batch
                .texture_requests
                .iter()
                .map(|request| request_bounds(batch, request)),
        )
        .expect("tooltip border should emit texture requests")
    }

    fn assert_text_origin_matches_nine_slice_corner(bounds: Rectangle, batch: &QuadBatch) {
        let ns = tooltip_nine_slice().expect("tooltip nine-slice info should exist");
        let tl_corner_bounds = request_bounds_by_base_path(batch, ns.corner_tl.file)
            .expect("tooltip top-left corner texture should emit one request");
        let text_origin = (
            bounds.x + tooltip_text_insets().left,
            bounds.y + tooltip_text_insets().top,
        );

        assert_eq!(
            tl_corner_bounds,
            (
                bounds.x,
                bounds.y,
                bounds.x + ns.corner_tl.width as f32,
                bounds.y + ns.corner_tl.height as f32,
            ),
            "top-left corner texture should match the tooltip's top-left corner coords"
        );
        assert!(
            (text_origin.0 - tl_corner_bounds.2 - (TOOLTIP_PADDING_H - TOOLTIP_CENTER_OVERLAP))
                .abs()
                <= f32::EPSILON,
            "text x origin should be offset from the top-left corner's right edge by padding minus overlap: text_origin={text_origin:?} tl_corner_bounds={tl_corner_bounds:?}"
        );
        assert!(
            (text_origin.1 - tl_corner_bounds.3 - (TOOLTIP_PADDING_V - TOOLTIP_CENTER_OVERLAP))
                .abs()
                <= f32::EPSILON,
            "text y origin should be offset from the top-left corner's bottom edge by padding minus overlap: text_origin={text_origin:?} tl_corner_bounds={tl_corner_bounds:?}"
        );
    }

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
                        left_segments: Vec::new(),
                        right_text: Some("Right".to_string()),
                        right_color: (0.2, 0.3, 0.4),
                        right_segments: Vec::new(),
                        wrap: false,
                        texture: None,
                    },
                    TooltipLine {
                        left_text: "Body".to_string(),
                        left_color: (0.1, 0.2, 0.3),
                        left_segments: Vec::new(),
                        right_text: None,
                        right_color: (0.0, 0.0, 0.0),
                        right_segments: Vec::new(),
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

    #[test]
    fn collect_tooltip_data_preserves_inline_color_segments() {
        let mut state = SimState::default();
        let mut frame = Frame::new(WidgetType::Frame, Some("GameTooltip".to_string()), None);
        frame.id = 42;
        frame.visible = true;
        frame.alpha = 0.5;
        state.widgets.register(frame);
        state.tooltips.insert(
            42,
            TooltipData {
                lines: vec![TooltipLine {
                    left_text: "Plain Hot Plain".to_string(),
                    left_color: (1.0, 0.82, 0.0),
                    left_segments: vec![
                        TooltipTextSegment {
                            text: "Plain ".to_string(),
                            color: (1.0, 0.82, 0.0),
                        },
                        TooltipTextSegment {
                            text: "Hot".to_string(),
                            color: (1.0, 1.0, 1.0),
                        },
                    ],
                    right_text: None,
                    right_color: (1.0, 1.0, 1.0),
                    right_segments: Vec::new(),
                    wrap: false,
                    texture: None,
                }],
                ..TooltipData::default()
            },
        );

        let data = collect_tooltip_data(&state);
        let segments = &data.get(&42).unwrap().lines[0].left_segments;

        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].color, [1.0, 0.82, 0.0, 0.5]);
        assert_eq!(segments[1].color, [1.0, 1.0, 1.0, 0.5]);
    }

    #[test]
    fn tooltip_renderer_emits_inline_color_segments() {
        let data = TooltipRenderData {
            lines: vec![TooltipLineRender {
                left_text: "AB".to_string(),
                left_color: [1.0, 0.82, 0.0, 1.0],
                left_segments: vec![
                    TooltipTextSegmentRender {
                        text: "A".to_string(),
                        color: [1.0, 0.0, 0.0, 1.0],
                    },
                    TooltipTextSegmentRender {
                        text: "B".to_string(),
                        color: [0.0, 1.0, 0.0, 1.0],
                    },
                ],
                right_text: None,
                right_color: [1.0, 1.0, 1.0, 1.0],
                right_segments: Vec::new(),
                font_size: TOOLTIP_HEADER_FONT_SIZE,
                wrap: false,
                measured_height: (TOOLTIP_HEADER_FONT_SIZE * 1.2).ceil(),
            }],
            line_spacing: TOOLTIP_LINE_SPACING,
        };

        let mut batch = QuadBatch::new();
        let mut font_sys = WowFontSystem::new();
        let mut glyph_atlas = GlyphAtlas::new();
        let tooltip_data = HashMap::from([(42_u64, data)]);
        let mut text_ctx = Some((&mut font_sys, &mut glyph_atlas));

        build_tooltip_quads(
            TooltipRender {
                batch: &mut batch,
                bounds: Rectangle::new(Point::new(100.0, 200.0), Size::new(80.0, 47.0)),
                tooltip_data: Some(&tooltip_data),
                id: 42,
                eff_alpha: 1.0,
                draw_background: false,
            },
            &mut text_ctx,
        );

        assert!(has_glyph_color(&batch, [1.0, 0.0, 0.0, 1.0]));
        assert!(has_glyph_color(&batch, [0.0, 1.0, 0.0, 1.0]));
    }

    #[test]
    fn tooltip_renderer_wraps_inline_color_segments() {
        let data = TooltipRenderData {
            lines: vec![TooltipLineRender {
                left_text: "Alpha Bravo Charlie Delta".to_string(),
                left_color: [1.0, 0.82, 0.0, 1.0],
                left_segments: vec![
                    TooltipTextSegmentRender {
                        text: "Alpha Bravo ".to_string(),
                        color: [1.0, 0.0, 0.0, 1.0],
                    },
                    TooltipTextSegmentRender {
                        text: "Charlie Delta".to_string(),
                        color: [0.0, 1.0, 0.0, 1.0],
                    },
                ],
                right_text: None,
                right_color: [1.0, 1.0, 1.0, 1.0],
                right_segments: Vec::new(),
                font_size: TOOLTIP_BODY_FONT_SIZE,
                wrap: true,
                measured_height: (TOOLTIP_BODY_FONT_SIZE * 3.0).ceil(),
            }],
            line_spacing: TOOLTIP_LINE_SPACING,
        };

        let mut batch = QuadBatch::new();
        let mut font_sys = WowFontSystem::new();
        let mut glyph_atlas = GlyphAtlas::new();
        let tooltip_data = HashMap::from([(42_u64, data)]);
        let mut text_ctx = Some((&mut font_sys, &mut glyph_atlas));
        let bounds = Rectangle::new(Point::new(100.0, 200.0), Size::new(95.0, 90.0));

        build_tooltip_quads(
            TooltipRender {
                batch: &mut batch,
                bounds,
                tooltip_data: Some(&tooltip_data),
                id: 42,
                eff_alpha: 1.0,
                draw_background: false,
            },
            &mut text_ctx,
        );

        let insets = tooltip_text_insets();
        let right_edge = bounds.x + bounds.width - insets.right;
        let red_bounds = glyph_bounds_for_color(&batch, [1.0, 0.0, 0.0, 1.0]).unwrap();
        let green_bounds = glyph_bounds_for_color(&batch, [0.0, 1.0, 0.0, 1.0]).unwrap();

        assert!(red_bounds.2 <= right_edge + 1.0);
        assert!(green_bounds.2 <= right_edge + 1.0);
        assert!(
            green_bounds.1 > red_bounds.1,
            "later colored segment should wrap below earlier text, red={red_bounds:?} green={green_bounds:?}"
        );
    }

    #[test]
    fn tooltip_text_insets_account_for_tooltip_nine_slice_overlap() {
        let insets = tooltip_text_insets();
        assert_eq!(
            insets,
            TooltipTextInsets {
                left: 15.0,
                right: 15.0,
                top: 15.0,
                bottom: 15.0,
            }
        );
    }

    #[test]
    fn tooltip_text_quads_start_inside_rendered_border_bounds() {
        let bounds = Rectangle::new(Point::new(100.0, 200.0), Size::new(80.0, 47.0));
        let (batch, border_bounds, glyph_bounds) = render_single_line_tooltip_batch(bounds);

        assert_eq!(
            border_bounds,
            (
                bounds.x,
                bounds.y,
                bounds.x + bounds.width,
                bounds.y + bounds.height
            ),
            "rendered tooltip border should match tooltip bounds"
        );
        assert_text_origin_matches_nine_slice_corner(bounds, &batch);

        let left_inset = glyph_bounds.0 - border_bounds.0;
        let top_inset = glyph_bounds.1 - border_bounds.1;

        assert!(
            left_inset >= 15.0,
            "glyphs should start at or inside the 15px left inset: glyphs={glyph_bounds:?} border={border_bounds:?}"
        );
        assert!(
            top_inset >= 15.0,
            "glyphs should start at or inside the 15px top inset: glyphs={glyph_bounds:?} border={border_bounds:?}"
        );
    }

    #[test]
    fn tooltip_renderer_skips_fallback_background_when_lua_nineslice_exists() {
        let data = TooltipRenderData {
            lines: vec![TooltipLineRender {
                left_text: "Header".to_string(),
                left_color: [1.0, 1.0, 1.0, 1.0],
                left_segments: Vec::new(),
                right_text: None,
                right_color: [1.0, 1.0, 1.0, 1.0],
                right_segments: Vec::new(),
                font_size: TOOLTIP_HEADER_FONT_SIZE,
                wrap: false,
                measured_height: (TOOLTIP_HEADER_FONT_SIZE * 1.2).ceil(),
            }],
            line_spacing: TOOLTIP_LINE_SPACING,
        };

        let mut batch = QuadBatch::new();
        let mut font_sys = WowFontSystem::new();
        let mut glyph_atlas = GlyphAtlas::new();
        let tooltip_data = HashMap::from([(42_u64, data)]);
        let mut text_ctx = Some((&mut font_sys, &mut glyph_atlas));

        build_tooltip_quads(
            TooltipRender {
                batch: &mut batch,
                bounds: Rectangle::new(Point::new(100.0, 200.0), Size::new(80.0, 47.0)),
                tooltip_data: Some(&tooltip_data),
                id: 42,
                eff_alpha: 1.0,
                draw_background: false,
            },
            &mut text_ctx,
        );

        assert!(
            batch.texture_requests.is_empty(),
            "Lua-owned tooltip NineSlice should suppress Rust fallback background requests"
        );
        assert!(
            glyph_bounds(&batch).is_some(),
            "Skipping the fallback background must still render tooltip text"
        );
    }

    #[test]
    fn tooltip_renderer_keeps_opaque_center_when_lua_nineslice_exists() {
        let data = TooltipRenderData {
            lines: vec![TooltipLineRender {
                left_text: "Header".to_string(),
                left_color: [1.0, 1.0, 1.0, 1.0],
                left_segments: Vec::new(),
                right_text: None,
                right_color: [1.0, 1.0, 1.0, 1.0],
                right_segments: Vec::new(),
                font_size: TOOLTIP_HEADER_FONT_SIZE,
                wrap: false,
                measured_height: (TOOLTIP_HEADER_FONT_SIZE * 1.2).ceil(),
            }],
            line_spacing: TOOLTIP_LINE_SPACING,
        };

        let mut batch = QuadBatch::new();
        let mut font_sys = WowFontSystem::new();
        let mut glyph_atlas = GlyphAtlas::new();
        let tooltip_data = HashMap::from([(42_u64, data)]);
        let mut text_ctx = Some((&mut font_sys, &mut glyph_atlas));

        build_tooltip_quads(
            TooltipRender {
                batch: &mut batch,
                bounds: Rectangle::new(Point::new(100.0, 200.0), Size::new(160.0, 80.0)),
                tooltip_data: Some(&tooltip_data),
                id: 42,
                eff_alpha: 1.0,
                draw_background: false,
            },
            &mut text_ctx,
        );

        assert!(
            has_solid_color(&batch, [0.0, 0.0, 0.0, 1.0]),
            "Lua-owned tooltip NineSlice should still get an opaque black center fill"
        );
    }
}
