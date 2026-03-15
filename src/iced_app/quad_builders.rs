//! Widget-specific quad emitters for rendering WoW frames.

use iced::{Point, Rectangle, Size};

use crate::render::font::WowFontSystem;
use crate::render::glyph::{GlyphAtlas, emit_text_quads};
use crate::render::shader::GLYPH_ATLAS_TEX_INDEX;
use crate::render::{BlendMode, QuadBatch};
use crate::widget::{TextJustify, WidgetType};

use super::masking::apply_mask_texture;
use super::message_frame_render::emit_message_frame_text;
use super::statusbar::StatusBarFill;
use super::tiling::emit_tiled_texture;
use super::tooltip::TooltipRenderData;

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
        let uvs = Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top));
        batch.push_textured_path_uv(
            bounds,
            uvs,
            tex_path,
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
            let uvs = Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top));
            batch.push_textured_path_uv(
                bounds,
                uvs,
                highlight_path,
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

/// Build quads for a Texture widget, optionally clipped by a StatusBar fill.
pub fn build_texture_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    bar_fill: Option<&StatusBarFill>,
    alpha: f32,
) {
    if let Some(ref ns) = f.nine_slice_atlas {
        super::nine_slice::emit_nine_slice_atlas(batch, bounds, ns, alpha);
        return;
    }

    let tint = resolve_tint(f, bar_fill, alpha);

    if let Some(color) = f.color_texture {
        let fill_bounds = apply_bar_fill(bounds, bar_fill);
        if let Some(ref grad) = f.gradient {
            push_gradient_quad(batch, fill_bounds, grad, alpha);
        } else {
            batch.push_solid(
                fill_bounds,
                [
                    color.r * tint[0],
                    color.g * tint[1],
                    color.b * tint[2],
                    color.a * alpha,
                ],
            );
        }
        return;
    }

    let Some(tex_path) = &f.texture else {
        emit_bar_fill_fallback(batch, bar_fill, bounds, alpha);
        return;
    };
    emit_textured_quad(batch, bounds, f, bar_fill, tex_path, tint, alpha);
}

/// Compute the vertex color tint from vertex_color and bar fill override.
fn resolve_tint(
    f: &crate::widget::Frame,
    bar_fill: Option<&StatusBarFill>,
    alpha: f32,
) -> [f32; 4] {
    if let Some(fill) = bar_fill
        && let Some(c) = &fill.color
    {
        return [c.r, c.g, c.b, c.a * alpha];
    }
    let vc = f.vertex_color.as_ref();
    [
        vc.map_or(1.0, |c| c.r),
        vc.map_or(1.0, |c| c.g),
        vc.map_or(1.0, |c| c.b),
        vc.map_or(1.0, |c| c.a) * alpha,
    ]
}

/// Emit a solid color quad when no texture path exists but a bar fill has a color.
fn emit_bar_fill_fallback(
    batch: &mut QuadBatch,
    bar_fill: Option<&StatusBarFill>,
    bounds: Rectangle,
    alpha: f32,
) {
    if let Some(fill) = bar_fill
        && let Some(c) = &fill.color
    {
        let fill_bounds = apply_bar_fill(bounds, bar_fill);
        batch.push_solid(fill_bounds, [c.r, c.g, c.b, c.a * alpha]);
    }
}

/// Emit a gradient quad with per-vertex colors (VERTICAL or HORIZONTAL).
fn push_gradient_quad(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    grad: &crate::widget::Gradient,
    alpha: f32,
) {
    let min = &grad.min_color;
    let max = &grad.max_color;
    let (top_color, bottom_color) = if grad.vertical {
        // VERTICAL: max at top, min at bottom
        (
            [max.r, max.g, max.b, max.a * alpha],
            [min.r, min.g, min.b, min.a * alpha],
        )
    } else {
        // HORIZONTAL: will use left/right instead; top=bottom=white, handled per-vertex below
        (
            [min.r, min.g, min.b, min.a * alpha],
            [min.r, min.g, min.b, min.a * alpha],
        )
    };
    let colors = if grad.vertical {
        [top_color, top_color, bottom_color, bottom_color] // TL, TR, BR, BL
    } else {
        let right = [max.r, max.g, max.b, max.a * alpha];
        let left = [min.r, min.g, min.b, min.a * alpha];
        [left, right, right, left] // TL, TR, BR, BL
    };
    batch.push_gradient(bounds, colors);
}

