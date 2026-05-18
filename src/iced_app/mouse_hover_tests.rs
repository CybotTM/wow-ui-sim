use super::test_support::*;
use super::*;
use crate::screen::ScreenKind;
use iced::Size;

#[test]
fn hit_test_reflects_frames_shown_by_previous_clicks() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            RevealParent = CreateFrame("Button", "RevealParent", UIParent)
            RevealParent:SetSize(100, 100)
            RevealParent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            RevealParent:EnableMouse(true)
            RevealParent:SetScript("OnClick", function()
                RevealTarget:Show()
            end)

            RevealTarget = CreateFrame("Button", "RevealTarget", UIParent)
            RevealTarget:SetSize(100, 100)
            RevealTarget:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 300, -100)
            RevealTarget:EnableMouse(true)
            RevealTarget:Hide()
            RevealTarget:SetScript("OnClick", function()
                __reveal_target_clicks = (__reveal_target_clicks or 0) + 1
            end)

            __reveal_target_clicks = 0
            "#,
        )
        .expect("reveal test setup should succeed");
    }

    rebuild_hittable_cache(&app);

    let reveal_pos = Point::new(150.0, 150.0);
    app.handle_mouse_move(reveal_pos);
    app.handle_mouse_down(reveal_pos);
    app.handle_mouse_up(reveal_pos);

    let target_center = {
        let env = app.env.borrow();
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        let target_id = state
            .widgets
            .iter_ids()
            .find(|&id| {
                state.widgets.get(id).and_then(|f| f.name.as_deref()) == Some("RevealTarget")
            })
            .expect("target frame should exist");
        let rect = state
            .widgets
            .get(target_id)
            .and_then(|f| f.layout_rect)
            .expect("target frame should have a layout rect after being shown");
        Point::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0)
    };

    let hit = app.hit_test(target_center);
    assert_eq!(
        hit,
        Some(
            app.env
                .borrow()
                .state()
                .borrow()
                .widgets
                .iter_ids()
                .find(|&id| {
                    app.env
                        .borrow()
                        .state()
                        .borrow()
                        .widgets
                        .get(id)
                        .and_then(|f| f.name.as_deref())
                        == Some("RevealTarget")
                })
                .expect("target frame should exist")
        ),
        "hit-test should see frames revealed by earlier UI mutations"
    );

    app.handle_mouse_down(target_center);
    app.handle_mouse_up(target_center);

    let clicks: f64 = app
        .env
        .borrow()
        .eval("return __reveal_target_clicks")
        .expect("click count should be readable");
    assert_eq!(
        clicks, 1.0,
        "newly revealed frame should be clickable immediately after the revealing click"
    );
}

#[test]
fn mouse_edge_scripts_imply_mouse_enabled_and_update_hit_grid() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            ImpliedMouseFrame = CreateFrame("Frame", "ImpliedMouseFrame", UIParent)
            ImpliedMouseFrame:SetSize(100, 100)
            ImpliedMouseFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            __implied_mouse_down = 0
            "#,
        )
        .expect("implied mouse setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_down(Point::new(150.0, 150.0));

    let before_script: f64 = app
        .env
        .borrow()
        .eval("return __implied_mouse_down")
        .expect("initial mouse counter should be readable");
    assert_eq!(before_script, 0.0);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            ImpliedMouseFrame:SetScript("OnMouseDown", function(_, button)
                if button == "LeftButton" then
                    __implied_mouse_down = __implied_mouse_down + 1
                end
            end)
            "#,
        )
        .expect("setting OnMouseDown should succeed");
    }

    app.handle_mouse_down(Point::new(150.0, 150.0));

    let (mouse_enabled, after_script): (bool, f64) = app
        .env
        .borrow()
        .eval("return ImpliedMouseFrame:IsMouseEnabled(), __implied_mouse_down")
        .expect("implied mouse state should be readable");
    assert!(
        mouse_enabled,
        "setting OnMouseDown should imply EnableMouse(true)"
    );
    assert_eq!(
        after_script, 1.0,
        "hit grid should accept the frame without a full rebuild after SetScript"
    );
}

