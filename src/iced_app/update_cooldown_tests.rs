use super::*;
use crate::iced_app::app::AppInit;
use crate::screen::ScreenKind;
use crate::texture::TextureManager;
use iced::Size;
use iced_runtime::Action;
use iced_runtime::futures::futures::StreamExt;
use iced_runtime::window::Action as WindowAction;
use rustc_hash::FxHashSet;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

fn build_test_app(screen_kind: ScreenKind) -> App {
    let env = Rc::new(RefCell::new(
        WowLuaEnv::new().expect("Failed to create Lua environment"),
    ));
    env.borrow().set_screen_mode(screen_kind);

    let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
    let font_system = Rc::new(RefCell::new(crate::render::WowFontSystem::new()));
    let glyph_atlas = Rc::new(RefCell::new(crate::render::GlyphAtlas::new()));
    let (_cmd_tx, cmd_rx) = mpsc::channel(1);
    let (_lua_tx, lua_rx) = std::sync::mpsc::channel();

    let app = App::build_app(AppInit {
        env,
        log_messages: Vec::new(),
        texture_manager,
        font_system,
        glyph_atlas,
        cmd_rx,
        lua_rx,
        debug_borders: false,
        debug_anchors: false,
        saved_vars: None,
        config: crate::config::SimConfig::default(),
    });
    app.gui_startup_complete.set(true);
    app
}

#[test]
fn process_timers_invalidates_active_cooldown_widgets() {
    let mut app = build_test_app(ScreenKind::Game);
    app.screen_size.set(Size::new(1024.0, 768.0));
    app.selected_rot_level = "Off".to_string();
    app.strata_dirty.set(0);
    app.textures_pending.set(false);
    *app.pending_dirty_ids.borrow_mut() = Some(FxHashSet::default());

    let cooldown_id = create_active_cooldown(&app);

    let task = app.handle_process_timers(Instant::now());
    let action = pollster::block_on(async {
        iced_runtime::task::into_stream(task)
            .expect("active cooldown should request a redraw")
            .next()
            .await
            .expect("task should emit a redraw action")
    });

    assert!(
        matches!(action, Action::Window(WindowAction::RedrawAll)),
        "active cooldown progress should request periodic redraws"
    );
    assert_ne!(
        app.strata_dirty.get(),
        0,
        "active cooldown should dirty its render strata"
    );
    assert!(
        app.pending_dirty_ids
            .borrow()
            .as_ref()
            .is_some_and(|ids| ids.contains(&cooldown_id)),
        "active cooldown should use incremental dirty IDs"
    );
}

#[test]
fn process_timers_invalidates_slow_mod_rate_cooldowns_after_real_duration() {
    let mut app = build_test_app(ScreenKind::Game);
    prepare_incremental_tick(&mut app);

    let cooldown_id = create_cooldown(&app, "SlowModRateActiveTick", 11.0, 10.0, 0.5);

    let task = app.handle_process_timers(Instant::now());
    let action = pollster::block_on(async {
        iced_runtime::task::into_stream(task)
            .expect("slow mod-rate cooldown should request a redraw")
            .next()
            .await
            .expect("task should emit a redraw action")
    });

    assert!(
        matches!(action, Action::Window(WindowAction::RedrawAll)),
        "slow mod-rate cooldown progress should still redraw after real duration"
    );
    assert_pending_dirty_contains(&app, cooldown_id);
}

#[test]
fn process_timers_marks_fast_mod_rate_cooldown_completion_once() {
    let mut app = build_test_app(ScreenKind::Game);
    prepare_incremental_tick(&mut app);

    let cooldown_id = create_cooldown(&app, "FastModRateCompletionTick", 4.9, 10.0, 2.0);
    let _ = app.handle_process_timers(Instant::now());
    clear_tick_dirty(&app);

    set_cooldown(&app, "FastModRateCompletionTick", 5.1, 10.0, 2.0);
    clear_tick_dirty(&app);

    let _ = app.handle_process_timers(Instant::now());
    assert_pending_dirty_contains(&app, cooldown_id);

    clear_tick_dirty(&app);
    let _ = app.handle_process_timers(Instant::now());
    assert_pending_dirty_excludes(&app, cooldown_id);
}

fn create_active_cooldown(app: &App) -> u64 {
    create_cooldown(app, "ActiveCooldownTick", 0.0, 30.0, 1.0)
}

fn prepare_incremental_tick(app: &mut App) {
    app.screen_size.set(Size::new(1024.0, 768.0));
    app.selected_rot_level = "Off".to_string();
    app.strata_dirty.set(0);
    app.textures_pending.set(false);
    *app.pending_dirty_ids.borrow_mut() = Some(FxHashSet::default());
}

fn create_cooldown(app: &App, name: &str, elapsed: f64, duration: f64, mod_rate: f64) -> u64 {
    let env = app.env.borrow();
    env.exec(&format!(
        r#"
        local cooldown = CreateFrame("Cooldown", "{name}", UIParent)
        cooldown:SetSize(36, 36)
        cooldown:SetPoint("CENTER")
        cooldown:SetCooldown(GetTime() - {elapsed}, {duration}, {mod_rate})
    "#,
    ))
    .expect("cooldown should be created");

    let state = env.state().borrow();
    let cooldown_id = state
        .widgets
        .get_id_by_name(name)
        .expect("cooldown widget should be registered");
    let _ = state.widgets.take_render_dirty_with_ids();
    cooldown_id
}

fn set_cooldown(app: &App, name: &str, elapsed: f64, duration: f64, mod_rate: f64) {
    app.env
        .borrow()
        .exec(&format!(
            r#"
            {name}:SetCooldown(GetTime() - {elapsed}, {duration}, {mod_rate})
        "#,
        ))
        .expect("cooldown should be updated");
}

fn clear_tick_dirty(app: &App) {
    app.strata_dirty.set(0);
    *app.pending_dirty_ids.borrow_mut() = Some(FxHashSet::default());
    let env = app.env.borrow();
    let state = env.state().borrow();
    let _ = state.widgets.take_render_dirty_with_ids();
}

fn assert_pending_dirty_contains(app: &App, cooldown_id: u64) {
    assert!(
        app.pending_dirty_ids
            .borrow()
            .as_ref()
            .is_some_and(|ids| ids.contains(&cooldown_id)),
        "cooldown should use incremental dirty IDs"
    );
}

fn assert_pending_dirty_excludes(app: &App, cooldown_id: u64) {
    assert!(
        !app.pending_dirty_ids
            .borrow()
            .as_ref()
            .is_some_and(|ids| ids.contains(&cooldown_id)),
        "completed cooldown should not stay dirty after final redraw"
    );
}
