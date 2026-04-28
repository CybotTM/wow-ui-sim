//! Button and EditBox widget quad emitters.

use iced::{Point, Rectangle, Size};

use crate::render::{BlendMode, QuadBatch};
use crate::widget::{TextJustify, WidgetType};

use super::textures::remap_atlas_crop;
use super::{FrameQuadEmit, WidgetTextLayout, WidgetTextRenderer, emit_widget_text_quads};

const BUTTON_TEXT_CHILD_KEYS: [&str; 3] = ["Text", "text", "ButtonText"];

/// Build quads for a Button widget.
pub(super) fn build_button_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    is_pressed: bool,
    is_hovered: bool,
    alpha: f32,
) {
    let has_normal_child = f.children_keys.contains_key("NormalTexture");
    let has_pushed_child = f.children_keys.contains_key("PushedTexture");

    let (texture_path, tex_coords, skip) = if is_pressed {
        (
            f.pushed_texture.as_ref().or(f.normal_texture.as_ref()),
            f.pushed_tex_coords.or(f.normal_tex_coords),
            if f.pushed_texture.is_some() {
                has_pushed_child
            } else {
                has_normal_child
            },
        )
    } else {
        (
            f.normal_texture.as_ref(),
            f.normal_tex_coords,
            has_normal_child,
        )
    };

    if !skip {
        emit_button_texture(batch, bounds, texture_path, tex_coords, alpha);
    }

    let has_highlight_child = f.children_keys.contains_key("HighlightTexture");
    if is_hovered && !is_pressed && !has_highlight_child {
        emit_button_highlight(batch, bounds, f, alpha);
    }
}

/// Render the button's normal/pushed texture (atlas UV or 3-slice).
fn emit_button_texture(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    texture_path: Option<&String>,
    tex_coords: Option<(f32, f32, f32, f32)>,
    alpha: f32,
) {
    let Some(tex_path) = texture_path else { return };
    if let Some((left, right, top, bottom)) = tex_coords {
        let (effective_path, effective_uvs) = remap_atlas_crop(
            tex_path,
            Some((left, right, top, bottom)),
            Some((left, right, top, bottom)),
        );
        let (left, right, top, bottom) = effective_uvs.unwrap_or((0.0, 1.0, 0.0, 1.0));
        let uvs = Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top));
        batch.push_textured_path_uv(
            bounds,
            uvs,
            &effective_path,
            [1.0, 1.0, 1.0, alpha],
            BlendMode::Alpha,
        );
    } else {
        const BUTTON_TEX_WIDTH: f32 = 128.0;
        const BUTTON_CAP_WIDTH: f32 = 4.0;
        batch.push_three_slice_h_path(
            bounds,
            BUTTON_CAP_WIDTH,
            BUTTON_CAP_WIDTH,
            tex_path,
            BUTTON_TEX_WIDTH,
            [1.0, 1.0, 1.0, alpha],
        );
    }
}

/// Render the button highlight overlay on hover.
pub(crate) fn emit_button_highlight(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    alpha: f32,
) {
    if let Some(highlight_path) = &f.highlight_texture {
        if let Some((left, right, top, bottom)) = f.highlight_tex_coords {
            let (effective_path, effective_uvs) = remap_atlas_crop(
                highlight_path,
                Some((left, right, top, bottom)),
                Some((left, right, top, bottom)),
            );
            let (left, right, top, bottom) = effective_uvs.unwrap_or((0.0, 1.0, 0.0, 1.0));
            let uvs = Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top));
            batch.push_textured_path_uv(
                bounds,
                uvs,
                &effective_path,
                [1.0, 1.0, 1.0, 0.5 * alpha],
                BlendMode::Additive,
            );
        } else {
            const BUTTON_TEX_WIDTH: f32 = 128.0;
            const BUTTON_CAP_WIDTH: f32 = 4.0;
            batch.push_three_slice_h_path_blend(
                bounds,
                BUTTON_CAP_WIDTH,
                BUTTON_CAP_WIDTH,
                highlight_path,
                BUTTON_TEX_WIDTH,
                [1.0, 1.0, 1.0, 0.5 * alpha],
                BlendMode::Additive,
            );
        }
    }
}