/// Emit a textured quad with atlas cropping, three-slice, tiling, rotation, desaturation.
fn emit_textured_quad(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    bar_fill: Option<&StatusBarFill>,
    tex_path: &str,
    tint: [f32; 4],
    alpha: f32,
) {
    let (fill_bounds, fill_uvs) = apply_bar_fill_with_uvs(bounds, f.tex_coords, bar_fill);
    let (effective_path, effective_uvs) = remap_atlas_crop(tex_path, fill_uvs, f.atlas_tex_coords);
    let vert_before = batch.vertices.len();

    if let Some((left_cap, right_cap, atlas_w)) = f.three_slice_h
        && let Some((left, right, top, bottom)) = effective_uvs
        && fill_bounds.width > left_cap + right_cap
    {
        emit_three_slice_h_atlas(
            batch,
            fill_bounds,
            left_cap,
            right_cap,
            atlas_w,
            (left, right, top, bottom),
            &effective_path,
            tint,
            f.blend_mode,
        );
    } else if let Some((left, right, top, bottom)) = effective_uvs {
        let uvs = Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top));
        if f.horiz_tile || f.vert_tile {
            emit_tiled_texture(batch, fill_bounds, &uvs, &effective_path, f, alpha);
        } else {
            batch.push_textured_path_uv(fill_bounds, uvs, &effective_path, tint, f.blend_mode);
        }
    } else {
        batch.push_textured_path(fill_bounds, &effective_path, tint, f.blend_mode);
    }

    if f.rotation != 0.0 {
        apply_uv_rotation(batch, vert_before, f.rotation);
    }
    if f.desaturated {
        apply_desaturate_flag(batch, vert_before);
    }
}

/// Render an atlas texture as 3 horizontal slices (left cap, stretched middle, right cap).
#[allow(clippy::too_many_arguments)]
fn emit_three_slice_h_atlas(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    left_cap_px: f32,
    right_cap_px: f32,
    atlas_width_px: f32,
    (left_uv, right_uv, top_uv, bottom_uv): (f32, f32, f32, f32),
    tex_path: &str,
    tint: [f32; 4],
    blend: crate::render::BlendMode,
) {
    let uv_w = right_uv - left_uv;
    let uv_h = bottom_uv - top_uv;
    let left_frac = left_cap_px / atlas_width_px;
    let right_frac = right_cap_px / atlas_width_px;

    let left_cap_uv_end = left_uv + left_frac * uv_w;
    let right_cap_uv_start = right_uv - right_frac * uv_w;

    let mid_x = bounds.x + left_cap_px;
    let mid_w = bounds.width - left_cap_px - right_cap_px;
    let right_x = bounds.x + bounds.width - right_cap_px;

    // Left cap
    batch.push_textured_path_uv(
        Rectangle::new(
            Point::new(bounds.x, bounds.y),
            Size::new(left_cap_px, bounds.height),
        ),
        Rectangle::new(
            Point::new(left_uv, top_uv),
            Size::new(left_cap_uv_end - left_uv, uv_h),
        ),
        tex_path,
        tint,
        blend,
    );
    // Middle (stretched)
    batch.push_textured_path_uv(
        Rectangle::new(Point::new(mid_x, bounds.y), Size::new(mid_w, bounds.height)),
        Rectangle::new(
            Point::new(left_cap_uv_end, top_uv),
            Size::new(right_cap_uv_start - left_cap_uv_end, uv_h),
        ),
        tex_path,
        tint,
        blend,
    );
    // Right cap
    batch.push_textured_path_uv(
        Rectangle::new(
            Point::new(right_x, bounds.y),
            Size::new(right_cap_px, bounds.height),
        ),
        Rectangle::new(
            Point::new(right_cap_uv_start, top_uv),
            Size::new(right_uv - right_cap_uv_start, uv_h),
        ),
        tex_path,
        tint,
        blend,
    );
}

