//! `SetSpacing` / `GetSpacing` round-trip.
//!
//! The value is stored on `Frame.text_line_spacing` (for FontString /
//! EditBox widgets) or the font-object `__spacing` slot (for
//! GameFontNormal-style Font tables). Rendering ignores it for now —
//! see `src/widget/frame.rs` `text_line_spacing` comment — so these
//! tests only pin the getter/setter contract.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

#[test]
fn font_string_spacing_defaults_to_zero() {
    let env = env();
    let spacing: f64 = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", nil, UIParent)
            local fs = frame:CreateFontString(nil, "ARTWORK", "GameFontNormal")
            return fs:GetSpacing()
            "#,
        )
        .unwrap();
    assert_eq!(spacing, 0.0);
}

#[test]
fn font_string_set_spacing_round_trips() {
    let env = env();
    let spacing: f64 = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", nil, UIParent)
            local fs = frame:CreateFontString(nil, "ARTWORK", "GameFontNormal")
            fs:SetSpacing(7.5)
            return fs:GetSpacing()
            "#,
        )
        .unwrap();
    assert!((spacing - 7.5).abs() < 1e-4, "got {spacing}");
}

#[test]
fn edit_box_set_spacing_round_trips() {
    let env = env();
    let spacing: f64 = env
        .eval(
            r#"
            local eb = CreateFrame("EditBox", nil, UIParent)
            eb:SetSpacing(3)
            return eb:GetSpacing()
            "#,
        )
        .unwrap();
    assert_eq!(spacing, 3.0);
}

#[test]
fn font_object_set_spacing_round_trips() {
    let env = env();
    let spacing: f64 = env
        .eval(
            r#"
            GameFontNormal:SetSpacing(4.25)
            return GameFontNormal:GetSpacing()
            "#,
        )
        .unwrap();
    assert!((spacing - 4.25).abs() < 1e-4, "got {spacing}");
}

#[test]
fn font_string_and_font_object_are_independent() {
    let env = env();
    let (fs_val, font_val): (f64, f64) = env
        .eval(
            r#"
            -- Use a fresh font so prior tests in this file don't leak state.
            local MyFont = CreateFont("SpacingIndependenceFont")
            MyFont:SetSpacing(11)
            local frame = CreateFrame("Frame", nil, UIParent)
            local fs = frame:CreateFontString(nil, "ARTWORK", "SpacingIndependenceFont")
            fs:SetSpacing(2)
            return fs:GetSpacing(), MyFont:GetSpacing()
            "#,
        )
        .unwrap();
    assert_eq!(fs_val, 2.0, "FontString value");
    assert_eq!(font_val, 11.0, "Font object value");
}

#[test]
fn spacing_accepts_zero_and_negative_values() {
    let env = env();
    let (zero, neg): (f64, f64) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", nil, UIParent)
            local fs = frame:CreateFontString(nil, "ARTWORK", "GameFontNormal")
            fs:SetSpacing(0)
            local z = fs:GetSpacing()
            fs:SetSpacing(-2.5)
            local n = fs:GetSpacing()
            return z, n
            "#,
        )
        .unwrap();
    assert_eq!(zero, 0.0);
    assert!((neg - (-2.5)).abs() < 1e-4, "got {neg}");
}
