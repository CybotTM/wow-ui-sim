//! Button and EditBox widget quad emitters.

use iced::{Point, Rectangle, Size};

use crate::render::glyph::{PreparedBufferTextRequest, emit_prepared_buffer_text_quads};
use crate::render::shader::GLYPH_ATLAS_TEX_INDEX;
use crate::render::{BlendMode, QuadBatch, ThreeSlicePathParams};
use crate::widget::{TextJustify, WidgetType};

use super::textures::remap_atlas_crop;
use super::{
    FrameQuadEmit, WidgetTextLayout, WidgetTextRenderer, clip_recent_quads, color_with_alpha,
    emit_widget_text_quads,
};

const BUTTON_TEXT_CHILD_KEYS: [&str; 3] = ["Text", "text", "ButtonText"];

type ButtonTextureState<'a> = (Option<&'a String>, Option<(f32, f32, f32, f32)>, bool);

/// UI-Panel-Button-Up is 128×32; button-up strip occupies rows 0-21 (V = 0 .. 22/32 = 0.6875).
const PANEL_BUTTON_UP_CROP_V: f32 = 22.0 / 32.0;

fn remap_panel_button_skin(
    tex_path: &str,
    fill_uvs: Option<(f32, f32, f32, f32)>,
    atlas_tex_coords: Option<(f32, f32, f32, f32)>,
) -> (String, Option<(f32, f32, f32, f32)>) {
    let crop = should_crop_panel_button_skin(tex_path);
    let clamp_v = |(left, right, top, bottom): (f32, f32, f32, f32)| {
        if crop {
            (left, right, top, bottom.min(PANEL_BUTTON_UP_CROP_V))
        } else {
            (left, right, top, bottom)
        }
    };
    remap_atlas_crop(
        tex_path,
        fill_uvs.map(clamp_v),
        atlas_tex_coords.map(clamp_v),
    )
}

fn should_crop_panel_button_skin(texture_path: &str) -> bool {
    matches!(
        texture_path,
        "Interface/Buttons/UI-Panel-Button-Up"
            | "Interface\\Buttons\\UI-Panel-Button-Up"
            | "Interface/Buttons/UI-Panel-Button-Highlight"
            | "Interface\\Buttons\\UI-Panel-Button-Highlight"
    )
}

/// Remap atlas crop UVs and emit a single quad with the resulting texture/UV mapping.
fn push_skinned_button_quad(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    tex_path: &str,
    tex_coords: (f32, f32, f32, f32),
    color: [f32; 4],
    blend_mode: BlendMode,
) {
    let (effective_path, effective_uvs) =
        remap_panel_button_skin(tex_path, Some(tex_coords), Some(tex_coords));
    let (left, right, top, bottom) = effective_uvs.unwrap_or((0.0, 1.0, 0.0, 1.0));
    let uvs = Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top));
    batch.push_textured_path_uv(bounds, uvs, &effective_path, color, blend_mode);
}

const BUTTON_TEX_V_BOTTOM: f32 = 0.6875;

/// Build quads for a Button widget.
pub(super) fn build_button_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    is_pressed: bool,
    is_hovered: bool,
    alpha: f32,
) {
    let (texture_path, tex_coords, skip) = button_texture_state(f, is_pressed);
    if !skip {
        emit_button_texture(batch, bounds, texture_path, tex_coords, alpha);
    }

    let has_highlight_child = f.children_keys.contains_key("HighlightTexture");
    if is_hovered && !is_pressed && !has_highlight_child {
        emit_button_highlight(batch, bounds, f, alpha);
    }
}

fn button_texture_state(f: &crate::widget::Frame, is_pressed: bool) -> ButtonTextureState<'_> {
    let has_normal_child = f.children_keys.contains_key("NormalTexture");
    let has_pushed_child = f.children_keys.contains_key("PushedTexture");

    if is_pressed {
        let texture_path = f.pushed_texture.as_ref().or(f.normal_texture.as_ref());
        let tex_coords = f.pushed_tex_coords.or(f.normal_tex_coords);
        let skip = if f.pushed_texture.is_some() {
            has_pushed_child
        } else {
            has_normal_child
        };
        return (texture_path, tex_coords, skip);
    }

    (
        f.normal_texture.as_ref(),
        f.normal_tex_coords,
        has_normal_child,
    )
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
    if is_ui_panel_button_atlas(tex_path) {
        emit_ui_panel_button_strip(batch, bounds, tex_path, tex_coords, alpha, BlendMode::Alpha);
        return;
    }

    emit_skinned_button_texture(batch, bounds, tex_path, tex_coords, alpha);
}

