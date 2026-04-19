//! Strata-level quad emission for headless and screenshot paths.

use iced::{Point, Rectangle, Size};
use rustc_hash::FxHashSet;

use crate::render::QuadBatch;
use crate::render::font::WowFontSystem;
use crate::render::glyph::GlyphAtlas;
use crate::render::texture::UI_SCALE;
use crate::widget::WidgetType;

use super::frame_collect::{CollectedFrames, collect_hittable_frames, collect_subtree_ids};
use super::quad_builders::{FrameQuadEmit, emit_frame_quads};
use super::statusbar::collect_statusbar_fills;
use super::tooltip::TooltipRenderData;

fn uses_parent_alpha_fallback(frame: &crate::widget::Frame) -> bool {
    matches!(
        frame.parent_key.as_deref(),
        Some("NormalTexture" | "PushedTexture" | "HighlightTexture" | "DisabledTexture")
    )
}

fn chain_effective_alpha_from(
    start_id: Option<u64>,
    registry: &crate::widget::WidgetRegistry,
) -> f32 {
    let Some(mut current_id) = start_id else {
        return 1.0;
    };
    let mut alpha = 1.0;
    loop {
        let Some(frame) = registry.get(current_id) else {
            return 0.0;
        };
        if !frame.visible {
            return 0.0;
        }
        alpha *= frame.alpha;
        let Some(parent_id) = frame.parent_id else {
            return alpha;
        };
        current_id = parent_id;
    }
}

/// Emit quads for a single strata bucket (used by headless/screenshot paths).
///
/// Reads rect and effective_alpha fresh from the registry for each frame.
/// Button state textures use parent's effective_alpha as fallback.
pub(super) fn emit_single_strata(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    params: SingleStrataEmit<'_>,
) {
    let render_list = build_render_list(params.bucket, params.registry, params.screen_size);
    let statusbar_fills = collect_statusbar_fills(&render_list, params.registry);

    for entry in &render_list {
        emit_render_list_entry(batch, text_ctx, &statusbar_fills, params, *entry);
    }
}

fn emit_render_list_entry(
    batch: &mut QuadBatch,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    statusbar_fills: &std::collections::HashMap<u64, super::statusbar::StatusBarFill>,
    params: SingleStrataEmit<'_>,
    entry: (u64, crate::LayoutRect, Option<crate::LayoutRect>, f32),
) {
    let (id, rect, clip_rect, eff_alpha) = entry;
    let Some(frame) = render_list_frame(params, id, rect, eff_alpha) else {
        return;
    };
    let bounds = layout_rect_to_screen_rect(rect);
    emit_frame_quads(
        batch,
        text_ctx,
        FrameQuadEmit {
            id,
            widget: frame,
            bounds,
            clip_bounds: clip_rect.map(layout_rect_to_screen_rect),
            bar_fill: statusbar_fills.get(&id),
            pressed_frame: params.pressed_frame,
            hovered_frame: params.hovered_frame,
            message_frames: params.message_frames,
            tooltip_data: params.tooltip_data,
            quest_blobs: params.quest_blobs,
            registry: params.registry,
            elapsed_secs: params.elapsed_secs,
            eff_alpha,
        },
    );
}

fn render_list_frame(
    params: SingleStrataEmit<'_>,
    id: u64,
    rect: crate::LayoutRect,
    eff_alpha: f32,
) -> Option<&crate::widget::Frame> {
    let frame = params.registry.get(id)?;
    if super::button_vis::should_skip_frame(
        frame,
        id,
        eff_alpha,
        params.visible_ids,
        params.registry,
        params.pressed_frame,
        params.hovered_frame,
    ) {
        return None;
    }
    renderable_frame_with_bounds(frame, rect)
}

fn renderable_frame_with_bounds(
    frame: &crate::widget::Frame,
    rect: crate::LayoutRect,
) -> Option<&crate::widget::Frame> {
    let is_fontstring = matches!(frame.widget_type, WidgetType::FontString);
    let is_line = matches!(frame.widget_type, WidgetType::Line);
    let has_height = rect.height > 0.0 || is_line;
    let has_width = rect.width > 0.0 || is_fontstring || is_line;
    (has_height && has_width).then_some(frame)
}

#[derive(Clone, Copy)]
pub(super) struct SingleStrataEmit<'a> {
    bucket: &'a [u64],
    registry: &'a crate::widget::WidgetRegistry,
    visible_ids: &'a Option<FxHashSet<u64>>,
    screen_size: (f32, f32),
    pressed_frame: Option<u64>,
    hovered_frame: Option<u64>,
    message_frames:
        Option<&'a std::collections::HashMap<u64, crate::lua_api::message_frame::MessageFrameData>>,
    tooltip_data: Option<&'a std::collections::HashMap<u64, TooltipRenderData>>,
    quest_blobs: Option<&'a std::collections::HashMap<u64, crate::lua_api::state::QuestBlobState>>,
    elapsed_secs: f64,
}

