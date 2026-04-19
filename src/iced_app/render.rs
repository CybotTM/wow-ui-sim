//! Shader rendering implementation.

use iced::mouse;
use iced::widget::shader;
use iced::{Event, Point, Rectangle, Size};

use rustc_hash::FxHashSet;

use crate::render::font::WowFontSystem;
use crate::render::glyph::GlyphAtlas;
use crate::render::{
    GpuBcTextureData, GpuTextureData, QuadBatch, WowUiPrimitive, load_texture_or_crop,
};

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
        mouse::Event::CursorMoved { position } if bounds.contains(*position) => {
            let local = Point::new(position.x - bounds.x, position.y - bounds.y);
            Some(shader::Action::publish(Message::CanvasEvent(
                CanvasMessage::MouseMove(local),
            )))
        }
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
        let t0 = std::time::Instant::now();
        let (mut dirty_strata, _) = self.get_or_rebuild_quads(size);
        let quad_dur = t0.elapsed();

        let overlay = self.build_overlay();
        let (mut textures, mut bc_textures, tex_dur) =
            self.load_all_textures(&dirty_strata, &overlay);

        if had_textures_pending {
            self.recover_pending_textures(&mut dirty_strata, &mut textures, &mut bc_textures);
        }

        log_slow_draw(quad_dur, tex_dur, textures.len(), bc_textures.len());

        self.update_frame_time_avg(start.elapsed());

        let mut primitive = WowUiPrimitive {
            strata_batches: dirty_strata,
            overlay,
            clear_color: [0.10, 0.11, 0.14, 1.0],
            textures,
            bc_textures,
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

use crate::widget::FrameStrata;
use std::sync::Arc;

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
        textures: &mut Vec<GpuTextureData>,
        bc_textures: &mut Vec<GpuBcTextureData>,
    ) {
        let cached = self.cached_strata_quads.borrow();
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(10);
        let mut exhausted = false;
        for i in 0..dirty_strata.len() {
            if dirty_strata[i].is_none()
                && let Some(batch) = &cached[i]
            {
                let (extra, extra_bc, _scan_elapsed, _load_elapsed, _telemetry, hit) =
                    self.load_new_textures_budgeted(batch, deadline);
                textures.extend(extra);
                bc_textures.extend(extra_bc);
                exhausted |= hit;
                dirty_strata[i] = Some(batch.clone());
            }
        }
        if exhausted {
            self.textures_pending.set(true);
        }
    }

    pub(crate) fn preload_initial_texture_requests(&self) {
        self.preload_current_render_requests(None);
    }

    fn current_texture_request_batches(&self) -> [Option<Arc<QuadBatch>>; FrameStrata::COUNT] {
        let strata = self.cached_strata_quads.borrow();
        std::array::from_fn(|i| strata[i].clone())
    }

    pub(crate) fn preload_current_render_requests_preserving_dirty(
        &self,
        budget: Option<std::time::Duration>,
    ) {
        let dirty_before = self.strata_dirty.get();
        let pending_ids_before = self.pending_dirty_ids.borrow().clone();
        self.preload_current_render_requests(budget);
        if dirty_before != 0 {
            self.mark_strata_dirty(dirty_before);
            *self.pending_dirty_ids.borrow_mut() = pending_ids_before;
        }
    }

    pub(crate) fn preload_current_render_requests(&self, budget: Option<std::time::Duration>) {
        let dirty_strata = self.current_texture_request_batches();
        let overlay = self.build_overlay();
        let paths = textures::collect_texture_request_paths(&dirty_strata, &overlay);

        let env = self.env.borrow();
        let is_glue_screen = env.state().borrow().screen_kind.is_glue();
        drop(env);

        let mut tex_mgr = self.texture_manager.borrow_mut();
        let mut remaining = false;
        let deadline = match budget {
            Some(budget) => Some(std::time::Instant::now() + budget),
            None => (!is_glue_screen)
                .then(|| std::time::Instant::now() + std::time::Duration::from_millis(250)),
        };

        remaining |= self.preload_queued_texture_requests(&mut tex_mgr, deadline);

        if !remaining {
            for path in &paths {
                if let Some(deadline) = deadline
                    && std::time::Instant::now() >= deadline
                {
                    remaining = true;
                    break;
                }
                preload_texture_request_source(&mut tex_mgr, path);
            }
        }

        let gpu_backlog = self.has_pending_gpu_texture_requests(&paths);
        self.textures_pending.set(remaining || gpu_backlog);
    }

    fn preload_queued_texture_requests(
        &self,
        tex_mgr: &mut crate::texture::TextureManager,
        deadline: Option<std::time::Instant>,
    ) -> bool {
        let queued_paths = {
            let env = self.env.borrow();
            env.state().borrow_mut().drain_texture_preloads()
        };
        if queued_paths.is_empty() {
            return false;
        }

        for (index, path) in queued_paths.iter().enumerate() {
            if let Some(deadline) = deadline
                && std::time::Instant::now() >= deadline
            {
                let env = self.env.borrow();
                env.state()
                    .borrow_mut()
                    .enqueue_texture_preloads(queued_paths[index..].iter().cloned());
                return true;
            }
            preload_texture_request_source(tex_mgr, path);
        }

        false
    }

    fn has_pending_gpu_texture_requests(&self, paths: &[String]) -> bool {
        let uploaded = self.gpu_uploaded_textures.borrow();
        let failed = self.gpu_failed_textures.borrow();
        paths
            .iter()
            .any(|path| !uploaded.contains(path) && !failed.contains(path))
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
        let dirty_ids = self.pending_dirty_ids.borrow_mut().take();
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
        effective_dirty
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
        state.ensure_layout_rects();
        let layout_dur = t0.elapsed();
        super::tooltip::update_tooltip_sizes(&mut state, font_sys);
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

fn preload_texture_request_source(tex_mgr: &mut crate::texture::TextureManager, path: &str) {
    if path.contains("@crop:") {
        let _ = load_texture_or_crop(tex_mgr, path);
        return;
    }
    if crate::render::shader::atlas::is_bc_supported() && tex_mgr.load_bc(path).is_some() {
        return;
    }
    let _ = tex_mgr.load(path);
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn build_test_app_with_textures(textures_path: &Path) -> App {
        let env = Rc::new(RefCell::new(
            WowLuaEnv::new().expect("Failed to create Lua environment"),
        ));
        env.borrow().set_screen_mode(ScreenKind::Game);

        let texture_manager = Rc::new(RefCell::new(TextureManager::new(textures_path)));
        let font_system = Rc::new(RefCell::new(WowFontSystem::new(&std::path::PathBuf::from(
            crate::iced_app::app::DEFAULT_FONTS_PATH,
        ))));
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
        let temp_dir = tempdir().unwrap();
        let app = build_test_app_with_textures(temp_dir.path());
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
    fn request_preload_map_warms_map_art_and_overlay_textures() {
        let Some((map_id, art_path, overlay_path)) = first_map_with_art_and_overlay_paths() else {
            eprintln!("Skipping test: no map with both art and exploration overlay textures found");
            return;
        };

        let temp_dir = tempdir().unwrap();
        write_test_texture(temp_dir.path(), &art_path, [0x22, 0x66, 0xaa, 0xff]);
        write_test_texture(temp_dir.path(), &overlay_path, [0xdd, 0xaa, 0x33, 0xff]);

        let app = build_test_app_with_textures(temp_dir.path());
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
}

#[cfg(test)]
mod pending_texture_tests;