/// Apply StatusBar fill clipping to bounds.
fn apply_bar_fill(bounds: Rectangle, bar_fill: Option<&StatusBarFill>) -> Rectangle {
    let Some(fill) = bar_fill else { return bounds };
    let fill_width = bounds.width * fill.fraction;
    if fill.reverse {
        Rectangle::new(
            Point::new(bounds.x + bounds.width - fill_width, bounds.y),
            Size::new(fill_width, bounds.height),
        )
    } else {
        Rectangle::new(bounds.position(), Size::new(fill_width, bounds.height))
    }
}

/// Remap atlas sub-region textures: encode crop coords in path, remap UVs to [0,1].
///
/// When a frame has `atlas_tex_coords` (from SetAtlas), the source texture may be very large
/// (e.g. 2048x1024 `talents.blp`). Uploading the full texture to the GPU atlas would downscale
/// it to 512x512, making small sub-regions (40x40) extremely pixelated (~12px after downscale).
///
/// Instead, encode the crop region in the texture path so `load_new_textures` extracts just
/// the sub-region at native resolution. The cropped region (e.g. 50x50) fits in a small GPU
/// atlas tier without any downscaling.
fn remap_atlas_crop(
    tex_path: &str,
    fill_uvs: Option<(f32, f32, f32, f32)>,
    atlas_tex_coords: Option<(f32, f32, f32, f32)>,
) -> (String, Option<(f32, f32, f32, f32)>) {
    let Some((cl, cr, ct, cb)) = atlas_tex_coords else {
        return (tex_path.to_string(), fill_uvs);
    };
    // Skip cropping if atlas covers the full texture (no sub-region)
    let is_full = (cl - 0.0).abs() < 0.001
        && (cr - 1.0).abs() < 0.001
        && (ct - 0.0).abs() < 0.001
        && (cb - 1.0).abs() < 0.001;
    if is_full {
        return (tex_path.to_string(), fill_uvs);
    }

    // Encode crop region in path for texture loader to extract
    let crop_key = format!("{}@crop:{:.6},{:.6},{:.6},{:.6}", tex_path, cl, cr, ct, cb,);

    // Remap vertex UVs from source-texture-space to crop-region-space [0,1]
    let remapped_uvs = fill_uvs.map(|(fl, fr, ft, fb)| {
        let cw = cr - cl;
        let ch = cb - ct;
        if cw <= 0.0 || ch <= 0.0 {
            return (0.0, 1.0, 0.0, 1.0);
        }
        (
            (fl - cl) / cw,
            (fr - cl) / cw,
            (ft - ct) / ch,
            (fb - ct) / ch,
        )
    });

    (crop_key, remapped_uvs)
}

/// Apply StatusBar fill clipping to bounds and UV coordinates.
fn apply_bar_fill_with_uvs(
    bounds: Rectangle,
    tex_coords: Option<(f32, f32, f32, f32)>,
    bar_fill: Option<&StatusBarFill>,
) -> (Rectangle, Option<(f32, f32, f32, f32)>) {
    let Some(fill) = bar_fill else {
        return (bounds, tex_coords);
    };
    let fill_bounds = apply_bar_fill(bounds, bar_fill);
    let (uv_left, uv_right, uv_top, uv_bottom) = tex_coords.unwrap_or((0.0, 1.0, 0.0, 1.0));
    let uv_range = uv_right - uv_left;
    let fill_uvs = if fill.reverse {
        (
            uv_left + uv_range * (1.0 - fill.fraction),
            uv_right,
            uv_top,
            uv_bottom,
        )
    } else {
        (
            uv_left,
            uv_left + uv_range * fill.fraction,
            uv_top,
            uv_bottom,
        )
    };
    (fill_bounds, Some(fill_uvs))
}

