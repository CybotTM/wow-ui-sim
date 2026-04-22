//! Thin App wrappers over the unified dump module.

use super::app::App;
use crate::render::{QuadBatch, QuadVertex, TextureRequest};

fn quad_bounds(batch: &QuadBatch, request: &TextureRequest) -> (f32, f32, f32, f32) {
    let start = request.vertex_start as usize;
    let end = start + request.vertex_count as usize;
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for vert in &batch.vertices[start..end] {
        min_x = min_x.min(vert.position[0]);
        min_y = min_y.min(vert.position[1]);
        max_x = max_x.max(vert.position[0]);
        max_y = max_y.max(vert.position[1]);
    }
    (min_x, min_y, max_x, max_y)
}

fn push_vertex_lines(lines: &mut Vec<String>, label: &str, vertices: &[QuadVertex]) {
    for (idx, vertex) in vertices.iter().enumerate() {
        lines.push(format!(
            "  {label}[{idx}] position={:?} tex_coords={:?} color={:?} tex_index={} flags={} local_uv={:?} mask_tex_index={} mask_tex_coords={:?}",
            vertex.position,
            vertex.tex_coords,
            vertex.color,
            vertex.tex_index,
            vertex.flags,
            vertex.local_uv,
            vertex.mask_tex_index,
            vertex.mask_tex_coords
        ));
    }
}

fn push_request_lines(
    lines: &mut Vec<String>,
    batch: &QuadBatch,
    strata_idx: usize,
    kind: &str,
    request: &TextureRequest,
    verbose: bool,
) {
    let start = request.vertex_start as usize;
    let end = start + request.vertex_count as usize;
    let bounds = quad_bounds(batch, request);
    lines.push(format!(
        "strata={strata_idx} kind={kind} path={} vertex_start={} vertex_count={} bounds=({:.2}, {:.2}) -> ({:.2}, {:.2})",
        request.path, request.vertex_start, request.vertex_count, bounds.0, bounds.1, bounds.2, bounds.3
    ));
    if verbose {
        push_vertex_lines(lines, "vertex", &batch.vertices[start..end]);
    }
}

fn build_cached_quad_dump(
    strata_batches: &[Option<std::sync::Arc<QuadBatch>>],
    filter: Option<&str>,
    verbose: bool,
) -> String {
    let mut lines = Vec::new();
    let filter = filter.map(str::to_ascii_lowercase);

    for (strata_idx, batch) in strata_batches.iter().enumerate() {
        let Some(batch) = batch else { continue };
        for request in &batch.texture_requests {
            if filter
                .as_deref()
                .is_some_and(|needle| !request.path.to_ascii_lowercase().contains(needle))
            {
                continue;
            }
            push_request_lines(&mut lines, batch, strata_idx, "texture", request, verbose);
        }
        for request in &batch.mask_texture_requests {
            if filter
                .as_deref()
                .is_some_and(|needle| !request.path.to_ascii_lowercase().contains(needle))
            {
                continue;
            }
            push_request_lines(&mut lines, batch, strata_idx, "mask", request, verbose);
        }
    }

    if lines.is_empty() {
        "No cached quads found".to_string()
    } else {
        lines.join("\n")
    }
}

impl App {
    /// Dump WoW frames for debug server (compact format with warnings).
    pub(crate) fn dump_wow_frames(&self) -> String {
        let env = self.env.borrow();
        let state = env.state().borrow();
        let screen_width = self.screen_size.get().width;
        let screen_height = self.screen_size.get().height;
        let addon_names: Vec<String> = state.addons.iter().map(|a| a.folder_name.clone()).collect();
        crate::dump::build_warning_dump(&state.widgets, &addon_names, screen_width, screen_height)
            .join("\n")
    }

    /// Build a frame tree dump with computed layout rects (for connected dump-tree).
    pub(crate) fn build_frame_tree_dump(
        &self,
        filter: Option<&str>,
        filter_key: Option<&str>,
        visible_only: bool,
        verbose: bool,
    ) -> String {
        let env = self.env.borrow();
        let state = env.state().borrow();
        let screen_width = self.screen_size.get().width;
        let screen_height = self.screen_size.get().height;
        let addon_names: Vec<String> = state.addons.iter().map(|a| a.folder_name.clone()).collect();
        let lines = crate::dump::build_tree(
            &state.widgets,
            &addon_names,
            filter,
            filter_key,
            visible_only,
            verbose,
            screen_width,
            screen_height,
        );
        if lines.is_empty() {
            "No frames found".to_string()
        } else {
            lines.join("\n")
        }
    }

    /// Dump cached live GUI quads from per-strata render caches.
    pub(crate) fn build_cached_quad_dump(&self, filter: Option<&str>, verbose: bool) -> String {
        let strata = self.cached_strata_quads.borrow();
        build_cached_quad_dump(&*strata, filter, verbose)
    }
}

#[cfg(test)]
mod tests {
    use super::build_cached_quad_dump;
    use crate::render::{BlendMode, QuadBatch};
    use iced::{Point, Rectangle, Size};
    use std::sync::Arc;

    #[test]
    fn cached_quad_dump_includes_request_and_vertex_fields() {
        let mut batch = QuadBatch::new();
        batch.push_quad(
            Rectangle::new(Point::new(10.0, 20.0), Size::new(30.0, 40.0)),
            Rectangle::new(Point::new(0.0, 0.0), Size::new(1.0, 1.0)),
            [1.0, 1.0, 1.0, 1.0],
            -2,
            BlendMode::Alpha,
        );
        batch
            .texture_requests
            .push(crate::render::TextureRequest::new(
                "Interface\\hud\\uigroupmanager@crop:test",
                0,
                4,
            ));
        let strata = [
            Some(Arc::new(batch)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];

        let dump = build_cached_quad_dump(&strata, Some("uigroupmanager"), true);
        assert!(dump.contains("kind=texture"));
        assert!(dump.contains("path=Interface\\hud\\uigroupmanager@crop:test"));
        assert!(dump.contains("vertex_start=0"));
        assert!(dump.contains("vertex[0] position="));
        assert!(dump.contains("mask_tex_index="));
    }
}
