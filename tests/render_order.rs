//! Render order tests: strata bucket ordering and z-order correctness.

mod common;

use common::env_with_shared_xml;
use image::RgbaImage;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::{cell::RefCell, rc::Rc};
use wow_ui_sim::iced_app::{build_quad_batch_for_registry, compute_frame_rect};
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::headless::render_to_image;
use wow_ui_sim::render::{GlyphAtlas, WowFontSystem};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::texture::TextureManager;
use wow_ui_sim::toc::TocFile;

/// Build strata buckets from a WowLuaEnv (mutable borrow), then return a clone.
fn build_strata_buckets(env: &wow_ui_sim::lua_api::WowLuaEnv) -> Vec<Vec<u64>> {
    let mut state = env.state().borrow_mut();
    let _ = state.get_strata_buckets();
    state.strata_buckets.as_ref().unwrap().clone()
}

fn make_font_system() -> Rc<RefCell<WowFontSystem>> {
    Rc::new(RefCell::new(WowFontSystem::new(&PathBuf::from("./fonts"))))
}

fn make_texture_manager() -> Option<TextureManager> {
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

fn build_screenshot_like_batch(
    env: &WowLuaEnv,
    width: u32,
    height: u32,
    filter: Option<&str>,
) -> wow_ui_sim::render::QuadBatch {
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

fn is_descendant_of(
    widgets: &wow_ui_sim::widget::WidgetRegistry,
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

fn diff_bounds(
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

fn diff_pixels_in_rect(
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

fn quad_bounds_from_vertices(verts: &[wow_ui_sim::render::QuadVertex]) -> (f32, f32, f32, f32) {
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

fn quad_bounds(
    batch: &wow_ui_sim::render::QuadBatch,
    request: &wow_ui_sim::render::TextureRequest,
) -> (f32, f32, f32, f32) {
    let start = request.vertex_start as usize;
    let end = start + request.vertex_count as usize;
    quad_bounds_from_vertices(&batch.vertices[start..end])
}

fn request_overlaps_rect(
    batch: &wow_ui_sim::render::QuadBatch,
    request: &wow_ui_sim::render::TextureRequest,
    rect: (f32, f32, f32, f32),
) -> bool {
    let bounds = quad_bounds(batch, request);
    let rect_right = rect.0 + rect.2;
    let rect_bottom = rect.1 + rect.3;
    bounds.0 < rect_right && bounds.2 > rect.0 && bounds.1 < rect_bottom && bounds.3 > rect.1
}

fn vertex_range_bounds(
    batch: &wow_ui_sim::render::QuadBatch,
    vertex_start: usize,
    vertex_count: usize,
) -> (f32, f32, f32, f32) {
    quad_bounds_from_vertices(&batch.vertices[vertex_start..vertex_start + vertex_count])
}

fn bounds_overlap_rect(bounds: (f32, f32, f32, f32), rect: (f32, f32, f32, f32)) -> bool {
    let rect_right = rect.0 + rect.2;
    let rect_bottom = rect.1 + rect.3;
    bounds.0 < rect_right && bounds.2 > rect.0 && bounds.1 < rect_bottom && bounds.3 > rect.1
}

fn request_path_for_frame_texture(
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

fn world_quest_pin_pairs(
    state: &wow_ui_sim::lua_api::SimState,
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

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Intentional root systems for reduced world-map harness.
///
/// Declared TOC dependencies load automatically. These roots stay explicit
/// because some Blizzard panels still rely on ambient systems that are not
/// modeled as TOC dependencies.
const ISOLATED_WORLD_MAP_ROOT_ADDONS: &[&str] = &[
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

fn discover_blizzard_addon_closure_for_screen(
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

fn env_with_root_addons_ui(roots: &[&str]) -> WowLuaEnv {
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

fn env_with_isolated_world_map_ui() -> WowLuaEnv {
    env_with_root_addons_ui(ISOLATED_WORLD_MAP_ROOT_ADDONS)
}

fn env_with_isolated_world_map() -> WowLuaEnv {
    let env = env_with_isolated_world_map_ui();
    env.exec("ToggleWorldMap()")
        .expect("failed to toggle isolated world map after startup");
    wow_ui_sim::startup::process_pending_timers(&env);
    wow_ui_sim::startup::fire_one_on_update_tick(&env);
    env
}

fn open_world_map(env: &WowLuaEnv) {
    env.exec("ToggleWorldMap()")
        .expect("failed to toggle world map after startup");
    wow_ui_sim::startup::process_pending_timers(env);
    wow_ui_sim::startup::fire_one_on_update_tick(env);
}

#[test]
fn opening_world_map_does_not_darken_the_strip_above_the_panel() {
    let env = env_with_isolated_world_map_ui();

    let baseline_batch = build_screenshot_like_batch(&env, 1024, 768, None);
    let mut baseline_mgr = make_texture_manager().expect("texture directories should exist");
    let baseline_render = render_to_image(&baseline_batch, &mut baseline_mgr, 1024, 768, None);

    open_world_map(&env);

    let world_map_batch = build_screenshot_like_batch(&env, 1024, 768, None);
    let mut world_map_mgr = make_texture_manager().expect("texture directories should exist");
    let world_map_render = render_to_image(&world_map_batch, &mut world_map_mgr, 1024, 768, None);

    let strip_rect = (80, 0, 820, 80);
    let diffs = diff_pixels_in_rect(&baseline_render, &world_map_render, strip_rect, 8);

    let texture_matches: Vec<_> = world_map_batch
        .texture_requests
        .iter()
        .filter(|request| {
            request_overlaps_rect(
                &world_map_batch,
                request,
                (
                    strip_rect.0 as f32,
                    strip_rect.1 as f32,
                    strip_rect.2 as f32,
                    strip_rect.3 as f32,
                ),
            )
        })
        .map(|request| {
            (
                request.path.as_str(),
                quad_bounds(&world_map_batch, request),
            )
        })
        .collect();

    let solid_matches: Vec<_> = world_map_batch
        .vertices
        .chunks_exact(4)
        .enumerate()
        .filter_map(|(quad_idx, verts)| {
            if verts[0].tex_index != -1 {
                return None;
            }
            let vertex_start = quad_idx * 4;
            let bounds = vertex_range_bounds(&world_map_batch, vertex_start, 4);
            bounds_overlap_rect(
                bounds,
                (
                    strip_rect.0 as f32,
                    strip_rect.1 as f32,
                    strip_rect.2 as f32,
                    strip_rect.3 as f32,
                ),
            )
            .then_some((quad_idx, bounds, verts[0].color, verts[0].flags))
        })
        .collect();

    assert!(
        diffs.is_empty(),
        "world map should not change the strip above its panel; diff_count={} first_diff={:?} textures={texture_matches:#?} solids={solid_matches:#?}",
        diffs.len(),
        diffs.first()
    );
}

#[test]
fn world_map_quest_track_checkboxes_use_high_res_checkbox_atlas() {
    let env = env_with_isolated_world_map();
    let state = env.state().borrow();
    let quest_map_id = state
        .widgets
        .get_id_by_name("QuestMapFrame")
        .expect("QuestMapFrame should exist after opening the world map");

    let checkbox_ids: Vec<u64> = state
        .widgets
        .iter_ids()
        .filter(|&id| {
            let Some(frame) = state.widgets.get(id) else {
                return false;
            };
            frame.atlas.as_deref() == Some("questlog-icon-ticksquare")
                && is_descendant_of(&state.widgets, id, quest_map_id)
        })
        .collect();

    assert!(
        !checkbox_ids.is_empty(),
        "world map quest log should contain questlog-icon-ticksquare textures"
    );

    for id in checkbox_ids {
        let frame = state
            .widgets
            .get(id)
            .expect("checkbox texture should still exist");
        let texture = frame
            .texture
            .as_deref()
            .expect("checkbox texture should resolve to a texture path");
        assert!(
            texture.to_ascii_lowercase().contains("questlogframe2x"),
            "quest log checkbox atlas should prefer the 2x texture path, got {texture}"
        );
        assert_eq!(
            frame.width, 14.0,
            "checkbox texture width should stay logical 14px"
        );
        assert_eq!(
            frame.height, 14.0,
            "checkbox texture height should stay logical 14px"
        );
    }
}

#[test]
fn world_map_quest_track_checkbox_quads_match_texture_layout_bounds() {
    let env = env_with_isolated_world_map();
    let batch = build_screenshot_like_batch(&env, 1024, 768, None);
    let state = env.state().borrow();
    let quest_map_id = state
        .widgets
        .get_id_by_name("QuestMapFrame")
        .expect("QuestMapFrame should exist after opening the world map");

    let checkbox_ids: Vec<u64> = state
        .widgets
        .iter_ids()
        .filter(|&id| {
            let Some(frame) = state.widgets.get(id) else {
                return false;
            };
            frame.atlas.as_deref() == Some("questlog-icon-ticksquare")
                && is_descendant_of(&state.widgets, id, quest_map_id)
        })
        .collect();

    assert!(
        !checkbox_ids.is_empty(),
        "world map quest log should contain questlog-icon-ticksquare textures"
    );

    for id in checkbox_ids {
        let frame = state
            .widgets
            .get(id)
            .expect("checkbox texture should still exist");
        let expected_path = request_path_for_frame_texture(
            frame
                .texture
                .as_deref()
                .expect("checkbox texture should resolve to a texture path"),
            frame.atlas_tex_coords,
        );
        let rect = compute_frame_rect(&state.widgets, id, 1024.0, 768.0);
        let expected_bounds = (rect.x, rect.y, rect.x + rect.width, rect.y + rect.height);

        let matching_bounds: Vec<_> = batch
            .texture_requests
            .iter()
            .filter(|request| request.path == expected_path)
            .map(|request| quad_bounds(&batch, request))
            .filter(|bounds| {
                let width_matches = (bounds.2 - bounds.0 - rect.width).abs() < 0.1;
                let height_matches = (bounds.3 - bounds.1 - rect.height).abs() < 0.1;
                let overlaps = bounds_overlap_rect(
                    *bounds,
                    (rect.x, rect.y, rect.width.max(0.1), rect.height.max(0.1)),
                );
                width_matches && height_matches && overlaps
            })
            .collect();

        assert!(
            !matching_bounds.is_empty(),
            "checkbox quad should match texture layout bounds; id={id} expected_path={expected_path} expected_bounds={expected_bounds:?}"
        );
    }
}

#[test]
fn isolated_world_map_dependency_closure_loads_declared_dependencies() {
    let ui = blizzard_ui_dir();
    let addons =
        discover_blizzard_addon_closure_for_screen(&ui, ScreenKind::Game, &["Blizzard_Channels"]);
    let loaded: HashSet<_> = addons.iter().map(|(name, _)| name.as_str()).collect();

    assert!(
        loaded.contains("Blizzard_SocialToast"),
        "dependency closure should include Blizzard_SocialToast when Blizzard_Channels is requested; loaded={loaded:?}"
    );
}

#[test]
fn voice_chat_prompt_renders_below_world_map_panel_in_combined_stack() {
    let mut roots = ISOLATED_WORLD_MAP_ROOT_ADDONS.to_vec();
    roots.extend(["Blizzard_ChatFrame", "Blizzard_Channels"]);
    let env = env_with_root_addons_ui(&roots);
    open_world_map(&env);
    env.exec(
        r#"
        VoiceChatPromptActivateChannel:ClearAllPoints();
        VoiceChatPromptActivateChannel:SetPoint("CENTER", WorldMapFrame, "CENTER", 0, 0);
        VoiceChatPromptActivateChannel:SetAlpha(1);
        VoiceChatPromptActivateChannel:Show();
    "#,
    )
    .expect("failed to show voice chat prompt over world map");
    wow_ui_sim::startup::process_pending_timers(&env);
    wow_ui_sim::startup::fire_one_on_update_tick(&env);

    let buckets = build_strata_buckets(&env);
    let flattened: Vec<u64> = buckets.iter().flatten().copied().collect();
    let state = env.state().borrow();

    let prompt_id = state
        .widgets
        .get_id_by_name("VoiceChatPromptActivateChannel")
        .expect("voice prompt should exist");
    let world_map_id = state
        .widgets
        .get_id_by_name("WorldMapFrame")
        .expect("world map should exist");
    let border_id = state
        .widgets
        .get(world_map_id)
        .and_then(|frame| frame.children_keys.get("BorderFrame"))
        .copied()
        .expect("world map border frame should exist");

    let prompt = state.widgets.get(prompt_id).unwrap();
    let border = state.widgets.get(border_id).unwrap();

    assert_eq!(prompt.frame_strata.as_str(), "LOW");
    assert_eq!(border.frame_strata.as_str(), "HIGH");

    let prompt_pos = flattened
        .iter()
        .position(|&id| id == prompt_id)
        .expect("voice prompt should be in render list");
    let border_pos = flattened
        .iter()
        .position(|&id| id == border_id)
        .expect("world map border should be in render list");

    assert!(
        prompt_pos < border_pos,
        "voice prompt should render before world map border when both overlap; prompt_pos={prompt_pos}, border_pos={border_pos}"
    );
}

#[test]
fn chat_frame_voice_button_overlaps_world_map_but_renders_below_panel_border() {
    let mut roots = ISOLATED_WORLD_MAP_ROOT_ADDONS.to_vec();
    roots.extend(["Blizzard_ChatFrame", "Blizzard_Channels"]);
    let env = env_with_root_addons_ui(&roots);
    open_world_map(&env);

    let buckets = build_strata_buckets(&env);
    let flattened: Vec<u64> = buckets.iter().flatten().copied().collect();
    let state = env.state().borrow();

    let world_map_id = state
        .widgets
        .get_id_by_name("WorldMapFrame")
        .expect("world map should exist");
    let border_id = state
        .widgets
        .get(world_map_id)
        .and_then(|frame| frame.children_keys.get("BorderFrame"))
        .copied()
        .expect("world map border frame should exist");
    let voice_button_id = state
        .widgets
        .get_id_by_name("ChatFrameChannelButton")
        .expect("chat voice button should exist");
    let voice_icon_id = state
        .widgets
        .get(voice_button_id)
        .and_then(|frame| frame.children_keys.get("Icon"))
        .copied()
        .expect("chat voice button icon should exist");

    let world_map_rect = compute_frame_rect(&state.widgets, world_map_id, 1024.0, 768.0);
    let voice_button_rect = compute_frame_rect(&state.widgets, voice_button_id, 1024.0, 768.0);
    let voice_icon = state.widgets.get(voice_icon_id).unwrap();
    let border = state.widgets.get(border_id).unwrap();

    assert_eq!(
        voice_icon.atlas.as_deref(),
        Some("chatframe-button-icon-voicechat")
    );
    assert_eq!(border.frame_strata.as_str(), "HIGH");

    let overlaps_horizontally = voice_button_rect.x < world_map_rect.x + world_map_rect.width
        && voice_button_rect.x + voice_button_rect.width > world_map_rect.x;
    let overlaps_vertically = voice_button_rect.y < world_map_rect.y + world_map_rect.height
        && voice_button_rect.y + voice_button_rect.height > world_map_rect.y;
    assert!(
        overlaps_horizontally && overlaps_vertically,
        "chat voice button should overlap world map bounds at 1024x768; button={voice_button_rect:?} map={world_map_rect:?}"
    );

    let voice_button_pos = flattened
        .iter()
        .position(|&id| id == voice_button_id)
        .expect("chat voice button should be in render list");
    let border_pos = flattened
        .iter()
        .position(|&id| id == border_id)
        .expect("world map border should be in render list");

    assert!(
        voice_button_pos < border_pos,
        "chat voice button should render before world map border even though the layouts overlap; button_pos={voice_button_pos}, border_pos={border_pos}"
    );
}

// ============================================================================
// High frame_level border must not cover lower-level content
// ============================================================================

/// Reproduces the world map quest log bug: a decorative BorderFrame at
/// frame_level 100 covers quest POI icons at level 5-7 because the DFS
/// emits the border's texture AFTER the content children.
///
/// In WoW, a border frame's textures (edges/corners) render as part of
/// that frame's draw layer — they should not occlude child content at
/// lower frame_levels in the same parent.
#[test]
fn high_level_border_does_not_cover_lower_level_content() {
    let env = env_with_shared_xml();

    // Replicate QuestScrollFrame structure:
    // - ScrollFrame with a Background texture (BACKGROUND layer)
    // - A content child with an icon texture (ARTWORK layer)
    // - A BorderFrame child at frame_level 100 with a covering texture
    env.exec(
        r#"
        local panel = CreateFrame("Frame", "TestPanel", UIParent)
        panel:SetSize(300, 400)
        panel:SetPoint("CENTER")
        panel:Show()

        -- Background texture on the panel (like QuestLog-main-background)
        local bg = panel:CreateTexture("TestPanelBg", "BACKGROUND")
        bg:SetAllPoints()
        bg:SetColorTexture(0.1, 0.1, 0.1, 1)

        -- Content child at default frame_level (like quest entries)
        local content = CreateFrame("Frame", "TestContent", panel)
        content:SetAllPoints()
        content:Show()

        -- Icon texture on content (like POI button icon at ARTWORK layer)
        local icon = content:CreateTexture("TestIcon", "ARTWORK")
        icon:SetSize(20, 20)
        icon:SetPoint("CENTER")
        icon:SetColorTexture(1, 0, 0, 1)

        -- Decorative border at high frame_level (like ScrollFrameTemplate BorderFrame)
        local border = CreateFrame("Frame", "TestBorder", panel)
        border:SetAllPoints()
        border:SetFrameLevel(100)
        border:Show()

        -- Border texture covers the whole area (like the Border texture at level 100)
        local borderTex = border:CreateTexture("TestBorderTex", "ARTWORK")
        borderTex:SetAllPoints()
        borderTex:SetColorTexture(0, 0, 0, 0.8)
    "#,
    )
    .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();

    let icon_id = state.widgets.get_id_by_name("TestIcon").unwrap();
    let border_tex_id = state.widgets.get_id_by_name("TestBorderTex").unwrap();

    // Find both IDs in the strata bucket and check their order.
    // The icon MUST render AFTER the border texture so it appears on top.
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];

    let icon_pos = medium_bucket.iter().position(|&id| id == icon_id);
    let border_pos = medium_bucket.iter().position(|&id| id == border_tex_id);

    assert!(
        icon_pos.is_some(),
        "TestIcon should be in the MEDIUM strata bucket"
    );
    assert!(
        border_pos.is_some(),
        "TestBorderTex should be in the MEDIUM strata bucket"
    );

    let icon_pos = icon_pos.unwrap();
    let border_pos = border_pos.unwrap();

    assert!(
        icon_pos > border_pos,
        "Content icon (pos={icon_pos}) must render AFTER border texture (pos={border_pos}). \
         A decorative border at high frame_level should not cover lower-level content."
    );
}

#[test]
fn late_created_texture_invalidates_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "LateBucketParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()

        local first = parent:CreateTexture("LateBucketFirstTexture", "ARTWORK")
        first:SetSize(20, 20)
        first:SetPoint("CENTER")
        first:SetColorTexture(1, 0, 0, 1)
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);

    env.exec(
        r#"
        local second = LateBucketParent:CreateTexture("LateBucketSecondTexture", "OVERLAY")
        second:SetSize(18, 18)
        second:SetPoint("CENTER")
        second:SetColorTexture(0, 1, 0, 1)
    "#,
    )
    .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();
    let second_id = state
        .widgets
        .get_id_by_name("LateBucketSecondTexture")
        .expect("late-created texture should exist in widget registry");
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];

    assert!(
        medium_bucket.contains(&second_id),
        "late-created texture must appear in rebuilt strata bucket after CreateTexture"
    );
}

#[test]
fn late_created_frame_invalidates_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "LateBucketFrameParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);

    env.exec(
        r#"
        local child = CreateFrame("Frame", "LateBucketChildFrame", LateBucketFrameParent)
        child:SetSize(16, 16)
        child:SetPoint("CENTER")
        child:Show()
    "#,
    )
    .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();
    let child_id = state
        .widgets
        .get_id_by_name("LateBucketChildFrame")
        .expect("late-created frame should exist in widget registry");
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];

    assert!(
        medium_bucket.contains(&child_id),
        "late-created frame must appear in rebuilt strata bucket after CreateFrame"
    );
}

#[test]
fn late_set_draw_layer_invalidates_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "LateLayerParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()

        local circle = parent:CreateTexture("LateLayerCircle", "BACKGROUND")
        circle:SetAllPoints()
        circle:SetColorTexture(1, 0, 0, 1)

        local map = parent:CreateTexture("LateLayerMap", "ARTWORK")
        map:SetAllPoints()
        map:SetColorTexture(0, 1, 0, 1)

        local star = parent:CreateTexture("LateLayerStar", "OVERLAY")
        star:SetAllPoints()
        star:SetColorTexture(0, 0, 1, 1)
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);

    env.exec(r#"LateLayerCircle:SetDrawLayer("ARTWORK", 1)"#)
        .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];
    let circle_id = state
        .widgets
        .get_id_by_name("LateLayerCircle")
        .expect("circle texture should exist");
    let map_id = state
        .widgets
        .get_id_by_name("LateLayerMap")
        .expect("map texture should exist");
    let star_id = state
        .widgets
        .get_id_by_name("LateLayerStar")
        .expect("star texture should exist");

    let circle_pos = medium_bucket
        .iter()
        .position(|&id| id == circle_id)
        .expect("circle should be in MEDIUM bucket");
    let map_pos = medium_bucket
        .iter()
        .position(|&id| id == map_id)
        .expect("map should be in MEDIUM bucket");
    let star_pos = medium_bucket
        .iter()
        .position(|&id| id == star_id)
        .expect("star should be in MEDIUM bucket");

    assert!(
        map_pos < circle_pos && circle_pos < star_pos,
        "late SetDrawLayer should rebuild region ordering: expected map -> circle -> star, \
         got positions map={map_pos} circle={circle_pos} star={star_pos}"
    );
}

#[test]
fn same_draw_layer_preserves_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "SameLayerParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()

        local tex = parent:CreateTexture("SameLayerTexture", "ARTWORK")
        tex:SetAllPoints()
        tex:SetColorTexture(1, 0, 0, 1)
        tex:SetDrawLayer("OVERLAY", 2)
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);
    assert!(
        env.state().borrow().strata_buckets.is_some(),
        "building buckets should populate the cache"
    );

    env.exec(r#"SameLayerTexture:SetDrawLayer("OVERLAY", 2)"#)
        .unwrap();

    assert!(
        env.state().borrow().strata_buckets.is_some(),
        "no-op SetDrawLayer should not invalidate cached strata buckets"
    );
}

#[test]
fn late_set_frame_level_invalidates_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "LateLevelParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()

        local circleFrame = CreateFrame("Frame", "LateLevelCircleFrame", parent)
        circleFrame:SetAllPoints()
        circleFrame:SetFrameLevel(1)
        circleFrame:Show()

        local circle = circleFrame:CreateTexture("LateLevelCircle", "ARTWORK")
        circle:SetAllPoints()
        circle:SetColorTexture(1, 0, 0, 1)

        local mapFrame = CreateFrame("Frame", "LateLevelMapFrame", parent)
        mapFrame:SetAllPoints()
        mapFrame:SetFrameLevel(2)
        mapFrame:Show()

        local map = mapFrame:CreateTexture("LateLevelMap", "ARTWORK")
        map:SetAllPoints()
        map:SetColorTexture(0, 1, 0, 1)

        local starFrame = CreateFrame("Frame", "LateLevelStarFrame", parent)
        starFrame:SetAllPoints()
        starFrame:SetFrameLevel(4)
        starFrame:Show()

        local star = starFrame:CreateTexture("LateLevelStar", "ARTWORK")
        star:SetAllPoints()
        star:SetColorTexture(0, 0, 1, 1)
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);

    env.exec(r#"LateLevelCircleFrame:SetFrameLevel(3)"#)
        .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];
    let circle_frame_id = state
        .widgets
        .get_id_by_name("LateLevelCircleFrame")
        .expect("circle frame should exist");
    let map_frame_id = state
        .widgets
        .get_id_by_name("LateLevelMapFrame")
        .expect("map frame should exist");
    let star_frame_id = state
        .widgets
        .get_id_by_name("LateLevelStarFrame")
        .expect("star frame should exist");

    let circle_pos = medium_bucket
        .iter()
        .position(|&id| id == circle_frame_id)
        .expect("circle frame should be in MEDIUM bucket");
    let map_pos = medium_bucket
        .iter()
        .position(|&id| id == map_frame_id)
        .expect("map frame should be in MEDIUM bucket");
    let star_pos = medium_bucket
        .iter()
        .position(|&id| id == star_frame_id)
        .expect("star frame should be in MEDIUM bucket");

    assert!(
        map_pos < circle_pos && circle_pos < star_pos,
        "late SetFrameLevel should rebuild frame ordering: expected map -> circle -> star, \
         got positions map={map_pos} circle={circle_pos} star={star_pos}"
    );
}

