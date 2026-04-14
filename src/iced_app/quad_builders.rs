//! Widget-specific quad emitters for rendering WoW frames.

use iced::Rectangle;

use crate::render::font::WowFontSystem;
use crate::render::glyph::{GlyphAtlas, emit_text_quads};
use crate::render::shader::GLYPH_ATLAS_TEX_INDEX;
use crate::render::{BlendMode, QuadBatch};
use crate::widget::{TextJustify, WidgetType};

use super::masking::apply_mask_texture;
use super::message_frame_render::{MessageFrameTextRenderer, emit_message_frame_text};
use super::statusbar::StatusBarFill;
use super::tooltip::TooltipRenderData;
#[path = "quad_builders_textures.rs"]
mod textures;

#[path = "quad_builders_cooldown.rs"]
mod cooldown;

#[path = "quad_builders_button.rs"]
mod button;

pub(super) use button::emit_button_highlight;
pub(super) use textures::{build_minimap_quads, build_texture_quads};

/// Build quads for a Frame widget (backdrop).
pub fn build_frame_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    alpha: f32,
) {
    emit_fog_of_war_quads(batch, bounds, f, alpha);

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

fn emit_fog_of_war_quads(
    _batch: &mut QuadBatch,
    _bounds: Rectangle,
    frame: &crate::widget::Frame,
    _alpha: f32,
) {
    if !is_fog_of_war_frame(frame) {
        return;
    }

    // Blizzard fog-of-war is a dedicated background+mask asset model.
    // Do not synthesize coverage from explored overlay chunks; that creates
    // fake geometry and incorrect overlap on maps without real fog data.
}

fn is_fog_of_war_frame(frame: &crate::widget::Frame) -> bool {
    frame
        .object_type_name
        .as_deref()
        .is_some_and(|name| name.eq_ignore_ascii_case("FogOfWarFrame"))
}

pub(super) fn color_with_alpha(c: &crate::widget::Color, alpha: f32) -> [f32; 4] {
    [c.r, c.g, c.b, c.a * alpha]
}

pub(super) struct WidgetTextRenderer<'a> {
    pub(super) batch: &'a mut QuadBatch,
    pub(super) font_sys: &'a mut WowFontSystem,
    pub(super) glyph_atlas: &'a mut GlyphAtlas,
}

pub(super) struct WidgetTextLayout<'a> {
    pub(super) text: &'a str,
    pub(super) bounds: Rectangle,
    pub(super) justify_h: TextJustify,
    pub(super) justify_v: TextJustify,
    pub(super) word_wrap: bool,
    pub(super) max_lines: u32,
    pub(super) alpha: f32,
}

/// Emit text quads for a widget, extracting color/shadow from the frame.
pub(super) fn emit_widget_text_quads(
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
    pub quest_blobs:
        Option<&'a std::collections::HashMap<u64, crate::lua_api::state::QuestBlobState>>,
    pub registry: &'a crate::widget::WidgetRegistry,
    pub elapsed_secs: f64,
    pub eff_alpha: f32,
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
    dispatch_widget_quads(batch, text_ctx, &frame);

    if let Some(clip_bounds) = frame.clip_bounds
        && frame.widget.rotation == 0.0
    {
        clip_recent_quads(batch, vert_before, clip_bounds);
    }

    emit_quest_blob_quads(batch, &frame);
}

fn dispatch_widget_quads(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    frame: &FrameQuadEmit<'_>,
) {
    match frame.widget.widget_type {
        WidgetType::Frame | WidgetType::StatusBar => {
            build_frame_quads(batch, frame.bounds, frame.widget, frame.eff_alpha)
        }
        WidgetType::MessageFrame => emit_message_frame_quads(batch, text_ctx, frame),
        WidgetType::GameTooltip => emit_tooltip_quads(batch, text_ctx, frame),
        WidgetType::Minimap => {
            build_minimap_quads(batch, frame.bounds, frame.widget, frame.eff_alpha)
        }
        WidgetType::Button => button::emit_button_quads_with_text(batch, text_ctx, frame),
        WidgetType::Texture => emit_texture_quads_with_mask(batch, frame),
        WidgetType::FontString => emit_fontstring_quads(batch, text_ctx, frame),
        WidgetType::CheckButton => button::emit_checkbutton_quads(batch, text_ctx, frame),
        WidgetType::EditBox => button::emit_editbox_with_text(
            batch,
            frame.bounds,
            frame.widget,
            text_ctx,
            frame.eff_alpha,
        ),
        WidgetType::Cooldown => cooldown::emit_cooldown_quads(batch, text_ctx, frame),
        WidgetType::Line => super::quad_builders_line::build_line_quads(
            batch,
            frame.widget,
            frame.registry,
            frame.eff_alpha,
        ),
        _ => {}
    }
}

fn emit_tooltip_quads(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    frame: &FrameQuadEmit<'_>,
) {
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
        let text_bounds = button::button_text_bounds(frame);
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

fn emit_quest_blob_quads(batch: &mut QuadBatch, frame: &FrameQuadEmit<'_>) {
    let Some(quest_blobs) = frame.quest_blobs else {
        return;
    };
    let Some(blob_state) = quest_blobs.get(&frame.id) else {
        return;
    };
    if blob_state.active_quests.is_empty() || blob_state.map_id == 0 {
        return;
    }

    let alpha = blob_state.fill_alpha.unwrap_or(1.0) as f32 * frame.eff_alpha;
    if alpha <= 0.0 {
        return;
    }

    for &quest_id in &blob_state.active_quests {
        for blob in crate::quest_poi_blobs::get_quest_blobs_for_map(quest_id, blob_state.map_id) {
            emit_blob_polygon(
                batch,
                frame.bounds,
                blob.vertices,
                blob_state.fill_texture.as_deref(),
                alpha,
            );
        }
    }
}

fn emit_blob_polygon(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    vertices: &[(f32, f32)],
    fill_texture: Option<&str>,
    alpha: f32,
) {
    if vertices.len() < 3 {
        return;
    }

    let color = [1.0, 1.0, 1.0, alpha];
    for i in 1..vertices.len() - 1 {
        let triangle = [vertices[0], vertices[i], vertices[i + 1]];
        let positions =
            triangle.map(|(u, v)| [bounds.x + u * bounds.width, bounds.y + v * bounds.height]);
        let uvs = triangle.map(|(u, v)| [u, v]);
        if let Some(path) = fill_texture {
            batch.push_textured_triangle_path(positions, uvs, path, color, BlendMode::Alpha);
        } else {
            batch.push_solid_triangle(positions, color);
        }
    }
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
