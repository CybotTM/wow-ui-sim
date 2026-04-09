#![cfg(feature = "gui")]

use std::path::PathBuf;

use wow_ui_sim::iced_app::build_quad_batch_for_registry;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn setup_full_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        if let Err(e) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }
    env.apply_post_load_workarounds();
    wow_ui_sim::startup::fire_startup_events(&env);
    env.apply_post_event_workarounds();
    wow_ui_sim::startup::process_pending_timers(&env);
    wow_ui_sim::startup::fire_one_on_update_tick(&env);
    let _ = wow_ui_sim::lua_api::globals::global_frames::hide_runtime_hidden_frames(env.lua());
    env
}

fn open_class_talent_frame(env: &WowLuaEnv) {
    env.exec("PlayerSpellsUtil.ToggleClassTalentFrame()")
        .expect("Failed to open class talent frame");
}

fn quad_bounds(
    batch: &wow_ui_sim::render::QuadBatch,
    request: &wow_ui_sim::render::TextureRequest,
) -> (f32, f32, f32, f32) {
    let start = request.vertex_start as usize;
    let end = start + request.vertex_count as usize;
    let verts = &batch.vertices[start..end];
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

#[test]
fn hero_spec_icon_and_mask_quads_match_layout_rect() {
    let env = setup_full_ui();
    open_class_talent_frame(&env);

    let (icon_rect, icon_path, mask_path) = {
        let state = env.state().borrow();
        let player_spells_id = state
            .widgets
            .get_id_by_name("PlayerSpellsFrame")
            .expect("PlayerSpellsFrame should exist");
        let talents_frame_id = *state
            .widgets
            .get(player_spells_id)
            .and_then(|frame| frame.children_keys.get("TalentsFrame"))
            .expect("TalentsFrame child should exist");
        let hero_container_id = *state
            .widgets
            .get(talents_frame_id)
            .and_then(|frame| frame.children_keys.get("HeroTalentsContainer"))
            .expect("HeroTalentsContainer child should exist");
        let button_id = *state
            .widgets
            .get(hero_container_id)
            .and_then(|frame| frame.children_keys.get("HeroSpecButton"))
            .expect("HeroSpecButton child should exist");
        let button = state.widgets.get(button_id).unwrap();
        let icon_id = *button.children_keys.get("Icon1").expect("Icon1 child");
        let mask_id = *button
            .children_keys
            .get("IconMask")
            .expect("IconMask child");
        let icon_rect =
            wow_ui_sim::iced_app::compute_frame_rect(&state.widgets, icon_id, 1024.0, 768.0);
        let icon_path = state
            .widgets
            .get(icon_id)
            .and_then(|frame| frame.texture.clone())
            .expect("Icon1 should have a texture path");
        let mask_path = state
            .widgets
            .get(mask_id)
            .and_then(|frame| frame.texture.clone())
            .expect("IconMask should have a texture path");
        (icon_rect, icon_path, mask_path)
    };

    let buckets = {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    let batch = build_quad_batch_for_registry(
        &state.widgets,
        (1024.0, 768.0),
        None,
        None,
        None,
        None,
        None,
        None,
        &buckets,
    );

    let icon_crop_prefix = format!("{icon_path}@crop:");
    let icon_requests: Vec<_> = batch
        .texture_requests
        .iter()
        .filter(|request| request.path == icon_path || request.path.starts_with(&icon_crop_prefix))
        .collect();
    assert_eq!(
        icon_requests.len(),
        1,
        "HeroSpecButton.Icon1 should emit exactly one textured quad request"
    );
    assert_bounds_match_rect(
        quad_bounds(&batch, icon_requests[0]),
        icon_rect,
        "hero spec icon",
    );

    let mask_crop_prefix = format!("{mask_path}@crop:");
    let mask_requests: Vec<_> = batch
        .mask_texture_requests
        .iter()
        .filter(|request| request.path == mask_path || request.path.starts_with(&mask_crop_prefix))
        .collect();
    assert_eq!(
        mask_requests.len(),
        1,
        "HeroSpecButton.IconMask should emit exactly one mask quad request"
    );
    assert_bounds_match_rect(
        quad_bounds(&batch, mask_requests[0]),
        icon_rect,
        "hero spec mask",
    );
}
