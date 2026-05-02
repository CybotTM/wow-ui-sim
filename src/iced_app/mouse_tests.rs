use super::*;
use crate::iced_app::{build_hittable_rects, frame_collect::collect_hittable_frames};
use crate::lua_api::WowLuaEnv;
use crate::render::{GlyphAtlas, WowFontSystem};
use crate::screen::ScreenKind;
use crate::texture::TextureManager;
use iced::Size;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

fn build_test_app(screen_kind: ScreenKind) -> App {
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

    let app = App::build_app(
        Rc::clone(&env),
        Vec::new(),
        texture_manager,
        font_system,
        glyph_atlas,
        cmd_rx,
        lua_rx,
        false,
        false,
        None,
        crate::config::SimConfig::default(),
    );
    app.screen_size.set(Size::new(800.0, 600.0));
    app
}

fn rebuild_hittable_cache(app: &App) {
    let env = app.env.borrow();
    let mut state = env.state().borrow_mut();
    state.ensure_layout_rects();
    let strata_buckets = state
        .get_strata_buckets()
        .expect("visible strata buckets should exist")
        .clone();
    let collected = collect_hittable_frames(&state.widgets, &strata_buckets);
    let hittable = build_hittable_rects(&collected, &state.widgets);
    let grid = super::super::hit_grid::HitGrid::new(hittable, 800.0, 600.0);
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

fn setup_pass_through_test_frames(app: &App) {
    app.env
        .borrow()
        .exec(PASS_THROUGH_SETUP_LUA)
        .expect("pass-through frame setup should succeed");
}

fn read_pass_through_counters(app: &App) -> (f64, f64, f64, f64) {
    app.env
        .borrow()
        .eval(
            "return __pass_parent_left, __pass_parent_right, __pass_child_left, __pass_child_right",
        )
        .expect("pass-through counters should be readable")
}

fn clear_pass_through_buttons(app: &App) {
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

#[test]
fn mouse_wheel_dispatch_requires_frame_mouse_wheel_enabled() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            MouseWheelDispatchFrame = CreateFrame("Frame", "MouseWheelDispatchFrame", UIParent)
            MouseWheelDispatchFrame:SetSize(100, 100)
            MouseWheelDispatchFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            MouseWheelDispatchFrame:EnableMouse(true)
            MouseWheelDispatchFrame:SetScript("OnMouseWheel", function(_, delta)
                __wheel_delta = (__wheel_delta or 0) + delta
            end)
            __wheel_delta = 0
            "#,
        )
        .expect("test frame setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(150.0, 150.0));
    app.handle_scroll(0.0, -1.0);

    let not_delivered: f64 = app
        .env
        .borrow()
        .eval("return __wheel_delta")
        .expect("wheel delta query should succeed");
    assert_eq!(
        not_delivered, 0.0,
        "mouse wheel scripts should not fire while mouse wheel is disabled"
    );

    {
        let env = app.env.borrow();
        env.exec("MouseWheelDispatchFrame:EnableMouseWheel(true)")
            .expect("enabling mouse wheel should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(150.0, 150.0));
    app.handle_scroll(0.0, -1.0);

    let delivered: f64 = app
        .env
        .borrow()
        .eval("return __wheel_delta")
        .expect("wheel delta query should succeed");
    assert_eq!(
        delivered, -1.0,
        "mouse wheel scripts should fire once the frame enables mouse wheel"
    );
}

#[test]
fn drag_start_can_transfer_to_delegate_and_abort_before_mouse_up() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            DragSourceFrame = CreateFrame("Frame", "DragSourceFrame", UIParent)
            DragSourceFrame:SetSize(100, 100)
            DragSourceFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            DragSourceFrame:EnableMouse(true)
            DragSourceFrame:SetScript("OnDragStart", function(self)
                self:InterceptStartDrag(DragDelegateFrame)
                DragDelegateFrame:AbortDrag()
            end)

            DragDelegateFrame = CreateFrame("Frame", "DragDelegateFrame", UIParent)
            DragDelegateFrame:SetScript("OnDragStop", function()
                __drag_stop_calls = (__drag_stop_calls or 0) + 1
            end)

            __drag_stop_calls = 0
            "#,
        )
        .expect("drag test frame setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(150.0, 150.0));
    app.handle_mouse_down(Point::new(150.0, 150.0));
    app.handle_mouse_move(Point::new(170.0, 170.0));

    let active_drag_frame = app.env.borrow().state().borrow().active_drag_frame;
    assert_eq!(
        active_drag_frame, None,
        "AbortDrag during OnDragStart should clear the active drag frame immediately"
    );

    let delegate_dragging: bool = app
        .env
        .borrow()
        .eval("return DragDelegateFrame:IsDragging()")
        .expect("delegate dragging query should succeed");
    assert!(
        !delegate_dragging,
        "delegate should not report dragging after AbortDrag clears the transfer"
    );

    app.handle_mouse_up(Point::new(170.0, 170.0));

    let drag_stop_calls: f64 = app
        .env
        .borrow()
        .eval("return __drag_stop_calls")
        .expect("drag stop count query should succeed");
    assert_eq!(
        drag_stop_calls, 0.0,
        "mouse up should not fire OnDragStop after AbortDrag already cleared the drag"
    );
}

#[test]
fn moving_drag_updates_frame_anchor_to_follow_mouse() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            MovingDragFrame = CreateFrame("Frame", "MovingDragFrame", UIParent)
            MovingDragFrame:SetSize(100, 100)
            MovingDragFrame:SetPoint("CENTER", UIParent, "TOPLEFT", 150, -150)
            MovingDragFrame:SetMovable(true)
            MovingDragFrame:EnableMouse(true)
            MovingDragFrame:RegisterForDrag("LeftButton")
            MovingDragFrame:SetScript("OnDragStart", function(self)
                self:StartMoving()
            end)
            "#,
        )
        .expect("moving drag frame setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(150.0, 150.0));
    app.handle_mouse_down(Point::new(150.0, 150.0));
    app.handle_mouse_move(Point::new(170.0, 170.0));

    let (num_points, point, relative_to, relative_point, x, y): (
        i64,
        String,
        String,
        String,
        f64,
        f64,
    ) = app
        .env
        .borrow()
        .eval(
            r#"
            local point, relativeTo, relativePoint, x, y = MovingDragFrame:GetPoint(1)
            local relativeName = relativeTo and relativeTo:GetName() or "nil"
            return MovingDragFrame:GetNumPoints(), point, relativeName, relativePoint, x, y
        "#,
        )
        .expect("moving drag anchor query should succeed");

    assert_eq!(
        num_points, 1,
        "moving drag should collapse anchors to one TOPLEFT point"
    );
    assert_eq!(
        point, "TOPLEFT",
        "moving drag should re-anchor the frame by TOPLEFT"
    );
    assert_eq!(
        relative_to, "UIParent",
        "moving drag should anchor relative to the parent"
    );
    assert_eq!(
        relative_point, "TOPLEFT",
        "moving drag should use the parent's TOPLEFT as its target"
    );
    assert_eq!(x, 120.0, "mouse delta should advance the frame x offset");
    assert_eq!(y, -120.0, "mouse delta should advance the frame y offset");
}

#[test]
fn moving_drag_clamps_frame_to_screen_when_enabled() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            ClampedMovingDragFrame = CreateFrame("Frame", "ClampedMovingDragFrame", UIParent)
            ClampedMovingDragFrame:SetSize(100, 100)
            ClampedMovingDragFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 10, -10)
            ClampedMovingDragFrame:SetMovable(true)
            ClampedMovingDragFrame:SetClampedToScreen(true)
            ClampedMovingDragFrame:EnableMouse(true)
            ClampedMovingDragFrame:RegisterForDrag("LeftButton")
            ClampedMovingDragFrame:SetScript("OnDragStart", function(self)
                self:StartMoving()
            end)
            "#,
        )
        .expect("clamped moving drag frame setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(20.0, 20.0));
    app.handle_mouse_down(Point::new(20.0, 20.0));
    app.handle_mouse_move(Point::new(-80.0, -80.0));

    let (point, relative_to, relative_point, x, y): (String, String, String, f64, f64) = app
        .env
        .borrow()
        .eval(
            r#"
            local point, relativeTo, relativePoint, x, y = ClampedMovingDragFrame:GetPoint(1)
            local relativeName = relativeTo and relativeTo:GetName() or "nil"
            return point, relativeName, relativePoint, x, y
        "#,
        )
        .expect("clamped moving drag anchor query should succeed");

    assert_eq!(point, "TOPLEFT");
    assert_eq!(relative_to, "UIParent");
    assert_eq!(relative_point, "TOPLEFT");
    assert_eq!(
        x, 0.0,
        "clamped moving drag should not move past the left edge"
    );
    assert_eq!(
        y, 0.0,
        "clamped moving drag should not move past the top edge"
    );
}

