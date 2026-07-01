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

fn create_active_cooldown(app: &App) -> u64 {
    let env = app.env.borrow();
    env.exec(
        r#"
        local cooldown = CreateFrame("Cooldown", "ActiveCooldownTick", UIParent)
        cooldown:SetSize(36, 36)
        cooldown:SetPoint("CENTER")
        cooldown:SetCooldown(GetTime(), 30)
    "#,
    )
    .expect("active cooldown should be created");

    let state = env.state().borrow();
    let cooldown_id = state
        .widgets
        .get_id_by_name("ActiveCooldownTick")
        .expect("cooldown widget should be registered");
    let _ = state.widgets.take_render_dirty_with_ids();
    cooldown_id
}
