//! Tests for frame level behavior: SetFrameLevel, SetFixedFrameLevel, SetParent interaction.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

// ============================================================================
// SetFixedFrameLevel + SetParent interaction
// ============================================================================

#[test]
fn test_fixed_frame_level_preserved_on_reparent() {
    let env = env();
    // Fixed level should be preserved when reparenting
    let (f_level, g_level): (i32, i32) = env
        .eval(
            r#"
        local h = CreateFrame("Frame")
        local f = CreateFrame("Frame")
        local g = CreateFrame("Frame", nil, f)
        f:SetParent(nil)
        f:SetFrameLevel(42)
        f:SetFixedFrameLevel(true)
        f:SetParent(h)
        return f:GetFrameLevel(), g:GetFrameLevel()
    "#,
        )
        .unwrap();
    assert_eq!(
        f_level, 42,
        "Fixed frame level should be preserved on reparent"
    );
    assert_eq!(g_level, 43, "Child should inherit from fixed parent level");
}

#[test]
fn test_frame_level_same_parent_no_recalc() {
    let env = env();
    // After disabling fixed level, re-assigning same parent should NOT recalculate
    let (f_level, g_level): (i32, i32) = env
        .eval(
            r#"
        local h = CreateFrame("Frame")
        local f = CreateFrame("Frame")
        local g = CreateFrame("Frame", nil, f)
        f:SetParent(nil)
        f:SetFrameLevel(42)
        f:SetFixedFrameLevel(true)
        f:SetParent(h)
        f:SetFixedFrameLevel(false)
        f:SetParent(h)
        return f:GetFrameLevel(), g:GetFrameLevel()
    "#,
        )
        .unwrap();
    assert_eq!(
        f_level, 42,
        "Same-parent reparent should not recalculate level"
    );
    assert_eq!(
        g_level, 43,
        "Child should keep inherited level on same-parent reparent"
    );
}

#[test]
fn test_same_parent_set_parent_preserves_child_custom_frame_level() {
    let env = env();
    let child_level: i32 = env
        .eval(
            r#"
        local host = CreateFrame("Frame")
        local parent = CreateFrame("Frame")
        local child = CreateFrame("Frame", nil, parent)

        parent:SetParent(host)
        parent:SetFrameLevel(100)
        child:SetFrameLevel(7)

        parent:SetParent(host)
        return child:GetFrameLevel()
    "#,
        )
        .unwrap();
    assert_eq!(
        child_level, 7,
        "same-parent SetParent should not clobber explicitly assigned child frame levels",
    );
}

#[test]
fn test_frame_level_recalc_after_nil_reparent() {
    let env = env();
    // After going through nil parent, reparenting should recalculate
    let (f_level, g_level): (i32, i32) = env
        .eval(
            r#"
        local h = CreateFrame("Frame")
        local f = CreateFrame("Frame")
        local g = CreateFrame("Frame", nil, f)
        f:SetParent(nil)
        f:SetFrameLevel(42)
        f:SetFixedFrameLevel(true)
        f:SetParent(h)
        f:SetFixedFrameLevel(false)
        f:SetParent(nil)
        f:SetParent(h)
        return f:GetFrameLevel(), g:GetFrameLevel()
    "#,
        )
        .unwrap();
    assert_eq!(
        f_level, 1,
        "Level should recalculate after nil->parent transition"
    );
    assert_eq!(
        g_level, 2,
        "Child should recalculate after parent's nil->parent transition"
    );
}