/// Rotate texture UV coordinates around their center for vertices added after `vert_before`.
fn apply_uv_rotation(batch: &mut QuadBatch, vert_before: usize, radians: f32) {
    let verts = &mut batch.vertices[vert_before..];
    if verts.len() < 4 {
        return;
    }
    let (sin_r, cos_r) = radians.sin_cos();
    for chunk in verts.chunks_exact_mut(4) {
        let cx = (chunk[0].tex_coords[0]
            + chunk[1].tex_coords[0]
            + chunk[2].tex_coords[0]
            + chunk[3].tex_coords[0])
            * 0.25;
        let cy = (chunk[0].tex_coords[1]
            + chunk[1].tex_coords[1]
            + chunk[2].tex_coords[1]
            + chunk[3].tex_coords[1])
            * 0.25;
        for v in chunk.iter_mut() {
            let du = v.tex_coords[0] - cx;
            let dv = v.tex_coords[1] - cy;
            v.tex_coords[0] = cx + du * cos_r - dv * sin_r;
            v.tex_coords[1] = cy + du * sin_r + dv * cos_r;
        }
    }
}

/// Apply the desaturation flag to vertices added after `vert_before`.
fn apply_desaturate_flag(batch: &mut QuadBatch, vert_before: usize) {
    use crate::render::shader::FLAG_DESATURATE;
    for v in &mut batch.vertices[vert_before..] {
        v.flags |= FLAG_DESATURATE;
    }
}

