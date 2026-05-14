//! Tests for methods_anchor.rs: SetPoint, ClearAllPoints, GetPoint, GetNumPoints,
//! SetAllPoints, AdjustPointsOffset, GetPointByName.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetPoint / GetPoint / GetNumPoints
// ============================================================================

#[test]
fn test_set_point_basic() {
    let env = env();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "AnchorFrame1", UIParent)
        f:SetPoint("CENTER", UIParent, "CENTER", 10, 20)
    "#,
    )
    .unwrap();

    let num: i32 = env.eval("return AnchorFrame1:GetNumPoints()").unwrap();
    assert_eq!(num, 1);
}

#[test]
fn test_set_point_reports_plumber_topelft_typo_compatibility() {
    let env = env();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "AnchorFrameTypo", UIParent)
        f:SetPoint("TOPELFT", UIParent, "TOPELFT", 0, 0)
        local point, _, relativePoint = f:GetPoint(1)
        assert(point == "TOPLEFT", "point should canonicalize to TOPLEFT")
        assert(relativePoint == "TOPLEFT", "relativePoint should canonicalize to TOPLEFT")
    "#,
    )
    .unwrap();

    let console_output = env.state().borrow().console_output.clone();
    assert!(
        console_output
            .iter()
            .any(|line| line.contains("Plumber typo compatibility: TOPELFT resolves as TOPLEFT.")),
        "TOPELFT compatibility should be reported in console output, got {console_output:?}"
    );
}

#[test]
fn test_live_addon_script_handlers_are_accepted() {
    let env = env();
    env.exec(
        r#"
        local editBox = CreateFrame("EditBox", "ScriptHandlerEditBox", UIParent)
        editBox:SetScript("OnCursorChanged", function() end)

        local tooltip = CreateFrame("GameTooltip", "ScriptHandlerTooltip", UIParent)
        tooltip:HookScript("OnTooltipSetDefaultAnchor", function() end)
        tooltip:SetScript("OnTooltipSetFrameStack", function() end)
        tooltip:SetScript("OnTooltipAddMoney", function() end)

        local model = CreateFrame("Model", "ScriptHandlerModel", UIParent)
        model:SetScript("OnModelLoaded", function() end)

        assert(editBox:HasScript("OnCursorChanged"), "EditBox should accept OnCursorChanged")
        assert(tooltip:HasScript("OnTooltipSetDefaultAnchor"), "GameTooltip should accept OnTooltipSetDefaultAnchor")
        assert(model:HasScript("OnModelLoaded"), "Model should accept OnModelLoaded")
    "#,
    )
    .unwrap();
}

#[test]
fn test_get_point_returns_values() {
    let env = env();
    // GetPoint returns (point, relativeTo, relativePoint, x, y) where relativeTo is a frame/nil
    // Do assertions in Lua and return offsets to Rust
    let (x, y): (f64, f64) = env.eval(r#"
        local f = CreateFrame("Frame", "AnchorFrame2", UIParent)
        f:SetPoint("TOPLEFT", UIParent, "BOTTOMRIGHT", 5, -10)
        local point, relTo, relPoint, x, y = f:GetPoint(1)
        assert(point == "TOPLEFT", "point should be TOPLEFT, got " .. tostring(point))
        assert(relPoint == "BOTTOMRIGHT", "relPoint should be BOTTOMRIGHT, got " .. tostring(relPoint))
        return x, y
    "#).unwrap();
    assert!((x - 5.0).abs() < 0.01);
    assert!((y - (-10.0)).abs() < 0.01);
}

#[test]
fn test_set_point_default_relative_point() {
    let env = env();
    env.exec(r#"
        local f = CreateFrame("Frame", "AnchorFrame3", UIParent)
        f:SetPoint("CENTER")
        local point, relTo, relPoint = f:GetPoint(1)
        assert(point == "CENTER", "point should be CENTER, got " .. tostring(point))
        assert(relPoint == "CENTER", "relativePoint should default to CENTER, got " .. tostring(relPoint))
    "#).unwrap();
}

#[test]
fn test_set_point_multiple() {
    let env = env();
    let num: i32 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorFrame4", UIParent)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 0, 0)
        f:SetPoint("BOTTOMRIGHT", UIParent, "BOTTOMRIGHT", 0, 0)
        return f:GetNumPoints()
    "#,
        )
        .unwrap();
    assert_eq!(num, 2);
}

#[test]
fn test_set_point_replaces_same_point() {
    let env = env();
    let x: f64 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorFrame5", UIParent)
        f:SetPoint("CENTER", UIParent, "CENTER", 10, 0)
        f:SetPoint("CENTER", UIParent, "CENTER", 99, 0)
        local _, _, _, xOfs = f:GetPoint(1)
        return xOfs
    "#,
        )
        .unwrap();
    assert!(
        (x - 99.0).abs() < 0.01,
        "Second SetPoint should replace the first"
    );
}

