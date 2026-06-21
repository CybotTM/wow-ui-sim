use super::*;
use crate::iced_app::app::AppInit;
use crate::iced_app::hit_grid::HitGrid;
use crate::iced_app::{build_hittable_rects, frame_collect::collect_hittable_frames};
use crate::lua_api::WowLuaEnv;
use crate::render::{GlyphAtlas, WowFontSystem};
use crate::screen::ScreenKind;
use crate::texture::TextureManager;
use iced::Size;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

pub(super) fn build_test_app(screen_kind: ScreenKind) -> App {
    let env = Rc::new(RefCell::new(
        WowLuaEnv::new().expect("Failed to create Lua environment"),
    ));
    env.borrow().set_screen_mode(screen_kind);
    env.borrow().set_screen_size(800.0, 600.0);

    let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
    let font_system = Rc::new(RefCell::new(WowFontSystem::new()));
    let glyph_atlas = Rc::new(RefCell::new(GlyphAtlas::new()));
    let (_cmd_tx, cmd_rx) = mpsc::channel(1);
    let (_lua_tx, lua_rx) = std::sync::mpsc::channel();

    let app = App::build_app(AppInit {
        env: Rc::clone(&env),
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
    app.screen_size.set(Size::new(800.0, 600.0));
    app
}

pub(super) fn rebuild_hittable_cache(app: &App) {
    let env = app.env.borrow();
    let mut state = env.state().borrow_mut();
    state.ensure_layout_rects();
    let strata_buckets = state
        .get_strata_buckets()
        .expect("visible strata buckets should exist")
        .clone();
    let collected = collect_hittable_frames(&state.widgets, &strata_buckets);
    let hittable = build_hittable_rects(&collected, &state.widgets);
    let grid = HitGrid::new(hittable, 800.0, 600.0);
    *app.cached_hittable.borrow_mut() = Some(grid);
}

const PASS_THROUGH_SETUP_LUA: &str = r#"
    PassThroughParent = CreateFrame("Button", "PassThroughParent", UIParent)
    PassThroughParent:SetSize(100, 100)
    PassThroughParent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
    PassThroughParent:EnableMouse(true)
    PassThroughParent:RegisterForClicks("LeftButtonUp", "RightButtonUp")
    PassThroughParent:SetScript("OnClick", function(_, button)
        if button == "LeftButton" then
            __pass_parent_left = (__pass_parent_left or 0) + 1
        elseif button == "RightButton" then
            __pass_parent_right = (__pass_parent_right or 0) + 1
        end
    end)

    PassThroughChild = CreateFrame("Button", "PassThroughChild", PassThroughParent)
    PassThroughChild:SetAllPoints(PassThroughParent)
    PassThroughChild:EnableMouse(true)
    PassThroughChild:RegisterForClicks("LeftButtonUp", "RightButtonUp")
    PassThroughChild:SetScript("OnClick", function(_, button)
        if button == "LeftButton" then
            __pass_child_left = (__pass_child_left or 0) + 1
        elseif button == "RightButton" then
            __pass_child_right = (__pass_child_right or 0) + 1
        end
    end)

    PassThroughChild:SetPassThroughButtons("RightButton")

    __pass_parent_left = 0
    __pass_parent_right = 0
    __pass_child_left = 0
    __pass_child_right = 0
"#;

pub(super) fn setup_pass_through_test_frames(app: &App) {
    app.env
        .borrow()
        .exec(PASS_THROUGH_SETUP_LUA)
        .expect("pass-through frame setup should succeed");
}

pub(super) fn read_pass_through_counters(app: &App) -> (f64, f64, f64, f64) {
    app.env
        .borrow()
        .eval(
            "return __pass_parent_left, __pass_parent_right, __pass_child_left, __pass_child_right",
        )
        .expect("pass-through counters should be readable")
}

pub(super) fn clear_pass_through_buttons(app: &App) {
    let env = app.env.borrow();
    env.exec(
        r#"
        PassThroughChild:SetPassThroughButtons()
        __pass_parent_right = 0
        __pass_child_right = 0
        "#,
    )
    .expect("clearing pass-through buttons should succeed");
}