/// Build quads for an EditBox widget.
pub(super) fn build_editbox_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    alpha: f32,
) {
    if !f.children_keys.is_empty() {
        return;
    }
    batch.push_solid(bounds, [0.06, 0.06, 0.08, 0.9 * alpha]);
    batch.push_border(bounds, 1.0, [0.3, 0.25, 0.15, alpha]);
}

/// Check if a button is visually pressed (mouse or Lua SetButtonState).
pub(super) fn is_button_pressed(
    f: &crate::widget::Frame,
    id: u64,
    pressed_frame: Option<u64>,
) -> bool {
    pressed_frame == Some(id) || f.button_state == 1
}

fn offset_bounds(bounds: Rectangle, offset: (f32, f32)) -> Rectangle {
    Rectangle::new(
        Point::new(bounds.x + offset.0, bounds.y + offset.1),
        Size::new(bounds.width, bounds.height),
    )
}

pub(super) fn pressed_button_text_offset(frame: &FrameQuadEmit<'_>) -> Option<(f32, f32)> {
    match frame.widget.widget_type {
        WidgetType::Button | WidgetType::CheckButton => {
            is_button_pressed(frame.widget, frame.id, frame.pressed_frame)
                .then_some(frame.widget.pushed_text_offset)
        }
        WidgetType::FontString => {
            let parent_id = frame.widget.parent_id?;
            let parent = frame.registry.get(parent_id)?;
            let is_button_text_child = has_button_text_child_id(parent, frame.id);
            (is_button_text_child
                && matches!(
                    parent.widget_type,
                    WidgetType::Button | WidgetType::CheckButton
                ))
            .then_some(parent)
            .filter(|parent| is_button_pressed(parent, parent_id, frame.pressed_frame))
            .map(|parent| parent.pushed_text_offset)
        }
        _ => None,
    }
}

fn has_button_text_child_id(parent: &crate::widget::Frame, child_id: u64) -> bool {
    BUTTON_TEXT_CHILD_KEYS
        .iter()
        .any(|key| parent.children_keys.get(*key).copied() == Some(child_id))
}

fn has_button_text_child(parent: &crate::widget::Frame) -> bool {
    BUTTON_TEXT_CHILD_KEYS
        .iter()
        .any(|key| parent.children_keys.contains_key(*key))
}

pub(super) fn button_text_bounds(frame: &FrameQuadEmit<'_>) -> Rectangle {
    pressed_button_text_offset(frame)
        .map(|offset| offset_bounds(frame.bounds, offset))
        .unwrap_or(frame.bounds)
}

pub(super) fn emit_button_quads_with_text(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(
        &mut crate::render::font::WowFontSystem,
        &mut crate::render::glyph::GlyphAtlas,
    )>,
    frame: &FrameQuadEmit<'_>,
) {
    build_button_quads(
        batch,
        frame.bounds,
        frame.widget,
        is_button_pressed(frame.widget, frame.id, frame.pressed_frame),
        frame.hovered_frame == Some(frame.id),
        frame.eff_alpha,
    );
    if !has_button_text_child(frame.widget)
        && let Some((fs, ga)) = text_ctx
        && let Some(ref txt) = frame.widget.text
    {
        let text_bounds = button_text_bounds(frame);
        let mut text_renderer = WidgetTextRenderer {
            batch,
            font_sys: fs,
            glyph_atlas: ga,
        };
        emit_widget_text_quads(
            &mut text_renderer,
            frame.widget,
            WidgetTextLayout {
                text: txt,
                bounds: text_bounds,
                justify_h: frame.widget.justify_h,
                justify_v: frame.widget.justify_v,
                word_wrap: false,
                max_lines: 0,
                alpha: frame.eff_alpha,
            },
        );
    }
}