#[test]
fn stop_moving_or_sizing_marks_dragged_frame_user_placed() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            UserPlacedDragFrame = CreateFrame("Frame", "UserPlacedDragFrame", UIParent)
            UserPlacedDragFrame:SetSize(100, 100)
            UserPlacedDragFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 50, -50)
            UserPlacedDragFrame:SetMovable(true)
            UserPlacedDragFrame:EnableMouse(true)
            UserPlacedDragFrame:RegisterForDrag("LeftButton")
            UserPlacedDragFrame:SetScript("OnDragStart", function(self)
                self:StartMoving()
            end)
            UserPlacedDragFrame:SetScript("OnDragStop", function(self)
                self:StopMovingOrSizing()
            end)
            "#,
        )
        .expect("user placed drag frame setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(60.0, 60.0));
    app.handle_mouse_down(Point::new(60.0, 60.0));
    app.handle_mouse_move(Point::new(90.0, 90.0));
    app.handle_mouse_up(Point::new(90.0, 90.0));

    let user_placed: bool = app
        .env
        .borrow()
        .eval(
            r#"
            return UserPlacedDragFrame:IsUserPlaced()
        "#,
        )
        .expect("user placed drag state query should succeed");
    let is_moving = app
        .env
        .borrow()
        .state()
        .borrow()
        .widgets
        .get_by_name("UserPlacedDragFrame")
        .map(|frame| frame.is_moving)
        .expect("user placed drag frame should exist");

    assert!(
        !is_moving,
        "StopMovingOrSizing should clear the moving state after drag stop"
    );
    assert!(
        user_placed,
        "StopMovingOrSizing should mark a dragged frame as user placed"
    );
}

