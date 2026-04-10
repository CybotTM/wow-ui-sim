//! Tests for scale: SetScale, GetScale, GetEffectiveScale, propagation.

use super::*;

#[test]
fn test_scale_defaults() {
    let (t, _) = load_test_lua(
        "layout-scale-def",
        r#"
        local f = CreateFrame("Frame", nil, UIParent)
        f:SetAllPoints(UIParent)
        SCALE = f:GetScale()
        EFF_SCALE = f:GetEffectiveScale()
    "#,
    );
    assert_eq!(t.env.eval::<f64>("return SCALE").unwrap(), 1.0);
    assert_eq!(t.env.eval::<f64>("return EFF_SCALE").unwrap(), 1.0);
}

#[test]
fn test_set_scale() {
    let (t, _) = load_test_lua(
        "layout-set-scale",
        r#"
        local f = CreateFrame("Frame")
        f:SetScale(2.5)
        SCALE = f:GetScale()
    "#,
    );
    let s = t.env.eval::<f64>("return SCALE").unwrap();
    assert!((s - 2.5).abs() < 0.001, "expected 2.5, got {}", s);
}

#[test]
fn test_set_scale_zero_errors() {
    let (t, _) = load_test_lua(
        "layout-scale-zero",
        r#"
        local f = CreateFrame("Frame")
        ERRORED = not pcall(function() f:SetScale(0) end)
    "#,
    );
    t.assert_lua_true("return ERRORED", "SetScale(0) should error");
}

#[test]
fn test_set_scale_negative_errors() {
    let (t, _) = load_test_lua(
        "layout-scale-neg",
        r#"
        local f = CreateFrame("Frame")
        ERRORED = not pcall(function() f:SetScale(-1) end)
    "#,
    );
    t.assert_lua_true("return ERRORED", "SetScale(-1) should error");
}

#[test]
fn test_set_scale_small_positive() {
    let (t, _) = load_test_lua(
        "layout-scale-small",
        r#"
        local f = CreateFrame("Frame")
        f:SetScale(0.001)
        SCALE = f:GetScale()
    "#,
    );
    let s = t.env.eval::<f64>("return SCALE").unwrap();
    assert!((s - 0.001).abs() < 0.0001, "expected 0.001, got {}", s);
}

#[test]
fn test_effective_scale_parent_child() {
    let (t, _) = load_test_lua(
        "layout-effscale",
        r#"
        local parent = CreateFrame("Frame", nil, UIParent)
        parent:SetAllPoints(UIParent)
        parent:SetScale(2.0)
        local child = CreateFrame("Frame", nil, parent)
        child:SetAllPoints(parent)
        child:SetScale(3.0)
        PARENT_EFF = parent:GetEffectiveScale()
        CHILD_EFF = child:GetEffectiveScale()
    "#,
    );
    let pe = t.env.eval::<f64>("return PARENT_EFF").unwrap();
    let ce = t.env.eval::<f64>("return CHILD_EFF").unwrap();
    assert!((pe - 2.0).abs() < 0.001, "parent: {}", pe);
    assert!((ce - 6.0).abs() < 0.001, "child: {}", ce);
}

#[test]
fn test_effective_scale_three_levels() {
    let (t, _) = load_test_lua(
        "layout-effscale3",
        r#"
        local a = CreateFrame("Frame", nil, UIParent)
        a:SetAllPoints(UIParent); a:SetScale(2.0)
        local b = CreateFrame("Frame", nil, a)
        b:SetAllPoints(a); b:SetScale(1.5)
        local c = CreateFrame("Frame", nil, b)
        c:SetAllPoints(b); c:SetScale(0.5)
        A_EFF = a:GetEffectiveScale()
        B_EFF = b:GetEffectiveScale()
        C_EFF = c:GetEffectiveScale()
    "#,
    );
    let a = t.env.eval::<f64>("return A_EFF").unwrap();
    let b = t.env.eval::<f64>("return B_EFF").unwrap();
    let c = t.env.eval::<f64>("return C_EFF").unwrap();
    assert!((a - 2.0).abs() < 0.001, "a: {}", a);
    assert!((b - 3.0).abs() < 0.001, "b: {}", b);
    assert!((c - 1.5).abs() < 0.001, "c: {}", c);
}

#[test]
fn test_set_scale_updates_child_effective() {
    let (t, _) = load_test_lua(
        "layout-scale-update",
        r#"
        local parent = CreateFrame("Frame", nil, UIParent)
        parent:SetAllPoints(UIParent); parent:SetScale(1.0)
        local child = CreateFrame("Frame", nil, parent)
        child:SetAllPoints(parent); child:SetScale(2.0)
        EFF_BEFORE = child:GetEffectiveScale()
        parent:SetScale(3.0)
        EFF_AFTER = child:GetEffectiveScale()
    "#,
    );
    let before = t.env.eval::<f64>("return EFF_BEFORE").unwrap();
    let after = t.env.eval::<f64>("return EFF_AFTER").unwrap();
    assert!((before - 2.0).abs() < 0.001, "before: {}", before);
    assert!((after - 6.0).abs() < 0.001, "after: {}", after);
}

