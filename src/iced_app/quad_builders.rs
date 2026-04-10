//! Widget-specific quad emitters for rendering WoW frames.

use iced::{Point, Rectangle, Size};

use crate::render::font::WowFontSystem;
use crate::render::glyph::{GlyphAtlas, emit_text_quads};
use crate::render::shader::GLYPH_ATLAS_TEX_INDEX;
use crate::render::{BlendMode, QuadBatch};
use crate::widget::{TextJustify, WidgetType};

use super::masking::apply_mask_texture;
use super::message_frame_render::{MessageFrameTextRenderer, emit_message_frame_text};
use super::statusbar::StatusBarFill;
use super::tooltip::TooltipRenderData;
use textures::remap_atlas_crop;

#[path = "quad_builders_textures.rs"]
mod textures;

pub(super) use textures::{build_minimap_quads, build_texture_quads};

/// Build quads for a Frame widget (backdrop).
pub fn build_frame_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    alpha: f32,
) {
    if f.backdrop.enabled {
        let bg = &f.backdrop.bg_color;
        batch.push_solid(bounds, [bg.r, bg.g, bg.b, bg.a * alpha]);

        if f.backdrop.edge_size > 0.0 {
            let bc = &f.backdrop.border_color;
            batch.push_border(
                bounds,
                f.backdrop.edge_size.max(1.0),
                [bc.r, bc.g, bc.b, bc.a * alpha],
            );
        }
    }

    if f.nine_slice_layout.is_some() {
        batch.push_border(bounds, 2.0, [0.6, 0.45, 0.15, alpha]);
    }
}