#[test]
fn drag_start_requires_registered_button_match() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            __mismatched_drag_start_calls = 0
            MismatchedDragButtonFrame = CreateFrame("Frame", "MismatchedDragButtonFrame", UIParent)
            MismatchedDragButtonFrame:SetSize(100, 100)
            MismatchedDragButtonFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 50, -50)
            MismatchedDragButtonFrame:EnableMouse(true)
            MismatchedDragButtonFrame:RegisterForDrag("RightButton")
            MismatchedDragButtonFrame:SetScript("OnDragStart", function()
                __mismatched_drag_start_calls = __mismatched_drag_start_calls + 1
            end)
            "#,
        )
        .expect("mismatched drag button frame setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(60.0, 60.0));
    app.handle_mouse_down(Point::new(60.0, 60.0));
    app.handle_mouse_move(Point::new(90.0, 90.0));

    let drag_start_calls: f64 = app
        .env
        .borrow()
        .eval("return __mismatched_drag_start_calls")
        .expect("drag start call count query should succeed");
    let active_drag_frame = app.env.borrow().state().borrow().active_drag_frame;

    assert_eq!(
        drag_start_calls, 0.0,
        "left-button drag should not fire OnDragStart when only RightButton is registered"
    );
    assert_eq!(
        active_drag_frame, None,
        "mismatched drag button should not activate a drag frame"
    );
}

#[test]
fn slider_reports_thumb_drag_state_while_mouse_is_held() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            SliderThumbDragFrame = CreateFrame("Slider", "SliderThumbDragFrame", UIParent)
            SliderThumbDragFrame:SetSize(120, 20)
            SliderThumbDragFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            SliderThumbDragFrame:EnableMouse(true)
            "#,
        )
        .expect("slider setup should succeed");
    }

    rebuild_hittable_cache(&app);
    let drag_pos = Point::new(150.0, 110.0);

    let before_mouse_down: bool = app
        .env
        .borrow()
        .eval("return SliderThumbDragFrame:IsDraggingThumb()")
        .expect("initial thumb drag query should succeed");
    assert!(
        !before_mouse_down,
        "fresh sliders should not report thumb dragging"
    );

    app.handle_mouse_move(drag_pos);
    app.handle_mouse_down(drag_pos);

    let during_mouse_down: bool = app
        .env
        .borrow()
        .eval("return SliderThumbDragFrame:IsDraggingThumb()")
        .expect("active thumb drag query should succeed");
    assert!(
        during_mouse_down,
        "slider should report thumb dragging while left mouse is held on it"
    );

    app.handle_mouse_up(drag_pos);

    let after_mouse_up: bool = app
        .env
        .borrow()
        .eval("return SliderThumbDragFrame:IsDraggingThumb()")
        .expect("post mouse up thumb drag query should succeed");
    assert!(
        !after_mouse_up,
        "slider thumb drag state should clear on mouse up"
    );
}

