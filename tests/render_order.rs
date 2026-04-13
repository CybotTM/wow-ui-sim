//! Render order tests: strata bucket ordering and z-order correctness.

mod common;

use common::env_with_shared_xml;
use std::path::PathBuf;
use wow_ui_sim::iced_app::{build_quad_batch_for_registry, compute_frame_rect};
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

/// Build strata buckets from a WowLuaEnv (mutable borrow), then return a clone.
fn build_strata_buckets(env: &wow_ui_sim::lua_api::WowLuaEnv) -> Vec<Vec<u64>> {
    let mut state = env.state().borrow_mut();
    let _ = state.get_strata_buckets();
    state.strata_buckets.as_ref().unwrap().clone()
}

fn build_quad_batch(env: &wow_ui_sim::lua_api::WowLuaEnv) -> wow_ui_sim::render::QuadBatch {
    {
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
    }
    let buckets = build_strata_buckets(env);
    let state = env.state().borrow();
    build_quad_batch_for_registry(
        &state.widgets,
        (1024.0, 768.0),
        None,
        None,
        None,
        None,
        None,
        None,
        &buckets,
    )
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

fn request_bounds(
    batch: &wow_ui_sim::render::QuadBatch,
    request: &wow_ui_sim::render::TextureRequest,
) -> (f32, f32, f32, f32) {
    let start = request.vertex_start as usize;
    let end = start + request.vertex_count as usize;
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for vertex in &batch.vertices[start..end] {
        min_x = min_x.min(vertex.position[0]);
        min_y = min_y.min(vertex.position[1]);
        max_x = max_x.max(vertex.position[0]);
        max_y = max_y.max(vertex.position[1]);
    }
    (min_x, min_y, max_x, max_y)
}

fn union_request_bounds(
    batch: &wow_ui_sim::render::QuadBatch,
    requests: &[&wow_ui_sim::render::TextureRequest],
) -> (f32, f32, f32, f32) {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for request in requests {
        let (x0, y0, x1, y1) = request_bounds(batch, request);
        min_x = min_x.min(x0);
        min_y = min_y.min(y0);
        max_x = max_x.max(x1);
        max_y = max_y.max(y1);
    }
    (min_x, min_y, max_x, max_y)
}

fn assert_bounds_match_rect(
    bounds: (f32, f32, f32, f32),
    rect: wow_ui_sim::LayoutRect,
    label: &str,
) {
    let tolerance = 0.1;
    assert!(
        (bounds.0 - rect.x).abs() <= tolerance,
        "{label} min_x should match rect.x: bounds={bounds:?} rect={rect:?}"
    );
    assert!(
        (bounds.1 - rect.y).abs() <= tolerance,
        "{label} min_y should match rect.y: bounds={bounds:?} rect={rect:?}"
    );
    assert!(
        (bounds.2 - (rect.x + rect.width)).abs() <= tolerance,
        "{label} max_x should match rect right edge: bounds={bounds:?} rect={rect:?}"
    );
    assert!(
        (bounds.3 - (rect.y + rect.height)).abs() <= tolerance,
        "{label} max_y should match rect bottom edge: bounds={bounds:?} rect={rect:?}"
    );
}

fn bounds_match_rect(bounds: (f32, f32, f32, f32), rect: wow_ui_sim::LayoutRect) -> bool {
    let tolerance = 0.1;
    (bounds.0 - rect.x).abs() <= tolerance
        && (bounds.1 - rect.y).abs() <= tolerance
        && (bounds.2 - (rect.x + rect.width)).abs() <= tolerance
        && (bounds.3 - (rect.y + rect.height)).abs() <= tolerance
}

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn env_with_world_map() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    for (name, toc_path) in discover_blizzard_addons(&ui) {
        if let Err(err) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("[load {name}] FAILED: {err}");
        }
    }

    env.apply_post_load_workarounds();
    wow_ui_sim::startup::settle_headless_startup(&env);
    env.exec("ToggleWorldMap()")
        .expect("failed to toggle world map after startup");
    wow_ui_sim::startup::process_pending_timers(&env);
    wow_ui_sim::startup::fire_one_on_update_tick(&env);
    env
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
fn world_map_tiles_render_after_tiled_background() {
    common::with_timeout(120, move || {
        let env = env_with_world_map();
        let buckets = build_strata_buckets(&env);
        let state = env.state().borrow();
        let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];

        let tiled_background_id = state
            .widgets
            .iter_ids()
            .find(|&id| {
                let Some(frame) = state.widgets.get(id) else {
                    return false;
                };
                frame.parent_key.as_deref() == Some("TiledBackground")
            })
            .expect("expected WorldMapFrame.ScrollContainer.Child.TiledBackground");

        let map_tile_id = state
            .widgets
            .iter_ids()
            .find(|&id| {
                let Some(frame) = state.widgets.get(id) else {
                    return false;
                };
                frame.texture
                    .as_deref()
                    .is_some_and(|path| path.starts_with("Interface\\WorldMap\\"))
            })
            .expect("expected a visible world map tile texture");

        let tiled_background_pos = medium_bucket
            .iter()
            .position(|&id| id == tiled_background_id)
            .expect("tiled background should be in MEDIUM bucket");
        let map_tile_pos = medium_bucket
            .iter()
            .position(|&id| id == map_tile_id)
            .expect("world map tile should be in MEDIUM bucket");

        assert!(
            map_tile_pos > tiled_background_pos,
            "world map tile (pos={map_tile_pos}) must render after tiled background \
             (pos={tiled_background_pos}) so the map is visible"
        );
    });
}