fn emit_skinned_button_texture(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    tex_path: &str,
    tex_coords: Option<(f32, f32, f32, f32)>,
    alpha: f32,
) {
    if let Some(tex_coords) = tex_coords {
        push_skinned_button_quad(
            batch,
            bounds,
            tex_path,
            tex_coords,
            [1.0, 1.0, 1.0, alpha],
            BlendMode::Alpha,
        );
    } else {
        emit_button_three_slice(batch, bounds, tex_path, alpha, BlendMode::Alpha);
    }
}

/// Render the button highlight overlay on hover.
pub(crate) fn emit_button_highlight(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    alpha: f32,
) {
    const HIGHLIGHT_ALPHA: f32 = 0.5;

    let Some(highlight_path) = &f.highlight_texture else {
        return;
    };
    let alpha = HIGHLIGHT_ALPHA * alpha;

    if is_ui_panel_button_atlas(highlight_path) {
        emit_ui_panel_button_strip(
            batch,
            bounds,
            highlight_path,
            f.highlight_tex_coords,
            alpha,
            BlendMode::Additive,
        );
        return;
    }

    emit_skinned_button_highlight(batch, bounds, highlight_path, f.highlight_tex_coords, alpha);
}

fn emit_skinned_button_highlight(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    highlight_path: &str,
    tex_coords: Option<(f32, f32, f32, f32)>,
    alpha: f32,
) {
    if let Some(tex_coords) = tex_coords {
        push_skinned_button_quad(
            batch,
            bounds,
            highlight_path,
            tex_coords,
            [1.0, 1.0, 1.0, alpha],
            BlendMode::Additive,
        );
    } else {
        emit_button_three_slice(batch, bounds, highlight_path, alpha, BlendMode::Additive);
    }
}

/// 3-slice horizontal fallback for button textures with the up-strip V crop.
fn emit_button_three_slice(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    tex_path: &str,
    alpha: f32,
    blend_mode: BlendMode,
) {
    const BUTTON_TEX_WIDTH: f32 = 128.0;
    const BUTTON_CAP_WIDTH: f32 = 4.0;
    batch.push_three_slice_h_path_blend(ThreeSlicePathParams {
        bounds,
        left_cap_width: BUTTON_CAP_WIDTH,
        right_cap_width: BUTTON_CAP_WIDTH,
        path: tex_path,
        tex_width: BUTTON_TEX_WIDTH,
        color: [1.0, 1.0, 1.0, alpha],
        blend_mode,
        v_top: 0.0,
        v_bottom: BUTTON_TEX_V_BOTTOM,
    });
}

fn emit_ui_panel_button_strip(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    tex_path: &str,
    tex_coords: Option<(f32, f32, f32, f32)>,
    alpha: f32,
    blend_mode: BlendMode,
) {
    const BUTTON_TEX_WIDTH: f32 = 128.0;
    const BUTTON_CAP_WIDTH: f32 = 4.0;

    if let Some(tex_coords) = tex_coords {
        emit_ui_panel_button_strip_crop(batch, bounds, tex_path, tex_coords, alpha, blend_mode);
        return;
    }

    emit_ui_panel_button_strip_fallback(
        batch,
        bounds,
        tex_path,
        alpha,
        blend_mode,
        BUTTON_TEX_WIDTH,
        BUTTON_CAP_WIDTH,
    );
}

fn emit_ui_panel_button_strip_crop(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    tex_path: &str,
    tex_coords: (f32, f32, f32, f32),
    alpha: f32,
    blend_mode: BlendMode,
) {
    let (left, right, top, bottom) = tex_coords;
    let bottom = if bottom > 0.9 {
        PANEL_BUTTON_UP_CROP_V
    } else {
        bottom
    };
    push_skinned_button_quad(
        batch,
        bounds,
        tex_path,
        (left, right, top, bottom),
        [1.0, 1.0, 1.0, alpha],
        blend_mode,
    );
}