#[test]
fn pass_through_buttons_reroute_clicks_and_can_be_cleared() {
    let mut app = build_test_app(ScreenKind::Game);
    setup_pass_through_test_frames(&app);

    rebuild_hittable_cache(&app);
    let click_pos = Point::new(150.0, 150.0);

    app.handle_mouse_down(click_pos);
    app.handle_mouse_up(click_pos);
    app.handle_right_mouse_down(click_pos);
    app.handle_right_mouse_up(click_pos);

    let (parent_left, parent_right, child_left, child_right) = read_pass_through_counters(&app);
    assert_eq!(parent_left, 0.0, "left click should not pass through");
    assert_eq!(
        parent_right, 1.0,
        "right click should pass through to parent"
    );
    assert_eq!(
        child_left, 1.0,
        "child should still receive non-passthrough left clicks"
    );
    assert_eq!(
        child_right, 0.0,
        "child should not receive passthrough right clicks"
    );

    clear_pass_through_buttons(&app);
    rebuild_hittable_cache(&app);
    app.handle_right_mouse_down(click_pos);
    app.handle_right_mouse_up(click_pos);

    let (_, cleared_parent_right, _, cleared_child_right) = read_pass_through_counters(&app);
    assert_eq!(
        cleared_parent_right, 0.0,
        "clearing pass-through buttons should stop rerouting right clicks"
    );
    assert_eq!(
        cleared_child_right, 1.0,
        "child should receive right clicks again after passthrough is cleared"
    );
}

#[test]
fn hover_and_click_use_child_render_order_when_parent_wins_phase_one() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            HitOrderParent = CreateFrame("Button", "HitOrderParent", UIParent)
            HitOrderParent:SetSize(100, 100)
            HitOrderParent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            HitOrderParent:SetFrameLevel(10)
            HitOrderParent:EnableMouse(true)

            HitOrderHigh = CreateFrame("Button", "HitOrderHigh", HitOrderParent)
            HitOrderHigh:SetAllPoints(HitOrderParent)
            HitOrderHigh:SetFrameLevel(3)
            HitOrderHigh:EnableMouse(true)
            HitOrderHigh:SetScript("OnClick", function()
                __hit_order_high_clicks = (__hit_order_high_clicks or 0) + 1
            end)

            HitOrderLow = CreateFrame("Button", "HitOrderLow", HitOrderParent)
            HitOrderLow:SetAllPoints(HitOrderParent)
            HitOrderLow:SetFrameLevel(1)
            HitOrderLow:EnableMouse(true)
            HitOrderLow:SetScript("OnClick", function()
                __hit_order_low_clicks = (__hit_order_low_clicks or 0) + 1
            end)

            __hit_order_high_clicks = 0
            __hit_order_low_clicks = 0
            "#,
        )
        .expect("hit-order test setup should succeed");
    }

    rebuild_hittable_cache(&app);
    let hover_pos = Point::new(150.0, 150.0);
    app.handle_mouse_move(hover_pos);

    let hovered_name = {
        let env = app.env.borrow();
        let state = env.state().borrow();
        let hovered_id = state.hovered_frame.expect("a child should be hovered");
        state
            .widgets
            .get(hovered_id)
            .and_then(|frame| frame.name.clone())
            .expect("hovered frame should have a name")
    };
    assert_eq!(
        hovered_name, "HitOrderHigh",
        "hover should resolve to the highest-rendered child, not the last-created child"
    );

    app.handle_mouse_down(hover_pos);
    app.handle_mouse_up(hover_pos);

    let (high_clicks, low_clicks): (f64, f64) = app
        .env
        .borrow()
        .eval("return __hit_order_high_clicks, __hit_order_low_clicks")
        .expect("click counters should be readable");
    assert_eq!(
        high_clicks, 1.0,
        "click should go to the highest-rendered child when the parent wins phase one"
    );
    assert_eq!(
        low_clicks, 0.0,
        "lower-rendered sibling should not steal the click by creation order"
    );
}

