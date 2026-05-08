use std::sync::OnceLock;

use rustc_hash::FxHashSet;

use super::super::app::App;
use super::PendingTextureRequestsByStrata;
use crate::render::load_texture_or_crop;

const TEXTURE_PRELOAD_LOG_ENV: &str = "WOW_SIM_LOG_TEXTURE_PRELOAD";
const TEXTURE_PRELOAD_SAMPLE_LIMIT: usize = 4;

#[derive(Debug, Default)]
pub(super) struct TexturePreloadPassTelemetry {
    pub(super) elapsed: std::time::Duration,
    pub(super) budget: Option<std::time::Duration>,
    pub(super) queued: usize,
    pub(super) loaded: usize,
    pub(super) remaining: usize,
    pub(super) remaining_sample: Vec<String>,
    pub(super) pending: bool,
}

struct TexturePreloadPass {
    started: std::time::Instant,
    log_enabled: bool,
    telemetry: TexturePreloadPassTelemetry,
}

#[derive(Debug, Default)]
struct QueuedTexturePreloadProgress {
    total: usize,
    loaded: usize,
    remaining: usize,
    remaining_sample: Vec<String>,
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

enum PendingTexturePathPruneResult {
    Keep,
    Resolved(String),
    Unresolved(String),
}

pub(super) fn prune_completed_texture_requests_by_strata(
    strata_pending: &mut PendingTextureRequestsByStrata,
) -> (bool, Vec<String>, Vec<String>) {
    let mut changed = false;
    let mut resolved_paths = Vec::new();
    let mut unresolved_paths = Vec::new();

    for strata_map in strata_pending.iter_mut() {
        strata_map.retain(|path, requests| {
            match prune_completed_texture_requests_for_path(path, requests) {
                PendingTexturePathPruneResult::Keep => true,
                PendingTexturePathPruneResult::Resolved(path) => {
                    resolved_paths.push(path);
                    changed = true;
                    false
                }
                PendingTexturePathPruneResult::Unresolved(path) => {
                    unresolved_paths.push(path);
                    changed = true;
                    false
                }
            }
        });
    }

    (changed, resolved_paths, unresolved_paths)
}

pub(super) fn update_ready_texture_path_cache(
    ready_paths: &mut FxHashSet<String>,
    resolved_paths: Vec<String>,
    unresolved_paths: Vec<String>,
) {
    for path in resolved_paths {
        ready_paths.insert(path);
    }
    for path in unresolved_paths {
        ready_paths.remove(&path);
    }
}

fn prune_completed_texture_requests_for_path(
    path: &str,
    requests: &mut Vec<crate::render::TextureRequest>,
) -> PendingTexturePathPruneResult {
    let had_ready = requests.iter().any(|request| request.handle.is_ready());
    requests.retain(|request| request.handle.is_pending());
    if !requests.is_empty() {
        return PendingTexturePathPruneResult::Keep;
    }

    if had_ready {
        PendingTexturePathPruneResult::Resolved(path.to_string())
    } else {
        PendingTexturePathPruneResult::Unresolved(path.to_string())
    }
}

pub(super) fn texture_preload_logging_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os(TEXTURE_PRELOAD_LOG_ENV).is_some())
}

pub(super) fn format_texture_preload_log(telemetry: &TexturePreloadPassTelemetry) -> String {
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

pub(super) fn texture_preload_reason(telemetry: &TexturePreloadPassTelemetry) -> &'static str {
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

pub(super) fn sample_texture_paths(paths: &[String], limit: usize) -> Vec<String> {
    paths.iter().take(limit).cloned().collect()
}

pub(super) fn sort_pending_texture_paths(paths: &mut [String]) {
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

impl App {
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
        let mut pass = self.begin_texture_preload_pass(budget);
        let deadline = self.texture_preload_deadline(budget);
        let queued_progress = self.preload_queued_textures_until(deadline, pass.log_enabled);

        self.finish_texture_preload_pass(&mut pass, queued_progress)
    }

    fn begin_texture_preload_pass(
        &self,
        budget: Option<std::time::Duration>,
    ) -> TexturePreloadPass {
        TexturePreloadPass {
            started: std::time::Instant::now(),
            log_enabled: texture_preload_logging_enabled(),
            telemetry: TexturePreloadPassTelemetry {
                budget,
                pending: self.textures_pending.get(),
                ..Default::default()
            },
        }
    }

    fn preload_queued_textures_until(
        &self,
        deadline: Option<std::time::Instant>,
        collect_samples: bool,
    ) -> QueuedTexturePreloadProgress {
        let mut tex_mgr = self.texture_manager.borrow_mut();
        self.preload_queued_texture_requests(&mut tex_mgr, deadline, collect_samples)
    }

    fn finish_texture_preload_pass(
        &self,
        pass: &mut TexturePreloadPass,
        queued_progress: QueuedTexturePreloadProgress,
    ) -> bool {
        let draw_pending = self.cached_render_requests_still_pending();
        let redraw_needed =
            self.apply_texture_preload_progress(&mut pass.telemetry, queued_progress, draw_pending);
        pass.telemetry.elapsed = pass.started.elapsed();
        if pass.log_enabled {
            eprintln!("{}", format_texture_preload_log(&pass.telemetry));
        }
        redraw_needed
    }

    fn texture_preload_deadline(
        &self,
        budget: Option<std::time::Duration>,
    ) -> Option<std::time::Instant> {
        let env = self.env.borrow();
        let is_glue_screen = env.state().borrow().screen_kind.is_glue();
        drop(env);

        match budget {
            Some(budget) => Some(std::time::Instant::now() + budget),
            None => (!is_glue_screen)
                .then(|| std::time::Instant::now() + std::time::Duration::from_millis(250)),
        }
    }

    fn apply_texture_preload_progress(
        &self,
        telemetry: &mut TexturePreloadPassTelemetry,
        queued_progress: QueuedTexturePreloadProgress,
        draw_pending: bool,
    ) -> bool {
        let pending_before = telemetry.pending;
        telemetry.queued = queued_progress.total;
        telemetry.loaded = queued_progress.loaded;
        telemetry.remaining = queued_progress.remaining;
        telemetry.remaining_sample = queued_progress.remaining_sample;

        if telemetry.queued == 0 {
            telemetry.pending = draw_pending;
            self.textures_pending.set(draw_pending);
            return !pending_before && draw_pending;
        }

        telemetry.pending = telemetry.remaining != 0 || draw_pending;
        self.textures_pending.set(telemetry.pending);
        telemetry.loaded != 0 || (!pending_before && telemetry.pending)
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
}
