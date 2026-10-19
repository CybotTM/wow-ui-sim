//! Staged, uncompiled. Requires the staged mouse-sequence adapter export.
#![cfg(all(feature = "gui", any(feature = "client-retail", feature = "client-wrath")))]

use iced::Point;
use wow_ui_sim::iced_app::{MouseSequenceStep as Step, run_headless_mouse_sequence};
use wow_ui_sim::lua_api::WowLuaEnv;

#[cfg(feature = "client-retail")]
fn load_damage_meter_consumer() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("SETUP: env");
    let path = wow_ui_sim::client_profile::blizzard_ui_addons_dir().join("Blizzard_DamageMeter/DamageMeterSourceWindow.lua");
    let source = std::fs::read_to_string(&path).expect("SETUP: original DamageMeter source");
    env.exec_named(&source, &format!("@{}", path.display())).expect("SETUP: whole original source");
    env
}

#[cfg(feature = "client-retail")]
fn damage_meter_fixture(right: bool, scale: f32) -> String {
    format!(r#"
        F2Source = CreateFrame("Frame", "F2Source", UIParent)
        Mixin(F2Source, DamageMeterSourceWindowMixin)
        F2Source:SetResizable(true)
        F2Source:SetScale({scale})
        F2Source:SetSize(200, 120)
        F2Source:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200 / {scale}, -200 / {scale})
        F2Source:SetResizeBounds(100, 80, 300, 200)
        F2Source.isRightSide = {right}
        local handle = CreateFrame("Button", "F2Handle", F2Source)
        F2Source.ResizeButton = handle
        handle:SetSize(12, 12)
        handle:SetPoint("BOTTOM{side}")
        handle:EnableMouse(true)
        handle:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
        handle:SetHighlightTexture("Interface/Buttons/ButtonHilight-Square")
        -- Original method installs original mouse-down/up/enter closures.
        -- In particular, DO NOT add RegisterForDrag or OnDragStart here.
        F2Source:InitializeResizeButton()
        F2Scale = {scale}
        F2Right = {right}
    "#, side = if right { "RIGHT" } else { "LEFT" })
}

#[cfg(feature = "client-retail")]
fn check_sizing_started(env: &WowLuaEnv) {
    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("F2Source").unwrap();
    assert!(state.widgets.get(id).unwrap().is_sizing, "original OnMouseDown starts sizing");
    drop(state);
    assert!(env.eval::<bool>("return F2Source:IsResizing()").unwrap());
}

#[cfg(feature = "client-retail")]
fn check_resized_geometry(env: &WowLuaEnv) {
    let (scale, right, width, height): (f32, bool, f32, f32) = env
        .eval("return F2Scale, F2Right, F2ExpectedWidth, F2ExpectedHeight").unwrap();
    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("F2Source").unwrap();
    let frame = state.widgets.get(id).unwrap();
    let rect = frame.layout_rect.expect("resized layout");
    let expected_x = if right { 200.0 } else { 200.0 + 200.0 * scale - width };
    for (actual, expected) in [(rect.x, expected_x), (rect.y, 200.0), (rect.width, width), (rect.height, height)] {
        assert!((actual - expected).abs() < 0.05, "F2 geometry/opposite corner: actual={actual}, expected={expected}");
    }
    // Stored dimensions must remain logical UI units, not screen pixels.
    assert!((frame.width - width / scale).abs() < 0.05);
    assert!((frame.height - height / scale).abs() < 0.05);
}

#[cfg(feature = "client-retail")]
fn check_sizing_stopped(env: &WowLuaEnv) {
    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("F2Source").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(!frame.is_sizing && !frame.is_moving, "F2 release clears native motion flags");
    assert!(state.active_drag_frame.is_none(), "release clears active drag capture");
    drop(state);
    let (resizing, placed): (bool, bool) = env
        .eval("return F2Source:IsResizing(), F2Source:IsUserPlaced()").unwrap();
    assert!(!resizing, "original mouse-up clears isResizing");
    assert!(!placed, "original DamageMeter mouse-up clears userPlaced");
}

#[cfg(feature = "client-retail")]
fn resize_case(right: bool, scale: f32, delta: (f32, f32), logical_size: (f32, f32)) {
    let start_x = if right { 200.0 + 194.0 * scale } else { 200.0 + 6.0 * scale };
    let start = Point::new(start_x, 200.0 + 114.0 * scale);
    let end = Point::new(start.x + delta.0, start.y + delta.1);
    let expected = format!("F2ExpectedWidth = {}; F2ExpectedHeight = {}", logical_size.0 * scale, logical_size.1 * scale);
    run_headless_mouse_sequence(load_damage_meter_consumer(), &damage_meter_fixture(right, scale), &[
        Step::Move(start), Step::Down(start), Step::Check(check_sizing_started),
        Step::Move(end), Step::Lua(&expected), Step::Check(check_resized_geometry),
        Step::Up(end), Step::Check(check_resized_geometry), Step::Check(check_sizing_stopped),
    ]);
}