#[test]
fn register_for_clicks_left_button_down_fires_click_on_mouse_down_only() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            DownClickButton = CreateFrame("Button", "DownClickButton", UIParent)
            DownClickButton:SetSize(100, 100)
            DownClickButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            DownClickButton:RegisterForClicks("LeftButtonDown")
            DownClickButton:SetScript("OnClick", function(_, button, down)
                __down_click_count = (__down_click_count or 0) + 1
                __down_click_button = button
                __down_click_down = down
            end)

            __down_click_count = 0
            __down_click_button = nil
            __down_click_down = nil
            "#,
        )
        .expect("down-click setup should succeed");
    }

    rebuild_hittable_cache(&app);
    let click_pos = Point::new(150.0, 150.0);

    app.handle_mouse_down(click_pos);

    let (count_after_down, button, down): (f64, String, bool) = app
        .env
        .borrow()
        .eval("return __down_click_count, __down_click_button, __down_click_down")
        .expect("down-click state should be readable after mouse down");
    assert_eq!(count_after_down, 1.0);
    assert_eq!(button, "LeftButton");
    assert!(down, "LeftButtonDown click should pass down=true");

    app.handle_mouse_up(click_pos);

    let count_after_up: f64 = app
        .env
        .borrow()
        .eval("return __down_click_count")
        .expect("down-click count should be readable after mouse up");
    assert_eq!(
        count_after_up, 1.0,
        "LeftButtonDown registration should not fire OnClick again on mouse up"
    );
}

#[test]
fn register_for_mouse_restricts_physical_mouse_button_events() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            MouseRegisteredButton = CreateFrame("Button", "MouseRegisteredButton", UIParent)
            MouseRegisteredButton:SetSize(100, 100)
            MouseRegisteredButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            MouseRegisteredButton:RegisterForMouse("LeftButtonDown", "LeftButtonUp")
            MouseRegisteredButton:SetScript("OnMouseDown", function(_, button)
                __mouse_registered_down = (__mouse_registered_down or "") .. button .. ";"
            end)
            MouseRegisteredButton:SetScript("OnMouseUp", function(_, button)
                __mouse_registered_up = (__mouse_registered_up or "") .. button .. ";"
            end)

            __mouse_registered_down = ""
            __mouse_registered_up = ""
            "#,
        )
        .expect("mouse registration setup should succeed");
    }

    rebuild_hittable_cache(&app);
    let click_pos = Point::new(150.0, 150.0);

    app.handle_right_mouse_down(click_pos);
    app.handle_right_mouse_up(click_pos);
    app.handle_mouse_down(click_pos);
    app.handle_mouse_up(click_pos);

    let (down_buttons, up_buttons): (String, String) = app
        .env
        .borrow()
        .eval("return __mouse_registered_down, __mouse_registered_up")
        .expect("mouse registration counters should be readable");
    assert_eq!(down_buttons, "LeftButton;");
    assert_eq!(up_buttons, "LeftButton;");
}

#[test]
fn propagated_mouse_clicks_fire_parent_mouse_handlers() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            PropagatingDropdownParent = CreateFrame("Button", "PropagatingDropdownParent", UIParent)
            PropagatingDropdownParent:SetSize(100, 100)
            PropagatingDropdownParent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            PropagatingDropdownParent:SetScript("OnMouseDown", function(_, button)
                __propagated_parent_down = (__propagated_parent_down or "") .. button .. ";"
            end)

            PropagatingDropdownChild = CreateFrame("Button", "PropagatingDropdownChild", PropagatingDropdownParent)
            PropagatingDropdownChild:SetAllPoints(PropagatingDropdownParent)
            PropagatingDropdownChild:SetPropagateMouseClicks(true)
            PropagatingDropdownChild:SetScript("OnMouseDown", function(_, button)
                __propagated_child_down = (__propagated_child_down or "") .. button .. ";"
            end)

            __propagated_parent_down = ""
            __propagated_child_down = ""
            "#,
        )
        .expect("propagating click setup should succeed");
    }

    rebuild_hittable_cache(&app);
    app.handle_mouse_down(Point::new(150.0, 150.0));

    let (parent_down, child_down): (String, String) = app
        .env
        .borrow()
        .eval("return __propagated_parent_down, __propagated_child_down")
        .expect("propagated click counters should be readable");
    assert_eq!(child_down, "LeftButton;");
    assert_eq!(parent_down, "LeftButton;");
}

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
