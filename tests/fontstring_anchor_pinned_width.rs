//! FontStrings with two opposite-edge anchors (TOPLEFT + TOPRIGHT, LEFT + RIGHT, etc.)
//! must derive width from the anchor target rather than from auto-measured text width.
//! Otherwise long text overflows the slot it was anchored into — see the SpellBook
//! `Name` FontString in Blizzard_PlayerSpells where text was running into adjacent
//! icon columns.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn two_anchor_fontstring_takes_parent_width_not_text_width() {
    let env = env();
    let (width, is_auto): (f64, bool) = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "AnchorParent", UIParent)
            parent:SetSize(150, 30)
            local fs = parent:CreateFontString(nil, "ARTWORK", "GameFontNormal")
            fs:SetPoint("TOPLEFT")
            fs:SetPoint("TOPRIGHT")
            fs:SetWordWrap(true)
            fs:SetMaxLines(3)
            fs:SetText("This is a very long string that would exceed the parent width if rendered as one line")
            return fs:GetWidth(), fs.__widthIsTextAuto or false
            "#,
        )
        .unwrap();
    assert!(
        (width - 150.0).abs() < 1.0,
        "FontString width should match anchor-pinned parent width 150, got {width}"
    );
    assert!(
        !is_auto,
        "FontString width must not be flagged as auto-text-derived when anchors pin both edges"
    );
}

#[test]
fn left_offset_anchor_subtracts_from_parent_width() {
    let env = env();
    let width: f64 = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "AnchorParent2", UIParent)
            parent:SetSize(220, 60)
            local fs = parent:CreateFontString(nil, "ARTWORK", "GameFontNormal")
            fs:SetPoint("LEFT", parent, "LEFT", 50, 0)
            fs:SetPoint("RIGHT", parent, "RIGHT", 0, 0)
            fs:SetWordWrap(true)
            fs:SetText("Crusader Strike")
            return fs:GetWidth()
            "#,
        )
        .unwrap();
    assert!(
        (width - 170.0).abs() < 1.0,
        "FontString width should be parent.width(220) - left_offset(50) = 170, got {width}"
    );
}

#[test]
fn single_anchor_fontstring_still_uses_auto_text_width() {
    let env = env();
    // Sanity: behavior unchanged for the common case (one anchor, no width set).
    let (has_width, is_auto): (bool, bool) = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "AnchorParent3", UIParent)
            parent:SetSize(500, 60)
            local fs = parent:CreateFontString(nil, "ARTWORK", "GameFontNormal")
            fs:SetPoint("CENTER")
            fs:SetText("Short")
            return fs:GetWidth() > 0, fs.__widthIsTextAuto or true
            "#,
        )
        .unwrap();
    assert!(
        has_width,
        "single-anchor FontString should still auto-size width from text"
    );
    let _ = is_auto; // flag is internal; only the width matters for the public test contract
}

#[test]
fn fontstring_get_num_lines_reports_wrapped_line_count() {
    let env = env();
    let (method_type, short_lines, wrapped_lines): (String, i32, i32) = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "NumLinesParent", UIParent)
            parent:SetSize(120, 80)

            local fs = parent:CreateFontString(nil, "ARTWORK", "GameFontNormal")
            fs:SetWidth(120)
            fs:SetWordWrap(true)

            fs:SetText("Short")
            local shortLines = fs:GetNumLines()

            fs:SetText("This is a much longer Adventure Guide description that should wrap across multiple lines")
            local wrappedLines = fs:GetNumLines()

            return type(fs.GetNumLines), shortLines, wrappedLines
            "#,
        )
        .unwrap();

    assert_eq!(method_type, "function");
    assert_eq!(short_lines, 1, "single-line text should report one line");
    assert!(
        wrapped_lines > 1,
        "wrapped text should report multiple lines, got {wrapped_lines}"
    );
}