#[test]
#[cfg(feature = "client-retail")]
fn f2_damage_meter_right_resize_preserves_top_left() {
    resize_case(true, 1.0, (20.0, 12.0), (220.0, 132.0));
}

#[test]
#[cfg(feature = "client-retail")]
fn f2_damage_meter_left_resize_preserves_top_right() {
    resize_case(false, 1.0, (-20.0, 12.0), (220.0, 132.0));
}

#[test]
#[cfg(feature = "client-retail")]
fn f2_damage_meter_resize_respects_maximum() {
    resize_case(true, 1.0, (160.0, 120.0), (300.0, 200.0));
}

#[test]
#[cfg(feature = "client-retail")]
fn f2_damage_meter_resize_respects_minimum() {
    resize_case(true, 1.0, (-160.0, -100.0), (100.0, 80.0));
}


#[test]
#[cfg(feature = "client-retail")]
fn f2_damage_meter_stationary_release_control() {
    let start = Point::new(394.0, 314.0);
    run_headless_mouse_sequence(load_damage_meter_consumer(), &damage_meter_fixture(true, 1.0), &[
        Step::Move(start), Step::Down(start), Step::Check(check_sizing_started),
        Step::Up(start), Step::Check(check_sizing_stopped),
    ]);
}

#[test]
#[cfg(feature = "client-retail")]
fn f2_damage_meter_mouse_release_after_motion() {
    let start = Point::new(394.0, 314.0);
    let end = Point::new(414.0, 326.0);
    run_headless_mouse_sequence(load_damage_meter_consumer(), &damage_meter_fixture(true, 1.0), &[
        Step::Move(start), Step::Down(start), Step::Check(check_sizing_started),
        Step::Move(end), Step::Up(end), Step::Check(check_sizing_stopped),
    ]);
}


#[cfg(feature = "client-wrath")]
fn check_chat_moved(env: &WowLuaEnv) {
    let (left, top, before_left, before_top): (f64, f64, f64, f64) = env.eval(
        "return ChatFrame1:GetLeft(), ChatFrame1:GetTop(), F2BeforeLeft, F2BeforeTop"
    ).unwrap();
    assert!((left - before_left - 20.0).abs() < 0.05, "original chat gesture horizontal delta");
    assert!((top - before_top + 20.0).abs() < 0.05, "original chat gesture vertical delta");
    let (width, height): (f64, f64) = env.eval("return ChatFrame1:GetWidth(), ChatFrame1:GetHeight()").unwrap();
    assert_eq!((width, height), (430.0, 120.0), "moving must not resize chat");
}

#[cfg(feature = "client-wrath")]
fn check_chat_stopped(env: &WowLuaEnv) {
    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("ChatFrame1").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(!frame.is_moving && !frame.is_sizing);
    assert!(state.active_drag_frame.is_none());
    drop(state);
    env.exec("assert(MOVING_CHATFRAME == nil); assert(ChatFrame1Tab.dragButton == nil)").unwrap();
}

#[test]
#[cfg(feature = "client-wrath")]
fn f2_original_wrath_chat_tab_moves_and_stops_on_original_update() {
    let env = WowLuaEnv::new().expect("SETUP: env");
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().expect("SETUP: Wrath cache");
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).expect("SETUP: original full FrameXML/XML scripts");
    // Never substitute FCF_Set_NormalChat: its StartMoving/Stop pair is not a gesture.
    // FloatingChatFrame.xml installs OnDragStart; FCFTab_OnUpdate handles release.
    let setup = r#"
        assert(ChatFrame1 and ChatFrame1Tab and GENERAL_CHAT_DOCK, "SETUP: original chat objects")
        ChatFrame1:ClearAllPoints()
        ChatFrame1:SetSize(430, 120)
        ChatFrame1:SetScale(1)
        ChatFrame1:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200, -200)
        ChatFrame1:SetMovable(true)
        ChatFrame1.isLocked = false
        ChatFrame1Tab:ClearAllPoints()
        ChatFrame1Tab:SetSize(100, 20)
        ChatFrame1Tab:SetPoint("TOPLEFT", ChatFrame1, "TOPLEFT", 0, 0)
        ChatFrame1Tab:Show()
        ChatFrame1:Show()
        F2BeforeLeft, F2BeforeTop = ChatFrame1:GetLeft(), ChatFrame1:GetTop()
    "#;
    run_headless_mouse_sequence(env, setup, &[
        Step::Move(Point::new(220.0, 210.0)), Step::Down(Point::new(220.0, 210.0)),
        Step::Move(Point::new(240.0, 230.0)), Step::Check(check_chat_moved),
        Step::Up(Point::new(240.0, 230.0)), Step::Tick(0.016), Step::Check(check_chat_stopped),
    ]);
}