#[test]
fn world_quest_pin_icon_renders_after_world_map_tiles() {
    common::with_timeout(120, move || {
        let env = env_with_world_map();
        let buckets = build_strata_buckets(&env);
        let state = env.state().borrow();
        let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];

        let map_tile_ids: Vec<_> = state
            .widgets
            .iter_ids()
            .filter(|&id| {
                let Some(frame) = state.widgets.get(id) else {
                    return false;
                };
                frame.texture
                    .as_deref()
                    .is_some_and(|path| path.starts_with("Interface\\WorldMap\\"))
            })
            .collect();
        assert!(
            !map_tile_ids.is_empty(),
            "expected at least one visible world map tile texture"
        );

        let world_quest_icon_id = state
            .widgets
            .iter_ids()
            .find(|&id| {
                let Some(frame) = state.widgets.get(id) else {
                    return false;
                };
                frame.atlas.as_deref() == Some("Worldquest-icon")
            })
            .expect("expected a visible world quest pin icon texture");
        let display_frame_id = state
            .widgets
            .get(world_quest_icon_id)
            .and_then(|frame| frame.parent_id)
            .expect("world quest icon should have display-frame parent");
        let button_id = state
            .widgets
            .get(display_frame_id)
            .and_then(|frame| frame.parent_id)
            .expect("display frame should have button parent");
        let button_texture_id = state
            .widgets
            .iter_ids()
            .find(|&id| {
                let Some(frame) = state.widgets.get(id) else {
                    return false;
                };
                frame.parent_id == Some(button_id)
                    && frame.atlas.as_deref() == Some("UI-QuestPoi-QuestNumber")
            })
            .expect("expected world quest button number texture");

        let map_tile_positions: Vec<_> = map_tile_ids
            .iter()
            .filter_map(|&id| medium_bucket.iter().position(|&bucket_id| bucket_id == id))
            .collect();
        assert!(
            !map_tile_positions.is_empty(),
            "expected at least one world map tile in MEDIUM bucket"
        );
        let max_map_tile_pos = *map_tile_positions
            .iter()
            .max()
            .expect("world map tiles should have at least one position");
        let button_texture_pos = medium_bucket
            .iter()
            .position(|&id| id == button_texture_id)
            .expect("world quest button texture should be in MEDIUM bucket");
        let world_quest_icon_pos = medium_bucket
            .iter()
            .position(|&id| id == world_quest_icon_id)
            .expect("world quest icon should be in MEDIUM bucket");

        assert!(
            world_quest_icon_pos > max_map_tile_pos,
            "world quest icon (pos={world_quest_icon_pos}) must render after world map tile \
             range ending at pos={max_map_tile_pos} so pins stay above the map art"
        );
        assert!(
            button_texture_pos > max_map_tile_pos,
            "world quest button circle (pos={button_texture_pos}) must render after world map \
             tile range ending at pos={max_map_tile_pos} so the pin base stays above the map art"
        );
        assert!(
            medium_bucket
                .iter()
                .position(|&id| id == button_id)
                .is_some_and(|button_pos| button_pos > max_map_tile_pos),
            "world quest button frame must sort after the world map tile range"
        );
    });
}