#[test]
fn isolated_world_map_stack_opens_and_populates_world_quest_pins() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let world_map_shown: bool = env
            .eval("return WorldMapFrame ~= nil and WorldMapFrame:IsShown() == true")
            .expect("should query WorldMapFrame visibility");
        assert!(
            world_map_shown,
            "isolated world map stack should show WorldMapFrame"
        );

        let state = env.state().borrow();
        let loaded_addons: Vec<_> = state
            .addons
            .iter()
            .filter(|addon| addon.loaded)
            .map(|addon| addon.folder_name.clone())
            .collect();
        let world_quest_pairs = world_quest_pin_pairs(&state);
        eprintln!(
            "isolated world map loaded {} addons: {:?}",
            loaded_addons.len(),
            loaded_addons
        );
        eprintln!(
            "isolated world map visible world quest pin pairs={}",
            world_quest_pairs.len()
        );

        assert!(
            !world_quest_pairs.is_empty(),
            "isolated world map stack should still populate visible world quest pins"
        );
    });
}

#[test]
fn isolated_world_map_fog_of_war_renders_only_on_unexplored_half() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let fog_rect = {
            let state = env.state().borrow();
            let world_map_id = state
                .widgets
                .get_id_by_name("WorldMapFrame")
                .expect("isolated world map should create WorldMapFrame");
            let fog_pin_id = state
                .widgets
                .iter_ids()
                .find(|&id| {
                    state.widgets.get(id).is_some_and(|frame| {
                        frame
                            .object_type_name
                            .as_deref()
                            .is_some_and(|name| name.eq_ignore_ascii_case("FogOfWarFrame"))
                            && is_descendant_of(&state.widgets, id, world_map_id)
                    })
                })
                .expect("isolated world map should create a FogOfWarFrame pin");
            compute_frame_rect(&state.widgets, fog_pin_id, 1024.0, 768.0)
        };

        let mut visible_mgr = make_texture_manager().expect("texture directories should exist");
        let visible_batch = build_screenshot_like_batch(&env, 1024, 768, Some("WorldMapFrame"));
        let visible_render = render_to_image(&visible_batch, &mut visible_mgr, 1024, 768, None);

        env.exec(
            r#"
            local fogPin = WorldMapFrame:EnumeratePinsByTemplate("FogOfWarPinTemplate")()
            assert(fogPin, "missing fog pin")
            fogPin:Hide()
        "#,
        )
        .expect("failed to hide fog pin");
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let mut hidden_mgr = make_texture_manager().expect("texture directories should exist");
        let hidden_batch = build_screenshot_like_batch(&env, 1024, 768, Some("WorldMapFrame"));
        let hidden_render = render_to_image(&hidden_batch, &mut hidden_mgr, 1024, 768, None);
        let fog_diff =
            diff_bounds(&visible_render, &hidden_render, 12).expect("fog should change pixels");

        assert!(
            fog_diff.0 as f32 >= fog_rect.x + fog_rect.width * 0.49,
            "fog should start on the unexplored half: diff={fog_diff:?} fog_rect={fog_rect:?}"
        );
        assert!(
            fog_diff.2 as f32 >= fog_rect.x + fog_rect.width * 0.92,
            "fog should reach the right side of the fog frame: diff={fog_diff:?} fog_rect={fog_rect:?}"
        );
        assert!(
            fog_diff.2 as f32 <= fog_rect.x + fog_rect.width + 2.0,
            "fog should not extend beyond the fog frame: diff={fog_diff:?} fog_rect={fog_rect:?}"
        );
        assert!(
            fog_diff.1 as f32 <= fog_rect.y + 2.0
                && fog_diff.3 as f32 >= fog_rect.y + fog_rect.height - 2.0,
            "fog should cover the full fog-frame height: diff={fog_diff:?} fog_rect={fog_rect:?}"
        );
    });
}

