use super::*;
use crate::render::glyph::GlyphAtlas;
use crate::widget::{Frame, TextSegment, WidgetRegistry, WidgetType};

fn has_glyph_color(batch: &QuadBatch, color: [f32; 4]) -> bool {
    batch
        .vertices
        .iter()
        .any(|vertex| vertex.tex_index == GLYPH_ATLAS_TEX_INDEX && vertex.color == color)
}

fn max_glyph_x(batch: &QuadBatch) -> f32 {
    batch
        .vertices
        .iter()
        .filter(|vertex| vertex.tex_index == GLYPH_ATLAS_TEX_INDEX && vertex.color[3] > 0.0)
        .map(|vertex| vertex.position[0])
        .fold(0.0, f32::max)
}

fn button_highlight_frame(
    registry: &WidgetRegistry,
    hovered_frame: Option<u64>,
) -> FrameQuadEmit<'_> {
    let widget = registry
        .get(2)
        .expect("highlight texture should be registered");
    FrameQuadEmit {
        id: 2,
        widget,
        bounds: Rectangle::new(iced::Point::ORIGIN, iced::Size::new(100.0, 20.0)),
        clip_bounds: None,
        bar_fill: None,
        pressed_frame: None,
        hovered_frame,
        message_frames: None,
        tooltip_data: None,
        quest_blobs: None,
        registry,
        elapsed_secs: 0.0,
        eff_alpha: 1.0,
    }
}

fn registry_with_button_highlight(locked: bool) -> WidgetRegistry {
    let mut registry = WidgetRegistry::new();
    let mut button = Frame::new(WidgetType::Button, None, None);
    button.id = 1;
    button.highlight_locked = locked;
    let mut highlight = Frame::new(WidgetType::Texture, None, Some(1));
    highlight.id = 2;
    highlight.parent_key = Some("HighlightTexture".to_string());
    registry.register(button);
    registry.register(highlight);
    registry
}

#[test]
fn button_highlight_texture_is_inactive_when_parent_not_hovered() {
    let registry = registry_with_button_highlight(false);
    let frame = button_highlight_frame(&registry, None);

    assert!(is_inactive_button_highlight_texture(&frame));
}

#[test]
fn locked_button_highlight_texture_renders_without_hover() {
    let registry = registry_with_button_highlight(true);
    let frame = button_highlight_frame(&registry, None);

    assert!(!is_inactive_button_highlight_texture(&frame));
}

#[test]
fn hovered_button_highlight_texture_stays_out_of_generic_pass() {
    let registry = registry_with_button_highlight(false);
    let frame = button_highlight_frame(&registry, Some(1));

    assert!(is_inactive_button_highlight_texture(&frame));
}

#[test]
fn text_auto_width_disables_render_wrapping() {
    let mut frame = Frame::new(crate::widget::WidgetType::FontString, None, None);
    frame.word_wrap = true;
    frame.width_is_text_auto = true;

    assert!(!effective_word_wrap(&frame, true));
}

#[test]
fn explicit_text_width_keeps_render_wrapping() {
    let mut frame = Frame::new(crate::widget::WidgetType::FontString, None, None);
    frame.word_wrap = true;
    frame.width = 40.0;
    frame.width_is_text_auto = false;

    assert!(effective_word_wrap(&frame, true));
}

#[test]
fn fixed_width_fontstring_clips_no_wrap_text_to_bounds() {
    let mut frame = Frame::new(crate::widget::WidgetType::FontString, None, None);
    frame.width = 60.0;
    frame.height = 20.0;
    frame.width_is_text_auto = false;
    frame.word_wrap = false;

    let mut batch = QuadBatch::new();
    let mut font_sys = WowFontSystem::new();
    let mut glyph_atlas = GlyphAtlas::new();
    let mut text_renderer = WidgetTextRenderer {
        batch: &mut batch,
        font_sys: &mut font_sys,
        glyph_atlas: &mut glyph_atlas,
    };
    let bounds = Rectangle::new(iced::Point::ORIGIN, iced::Size::new(60.0, 20.0));

    emit_widget_text_quads(
        &mut text_renderer,
        &frame,
        WidgetTextLayout {
            text: "Open quest gossip with completed incomplete reward quests",
            bounds,
            justify_h: TextJustify::Left,
            justify_v: TextJustify::Center,
            word_wrap: frame.word_wrap,
            max_lines: 0,
            alpha: 1.0,
        },
    );

    assert!(
        max_glyph_x(&batch) <= bounds.x + bounds.width + 0.5,
        "glyph vertices should be clipped to the FontString right edge"
    );
}

