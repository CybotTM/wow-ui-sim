use std::path::PathBuf;
use std::time::Duration;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::{WowLuaEnv, compute_frame_rect};
use wow_ui_sim::startup::{fire_one_on_update_tick, fire_startup_events, process_pending_timers};

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn full_game_env_after_edit_mode_init() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1600.0, 1200.0);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    load_all_blizzard_addons(&env);
    settle_env_after_edit_mode_init(&env);

    env
}

fn load_all_blizzard_addons(env: &WowLuaEnv) {
    let ui = blizzard_ui_dir();
    for (name, toc_path) in &discover_blizzard_addons(&ui) {
        if let Err(err) = load_addon(&env.loader_env(), toc_path) {
            panic!("[load {name}] FAILED: {err}");
        }
    }
}

fn settle_env_after_edit_mode_init(env: &WowLuaEnv) {
    env.apply_post_load_workarounds();
    fire_startup_events(env);
    env.apply_post_event_workarounds();
    env.state().borrow_mut().widgets.rebuild_anchor_index();
    process_pending_timers(env);
    fire_one_on_update_tick(env);
    let _ = wow_ui_sim::lua_api::globals::global_frames::hide_runtime_hidden_frames(env.lua());

    std::thread::sleep(Duration::from_secs(2));
    for _ in 0..3 {
        env.state().borrow_mut().ensure_layout_rects();
        fire_one_on_update_tick(env);
        process_pending_timers(env);
    }
}

fn bag_bar_anchor(env: &WowLuaEnv) -> (String, String, String, f32, f32) {
    env.eval(
        r#"
        local point, rel, relPoint, x, y = BagsBar:GetPoint(1)
        return point, rel:GetName(), relPoint, x, y
        "#,
    )
    .unwrap()
}

fn frame_rect(env: &WowLuaEnv, name: &str) -> (f32, f32, f32, f32) {
    let state = env.state().borrow();
    let id = state
        .widgets
        .get_id_by_name(name)
        .unwrap_or_else(|| panic!("Frame '{name}' not found"));
    let rect = compute_frame_rect(&state.widgets, id, 1600.0, 1200.0);
    (rect.x, rect.y, rect.width, rect.height)
}

#[test]
fn bags_bar_reanchor_replay_moves_the_bar_off_its_correct_rect() {
    let env = full_game_env_after_edit_mode_init();

    let before_anchor = bag_bar_anchor(&env);
    assert_eq!(
        (
            before_anchor.0.clone(),
            before_anchor.1.clone(),
            before_anchor.2.clone(),
        ),
        (
            "TOPRIGHT".to_string(),
            "MicroButtonAndBagsBar".to_string(),
            "TOPRIGHT".to_string(),
        )
    );

    let before_rect = frame_rect(&env, "BagsBar");
    assert_eq!(before_rect, (1386.0, 1104.0, 208.0, 47.0));

    env.exec(
        r#"
        BagsBar:ClearAllPoints()
        BagsBar:SetPoint("TOPRIGHT", MicroButtonAndBagsBar, "TOPRIGHT", 0, 0)
        "#,
    )
    .unwrap();
    env.state().borrow_mut().widgets.rebuild_anchor_index();
    env.state().borrow_mut().ensure_layout_rects();

    let after_anchor = bag_bar_anchor(&env);
    assert_eq!(
        after_anchor,
        (
            "TOPRIGHT".to_string(),
            "MicroButtonAndBagsBar".to_string(),
            "TOPRIGHT".to_string(),
            0.0,
            0.0,
        )
    );
    let after_rect = frame_rect(&env, "BagsBar");
    assert_eq!(after_rect, (1386.0, 1114.0, 208.0, 47.0));
    assert_ne!(after_rect, before_rect);
}

#[test]
fn main_menu_bag_buttons_have_correct_item_context_without_overlay_replay() {
    let env = full_game_env_after_edit_mode_init();

    let (
        util_exists,
        bag_match_is_dna,
        backpack_match_is_dna,
        bag_overlay_hidden,
        backpack_overlay_hidden,
    ): (bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            local dna = ItemButtonUtil and ItemButtonUtil.ItemContextMatchResult
                and ItemButtonUtil.ItemContextMatchResult.DoesNotApply
            local bag = MainMenuBarBagManager and MainMenuBarBagManager.allBagButtons
                and MainMenuBarBagManager.allBagButtons[1]
            return type(ItemButtonUtil) == "table",
                bag and bag.itemContextMatchResult == dna or false,
                MainMenuBarBackpackButton.itemContextMatchResult == dna,
                bag and bag:GetItemContextOverlayMode() == nil or false,
                MainMenuBarBackpackButton:GetItemContextOverlayMode() == nil
            "#,
        )
        .unwrap();
    assert!(util_exists);
    assert!(bag_match_is_dna);
    assert!(backpack_match_is_dna);
    assert!(bag_overlay_hidden);
    assert!(backpack_overlay_hidden);
}