#[test]
fn world_quest_star_and_circle_emit_live_quads_after_map_tiles() {
    common::with_timeout(120, move || {
        let env = env_with_world_map();
        let (
            map_tile_texture,
            circle_rect,
            circle_request_path,
            icon_rect,
            icon_request_path,
            circle_id,
            icon_id,
        ) = {
            let state = env.state().borrow();
            let map_tile_id = state
                .widgets
                .iter_ids()
                .find(|&id| {
                    let Some(frame) = state.widgets.get(id) else {
                        return false;
                    };
                    frame.texture
                        .as_deref()
                        .is_some_and(|path| path.starts_with("Interface\\WorldMap\\"))
                })
                .expect("expected a visible world map tile texture");
            let icon_id = state
                .widgets
                .iter_ids()
                .find(|&id| {
                    let Some(frame) = state.widgets.get(id) else {
                        return false;
                    };
                    frame.atlas.as_deref() == Some("Worldquest-icon")
                })
                .expect("expected a visible world quest pin icon texture");
            let display_frame_id = state
                .widgets
                .get(icon_id)
                .and_then(|frame| frame.parent_id)
                .expect("world quest icon should have display-frame parent");
            let button_id = state
                .widgets
                .get(display_frame_id)
                .and_then(|frame| frame.parent_id)
                .expect("display frame should have button parent");
            let circle_id = state
                .widgets
                .iter_ids()
                .find(|&id| {
                    let Some(frame) = state.widgets.get(id) else {
                        return false;
                    };
                    frame.parent_id == Some(button_id)
                        && frame.atlas.as_deref() == Some("UI-QuestPoi-QuestNumber")
                })
                .expect("expected world quest button circle texture");
            let map_tile_texture = state
                .widgets
                .get(map_tile_id)
                .and_then(|frame| frame.texture.clone())
                .expect("map tile should have texture path");
            let circle_texture = state
                .widgets
                .get(circle_id)
                .and_then(|frame| frame.texture.clone())
                .expect("world quest circle should have texture path");
            let circle_request_path = request_path_for_frame_texture(
                &circle_texture,
                state
                    .widgets
                    .get(circle_id)
                    .and_then(|frame| frame.atlas_tex_coords),
            );
            let icon_texture = state
                .widgets
                .get(icon_id)
                .and_then(|frame| frame.texture.clone())
                .expect("world quest icon should have texture path");
            let icon_request_path = request_path_for_frame_texture(
                &icon_texture,
                state
                    .widgets
                    .get(icon_id)
                    .and_then(|frame| frame.atlas_tex_coords),
            );
            let circle_rect = compute_frame_rect(&state.widgets, circle_id, 1024.0, 768.0);
            let icon_rect = compute_frame_rect(&state.widgets, icon_id, 1024.0, 768.0);
            (
                map_tile_texture,
                circle_rect,
                circle_request_path,
                icon_rect,
                icon_request_path,
                circle_id,
                icon_id,
            )
        };

        let batch = build_quad_batch(&env);
        let map_tile_request = batch
            .texture_requests
            .iter()
            .find(|request| request.path.starts_with(&map_tile_texture))
            .or_else(|| {
                batch
                    .texture_requests
                    .iter()
                    .find(|request| request.path.starts_with("Interface\\WorldMap\\"))
            })
            .expect("world map tile texture request should exist");
        let circle_requests: Vec<_> = batch
            .texture_requests
            .iter()
            .filter(|request| request.path == circle_request_path.as_str())
            .filter(|request| bounds_match_rect(request_bounds(&batch, request), circle_rect))
            .collect();
        let icon_requests: Vec<_> = batch
            .texture_requests
            .iter()
            .filter(|request| request.path == icon_request_path.as_str())
            .filter(|request| bounds_match_rect(request_bounds(&batch, request), icon_rect))
            .collect();

        assert!(
            !circle_requests.is_empty(),
            "world quest circle frame {circle_id} should emit at least one textured quad request"
        );
        assert!(
            !icon_requests.is_empty(),
            "world quest icon frame {icon_id} should emit at least one textured quad request"
        );

        assert_bounds_match_rect(
            union_request_bounds(&batch, &circle_requests),
            circle_rect,
            "world quest circle quad union",
        );
        assert_bounds_match_rect(
            union_request_bounds(&batch, &icon_requests),
            icon_rect,
            "world quest star quad union",
        );

        assert!(
            circle_requests.iter().all(|request| {
                let start = request.vertex_start as usize;
                let end = start + request.vertex_count as usize;
                batch.vertices[start..end]
                    .iter()
                    .any(|vertex| vertex.color[3] > 0.0)
            }),
            "world quest circle quads should have visible alpha"
        );
        assert!(
            icon_requests.iter().all(|request| {
                let start = request.vertex_start as usize;
                let end = start + request.vertex_count as usize;
                batch.vertices[start..end]
                    .iter()
                    .any(|vertex| vertex.color[3] > 0.0)
            }),
            "world quest star quads should have visible alpha"
        );

        let map_tile_start = map_tile_request.vertex_start;
        let first_circle_start = circle_requests
            .iter()
            .map(|request| request.vertex_start)
            .min()
            .expect("world quest circle request should have vertex_start");
        let first_icon_start = icon_requests
            .iter()
            .map(|request| request.vertex_start)
            .min()
            .expect("world quest icon request should have vertex_start");
        assert!(
            first_circle_start > map_tile_start,
            "world quest circle quads must emit after the map tile request"
        );
        assert!(
            first_icon_start > map_tile_start,
            "world quest star quads must emit after the map tile request"
        );
    });
}
