use image::RgbaImage;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::{cell::RefCell, rc::Rc};
use wow_ui_sim::iced_app::{build_quad_batch_for_registry, compute_frame_rect};
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::{SimState, WowLuaEnv};
use wow_ui_sim::render::{GlyphAtlas, QuadBatch, QuadVertex, TextureRequest, WowFontSystem};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::texture::TextureManager;
use wow_ui_sim::toc::TocFile;
use wow_ui_sim::widget::WidgetRegistry;

pub(super) fn build_strata_buckets(env: &WowLuaEnv) -> Vec<Vec<u64>> {
    let mut state = env.state().borrow_mut();
    let _ = state.get_strata_buckets();
    state.strata_buckets.as_ref().unwrap().clone()
}

fn make_font_system() -> Rc<RefCell<WowFontSystem>> {
    Rc::new(RefCell::new(WowFontSystem::new(&PathBuf::from("./fonts"))))
}

pub(super) fn make_texture_manager() -> Option<TextureManager> {
    let home = dirs::home_dir().unwrap_or_default();
    let local_textures = PathBuf::from("./textures");
    let textures_path = if local_textures.exists() {
        local_textures
    } else {
        let fallback = home.join("Repos/wow-ui-textures");
        if !fallback.exists() {
            return None;
        }
        fallback
    };

    let interface_path = home.join("Projects/wow/Interface");
    let mut mgr = TextureManager::new(textures_path);
    if interface_path.exists() {
        mgr = mgr.with_interface_path(interface_path);
    }
    Some(mgr)
}

pub(super) fn build_screenshot_like_batch(
    env: &WowLuaEnv,
    width: u32,
    height: u32,
    filter: Option<&str>,
) -> QuadBatch {
    let font_system = make_font_system();
    env.set_font_system(Rc::clone(&font_system));
    env.set_screen_size(width as f32, height as f32);
    wow_ui_sim::startup::run_extra_update_ticks(env, 3);

    let mut glyph_atlas = GlyphAtlas::new();
    let mut font_system = font_system.borrow_mut();
    let buckets = {
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_system);
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    let tooltip_data = wow_ui_sim::iced_app::tooltip::collect_tooltip_data(&state);
    build_quad_batch_for_registry(
        &state.widgets,
        (width as f32, height as f32),
        filter,
        None,
        None,
        Some((&mut font_system, &mut glyph_atlas)),
        Some(&state.message_frames),
        Some(&tooltip_data),
        &buckets,
    )
}

pub(super) fn is_descendant_of(
    widgets: &WidgetRegistry,
    mut frame_id: u64,
    ancestor_id: u64,
) -> bool {
    loop {
        if frame_id == ancestor_id {
            return true;
        }
        let Some(parent_id) = widgets.get(frame_id).and_then(|frame| frame.parent_id) else {
            return false;
        };
        frame_id = parent_id;
    }
}

pub(super) fn diff_bounds(
    before: &RgbaImage,
    after: &RgbaImage,
    per_channel_tolerance: u8,
) -> Option<(u32, u32, u32, u32)> {
    let mut min_x = u32::MAX;
    let mut min_y = u32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..before.height() {
        for x in 0..before.width() {
            let lhs = before.get_pixel(x, y).0;
            let rhs = after.get_pixel(x, y).0;
            let differs =
                (0..4).any(|channel| lhs[channel].abs_diff(rhs[channel]) > per_channel_tolerance);
            if differs {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                found = true;
            }
        }
    }

    found.then_some((min_x, min_y, max_x, max_y))
}

pub(super) fn diff_pixels_in_rect(
    before: &RgbaImage,
    after: &RgbaImage,
    rect: (u32, u32, u32, u32),
    per_channel_tolerance: u8,
) -> Vec<(u32, u32, [u8; 4], [u8; 4])> {
    let mut diffs = Vec::new();
    let max_x = (rect.0 + rect.2).min(before.width());
    let max_y = (rect.1 + rect.3).min(before.height());
    for y in rect.1..max_y {
        for x in rect.0..max_x {
            let lhs = before.get_pixel(x, y).0;
            let rhs = after.get_pixel(x, y).0;
            let differs =
                (0..4).any(|channel| lhs[channel].abs_diff(rhs[channel]) > per_channel_tolerance);
            if differs {
                diffs.push((x, y, lhs, rhs));
            }
        }
    }
    diffs
}

