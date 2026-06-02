use super::test_support::*;
use super::*;
use crate::screen::ScreenKind;

#[test]
fn register_for_clicks_any_up_matches_addon_lowercase_spelling() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            LowercaseAnyUpButton = CreateFrame("Button", "LowercaseAnyUpButton", UIParent)
            LowercaseAnyUpButton:SetSize(100, 100)
            LowercaseAnyUpButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            LowercaseAnyUpButton:RegisterForClicks("anyUp")
            LowercaseAnyUpButton:SetScript("OnClick", function(_, button, down)
                __lowercase_any_up_count = (__lowercase_any_up_count or 0) + 1
                __lowercase_any_up_button = button
                __lowercase_any_up_down = down
            end)

            __lowercase_any_up_count = 0
            __lowercase_any_up_button = nil
            __lowercase_any_up_down = nil
            "#,
        )
        .expect("lowercase anyUp setup should succeed");
    }

    rebuild_hittable_cache(&app);
    let click_pos = Point::new(150.0, 150.0);
    app.handle_mouse_down(click_pos);
    app.handle_mouse_up(click_pos);

    let (count, button, down): (f64, String, bool) = app
        .env
        .borrow()
        .eval("return __lowercase_any_up_count, __lowercase_any_up_button, __lowercase_any_up_down")
        .expect("lowercase anyUp click state should be readable");
    assert_eq!(count, 1.0);
    assert_eq!(button, "LeftButton");
    assert!(!down, "anyUp click should pass down=false");
}

#[test]
fn left_button_up_fires_mouse_up_before_click() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            UpOrderButton = CreateFrame("Button", "UpOrderButton", UIParent)
            UpOrderButton:SetSize(100, 100)
            UpOrderButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            UpOrderButton:RegisterForClicks("LeftButtonUp")
            UpOrderButton:SetScript("OnMouseUp", function()
                __up_order = (__up_order or "") .. "up;"
            end)
            UpOrderButton:SetScript("OnClick", function()
                __up_order = (__up_order or "") .. "click;"
            end)
            __up_order = ""
            "#,
        )
        .expect("up-order setup should succeed");
    }

    rebuild_hittable_cache(&app);
    let click_pos = Point::new(150.0, 150.0);

    app.handle_mouse_down(click_pos);
    app.handle_mouse_up(click_pos);

    let order: String = app
        .env
        .borrow()
        .eval("return __up_order")
        .expect("up-order should be readable");
    assert_eq!(order, "up;click;");
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
fn register_for_mouse_matches_addon_lowercase_spelling() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            LowercaseMouseRegisteredButton = CreateFrame("Button", "LowercaseMouseRegisteredButton", UIParent)
            LowercaseMouseRegisteredButton:SetSize(100, 100)
            LowercaseMouseRegisteredButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            LowercaseMouseRegisteredButton:RegisterForMouse("leftbuttondown", "leftbuttonup")
            LowercaseMouseRegisteredButton:SetScript("OnMouseDown", function(_, button)
                __lowercase_mouse_down = (__lowercase_mouse_down or "") .. button .. ";"
            end)
            LowercaseMouseRegisteredButton:SetScript("OnMouseUp", function(_, button)
                __lowercase_mouse_up = (__lowercase_mouse_up or "") .. button .. ";"
            end)

            __lowercase_mouse_down = ""
            __lowercase_mouse_up = ""
            "#,
        )
        .expect("lowercase mouse registration setup should succeed");
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
        .eval("return __lowercase_mouse_down, __lowercase_mouse_up")
        .expect("lowercase mouse registration counters should be readable");
    assert_eq!(down_buttons, "LeftButton;");
    assert_eq!(up_buttons, "LeftButton;");
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