pub(super) fn emit_checkbutton_quads(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(
        &mut crate::render::font::WowFontSystem,
        &mut crate::render::glyph::GlyphAtlas,
    )>,
    frame: &FrameQuadEmit<'_>,
) {
    build_button_quads(
        batch,
        frame.bounds,
        frame.widget,
        is_button_pressed(frame.widget, frame.id, frame.pressed_frame),
        frame.hovered_frame == Some(frame.id),
        frame.eff_alpha,
    );
    if let Some((fs, ga)) = text_ctx
        && let Some(ref txt) = frame.widget.text
    {
        let label_bounds = Rectangle::new(
            Point::new(frame.bounds.x + 20.0, frame.bounds.y),
            Size::new(frame.bounds.width - 20.0, frame.bounds.height),
        );
        let mut text_renderer = WidgetTextRenderer {
            batch,
            font_sys: fs,
            glyph_atlas: ga,
        };
        emit_widget_text_quads(
            &mut text_renderer,
            frame.widget,
            WidgetTextLayout {
                text: txt,
                bounds: label_bounds,
                justify_h: TextJustify::Left,
                justify_v: TextJustify::Center,
                word_wrap: false,
                max_lines: 0,
                alpha: frame.eff_alpha,
            },
        );
    }
}

/// EditBox with text insets.
pub(super) fn emit_editbox_with_text(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    text_ctx: &mut Option<(
        &mut crate::render::font::WowFontSystem,
        &mut crate::render::glyph::GlyphAtlas,
    )>,
    alpha: f32,
) {
    build_editbox_quads(batch, bounds, f, alpha);
    let (left_inset, right_inset, top_inset, bottom_inset) = f.editbox_text_insets;
    let left_pad = if left_inset > 0.0 { left_inset } else { 4.0 };
    let right_pad = if right_inset > 0.0 { right_inset } else { 4.0 };
    let text_bounds = Rectangle::new(
        Point::new(bounds.x + left_pad, bounds.y + top_inset),
        Size::new(
            (bounds.width - left_pad - right_pad).max(0.0),
            (bounds.height - top_inset - bottom_inset).max(0.0),
        ),
    );
    let mut text_width = 0.0_f32;
    if let Some((fs, ga)) = text_ctx {
        if let Some(ref txt) = f.text {
            let mut text_renderer = WidgetTextRenderer {
                batch,
                font_sys: fs,
                glyph_atlas: ga,
            };
            emit_widget_text_quads(
                &mut text_renderer,
                f,
                WidgetTextLayout {
                    text: txt,
                    bounds: text_bounds,
                    justify_h: TextJustify::Left,
                    justify_v: TextJustify::Center,
                    word_wrap: false,
                    max_lines: 0,
                    alpha,
                },
            );
            if f.editbox_focused {
                let cursor_chars = f.editbox_cursor_pos.max(0) as usize;
                let measured: String = txt.chars().take(cursor_chars).collect();
                let font_path = f.font.as_deref();
                let font_size = if f.font_size > 0.0 { f.font_size } else { 12.0 };
                text_width = fs.measure_text_width(&measured, font_path, font_size);
            }
        }
    }
    if f.editbox_focused {
        let caret_x = (bounds.x + left_pad + text_width).min(bounds.x + bounds.width - 1.0);
        let caret_top = bounds.y + top_inset + 1.0;
        let caret_height = (bounds.height - top_inset - bottom_inset - 2.0).max(2.0);
        batch.push_solid(
            Rectangle::new(Point::new(caret_x, caret_top), Size::new(1.5, caret_height)),
            [1.0, 0.95, 0.55, 0.95 * alpha],
        );
        batch.push_border(bounds, 1.0, [1.0, 0.85, 0.30, 0.85 * alpha]);
    }
}
