//! Strata-level quad emission for headless and screenshot paths.

use iced::{Point, Rectangle, Size};

use crate::render::QuadBatch;
use crate::render::font::WowFontSystem;
use crate::render::glyph::GlyphAtlas;
use crate::render::texture::UI_SCALE;
use crate::widget::WidgetType;

use super::frame_collect::{CollectedFrames, collect_hittable_frames, collect_subtree_ids};
use super::quad_builders::emit_frame_quads;
use super::statusbar::collect_statusbar_fills;
use super::tooltip::TooltipRenderData;

/// Emit quads for a single strata bucket (used by headless/screenshot paths).
///
/// Reads rect and effective_alpha fresh from the registry for each frame.
/// Button state textures use parent's effective_alpha as fallback.
#[allow(clippy::too_many_arguments)]
pub(super) fn emit_single_strata(
    batch: &mut QuadBatch,
    bucket: &[u64],
    registry: &crate::widget::WidgetRegistry,
    visible_ids: &Option<std::collections::HashSet<u64>>,
    pressed_frame: Option<u64>,
    hovered_frame: Option<u64>,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: Option<
        &std::collections::HashMap<u64, crate::lua_api::message_frame::MessageFrameData>,
    >,
    tooltip_data: Option<&std::collections::HashMap<u64, TooltipRenderData>>,
    elapsed_secs: f64,
) {
    let render_list = build_render_list(bucket, registry);
    let statusbar_fills = collect_statusbar_fills(&render_list, registry);

    for &(id, rect, clip_rect, eff_alpha) in &render_list {
        let Some(f) = registry.get(id) else { continue };
        if super::button_vis::should_skip_frame(
            f,
            id,
            eff_alpha,
            visible_ids,
            registry,
            pressed_frame,
            hovered_frame,
        ) {
            continue;
        }
        let is_fontstring = matches!(f.widget_type, WidgetType::FontString);
        let is_line = matches!(f.widget_type, WidgetType::Line);
        if (rect.height <= 0.0 && !is_line) || (rect.width <= 0.0 && !is_fontstring && !is_line) {
            continue;
        }
        let bounds = Rectangle::new(
            Point::new(rect.x * UI_SCALE, rect.y * UI_SCALE),
            Size::new(rect.width * UI_SCALE, rect.height * UI_SCALE),
        );
        let bar_fill = statusbar_fills.get(&id);
        emit_frame_quads(
            batch,
            id,
            f,
            bounds,
            clip_rect.map(layout_rect_to_screen_rect),
            bar_fill,
            pressed_frame,
            hovered_frame,
            text_ctx,
            message_frames,
            tooltip_data,
            registry,
            elapsed_secs,
            eff_alpha,
        );
    }
}

/// Build the render list: visible frames with resolved rects and effective alpha.
pub(super) fn build_render_list(
    bucket: &[u64],
    registry: &crate::widget::WidgetRegistry,
) -> Vec<(u64, crate::LayoutRect, Option<crate::LayoutRect>, f32)> {
    let mut list = Vec::new();
    for &id in bucket {
        let Some(f) = registry.get(id) else { continue };
        let Some(rect) = f.layout_rect else { continue };
        let clip_rect = resolve_clip_rect(id, registry);
        let eff_alpha = resolve_eff_alpha(f, registry);
        if eff_alpha <= 0.0 {
            continue;
        }
        if clip_rect.is_some_and(|clip| intersect_rects(rect, clip).is_none()) {
            continue;
        }
        list.push((id, rect, clip_rect, eff_alpha));
    }
    list
}

/// Resolve effective alpha for a frame, with button-state-texture parent fallback.
fn resolve_eff_alpha(f: &crate::widget::Frame, registry: &crate::widget::WidgetRegistry) -> f32 {
    if f.effective_alpha > 0.0 {
        return f.effective_alpha;
    }
    if f.alpha > 0.0 {
        return f
            .parent_id
            .and_then(|pid| registry.get(pid))
            .map(|p| p.effective_alpha)
            .unwrap_or(0.0);
    }
    0.0
}

fn resolve_clip_rect(
    id: u64,
    registry: &crate::widget::WidgetRegistry,
) -> Option<crate::LayoutRect> {
    let mut current_id = registry.get(id).and_then(|f| f.parent_id);
    let mut clip_rect: Option<crate::LayoutRect> = None;
    while let Some(parent_id) = current_id {
        let Some(parent) = registry.get(parent_id) else { break };
        if parent.clips_children
            && let Some(parent_rect) = parent.layout_rect
        {
            clip_rect = Some(match clip_rect {
                Some(existing) => intersect_rects(existing, parent_rect)?,
                None => parent_rect,
            });
        }
        current_id = parent.parent_id;
    }
    clip_rect
}

