use std::sync::Arc;

use crate::render::QuadBatch;
use crate::widget::FrameStrata;

pub(super) struct DrawLogMetrics<'a> {
    pub total_dur: std::time::Duration,
    pub quad_dur: std::time::Duration,
    pub tex_dur: std::time::Duration,
    pub dirty_before: u16,
    pub had_textures_pending: bool,
    pub dirty_strata: &'a [Option<Arc<QuadBatch>>; FrameStrata::COUNT],
    pub rgba_count: usize,
    pub bc_count: usize,
    pub texture_requests:
        &'a Arc<std::sync::Mutex<crate::render::shader::primitive::TextureRequestTracker>>,
}

pub(super) fn log_draw_metrics(metrics: DrawLogMetrics<'_>) {
    log_stalled_draw(metrics.total_dur, metrics.quad_dur, metrics.tex_dur);
    log_slow_draw(
        metrics.quad_dur,
        metrics.tex_dur,
        metrics.rgba_count,
        metrics.bc_count,
    );
    log_draw_trace(
        metrics.total_dur,
        metrics.quad_dur,
        metrics.tex_dur,
        metrics.dirty_before,
        metrics.had_textures_pending,
        metrics.dirty_strata,
        metrics.rgba_count,
        metrics.bc_count,
        metrics.texture_requests,
    );
}

/// A draw that blocks the main thread this long freezes input, ticks, and the
/// cursor overlay; always make it loud, no debug env required.
fn log_stalled_draw(
    total_dur: std::time::Duration,
    quad_dur: std::time::Duration,
    tex_dur: std::time::Duration,
) {
    if total_dur.as_millis() < 500 {
        return;
    }
    let other = total_dur.saturating_sub(quad_dur).saturating_sub(tex_dur);
    eprintln!(
        "{} [draw] STALL total={total_dur:.1?} quads={quad_dur:.1?} textures={tex_dur:.1?} other={other:.1?}",
        crate::logging::global_elapsed_prefix(),
    );
}

pub(super) fn log_slow_draw(
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

fn log_draw_trace(
    total_dur: std::time::Duration,
    quad_dur: std::time::Duration,
    tex_dur: std::time::Duration,
    dirty_before: u16,
    had_textures_pending: bool,
    dirty_strata: &[Option<Arc<QuadBatch>>; FrameStrata::COUNT],
    rgba_count: usize,
    bc_count: usize,
    texture_requests: &Arc<
        std::sync::Mutex<crate::render::shader::primitive::TextureRequestTracker>,
    >,
) {
    if !crate::logging::gui_trace_enabled() {
        return;
    }
    let ready_count = texture_requests
        .lock()
        .map(|tracker| tracker.ready_count())
        .unwrap_or_default();
    crate::logging::eprintln_gui_trace(&format!(
        "draw total={total_dur:.1?} quads={quad_dur:.1?} tex={tex_dur:.1?} dirty_before=0x{dirty_before:x} had_pending={} ready={ready_count} dirty_batches={} new_rgba={} new_bc={}",
        had_textures_pending,
        dirty_strata.iter().filter(|batch| batch.is_some()).count(),
        rgba_count,
        bc_count,
    ));
}