fn emit_ui_panel_button_strip_fallback(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    tex_path: &str,
    alpha: f32,
    blend_mode: BlendMode,
    texture_width: f32,
    cap_width: f32,
) {
    batch.push_three_slice_h_path_blend(ThreeSlicePathParams {
        bounds,
        left_cap_width: cap_width,
        right_cap_width: cap_width,
        path: tex_path,
        tex_width: texture_width,
        color: [1.0, 1.0, 1.0, alpha],
        blend_mode,
        v_top: 0.0,
        v_bottom: PANEL_BUTTON_UP_CROP_V,
    });
}

fn is_ui_panel_button_atlas(path: &str) -> bool {
    let lower = path.replace('\\', "/").to_ascii_lowercase();
    lower.ends_with("interface/buttons/ui-panel-button-up")
        || lower.ends_with("interface/buttons/ui-panel-button-highlight")
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
    emit_editbox_presentation(batch, bounds, f, text_ctx, alpha);
    if f.editbox_focused {
        batch.push_border(bounds, 1.0, [1.0, 0.85, 0.30, 0.85 * alpha]);
    }
}

fn emit_editbox_presentation(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    text_ctx: &mut Option<(
        &mut crate::render::font::WowFontSystem,
        &mut crate::render::glyph::GlyphAtlas,
    )>,
    alpha: f32,
) {
    let Some(presentation) = f.editbox_presentation.as_deref() else {
        return;
    };
    let Some(input) = presentation.input.as_ref() else {
        return;
    };
    let (left, top, width, height) = input.settings.content_bounds();
    let text_bounds = Rectangle::new(
        Point::new(bounds.x + left, bounds.y + top),
        Size::new(width, height),
    );
    emit_editbox_text(batch, f, presentation, text_ctx, text_bounds, alpha);
    if f.editbox_focused
        && let Some(geometry) = presentation.cursor_geometry
    {
        emit_editbox_caret(
            batch,
            bounds,
            text_bounds,
            geometry,
            input.settings.scale,
            alpha,
        );
    }
}

fn emit_editbox_caret(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    text_bounds: Rectangle,
    geometry: (f32, f32, f32, f32),
    scale: f32,
    alpha: f32,
) {
    let (x, y, width, height) = geometry;
    let caret_bounds = Rectangle::new(
        Point::new(bounds.x + x * scale, bounds.y - y * scale),
        Size::new(width * scale, height * scale),
    );
    let vert_before = batch.vertices.len();
    batch.push_solid(caret_bounds, [1.0, 0.95, 0.55, 0.95 * alpha]);
    clip_recent_quads(batch, vert_before, text_bounds);
}

fn emit_editbox_text(
    batch: &mut QuadBatch,
    f: &crate::widget::Frame,
    presentation: &crate::widget::editbox::EditBoxPresentation,
    text_ctx: &mut Option<(
        &mut crate::render::font::WowFontSystem,
        &mut crate::render::glyph::GlyphAtlas,
    )>,
    text_bounds: Rectangle,
    alpha: f32,
) {
    let Some((fs, ga)) = text_ctx else {
        return;
    };
    let Some(request) = editbox_text_request(f, presentation, text_bounds, alpha) else {
        return;
    };
    let vert_before = batch.vertices.len();
    emit_prepared_buffer_text_quads(batch, fs, ga, request);
    clip_recent_quads(batch, vert_before, text_bounds);
}

fn editbox_text_request<'a>(
    f: &crate::widget::Frame,
    presentation: &'a crate::widget::editbox::EditBoxPresentation,
    text_bounds: Rectangle,
    alpha: f32,
) -> Option<PreparedBufferTextRequest<'a>> {
    let input = presentation.input.as_ref()?;
    let buffer = presentation.shaped_text.as_ref()?;
    let settings = input.settings;
    let shadow_color = (f.shadow_color.a > 0.0).then(|| color_with_alpha(&f.shadow_color, alpha));
    Some(PreparedBufferTextRequest {
        buffer,
        bounds: text_bounds,
        spacing: settings.spacing,
        justify_h: settings.justify_h,
        justify_v: settings.justify_v,
        color: color_with_alpha(&f.text_color, alpha),
        glyph_tex_index: GLYPH_ATLAS_TEX_INDEX,
        shadow_color,
        shadow_offset: f.shadow_offset,
        outline: f.font_outline,
    })
}