fn intersect_rects(a: crate::LayoutRect, b: crate::LayoutRect) -> Option<crate::LayoutRect> {
    let left = a.x.max(b.x);
    let top = a.y.max(b.y);
    let right = (a.x + a.width).min(b.x + b.width);
    let bottom = (a.y + a.height).min(b.y + b.height);
    (right > left && bottom > top).then_some(crate::LayoutRect {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    })
}

fn layout_rect_to_screen_rect(rect: crate::LayoutRect) -> Rectangle {
    Rectangle::new(
        Point::new(rect.x * UI_SCALE, rect.y * UI_SCALE),
        Size::new(rect.width * UI_SCALE, rect.height * UI_SCALE),
    )
}

/// Build a QuadBatch from a WidgetRegistry without needing an App instance.
#[allow(clippy::too_many_arguments)]
pub fn build_quad_batch_for_registry(
    registry: &crate::widget::WidgetRegistry,
    screen_size: (f32, f32),
    root_name: Option<&str>,
    pressed_frame: Option<u64>,
    hovered_frame: Option<u64>,
    mut text_ctx: Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: Option<
        &std::collections::HashMap<u64, crate::lua_api::message_frame::MessageFrameData>,
    >,
    tooltip_data: Option<&std::collections::HashMap<u64, TooltipRenderData>>,
    strata_buckets: &Vec<Vec<u64>>,
) -> QuadBatch {
    let (batch, _) = build_quad_batch_with_cache(
        registry,
        screen_size,
        root_name,
        pressed_frame,
        hovered_frame,
        &mut text_ctx,
        message_frames,
        tooltip_data,
        strata_buckets,
        0.0,
    );
    batch
}

/// Scale hittable layout rects to screen coordinates, applying hit rect insets.
pub fn build_hittable_rects(
    collected: &CollectedFrames,
    registry: &crate::widget::WidgetRegistry,
) -> Vec<(u64, Rectangle)> {
    collected
        .hittable
        .iter()
        .map(|&(id, r)| {
            let (il, ir, it, ib) = registry
                .get(id)
                .map(|f| f.hit_rect_insets)
                .unwrap_or((0.0, 0.0, 0.0, 0.0));
            (
                id,
                Rectangle::new(
                    Point::new((r.x + il) * UI_SCALE, (r.y + it) * UI_SCALE),
                    Size::new(
                        (r.width - il - ir).max(0.0) * UI_SCALE,
                        (r.height - it - ib).max(0.0) * UI_SCALE,
                    ),
                ),
            )
        })
        .collect()
}

/// Build a QuadBatch by iterating visible-only strata buckets directly.
///
/// Also builds a hittable frame list as a side output for hit testing.
#[allow(clippy::too_many_arguments)]
pub fn build_quad_batch_with_cache(
    registry: &crate::widget::WidgetRegistry,
    screen_size: (f32, f32),
    root_name: Option<&str>,
    pressed_frame: Option<u64>,
    hovered_frame: Option<u64>,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: Option<
        &std::collections::HashMap<u64, crate::lua_api::message_frame::MessageFrameData>,
    >,
    tooltip_data: Option<&std::collections::HashMap<u64, TooltipRenderData>>,
    strata_buckets: &[Vec<u64>],
    elapsed_secs: f64,
) -> (QuadBatch, CollectedFrames) {
    let mut batch = QuadBatch::with_capacity(1000);
    let size = Size::new(screen_size.0, screen_size.1);

    batch.push_tiled_path(
        Rectangle::new(Point::ORIGIN, size),
        256.0,
        256.0,
        "framegeneral/ui-background-marble",
        [0.55, 0.55, 0.55, 1.0],
    );

    let visible_ids = root_name.map(|name| collect_subtree_ids(registry, name));
    let collected = collect_hittable_frames(registry, strata_buckets);

    for bucket in strata_buckets {
        emit_single_strata(
            &mut batch,
            bucket,
            registry,
            &visible_ids,
            pressed_frame,
            hovered_frame,
            text_ctx,
            message_frames,
            tooltip_data,
            elapsed_secs,
        );
    }
    (batch, collected)
}
