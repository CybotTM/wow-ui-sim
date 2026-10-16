//! Content-relative EditBox caret geometry from the renderer's shaping inputs.

use cosmic_text::{Buffer, Cursor, Edit, Editor, LayoutRun, Metrics, Shaping};

use super::font::{WowFontSystem, line_height_for_font_size, spaced_text_height};
use crate::widget::TextJustify;

pub(crate) struct EditBoxLayoutRequest<'a> {
    /// Raw logical text, before WoW markup processing.
    pub(crate) text: &'a str,
    pub(crate) font_path: Option<&'a str>,
    /// Actual pixel size; no additional UI scale is applied here.
    pub(crate) font_size: f32,
    pub(crate) content_width: f32,
    pub(crate) content_height: f32,
    pub(crate) word_wrap: bool,
    pub(crate) line_spacing: f32,
    pub(crate) smooth_scaling: bool,
    pub(crate) justify_h: TextJustify,
    pub(crate) justify_v: TextJustify,
    pub(crate) cursor_scalar: usize,
}

pub(crate) struct PreparedEditBoxLayout {
    pub buffer: Buffer,
    /// Content-relative x/top/height pixels, before viewport clipping.
    pub cursor: Option<(f32, f32, f32)>,
}

/// Prepare one authoritative full-content layout for caret and glyph consumers.
pub(crate) fn prepare_editbox_layout(
    fonts: &mut WowFontSystem,
    request: &EditBoxLayoutRequest<'_>,
) -> Option<PreparedEditBoxLayout> {
    let line_height = valid_line_height(fonts, request)?;
    let visible = super::strip_wow_markup(request.text);
    let mut buffer = shape_content(fonts, request, &visible, line_height);
    let cursor = read_cursor_geometry(&mut buffer, request, &visible, line_height);
    Some(PreparedEditBoxLayout { buffer, cursor })
}

fn read_cursor_geometry(
    buffer: &mut Buffer,
    request: &EditBoxLayoutRequest<'_>,
    visible: &str,
    line_height: f32,
) -> Option<(f32, f32, f32)> {
    let prefix_bytes = visible_cursor_prefix_bytes(request, visible)?;
    let cursor = cursor_at_visible_byte(buffer, prefix_bytes)?;
    let mut editor = Editor::new(buffer);
    editor.set_cursor(cursor);
    let position = editor.cursor_position()?;
    editor.with_buffer(|buffer| positioned_caret(buffer, request, cursor, position, line_height))
}

fn valid_line_height(fonts: &WowFontSystem, request: &EditBoxLayoutRequest<'_>) -> Option<f32> {
    if fonts.font_system.db().faces().next().is_none() {
        return None;
    }
    let line_height = line_height_for_font_size(request.font_size, request.smooth_scaling)?;
    let dimensions = [
        line_height,
        request.content_width,
        request.content_height,
        request.line_spacing,
    ];
    if line_height <= 0.0 || dimensions.iter().any(|value| !value.is_finite()) {
        return None;
    }
    Some(line_height)
}

fn visible_cursor_prefix_bytes(request: &EditBoxLayoutRequest<'_>, visible: &str) -> Option<usize> {
    let raw_boundary = request
        .text
        .char_indices()
        .nth(request.cursor_scalar)
        .map_or(request.text.len(), |(byte, _)| byte);
    let prefix = super::strip_wow_markup(&request.text[..raw_boundary]);
    visible.starts_with(&prefix).then_some(prefix.len())
}

fn shape_content(
    fonts: &mut WowFontSystem,
    request: &EditBoxLayoutRequest<'_>,
    visible: &str,
    line_height: f32,
) -> Buffer {
    let width = if request.word_wrap && request.content_width > 0.0 {
        request.content_width
    } else {
        10000.0 // Same shaping width as glyph.rs, including its default wrap mode.
    };
    let attrs = fonts.attrs_owned(request.font_path);
    let metrics = Metrics::new(request.font_size, line_height);
    let mut buffer = Buffer::new(&mut fonts.font_system, metrics);
    buffer.set_size(&mut fonts.font_system, Some(width), None);
    buffer.set_text(
        &mut fonts.font_system,
        visible,
        &attrs.as_attrs(),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(&mut fonts.font_system, true);
    buffer
}

fn cursor_at_visible_byte(buffer: &Buffer, mut byte: usize) -> Option<Cursor> {
    for (line_index, line) in buffer.lines.iter().enumerate() {
        let text_length = line.text().len();
        let ending_length = line.ending().as_str().len();
        if byte <= text_length {
            return Some(Cursor::new(line_index, byte));
        }
        byte = byte.checked_sub(text_length + ending_length)?;
    }
    None
}

fn positioned_caret(
    buffer: &Buffer,
    request: &EditBoxLayoutRequest<'_>,
    cursor: Cursor,
    position: (i32, i32),
    line_height: f32,
) -> Option<(f32, f32, f32)> {
    let runs: Vec<_> = buffer.layout_runs().collect();
    let (run_index, run) = runs
        .iter()
        .enumerate()
        .find(|(_, run)| run.line_i == cursor.line && run.line_top as i32 == position.1)?;
    let total_height = glyph_total_height(&runs, line_height, request.line_spacing)?;
    let horizontal = horizontal_offset(request, run.line_w);
    let vertical = justify_offset(request.justify_v, request.content_height - total_height);
    let x = position.0 as f32 + horizontal;
    let top = position.1 as f32 + run_index as f32 * request.line_spacing + vertical;
    let height = run.line_height;
    let geometry = [x, top, height];
    if height <= 0.0 || geometry.iter().any(|value| !value.is_finite()) {
        return None;
    }
    Some((x, top, height))
}

fn glyph_total_height(runs: &[LayoutRun<'_>], line_height: f32, spacing: f32) -> Option<f32> {
    let first = runs.first()?;
    let last = runs.last()?;
    // Exact glyph.rs baseline-span formula, not font.rs's legacy false-mode
    // first-baseline measurement contribution.
    let base_height = if runs.len() <= 1 {
        line_height
    } else {
        last.line_y - first.line_y + line_height
    };
    Some(spaced_text_height(base_height, runs.len(), spacing))
}

fn horizontal_offset(request: &EditBoxLayoutRequest<'_>, run_width: f32) -> f32 {
    if request.content_width <= 0.0 {
        return 0.0;
    }
    justify_offset(request.justify_h, request.content_width - run_width)
}

fn justify_offset(justify: TextJustify, remaining: f32) -> f32 {
    match justify {
        TextJustify::Left => 0.0,
        TextJustify::Center => remaining / 2.0,
        TextJustify::Right => remaining,
    }
}
