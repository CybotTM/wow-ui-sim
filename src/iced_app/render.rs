//! Shader rendering implementation.

use iced::mouse;
use iced::widget::shader;
use iced::{Event, Point, Rectangle, Size};

use std::collections::{HashMap, HashSet};

use crate::render::FrameQuadSnapshot;
use crate::render::font::WowFontSystem;
use crate::render::glyph::GlyphAtlas;
use crate::render::texture::UI_SCALE;
use crate::render::{GpuTextureData, QuadBatch, WowUiPrimitive, load_texture_or_crop};
use crate::widget::WidgetType;

use super::Message;
use super::app::App;
use super::frame_collect::collect_hittable_frames;
use super::quad_builders::{build_texture_quads, emit_button_highlight, emit_frame_quads};
use super::state::CanvasMessage;
use super::statusbar::collect_statusbar_fills;
use super::strata_emit::{build_hittable_rects, build_render_list};
use super::tooltip::TooltipRenderData;

/// Shader program implementation for GPU rendering of WoW frames.
impl shader::Program<Message> for &App {
    type State = ();
    type Primitive = WowUiPrimitive;

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<shader::Action<Message>> {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { position } => {
                    if bounds.contains(*position) {
                        let local = Point::new(position.x - bounds.x, position.y - bounds.y);
                        return Some(shader::Action::publish(Message::CanvasEvent(
                            CanvasMessage::MouseMove(local),
                        )));
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if let Some(pos) = cursor.position_in(bounds) {
                        return Some(shader::Action::publish(Message::CanvasEvent(
                            CanvasMessage::MouseDown(pos),
                        )));
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    if let Some(pos) = cursor.position_in(bounds) {
                        return Some(shader::Action::publish(Message::CanvasEvent(
                            CanvasMessage::MouseUp(pos),
                        )));
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Right) => {
                    if let Some(pos) = cursor.position_in(bounds) {
                        return Some(shader::Action::publish(Message::CanvasEvent(
                            CanvasMessage::RightMouseDown(pos),
                        )));
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Right) => {
                    if let Some(pos) = cursor.position_in(bounds) {
                        return Some(shader::Action::publish(Message::CanvasEvent(
                            CanvasMessage::RightMouseUp(pos),
                        )));
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Middle) => {
                    if let Some(pos) = cursor.position_in(bounds) {
                        return Some(shader::Action::publish(Message::CanvasEvent(
                            CanvasMessage::MiddleClick(pos),
                        )));
                    }
                }
                mouse::Event::WheelScrolled { delta } => {
                    let dy = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => *y,
                        mouse::ScrollDelta::Pixels { y, .. } => *y / 30.0,
                    };
                    return Some(shader::Action::publish(Message::Scroll(0.0, dy)));
                }
                _ => {}
            },
            Event::Keyboard(_) => {}
            _ => {}
        }
        None
    }

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive {
        let start = std::time::Instant::now();
        self.frame_count.set(self.frame_count.get() + 1);

        let size = bounds.size();
        self.screen_size.set(size);
        self.sync_screen_size_to_state(size);
        let t0 = std::time::Instant::now();
        let (dirty_strata, _) = self.get_or_rebuild_quads(size);
        let quad_dur = t0.elapsed();

        let overlay = self.build_overlay();
        let (textures, tex_dur) = self.load_all_textures(&dirty_strata, &overlay);
        log_slow_draw(quad_dur, tex_dur, textures.len());

        self.update_frame_time_avg(start.elapsed());

        let mut primitive = WowUiPrimitive {
            strata_batches: dirty_strata,
            overlay,
            clear_color: [0.10, 0.11, 0.14, 1.0],
            textures,
            glyph_atlas_data: None,
            glyph_atlas_size: 0,
        };
        self.attach_dirty_glyph_atlas(&mut primitive);
        primitive
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.position_in(bounds).is_some() {
            mouse::Interaction::Hidden
        } else {
            mouse::Interaction::default()
        }
    }
}

/// Rebuild strata batches for all dirty strata indices.
///
/// When `dirty_ids` is `Some`, uses per-frame snapshot cache for incremental
/// rebuild — only re-emitting dirty frames, copying cached quads for the rest.
#[allow(clippy::too_many_arguments)]
fn rebuild_strata_batches(
    strata_cache: &mut [Option<Arc<QuadBatch>>; FrameStrata::COUNT],
    snapshot_cache: &mut [Option<HashMap<u64, FrameQuadSnapshot>>; FrameStrata::COUNT],
    dirty: u16,
    dirty_ids: Option<&HashSet<u64>>,
    size: Size,
    strata_buckets: &[Vec<u64>],
    widgets: &crate::widget::WidgetRegistry,
    pressed_frame: Option<u64>,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: &HashMap<u64, crate::lua_api::MessageFrameData>,
    tooltip_data: &HashMap<u64, super::tooltip::TooltipRenderData>,
    elapsed_secs: f64,
) {
    for i in 0..FrameStrata::COUNT {
        if dirty & (1 << i) == 0 && strata_cache[i].is_some() {
            continue;
        }
        let bucket = strata_buckets.get(i).map(|b| b.as_slice()).unwrap_or(&[]);
        let strata_start = std::time::Instant::now();
        let mut batch = QuadBatch::new();
        if i == 0 {
            emit_marble_background(&mut batch, size);
        }
        let snapshots = snapshot_cache[i].get_or_insert_with(HashMap::new);
        let stats = emit_strata_cached(
            &mut batch,
            snapshots,
            bucket,
            dirty_ids,
            widgets,
            pressed_frame,
            text_ctx,
            message_frames,
            tooltip_data,
            elapsed_secs,
        );
        log_strata_timing(i, bucket.len(), &stats, strata_start.elapsed());
        strata_cache[i] = Some(Arc::new(batch));
    }
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

fn log_strata_timing(i: usize, n: usize, stats: &EmitStats, dur: std::time::Duration) {
    if dur.as_millis() <= 5 {
        return;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs() % 86400;
    let (h, m, s, ms) = (
        secs / 3600,
        (secs % 3600) / 60,
        secs % 60,
        now.subsec_millis(),
    );
    eprintln!(
        "[{h:02}:{m:02}:{s:02}.{ms:03}] [render] strata {i}: {n} frames, {dur:.1?} (cached={} emitted={})",
        stats.cached, stats.emitted
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
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: &HashMap<u64, crate::lua_api::message_frame::MessageFrameData>,
    tooltip_data: &HashMap<u64, TooltipRenderData>,
    statusbar_fills: &HashMap<u64, super::statusbar::StatusBarFill>,
    elapsed_secs: f64,
) -> bool {
    let Some(f) = registry.get(id) else {
        return false;
    };
    let no_vis: Option<HashSet<u64>> = None;
    if super::button_vis::should_skip_frame(
        f,
        id,
        eff_alpha,
        &no_vis,
        registry,
        pressed_frame,
        None,
    ) {
        return false;
    }
    if !has_renderable_size(f, rect) {
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
        id,
        f,
        bounds,
        clip_bounds,
        statusbar_fills.get(&id),
        pressed_frame,
        None,
        text_ctx,
        Some(message_frames),
        Some(tooltip_data),
        registry,
        elapsed_secs,
        eff_alpha,
    );
    true
}

fn has_renderable_size(f: &crate::widget::Frame, rect: crate::LayoutRect) -> bool {
    let is_fontstring = matches!(f.widget_type, WidgetType::FontString);
    let is_line = matches!(f.widget_type, WidgetType::Line);
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
    dirty_ids: Option<&HashSet<u64>>,
    registry: &crate::widget::WidgetRegistry,
    pressed_frame: Option<u64>,
    text_ctx: &mut Option<(&mut WowFontSystem, &mut GlyphAtlas)>,
    message_frames: &HashMap<u64, crate::lua_api::message_frame::MessageFrameData>,
    tooltip_data: &HashMap<u64, TooltipRenderData>,
    elapsed_secs: f64,
) -> EmitStats {
    let render_list = build_render_list(bucket, registry);
    let statusbar_fills = collect_statusbar_fills(&render_list, registry);
    let mut stats = EmitStats {
        cached: 0,
        emitted: 0,
    };

    for &(id, rect, clip_rect, eff_alpha) in &render_list {
        if try_use_cached(batch, snapshots, dirty_ids, id) {
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
            text_ctx,
            message_frames,
            tooltip_data,
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
    dirty_ids: Option<&HashSet<u64>>,
    id: u64,
) -> bool {
    let Some(dirty) = dirty_ids else { return false };
    if dirty.contains(&id) {
        return false;
    }
    let Some(snap) = snapshots.get(&id) else {
        return false;
    };
    batch.append_snapshot(snap);
    true
}

fn snapshot_offsets(batch: &QuadBatch) -> (usize, usize, usize, usize) {
    (
        batch.vertices.len(),
        batch.indices.len(),
        batch.texture_requests.len(),
        batch.mask_texture_requests.len(),
    )
}

use crate::widget::FrameStrata;
use std::sync::Arc;

fn log_slow_draw(quad_dur: std::time::Duration, tex_dur: std::time::Duration, tex_count: usize) {
    if quad_dur.as_millis() > 10 || tex_dur.as_millis() > 10 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = now.as_secs() % 86400;
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;
        let ms = now.subsec_millis();
        eprintln!(
            "[{h:02}:{m:02}:{s:02}.{ms:03}] [draw] quads={quad_dur:.1?} textures={tex_dur:.1?} ({tex_count} new)"
        );
    }
}

impl App {
    pub(crate) fn preload_initial_texture_requests(&self) {
        let size = self.screen_size.get();
        let (dirty_strata, _) = self.get_or_rebuild_quads(size);
        let overlay = self.build_overlay();
        let paths = collect_texture_request_paths(&dirty_strata, &overlay);
        if paths.is_empty() {
            return;
        }

        let env = self.env.borrow();
        let is_glue_screen = env.state().borrow().screen_kind.is_glue();
        drop(env);

        let mut tex_mgr = self.texture_manager.borrow_mut();
        let before = tex_mgr.cache_len();
        let mut warmed = 0usize;
        let mut remaining = false;
        let deadline = (!is_glue_screen)
            .then(|| std::time::Instant::now() + std::time::Duration::from_millis(250));

        for path in &paths {
            if let Some(deadline) = deadline
                && std::time::Instant::now() >= deadline
            {
                remaining = true;
                break;
            }
            if load_texture_or_crop(&mut tex_mgr, path).is_some() {
                warmed += 1;
            }
        }

        self.textures_pending.set(remaining);
        let loaded = tex_mgr.cache_len().saturating_sub(before);
        if warmed > 0 || loaded > 0 {
            eprintln!(
                "[preload] warmed {warmed} render requests ({loaded} new base textures, {} total requests)",
                paths.len()
            );
        }
    }

    fn build_overlay(&self) -> QuadBatch {
        let mut overlay = QuadBatch::new();
        self.append_hover_highlight(&mut overlay);
        if let Some(pos) = self.mouse_position {
            self.append_cursor_item_icon(&mut overlay, pos);
            const CURSOR_SIZE: f32 = 32.0;
            overlay.push_textured_path(
                Rectangle::new(
                    Point::new(pos.x, pos.y),
                    Size::new(CURSOR_SIZE, CURSOR_SIZE),
                ),
                r"Interface\Cursor\Point",
                [1.0, 1.0, 1.0, 1.0],
                crate::render::BlendMode::Alpha,
            );
        }
        overlay
    }

    /// Load textures with a time budget (~50ms). Sets `textures_pending` if more remain.
    fn load_all_textures(
        &self,
        dirty_strata: &[Option<Arc<QuadBatch>>; FrameStrata::COUNT],
        overlay: &QuadBatch,
    ) -> (Vec<GpuTextureData>, std::time::Duration) {
        let t = std::time::Instant::now();
        let deadline = t + std::time::Duration::from_millis(50);
        let mut textures = Vec::new();
        let mut exhausted = false;
        for batch_opt in dirty_strata {
            if exhausted {
                break;
            }
            if let Some(batch) = batch_opt {
                let (loaded, hit) = self.load_new_textures_budgeted(batch, deadline);
                textures.extend(loaded);
                exhausted |= hit;
            }
        }
        if !exhausted {
            let (loaded, hit) = self.load_new_textures_budgeted(overlay, deadline);
            textures.extend(loaded);
            exhausted |= hit;
        }
        self.textures_pending.set(exhausted);
        (textures, t.elapsed())
    }

    fn update_frame_time_avg(&self, elapsed: std::time::Duration) {
        let elapsed_ms = elapsed.as_secs_f32() * 1000.0;
        self.frame_time_ms.set(elapsed_ms);
        let avg = self.frame_time_avg.get();
        self.frame_time_avg.set(0.33 * elapsed_ms + 0.67 * avg);
    }

    /// Return per-strata dirty batches, rebuilding only strata whose bit is
    /// set in `strata_dirty`. Clean strata get `None` — the GPU pipeline
    /// keeps their buffers from the previous frame.
    ///
    /// Returns `(batches, rebuilt)` where `rebuilt` is true when any strata
    /// was re-emitted (used for frame-time measurement).
    fn get_or_rebuild_quads(
        &self,
        size: Size,
    ) -> ([Option<Arc<QuadBatch>>; FrameStrata::COUNT], bool) {
        let mut size_cache = self.cached_quads.borrow_mut();
        let size_changed = size_cache.as_ref().map(|(s, _)| *s != size).unwrap_or(true);

        if size_changed {
            self.mark_all_strata_dirty();
            // Invalidate per-strata cache — screen size changed.
            *self.cached_strata_quads.borrow_mut() = std::array::from_fn(|_| None);
            // Invalidate hit grid — frame positions change with screen size.
            *self.cached_hittable.borrow_mut() = None;
        }

        let dirty = self.strata_dirty.get();
        if dirty == 0 {
            return (std::array::from_fn(|_| None), false);
        }

        self.rebuild_dirty_strata(size, dirty);
        self.strata_dirty.set(0);
        // Record current size so next frame detects resize.
        *size_cache = Some((size, Arc::new(QuadBatch::new())));

        let strata = self.cached_strata_quads.borrow();
        let result = std::array::from_fn(|i| {
            if dirty & (1 << i) != 0 {
                strata[i].clone()
            } else {
                None
            }
        });
        (result, true)
    }

    /// Rebuild only the strata whose bits are set in `dirty`.
    ///
    /// Stores results in `cached_strata_quads`. Also updates the hittable
    /// grid on first build and syncs layout caches.
    fn rebuild_dirty_strata(&self, size: Size, dirty: u16) {
        let dirty_ids = self.pending_dirty_ids.borrow_mut().take();
        let env = self.env.borrow();
        let mut font_sys = self.font_system.borrow_mut();
        let strata_buckets = self.resolve_layout_and_buckets(&env, &mut font_sys);

        let state = env.state().borrow();
        let elapsed_secs = state.start_time.elapsed().as_secs_f64();
        let tooltip_data = super::tooltip::collect_tooltip_data(&state);
        let mut glyph_atlas = self.glyph_atlas.borrow_mut();
        glyph_atlas.advance_generation();
        let mut text_ctx: Option<(&mut WowFontSystem, &mut GlyphAtlas)> =
            Some((&mut font_sys, &mut glyph_atlas));

        let mut strata_cache = self.cached_strata_quads.borrow_mut();
        let mut snap_cache = self.cached_frame_snapshots.borrow_mut();
        rebuild_strata_batches(
            &mut strata_cache,
            &mut snap_cache,
            dirty,
            dirty_ids.as_ref(),
            size,
            &strata_buckets,
            &state.widgets,
            self.pressed_frame,
            &mut text_ctx,
            &state.message_frames,
            &tooltip_data,
            elapsed_secs,
        );
        drop(strata_cache);
        drop(snap_cache);
        self.rebuild_hit_grid_if_needed(&state, &strata_buckets, size);
        drop(state);
        self.apply_hit_grid_changes();

        env.state().borrow_mut().strata_buckets = Some(strata_buckets);
    }

    /// Resolve layout rects and build strata buckets, logging slow phases.
    fn resolve_layout_and_buckets(
        &self,
        env: &crate::lua_api::WowLuaEnv,
        font_sys: &mut WowFontSystem,
    ) -> Vec<Vec<u64>> {
        let mut state = env.state().borrow_mut();
        let t0 = std::time::Instant::now();
        state.ensure_layout_rects();
        let layout_dur = t0.elapsed();
        super::tooltip::update_tooltip_sizes(&mut state, font_sys);
        let t1 = std::time::Instant::now();
        let _ = state.get_strata_buckets();
        let bucket_dur = t1.elapsed();
        if layout_dur.as_millis() > 5 || bucket_dur.as_millis() > 5 {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            let secs = now.as_secs() % 86400;
            let (h, m, s, ms) = (
                secs / 3600,
                (secs % 3600) / 60,
                secs % 60,
                now.subsec_millis(),
            );
            eprintln!(
                "[{h:02}:{m:02}:{s:02}.{ms:03}] [rebuild] layout={layout_dur:.1?} buckets={bucket_dur:.1?}"
            );
        }
        state.strata_buckets.take().unwrap()
    }

    fn rebuild_hit_grid_if_needed(
        &self,
        state: &crate::lua_api::SimState,
        buckets: &[Vec<u64>],
        size: Size,
    ) {
        if self.cached_hittable.borrow().is_some() {
            return;
        }
        // Interaction has to work while textures are still streaming. Building
        // the hit grid here keeps early clicks on glue/login screens from
        // being dropped until texture preloading finishes.
        let t = std::time::Instant::now();
        let collected = collect_hittable_frames(&state.widgets, buckets);
        let hittable = build_hittable_rects(&collected, &state.widgets);
        let grid = super::hit_grid::HitGrid::new(hittable, size.width, size.height);
        *self.cached_hittable.borrow_mut() = Some(grid);
        let dur = t.elapsed();
        if dur.as_millis() > 5 {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            let secs = now.as_secs() % 86400;
            let (h, m, s, ms) = (
                secs / 3600,
                (secs % 3600) / 60,
                secs % 60,
                now.subsec_millis(),
            );
            eprintln!("[{h:02}:{m:02}:{s:02}.{ms:03}] [rebuild] hit_grid={dur:.1?}");
        }
    }

    /// Load textures not yet uploaded to the GPU atlas.
    /// Load new textures from the batch's requests within a time budget.
    /// Returns the loaded textures and whether the deadline was reached.
    fn load_new_textures_budgeted(
        &self,
        quads: &QuadBatch,
        deadline: std::time::Instant,
    ) -> (Vec<GpuTextureData>, bool) {
        let mut textures = Vec::new();
        let mut uploaded = self.gpu_uploaded_textures.borrow_mut();
        let mut failed = self.gpu_failed_textures.borrow_mut();
        let mut tex_mgr = self.texture_manager.borrow_mut();
        let all_requests = quads
            .texture_requests
            .iter()
            .chain(&quads.mask_texture_requests);
        for request in all_requests {
            if uploaded.contains(&request.path) || failed.contains(&request.path) {
                continue;
            }
            if textures
                .iter()
                .any(|t: &GpuTextureData| t.path == request.path)
            {
                continue;
            }
            if !textures.is_empty() && std::time::Instant::now() >= deadline {
                // Skip deadline for CPU-cached textures (preloaded) — no disk I/O.
                let base = request
                    .path
                    .find("@crop:")
                    .map_or(request.path.as_str(), |i| &request.path[..i]);
                if !tex_mgr.is_cached(base) {
                    return (textures, true);
                }
            }
            if let Some(gpu_data) = load_texture_or_crop(&mut tex_mgr, &request.path) {
                uploaded.insert(request.path.clone());
                textures.push(gpu_data);
            } else {
                failed.insert(request.path.clone());
            }
        }
        (textures, false)
    }

    /// Attach glyph atlas data to the primitive if there are new glyphs.
    fn attach_dirty_glyph_atlas(&self, primitive: &mut WowUiPrimitive) {
        let mut ga = self.glyph_atlas.borrow_mut();
        if ga.is_dirty() {
            let (data, size, _) = ga.texture_data();
            primitive.glyph_atlas_data = Some(data.to_vec());
            primitive.glyph_atlas_size = size;
            ga.mark_clean();
        }
    }

    /// Append hover highlight quads for the currently hovered button.
    fn append_hover_highlight(&self, quads: &mut QuadBatch) {
        let Some(hovered_id) = self.hovered_frame else {
            return;
        };
        let env = self.env.borrow();
        let state = env.state().borrow();
        let registry = &state.widgets;
        let Some(f) = registry.get(hovered_id) else {
            return;
        };

        if !matches!(f.widget_type, WidgetType::Button | WidgetType::CheckButton) {
            return;
        }

        let Some(rect) = f.layout_rect else { return };
        if rect.width <= 0.0 || rect.height <= 0.0 {
            return;
        }
        let bounds = Rectangle::new(
            Point::new(rect.x * UI_SCALE, rect.y * UI_SCALE),
            Size::new(rect.width * UI_SCALE, rect.height * UI_SCALE),
        );

        let has_highlight_child = f.children_keys.contains_key("HighlightTexture");
        let is_pressed = self.pressed_frame == Some(hovered_id) || f.button_state == 1;
        if !is_pressed && !has_highlight_child {
            emit_button_highlight(quads, bounds, f, f.alpha);
        }

        if let Some(&ht_id) = f.children_keys.get("HighlightTexture")
            && let Some(ht) = registry.get(ht_id)
        {
            let Some(ht_rect) = ht.layout_rect else {
                return;
            };
            if ht_rect.width > 0.0 && ht_rect.height > 0.0 {
                let ht_bounds = Rectangle::new(
                    Point::new(ht_rect.x * UI_SCALE, ht_rect.y * UI_SCALE),
                    Size::new(ht_rect.width * UI_SCALE, ht_rect.height * UI_SCALE),
                );
                build_texture_quads(quads, ht_bounds, ht, None, ht.alpha);
            }
        }
    }

    /// Render the spell icon attached to the cursor when dragging.
    fn append_cursor_item_icon(&self, overlay: &mut QuadBatch, pos: Point) {
        let env = self.env.borrow();
        let state = env.state().borrow();
        let spell_id = match &state.cursor_item {
            Some(crate::lua_api::state::CursorInfo::Action { spell_id, .. }) => *spell_id,
            Some(crate::lua_api::state::CursorInfo::Spell { spell_id }) => *spell_id,
            None => return,
        };
        let Some(spell) = crate::spells::get_spell(spell_id) else {
            return;
        };
        let Some(path) = crate::manifest_interface_data::get_texture_path(spell.icon_file_data_id)
        else {
            return;
        };
        let tex_path = format!("Interface\\{}", path.replace('/', "\\"));

        const ICON_SIZE: f32 = 32.0;
        // WoW centers the drag icon on the cursor position.
        let icon_bounds = Rectangle::new(
            Point::new(pos.x - ICON_SIZE * 0.5, pos.y - ICON_SIZE * 0.5),
            Size::new(ICON_SIZE, ICON_SIZE),
        );
        overlay.push_textured_path(
            icon_bounds,
            &tex_path,
            [1.0, 1.0, 1.0, 1.0],
            crate::render::BlendMode::Alpha,
        );
    }
}

fn collect_texture_request_paths(
    dirty_strata: &[Option<Arc<QuadBatch>>; FrameStrata::COUNT],
    overlay: &QuadBatch,
) -> Vec<String> {
    let mut paths = Vec::new();
    let mut seen = HashSet::new();
    for batch in dirty_strata.iter().flatten() {
        for request in batch
            .texture_requests
            .iter()
            .chain(&batch.mask_texture_requests)
        {
            if seen.insert(request.path.clone()) {
                paths.push(request.path.clone());
            }
        }
    }
    for request in overlay
        .texture_requests
        .iter()
        .chain(&overlay.mask_texture_requests)
    {
        if seen.insert(request.path.clone()) {
            paths.push(request.path.clone());
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::collect_texture_request_paths;
    use crate::render::{QuadBatch, TextureRequest};
    use crate::widget::FrameStrata;
    use std::sync::Arc;

    fn request(path: &str) -> TextureRequest {
        TextureRequest {
            path: path.to_string(),
            vertex_start: 0,
            vertex_count: 4,
        }
    }

    #[test]
    fn collect_texture_request_paths_deduplicates_across_batches() {
        let mut strata: [Option<Arc<QuadBatch>>; FrameStrata::COUNT] =
            std::array::from_fn(|_| None);
        let mut batch = QuadBatch::new();
        batch.texture_requests.push(request("foo"));
        batch.texture_requests.push(request("foo"));
        batch.mask_texture_requests.push(request("bar"));
        strata[0] = Some(Arc::new(batch));

        let mut overlay = QuadBatch::new();
        overlay.texture_requests.push(request("bar"));
        overlay.texture_requests.push(request("baz"));

        let paths = collect_texture_request_paths(&strata, &overlay);
        assert_eq!(
            paths,
            vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
        );
    }
}