#[test]
fn disabled_buttons_only_run_hover_motion_scripts_when_opted_in() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            DisabledMotionButton = CreateFrame("Button", "DisabledMotionButton", UIParent)
            DisabledMotionButton:SetSize(100, 100)
            DisabledMotionButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            DisabledMotionButton:EnableMouse(true)
            DisabledMotionButton:Disable()
            DisabledMotionButton:SetScript("OnEnter", function()
                __disabled_motion_enter = (__disabled_motion_enter or 0) + 1
            end)
            DisabledMotionButton:SetScript("OnLeave", function()
                __disabled_motion_leave = (__disabled_motion_leave or 0) + 1
            end)
            __disabled_motion_enter = 0
            __disabled_motion_leave = 0
            "#,
        )
        .expect("disabled motion test button setup should succeed");
    }

    rebuild_hittable_cache(&app);
    let outside_pos = Point::new(20.0, 20.0);
    let inside_pos = Point::new(150.0, 150.0);

    app.handle_mouse_move(outside_pos);
    app.handle_mouse_move(inside_pos);
    app.handle_mouse_move(outside_pos);

    let (blocked_enter, blocked_leave, default_flag): (f64, f64, bool) = app
        .env
        .borrow()
        .eval(
            "return __disabled_motion_enter, __disabled_motion_leave, DisabledMotionButton:GetMotionScriptsWhileDisabled()",
        )
        .expect("default disabled motion state should be readable");
    assert_eq!(
        blocked_enter, 0.0,
        "disabled buttons should not receive OnEnter by default"
    );
    assert_eq!(
        blocked_leave, 0.0,
        "disabled buttons should not receive OnLeave by default"
    );
    assert!(
        !default_flag,
        "disabled buttons should default motion scripts while disabled to false"
    );

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            DisabledMotionButton:SetMotionScriptsWhileDisabled(true)
            __disabled_motion_enter = 0
            __disabled_motion_leave = 0
            "#,
        )
        .expect("enabling motion scripts while disabled should succeed");
    }

    app.handle_mouse_move(outside_pos);
    app.handle_mouse_move(inside_pos);
    app.handle_mouse_move(outside_pos);

    let (allowed_enter, allowed_leave, enabled_flag): (f64, f64, bool) = app
        .env
        .borrow()
        .eval(
            "return __disabled_motion_enter, __disabled_motion_leave, DisabledMotionButton:GetMotionScriptsWhileDisabled()",
        )
        .expect("enabled disabled-motion state should be readable");
    assert_eq!(
        allowed_enter, 1.0,
        "disabled buttons should receive OnEnter after opting into motion scripts"
    );
    assert_eq!(
        allowed_leave, 1.0,
        "disabled buttons should receive OnLeave after opting into motion scripts"
    );
    assert!(
        enabled_flag,
        "GetMotionScriptsWhileDisabled should reflect the opt-in flag"
    );
}

#[test]
fn cursor_leaving_canvas_fires_on_leave_for_hovered_frame() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            CanvasLeaveButton = CreateFrame("Button", "CanvasLeaveButton", UIParent)
            CanvasLeaveButton:SetSize(100, 100)
            CanvasLeaveButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            CanvasLeaveButton:EnableMouse(true)
            CanvasLeaveButton:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
            CanvasLeaveButton:SetScript("OnEnter", function(self)
                self:GetNormalTexture():SetAlpha(0)
            end)
            CanvasLeaveButton:SetScript("OnLeave", function(self)
                self:GetNormalTexture():SetAlpha(1)
                __canvas_leave_count = (__canvas_leave_count or 0) + 1
            end)
            __canvas_leave_count = 0
            "#,
        )
        .expect("canvas leave test setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(150.0, 150.0));

    let alpha_after_enter: f64 = app
        .env
        .borrow()
        .eval("return CanvasLeaveButton:GetNormalTexture():GetAlpha()")
        .expect("normal texture alpha should be readable after enter");
    assert_eq!(alpha_after_enter, 0.0);

    app.handle_mouse_leave();

    let (alpha_after_leave, leave_count): (f64, f64) = app
        .env
        .borrow()
        .eval("return CanvasLeaveButton:GetNormalTexture():GetAlpha(), __canvas_leave_count")
        .expect("normal texture alpha should be readable after canvas leave");
    assert_eq!(
        alpha_after_leave, 1.0,
        "cursor leaving the canvas should restore hover-hidden normal textures"
    );
    assert_eq!(leave_count, 1.0, "OnLeave should fire exactly once");
    assert!(
        app.hovered_frame.is_none(),
        "canvas leave should clear the app hover target"
    );
}

