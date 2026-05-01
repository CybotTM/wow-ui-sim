use iced::{Point, Rectangle, Size};

use rustc_hash::FxHashSet;
use std::collections::HashMap;
use std::sync::Arc;

use crate::render::font::WowFontSystem;
use crate::render::glyph::GlyphAtlas;
use crate::render::texture::UI_SCALE;
use crate::render::{FrameQuadSnapshot, QuadBatch};
use crate::widget::{FrameStrata, WidgetType};

use super::super::quad_builders::{FrameQuadEmit, emit_frame_quads};
use super::super::statusbar::collect_statusbar_fills;
use super::super::strata_emit::build_render_list;
use super::super::tooltip::TooltipRenderData;

pub(crate) type StrataBatchCache = [Option<Arc<QuadBatch>>; FrameStrata::COUNT];
pub(crate) type StrataSnapshotCache = [Option<HashMap<u64, FrameQuadSnapshot>>; FrameStrata::COUNT];

/// Rebuild strata batches for all dirty strata indices.
///
/// When `dirty_ids` is `Some`, uses per-frame snapshot cache for incremental
/// rebuild: only dirty frames re-emit, while clean frames reuse cached quads.
fn rebuild_strata_batches(
    strata_cache: &mut StrataBatchCache,
    snapshot_cache: &mut StrataSnapshotCache,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    params: RebuildStrataBatches<'_>,
) {
    for strata_idx in 0..FrameStrata::COUNT {
        if params.dirty & (1 << strata_idx) == 0 && strata_cache[strata_idx].is_some() {
            continue;
        }

        let bucket = params
            .strata_buckets
            .get(strata_idx)
            .map(|bucket| bucket.as_slice())
            .unwrap_or(&[]);
        let strata_start = std::time::Instant::now();
        let mut batch = QuadBatch::new();
        if strata_idx == 0 {
            emit_marble_background(&mut batch, params.size);
        }

        let snapshots = snapshot_cache[strata_idx].get_or_insert_with(HashMap::new);
        let stats = emit_strata_cached(
            &mut batch,
            snapshots,
            bucket,
            params.dirty_ids,
            params.widgets,
            (params.size.width / UI_SCALE, params.size.height / UI_SCALE),
            params.pressed_frame,
            params.hovered_frame,
            text_ctx,
            params.message_frames,
            params.tooltip_data,
            params.quest_blobs,
            params.elapsed_secs,
        );
        log_strata_timing(strata_idx, bucket.len(), &stats, strata_start.elapsed());
        strata_cache[strata_idx] = Some(Arc::new(batch));
    }
}

struct RebuildStrataBatches<'a> {
    dirty: u16,
    dirty_ids: Option<&'a FxHashSet<u64>>,
    size: Size,
    strata_buckets: &'a [Vec<u64>],
    widgets: &'a crate::widget::WidgetRegistry,
    pressed_frame: Option<u64>,
    hovered_frame: Option<u64>,
    message_frames: &'a HashMap<u64, crate::lua_api::MessageFrameData>,
    tooltip_data: &'a HashMap<u64, TooltipRenderData>,
    quest_blobs: &'a HashMap<u64, crate::lua_api::state::QuestBlobState>,
    elapsed_secs: f64,
}

fn emit_marble_background(batch: &mut QuadBatch, size: Size) {
    batch.push_tiled_path(
        Rectangle::new(Point::ORIGIN, size),
        256.0,
        256.0,
        "framegeneral/ui-background-marble",
        [0.55, 0.55, 0.55, 1.0],
    );
}

struct EmitStats {
    cached: u32,
    emitted: u32,
}

fn log_strata_timing(
    strata_idx: usize,
    frame_count: usize,
    stats: &EmitStats,
    duration: std::time::Duration,
) {
    if !super::super::perf_logging_enabled() || duration.as_millis() <= 5 {
        return;
    }

    eprintln!(
        "{} [render] strata {strata_idx}: {frame_count} frames, {duration:.1?} (cached={} emitted={})",
        crate::logging::global_elapsed_prefix(),
        stats.cached,
        stats.emitted
    );
}

