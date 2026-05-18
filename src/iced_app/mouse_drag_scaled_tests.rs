use super::test_support::*;
use super::*;
use crate::screen::ScreenKind;

#[test]
fn scaled_moving_drag_converts_screen_delta_to_anchor_offset() {
    let mut app = build_test_app(ScreenKind::Game);
    let env = app.env.borrow();
    env.exec(
        r#"
        ScaledMovingDragFrame = CreateFrame("Frame", "ScaledMovingDragFrame", UIParent)
        ScaledMovingDragFrame:SetSize(100, 100)
        ScaledMovingDragFrame:SetScale(0.5)
        ScaledMovingDragFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
        ScaledMovingDragFrame:SetMovable(true)
        ScaledMovingDragFrame:EnableMouse(true)
        ScaledMovingDragFrame:RegisterForDrag("LeftButton")
        ScaledMovingDragFrame:SetScript("OnDragStart", function(self) self:StartMoving() end)
        "#,
    )
    .expect("scaled moving drag frame setup should succeed");
    drop(env);

    rebuild_hittable_cache(&app);
    app.handle_mouse_move(Point::new(75.0, 75.0));
    app.handle_mouse_down(Point::new(75.0, 75.0));
    app.handle_mouse_move(Point::new(95.0, 95.0));

    let (x, y): (f64, f64) = app
        .env
        .borrow()
        .eval("local _, _, _, x, y = ScaledMovingDragFrame:GetPoint(1); return x, y")
        .expect("scaled moving drag anchor query should succeed");
    assert_eq!(x, 140.0, "20 screen pixels should become 40 UI units");
    assert_eq!(y, -140.0, "20 screen pixels should become 40 UI units");
}