/// Build quads for a Minimap widget - map texture clipped to a circle.
pub fn build_minimap_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    _f: &crate::widget::Frame,
    alpha: f32,
) {
    use crate::render::shader::FLAG_CIRCLE_CLIP;
    batch.push_textured_path(
        bounds,
        r"Interface\Minimap\placeholder-map",
        [1.0, 1.0, 1.0, alpha],
        BlendMode::Alpha,
    );
    batch.set_extra_flags(4, FLAG_CIRCLE_CLIP);
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
#[allow(clippy::too_many_arguments)]
fn emit_widget_text_quads(
    batch: &mut QuadBatch,
    font_sys: &mut WowFontSystem,
    glyph_atlas: &mut GlyphAtlas,
    f: &crate::widget::Frame,
    text: &str,
    text_bounds: Rectangle,
    justify_h: TextJustify,
    justify_v: TextJustify,
    word_wrap: bool,
    max_lines: u32,
    alpha: f32,
) {
    let color = [
        f.text_color.r,
        f.text_color.g,
        f.text_color.b,
        f.text_color.a * alpha,
    ];
    let shadow = if f.shadow_color.a > 0.0 {
        Some([
            f.shadow_color.r,
            f.shadow_color.g,
            f.shadow_color.b,
            f.shadow_color.a * alpha,
        ])
    } else {
        None
    };
    let scaled_font_size = f.font_size * f.effective_scale;
    emit_text_quads(
        batch,
        font_sys,
        glyph_atlas,
        text,
        text_bounds,
        f.font.as_deref(),
        scaled_font_size,
        color,
        justify_h,
        justify_v,
        GLYPH_ATLAS_TEX_INDEX,
        shadow,
        f.shadow_offset,
        f.font_outline,
        word_wrap,
        max_lines,
        f.text_stripped.as_deref(),
    );
}

/// Check if a button is visually pressed (mouse or Lua SetButtonState).
fn is_button_pressed(f: &crate::widget::Frame, id: u64, pressed_frame: Option<u64>) -> bool {
    pressed_frame == Some(id) || f.button_state == 1
}

/// Emit quads for a single visible frame based on its widget type.
///
/// `eff_alpha` is the effective alpha from the ancestor chain (`parent_alpha * f.alpha`),
/// matching WoW's `GetEffectiveAlpha()` behavior where parent alpha dims all descendants.
#[allow(clippy::too_many_arguments)]
pub fn emit_frame_quads(
    batch: &mut QuadBatch,
    id: u64,
    f: &crate::widget::Frame,
    bounds: Rectangle,
    clip_bounds: Option<Rectangle>,
    bar_fill: Option<&StatusBarFill>,
    pressed_frame: Option<u64>,
    hovered_frame: Option<u64>,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: Option<
        &std::collections::HashMap<u64, crate::lua_api::message_frame::MessageFrameData>,
    >,
    tooltip_data: Option<&std::collections::HashMap<u64, TooltipRenderData>>,
    registry: &crate::widget::WidgetRegistry,
    elapsed_secs: f64,
    eff_alpha: f32,
) {
    let vert_before = batch.vertices.len();
    match f.widget_type {
        WidgetType::Frame | WidgetType::StatusBar => build_frame_quads(batch, bounds, f, eff_alpha),
        WidgetType::MessageFrame => {
            build_frame_quads(batch, bounds, f, eff_alpha);
            if let Some((fs, ga)) = text_ctx
                && let Some(mf_map) = message_frames
            {
                emit_message_frame_text(
                    batch,
                    fs,
                    ga,
                    f,
                    id,
                    bounds,
                    mf_map,
                    eff_alpha,
                    elapsed_secs,
                );
            }
        }
        WidgetType::GameTooltip => {
            super::tooltip::build_tooltip_quads(
                batch,
                bounds,
                f,
                tooltip_data,
                id,
                text_ctx,
                eff_alpha,
            );
        }
        WidgetType::Minimap => build_minimap_quads(batch, bounds, f, eff_alpha),
        WidgetType::Button => {
            build_button_quads(
                batch,
                bounds,
                f,
                is_button_pressed(f, id, pressed_frame),
                hovered_frame == Some(id),
                eff_alpha,
            );
            if !f.children_keys.contains_key("Text")
                && let Some((fs, ga)) = text_ctx
                && let Some(ref txt) = f.text
            {
                emit_widget_text_quads(
                    batch,
                    fs,
                    ga,
                    f,
                    txt,
                    bounds,
                    f.justify_h,
                    f.justify_v,
                    false,
                    0,
                    eff_alpha,
                );
            }
        }
        WidgetType::Texture => {
            if !f.is_mask {
                let vert_before = batch.vertices.len();
                build_texture_quads(batch, bounds, f, bar_fill, eff_alpha);
                if !f.mask_textures.is_empty() {
                    apply_mask_texture(batch, vert_before, bounds, &f.mask_textures, registry);
                }
            }
        }
        WidgetType::FontString => {
            if let Some((fs, ga)) = text_ctx
                && let Some(ref txt) = f.text
            {
                emit_widget_text_quads(
                    batch,
                    fs,
                    ga,
                    f,
                    txt,
                    bounds,
                    f.justify_h,
                    f.justify_v,
                    f.word_wrap,
                    f.max_lines,
                    eff_alpha,
                );
            }
        }
        WidgetType::CheckButton => {
            build_button_quads(
                batch,
                bounds,
                f,
                is_button_pressed(f, id, pressed_frame),
                hovered_frame == Some(id),
                eff_alpha,
            );
            if let Some((fs, ga)) = text_ctx
                && let Some(ref txt) = f.text
            {
                let label_bounds = Rectangle::new(
                    Point::new(bounds.x + 20.0, bounds.y),
                    Size::new(bounds.width - 20.0, bounds.height),
                );
                emit_widget_text_quads(
                    batch,
                    fs,
                    ga,
                    f,
                    txt,
                    label_bounds,
                    TextJustify::Left,
                    TextJustify::Center,
                    false,
                    0,
                    eff_alpha,
                );
            }
        }
        WidgetType::EditBox => {
            emit_editbox_with_text(batch, bounds, f, text_ctx, eff_alpha);
        }
        WidgetType::Cooldown => {
            build_cooldown_quads(batch, bounds, f, elapsed_secs);
        }
        WidgetType::Line => {
            build_line_quads(batch, f, registry, eff_alpha);
        }
        _ => {}
    }

    if let Some(clip_bounds) = clip_bounds
        && f.rotation == 0.0
    {
        clip_recent_quads(batch, vert_before, clip_bounds);
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
        emit_widget_text_quads(
            batch,
            fs,
            ga,
            f,
            txt,
            text_bounds,
            TextJustify::Left,
            TextJustify::Center,
            false,
            0,
            alpha,
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
    batch.push_cooldown_swipe(bounds, swipe_progress, color);
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

/// Resolve a line anchor to screen-space pixel coordinates.
fn resolve_line_endpoint(
    anchor: &crate::widget::LineAnchor,
    registry: &crate::widget::WidgetRegistry,
) -> Option<(f32, f32)> {
    use super::layout::anchor_position;

    let target_id = anchor.target_id?;
    let r = registry.get(target_id)?.layout_rect?;
    let (ax, ay) = anchor_position(anchor.point, r.x, r.y, r.width, r.height);
    let ui_scale = crate::render::texture::UI_SCALE;
    Some((
        (ax + anchor.x_offset) * ui_scale,
        (ay - anchor.y_offset) * ui_scale,
    ))
}

/// Compute the 4 corner positions of a rotated line quad from endpoints and thickness.
fn line_quad_positions(
    start: (f32, f32),
    end: (f32, f32),
    thickness: f32,
) -> Option<[[f32; 2]; 4]> {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.001 {
        return None;
    }
    let half_t = thickness / 2.0;
    let px = -dy / len * half_t;
    let py = dx / len * half_t;
    Some([
        [start.0 + px, start.1 + py],
        [start.0 - px, start.1 - py],
        [end.0 - px, end.1 - py],
        [end.0 + px, end.1 + py],
    ])
}

/// Build quads for a Line widget — a rotated quad between two anchor points.
fn build_line_quads(
    batch: &mut QuadBatch,
    f: &crate::widget::Frame,
    registry: &crate::widget::WidgetRegistry,
    alpha: f32,
) {
    let (Some(start_anchor), Some(end_anchor)) = (&f.line_start, &f.line_end) else {
        return;
    };
    let Some(sp) = resolve_line_endpoint(start_anchor, registry) else {
        return;
    };
    let Some(ep) = resolve_line_endpoint(end_anchor, registry) else {
        return;
    };

    let thickness = f.line_thickness * crate::render::texture::UI_SCALE;
    let Some(positions) = line_quad_positions(sp, ep, thickness) else {
        return;
    };

    // Use atlas/SetTexCoord UVs when available, otherwise full texture.
    let uvs = if let Some((left, right, top, bottom)) = f.tex_coords {
        [[left, top], [left, bottom], [right, bottom], [right, top]]
    } else {
        [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]]
    };

    let vc = f.vertex_color.as_ref();
    let tint = [
        vc.map_or(1.0, |c| c.r),
        vc.map_or(1.0, |c| c.g),
        vc.map_or(1.0, |c| c.b),
        vc.map_or(1.0, |c| c.a) * alpha,
    ];

    if let Some(color) = f.color_texture {
        let c = [
            color.r * tint[0],
            color.g * tint[1],
            color.b * tint[2],
            color.a * alpha,
        ];
        emit_line_vertices(batch, &positions, &uvs, c, -1, f.blend_mode);
    } else if let Some(ref tex_path) = f.texture {
        let vertex_start = batch.vertices.len() as u32;
        emit_line_vertices(batch, &positions, &uvs, tint, -2, f.blend_mode);
        batch
            .texture_requests
            .push(crate::render::shader::TextureRequest {
                path: tex_path.clone(),
                vertex_start,
                vertex_count: 4,
            });
    } else {
        emit_line_vertices(batch, &positions, &uvs, tint, -1, f.blend_mode);
    }
}

/// Push 4 vertices and 6 indices for a line quad with arbitrary positions.
fn emit_line_vertices(
    batch: &mut QuadBatch,
    positions: &[[f32; 2]; 4],
    uvs: &[[f32; 2]; 4],
    color: [f32; 4],
    tex_index: i32,
    blend_mode: BlendMode,
) {
    use crate::render::shader::QuadVertex;

    let base = batch.vertices.len() as u32;
    let flags = blend_mode as u32;
    for i in 0..4 {
        batch.vertices.push(QuadVertex {
            position: positions[i],
            tex_coords: uvs[i],
            color,
            tex_index,
            flags,
            local_uv: uvs[i],
            mask_tex_index: -1,
            mask_tex_coords: [0.0, 0.0],
        });
    }
    // TL(0)-BL(1)-BR(2) and TL(0)-BR(2)-TR(3)
    batch
        .indices
        .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}
