use iced::{Point, Rectangle, Size};

use crate::iced_app::layout::anchor_position;
use std::collections::HashSet;
use std::sync::Arc;

use crate::render::shader::primitive::load_texture_prefer_bc;
use crate::render::texture::UI_SCALE;
use crate::render::{GpuBcTextureData, GpuTextureData, QuadBatch};
use crate::widget::{Frame, FrameStrata, WidgetType};

use super::super::app::App;
use super::super::quad_builders::{build_texture_quads, emit_button_highlight};

impl App {
    pub(super) fn build_overlay(&self) -> QuadBatch {
        let mut overlay = QuadBatch::new();
        self.append_debug_overlay(&mut overlay);
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

    fn append_debug_overlay(&self, overlay: &mut QuadBatch) {
        let env = self.env.borrow();
        let state = env.state().borrow();
        let (show_borders, show_anchors) = self.debug_overlay_flags(&state);
        if !show_borders && !show_anchors {
            return;
        }

        for id in collect_debug_overlay_ids(&state) {
            self.append_frame_debug_overlay(overlay, &state, id, show_borders, show_anchors);
        }
    }

    fn debug_overlay_flags(&self, state: &crate::lua_api::SimState) -> (bool, bool) {
        (
            self.debug_borders || state.debug_borders,
            self.debug_anchors || state.debug_anchors,
        )
    }

    fn append_frame_debug_overlay(
        &self,
        overlay: &mut QuadBatch,
        state: &crate::lua_api::SimState,
        id: u64,
        show_borders: bool,
        show_anchors: bool,
    ) {
        let Some((frame, rect, bounds)) = debug_overlay_frame(state, id) else {
            return;
        };
        if show_borders {
            overlay.push_border(bounds, 1.0, [1.0, 0.1, 0.1, 0.9]);
        }
        if show_anchors {
            append_anchor_markers(overlay, frame, rect);
        }
    }

    /// Load textures with a small draw-thread budget (~10ms).
    /// Remaining work stays pending for the tick preloader.
    pub(super) fn load_all_textures(
        &self,
        dirty_strata: &[Option<Arc<QuadBatch>>; FrameStrata::COUNT],
        overlay: &QuadBatch,
    ) -> (
        Vec<GpuTextureData>,
        Vec<GpuBcTextureData>,
        std::time::Duration,
    ) {
        let t = std::time::Instant::now();
        let deadline = t + std::time::Duration::from_millis(10);
        let mut textures = Vec::new();
        let mut bc_textures = Vec::new();
        let mut exhausted = false;
        for batch_opt in dirty_strata {
            if exhausted {
                break;
            }
            if let Some(batch) = batch_opt {
                let (loaded, loaded_bc, hit) = self.load_new_textures_budgeted(batch, deadline);
                textures.extend(loaded);
                bc_textures.extend(loaded_bc);
                exhausted |= hit;
            }
        }
        if !exhausted {
            let (loaded, loaded_bc, hit) = self.load_new_textures_budgeted(overlay, deadline);
            textures.extend(loaded);
            bc_textures.extend(loaded_bc);
            exhausted |= hit;
        }
        self.textures_pending.set(exhausted);
        let elapsed = t.elapsed();
        if elapsed.as_millis() > 10 && (!textures.is_empty() || !bc_textures.is_empty()) {
            log_slow_texture_load(&textures, &bc_textures, elapsed);
        }
        (textures, bc_textures, elapsed)
    }

    /// Load new textures from the batch's requests within a time budget.
    /// Returns (RGBA textures, BC textures, deadline_reached).
    pub(super) fn load_new_textures_budgeted(
        &self,
        quads: &QuadBatch,
        deadline: std::time::Instant,
    ) -> (Vec<GpuTextureData>, Vec<GpuBcTextureData>, bool) {
        let mut textures = Vec::new();
        let mut bc_textures = Vec::new();
        let mut uploaded = self.gpu_uploaded_textures.borrow_mut();
        let mut failed = self.gpu_failed_textures.borrow_mut();
        let mut tex_mgr = self.texture_manager.borrow_mut();
        let pending_paths = unresolved_texture_request_paths(quads, &uploaded, &failed);

        for path in pending_paths {
            if process_budgeted_texture_request(
                deadline,
                path,
                &mut tex_mgr,
                &mut uploaded,
                &mut failed,
                &mut textures,
                &mut bc_textures,
            ) {
                return (textures, bc_textures, true);
            }
        }
        (textures, bc_textures, false)
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

fn collect_debug_overlay_ids(state: &crate::lua_api::SimState) -> Vec<u64> {
    if let Some(buckets) = state.strata_buckets.as_ref() {
        return buckets
            .iter()
            .flat_map(|bucket| bucket.iter().copied())
            .collect();
    }

    state
        .widgets
        .iter_ids()
        .filter(|&id| state.widgets.is_ancestor_visible(id))
        .collect()
}

fn debug_overlay_frame(
    state: &crate::lua_api::SimState,
    id: u64,
) -> Option<(&Frame, crate::LayoutRect, Rectangle)> {
    if !state.widgets.is_ancestor_visible(id) {
        return None;
    }
    let frame = state.widgets.get(id)?;
    let rect = frame.layout_rect?;
    if rect.width <= 0.0 || rect.height <= 0.0 {
        return None;
    }
    let bounds = Rectangle::new(
        Point::new(rect.x * UI_SCALE, rect.y * UI_SCALE),
        Size::new(rect.width * UI_SCALE, rect.height * UI_SCALE),
    );
    Some((frame, rect, bounds))
}

fn append_anchor_markers(overlay: &mut QuadBatch, frame: &Frame, rect: crate::LayoutRect) {
    const ANCHOR_MARKER_SIZE: f32 = 5.0;
    const ANCHOR_MARKER_OFFSET: f32 = ANCHOR_MARKER_SIZE * 0.5;
    const ANCHOR_MARKER_COLOR: [f32; 4] = [0.1, 1.0, 0.1, 1.0];

    for anchor in &frame.anchors {
        let (x, y) = anchor_position(
            anchor.point,
            rect.x * UI_SCALE,
            rect.y * UI_SCALE,
            rect.width * UI_SCALE,
            rect.height * UI_SCALE,
        );
        overlay.push_solid(
            Rectangle::new(
                Point::new(x - ANCHOR_MARKER_OFFSET, y - ANCHOR_MARKER_OFFSET),
                Size::new(ANCHOR_MARKER_SIZE, ANCHOR_MARKER_SIZE),
            ),
            ANCHOR_MARKER_COLOR,
        );
    }
}

fn log_slow_texture_load(
    textures: &[GpuTextureData],
    bc_textures: &[GpuBcTextureData],
    elapsed: std::time::Duration,
) {
    let mut preview: Vec<&str> = textures.iter().map(|tex| tex.path.as_str()).collect();
    preview.extend(bc_textures.iter().map(|tex| tex.path.as_str()));
    preview.truncate(12);
    eprintln!(
        "{} [textures] loaded {} in {elapsed:.1?}: {} (rgba={} bc={})",
        crate::logging::global_elapsed_prefix(),
        textures.len() + bc_textures.len(),
        preview.join(", "),
        textures.len(),
        bc_textures.len(),
    );
}

fn should_pause_texture_loading(
    textures: &[GpuTextureData],
    bc_textures: &[GpuBcTextureData],
    deadline: std::time::Instant,
    path: &str,
    tex_mgr: &crate::texture::TextureManager,
) -> bool {
    let loaded_any_textures = !textures.is_empty() || !bc_textures.is_empty();
    let deadline_reached = std::time::Instant::now() >= deadline;
    let base_path_cached = tex_mgr.is_cached(texture_request_base_path(path));
    should_pause_texture_loading_state(loaded_any_textures, deadline_reached, base_path_cached)
}

fn should_pause_texture_loading_state(
    loaded_any_textures: bool,
    deadline_reached: bool,
    base_path_cached: bool,
) -> bool {
    loaded_any_textures && deadline_reached && !base_path_cached
}

fn texture_request_base_path(path: &str) -> &str {
    path.find("@crop:").map_or(path, |index| &path[..index])
}

fn process_budgeted_texture_request(
    deadline: std::time::Instant,
    path: &str,
    tex_mgr: &mut crate::texture::TextureManager,
    uploaded: &mut std::collections::HashSet<String>,
    failed: &mut std::collections::HashSet<String>,
    textures: &mut Vec<GpuTextureData>,
    bc_textures: &mut Vec<GpuBcTextureData>,
) -> bool {
    if should_pause_texture_loading(textures, bc_textures, deadline, path, tex_mgr) {
        return true;
    }
    load_pending_texture(tex_mgr, path, uploaded, failed, textures, bc_textures);
    false
}

fn load_pending_texture(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &str,
    uploaded: &mut std::collections::HashSet<String>,
    failed: &mut std::collections::HashSet<String>,
    textures: &mut Vec<GpuTextureData>,
    bc_textures: &mut Vec<GpuBcTextureData>,
) {
    use crate::render::shader::primitive::LoadedTexture;

    if let Some(loaded) = load_texture_prefer_bc(tex_mgr, path) {
        uploaded.insert(path.to_string());
        match loaded {
            LoadedTexture::Rgba(data) => textures.push(data),
            LoadedTexture::Bc(data) => bc_textures.push(data),
        }
        return;
    }

    failed.insert(path.to_string());
}

fn unresolved_texture_request_paths<'a>(
    quads: &'a QuadBatch,
    uploaded: &std::collections::HashSet<String>,
    failed: &std::collections::HashSet<String>,
) -> Vec<&'a str> {
    let mut paths = Vec::new();
    let mut seen = HashSet::new();

    for request in quads
        .texture_requests
        .iter()
        .chain(&quads.mask_texture_requests)
    {
        let path = request.path.as_str();
        let is_duplicate = !seen.insert(path);
        let already_resolved = uploaded.contains(path) || failed.contains(path);
        if is_duplicate || already_resolved {
            continue;
        }
        paths.push(path);
    }

    sort_texture_request_paths(&mut paths);
    paths
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
    sort_texture_request_paths(&mut paths);
    paths
}

fn sort_texture_request_paths<T: AsRef<str>>(paths: &mut [T]) {
    paths.sort_by(|a, b| {
        texture_request_priority(a.as_ref())
            .cmp(&texture_request_priority(b.as_ref()))
            .then_with(|| a.as_ref().cmp(b.as_ref()))
    });
}

fn texture_request_priority(path: &str) -> (u8, u8) {
    let is_world_map = path
        .get(..19)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Interface\\WorldMap\\"))
        || path
            .get(..19)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Interface/WorldMap/"));
    let is_crop = path.contains("@crop:");
    (u8::from(is_world_map), u8::from(!is_crop))
}

#[cfg(test)]
mod tests {
    use super::{
        collect_texture_request_paths, process_budgeted_texture_request,
        should_pause_texture_loading_state, texture_request_base_path,
        unresolved_texture_request_paths,
    };
    use crate::iced_app::App;
    use crate::render::{GlyphAtlas, GpuTextureData, QuadBatch, TextureRequest, WowFontSystem};
    use crate::screen::ScreenKind;
    use crate::texture::TextureManager;
    use crate::widget::{AnchorPoint, Frame, FrameStrata, WidgetType};
    use crate::{LayoutRect, lua_api::WowLuaEnv};
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::Arc;
    use tokio::sync::mpsc;

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
            vec!["bar".to_string(), "baz".to_string(), "foo".to_string()]
        );
    }

    #[test]
    fn collect_texture_request_paths_prioritizes_non_worldmap_crops_before_tiles() {
        let mut strata: [Option<Arc<QuadBatch>>; FrameStrata::COUNT] =
            std::array::from_fn(|_| None);
        let mut batch = QuadBatch::new();
        batch
            .texture_requests
            .push(request(r"Interface\WorldMap\IsleofDorn\IsleOfDorn1"));
        batch.texture_requests.push(request(
            r"Interface\questframe\questmaplogatlas@crop:0.1,0.2,0.3,0.4",
        ));
        batch
            .texture_requests
            .push(request(r"Interface\Minimap\UI-Minimap-Background"));
        batch
            .texture_requests
            .push(request(r"Interface\WorldMap\IsleofDorn\IsleOfDorn2"));
        strata[0] = Some(Arc::new(batch));

        let paths = collect_texture_request_paths(&strata, &QuadBatch::new());
        assert_eq!(
            paths,
            vec![
                r"Interface\questframe\questmaplogatlas@crop:0.1,0.2,0.3,0.4".to_string(),
                r"Interface\Minimap\UI-Minimap-Background".to_string(),
                r"Interface\WorldMap\IsleofDorn\IsleOfDorn1".to_string(),
                r"Interface\WorldMap\IsleofDorn\IsleOfDorn2".to_string(),
            ]
        );
    }

    #[test]
    fn unresolved_texture_request_paths_filters_uploaded_and_failed_requests() {
        let mut batch = QuadBatch::new();
        batch
            .texture_requests
            .push(request(r"Interface\WorldMap\IsleofDorn\IsleOfDorn1"));
        batch
            .texture_requests
            .push(request(r"Interface\Minimap\UI-Minimap-Background"));
        batch.mask_texture_requests.push(request("uploaded-mask"));
        batch.texture_requests.push(request("failed-path"));

        let uploaded = HashSet::from(["uploaded-mask".to_string()]);
        let failed = HashSet::from(["failed-path".to_string()]);

        let paths = unresolved_texture_request_paths(&batch, &uploaded, &failed);
        assert_eq!(
            paths,
            vec![
                r"Interface\Minimap\UI-Minimap-Background",
                r"Interface\WorldMap\IsleofDorn\IsleOfDorn1",
            ]
        );
    }

    #[test]
    fn unresolved_texture_request_paths_deduplicates_before_sorting() {
        let mut batch = QuadBatch::new();
        batch
            .texture_requests
            .push(request(r"Interface\WorldMap\IsleofDorn\IsleOfDorn1"));
        batch
            .texture_requests
            .push(request(r"Interface\WorldMap\IsleofDorn\IsleOfDorn1"));
        batch.texture_requests.push(request(
            r"Interface\questframe\questmaplogatlas@crop:0.1,0.2,0.3,0.4",
        ));

        let paths = unresolved_texture_request_paths(&batch, &HashSet::new(), &HashSet::new());
        assert_eq!(
            paths,
            vec![
                r"Interface\questframe\questmaplogatlas@crop:0.1,0.2,0.3,0.4",
                r"Interface\WorldMap\IsleofDorn\IsleOfDorn1",
            ]
        );
    }

    #[test]
    fn texture_request_base_path_strips_crop_suffix() {
        assert_eq!(
            texture_request_base_path(r"Interface\Foo\Bar@crop:0.1,0.2,0.3,0.4"),
            r"Interface\Foo\Bar"
        );
        assert_eq!(
            texture_request_base_path(r"Interface\Foo\Bar"),
            r"Interface\Foo\Bar"
        );
    }

    #[test]
    fn texture_loading_only_pauses_after_budget_hit_for_uncached_base_path() {
        assert!(!should_pause_texture_loading_state(false, true, false));
        assert!(!should_pause_texture_loading_state(true, false, false));
        assert!(!should_pause_texture_loading_state(true, true, true));
        assert!(should_pause_texture_loading_state(true, true, false));
    }

    #[test]
    fn process_budgeted_texture_request_returns_true_before_loading_uncached_work() {
        let mut tex_mgr = TextureManager::new(PathBuf::from("./textures"));
        let mut uploaded = HashSet::new();
        let mut failed = HashSet::new();
        let mut textures = vec![GpuTextureData {
            path: "already-loaded".to_string(),
            width: 1,
            height: 1,
            rgba: Arc::<[u8]>::from(vec![0xff; 4]),
        }];
        let mut bc_textures = Vec::new();

        let paused = process_budgeted_texture_request(
            std::time::Instant::now(),
            r"Interface\Foo\Bar",
            &mut tex_mgr,
            &mut uploaded,
            &mut failed,
            &mut textures,
            &mut bc_textures,
        );

        assert!(paused);
        assert!(uploaded.is_empty());
        assert!(failed.is_empty());
        assert_eq!(textures.len(), 1);
        assert!(bc_textures.is_empty());
    }

    fn build_test_app(debug_borders: bool, debug_anchors: bool) -> App {
        let env = Rc::new(RefCell::new(
            WowLuaEnv::new().expect("Failed to create Lua environment"),
        ));
        env.borrow().set_screen_mode(ScreenKind::Game);

        let texture_manager = Rc::new(RefCell::new(TextureManager::new(PathBuf::from(
            "./textures",
        ))));
        let font_system = Rc::new(RefCell::new(WowFontSystem::new(&PathBuf::from(
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
            debug_borders,
            debug_anchors,
            None,
            crate::config::SimConfig::default(),
        )
    }

    fn register_debug_frame(app: &App) {
        let env = app.env.borrow();
        let mut state = env.state().borrow_mut();
        let widgets = &mut state.widgets;
        let mut frame = Frame::new(WidgetType::Frame, Some("DebugFrame".to_string()), None);
        frame.layout_rect = Some(LayoutRect {
            x: 10.0,
            y: 20.0,
            width: 40.0,
            height: 30.0,
        });
        frame.set_point(AnchorPoint::TopLeft, None, AnchorPoint::TopLeft, 0.0, 0.0);
        widgets.register(frame);
    }

    #[test]
    fn build_overlay_emits_debug_quads_from_startup_flags() {
        let app = build_test_app(true, true);
        register_debug_frame(&app);

        let overlay = app.build_overlay();
        assert_eq!(overlay.quad_count(), 5);
        assert!(overlay.texture_requests.is_empty());
        assert!(overlay.mask_texture_requests.is_empty());
    }

    #[test]
    fn build_overlay_emits_debug_quads_from_runtime_toggles() {
        let app = build_test_app(false, false);
        register_debug_frame(&app);
        {
            let env = app.env.borrow();
            let mut state = env.state().borrow_mut();
            state.debug_borders = true;
            state.debug_anchors = true;
        }

        let overlay = app.build_overlay();
        assert_eq!(overlay.quad_count(), 5);
        assert!(overlay.texture_requests.is_empty());
        assert!(overlay.mask_texture_requests.is_empty());
    }
}