// ============================================================================
// ClearAllPoints
// ============================================================================

#[test]
fn test_clear_all_points() {
    let env = env();
    let num: i32 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorClear", UIParent)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 0, 0)
        f:SetPoint("BOTTOMRIGHT", UIParent, "BOTTOMRIGHT", 0, 0)
        f:ClearAllPoints()
        return f:GetNumPoints()
    "#,
        )
        .unwrap();
    assert_eq!(num, 0);
}

// ============================================================================
// SetAllPoints
// ============================================================================

#[test]
fn test_set_all_points() {
    let env = env();
    let num: i32 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorAll", UIParent)
        f:SetAllPoints(UIParent)
        return f:GetNumPoints()
    "#,
        )
        .unwrap();
    assert_eq!(num, 2, "SetAllPoints should add TOPLEFT and BOTTOMRIGHT");
}

#[test]
fn test_set_all_points_offsets_zero() {
    let env = env();
    let (x1, y1): (f64, f64) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorAllOff", UIParent)
        f:SetAllPoints(UIParent)
        local _, _, _, x, y = f:GetPoint(1)
        return x, y
    "#,
        )
        .unwrap();
    assert_eq!(x1, 0.0);
    assert_eq!(y1, 0.0);
}

// ============================================================================
// AdjustPointsOffset
// ============================================================================

#[test]
fn test_adjust_points_offset() {
    let env = env();
    let (x, y): (f64, f64) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorAdj", UIParent)
        f:SetPoint("CENTER", UIParent, "CENTER", 10, 20)
        f:AdjustPointsOffset(5, -3)
        local _, _, _, xOfs, yOfs = f:GetPoint(1)
        return xOfs, yOfs
    "#,
        )
        .unwrap();
    assert!((x - 15.0).abs() < 0.01, "x should be 10+5=15, got {}", x);
    assert!((y - 17.0).abs() < 0.01, "y should be 20+(-3)=17, got {}", y);
}

#[test]
fn test_adjust_points_offset_multiple_anchors() {
    let env = env();
    let (x2, y2): (f64, f64) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorAdjMulti", UIParent)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 0, 0)
        f:SetPoint("BOTTOMRIGHT", UIParent, "BOTTOMRIGHT", 0, 0)
        f:AdjustPointsOffset(10, 10)
        local _, _, _, x, y = f:GetPoint(2)
        return x, y
    "#,
        )
        .unwrap();
    assert!((x2 - 10.0).abs() < 0.01);
    assert!((y2 - 10.0).abs() < 0.01);
}

// ============================================================================
// GetPointByName
// ============================================================================

#[test]
fn test_get_point_by_name() {
    let env = env();
    let (x, y): (f64, f64) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorByName", UIParent)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 5, 10)
        f:SetPoint("BOTTOMRIGHT", UIParent, "BOTTOMRIGHT", -5, -10)
        local point, relTo, relPoint, xOfs, yOfs = f:GetPointByName("BOTTOMRIGHT")
        assert(point == "BOTTOMRIGHT", "point should be BOTTOMRIGHT")
        assert(relPoint == "BOTTOMRIGHT", "relPoint should be BOTTOMRIGHT")
        return xOfs, yOfs
    "#,
        )
        .unwrap();
    assert!((x - (-5.0)).abs() < 0.01);
    assert!((y - (-10.0)).abs() < 0.01);
}

