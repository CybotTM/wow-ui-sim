use super::*;
use crate::screen::ScreenKind;
use crate::texture::TextureManager;
use iced::Size;
use tokio::sync::mpsc;

fn build_env() -> Rc<RefCell<WowLuaEnv>> {
    Rc::new(RefCell::new(
        WowLuaEnv::new().expect("Failed to create Lua environment"),
    ))
}

fn build_test_app(screen_kind: ScreenKind) -> App {
    let env = build_env();
    env.borrow().set_screen_mode(screen_kind);
    env.borrow().set_screen_size(800.0, 600.0);

    let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
    let font_system = Rc::new(RefCell::new(WowFontSystem::new()));
    let glyph_atlas = Rc::new(RefCell::new(GlyphAtlas::new()));
    let (_cmd_tx, cmd_rx) = mpsc::channel(1);
    let (_lua_tx, lua_rx) = std::sync::mpsc::channel();

    App::build_app(AppInit {
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
    })
}

#[test]
fn glue_screens_tick_on_update_at_interactive_rate() {
    let app = build_test_app(ScreenKind::CharacterSelect);
    app.strata_dirty.set(0);

    assert_eq!(app.screen_size.get(), Size::new(800.0, 600.0));
    assert_eq!(
        app.compute_tick_interval(),
        Some(std::time::Duration::from_millis(33)),
    );
}

#[test]
fn game_screen_keeps_idle_on_update_heartbeat() {
    let app = build_test_app(ScreenKind::Game);
    app.strata_dirty.set(0);

    assert_eq!(
        app.compute_tick_interval(),
        Some(std::time::Duration::from_secs(1)),
    );
}

#[test]
fn active_cooldown_widget_uses_fast_tick_interval() {
    let app = build_test_app(ScreenKind::Game);
    app.strata_dirty.set(0);
    app.textures_pending.set(false);

    app.env
        .borrow()
        .exec(
            r#"
            local cooldown = CreateFrame("Cooldown", "FastTickCooldown", UIParent)
            cooldown:SetCooldown(GetTime(), 30)
        "#,
        )
        .expect("active cooldown should be created");

    assert_eq!(
        app.compute_tick_interval(),
        Some(std::time::Duration::from_millis(DEFAULT_FAST_TICK_MS)),
    );
}

#[test]
fn slow_mod_rate_cooldown_keeps_fast_tick_after_real_duration() {
    let app = build_test_app(ScreenKind::Game);
    app.strata_dirty.set(0);
    app.textures_pending.set(false);

    app.env
        .borrow()
        .exec(
            r#"
            local cooldown = CreateFrame("Cooldown", "SlowModRateCooldown", UIParent)
            cooldown:SetCooldown(GetTime() - 11, 10, 0.5)
        "#,
        )
        .expect("slow mod-rate cooldown should be created");

    assert_eq!(
        app.compute_tick_interval(),
        Some(std::time::Duration::from_millis(DEFAULT_FAST_TICK_MS)),
    );
}

#[test]
fn fast_mod_rate_completed_cooldown_returns_to_idle_tick() {
    let app = build_test_app(ScreenKind::Game);
    app.strata_dirty.set(0);
    app.textures_pending.set(false);

    app.env
        .borrow()
        .exec(
            r#"
            local cooldown = CreateFrame("Cooldown", "FastModRateDoneCooldown", UIParent)
            cooldown:SetCooldown(GetTime() - 5.1, 10, 2)
        "#,
        )
        .expect("fast mod-rate cooldown should be created");

    assert_eq!(
        app.compute_tick_interval(),
        Some(std::time::Duration::from_secs(1)),
    );
}

#[test]
fn gui_startup_uses_first_real_canvas_size_for_display_size_changed() {
    let app = build_test_app(ScreenKind::Game);
    app.env
        .borrow()
        .exec(
            r#"
            __startup_display_width = nil
            __startup_display_height = nil
            local frame = CreateFrame("Frame")
            frame:RegisterEvent("DISPLAY_SIZE_CHANGED")
            frame:SetScript("OnEvent", function()
                __startup_display_width = GetScreenWidth()
                __startup_display_height = GetScreenHeight()
            end)
            "#,
        )
        .expect("startup size recorder should install");

    app.ensure_gui_startup_for_canvas_size(Size::new(1266.0, 822.0));

    let (width, height): (f64, f64) = app
        .env
        .borrow()
        .eval("return __startup_display_width, __startup_display_height")
        .expect("startup size should be readable");
    assert_eq!(width, 1266.0);
    assert_eq!(height, 822.0);
}

#[test]
fn app_screen_size_starts_from_sim_state_size() {
    let app = build_test_app(ScreenKind::Game);

    assert_eq!(app.screen_size.get(), current_env_screen_size(&app.env));
}

#[test]
fn gui_startup_drains_ready_timers_before_interactive_ticks() {
    let env = build_env();
    env.borrow()
        .exec(
            r#"
            __gui_startup_timer_fired = 0
            C_Timer.After(0, function()
                __gui_startup_timer_fired = __gui_startup_timer_fired + 1
            end)
            "#,
        )
        .expect("startup timer setup should succeed");

    App::run_startup_sequence(&env);

    let fired: f64 = env
        .borrow()
        .eval("return __gui_startup_timer_fired")
        .expect("startup timer result should be readable");
    assert_eq!(fired, 1.0, "ready startup timers should be settled");
}

#[test]
fn gui_startup_settles_bounded_on_update_work_before_interactive_ticks() {
    let env = build_env();
    env.borrow()
        .exec(
            r#"
            __gui_startup_on_update_fired = 0
            local frame = CreateFrame("Frame")
            frame:SetScript("OnUpdate", function(self)
                __gui_startup_on_update_fired = __gui_startup_on_update_fired + 1
                if __gui_startup_on_update_fired == 3 then
                    self:SetScript("OnUpdate", nil)
                end
            end)
            "#,
        )
        .expect("startup OnUpdate setup should succeed");

    App::run_startup_sequence(&env);

    let fired: f64 = env
        .borrow()
        .eval("return __gui_startup_on_update_fired")
        .expect("startup OnUpdate result should be readable");
    assert_eq!(
        fired, 3.0,
        "bounded startup OnUpdate work should be settled before GUI ticks"
    );
}

#[test]
fn gui_startup_closes_windows_created_by_startup_timers() {
    let env = build_env();
    env.borrow()
        .exec(
            r#"
            C_Timer.After(0, function()
                Baganator_WelcomeFrame = CreateFrame("Frame", "Baganator_WelcomeFrame", UIParent)
                Baganator_WelcomeFrame:Show()
            end)
            "#,
        )
        .expect("startup timer window setup should succeed");

    App::run_startup_sequence(&env);

    let shown: bool = env
        .borrow()
        .eval("return Baganator_WelcomeFrame and Baganator_WelcomeFrame:IsShown() or false")
        .expect("startup timer window visibility should be readable");
    assert!(
        !shown,
        "startup cleanup should also close windows created by startup timers"
    );
}

#[test]
fn parse_fast_tick_ms_accepts_positive_integers() {
    assert_eq!(parse_fast_tick_ms("1"), Some(1));
    assert_eq!(parse_fast_tick_ms(" 8 "), Some(8));
}

#[test]
fn parse_fast_tick_ms_rejects_zero_and_invalid_values() {
    assert_eq!(parse_fast_tick_ms("0"), None);
    assert_eq!(parse_fast_tick_ms("abc"), None);
    assert_eq!(parse_fast_tick_ms(""), None);
}