#[test]
fn moving_inside_canvas_off_hovered_frame_fires_on_leave() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            CanvasMoveAwayButton = CreateFrame("Button", "CanvasMoveAwayButton", UIParent)
            CanvasMoveAwayButton:SetSize(100, 100)
            CanvasMoveAwayButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            CanvasMoveAwayButton:EnableMouse(true)
            CanvasMoveAwayButton:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
            CanvasMoveAwayButton:SetScript("OnEnter", function(self)
                self:GetNormalTexture():SetAlpha(0)
            end)
            CanvasMoveAwayButton:SetScript("OnLeave", function(self)
                self:GetNormalTexture():SetAlpha(1)
                __canvas_move_leave_count = (__canvas_move_leave_count or 0) + 1
            end)
            __canvas_move_leave_count = 0
            "#,
        )
        .expect("canvas move-away test setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(150.0, 150.0));
    app.handle_mouse_move(Point::new(20.0, 20.0));

    let (alpha_after_leave, leave_count): (f64, f64) = app
        .env
        .borrow()
        .eval(
            "return CanvasMoveAwayButton:GetNormalTexture():GetAlpha(), __canvas_move_leave_count",
        )
        .expect("normal texture alpha should be readable after moving away");
    assert_eq!(
        alpha_after_leave, 1.0,
        "moving off the button inside the canvas should restore normal texture alpha"
    );
    assert_eq!(leave_count, 1.0, "OnLeave should fire exactly once");
    assert!(
        app.hovered_frame.is_none(),
        "moving to empty canvas should clear the app hover target"
    );
}

#[test]
fn moving_inside_same_hovered_frame_does_not_dirty_render_state() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            SameHoverButton = CreateFrame("Button", "SameHoverButton", UIParent)
            SameHoverButton:SetSize(100, 100)
            SameHoverButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            SameHoverButton:EnableMouse(true)
            SameHoverButton:SetScript("OnEnter", function()
                __same_hover_enter = (__same_hover_enter or 0) + 1
            end)
            SameHoverButton:SetScript("OnLeave", function()
                __same_hover_leave = (__same_hover_leave or 0) + 1
            end)
            __same_hover_enter = 0
            __same_hover_leave = 0
            "#,
        )
        .expect("same-hover test setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(150.0, 150.0));
    drain_mouse_move_dirty(&app);

    app.handle_mouse_move(Point::new(160.0, 160.0));
    let (dirty_mask, dirty_ids) = drain_mouse_move_dirty(&app);
    let (enter_count, leave_count): (f64, f64) = app
        .env
        .borrow()
        .eval("return __same_hover_enter, __same_hover_leave")
        .expect("same-hover counters should be readable");

    assert_eq!(enter_count, 1.0, "OnEnter should only fire once");
    assert_eq!(
        leave_count, 0.0,
        "OnLeave should not fire inside same frame"
    );
    assert_eq!(dirty_mask, 0, "same-hover movement should not dirty strata");
    assert!(
        dirty_ids.is_some_and(|ids| ids.is_empty()),
        "same-hover movement should not dirty frame IDs"
    );
}

#[test]
fn character_slot_tooltip_hides_when_cursor_leaves_rendered_slot() {
    let mut app = build_test_app(ScreenKind::Game);
    app.screen_size.set(Size::new(1024.0, 768.0));
    {
        let env = app.env.borrow();
        env.set_screen_size(1024.0, 768.0);
        load_character_tooltip_ui(&env);
        open_character_panel(&env);
    }

    let (slot_id, slot_rect) = {
        let env = app.env.borrow();
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        let slot_id = state
            .widgets
            .get_id_by_name("CharacterFinger0Slot")
            .expect("CharacterFinger0Slot should exist after opening character panel");
        let slot_rect = state
            .widgets
            .get(slot_id)
            .and_then(|frame| frame.layout_rect)
            .expect("CharacterFinger0Slot should have a layout rect");
        (slot_id, slot_rect)
    };
    let scale = crate::render::texture::UI_SCALE;
    let slot_center = Point::new(
        (slot_rect.x + slot_rect.width / 2.0) * scale,
        (slot_rect.y + slot_rect.height / 2.0) * scale,
    );
    let outside_slot = Point::new(
        (slot_rect.x + slot_rect.width + 80.0) * scale,
        (slot_rect.y + slot_rect.height / 2.0) * scale,
    );

    rebuild_hittable_cache(&app);
    assert_eq!(
        app.hit_test(slot_center),
        Some(slot_id),
        "slot center should hit the rendered character slot"
    );

    app.handle_mouse_move(slot_center);
    let tooltip_visible_after_enter: bool = app
        .env
        .borrow()
        .eval("return GameTooltip:IsShown()")
        .expect("tooltip visibility should be readable after slot enter");
    assert!(
        tooltip_visible_after_enter,
        "moving over the character slot should show its tooltip"
    );

    app.handle_mouse_move(outside_slot);
    let tooltip_visible_after_leave: bool = app
        .env
        .borrow()
        .eval("return GameTooltip:IsShown()")
        .expect("tooltip visibility should be readable after slot leave");
    assert_ne!(
        app.hovered_frame,
        Some(slot_id),
        "moving beside the rendered slot should clear the slot hover target"
    );
    assert!(
        !tooltip_visible_after_leave,
        "moving beside the rendered slot should fire OnLeave and hide GameTooltip"
    );
}