#[test]
fn test_get_point_by_name_not_found() {
    let env = env();
    let is_nil: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorByNameNil", UIParent)
        f:SetPoint("CENTER")
        return f:GetPointByName("TOPLEFT") == nil
    "#,
        )
        .unwrap();
    assert!(
        is_nil,
        "GetPointByName should return nil for non-existent anchor"
    );
}

// ============================================================================
// Cycle detection
// ============================================================================

#[test]
fn test_set_point_self_reference_raises_error() {
    let env = env();
    let failed: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorSelf", UIParent)
        local ok = pcall(f.SetPoint, f, "CENTER", f, "CENTER", 0, 0)
        return not ok
    "#,
        )
        .unwrap();
    assert!(failed, "SetPoint to self should raise a Lua error");
}

// ============================================================================
// SetAllPoints with explicit parent — GetPoint returns the parent frame
// (frame was created with explicit parent arg, so SetAllPoints() anchors to it)
// ============================================================================

#[test]
fn test_set_all_points_explicit_parent_returns_parent() {
    let env = env();
    // Frame created with explicit parent arg — SetAllPoints() no-arg should
    // store the parent reference, GetPoint returns parent (not nil).
    let (r1_is_parent, r2_is_parent): (bool, bool) = env
        .eval(
            r#"
        local parent = CreateFrame("Frame", "SAPParent2")
        local f = CreateFrame("Frame", "SAPChild2", SAPParent2)
        f:SetAllPoints()
        local p1, r1 = f:GetPoint(1)
        local p2, r2 = f:GetPoint(2)
        return r1 == SAPParent2, r2 == SAPParent2
    "#,
        )
        .unwrap();
    assert!(
        r1_is_parent,
        "GetPoint(1) relativeTo should be parent after SetAllPoints() with explicit parent"
    );
    assert!(
        r2_is_parent,
        "GetPoint(2) relativeTo should be parent after SetAllPoints() with explicit parent"
    );
}

#[test]
fn test_set_all_points_explicit_nil_returns_nil() {
    let env = env();
    let (r1_nil, r2_nil): (bool, bool) = env
        .eval(
            r#"
        local parent = CreateFrame("Frame", "SAPNilParent")
        local f = CreateFrame("Frame", "SAPNilChild", SAPNilParent)
        f:SetParent(SAPNilParent)
        f:SetAllPoints(nil)
        local _, r1 = f:GetPoint(1)
        local _, r2 = f:GetPoint(2)
        return r1 == nil, r2 == nil
    "#,
        )
        .unwrap();
    assert!(
        r1_nil,
        "GetPoint(1) relativeTo should be nil after SetAllPoints(nil)"
    );
    assert!(
        r2_nil,
        "GetPoint(2) relativeTo should be nil after SetAllPoints(nil)"
    );
}

// ============================================================================
// SetAllPoints implicit parent — GetPoint returns parent for relativeTo
// ============================================================================

#[test]
fn test_set_all_points_implicit_parent_returns_parent() {
    let env = env();
    // Frame parented to UIParent, SetAllPoints() with no args
    // In WoW, nil relativeTo always means "parent", so GetPoint returns the parent frame.
    let (point, rel_name, rel_point, x, y): (String, String, String, f64, f64) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "SAP_TestFrame", UIParent)
        f:SetAllPoints()
        local p, rel, rp, x, y = f:GetPoint(1)
        return p, rel:GetName(), rp, x, y
    "#,
        )
        .unwrap();
    assert_eq!(point, "TOPLEFT");
    assert_eq!(
        rel_name, "UIParent",
        "relativeTo should be the parent frame"
    );
    assert_eq!(rel_point, "TOPLEFT");
    assert_eq!(x, 0.0);
    assert_eq!(y, 0.0);
}

// ============================================================================
// GetNumPoints default
// ============================================================================

#[test]
fn test_get_num_points_default_zero() {
    let env = env();
    let num: i32 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "AnchorNum", UIParent)
        return f:GetNumPoints()
    "#,
        )
        .unwrap();
    assert_eq!(num, 0);
}