#[test]
fn isolated_world_map_exploration_overlay_renders_on_explored_half() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let map_rect = {
            let state = env.state().borrow();
            let world_map_id = state
                .widgets
                .get_id_by_name("WorldMapFrame")
                .expect("isolated world map should create WorldMapFrame");
            let fog_pin_id = state
                .widgets
                .iter_ids()
                .find(|&id| {
                    state.widgets.get(id).is_some_and(|frame| {
                        frame
                            .object_type_name
                            .as_deref()
                            .is_some_and(|name| name.eq_ignore_ascii_case("FogOfWarFrame"))
                            && is_descendant_of(&state.widgets, id, world_map_id)
                    })
                })
                .expect("isolated world map should create a FogOfWarFrame pin");
            compute_frame_rect(&state.widgets, fog_pin_id, 1024.0, 768.0)
        };
        let expected_overlay_bounds: (f32, f32, f32, f32) = env
            .eval(
                r#"
                local mapID = C_Map.GetCurrentMapID()
                local layer = C_Map.GetMapArtLayers(mapID)[1]
                local explored = C_MapExplorationInfo.GetExploredMapTextures(mapID)
                assert(type(explored) == "table" and #explored > 0, "missing explored overlays")

                local minLeft = math.huge
                local minTop = math.huge
                local maxRight = 0
                local maxBottom = 0

                for _, overlay in ipairs(explored) do
                    minLeft = math.min(minLeft, overlay.offsetX)
                    minTop = math.min(minTop, overlay.offsetY)
                    maxRight = math.max(maxRight, overlay.offsetX + overlay.textureWidth)
                    maxBottom = math.max(maxBottom, overlay.offsetY + overlay.textureHeight)
                end

                return minLeft / layer.layerWidth,
                    minTop / layer.layerHeight,
                    maxRight / layer.layerWidth,
                    maxBottom / layer.layerHeight
            "#,
            )
            .expect("failed to compute expected exploration bounds");

        let mut visible_mgr = make_texture_manager().expect("texture directories should exist");
        let visible_batch = build_screenshot_like_batch(&env, 1024, 768, Some("WorldMapFrame"));
        let visible_render = render_to_image(&visible_batch, &mut visible_mgr, 1024, 768, None);

        env.exec(
            r#"
            local pin = WorldMapFrame:EnumeratePinsByTemplate("MapExplorationPinTemplate")()
            assert(pin, "missing exploration pin")
            pin:Hide()
        "#,
        )
        .expect("failed to hide exploration pin");
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let mut hidden_mgr = make_texture_manager().expect("texture directories should exist");
        let hidden_batch = build_screenshot_like_batch(&env, 1024, 768, Some("WorldMapFrame"));
        let hidden_render = render_to_image(&hidden_batch, &mut hidden_mgr, 1024, 768, None);
        let overlay_diff = diff_bounds(&visible_render, &hidden_render, 12)
            .expect("exploration overlay should change pixels");
        let expected_left = map_rect.x + map_rect.width * expected_overlay_bounds.0;
        let expected_top = map_rect.y + map_rect.height * expected_overlay_bounds.1;
        let expected_right = map_rect.x + map_rect.width * expected_overlay_bounds.2;
        let expected_bottom = map_rect.y + map_rect.height * expected_overlay_bounds.3;
        let tolerance = 32.0;

        assert!(
            (overlay_diff.0 as f32 - expected_left).abs() <= tolerance,
            "exploration overlay should start where the API overlay data starts: diff={overlay_diff:?} expected_left={expected_left} rect={map_rect:?}"
        );
        assert!(
            (overlay_diff.1 as f32 - expected_top).abs() <= tolerance,
            "exploration overlay should start at the expected top bound: diff={overlay_diff:?} expected_top={expected_top} rect={map_rect:?}"
        );
        assert!(
            (overlay_diff.2 as f32 - expected_right).abs() <= tolerance,
            "exploration overlay should end where the API overlay data ends: diff={overlay_diff:?} expected_right={expected_right} rect={map_rect:?}"
        );
        assert!(
            (overlay_diff.3 as f32 - expected_bottom).abs() <= tolerance,
            "exploration overlay should end at the expected bottom bound: diff={overlay_diff:?} expected_bottom={expected_bottom} rect={map_rect:?}"
        );
    });
}