/// Build the render list: visible frames with resolved rects and effective alpha.
pub(super) fn build_render_list(
    bucket: &[u64],
    registry: &crate::widget::WidgetRegistry,
    screen_size: (f32, f32),
) -> Vec<(u64, crate::LayoutRect, Option<crate::LayoutRect>, f32)> {
    let mut list = Vec::new();
    for &id in bucket {
        let Some(f) = registry.get(id) else { continue };
        let rect = f.layout_rect.unwrap_or_else(|| {
            crate::layout::compute_frame_rect(registry, id, screen_size.0, screen_size.1)
        });
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
    if f.alpha > 0.0 && uses_parent_alpha_fallback(f) {
        return chain_effective_alpha_from(f.parent_id, registry) * f.alpha;
    }
    chain_effective_alpha_from(Some(f.id), registry)
}

fn resolve_clip_rect(
    id: u64,
    registry: &crate::widget::WidgetRegistry,
) -> Option<crate::LayoutRect> {
    let mut current_id = id;
    let mut clip_rect: Option<crate::LayoutRect> = None;
    while let Some(parent_id) = registry.get(current_id).and_then(|f| f.parent_id) {
        let Some(parent) = registry.get(parent_id) else {
            break;
        };
        let clips_scroll_child =
            matches!(parent.widget_type, crate::widget::WidgetType::ScrollFrame)
                && parent.scroll_child_id == Some(current_id);
        if (parent.clips_children || clips_scroll_child)
            && let Some(parent_rect) = parent.layout_rect
        {
            clip_rect = Some(match clip_rect {
                Some(existing) => intersect_rects(existing, parent_rect)?,
                None => parent_rect,
            });
        }
        current_id = parent_id;
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
    text_ctx: Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: Option<
        &std::collections::HashMap<u64, crate::lua_api::message_frame::MessageFrameData>,
    >,
    tooltip_data: Option<&std::collections::HashMap<u64, TooltipRenderData>>,
    strata_buckets: &Vec<Vec<u64>>,
) -> QuadBatch {
    build_quad_batch_for_registry_with_quest_blobs(
        registry,
        screen_size,
        root_name,
        pressed_frame,
        hovered_frame,
        text_ctx,
        message_frames,
        tooltip_data,
        None,
        strata_buckets,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn build_quad_batch_for_registry_with_quest_blobs(
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
    quest_blobs: Option<&std::collections::HashMap<u64, crate::lua_api::state::QuestBlobState>>,
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
        quest_blobs,
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
    quest_blobs: Option<&std::collections::HashMap<u64, crate::lua_api::state::QuestBlobState>>,
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
            text_ctx,
            SingleStrataEmit {
                bucket,
                registry,
                visible_ids: &visible_ids,
                screen_size,
                pressed_frame,
                hovered_frame,
                message_frames,
                tooltip_data,
                quest_blobs,
                elapsed_secs,
            },
        );
    }
    (batch, collected)
}

#[cfg(test)]
mod tests {
    use super::resolve_eff_alpha;
    use crate::widget::{Frame, WidgetRegistry, WidgetType};

    #[test]
    fn button_state_texture_alpha_fallback_multiplies_own_alpha() {
        let mut registry = WidgetRegistry::new();

        let root = Frame::new(WidgetType::Frame, Some("UIParent".to_string()), None);
        let root_id = root.id;
        registry.register(root);

        let mut button = Frame::new(
            WidgetType::Button,
            Some("GameMenuButton".to_string()),
            Some(root_id),
        );
        button.alpha = 0.8;
        let button_id = button.id;
        registry.register(button);
        registry.add_child(root_id, button_id);

        let mut normal = Frame::new(WidgetType::Texture, None, Some(button_id));
        normal.visible = false;
        normal.alpha = 0.4;
        normal.parent_key = Some("NormalTexture".to_string());
        let normal_id = normal.id;
        registry.register(normal);
        registry.add_child(button_id, normal_id);

        registry.propagate_all_effective_alpha();

        let alpha = resolve_eff_alpha(registry.get(normal_id).unwrap(), &registry);
        assert!(
            (alpha - 0.32).abs() < f32::EPSILON,
            "expected hidden button texture alpha fallback to keep child alpha, got {alpha}"
        );
    }
}
