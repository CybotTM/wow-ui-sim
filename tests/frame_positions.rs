//! Regression test: verify key frame positions match the origin/master baseline.
//!
//! Loads all Blizzard addons at 1024x768, fires startup events (same sequence
//! as the dump-tree/screenshot headless path), then checks that important UI
//! elements are positioned correctly. Expected values from origin/master dump-tree.
//!
//! Uses `harness = false` with a custom main to load the Blizzard UI once and
//! run all position checks against the shared environment.

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::{fire_one_on_update_tick, fire_startup_events, process_pending_timers};

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn create_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    let ui = blizzard_ui_dir();
    for (name, toc_path) in &discover_blizzard_addons(&ui) {
        if let Err(e) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }

    env.apply_post_load_workarounds();

    // Same sequence as run_headless_startup in main.rs
    fire_startup_events(&env);
    env.apply_post_event_workarounds();
    env.state().borrow_mut().widgets.rebuild_anchor_index();
    process_pending_timers(&env);
    fire_one_on_update_tick(&env);
    let _ = wow_ui_sim::lua_api::globals::global_frames::hide_runtime_hidden_frames(env.lua());

    // Allow timer-driven layout callbacks to become due (real wall clock via Instant)
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Extra update ticks — drain timers and fire OnUpdate (same as main.rs)
    for _ in 0..3 {
        env.state().borrow_mut().ensure_layout_rects();
        fire_one_on_update_tick(&env);
        process_pending_timers(&env);
    }

    env
}

/// Query a frame's computed rect: (x, y, width, height) via layout's compute_frame_rect.
fn frame_rect(env: &WowLuaEnv, name: &str) -> (f32, f32, f32, f32) {
    use wow_ui_sim::iced_app::layout::compute_frame_rect;
    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(name)
        .unwrap_or_else(|| panic!("Frame '{}' not found", name));
    let rect = compute_frame_rect(&state.widgets, id, 1024.0, 768.0);
    (rect.x, rect.y, rect.width, rect.height)
}

/// Assert frame position and size within tolerance (±1px for rounding).
fn assert_frame_rect(env: &WowLuaEnv, name: &str, ex: f32, ey: f32, ew: f32, eh: f32) {
    let (x, y, w, h) = frame_rect(env, name);
    let tol = 1.0;
    assert!(
        (x - ex).abs() <= tol && (y - ey).abs() <= tol
            && (w - ew).abs() <= tol && (h - eh).abs() <= tol,
        "{name}: expected ({ex}, {ey}, {ew}x{eh}), got ({x}, {y}, {w}x{h})"
    );
}

type TestCase = (&'static str, &'static str, f32, f32, f32, f32);

/// Frame name, expected x, y, width, height.
fn position_tests() -> Vec<TestCase> {
    vec![
        // Player / Target / Group frames
        ("player_frame_position", "PlayerFrame", 0.0, 418.0, 232.0, 100.0),
        ("target_frame_position", "TargetFrame", 792.0, 418.0, 232.0, 100.0),
        ("focus_frame_position", "FocusFrame", 850.0, 494.0, 174.0, 75.0),
        ("pet_frame_position", "PetFrame", 93.0, 535.0, 120.0, 49.0),
        ("paladin_power_bar_position", "PaladinPowerBarFrame", 73.0, 490.0, 150.0, 43.0),
        ("party_frame_position", "PartyFrame", 22.0, 147.0, 120.0, 244.0),
        ("compact_party_frame_position", "CompactPartyFrame", 22.0, 147.0, 90.0, 224.0),
        // HUD elements
        ("minimap_position", "Minimap", 807.0, 44.0, 198.0, 198.0),
        ("objective_tracker_position", "ObjectiveTrackerFrame", 759.0, 271.0, 260.0, 400.0),
        ("bags_bar_position", "BagsBar", 810.0, 672.0, 208.0, 47.0),
        ("micro_menu_position", "MicroMenu", 629.0, 717.0, 329.0, 40.0),
        ("buff_frame_position", "BuffFrame", 369.0, 10.0, 400.0, 135.0),
        ("debuff_frame_position", "DebuffFrame", 474.0, 155.0, 280.0, 90.0),
        ("chat_frame_position", "ChatFrame1", 35.0, 548.0, 430.0, 170.0),
        ("micro_menu_container_position", "MicroMenuContainer", 629.0, 717.0, 389.0, 45.0),
    ]
}

/// ActionButton1 only checks x position (y/size depend on bar layout).
fn check_action_button(env: &WowLuaEnv) {
    let (x, _y, _w, _h) = frame_rect(env, "ActionButton1");
    assert!((x - 512.0).abs() <= 1.0, "ActionButton1 x: expected 512, got {x}");
}

fn run_tests(env: &WowLuaEnv) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;

    for (name, frame, ex, ey, ew, eh) in &position_tests() {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            assert_frame_rect(env, frame, *ex, *ey, *ew, *eh);
        }));
        report_result(&result, name, &mut passed, &mut failed);
    }

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        check_action_button(env);
    }));
    report_result(&result, "action_button_1_position", &mut passed, &mut failed);

    (passed, failed)
}

fn report_result(
    result: &Result<(), Box<dyn std::any::Any + Send>>,
    name: &str,
    passed: &mut usize,
    failed: &mut usize,
) {
    match result {
        Ok(()) => {
            *passed += 1;
            eprintln!("test {name} ... ok");
        }
        Err(e) => {
            *failed += 1;
            let msg = e.downcast_ref::<String>()
                .map(|s| s.as_str())
                .or_else(|| e.downcast_ref::<&str>().copied())
                .unwrap_or("(unknown panic)");
            eprintln!("test {name} ... FAILED\n  {msg}");
        }
    }
}

fn main() {
    let env = create_env();
    let (passed, failed) = run_tests(&env);
    let total = passed + failed;

    eprintln!("\ntest result: {}. {passed} passed; {failed} failed; 0 ignored; \
              0 measured; 0 filtered out",
        if failed == 0 { "ok" } else { "FAILED" });

    if failed > 0 {
        std::process::exit(1);
    }
    assert_eq!(total, 16, "Expected 16 tests, ran {total}");
}
