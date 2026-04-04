use iced::{Point, Rectangle, Size};

use std::collections::HashSet;
use std::sync::Arc;

use crate::render::texture::UI_SCALE;
use crate::render::{GpuTextureData, QuadBatch, load_texture_or_crop};
use crate::widget::{FrameStrata, WidgetType};

use super::super::app::App;
use super::super::quad_builders::{build_texture_quads, emit_button_highlight};

impl App {
    pub(super) fn build_overlay(&self) -> QuadBatch {
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
    pub(super) fn load_all_textures(
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
        let elapsed = t.elapsed();
        if elapsed.as_millis() > 50 && !textures.is_empty() {
            log_slow_texture_load(&textures, elapsed);
        }
        (textures, elapsed)
    }

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

fn log_slow_texture_load(textures: &[GpuTextureData], elapsed: std::time::Duration) {
    let preview: Vec<&str> = textures
        .iter()
        .take(12)
        .map(|tex| tex.path.as_str())
        .collect();
    eprintln!(
        "{} [textures] loaded {} in {elapsed:.1?}: {}",
        crate::logging::global_elapsed_prefix(),
        textures.len(),
        preview.join(", ")
    );
}

pub(super) fn collect_texture_request_paths(
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
