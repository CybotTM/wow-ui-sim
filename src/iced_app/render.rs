//! Shader rendering implementation.

use iced::mouse;
use iced::widget::shader;
use iced::{Event, Point, Rectangle, Size, window};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::render::font::WowFontSystem;
use crate::render::glyph::GlyphAtlas;
use crate::render::{QuadBatch, WowUiPrimitive, load_texture_or_crop};

use super::Message;
use super::app::App;
use super::frame_collect::collect_hittable_frames;
use super::state::CanvasMessage;
use super::strata_emit::build_hittable_rects;

mod rebuild;

#[path = "render_textures.rs"]
mod textures;

use rebuild::prune_irrelevant_dirty_strata;
pub use rebuild::rebuild_dirty_strata_batches_for_registry;

/// Map a mouse event inside `bounds` to a canvas message action.
fn handle_mouse_event(
    mouse_event: &mouse::Event,
    bounds: Rectangle,
    cursor: mouse::Cursor,
) -> Option<shader::Action<Message>> {
    match mouse_event {
        mouse::Event::CursorMoved { position } => {
            if bounds.contains(*position) {
                let local = Point::new(position.x - bounds.x, position.y - bounds.y);
                Some(shader::Action::publish(Message::CanvasEvent(
                    CanvasMessage::MouseMove(local),
                )))
            } else {
                Some(shader::Action::publish(Message::CanvasEvent(
                    CanvasMessage::MouseLeave,
                )))
            }
        }
        mouse::Event::CursorLeft => Some(shader::Action::publish(Message::CanvasEvent(
            CanvasMessage::MouseLeave,
        ))),
        mouse::Event::ButtonPressed(mouse::Button::Left) => cursor
            .position_in(bounds)
            .map(|p| shader::Action::publish(Message::CanvasEvent(CanvasMessage::MouseDown(p)))),
        mouse::Event::ButtonReleased(mouse::Button::Left) => cursor
            .position_in(bounds)
            .map(|p| shader::Action::publish(Message::CanvasEvent(CanvasMessage::MouseUp(p)))),
        mouse::Event::ButtonPressed(mouse::Button::Right) => cursor.position_in(bounds).map(|p| {
            shader::Action::publish(Message::CanvasEvent(CanvasMessage::RightMouseDown(p)))
        }),
        mouse::Event::ButtonReleased(mouse::Button::Right) => cursor
            .position_in(bounds)
            .map(|p| shader::Action::publish(Message::CanvasEvent(CanvasMessage::RightMouseUp(p)))),
        mouse::Event::ButtonPressed(mouse::Button::Middle) => cursor
            .position_in(bounds)
            .map(|p| shader::Action::publish(Message::CanvasEvent(CanvasMessage::MiddleClick(p)))),
        mouse::Event::WheelScrolled { delta } => {
            let dy = match delta {
                mouse::ScrollDelta::Lines { y, .. } => *y,
                mouse::ScrollDelta::Pixels { y, .. } => *y / 30.0,
            };
            Some(shader::Action::publish(Message::Scroll(0.0, dy)))
        }
        _ => None,
    }
}

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
            Event::Mouse(me) => handle_mouse_event(me, bounds, cursor),
            Event::Window(window::Event::Unfocused | window::Event::Closed) => Some(
                shader::Action::publish(Message::CanvasEvent(CanvasMessage::MouseLeave)),
            ),
            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive {
        self.set_main_thread_phase("draw");
        let start = std::time::Instant::now();
        self.frame_count.set(self.frame_count.get() + 1);

        let size = bounds.size();
        self.screen_size.set(size);
        self.sync_screen_size_to_state(size);
        let had_textures_pending = self.textures_pending.get();
        let dirty_before = self.strata_dirty.get();
        let t0 = std::time::Instant::now();
        let (mut dirty_strata, _) = self.get_or_rebuild_quads(size);
        let quad_dur = t0.elapsed();

        let overlay = self.build_overlay();
        let (textures, bc_textures, tex_dur, texture_requests) =
            self.load_all_textures(&dirty_strata, &overlay);

        if had_textures_pending {
            self.recover_pending_textures(&mut dirty_strata, &texture_requests);
        }

        log_slow_draw(quad_dur, tex_dur, textures.len(), bc_textures.len());
        if crate::logging::gui_trace_enabled() {
            let ready_count = texture_requests
                .lock()
                .map(|tracker| tracker.ready_count())
                .unwrap_or_default();
            crate::logging::eprintln_gui_trace(&format!(
                "draw dirty_before=0x{dirty_before:x} had_pending={} ready={ready_count} dirty_batches={} new_rgba={} new_bc={}",
                had_textures_pending,
                dirty_strata.iter().filter(|batch| batch.is_some()).count(),
                textures.len(),
                bc_textures.len()
            ));
        }

        self.record_draw_time(start.elapsed());

        let mut primitive = WowUiPrimitive {
            strata_batches: dirty_strata,
            overlay,
            clear_color: [0.10, 0.11, 0.14, 1.0],
            textures,
            bc_textures,
            glyph_atlas_data: None,
            glyph_atlas_size: 0,
            texture_requests: Some(texture_requests),
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

use crate::widget::FrameStrata;
use std::sync::Arc;
use std::sync::OnceLock;

const TEXTURE_PRELOAD_LOG_ENV: &str = "WOW_SIM_LOG_TEXTURE_PRELOAD";
const TEXTURE_PRELOAD_SAMPLE_LIMIT: usize = 4;

#[derive(Debug, Default)]
struct TexturePreloadPassTelemetry {
    elapsed: std::time::Duration,
    budget: Option<std::time::Duration>,
    queued: usize,
    loaded: usize,
    remaining: usize,
    remaining_sample: Vec<String>,
    pending: bool,
}

#[derive(Debug, Default)]
struct QueuedTexturePreloadProgress {
    total: usize,
    loaded: usize,
    remaining: usize,
    remaining_sample: Vec<String>,
}

fn log_slow_draw(
    quad_dur: std::time::Duration,
    tex_dur: std::time::Duration,
    rgba_count: usize,
    bc_count: usize,
) {
    if !crate::logging::texture_load_debug_enabled() {
        return;
    }
    if quad_dur.as_millis() > 10 || tex_dur.as_millis() > 10 {
        eprintln!(
            "{} [draw] quads={quad_dur:.1?} textures={tex_dur:.1?} (new={} rgba={} bc={})",
            crate::logging::global_elapsed_prefix(),
            rgba_count + bc_count,
            rgba_count,
            bc_count,
        );
    }
}

impl App {
    /// Scan cached (non-dirty) strata for unresolved texture requests and
    /// include them in the primitive so `prepare()` re-resolves their refs
    /// against the updated GPU atlas.
    fn recover_pending_textures(
        &self,
        dirty_strata: &mut [Option<Arc<QuadBatch>>; FrameStrata::COUNT],
        texture_requests: &Arc<
            std::sync::Mutex<crate::render::shader::primitive::TextureRequestTracker>,
        >,
    ) {
        self.prune_completed_pending_texture_paths();
        let cached = self.cached_strata_quads.borrow();
        let strata_pending = self.strata_pending_texture_requests.borrow();
        let mut texture_requests = texture_requests.lock().unwrap();
        let mut reinjected = false;
        for i in 0..dirty_strata.len() {
            if dirty_strata[i].is_none()
                && let Some(batch) = &cached[i]
                && strata_pending[i]
                    .values()
                    .any(|requests| requests.iter().any(|request| request.handle.is_pending()))
            {
                texture_requests.register_batch(batch);
                dirty_strata[i] = Some(batch.clone());
                reinjected = true;
            }
        }
        if reinjected {
            self.textures_pending.set(true);
        }
    }

    pub(crate) fn preload_initial_texture_requests(&self) {
        let _ = self.preload_current_render_requests(None);
    }

    pub(crate) fn preload_current_render_requests_preserving_dirty(
        &self,
        budget: Option<std::time::Duration>,
    ) -> bool {
        let dirty_before = self.strata_dirty.get();
        let pending_ids_before = self.pending_dirty_ids.borrow().clone();
        let redraw_needed = self.preload_current_render_requests(budget);
        if dirty_before != 0 {
            self.mark_strata_dirty(dirty_before);
            *self.pending_dirty_ids.borrow_mut() = pending_ids_before;
        }
        redraw_needed
    }

    pub(crate) fn preload_current_render_requests(
        &self,
        budget: Option<std::time::Duration>,
    ) -> bool {
        let started = std::time::Instant::now();
        let log_preload = texture_preload_logging_enabled();
        let pending_before = self.textures_pending.get();
        let mut telemetry = TexturePreloadPassTelemetry {
            budget,
            pending: pending_before,
            ..Default::default()
        };
        let redraw_needed;

        let env = self.env.borrow();
        let is_glue_screen = env.state().borrow().screen_kind.is_glue();
        drop(env);

        let mut tex_mgr = self.texture_manager.borrow_mut();
        let deadline = match budget {
            Some(budget) => Some(std::time::Instant::now() + budget),
            None => (!is_glue_screen)
                .then(|| std::time::Instant::now() + std::time::Duration::from_millis(250)),
        };

        let queued_progress =
            self.preload_queued_texture_requests(&mut tex_mgr, deadline, log_preload);
        telemetry.queued = queued_progress.total;
        telemetry.loaded = queued_progress.loaded;
        telemetry.remaining = queued_progress.remaining;
        telemetry.remaining_sample = queued_progress.remaining_sample;
        let draw_pending = self.cached_render_requests_still_pending();
        if queued_progress.total != 0 {
            telemetry.pending = queued_progress.remaining != 0 || draw_pending;
            self.textures_pending.set(telemetry.pending);
            redraw_needed = queued_progress.loaded != 0 || (!pending_before && telemetry.pending);
        } else {
            telemetry.pending = draw_pending;
            self.textures_pending.set(draw_pending);
            redraw_needed = !pending_before && draw_pending;
        }
        telemetry.elapsed = started.elapsed();
        if log_preload {
            eprintln!("{}", format_texture_preload_log(&telemetry));
        }
        redraw_needed
    }

    fn preload_queued_texture_requests(
        &self,
        tex_mgr: &mut crate::texture::TextureManager,
        deadline: Option<std::time::Instant>,
        collect_samples: bool,
    ) -> QueuedTexturePreloadProgress {
        let queued_paths = {
            let env = self.env.borrow();
            env.state().borrow_mut().drain_texture_preloads()
        };
        if queued_paths.is_empty() {
            return QueuedTexturePreloadProgress::default();
        }
        let mut progress = QueuedTexturePreloadProgress {
            total: queued_paths.len(),
            ..Default::default()
        };

        for (index, path) in queued_paths.iter().enumerate() {
            if let Some(deadline) = deadline
                && std::time::Instant::now() >= deadline
            {
                let env = self.env.borrow();
                env.state()
                    .borrow_mut()
                    .enqueue_texture_preloads(queued_paths[index..].iter().cloned());
                progress.remaining = queued_paths.len().saturating_sub(index);
                if collect_samples {
                    progress.remaining_sample =
                        sample_texture_paths(&queued_paths[index..], TEXTURE_PRELOAD_SAMPLE_LIMIT);
                }
                return progress;
            }
            preload_texture_request_source(tex_mgr, path);
            progress.loaded += 1;
        }

        progress
    }

    fn record_draw_time(&self, elapsed: std::time::Duration) {
        let elapsed_ms = elapsed.as_secs_f32() * 1000.0;
        self.draw_time_accum_ms
            .set(self.draw_time_accum_ms.get() + elapsed_ms);
    }

    pub(crate) fn cached_render_requests_still_pending(&self) -> bool {
        self.prune_completed_pending_texture_paths();
        if self.pending_texture_path_set.borrow().is_empty() && self.textures_pending.get() {
            self.seed_pending_texture_paths_from_cached_strata();
            self.prune_completed_pending_texture_paths();
        }
        !self.pending_texture_path_set.borrow().is_empty()
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

        let rebuilt = self.rebuild_dirty_strata(size, dirty);
        self.strata_dirty.set(0);
        // Record current size so next frame detects resize.
        *size_cache = Some((size, Arc::new(QuadBatch::new())));

        let strata = self.cached_strata_quads.borrow();
        let result = std::array::from_fn(|i| {
            if rebuilt & (1 << i) != 0 {
                strata[i].clone()
            } else {
                None
            }
        });
        (result, rebuilt != 0)
    }

    /// Rebuild only the strata whose bits are set in `dirty`.
    ///
    /// Stores results in `cached_strata_quads`. Also updates the hittable
    /// grid on first build and syncs layout caches.
    fn rebuild_dirty_strata(&self, size: Size, dirty: u16) -> u16 {
        let dirty_ids = {
            let mut pending_dirty_ids = self.pending_dirty_ids.borrow_mut();
            let drained = pending_dirty_ids.take();
            // A full-rebuild sentinel (`None`) only applies to this rebuild pass.
            // Reset to an empty concrete set so subsequent ticks can recover
            // incremental dirty IDs instead of remaining in permanent full mode.
            *pending_dirty_ids = Some(FxHashSet::default());
            drained
        };
        let effective_dirty = self.prune_dirty_strata(dirty, dirty_ids.as_ref());
        if effective_dirty == 0 {
            return self.finish_without_strata_rebuild();
        }

        let env = self.env.borrow();
        let mut font_sys = self.font_system.borrow_mut();
        let strata_buckets = self.resolve_layout_and_buckets(&env, &mut font_sys);
        let state = env.state().borrow();

        self.emit_dirty_strata_batches(
            effective_dirty,
            dirty_ids.as_ref(),
            size,
            &strata_buckets,
            &state,
            &mut font_sys,
        );
        self.rebuild_hit_grid_if_needed(&state, &strata_buckets, size);
        drop(state);
        self.store_rebuilt_strata_buckets(&env, strata_buckets);
        self.refresh_pending_texture_requests_for_rebuilt_strata(effective_dirty);
        effective_dirty
    }

    fn refresh_pending_texture_requests_for_rebuilt_strata(&self, rebuilt: u16) {
        if rebuilt == 0 {
            return;
        }
        let strata_cache = self.cached_strata_quads.borrow();
        let mut strata_pending = self.strata_pending_texture_requests.borrow_mut();
        for strata_idx in 0..FrameStrata::COUNT {
            if rebuilt & (1 << strata_idx) == 0 {
                continue;
            }
            strata_pending[strata_idx] = strata_cache[strata_idx]
                .as_deref()
                .map_or_else(FxHashMap::default, |batch| {
                    self.collect_pending_texture_requests_for_batch(batch)
                });
        }
        drop(strata_pending);
        drop(strata_cache);
        self.rebuild_pending_texture_queue_from_strata_maps();
    }

    pub(crate) fn seed_pending_texture_paths_from_cached_strata(&self) {
        let strata_cache = self.cached_strata_quads.borrow();
        let mut strata_pending = self.strata_pending_texture_requests.borrow_mut();
        for strata_idx in 0..FrameStrata::COUNT {
            strata_pending[strata_idx] = strata_cache[strata_idx]
                .as_deref()
                .map_or_else(FxHashMap::default, |batch| {
                    self.collect_pending_texture_requests_for_batch(batch)
                });
        }
        drop(strata_pending);
        drop(strata_cache);
        self.rebuild_pending_texture_queue_from_strata_maps();
    }

    fn rebuild_pending_texture_queue_from_strata_maps(&self) {
        let strata_pending = self.strata_pending_texture_requests.borrow();
        let mut pending_by_path: FxHashMap<String, Vec<crate::render::TextureRequest>> =
            FxHashMap::default();
        for strata_map in strata_pending.iter() {
            for (path, requests) in strata_map {
                pending_by_path.entry(path.clone()).or_default().extend(
                    requests
                        .iter()
                        .filter(|request| request.handle.is_pending())
                        .cloned(),
                );
            }
        }
        pending_by_path.retain(|_, requests| !requests.is_empty());
        let mut ordered_paths: Vec<String> = pending_by_path.keys().cloned().collect();
        sort_pending_texture_paths(&mut ordered_paths);

        *self.pending_texture_requests_by_path.borrow_mut() = pending_by_path;
        *self.pending_texture_path_set.borrow_mut() = ordered_paths.iter().cloned().collect();
        *self.pending_texture_path_queue.borrow_mut() =
            std::collections::VecDeque::from(ordered_paths);
    }

    fn prune_completed_pending_texture_paths(&self) {
        let mut changed = false;
        let mut resolved_paths = Vec::new();
        let mut unresolved_paths = Vec::new();
        {
            let mut strata_pending = self.strata_pending_texture_requests.borrow_mut();
            for strata_map in strata_pending.iter_mut() {
                strata_map.retain(|path, requests| {
                    let had_ready = requests.iter().any(|request| request.handle.is_ready());
                    requests.retain(|request| request.handle.is_pending());
                    let keep = !requests.is_empty();
                    if !keep {
                        if had_ready {
                            resolved_paths.push(path.clone());
                        } else {
                            unresolved_paths.push(path.clone());
                        }
                        changed = true;
                    }
                    keep
                });
            }
        }
        if !resolved_paths.is_empty() || !unresolved_paths.is_empty() {
            let mut ready_paths = self.ready_texture_path_cache.borrow_mut();
            for path in resolved_paths {
                ready_paths.insert(path);
            }
            for path in unresolved_paths {
                ready_paths.remove(&path);
            }
        }
        if changed {
            self.rebuild_pending_texture_queue_from_strata_maps();
        }
    }

    fn pop_next_pending_texture_path(&self) -> Option<String> {
        self.pending_texture_path_queue.borrow_mut().pop_front()
    }

    fn requeue_pending_texture_path(&self, path: String) {
        if self.pending_texture_path_set.borrow().contains(&path) {
            self.pending_texture_path_queue.borrow_mut().push_back(path);
        }
    }

    fn pending_path_state(&self, path: &str) -> (bool, bool) {
        let pending_by_path = self.pending_texture_requests_by_path.borrow();
        let Some(requests) = pending_by_path.get(path) else {
            return (false, false);
        };
        let mut has_pending = false;
        let mut should_load = false;
        for request in requests {
            if request.handle.is_pending() {
                has_pending = true;
                if request.handle.should_load() {
                    should_load = true;
                    break;
                }
            }
        }
        (has_pending, should_load)
    }

    fn register_pending_texture_requests_for_path(
        &self,
        path: &str,
        texture_requests: &mut crate::render::shader::primitive::TextureRequestTracker,
    ) {
        let pending_by_path = self.pending_texture_requests_by_path.borrow();
        let Some(requests) = pending_by_path.get(path) else {
            return;
        };
        for request in requests {
            texture_requests.register_request(request);
        }
    }

    fn remove_pending_texture_path(&self, path: &str) {
        if !self.pending_texture_path_set.borrow_mut().remove(path) {
            return;
        }
        let had_ready = self
            .pending_texture_requests_by_path
            .borrow()
            .get(path)
            .is_some_and(|requests| requests.iter().any(|request| request.handle.is_ready()));
        if had_ready {
            self.ready_texture_path_cache
                .borrow_mut()
                .insert(path.to_string());
        } else {
            self.ready_texture_path_cache.borrow_mut().remove(path);
        }
        self.pending_texture_requests_by_path
            .borrow_mut()
            .remove(path);
        for strata_map in self.strata_pending_texture_requests.borrow_mut().iter_mut() {
            strata_map.remove(path);
        }
        self.pending_texture_path_queue
            .borrow_mut()
            .retain(|queued_path| queued_path != path);
    }

    fn collect_pending_texture_requests_for_batch(
        &self,
        batch: &crate::render::QuadBatch,
    ) -> FxHashMap<String, Vec<crate::render::TextureRequest>> {
        let ready_paths = self.ready_texture_path_cache.borrow();
        let mut pending: FxHashMap<String, Vec<crate::render::TextureRequest>> =
            FxHashMap::default();
        for request in batch
            .texture_requests
            .iter()
            .chain(&batch.mask_texture_requests)
        {
            if !request.handle.is_pending() {
                continue;
            }
            if ready_paths.contains(&request.path) {
                request.handle.mark_ready();
                continue;
            }
            pending
                .entry(request.path.clone())
                .or_default()
                .push(request.clone());
        }
        pending
    }

    fn prune_dirty_strata(&self, dirty: u16, dirty_ids: Option<&FxHashSet<u64>>) -> u16 {
        let env = self.env.borrow();
        let state = env.state().borrow();
        let strata_cache = self.cached_strata_quads.borrow();
        let snapshot_cache = self.cached_frame_snapshots.borrow();
        prune_irrelevant_dirty_strata(
            dirty,
            dirty_ids,
            state.strata_buckets.as_deref(),
            &strata_cache,
            &snapshot_cache,
        )
    }

    fn finish_without_strata_rebuild(&self) -> u16 {
        self.apply_hit_grid_changes();
        0
    }

    /// Emit quads for dirty strata into the cache.
    fn emit_dirty_strata_batches(
        &self,
        dirty: u16,
        dirty_ids: Option<&FxHashSet<u64>>,
        size: Size,
        strata_buckets: &[Vec<u64>],
        state: &crate::lua_api::SimState,
        font_sys: &mut WowFontSystem,
    ) {
        let elapsed_secs = state.start_time.elapsed().as_secs_f64();
        let tooltip_data = super::tooltip::collect_tooltip_data(state);
        let mut glyph_atlas = self.glyph_atlas.borrow_mut();
        glyph_atlas.advance_generation();
        let mut text_ctx: Option<(&mut WowFontSystem, &mut GlyphAtlas)> =
            Some((font_sys, &mut glyph_atlas));

        let mut strata_cache = self.cached_strata_quads.borrow_mut();
        let mut snap_cache = self.cached_frame_snapshots.borrow_mut();
        rebuild_dirty_strata_batches_for_registry(
            &mut strata_cache,
            &mut snap_cache,
            &mut text_ctx,
            dirty,
            dirty_ids,
            size,
            strata_buckets,
            &state.widgets,
            self.pressed_frame,
            self.hovered_frame,
            &state.message_frames,
            &tooltip_data,
            &state.quest_blobs,
            elapsed_secs,
        );
    }

    fn store_rebuilt_strata_buckets(
        &self,
        env: &crate::lua_api::WowLuaEnv,
        strata_buckets: Vec<Vec<u64>>,
    ) {
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
        super::tooltip::update_tooltip_sizes(&mut state, font_sys);
        state.ensure_layout_rects();
        let layout_dur = t0.elapsed();
        let t1 = std::time::Instant::now();
        let _ = state.get_strata_buckets();
        let bucket_dur = t1.elapsed();
        if layout_dur.as_millis() > 5 || bucket_dur.as_millis() > 5 {
            eprintln!(
                "{} [rebuild] layout={layout_dur:.1?} buckets={bucket_dur:.1?}",
                crate::logging::global_elapsed_prefix()
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
        let t = std::time::Instant::now();
        let collected = collect_hittable_frames(&state.widgets, buckets);
        let hittable = build_hittable_rects(&collected, &state.widgets);
        let grid = super::hit_grid::HitGrid::new(hittable, size.width, size.height);
        *self.cached_hittable.borrow_mut() = Some(grid);
        let dur = t.elapsed();
        if dur.as_millis() > 5 {
            eprintln!(
                "{} [rebuild] hit_grid={dur:.1?}",
                crate::logging::global_elapsed_prefix()
            );
        }
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
}

pub(crate) fn preload_texture_request_source(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &str,
) {
    if path.contains("@crop:") {
        let _ = load_texture_or_crop(tex_mgr, path);
        return;
    }
    if crate::render::shader::atlas::is_bc_supported() && tex_mgr.load_bc(path).is_some() {
        return;
    }
    let _ = tex_mgr.load(path);
}

fn texture_preload_logging_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os(TEXTURE_PRELOAD_LOG_ENV).is_some())
}

fn format_texture_preload_log(telemetry: &TexturePreloadPassTelemetry) -> String {
    let budget_ms = telemetry
        .budget
        .map(duration_ms)
        .map(|value| format!("{value:.3}"))
        .unwrap_or_else(|| "none".to_string());
    let reason = texture_preload_reason(telemetry);
    format!(
        "[texture-preload] elapsed={:.3}ms budget_ms={budget_ms} queued={} loaded={} remaining={} pending={} reason={} sample={}",
        duration_ms(telemetry.elapsed),
        telemetry.queued,
        telemetry.loaded,
        telemetry.remaining,
        telemetry.pending,
        reason,
        format_texture_path_sample(&telemetry.remaining_sample),
    )
}

fn texture_preload_reason(telemetry: &TexturePreloadPassTelemetry) -> &'static str {
    if telemetry.remaining != 0 {
        return "queued_budget";
    }
    "complete"
}

fn format_texture_path_sample(paths: &[String]) -> String {
    if paths.is_empty() {
        return "-".to_string();
    }
    paths.join(" | ")
}

fn sample_texture_paths(paths: &[String], limit: usize) -> Vec<String> {
    paths.iter().take(limit).cloned().collect()
}

fn sort_pending_texture_paths(paths: &mut [String]) {
    paths.sort_by(|a, b| {
        pending_texture_path_priority(a)
            .cmp(&pending_texture_path_priority(b))
            .then_with(|| a.cmp(b))
    });
}

fn pending_texture_path_priority(path: &str) -> (u8, u8) {
    let is_world_map = path
        .get(..19)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Interface\\WorldMap\\"))
        || path
            .get(..19)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Interface/WorldMap/"));
    let is_crop = path.contains("@crop:");
    (u8::from(!is_world_map), u8::from(is_crop))
}

fn duration_ms(duration: std::time::Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iced_app::{build_hittable_rects, frame_collect::collect_hittable_frames};
    use crate::lua_api::WowLuaEnv;
    use crate::render::{FrameQuadSnapshot, GlyphAtlas, WowFontSystem};
    use crate::screen::ScreenKind;
    use crate::texture::{TextureManager, normalize_wow_path};
    use rustc_hash::FxHashSet;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use std::rc::Rc;
    use tempfile::tempdir;
    use tokio::sync::mpsc;

    fn dirty_mask(strata: usize) -> u16 {
        1u16 << strata
    }

    fn build_test_app() -> App {
        let env = Rc::new(RefCell::new(
            WowLuaEnv::new().expect("Failed to create Lua environment"),
        ));
        env.borrow().set_screen_mode(ScreenKind::Game);

        let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
        let font_system = Rc::new(RefCell::new(WowFontSystem::new()));
        let glyph_atlas = Rc::new(RefCell::new(GlyphAtlas::new()));
        let (_cmd_tx, cmd_rx) = mpsc::channel(1);
        let (_lua_tx, lua_rx) = std::sync::mpsc::channel();

        App::build_app(
            env,
            Vec::new(),
            texture_manager,
            font_system,
            glyph_atlas,
            cmd_rx,
            lua_rx,
            false,
            false,
            None,
            crate::config::SimConfig::default(),
        )
    }

    #[test]
    fn cursor_moved_outside_canvas_publishes_mouse_leave() {
        let bounds = Rectangle::new(Point::new(10.0, 20.0), Size::new(100.0, 80.0));
        let event = mouse::Event::CursorMoved {
            position: Point::new(150.0, 40.0),
        };

        let action = handle_mouse_event(&event, bounds, mouse::Cursor::Unavailable)
            .expect("outside canvas movement should publish a canvas leave event");
        let (message, _, _) = action.into_inner();

        assert!(
            matches!(
                message,
                Some(Message::CanvasEvent(CanvasMessage::MouseLeave))
            ),
            "outside canvas movement should clear hover state"
        );
    }

    #[test]
    fn window_unfocused_publishes_mouse_leave() {
        let app = build_test_app();
        let mut shader_state = ();
        let action = <&App as shader::Program<Message>>::update(
            &&app,
            &mut shader_state,
            &Event::Window(window::Event::Unfocused),
            Rectangle::new(Point::ORIGIN, Size::new(100.0, 80.0)),
            mouse::Cursor::Unavailable,
        )
        .expect("window unfocus should publish a canvas leave event");
        let (message, _, _) = action.into_inner();

        assert!(
            matches!(
                message,
                Some(Message::CanvasEvent(CanvasMessage::MouseLeave))
            ),
            "window unfocus should clear hover state"
        );
    }

    #[test]
    fn format_texture_preload_log_reports_budget_reason_and_samples() {
        let log = format_texture_preload_log(&TexturePreloadPassTelemetry {
            elapsed: std::time::Duration::from_millis(26),
            budget: Some(std::time::Duration::from_millis(25)),
            queued: 2,
            loaded: 1,
            remaining: 1,
            remaining_sample: vec!["queued-a".to_string()],
            pending: true,
        });

        assert!(log.contains("elapsed=26.000ms"));
        assert!(log.contains("budget_ms=25.000"));
        assert!(log.contains("queued=2"));
        assert!(log.contains("loaded=1"));
        assert!(log.contains("remaining=1"));
        assert!(log.contains("pending=true"));
        assert!(log.contains("reason=queued_budget"));
        assert!(log.contains("sample=queued-a"));
    }

    #[test]
    fn texture_preload_reason_reports_complete_after_queue_drains() {
        assert_eq!(
            texture_preload_reason(&TexturePreloadPassTelemetry::default()),
            "complete"
        );
    }

    fn file_data_id_to_wow_path(file_data_id: u32) -> Option<String> {
        let path = crate::manifest_interface_data::get_texture_path(file_data_id)?;
        Some(format!("Interface\\{}", path.replace('/', "\\")))
    }

    fn first_map_with_art_and_overlay_paths() -> Option<(u32, String, String)> {
        for map_id in 1..=10_000 {
            let art_path = crate::map_art::get_map_art(map_id).and_then(|info| {
                info.tiles
                    .iter()
                    .flat_map(|tiles| tiles.iter().copied())
                    .find_map(file_data_id_to_wow_path)
            });
            let overlay_path =
                crate::map_exploration::get_overlays_for_map(map_id).and_then(|overlays| {
                    overlays
                        .iter()
                        .flat_map(|overlay| overlay.file_data_ids.iter().copied())
                        .find_map(file_data_id_to_wow_path)
                });
            if let (Some(art_path), Some(overlay_path)) = (art_path, overlay_path) {
                return Some((map_id, art_path, overlay_path));
            }
        }
        None
    }

    fn write_test_texture(base: &Path, wow_path: &str, color: [u8; 4]) {
        let normalized = normalize_wow_path(wow_path);
        let relative = normalized
            .strip_prefix("Interface/")
            .unwrap_or(normalized.as_str());
        let file_path = base.join(format!("{relative}.webp"));
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let image = image::RgbaImage::from_pixel(4, 4, image::Rgba(color));
        image.save(&file_path).unwrap();
    }

    fn rebuild_after_widget_dirty(app: &App, size: Size) -> u16 {
        let (dirty_mask, dirty_ids) = app
            .env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids();
        app.mark_strata_dirty(dirty_mask);
        app.merge_pending_dirty_ids(dirty_ids);
        app.rebuild_dirty_strata(size, dirty_mask)
    }

    fn texture_request_alphas(app: &App, needle: &str) -> Vec<f32> {
        let mut alphas = Vec::new();
        let strata = app.cached_strata_quads.borrow();
        for batch in strata.iter().flatten() {
            for request in &batch.texture_requests {
                if !request.path.contains(needle) {
                    continue;
                }
                let start = request.vertex_start as usize;
                let end = start + request.vertex_count as usize;
                alphas.extend(
                    batch.vertices[start..end]
                        .iter()
                        .map(|vertex| vertex.color[3]),
                );
            }
        }
        alphas
    }

    fn snapshot_texture_alphas(app: &App, frame_id: u64) -> Vec<f32> {
        let mut alphas = Vec::new();
        let snapshots = app.cached_frame_snapshots.borrow();
        for snapshot in snapshots.iter().flatten() {
            let Some(snapshot) = snapshot.get(&frame_id) else {
                continue;
            };
            for request in &snapshot.texture_requests {
                let start = request.vertex_start as usize;
                let end = start + request.vertex_count as usize;
                alphas.extend(
                    snapshot.vertices[start..end]
                        .iter()
                        .map(|vertex| vertex.color[3]),
                );
            }
        }
        alphas
    }

    fn snapshot_texture_paths(app: &App, frame_id: u64) -> Vec<String> {
        let mut paths = Vec::new();
        let snapshots = app.cached_frame_snapshots.borrow();
        for snapshot in snapshots.iter().flatten() {
            let Some(snapshot) = snapshot.get(&frame_id) else {
                continue;
            };
            paths.extend(
                snapshot
                    .texture_requests
                    .iter()
                    .map(|request| request.path.clone()),
            );
        }
        paths
    }

    fn mark_frames_dirty(app: &App, frame_ids: &[u64]) {
        let env = app.env.borrow();
        let state = env.state().borrow();
        for frame_id in frame_ids {
            state.widgets.mark_visual_dirty(*frame_id);
        }
    }

    fn rebuild_hittable_cache(app: &App, size: Size) {
        let env = app.env.borrow();
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        let strata_buckets = state
            .get_strata_buckets()
            .expect("visible strata buckets should exist")
            .clone();
        let collected = collect_hittable_frames(&state.widgets, &strata_buckets);
        let hittable = build_hittable_rects(&collected, &state.widgets);
        let grid = super::super::hit_grid::HitGrid::new(hittable, size.width, size.height);
        *app.cached_hittable.borrow_mut() = Some(grid);
    }

    #[test]
    fn cached_button_normal_texture_alpha_restores_after_hover_hide() {
        let temp_dir = tempdir().unwrap();
        let normal_path = "Interface/Buttons/UI-Panel-Button-Up";
        write_test_texture(temp_dir.path(), normal_path, [0xaa, 0x22, 0x22, 0xff]);

        let app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                CachedHoverButton = CreateFrame("Button", "CachedHoverButton", UIParent)
                CachedHoverButton:SetSize(100, 40)
                CachedHoverButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -20)
                CachedHoverButton:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
            "#,
            )
            .expect("cached hover button setup should succeed");

        let size = Size::new(320.0, 240.0);
        *app.pending_dirty_ids.borrow_mut() = None;
        app.mark_all_strata_dirty();
        app.rebuild_dirty_strata(size, app.strata_dirty.get());

        let initial_alphas = texture_request_alphas(&app, "UI-Panel-Button-Up");
        assert!(
            initial_alphas.iter().any(|alpha| *alpha == 1.0),
            "initial normal texture should render opaque"
        );

        app.env
            .borrow()
            .exec("CachedHoverButton:GetNormalTexture():SetAlpha(0)")
            .expect("normal texture should hide");
        rebuild_after_widget_dirty(&app, size);
        let hidden_alphas = texture_request_alphas(&app, "UI-Panel-Button-Up");
        assert!(
            hidden_alphas.iter().all(|alpha| *alpha == 0.0),
            "normal texture should render transparent while hover code hides it"
        );

        app.env
            .borrow()
            .exec("CachedHoverButton:GetNormalTexture():SetAlpha(1)")
            .expect("normal texture should restore");
        rebuild_after_widget_dirty(&app, size);
        let restored_alphas = texture_request_alphas(&app, "UI-Panel-Button-Up");
        assert!(
            restored_alphas.iter().any(|alpha| *alpha == 1.0),
            "normal texture should render opaque again after OnLeave restores alpha"
        );
    }

    #[test]
    fn cached_button_state_texture_restores_normal_after_hover() {
        let mut app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                CachedMicroButton = CreateFrame("Button", "CachedMicroButton", UIParent)
                CachedMicroButton:SetSize(32, 40)
                CachedMicroButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -20)
                CachedMicroButton:SetNormalAtlas("UI-HUD-MicroMenu-Professions-Up")
                CachedMicroButton:SetHighlightAtlas("UI-HUD-MicroMenu-Professions-Mouseover", "BLEND")
            "#,
            )
            .expect("cached micro button setup should succeed");

        let (button_id, normal_id, highlight_id) = {
            let env = app.env.borrow();
            let state = env.state().borrow();
            let button = state
                .widgets
                .get_by_name("CachedMicroButton")
                .expect("cached micro button should exist");
            let normal_id = *button
                .children_keys
                .get("NormalTexture")
                .expect("normal texture child should exist");
            let highlight_id = *button
                .children_keys
                .get("HighlightTexture")
                .expect("highlight texture child should exist");
            let normal = state
                .widgets
                .get(normal_id)
                .expect("normal texture child should resolve");
            assert_eq!(
                normal.atlas_tex_coords, normal.tex_coords,
                "button SetNormalAtlas should preserve atlas sub-region metadata on the child texture",
            );
            (button.id, normal_id, highlight_id)
        };

        let size = Size::new(320.0, 240.0);
        *app.pending_dirty_ids.borrow_mut() = None;
        app.mark_all_strata_dirty();
        app.rebuild_dirty_strata(size, app.strata_dirty.get());

        app.env
            .borrow()
            .exec("CachedMicroButton:GetNormalTexture():SetAlpha(0)")
            .expect("normal texture should hide on hover");
        {
            app.env.borrow().state().borrow_mut().hovered_frame = Some(button_id);
        }
        app.hovered_frame = Some(button_id);
        mark_frames_dirty(&app, &[button_id, normal_id, highlight_id]);
        rebuild_after_widget_dirty(&app, size);
        assert!(
            snapshot_texture_alphas(&app, highlight_id)
                .iter()
                .any(|alpha| *alpha == 1.0),
            "hover should emit the highlight texture"
        );

        app.env
            .borrow()
            .exec("CachedMicroButton:GetNormalTexture():SetAlpha(1)")
            .expect("normal texture should restore after hover");
        {
            app.env.borrow().state().borrow_mut().hovered_frame = None;
        }
        app.hovered_frame = None;
        mark_frames_dirty(&app, &[button_id, normal_id, highlight_id]);
        rebuild_after_widget_dirty(&app, size);

        assert!(
            snapshot_texture_alphas(&app, normal_id)
                .iter()
                .any(|alpha| *alpha == 1.0),
            "leaving hover should re-emit the normal texture at full alpha"
        );
        assert!(
            snapshot_texture_paths(&app, normal_id)
                .iter()
                .any(|path| path.contains("@crop:")),
            "restored normal texture should render through an isolated atlas crop"
        );
        assert!(
            snapshot_texture_alphas(&app, highlight_id).is_empty(),
            "leaving hover should remove the highlight texture snapshot"
        );
    }

    #[test]
    fn mouse_leave_rebuild_restores_button_normal_texture() {
        let mut app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                MouseLeaveMicroButton = CreateFrame("Button", "MouseLeaveMicroButton", UIParent)
                MouseLeaveMicroButton:SetSize(32, 40)
                MouseLeaveMicroButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -20)
                MouseLeaveMicroButton:SetNormalAtlas("UI-HUD-MicroMenu-Professions-Up")
                MouseLeaveMicroButton:SetHighlightAtlas("UI-HUD-MicroMenu-Professions-Mouseover", "BLEND")
                MouseLeaveMicroButton:SetScript("OnEnter", function(self)
                    self:GetNormalTexture():SetAlpha(0)
                end)
                MouseLeaveMicroButton:SetScript("OnLeave", function(self)
                    self:GetNormalTexture():SetAlpha(1)
                end)
            "#,
            )
            .expect("mouse-leave micro button setup should succeed");

        let (normal_id, highlight_id) = {
            let env = app.env.borrow();
            let state = env.state().borrow();
            let button = state
                .widgets
                .get_by_name("MouseLeaveMicroButton")
                .expect("mouse-leave micro button should exist");
            let normal_id = *button
                .children_keys
                .get("NormalTexture")
                .expect("normal texture child should exist");
            let highlight_id = *button
                .children_keys
                .get("HighlightTexture")
                .expect("highlight texture child should exist");
            (normal_id, highlight_id)
        };

        let size = Size::new(320.0, 240.0);
        app.mark_all_strata_dirty();
        app.rebuild_dirty_strata(size, app.strata_dirty.get());

        app.handle_mouse_move(Point::new(30.0, 40.0));
        app.rebuild_dirty_strata(size, app.strata_dirty.get());
        assert!(
            snapshot_texture_alphas(&app, highlight_id)
                .iter()
                .any(|alpha| *alpha == 1.0),
            "hover should emit the highlight texture through the real mouse path"
        );

        app.handle_mouse_leave();
        app.rebuild_dirty_strata(size, app.strata_dirty.get());
        assert!(
            snapshot_texture_alphas(&app, normal_id)
                .iter()
                .any(|alpha| *alpha == 1.0),
            "mouse leave should re-emit the normal texture after OnLeave restores alpha"
        );
        assert!(
            snapshot_texture_alphas(&app, highlight_id).is_empty(),
            "mouse leave should remove the highlight texture snapshot"
        );
    }

    #[test]
    fn mouse_up_rebuild_restores_pressed_button_normal_texture() {
        let mut app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                PressedMicroButton = CreateFrame("Button", "PressedMicroButton", UIParent)
                PressedMicroButton:SetSize(32, 40)
                PressedMicroButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -20)
                PressedMicroButton:EnableMouse(true)
                PressedMicroButton:SetNormalAtlas("UI-HUD-MicroMenu-SpecTalents-Up")
                PressedMicroButton:SetPushedAtlas("UI-HUD-MicroMenu-SpecTalents-Down")
            "#,
            )
            .expect("pressed micro button setup should succeed");

        let (normal_id, pushed_id) = {
            let env = app.env.borrow();
            let state = env.state().borrow();
            let button = state
                .widgets
                .get_by_name("PressedMicroButton")
                .expect("pressed micro button should exist");
            let normal_id = *button
                .children_keys
                .get("NormalTexture")
                .expect("normal texture child should exist");
            let pushed_id = *button
                .children_keys
                .get("PushedTexture")
                .expect("pushed texture child should exist");
            (normal_id, pushed_id)
        };

        let size = Size::new(320.0, 240.0);
        app.screen_size.set(size);
        app.mark_all_strata_dirty();
        app.rebuild_dirty_strata(size, app.strata_dirty.get());
        rebuild_hittable_cache(&app, size);

        let click_pos = Point::new(30.0, 40.0);
        app.handle_mouse_down(click_pos);
        app.rebuild_dirty_strata(size, app.strata_dirty.get());
        assert!(
            snapshot_texture_alphas(&app, normal_id).is_empty(),
            "pressed button should remove the normal texture snapshot"
        );
        assert!(
            snapshot_texture_alphas(&app, pushed_id)
                .iter()
                .any(|alpha| *alpha == 1.0),
            "pressed button should emit the pushed texture snapshot"
        );

        app.handle_mouse_up(click_pos);
        app.rebuild_dirty_strata(size, app.strata_dirty.get());
        assert!(
            snapshot_texture_alphas(&app, normal_id)
                .iter()
                .any(|alpha| *alpha == 1.0),
            "mouse up should dirty and re-emit the normal texture snapshot"
        );
        assert!(
            snapshot_texture_alphas(&app, pushed_id).is_empty(),
            "mouse up should remove the pushed texture snapshot"
        );
    }

    #[test]
    fn mouse_leave_clears_pressed_button_texture_state() {
        let mut app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                PressedLeaveMicroButton = CreateFrame("Button", "PressedLeaveMicroButton", UIParent)
                PressedLeaveMicroButton:SetSize(32, 40)
                PressedLeaveMicroButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -20)
                PressedLeaveMicroButton:EnableMouse(true)
                PressedLeaveMicroButton:SetNormalAtlas("UI-HUD-MicroMenu-SpecTalents-Up")
                PressedLeaveMicroButton:SetPushedAtlas("UI-HUD-MicroMenu-SpecTalents-Down")
            "#,
            )
            .expect("pressed leave micro button setup should succeed");

        let normal_id = {
            let env = app.env.borrow();
            let state = env.state().borrow();
            let button = state
                .widgets
                .get_by_name("PressedLeaveMicroButton")
                .expect("pressed leave micro button should exist");
            *button
                .children_keys
                .get("NormalTexture")
                .expect("normal texture child should exist")
        };

        let size = Size::new(320.0, 240.0);
        app.screen_size.set(size);
        app.mark_all_strata_dirty();
        app.rebuild_dirty_strata(size, app.strata_dirty.get());
        rebuild_hittable_cache(&app, size);

        app.handle_mouse_down(Point::new(30.0, 40.0));
        app.rebuild_dirty_strata(size, app.strata_dirty.get());
        app.handle_mouse_leave();
        app.rebuild_dirty_strata(size, app.strata_dirty.get());

        assert!(
            app.pressed_frame.is_none(),
            "mouse leave should clear the app pressed target"
        );
        assert!(
            snapshot_texture_alphas(&app, normal_id)
                .iter()
                .any(|alpha| *alpha == 1.0),
            "mouse leave should dirty and restore the normal texture snapshot"
        );
    }

    #[test]
    fn prune_irrelevant_dirty_strata_skips_cached_strata_without_bucket_or_snapshot_hits() {
        let dirty_ids = FxHashSet::from_iter([99_u64]);
        let buckets = vec![vec![1_u64, 2_u64]];
        let cached = std::array::from_fn(|i| (i == 0).then(|| Arc::new(QuadBatch::new())));
        let snapshots = std::array::from_fn(|i| {
            (i == 0).then(|| HashMap::from([(1_u64, FrameQuadSnapshot::default())]))
        });

        let pruned = prune_irrelevant_dirty_strata(
            dirty_mask(0),
            Some(&dirty_ids),
            Some(&buckets),
            &cached,
            &snapshots,
        );

        assert_eq!(
            pruned, 0,
            "irrelevant dirty ids should not rebuild cached strata"
        );
    }

    #[test]
    fn prune_irrelevant_dirty_strata_keeps_strata_when_snapshot_must_be_removed() {
        let dirty_ids = FxHashSet::from_iter([3_u64]);
        let buckets = vec![vec![1_u64, 2_u64]];
        let cached = std::array::from_fn(|i| (i == 0).then(|| Arc::new(QuadBatch::new())));
        let snapshots = std::array::from_fn(|i| {
            (i == 0).then(|| HashMap::from([(3_u64, FrameQuadSnapshot::default())]))
        });

        let pruned = prune_irrelevant_dirty_strata(
            dirty_mask(0),
            Some(&dirty_ids),
            Some(&buckets),
            &cached,
            &snapshots,
        );

        assert_eq!(
            pruned,
            dirty_mask(0),
            "dirty frames with cached snapshots still need a rebuild to clear old quads"
        );
    }

    #[test]
    fn prune_irrelevant_dirty_strata_keeps_strata_when_bucket_contains_dirty_frame() {
        let dirty_ids = FxHashSet::from_iter([2_u64]);
        let buckets = vec![vec![1_u64, 2_u64]];
        let cached = std::array::from_fn(|i| (i == 0).then(|| Arc::new(QuadBatch::new())));
        let snapshots = std::array::from_fn(|i| {
            (i == 0).then(|| HashMap::from([(1_u64, FrameQuadSnapshot::default())]))
        });

        let pruned = prune_irrelevant_dirty_strata(
            dirty_mask(0),
            Some(&dirty_ids),
            Some(&buckets),
            &cached,
            &snapshots,
        );

        assert_eq!(
            pruned,
            dirty_mask(0),
            "dirty frames still present in the bucket must rebuild that strata"
        );
    }

    #[test]
    fn rebuild_dirty_strata_skips_irrelevant_cached_strata() {
        let app = build_test_app();
        app.cached_strata_quads.borrow_mut()[0] = Some(Arc::new(QuadBatch::new()));
        app.cached_frame_snapshots.borrow_mut()[0] =
            Some(HashMap::from([(1_u64, FrameQuadSnapshot::default())]));
        *app.pending_dirty_ids.borrow_mut() = Some(rustc_hash::FxHashSet::from_iter([99_u64]));
        app.env.borrow().state().borrow_mut().strata_buckets = Some(vec![vec![1_u64, 2_u64]]);

        let rebuilt = app.rebuild_dirty_strata(Size::new(64.0, 64.0), dirty_mask(0));

        assert_eq!(
            rebuilt, 0,
            "irrelevant dirty ids should short-circuit cached strata rebuilds"
        );
    }

    #[test]
    fn rebuild_dirty_strata_resets_consumed_full_rebuild_sentinel() {
        let app = build_test_app();
        app.pending_dirty_ids.borrow_mut().take();

        let _ = app.rebuild_dirty_strata(Size::new(64.0, 64.0), dirty_mask(0));

        let pending = app.pending_dirty_ids.borrow();
        assert!(
            pending.as_ref().is_some_and(FxHashSet::is_empty),
            "after consuming a full-rebuild sentinel, pending dirty IDs must reset to an empty concrete set"
        );
    }

    #[test]
    fn consumed_full_rebuild_sentinel_preserves_next_incremental_fast_path() {
        let app = build_test_app();
        app.cached_strata_quads.borrow_mut()[0] = Some(Arc::new(QuadBatch::new()));
        app.cached_frame_snapshots.borrow_mut()[0] =
            Some(HashMap::from([(1_u64, FrameQuadSnapshot::default())]));
        app.env.borrow().state().borrow_mut().strata_buckets = Some(vec![vec![1_u64, 2_u64]]);

        app.pending_dirty_ids.borrow_mut().take();
        let first = app.rebuild_dirty_strata(Size::new(64.0, 64.0), dirty_mask(0));
        assert_eq!(
            first,
            dirty_mask(0),
            "consuming a full-rebuild sentinel should allow one full rebuild pass"
        );

        app.merge_pending_dirty_ids(Some(FxHashSet::from_iter([99_u64])));
        let second = app.rebuild_dirty_strata(Size::new(64.0, 64.0), dirty_mask(0));
        assert_eq!(
            second, 0,
            "after the sentinel is consumed, unrelated dirty IDs must still prune cached strata rebuilds"
        );
    }

    #[test]
    fn request_preload_map_warms_map_art_and_overlay_textures() {
        let Some((map_id, art_path, overlay_path)) = first_map_with_art_and_overlay_paths() else {
            eprintln!("Skipping test: no map with both art and exploration overlay textures found");
            return;
        };

        let temp_dir = tempdir().unwrap();
        write_test_texture(temp_dir.path(), &art_path, [0x22, 0x66, 0xaa, 0xff]);
        write_test_texture(temp_dir.path(), &overlay_path, [0xdd, 0xaa, 0x33, 0xff]);

        let app = build_test_app();
        app.env
            .borrow()
            .exec(&format!("C_Map.RequestPreloadMap({map_id})"))
            .expect("RequestPreloadMap should succeed");

        assert!(
            app.texture_manager.borrow().get(&art_path).is_none(),
            "map art texture should not already be cached before preload runs"
        );
        assert!(
            app.texture_manager.borrow().get(&overlay_path).is_none(),
            "map overlay texture should not already be cached before preload runs"
        );

        app.preload_initial_texture_requests();

        let tex_mgr = app.texture_manager.borrow();
        assert!(
            tex_mgr.get(&art_path).is_some(),
            "RequestPreloadMap should warm map art tile texture {art_path}"
        );
        assert!(
            tex_mgr.get(&overlay_path).is_some(),
            "RequestPreloadMap should warm exploration overlay texture {overlay_path}"
        );
    }

    #[test]
    fn resolve_layout_and_buckets_recomputes_tooltip_layout_after_sizing() {
        let app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                local owner = CreateFrame("Frame", "TooltipLayoutOwner", UIParent)
                owner:SetSize(100, 50)
                owner:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
                GameTooltip:SetOwner(owner, "ANCHOR_RIGHT")
                GameTooltip:AddLine("Tooltip layout must resize before render buckets lock in")
            "#,
            )
            .expect("tooltip setup should succeed");

        {
            let env = app.env.borrow();
            let mut font_sys = app.font_system.borrow_mut();
            let _ = app.resolve_layout_and_buckets(&env, &mut font_sys);
        }

        let state_ref = app.env.borrow();
        let state = state_ref.state().borrow();
        let tooltip_id = state
            .widgets
            .get_id_by_name("GameTooltip")
            .expect("GameTooltip should exist");
        let tooltip = state
            .widgets
            .get(tooltip_id)
            .expect("GameTooltip frame should exist");
        let tooltip_rect = tooltip
            .layout_rect
            .expect("render prep should resolve the tooltip layout rect");

        assert!(
            (tooltip_rect.width - tooltip.width).abs() < f32::EPSILON,
            "tooltip layout width {} should match sized width {} after render prep",
            tooltip_rect.width,
            tooltip.width
        );
        assert!(
            (tooltip_rect.height - tooltip.height).abs() < f32::EPSILON,
            "tooltip layout height {} should match sized height {} after render prep",
            tooltip_rect.height,
            tooltip.height
        );
    }
}

#[cfg(test)]
mod pending_texture_tests;