fn quad_bounds_from_vertices(verts: &[QuadVertex]) -> (f32, f32, f32, f32) {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for vert in verts {
        min_x = min_x.min(vert.position[0]);
        min_y = min_y.min(vert.position[1]);
        max_x = max_x.max(vert.position[0]);
        max_y = max_y.max(vert.position[1]);
    }
    (min_x, min_y, max_x, max_y)
}

pub(super) fn quad_bounds(batch: &QuadBatch, request: &TextureRequest) -> (f32, f32, f32, f32) {
    let start = request.vertex_start as usize;
    let end = start + request.vertex_count as usize;
    quad_bounds_from_vertices(&batch.vertices[start..end])
}

pub(super) fn request_overlaps_rect(
    batch: &QuadBatch,
    request: &TextureRequest,
    rect: (f32, f32, f32, f32),
) -> bool {
    let bounds = quad_bounds(batch, request);
    let rect_right = rect.0 + rect.2;
    let rect_bottom = rect.1 + rect.3;
    bounds.0 < rect_right && bounds.2 > rect.0 && bounds.1 < rect_bottom && bounds.3 > rect.1
}

pub(super) fn vertex_range_bounds(
    batch: &QuadBatch,
    vertex_start: usize,
    vertex_count: usize,
) -> (f32, f32, f32, f32) {
    quad_bounds_from_vertices(&batch.vertices[vertex_start..vertex_start + vertex_count])
}

pub(super) fn bounds_overlap_rect(
    bounds: (f32, f32, f32, f32),
    rect: (f32, f32, f32, f32),
) -> bool {
    let rect_right = rect.0 + rect.2;
    let rect_bottom = rect.1 + rect.3;
    bounds.0 < rect_right && bounds.2 > rect.0 && bounds.1 < rect_bottom && bounds.3 > rect.1
}

pub(super) fn request_path_for_frame_texture(
    texture: &str,
    atlas_tex_coords: Option<(f32, f32, f32, f32)>,
) -> String {
    let Some((cl, cr, ct, cb)) = atlas_tex_coords else {
        return texture.to_string();
    };
    let is_full = (cl - 0.0).abs() < 0.001
        && (cr - 1.0).abs() < 0.001
        && (ct - 0.0).abs() < 0.001
        && (cb - 1.0).abs() < 0.001;
    if is_full {
        texture.to_string()
    } else {
        format!("{texture}@crop:{cl:.6},{cr:.6},{ct:.6},{cb:.6}")
    }
}

pub(super) fn world_quest_pin_pairs(
    state: &SimState,
) -> Vec<(u64, u64, wow_ui_sim::LayoutRect, String, String)> {
    let mut pairs = Vec::new();

    for icon_id in state.widgets.iter_ids().filter(|&id| {
        let Some(frame) = state.widgets.get(id) else {
            return false;
        };
        frame.atlas.as_deref() == Some("Worldquest-icon")
    }) {
        let Some(display_frame_id) = state.widgets.get(icon_id).and_then(|frame| frame.parent_id)
        else {
            continue;
        };
        let Some(button_id) = state
            .widgets
            .get(display_frame_id)
            .and_then(|frame| frame.parent_id)
        else {
            continue;
        };
        let Some(circle_id) = state.widgets.iter_ids().find(|&id| {
            let Some(frame) = state.widgets.get(id) else {
                return false;
            };
            frame.parent_id == Some(button_id)
                && frame.atlas.as_deref() == Some("UI-QuestPoi-QuestNumber")
        }) else {
            continue;
        };

        let Some(circle_texture) = state
            .widgets
            .get(circle_id)
            .and_then(|frame| frame.texture.clone())
        else {
            continue;
        };
        let Some(icon_texture) = state
            .widgets
            .get(icon_id)
            .and_then(|frame| frame.texture.clone())
        else {
            continue;
        };

        pairs.push((
            circle_id,
            icon_id,
            compute_frame_rect(&state.widgets, circle_id, 1024.0, 768.0),
            request_path_for_frame_texture(
                &circle_texture,
                state
                    .widgets
                    .get(circle_id)
                    .and_then(|frame| frame.atlas_tex_coords),
            ),
            request_path_for_frame_texture(
                &icon_texture,
                state
                    .widgets
                    .get(icon_id)
                    .and_then(|frame| frame.atlas_tex_coords),
            ),
        ));
    }

    pairs
}