#[test]
fn test_scale_does_not_affect_explicit_size() {
    let (t, _) = load_test_lua(
        "layout-scale-size",
        r#"
        local f = CreateFrame("Frame", nil, UIParent)
        f:SetSize(100, 50)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT")
        f:SetScale(2.0)
        W = f:GetWidth()
        H = f:GetHeight()
    "#,
    );
    assert_eq!(t.env.eval::<f64>("return W").unwrap(), 100.0);
    assert_eq!(t.env.eval::<f64>("return H").unwrap(), 50.0);
}

#[test]
fn test_scale_divides_position_queries() {
    let (t, _) = load_test_lua(
        "layout-scale-leftright",
        r#"
        local f = CreateFrame("Frame", nil, UIParent)
        f:SetSize(100, 50)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 0, 0)
        f:SetScale(2.0)
        LEFT = f:GetLeft()
        RIGHT = f:GetRight()
        WIDTH = f:GetWidth()
        INVARIANT = math.abs(RIGHT - LEFT - WIDTH) < 0.01
    "#,
    );
    let left = t.env.eval::<f64>("return LEFT").unwrap();
    let right = t.env.eval::<f64>("return RIGHT").unwrap();
    assert!((left - 0.0).abs() < 0.01, "left: {}", left);
    assert!((right - 100.0).abs() < 0.01, "right: {}", right);
    t.assert_lua_true("return INVARIANT", "right - left should equal width");
}

#[test]
fn test_scale_affects_get_rect() {
    let (t, _) = load_test_lua(
        "layout-scale-rect",
        r#"
        local f = CreateFrame("Frame", nil, UIParent)
        f:SetSize(100, 50)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 0, 0)
        f:SetScale(2.0)
        local left, bottom, w, h = f:GetRect()
        LEFT = left; W = w; H = h
    "#,
    );
    assert!((t.env.eval::<f64>("return LEFT").unwrap()).abs() < 0.01);
    assert!((t.env.eval::<f64>("return W").unwrap() - 100.0).abs() < 0.01);
    assert!((t.env.eval::<f64>("return H").unwrap() - 50.0).abs() < 0.01);
}

#[test]
fn test_ignore_parent_scale_flag_round_trip() {
    let (t, _) = load_test_lua(
        "layout-ignore-parent-scale-flag",
        r#"
        local f = CreateFrame("Frame", nil, UIParent)
        BEFORE = f:GetIgnoreParentScale()
        f:SetIgnoreParentScale(true)
        AFTER_ENABLE = f:GetIgnoreParentScale()
        f:SetIgnoreParentScale(false)
        AFTER_DISABLE = f:GetIgnoreParentScale()
    "#,
    );
    assert!(
        !t.env.eval::<bool>("return BEFORE").unwrap(),
        "frames should default to GetIgnoreParentScale() == false",
    );
    t.assert_lua_true(
        "return AFTER_ENABLE",
        "SetIgnoreParentScale(true) should persist on the frame",
    );
    assert!(
        !t.env.eval::<bool>("return AFTER_DISABLE").unwrap(),
        "SetIgnoreParentScale(false) should clear the frame state",
    );
}

#[test]
fn test_ignore_parent_scale_changes_effective_scale_propagation() {
    let (t, _) = load_test_lua(
        "layout-ignore-parent-scale-propagation",
        r#"
        local parent = CreateFrame("Frame", nil, UIParent)
        parent:SetAllPoints(UIParent)
        parent:SetScale(2.0)

        local child = CreateFrame("Frame", nil, parent)
        child:SetAllPoints(parent)
        child:SetScale(3.0)

        INHERITED = child:GetEffectiveScale()
        child:SetIgnoreParentScale(true)
        IGNORED = child:GetEffectiveScale()

        parent:SetScale(4.0)
        IGNORED_AFTER_PARENT_CHANGE = child:GetEffectiveScale()

        child:SetIgnoreParentScale(false)
        REINHERITED = child:GetEffectiveScale()
    "#,
    );

    assert!((t.env.eval::<f64>("return INHERITED").unwrap() - 6.0).abs() < 0.001);
    assert!((t.env.eval::<f64>("return IGNORED").unwrap() - 3.0).abs() < 0.001);
    assert!(
        (t.env
            .eval::<f64>("return IGNORED_AFTER_PARENT_CHANGE")
            .unwrap()
            - 3.0)
            .abs()
            < 0.001
    );
    assert!((t.env.eval::<f64>("return REINHERITED").unwrap() - 12.0).abs() < 0.001);
}
