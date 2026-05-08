use iced::{Rectangle, Size};
use std::sync::Arc;

use crate::iced_app::app::App;
use crate::render::{GpuBcTextureData, GpuTextureData, QuadBatch, WowUiPrimitive};

use super::{DrawLogMetrics, DrawQuadRebuild, log_draw_metrics};

struct DrawTextureLoad {
    textures: Vec<GpuTextureData>,
    bc_textures: Vec<GpuBcTextureData>,
    tex_dur: std::time::Duration,
    texture_requests:
        Arc<std::sync::Mutex<crate::render::shader::primitive::TextureRequestTracker>>,
}

impl App {
    pub(super) fn draw_wow_ui_primitive(&self, bounds: Rectangle) -> WowUiPrimitive {
        let start = self.begin_draw_frame();
        let size = self.sync_draw_bounds(bounds);
        let mut quads = self.rebuild_draw_quads(size);
        let overlay = self.build_overlay();
        let texture_load = self.load_draw_textures(&mut quads, &overlay);

        self.log_draw_frame(&quads, &texture_load);
        self.record_draw_time(start.elapsed());

        self.build_draw_primitive(
            quads.dirty_strata,
            overlay,
            texture_load.textures,
            texture_load.bc_textures,
            texture_load.texture_requests,
        )
    }

    fn begin_draw_frame(&self) -> std::time::Instant {
        self.set_main_thread_phase("draw");
        self.frame_count.set(self.frame_count.get() + 1);
        std::time::Instant::now()
    }

    fn sync_draw_bounds(&self, bounds: Rectangle) -> Size {
        let size = bounds.size();
        self.screen_size.set(size);
        self.sync_screen_size_to_state(size);
        size
    }

    fn load_draw_textures(
        &self,
        quads: &mut DrawQuadRebuild,
        overlay: &QuadBatch,
    ) -> DrawTextureLoad {
        let (textures, bc_textures, tex_dur, texture_requests) =
            self.load_all_textures(&quads.dirty_strata, overlay);

        if quads.had_textures_pending {
            self.recover_pending_textures(&mut quads.dirty_strata, &texture_requests);
        }

        DrawTextureLoad {
            textures,
            bc_textures,
            tex_dur,
            texture_requests,
        }
    }

    fn log_draw_frame(&self, quads: &DrawQuadRebuild, texture_load: &DrawTextureLoad) {
        log_draw_metrics(DrawLogMetrics {
            quad_dur: quads.quad_dur,
            tex_dur: texture_load.tex_dur,
            dirty_before: quads.dirty_before,
            had_textures_pending: quads.had_textures_pending,
            dirty_strata: &quads.dirty_strata,
            rgba_count: texture_load.textures.len(),
            bc_count: texture_load.bc_textures.len(),
            texture_requests: &texture_load.texture_requests,
        });
    }
}