pub(super) fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

pub(super) const ISOLATED_WORLD_MAP_ROOT_ADDONS: &[&str] = &[
    "Blizzard_FrameEffects",
    "Blizzard_StoreUI",
    "Blizzard_UIPanels_Game",
    "Blizzard_MapCanvasSecureUtil",
    "Blizzard_MapCanvas",
    "Blizzard_SharedMapDataProviders",
    "Blizzard_WorldMap",
    "Blizzard_GameMenu",
    "Blizzard_UIWidgets",
    "Blizzard_AddOnList",
    "Blizzard_TimerunningUtil",
];

pub(super) fn discover_blizzard_addon_closure_for_screen(
    blizzard_ui_dir: &Path,
    screen: ScreenKind,
    roots: &[&str],
) -> Vec<(String, PathBuf)> {
    let addons = discover_blizzard_addons_for_screen(blizzard_ui_dir, screen);
    let toc_map: HashMap<String, (PathBuf, TocFile)> = addons
        .iter()
        .map(|(name, toc_path)| {
            let toc = TocFile::from_file(toc_path)
                .unwrap_or_else(|err| panic!("failed to parse TOC for {name}: {err}"));
            (name.clone(), (toc_path.clone(), toc))
        })
        .collect();

    let wanted = collect_declared_dependency_closure(&toc_map, roots);
    addons
        .into_iter()
        .filter(|(name, _)| wanted.contains(name))
        .collect()
}

fn collect_declared_dependency_closure(
    toc_map: &HashMap<String, (PathBuf, TocFile)>,
    roots: &[&str],
) -> HashSet<String> {
    let mut wanted = HashSet::new();
    let mut pending: Vec<String> = roots.iter().map(|name| (*name).to_string()).collect();

    while let Some(name) = pending.pop() {
        if !wanted.insert(name.clone()) {
            continue;
        }

        let Some((_, toc)) = toc_map.get(&name) else {
            panic!("missing Blizzard addon root/dependency in discovered set: {name}");
        };

        for dep in toc.dependencies() {
            if toc_map.contains_key(&dep) && !wanted.contains(&dep) {
                pending.push(dep);
            }
        }
    }

    wanted
}

pub(super) fn env_with_root_addons_ui(roots: &[&str]) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    let ui = blizzard_ui_dir();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    for (name, toc_path) in discover_blizzard_addon_closure_for_screen(&ui, ScreenKind::Game, roots)
    {
        if let Err(err) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("[isolated load {name}] FAILED: {err}");
        }
    }

    env.apply_post_load_workarounds();
    wow_ui_sim::startup::settle_headless_startup(&env);
    env
}

pub(super) fn env_with_isolated_world_map_ui() -> WowLuaEnv {
    env_with_root_addons_ui(ISOLATED_WORLD_MAP_ROOT_ADDONS)
}

pub(super) fn env_with_isolated_world_map() -> WowLuaEnv {
    let env = env_with_isolated_world_map_ui();
    open_world_map(&env);
    env
}

pub(super) fn open_world_map(env: &WowLuaEnv) {
    env.exec("ToggleWorldMap()")
        .expect("failed to toggle world map after startup");
    wow_ui_sim::startup::process_pending_timers(env);
    wow_ui_sim::startup::fire_one_on_update_tick(env);
}
