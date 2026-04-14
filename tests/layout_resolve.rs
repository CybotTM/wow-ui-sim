//! Tests for resolve_rect_if_dirty fast path: directly dirty frames skip the
//! ancestor walk, while frames inheriting dirtiness from ancestors still resolve
//! correctly.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn directly_dirty_frame_resolves_via_fast_path() {
    let env = env();
    // Create a frame, set its anchor and size (marks it dirty), then query its
    // rect. The fast path should resolve it without walking ancestors.
    let (w, h): (f64, f64) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "FastPathFrame", UIParent)
        f:SetSize(200, 100)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 50, -30)
        return f:GetWidth(), f:GetHeight()
    "#,
        )
        .unwrap();
    assert!((w - 200.0).abs() < 0.01, "width should be 200, got {w}");
    assert!((h - 100.0).abs() < 0.01, "height should be 100, got {h}");
}

#[test]
fn inherited_dirty_ancestor_resolves_child() {
    let env = env();
    // Create parent + child. Dirty the parent, then query the child's rect.
    // The child inherits dirtiness from the parent and should still resolve.
    let (w, h): (f64, f64) = env
        .eval(
            r#"
        local parent = CreateFrame("Frame", "DirtyParent", UIParent)
        parent:SetSize(400, 300)
        parent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 0, 0)
        local child = CreateFrame("Frame", "DirtyChild", parent)
        child:SetSize(100, 50)
        child:SetPoint("CENTER", parent, "CENTER", 0, 0)
        -- Force initial layout resolution
        local _ = child:GetWidth()
        -- Now dirty the parent (simulates runtime anchor change)
        parent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 10, -10)
        -- Query child — should still return correct dimensions
        return child:GetWidth(), child:GetHeight()
    "#,
        )
        .unwrap();
    assert!((w - 100.0).abs() < 0.01, "child width should be 100, got {w}");
    assert!((h - 50.0).abs() < 0.01, "child height should be 50, got {h}");
}

#[test]
fn multiple_dirty_ancestors_resolve_correctly() {
    let env = env();
    // Chain: grandparent → parent → child. Dirty both grandparent and parent,
    // then query child's rect.
    let (w, h): (f64, f64) = env
        .eval(
            r#"
        local gp = CreateFrame("Frame", "DirtyGrandparent", UIParent)
        gp:SetSize(600, 400)
        gp:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 0, 0)
        local parent = CreateFrame("Frame", "DirtyParent2", gp)
        parent:SetSize(300, 200)
        parent:SetPoint("CENTER", gp, "CENTER", 0, 0)
        local child = CreateFrame("Frame", "DirtyChild2", parent)
        child:SetSize(80, 40)
        child:SetPoint("CENTER", parent, "CENTER", 0, 0)
        -- Force initial layout
        local _ = child:GetWidth()
        -- Dirty both ancestors
        gp:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -20)
        parent:SetPoint("CENTER", gp, "CENTER", 5, -5)
        -- Query child
        return child:GetWidth(), child:GetHeight()
    "#,
        )
        .unwrap();
    assert!((w - 80.0).abs() < 0.01, "child width should be 80, got {w}");
    assert!((h - 40.0).abs() < 0.01, "child height should be 40, got {h}");
}