// ============================================================================
// Cycle detection — SetPoint/SetAllPoints should raise Lua errors
// ============================================================================

#[test]
fn test_set_point_self_cycle_raises_error() {
    let env = env();
    let failed: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame")
        local ok, msg = pcall(f.SetPoint, f, "CENTER", f)
        return not ok
    "#,
        )
        .unwrap();
    assert!(failed, "SetPoint to self should raise a Lua error");
}

#[test]
fn test_set_point_indirect_cycle_raises_error() {
    let env = env();
    let failed: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame")
        local g = CreateFrame("Frame")
        g:SetPoint("CENTER", f)
        local ok, msg = pcall(f.SetPoint, f, "CENTER", g)
        return not ok
    "#,
        )
        .unwrap();
    assert!(
        failed,
        "SetPoint creating indirect cycle should raise a Lua error"
    );
}

#[test]
fn test_set_all_points_self_cycle_raises_error() {
    let env = env();
    let failed: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame")
        local ok, msg = pcall(f.SetAllPoints, f, f)
        return not ok
    "#,
        )
        .unwrap();
    assert!(failed, "SetAllPoints to self should raise a Lua error");
}

// ============================================================================
// Cycle error message format — matches wowless dag tests exactly
// ============================================================================

/// Helper: extract hex ID from a frame LightUserData, matching wowless rstr().
/// tostring(lud) gives "userdata: 0x<hex>", rstr extracts after "0x".
const RSTR_LUA: &str = r#"
local function rstr(r)
    return tostring(r):gsub('^.*0x(.*)$', '%1')
end
"#;

#[test]
fn test_cycle_error_dag0_self_anchor_message() {
    let env = env();
    let msg: String = env
        .eval(&format!(
            r#"
        {RSTR_LUA}
        local f = CreateFrame("Frame")
        local expected = table.concat({{
            'Action[SetPoint] failed because',
            '[Cannot anchor to itself]: ',
            'attempted from: Frame:SetPoint.',
        }})
        local ok, got = pcall(f.SetPoint, f, "CENTER", f)
        assert(not ok, "should fail")
        assert(got == expected, "msg mismatch:\nexpected: " .. expected .. "\ngot: " .. got)
        return got
    "#
        ))
        .unwrap();
    assert!(
        msg.starts_with("Action[SetPoint] failed because[Cannot anchor to itself]"),
        "msg: {msg}"
    );
}

