//! SimpleHTML block presentation using the shared document layout.
use super::quad_builders::{FrameQuadEmit, WidgetTextRenderer, clip_recent_quads};
use crate::lua_api::simple_html::layout::{LaidOutBlock, layout_document};
use crate::render::shader::GLYPH_ATLAS_TEX_INDEX;
use crate::render::{GlyphAtlas, QuadBatch, WowFontSystem, emit_text_quads};
use crate::widget::{TextJustify, TextOutline};
use iced::{Point, Rectangle, Size};

pub(super) fn emit_simple_html(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    frame: &FrameQuadEmit<'_>,
) {
    let Some((fonts, atlas)) = text_ctx else {
        return;
    };
    let scale = frame.widget.effective_scale.max(f32::EPSILON) * crate::render::texture::UI_SCALE;
    let layout = layout_document(frame.widget, frame.bounds.width / scale, fonts);
    let mut renderer = WidgetTextRenderer {
        batch,
        font_sys: fonts,
        glyph_atlas: atlas,
    };
    for block in layout.blocks {
        let bounds = Rectangle::new(
            Point::new(frame.bounds.x, frame.bounds.y + block.y * scale),
            Size::new(frame.bounds.width, block.height * scale),
        );
        let start = renderer.batch.vertices.len();
        emit_block(&mut renderer, &block, bounds, (scale, frame.eff_alpha));
        clip_recent_quads(renderer.batch, start, bounds);
    }
}

fn emit_block(
    renderer: &mut WidgetTextRenderer<'_>,
    block: &LaidOutBlock,
    bounds: Rectangle,
    presentation: (f32, f32),
) {
    let style = &block.style;
    let (scale, alpha) = presentation;
    let (r, g, b, a) = style.text_color;
    emit_text_quads(
        renderer.batch,
        renderer.font_sys,
        renderer.glyph_atlas,
        &block.block.text,
        bounds,
        style.font.as_deref(),
        style.font_size * scale,
        [r, g, b, a * alpha],
        TextJustify::from_wow_str(&style.justify_h),
        TextJustify::Left,
        GLYPH_ATLAS_TEX_INDEX,
        None,
        (0.0, 0.0),
        text_outline(style.font_object.as_deref()),
        true,
        0,
        style.spacing * scale,
        false,
        Some(&block.block.text),
        None, // SimpleHTML gradients are outside the approved H1 contract.
    );
}

fn text_outline(flags: Option<&str>) -> TextOutline {
    match flags {
        Some("OUTLINE") => TextOutline::Outline,
        Some("THICKOUTLINE") => TextOutline::ThickOutline,
        _ => TextOutline::None,
    }
}