/// Emit one frame's quads into the batch. Returns true if quads were emitted.
#[allow(clippy::too_many_arguments)]
fn emit_one_frame(
    batch: &mut QuadBatch,
    id: u64,
    rect: crate::LayoutRect,
    clip_rect: Option<crate::LayoutRect>,
    eff_alpha: f32,
    registry: &crate::widget::WidgetRegistry,
    pressed_frame: Option<u64>,
    hovered_frame: Option<u64>,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: &HashMap<u64, crate::lua_api::message_frame::MessageFrameData>,
    tooltip_data: &HashMap<u64, TooltipRenderData>,
    quest_blobs: &HashMap<u64, crate::lua_api::state::QuestBlobState>,
    statusbar_fills: &HashMap<u64, super::super::statusbar::StatusBarFill>,
    elapsed_secs: f64,
) -> bool {
    let Some(frame) = registry.get(id) else {
        return false;
    };

    let no_visible_ids: Option<FxHashSet<u64>> = None;
    if super::super::button_vis::should_skip_frame(
        frame,
        id,
        eff_alpha,
        &no_visible_ids,
        registry,
        pressed_frame,
        hovered_frame,
    ) {
        return false;
    }
    if !has_renderable_size(frame, rect) {
        return false;
    }

    let bounds = Rectangle::new(
        Point::new(rect.x * UI_SCALE, rect.y * UI_SCALE),
        Size::new(rect.width * UI_SCALE, rect.height * UI_SCALE),
    );
    let clip_bounds = clip_rect.map(|rect| {
        Rectangle::new(
            Point::new(rect.x * UI_SCALE, rect.y * UI_SCALE),
            Size::new(rect.width * UI_SCALE, rect.height * UI_SCALE),
        )
    });
    emit_frame_quads(
        batch,
        text_ctx,
        FrameQuadEmit {
            id,
            widget: frame,
            bounds,
            clip_bounds,
            bar_fill: statusbar_fills.get(&id),
            pressed_frame,
            hovered_frame,
            message_frames: Some(message_frames),
            tooltip_data: Some(tooltip_data),
            quest_blobs: Some(quest_blobs),
            registry,
            elapsed_secs,
            eff_alpha,
        },
    );
    true
}

fn has_renderable_size(frame: &crate::widget::Frame, rect: crate::LayoutRect) -> bool {
    let is_fontstring = matches!(
        frame.widget_type,
        WidgetType::FontString | WidgetType::SimpleHTML
    );
    let is_line = matches!(frame.widget_type, WidgetType::Line);
    !((rect.height <= 0.0 && !is_line) || (rect.width <= 0.0 && !is_fontstring && !is_line))
}

/// Emit quads for a strata bucket with per-frame snapshot caching.
///
/// For frames not in `dirty_ids` that have a cached snapshot, appends the
/// cached data (fast memcpy). Dirty or uncached frames are emitted fresh
/// and their snapshots recorded for future incremental rebuilds.
#[allow(clippy::too_many_arguments)]
fn emit_strata_cached(
    batch: &mut QuadBatch,
    snapshots: &mut HashMap<u64, FrameQuadSnapshot>,
    bucket: &[u64],
    dirty_ids: Option<&FxHashSet<u64>>,
    registry: &crate::widget::WidgetRegistry,
    screen_size: (f32, f32),
    pressed_frame: Option<u64>,
    hovered_frame: Option<u64>,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: &HashMap<u64, crate::lua_api::message_frame::MessageFrameData>,
    tooltip_data: &HashMap<u64, TooltipRenderData>,
    quest_blobs: &HashMap<u64, crate::lua_api::state::QuestBlobState>,
    elapsed_secs: f64,
) -> EmitStats {
    let render_list = build_render_list(bucket, registry, screen_size);
    let statusbar_fills = collect_statusbar_fills(&render_list, registry);
    let mut stats = EmitStats {
        cached: 0,
        emitted: 0,
    };

    for &(id, rect, clip_rect, eff_alpha) in &render_list {
        if try_use_cached(batch, snapshots, dirty_ids, registry, id) {
            stats.cached += 1;
            continue;
        }

        let before = snapshot_offsets(batch);
        let emitted = emit_one_frame(
            batch,
            id,
            rect,
            clip_rect,
            eff_alpha,
            registry,
            pressed_frame,
            hovered_frame,
            text_ctx,
            message_frames,
            tooltip_data,
            quest_blobs,
            &statusbar_fills,
            elapsed_secs,
        );
        snapshots.insert(
            id,
            batch.take_snapshot_since(before.0, before.1, before.2, before.3),
        );
        if emitted {
            stats.emitted += 1;
        }
    }

    stats
}

/// Try to append a cached snapshot for a clean frame. Returns true on hit.
fn try_use_cached(
    batch: &mut QuadBatch,
    snapshots: &HashMap<u64, FrameQuadSnapshot>,
    dirty_ids: Option<&FxHashSet<u64>>,
    registry: &crate::widget::WidgetRegistry,
    id: u64,
) -> bool {
    let Some(dirty_ids) = dirty_ids else {
        return false;
    };
    if frame_or_ancestor_is_dirty(id, dirty_ids, registry) {
        return false;
    }

    let Some(snapshot) = snapshots.get(&id) else {
        return false;
    };
    batch.append_snapshot(snapshot);
    true
}