#[test]
fn isolated_world_map_seeded_world_quests_do_not_show_expiration_clock() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let lua_probe: String = env
            .eval(
                r#"
                local pin = WorldMapFrame and WorldMapFrame:EnumeratePinsByTemplate("WorldMap_WorldQuestPinTemplate")()
                return string.format(
                    "seconds=%s low=%s critical=%s timelow=%s",
                    tostring(C_TaskQuest.GetQuestTimeLeftSeconds(90101)),
                    tostring(QuestUtils_IsQuestWithinLowTimeThreshold(90101)),
                    tostring(QuestUtils_IsQuestWithinCriticalTimeThreshold(90101)),
                    tostring(pin and pin.TimeLowFrame and pin.TimeLowFrame:IsShown())
                )
                "#,
            )
            .expect("lua probe should run");
        eprintln!("{lua_probe}");

        let state = env.state().borrow();
        let visible_clock_icons: Vec<_> = state
            .widgets
            .iter_ids()
            .filter(|&id| {
                let Some(frame) = state.widgets.get(id) else {
                    return false;
                };
                frame.effective_alpha > 0.0
                    && frame.atlas.as_deref() == Some("worldquest-icon-clock")
            })
            .collect();

        assert!(
            visible_clock_icons.is_empty(),
            "seeded world quests default to 120 minutes left, so expiration clocks should stay hidden; visible clock ids={visible_clock_icons:?}"
        );
    });
}

#[test]
fn isolated_world_map_world_quest_circle_keeps_atlas_size() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let state = env.state().borrow();
        let (_circle_id, _icon_id, circle_rect, _circle_request_path, _icon_request_path) =
            world_quest_pin_pairs(&state)
                .into_iter()
                .next()
                .expect("isolated world map should have at least one world quest pair");

        assert!(
            (circle_rect.width - 32.0).abs() <= 0.1 && (circle_rect.height - 32.0).abs() <= 0.1,
            "world quest NormalTexture should keep its 32x32 atlas-sized rect, got {circle_rect:?}"
        );
    });
}