/// Build quads for a Button widget.
pub fn build_button_quads(
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
pub(super) fn emit_button_highlight(
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
pub fn build_editbox_quads(
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

/// Emit text quads for a widget, extracting color/shadow from the frame.
fn emit_widget_text_quads(
    text_renderer: &mut WidgetTextRenderer<'_>,
    f: &crate::widget::Frame,
    layout: WidgetTextLayout<'_>,
) {
    let color = color_with_alpha(&f.text_color, layout.alpha);
    let shadow = (f.shadow_color.a > 0.0).then(|| color_with_alpha(&f.shadow_color, layout.alpha));
    emit_text_quads(
        text_renderer.batch,
        text_renderer.font_sys,
        text_renderer.glyph_atlas,
        layout.text,
        layout.bounds,
        f.font.as_deref(),
        f.font_size * f.effective_scale,
        color,
        layout.justify_h,
        layout.justify_v,
        GLYPH_ATLAS_TEX_INDEX,
        shadow,
        f.shadow_offset,
        f.font_outline,
        layout.word_wrap,
        layout.max_lines,
        f.text_stripped.as_deref(),
    );
}

fn color_with_alpha(c: &crate::widget::Color, alpha: f32) -> [f32; 4] {
    [c.r, c.g, c.b, c.a * alpha]
}

struct WidgetTextRenderer<'a> {
    batch: &'a mut QuadBatch,
    font_sys: &'a mut WowFontSystem,
    glyph_atlas: &'a mut GlyphAtlas,
}

struct WidgetTextLayout<'a> {
    text: &'a str,
    bounds: Rectangle,
    justify_h: TextJustify,
    justify_v: TextJustify,
    word_wrap: bool,
    max_lines: u32,
    alpha: f32,
}

/// Check if a button is visually pressed (mouse or Lua SetButtonState).
fn is_button_pressed(f: &crate::widget::Frame, id: u64, pressed_frame: Option<u64>) -> bool {
    pressed_frame == Some(id) || f.button_state == 1
}

fn offset_bounds(bounds: Rectangle, offset: (f32, f32)) -> Rectangle {
    Rectangle::new(
        Point::new(bounds.x + offset.0, bounds.y + offset.1),
        Size::new(bounds.width, bounds.height),
    )
}

fn pressed_button_text_offset(frame: &FrameQuadEmit<'_>) -> Option<(f32, f32)> {
    match frame.widget.widget_type {
        WidgetType::Button | WidgetType::CheckButton => {
            is_button_pressed(frame.widget, frame.id, frame.pressed_frame)
                .then_some(frame.widget.pushed_text_offset)
        }
        WidgetType::FontString => {
            let parent_id = frame.widget.parent_id?;
            let parent = frame.registry.get(parent_id)?;
            let is_button_text_child = parent.children_keys.get("Text").copied() == Some(frame.id);
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

fn button_text_bounds(frame: &FrameQuadEmit<'_>) -> Rectangle {
    pressed_button_text_offset(frame)
        .map(|offset| offset_bounds(frame.bounds, offset))
        .unwrap_or(frame.bounds)
}

/// Emit quads for a single visible frame based on its widget type.
///
/// `eff_alpha` is the effective alpha from the ancestor chain (`parent_alpha * f.alpha`),
/// matching WoW's `GetEffectiveAlpha()` behavior where parent alpha dims all descendants.
pub fn emit_frame_quads(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    frame: FrameQuadEmit<'_>,
) {
    let vert_before = batch.vertices.len();
    match frame.widget.widget_type {
        WidgetType::Frame | WidgetType::StatusBar => {
            build_frame_quads(batch, frame.bounds, frame.widget, frame.eff_alpha)
        }
        WidgetType::MessageFrame => emit_message_frame_quads(batch, text_ctx, &frame),
        WidgetType::GameTooltip => {
            super::tooltip::build_tooltip_quads(
                super::tooltip::TooltipRender {
                    batch,
                    bounds: frame.bounds,
                    tooltip_data: frame.tooltip_data,
                    id: frame.id,
                    eff_alpha: frame.eff_alpha,
                },
                text_ctx,
            );
        }
        WidgetType::Minimap => {
            build_minimap_quads(batch, frame.bounds, frame.widget, frame.eff_alpha)
        }
        WidgetType::Button => emit_button_quads_with_text(batch, text_ctx, &frame),
        WidgetType::Texture => emit_texture_quads_with_mask(batch, &frame),
        WidgetType::FontString => emit_fontstring_quads(batch, text_ctx, &frame),
        WidgetType::CheckButton => emit_checkbutton_quads(batch, text_ctx, &frame),
        WidgetType::EditBox => {
            emit_editbox_with_text(batch, frame.bounds, frame.widget, text_ctx, frame.eff_alpha);
        }
        WidgetType::Cooldown => {
            build_cooldown_quads(batch, frame.bounds, frame.widget, frame.elapsed_secs);
        }
        WidgetType::Line => {
            super::quad_builders_line::build_line_quads(
                batch,
                frame.widget,
                frame.registry,
                frame.eff_alpha,
            );
        }
        _ => {}
    }

    if let Some(clip_bounds) = frame.clip_bounds
        && frame.widget.rotation == 0.0
    {
        clip_recent_quads(batch, vert_before, clip_bounds);
    }
}

fn emit_message_frame_quads(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    frame: &FrameQuadEmit<'_>,
) {
    build_frame_quads(batch, frame.bounds, frame.widget, frame.eff_alpha);
    if let Some((fs, ga)) = text_ctx
        && let Some(mf_map) = frame.message_frames
    {
        let mut render = MessageFrameTextRenderer {
            batch,
            font_sys: fs,
            glyph_atlas: ga,
        };
        emit_message_frame_text(
            &mut render,
            frame.widget,
            frame.id,
            frame.bounds,
            mf_map,
            frame.eff_alpha,
            frame.elapsed_secs,
        );
    }
}

fn emit_button_quads_with_text(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
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
    if !frame.widget.children_keys.contains_key("Text")
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

fn emit_texture_quads_with_mask(batch: &mut QuadBatch, frame: &FrameQuadEmit<'_>) {
    if frame.widget.is_mask {
        return;
    }
    let vert_before = batch.vertices.len();
    build_texture_quads(
        batch,
        frame.bounds,
        frame.widget,
        frame.bar_fill,
        frame.eff_alpha,
    );
    if !frame.widget.mask_textures.is_empty() {
        apply_mask_texture(
            batch,
            vert_before,
            frame.bounds,
            &frame.widget.mask_textures,
            frame.registry,
        );
    }
}

fn emit_fontstring_quads(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    frame: &FrameQuadEmit<'_>,
) {
    if let Some((fs, ga)) = text_ctx
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
                word_wrap: frame.widget.word_wrap,
                max_lines: frame.widget.max_lines,
                alpha: frame.eff_alpha,
            },
        );
    }
}

fn emit_checkbutton_quads(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
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

pub struct FrameQuadEmit<'a> {
    pub id: u64,
    pub widget: &'a crate::widget::Frame,
    pub bounds: Rectangle,
    pub clip_bounds: Option<Rectangle>,
    pub bar_fill: Option<&'a StatusBarFill>,
    pub pressed_frame: Option<u64>,
    pub hovered_frame: Option<u64>,
    pub message_frames:
        Option<&'a std::collections::HashMap<u64, crate::lua_api::message_frame::MessageFrameData>>,
    pub tooltip_data: Option<&'a std::collections::HashMap<u64, TooltipRenderData>>,
    pub registry: &'a crate::widget::WidgetRegistry,
    pub elapsed_secs: f64,
    pub eff_alpha: f32,
}

fn clip_recent_quads(batch: &mut QuadBatch, vert_before: usize, clip: Rectangle) {
    for chunk in batch.vertices[vert_before..].chunks_exact_mut(4) {
        let original = [chunk[0], chunk[1], chunk[2], chunk[3]];
        let left = chunk[0].position[0].min(chunk[3].position[0]);
        let right = chunk[1].position[0].max(chunk[2].position[0]);
        let top = chunk[0].position[1].min(chunk[1].position[1]);
        let bottom = chunk[2].position[1].max(chunk[3].position[1]);

        let clipped_left = left.max(clip.x);
        let clipped_right = right.min(clip.x + clip.width);
        let clipped_top = top.max(clip.y);
        let clipped_bottom = bottom.min(clip.y + clip.height);

        if clipped_left >= clipped_right || clipped_top >= clipped_bottom {
            for vertex in chunk.iter_mut() {
                vertex.color[3] = 0.0;
            }
            continue;
        }

        let x0 = inv_lerp(left, right, clipped_left);
        let x1 = inv_lerp(left, right, clipped_right);
        let y0 = inv_lerp(top, bottom, clipped_top);
        let y1 = inv_lerp(top, bottom, clipped_bottom);

        clip_vertex(chunk, &original, 0, clipped_left, clipped_top, x0, y0);
        clip_vertex(chunk, &original, 1, clipped_right, clipped_top, x1, y0);
        clip_vertex(chunk, &original, 2, clipped_right, clipped_bottom, x1, y1);
        clip_vertex(chunk, &original, 3, clipped_left, clipped_bottom, x0, y1);
    }
}

fn clip_vertex(
    chunk: &mut [crate::render::QuadVertex],
    original: &[crate::render::QuadVertex; 4],
    index: usize,
    x: f32,
    y: f32,
    tx: f32,
    ty: f32,
) {
    const FLAG_COOLDOWN_SWIPE: u32 = 0x200;
    chunk[index].position = [x, y];
    if (chunk[index].flags & FLAG_COOLDOWN_SWIPE) == 0 {
        chunk[index].tex_coords = [
            lerp(original[0].tex_coords[0], original[1].tex_coords[0], tx),
            lerp(original[0].tex_coords[1], original[3].tex_coords[1], ty),
        ];
        chunk[index].local_uv = [
            lerp(original[0].local_uv[0], original[1].local_uv[0], tx),
            lerp(original[0].local_uv[1], original[3].local_uv[1], ty),
        ];
        if original[index].mask_tex_index != -1 || original[index].mask_tex_coords != [0.0, 0.0] {
            chunk[index].mask_tex_coords = [
                lerp(
                    original[0].mask_tex_coords[0],
                    original[1].mask_tex_coords[0],
                    tx,
                ),
                lerp(
                    original[0].mask_tex_coords[1],
                    original[3].mask_tex_coords[1],
                    ty,
                ),
            ];
        }
    }
}

fn inv_lerp(a: f32, b: f32, v: f32) -> f32 {
    let denom = b - a;
    if denom.abs() < f32::EPSILON {
        0.0
    } else {
        ((v - a) / denom).clamp(0.0, 1.0)
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// EditBox with text insets.
fn emit_editbox_with_text(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    alpha: f32,
) {
    build_editbox_quads(batch, bounds, f, alpha);
    if let Some((fs, ga)) = text_ctx
        && let Some(ref txt) = f.text
    {
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
    }
}

/// Build a cooldown swipe overlay quad.
fn build_cooldown_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    elapsed_secs: f64,
) {
    if !f.cooldown_draw_swipe || f.cooldown_duration <= 0.0 {
        return;
    }
    let elapsed_since_start = elapsed_secs - f.cooldown_start;
    let progress = (elapsed_since_start / f.cooldown_duration).clamp(0.0, 1.0);
    if progress >= 1.0 {
        return; // Cooldown finished, no overlay
    }
    let swipe_progress = if f.cooldown_reverse {
        1.0 - progress
    } else {
        progress
    } as f32;
    let color = parse_swipe_color(f);
    batch.push_cooldown_swipe(bounds, swipe_progress, color, f.cooldown_tex_coord_range);
}

/// Parse the swipe color from the frame's `__swipe_color` attribute, or return default.
fn parse_swipe_color(f: &crate::widget::Frame) -> [f32; 4] {
    use crate::widget::AttributeValue;
    if let Some(AttributeValue::String(s)) = f.attributes.get("__swipe_color") {
        let parts: Vec<f32> = s.split(',').filter_map(|p| p.trim().parse().ok()).collect();
        if parts.len() == 4 {
            return [parts[0], parts[1], parts[2], parts[3]];
        }
    }
    [0.0, 0.0, 0.0, 0.62] // WoW default: semi-transparent black
}

#[cfg(test)]
mod tests {
    use super::build_cooldown_quads;
    use crate::render::QuadBatch;
    use crate::widget::{Frame, WidgetType};
    use iced::{Point, Rectangle, Size};

    #[test]
    fn cooldown_swipe_uses_configured_texcoord_range_for_local_uvs() {
        let mut batch = QuadBatch::new();
        let mut cooldown = Frame::new(WidgetType::Cooldown, None, None);
        cooldown.cooldown_duration = 10.0;
        cooldown.cooldown_start = 0.0;
        cooldown.cooldown_tex_coord_range = Some((0.2, 0.3, 0.8, 0.9));

        build_cooldown_quads(
            &mut batch,
            Rectangle::new(Point::new(10.0, 20.0), Size::new(40.0, 50.0)),
            &cooldown,
            2.5,
        );

        assert_eq!(
            batch.vertices.len(),
            4,
            "cooldown should emit one swipe quad"
        );
        assert_eq!(batch.vertices[0].local_uv, [0.2, 0.3]);
        assert_eq!(batch.vertices[1].local_uv, [0.8, 0.3]);
        assert_eq!(batch.vertices[2].local_uv, [0.8, 0.9]);
        assert_eq!(batch.vertices[3].local_uv, [0.2, 0.9]);
    }
}