/// A frame that only has a nine-slice layout registered (no active backdrop)
/// must not emit any quads from `build_frame_quads`. The real nine-slice
/// rendering happens through the child Texture pass.
#[test]
fn build_frame_quads_emits_nothing_for_nine_slice_only_frame() {
    let mut frame = Frame::new(crate::widget::WidgetType::Frame, None, None);
    frame.nine_slice_layout = Some("ChatBubble".into());
    let mut batch = QuadBatch::default();
    let before = batch.vertices.len();
    build_frame_quads(
        &mut batch,
        Rectangle::new(iced::Point::new(0.0, 0.0), iced::Size::new(100.0, 50.0)),
        &frame,
        1.0,
    );
    assert_eq!(
        batch.vertices.len(),
        before,
        "nine-slice-only frame must not emit any quads from build_frame_quads"
    );
}

/// A frame with `backdrop.enabled` still emits the solid background the user
/// explicitly configured. Regression guard for the fix above.
#[test]
fn build_frame_quads_still_emits_enabled_backdrop() {
    let mut frame = Frame::new(crate::widget::WidgetType::Frame, None, None);
    frame.backdrop.enabled = true;
    frame.backdrop.bg_color = crate::widget::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };
    let mut batch = QuadBatch::default();
    let before = batch.vertices.len();
    build_frame_quads(
        &mut batch,
        Rectangle::new(iced::Point::new(0.0, 0.0), iced::Size::new(100.0, 50.0)),
        &frame,
        1.0,
    );
    assert!(
        batch.vertices.len() > before,
        "enabled backdrop should still emit quads"
    );
}

#[test]
fn emit_widget_text_quads_uses_text_segment_colors() {
    let mut frame = Frame::new(crate::widget::WidgetType::FontString, None, None);
    frame.text = Some("AB".to_string());
    frame.text_segments = vec![
        TextSegment {
            text: "A".to_string(),
            color: crate::widget::Color::new(1.0, 0.0, 0.0, 1.0),
        },
        TextSegment {
            text: "B".to_string(),
            color: crate::widget::Color::new(0.0, 1.0, 0.0, 1.0),
        },
    ];

    let mut batch = QuadBatch::new();
    let mut font_sys = WowFontSystem::new();
    let mut glyph_atlas = GlyphAtlas::new();
    let mut text_renderer = WidgetTextRenderer {
        batch: &mut batch,
        font_sys: &mut font_sys,
        glyph_atlas: &mut glyph_atlas,
    };

    emit_widget_text_quads(
        &mut text_renderer,
        &frame,
        WidgetTextLayout {
            text: "AB",
            bounds: Rectangle::new(iced::Point::new(0.0, 0.0), iced::Size::new(80.0, 24.0)),
            justify_h: TextJustify::Left,
            justify_v: TextJustify::Center,
            word_wrap: false,
            max_lines: 0,
            alpha: 1.0,
        },
    );

    assert!(has_glyph_color(&batch, [1.0, 0.0, 0.0, 1.0]));
    assert!(has_glyph_color(&batch, [0.0, 1.0, 0.0, 1.0]));
}

#[test]
fn tooltip_line_fontstrings_do_not_render_as_generic_fontstrings() {
    let mut registry = WidgetRegistry::new();
    let mut tooltip = Frame::new(
        WidgetType::GameTooltip,
        Some("GameTooltip".to_string()),
        None,
    );
    tooltip.id = 1;
    let mut line = Frame::new(
        WidgetType::FontString,
        Some("GameTooltipTextLeft1".to_string()),
        Some(1),
    );
    line.id = 2;
    line.parent_key = Some("TextLeft1".to_string());
    line.text = Some("Tooltip line backing text must not render twice".to_string());
    registry.register(tooltip);
    registry.register(line);

    let widget = registry
        .get(2)
        .expect("tooltip line fontstring should be registered");
    let frame = FrameQuadEmit {
        id: 2,
        widget,
        bounds: Rectangle::new(iced::Point::ORIGIN, iced::Size::new(0.0, 0.0)),
        clip_bounds: None,
        bar_fill: None,
        pressed_frame: None,
        hovered_frame: None,
        message_frames: None,
        tooltip_data: None,
        quest_blobs: None,
        registry: &registry,
        elapsed_secs: 0.0,
        eff_alpha: 1.0,
    };

    let mut batch = QuadBatch::new();
    let mut font_sys = WowFontSystem::new();
    let mut glyph_atlas = GlyphAtlas::new();
    let mut text_ctx = Some((&mut font_sys, &mut glyph_atlas));

    emit_frame_quads(&mut batch, &mut text_ctx, frame);

    assert!(
        batch
            .vertices
            .iter()
            .all(|vertex| vertex.tex_index != GLYPH_ATLAS_TEX_INDEX),
        "tooltip line backing FontStrings should not emit their own glyphs"
    );
}
