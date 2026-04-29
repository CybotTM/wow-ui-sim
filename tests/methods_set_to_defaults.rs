//! Regression tests for `Frame:SetToDefaults()`.
//!
//! The menu system reuses pooled frames and measures their extents after reset.
//! If `SetToDefaults()` leaves the old size behind, later menus inherit stale
//! widths from prior uses.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn test_set_to_defaults_clears_size_and_anchors() {
    let env = env();
    let (width, height, points): (f64, f64, i32) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "SetToDefaultsRegression", UIParent)
            frame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 10, -20)
            frame:SetSize(123, 45)
            frame:SetToDefaults()
            local w, h = frame:GetSize()
            return w, h, frame:GetNumPoints()
        "#,
        )
        .unwrap();

    assert_eq!(width, 0.0);
    assert_eq!(height, 0.0);
    assert_eq!(points, 0);
}