fn drain_mouse_move_dirty(app: &App) -> (u16, Option<rustc_hash::FxHashSet<u64>>) {
    app.env
        .borrow()
        .state()
        .borrow()
        .widgets
        .take_render_dirty_with_ids()
}

fn load_character_tooltip_ui(env: &crate::lua_api::WowLuaEnv) {
    let ui = crate::paths::default_blizzard_ui_addons_path()
        .expect("Blizzard UI cache should be synced");
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];

    for (name, toc) in CHARACTER_TOOLTIP_TEST_ADDONS {
        let toc_path = ui.join(name).join(toc);
        if let Err(error) = crate::loader::load_addon(&env.loader_env(), &toc_path) {
            panic!("failed to load {name}: {error}");
        }
    }
    env.apply_post_load_workarounds();
    fire_character_tooltip_startup_events(env);
    crate::startup::process_pending_timers(env);
    crate::startup::run_extra_update_ticks(env, 3);
}

fn fire_character_tooltip_startup_events(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.fire_event("ADDON_LOADED");
    let _ = env.fire_event("VARIABLES_LOADED");
    let _ = env.fire_event("PLAYER_LOGIN");
    let _ = env.fire_event("PLAYER_ENTERING_WORLD");
    let _ = env.fire_event("UPDATE_BINDINGS");
    let _ = env.fire_event("DISPLAY_SIZE_CHANGED");
    let _ = env.fire_event("UI_SCALE_CHANGED");
}

fn open_character_panel(env: &crate::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        local btn = CharacterMicroButton
        assert(btn, "CharacterMicroButton should exist")
        local onclick = btn:GetScript("OnClick")
        assert(onclick, "CharacterMicroButton should have an OnClick handler")
        onclick(btn, "LeftButton", false)
        assert(CharacterFrame and CharacterFrame:IsShown(), "CharacterFrame should be shown")
        assert(CharacterFinger0Slot ~= nil, "CharacterFinger0Slot should exist")
        "#,
    )
    .expect("character panel should open");
}

const CHARACTER_TOOLTIP_TEST_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    (
        "Blizzard_SharedXMLGame",
        "Blizzard_SharedXMLGame_Mainline.toc",
    ),
    (
        "Blizzard_UIPanelTemplates",
        "Blizzard_UIPanelTemplates_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLBase",
        "Blizzard_FrameXMLBase_Mainline.toc",
    ),
    ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
    ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
    ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
    (
        "Blizzard_AccessibilityTemplates",
        "Blizzard_AccessibilityTemplates.toc",
    ),
    ("Blizzard_ObjectAPI", "Blizzard_ObjectAPI_Mainline.toc"),
    ("Blizzard_UIParent", "Blizzard_UIParent_Mainline.toc"),
    ("Blizzard_TextStatusBar", "Blizzard_TextStatusBar.toc"),
    ("Blizzard_MoneyFrame", "Blizzard_MoneyFrame_Mainline.toc"),
    ("Blizzard_POIButton", "Blizzard_POIButton.toc"),
    ("Blizzard_Flyout", "Blizzard_Flyout.toc"),
    ("Blizzard_StoreUI", "Blizzard_StoreUI_Mainline.toc"),
    ("Blizzard_MicroMenu", "Blizzard_MicroMenu_Mainline.toc"),
    ("Blizzard_EditMode", "Blizzard_EditMode.toc"),
    ("Blizzard_GameTooltip", "Blizzard_GameTooltip_Mainline.toc"),
    (
        "Blizzard_UIParentPanelManager",
        "Blizzard_UIParentPanelManager_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLUtil",
        "Blizzard_FrameXMLUtil_Mainline.toc",
    ),
    ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
    ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    (
        "Blizzard_UIPanels_Game",
        "Blizzard_UIPanels_Game_Mainline.toc",
    ),
    ("Blizzard_ActionBar", "Blizzard_ActionBar_Mainline.toc"),
];