#[test]
fn test_cycle_error_dag1_direct_cycle_message() {
    let env = env();
    let msg: String = env.eval(&format!(r#"
        {RSTR_LUA}
        local f = CreateFrame("Frame")
        local g = CreateFrame("Frame")
        local expected = table.concat({{
            'Action[SetPoint] failed because',
            '[Cannot anchor to a region dependent on it]: ',
            'attempted from: Frame:SetPoint.\n',
            'Relative: [' .. rstr(g) .. ']\n',
            'Dependent: [' .. rstr(g) .. ']',
        }})
        g:SetPoint("CENTER", f)
        local ok, got = pcall(f.SetPoint, f, "CENTER", g)
        assert(not ok, "should fail")
        assert(got == expected, "msg mismatch:\nexpected: " .. tostring(expected) .. "\ngot: " .. tostring(got))
        return got
    "#)).unwrap();
    assert!(
        msg.contains("Relative:") && msg.contains("Dependent:"),
        "msg: {msg}"
    );
    assert!(
        !msg.contains("Dependent ancestors:"),
        "dag1 should have no ancestors, msg: {msg}"
    );
}

#[test]
fn test_cycle_error_dag2_one_ancestor_message() {
    let env = env();
    let msg: String = env.eval(&format!(r#"
        {RSTR_LUA}
        local f = CreateFrame("Frame")
        local g = CreateFrame("Frame")
        local h = CreateFrame("Frame")
        local expected = table.concat({{
            'Action[SetPoint] failed because',
            '[Cannot anchor to a region dependent on it]: ',
            'attempted from: Frame:SetPoint.\n',
            'Relative: [' .. rstr(h) .. ']\n',
            'Dependent: [' .. rstr(g) .. ']\n',
            'Dependent ancestors:\n',
            '[' .. rstr(h) .. ']',
        }})
        g:SetPoint("CENTER", f)
        h:SetPoint("CENTER", g)
        local ok, got = pcall(f.SetPoint, f, "CENTER", h)
        assert(not ok, "should fail")
        assert(got == expected, "msg mismatch:\nexpected: " .. tostring(expected) .. "\ngot: " .. tostring(got))
        return got
    "#)).unwrap();
    assert!(
        msg.contains("Dependent ancestors:"),
        "dag2 should have ancestors, msg: {msg}"
    );
}

#[test]
fn test_cycle_error_dag3_two_ancestors_message() {
    let env = env();
    let msg: String = env.eval(&format!(r#"
        {RSTR_LUA}
        local f = CreateFrame("Frame")
        local g = CreateFrame("Frame")
        local h = CreateFrame("Frame")
        local i = CreateFrame("Frame")
        local expected = table.concat({{
            'Action[SetPoint] failed because',
            '[Cannot anchor to a region dependent on it]: ',
            'attempted from: Frame:SetPoint.\n',
            'Relative: [' .. rstr(i) .. ']\n',
            'Dependent: [' .. rstr(g) .. ']\n',
            'Dependent ancestors:\n',
            '[' .. rstr(h) .. ']\n',
            '[' .. rstr(i) .. ']',
        }})
        g:SetPoint("CENTER", f)
        h:SetPoint("CENTER", g)
        i:SetPoint("CENTER", h)
        local ok, got = pcall(f.SetPoint, f, "CENTER", i)
        assert(not ok, "should fail")
        assert(got == expected, "msg mismatch:\nexpected: " .. tostring(expected) .. "\ngot: " .. tostring(got))
        return got
    "#)).unwrap();
    assert!(
        msg.contains("[Cannot anchor to a region dependent on it]"),
        "msg: {msg}"
    );
}

#[test]
fn test_cycle_error_all0_set_all_points_self() {
    let env = env();
    let msg: String = env.eval(&format!(r#"
        {RSTR_LUA}
        local f = CreateFrame("Frame")
        local expected = table.concat({{
            'Action[SetPoint] failed because',
            '[Cannot anchor to itself]: ',
            'attempted from: Frame:SetAllPoints.',
        }})
        local ok, got = pcall(f.SetAllPoints, f, f)
        assert(not ok, "should fail")
        assert(got == expected, "msg mismatch:\nexpected: " .. tostring(expected) .. "\ngot: " .. tostring(got))
        return got
    "#)).unwrap();
    assert!(msg.contains("Frame:SetAllPoints"), "msg: {msg}");
}

#[test]
fn test_cycle_error_all3_set_all_points_chain() {
    let env = env();
    let msg: String = env.eval(&format!(r#"
        {RSTR_LUA}
        local f = CreateFrame("Frame")
        local g = CreateFrame("Frame")
        local h = CreateFrame("Frame")
        local i = CreateFrame("Frame")
        local expected = table.concat({{
            'Action[SetPoint] failed because',
            '[Cannot anchor to a region dependent on it]: ',
            'attempted from: Frame:SetAllPoints.\n',
            'Relative: [' .. rstr(i) .. ']\n',
            'Dependent: [' .. rstr(g) .. ']\n',
            'Dependent ancestors:\n',
            '[' .. rstr(h) .. ']\n',
            '[' .. rstr(i) .. ']',
        }})
        g:SetAllPoints(f)
        h:SetAllPoints(g)
        i:SetAllPoints(h)
        local ok, got = pcall(f.SetAllPoints, f, i)
        assert(not ok, "should fail")
        assert(got == expected, "msg mismatch:\nexpected: " .. tostring(expected) .. "\ngot: " .. tostring(got))
        return got
    "#)).unwrap();
    assert!(msg.contains("Frame:SetAllPoints"), "msg: {msg}");
    assert!(msg.contains("Dependent ancestors:"), "msg: {msg}");
}

/// Explicit nil relativeTo in SetPoint should resolve to parent, not screen.
/// This is the pattern used by EditMode's SetPointOverride when forwarding
/// 3-arg SetPoint calls: `base(self, point, nil, nil, offsetX, offsetY)`.
#[test]
fn test_set_point_explicit_nil_relative_to_resolves_to_parent() {
    let env = env();
    let parent_name: String = env
        .eval(
            r#"
        local parent = CreateFrame("Frame", "NilRelParent", UIParent)
        parent:SetSize(200, 100)
        parent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 50, 30)

        local child = CreateFrame("Frame", "NilRelChild", parent)
        child:SetSize(80, 40)
        -- Explicit nil relativeTo (5-arg form with nil, nil)
        child:SetPoint("TOPRIGHT", nil, nil, 0, -11)

        local _, relTo = child:GetPoint(1)
        return relTo:GetName()
    "#,
        )
        .unwrap();
    assert_eq!(
        parent_name, "NilRelParent",
        "explicit nil relativeTo should resolve to parent frame"
    );
}

// ============================================================================
// SetPoint with relativeTo frame and numeric offsets (3-arg after point)
// ============================================================================

/// SetPoint("TOPLEFT", otherFrame, xOfs, yOfs) must anchor to otherFrame,
/// not ignore it and treat (xOfs, yOfs) as a parent-relative offset.
///
/// Reproduces the quest log POI button positioning bug where
/// `poiButton:SetPoint("TOPLEFT", button, 6, -4)` ignored `button` and
/// anchored to the parent instead, placing all icons at the top.
#[test]
fn test_set_point_relative_frame_with_offsets() {
    let env = env();
    let rel_name: String = env
        .eval(
            r#"
        local parent = CreateFrame("Frame", "SPRelParent", UIParent)
        local anchor = CreateFrame("Frame", "SPRelAnchor", parent)
        local child = CreateFrame("Frame", "SPRelChild", parent)
        child:SetPoint("TOPLEFT", anchor, 6, -4)
        local _, relTo = child:GetPoint(1)
        return relTo and relTo:GetName() or "nil"
    "#,
        )
        .unwrap();
    assert_eq!(
        rel_name, "SPRelAnchor",
        "SetPoint('TOPLEFT', frame, x, y) should anchor to frame, not parent"
    );
}

/// Same as above but verify the offsets are correct.
#[test]
fn test_set_point_relative_frame_with_offsets_values() {
    let env = env();
    let (x, y): (f64, f64) = env
        .eval(
            r#"
        local parent = CreateFrame("Frame", "SPOfsParent", UIParent)
        local anchor = CreateFrame("Frame", "SPOfsAnchor", parent)
        local child = CreateFrame("Frame", "SPOfsChild", parent)
        child:SetPoint("TOPLEFT", anchor, 6, -4)
        local _, _, _, x, y = child:GetPoint(1)
        return x, y
    "#,
        )
        .unwrap();
    assert!((x - 6.0).abs() < 0.01, "x offset should be 6, got {x}");
    assert!((y - (-4.0)).abs() < 0.01, "y offset should be -4, got {y}");
}

/// SetPoint("TOPLEFT", otherFrame, "BOTTOMLEFT", x, y) — full 5-arg form
/// should still work (regression guard).
#[test]
fn test_set_point_full_form_still_works() {
    let env = env();
    let (rel_name, rel_point): (String, String) = env
        .eval(
            r#"
        local parent = CreateFrame("Frame", "SPFullParent", UIParent)
        local anchor = CreateFrame("Frame", "SPFullAnchor", parent)
        local child = CreateFrame("Frame", "SPFullChild", parent)
        child:SetPoint("TOPLEFT", anchor, "BOTTOMLEFT", 10, -5)
        local point, relTo, relPoint, x, y = child:GetPoint(1)
        return relTo:GetName(), relPoint
    "#,
        )
        .unwrap();
    assert_eq!(rel_name, "SPFullAnchor");
    assert_eq!(rel_point, "BOTTOMLEFT");
}