fn frame_or_ancestor_is_dirty(
    id: u64,
    dirty_ids: &FxHashSet<u64>,
    registry: &crate::widget::WidgetRegistry,
) -> bool {
    let mut current_id = Some(id);
    while let Some(frame_id) = current_id {
        if dirty_ids.contains(&frame_id) {
            return true;
        }
        current_id = registry.get(frame_id).and_then(|frame| frame.parent_id);
    }
    false
}

fn snapshot_offsets(batch: &QuadBatch) -> (usize, usize, usize, usize) {
    (
        batch.vertices.len(),
        batch.indices.len(),
        batch.texture_requests.len(),
        batch.mask_texture_requests.len(),
    )
}

pub(super) fn prune_irrelevant_dirty_strata(
    dirty: u16,
    dirty_ids: Option<&FxHashSet<u64>>,
    strata_buckets: Option<&[Vec<u64>]>,
    cached_strata: &StrataBatchCache,
    snapshot_cache: &StrataSnapshotCache,
) -> u16 {
    let Some(dirty_ids) = dirty_ids else {
        return dirty;
    };
    let Some(strata_buckets) = strata_buckets else {
        return dirty;
    };

    let mut pruned = dirty;
    for strata_idx in 0..FrameStrata::COUNT {
        let strata_bit = 1u16 << strata_idx;
        if dirty & strata_bit == 0 {
            continue;
        }
        if strata_needs_rebuild(
            strata_idx,
            dirty_ids,
            strata_buckets,
            cached_strata,
            snapshot_cache,
        ) {
            continue;
        }
        pruned &= !strata_bit;
    }

    pruned
}

fn strata_needs_rebuild(
    strata_idx: usize,
    dirty_ids: &FxHashSet<u64>,
    strata_buckets: &[Vec<u64>],
    cached_strata: &StrataBatchCache,
    snapshot_cache: &StrataSnapshotCache,
) -> bool {
    let Some(_) = cached_strata[strata_idx].as_ref() else {
        return true;
    };
    let Some(snapshots) = snapshot_cache[strata_idx].as_ref() else {
        return true;
    };

    let bucket = strata_buckets
        .get(strata_idx)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    if bucket.iter().any(|id| dirty_ids.contains(id)) {
        return true;
    }

    dirty_ids.iter().any(|id| snapshots.contains_key(id))
}

/// Rebuild cached strata batches for the given dirty mask and frame IDs.
///
/// This drives the same incremental snapshot path the live renderer uses:
/// clean frames append cached snapshots, while dirty frames re-emit fresh quads.
#[allow(clippy::too_many_arguments)]
pub fn rebuild_dirty_strata_batches_for_registry(
    strata_cache: &mut StrataBatchCache,
    snapshot_cache: &mut StrataSnapshotCache,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    dirty: u16,
    dirty_ids: Option<&FxHashSet<u64>>,
    size: Size,
    strata_buckets: &[Vec<u64>],
    widgets: &crate::widget::WidgetRegistry,
    pressed_frame: Option<u64>,
    hovered_frame: Option<u64>,
    message_frames: &HashMap<u64, crate::lua_api::MessageFrameData>,
    tooltip_data: &HashMap<u64, TooltipRenderData>,
    quest_blobs: &HashMap<u64, crate::lua_api::state::QuestBlobState>,
    elapsed_secs: f64,
) {
    rebuild_strata_batches(
        strata_cache,
        snapshot_cache,
        text_ctx,
        RebuildStrataBatches {
            dirty,
            dirty_ids,
            size,
            strata_buckets,
            widgets,
            pressed_frame,
            hovered_frame,
            message_frames,
            tooltip_data,
            quest_blobs,
            elapsed_secs,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::frame_or_ancestor_is_dirty;
    use crate::widget::{Frame, WidgetRegistry, WidgetType};
    use rustc_hash::FxHashSet;

    #[test]
    fn cached_descendants_of_dirty_parents_are_not_clean() {
        let mut registry = WidgetRegistry::new();

        let root = Frame::new(WidgetType::Frame, Some("Root".to_string()), None);
        let root_id = root.id;
        registry.register(root);

        let parent = Frame::new(
            WidgetType::Frame,
            Some("Tooltip".to_string()),
            Some(root_id),
        );
        let parent_id = parent.id;
        registry.register(parent);
        registry.add_child(root_id, parent_id);

        let child = Frame::new(
            WidgetType::Frame,
            Some("TooltipNineSlice".to_string()),
            Some(parent_id),
        );
        let child_id = child.id;
        registry.register(child);
        registry.add_child(parent_id, child_id);

        let dirty_ids = FxHashSet::from_iter([parent_id]);

        assert!(
            frame_or_ancestor_is_dirty(child_id, &dirty_ids, &registry),
            "cached child snapshots must be discarded when the parent frame is dirty"
        );
    }
}
