//! Pins the rect / geometry query family ported from master-era
//! `methods_rect.rs` onto the live rilua registrar. Each test creates
//! a frame, anchors it into UIParent at a known position, then asserts
//! the returned WoW-coord values match the expected layout.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

/// Anchor a 100×50 frame at TOPLEFT + (10, -20) and read back every
/// rect query in one eval. The TOPLEFT anchor puts the frame's top
/// edge 20 below the top of UIParent in screen space — which, in the
/// WoW UI coord system (origin bottom-left), means the top edge sits
/// at `screen_height - 20` (before effective-scale division).
#[test]
fn rect_family_returns_consistent_coords_for_topleft_anchored_frame() {
    let env = env();
    let (left, bottom, width, height, get_left, get_right, get_top, get_bottom, cx, cy): (
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
    ) = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "RectProbe", UIParent)
            f:SetSize(100, 50)
            f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 10, -20)
            local left, bottom, width, height = f:GetRect()
            local get_left = f:GetLeft()
            local get_right = f:GetRight()
            local get_top = f:GetTop()
            local get_bottom = f:GetBottom()
            local cx, cy = f:GetCenter()
            return left, bottom, width, height,
                   get_left, get_right, get_top, get_bottom, cx, cy
            "#,
        )
        .expect("eval should succeed");

    // Position/size come back intact after coord-system conversion.
    assert_eq!(width, 100.0);
    assert_eq!(height, 50.0);
    assert_eq!(left, 10.0, "GetRect.left should match TOPLEFT.x offset");
    assert_eq!(get_left, left);
    assert_eq!(get_right, left + width);
    // GetRect.bottom + height == GetTop.
    assert_eq!(get_top, bottom + height);
    assert_eq!(get_bottom, bottom);
    // Center is mid-rect.
    assert_eq!(cx, left + width / 2.0);
    assert_eq!(cy, bottom + height / 2.0);
}

#[test]
fn get_size_returns_width_height_pair() {
    let env = env();
    let (w, h): (f64, f64) = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "SizeProbe", UIParent)
            f:SetSize(123, 77)
            f:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
            return f:GetSize()
            "#,
        )
        .expect("eval should succeed");
    assert_eq!(w, 123.0);
    assert_eq!(h, 77.0);
}

#[test]
fn scaled_rect_is_unscaled_layout_rect() {
    // When the frame inherits UIParent's effective scale (the default
    // 1.0), GetScaledRect width/height match GetRect's. The bottom
    // edge in GetScaledRect uses screen-space coords (no scale
    // division) so it differs from GetRect.bottom only when scale ≠ 1.
    let env = env();
    let (rect_left, rect_bottom, rect_w, rect_h, scaled_left, scaled_bottom, scaled_w, scaled_h): (
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
    ) = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "ScaledRectProbe", UIParent)
            f:SetSize(40, 30)
            f:SetPoint("BOTTOMLEFT", UIParent, "BOTTOMLEFT", 5, 5)
            local rl, rb, rw, rh = f:GetRect()
            local sl, sb, sw, sh = f:GetScaledRect()
            return rl, rb, rw, rh, sl, sb, sw, sh
            "#,
        )
        .expect("eval should succeed");
    assert_eq!(rect_w, scaled_w);
    assert_eq!(rect_h, scaled_h);
    assert_eq!(rect_left, scaled_left);
    assert_eq!(rect_bottom, scaled_bottom);
}

#[test]
fn rect_queries_return_nothing_for_unanchored_frame() {
    // A frame with no anchors should return nil (0 values from Lua) —
    // matches Blizzard's "unanchored frames have no queryable rect".
    let env = env();
    let result: String = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "UnanchoredProbe", UIParent)
            return type(f:GetRect())
            "#,
        )
        .expect("eval should succeed");
    assert_eq!(result, "nil");
}

#[test]
fn child_anchored_to_unanchored_parent_has_no_queryable_rect() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "UnanchoredParentProbe", UIParent)
            parent:SetSize(100, 100)

            local child = CreateFrame("Frame", "ChildOfUnanchoredParentProbe", parent)
            child:SetSize(20, 20)
            child:SetPoint("CENTER", parent, "CENTER", 0, 0)

            return type(child:GetRect())
            "#,
        )
        .expect("eval should succeed");
    assert_eq!(result, "nil");
}

#[test]
fn waypoint_style_fit_content_skips_child_without_queryable_ancestor_chain() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "WaypointFitParentProbe", UIParent)
            parent:SetSize(100, 100)

            local child = CreateFrame("Frame", "WaypointFitChildProbe", parent)
            child:SetSize(20, 20)
            child:SetPoint("CENTER", parent, "CENTER", 0, 0)

            local parentLeft = parent:GetLeft()
            local childLeft = child:GetRect()
            if childLeft then
                return tostring(childLeft - parentLeft)
            end
            return "skipped"
            "#,
        )
        .expect("eval should succeed");
    assert_eq!(result, "skipped");
}
