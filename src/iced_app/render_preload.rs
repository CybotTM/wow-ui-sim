use std::sync::OnceLock;

use super::{PendingTextureRequestsByStrata, TEXTURE_PRELOAD_LOG_ENV, TexturePreloadPassTelemetry};
use crate::render::load_texture_or_crop;

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
