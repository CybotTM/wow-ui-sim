//! Wrath 12340 region contracts; Cataclysm 15595 corroborates only.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn check(body: &str) {
    let env = WowLuaEnv::new().expect("create minimal environment");
    env.exec(FIXTURE).expect("install fixture");
    env.exec(body).expect("native behavioural contract");
}

const FIXTURE: &str = r#"
parent = CreateFrame('Frame', 'F1Parent', UIParent)
parent:SetSize(200,100)
parent:SetPoint('BOTTOMLEFT', UIParent, 'BOTTOMLEFT', 31,47)
other = CreateFrame('Frame', 'F1Other', UIParent)
other:SetSize(80,60)
other:SetPoint('BOTTOMLEFT', UIParent, 'BOTTOMLEFT', 7,11)
child = CreateFrame('Frame', 'F1Child', parent)
child:SetSize(15,17)
function seed_anchor()
    child:ClearAllPoints()
    child:SetPoint('CENTER', other, 'TOPLEFT', 7,-9)
end
function retained_anchor()
    assert(child:GetNumPoints() == 1)
    local p,r,rp,x,y = child:GetPoint(1)
    assert(p == 'CENTER' and r == other and rp == 'TOPLEFT' and x == 7 and y == -9)
end
function all_points_to(target)
    assert(child:GetNumPoints() == 2)
    local seen = {}
    for i = 1, 2 do
        local p,r,rp,x,y = child:GetPoint(i)
        assert(p == 'TOPLEFT' or p == 'BOTTOMRIGHT')
        assert(not seen[p]); seen[p] = true
        assert(r == target and rp == p and x == 0 and y == 0)
    end
    assert(seen.TOPLEFT and seen.BOTTOMRIGHT)
    assert(child:GetParent() == parent)
end
seed_anchor()
"#;

#[test]
fn f1_false_uses_parent_not_noop() {
    check(
        r#"
        assert(select('#', child:SetAllPoints(false)) == 0)
        all_points_to(parent)
        "#,
    );
}

#[test]
fn f1_omitted_and_non_string_scalars_use_parent() {
    check(
        r#"
        assert(select('#', child:SetAllPoints()) == 0)
        all_points_to(parent)
        seed_anchor()
        assert(select('#', child:SetAllPoints(true)) == 0)
        all_points_to(parent)
        seed_anchor()
        assert(select('#', child:SetAllPoints(function() end)) == 0)
        all_points_to(parent)
        "#,
    );
}

#[test]
fn f1_explicit_nil_uses_root_geometry_not_parent() {
    check(
        r#"
        assert(select('#', child:SetAllPoints(nil)) == 0)
        assert(child:GetNumPoints() == 2)
        assert(child:GetParent() == parent)
        local x,y,w,h = child:GetRect()
        local rx,ry,rw,rh = UIParent:GetRect()
        assert(type(w) == 'number' and rw > 200 and rh > 100)
        assert(x == rx and y == ry and w == rw and h == rh)
        -- Oracle does not specify GetPoint's nil-vs-root reference representation.
        "#,
    );
}

#[test]
fn f1_string_name_looks_up_named_target() {
    check(
        r#"
        assert(select('#', child:SetAllPoints('F1Other')) == 0)
        all_points_to(other)
        "#,
    );
}

#[test]
fn f1_numeric_name_looks_up_named_target() {
    check(
        r#"
        local named = CreateFrame('Frame', '42', UIParent)
        assert(select('#', child:SetAllPoints(42)) == 0)
        all_points_to(named)
        "#,
    );
}

#[test]
fn f1_unknown_names_error_and_retain_anchors() {
    check(
        r#"
        for _, value in ipairs({'F1NeverExists', 987654321}) do
            seed_anchor()
            local ok, err = pcall(function() child:SetAllPoints(value) end)
            assert(not ok and type(err) == 'string')
            retained_anchor()
        end
        "#,
    );
}

#[test]
fn f1_valid_frame_and_region_identity_return_no_values() {
    check(
        r#"
        assert(select('#', child:SetAllPoints(other)) == 0)
        all_points_to(other)
        local region = other:CreateTexture()
        assert(select('#', child:SetAllPoints(region)) == 0)
        all_points_to(region)
        "#,
    );
}

#[test]
fn f1_null_or_invalid_slot_zero_errors_before_mutation() {
    check(
        r#"
        -- Real simulator frame table, not arbitrary-table safety testing.
        for _, invalid in ipairs({'nil', 'false'}) do
            local target = CreateFrame('Frame', nil, UIParent)
            if invalid == 'nil' then rawset(target, 0, nil) else rawset(target, 0, false) end
            seed_anchor()
            local ok, err = pcall(function() child:SetAllPoints(target) end)
            assert(not ok and type(err) == 'string')
            retained_anchor()
        end
        "#,
    );
}

#[test]
fn f1_self_and_recursive_dependency_errors_retain_anchors() {
    check(
        r#"
        local a = CreateFrame('Frame', nil, UIParent)
        local b = CreateFrame('Frame', nil, UIParent)
        a:SetPoint('TOPLEFT', child, 'TOPLEFT', 0,0)
        b:SetPoint('BOTTOMRIGHT', a, 'BOTTOMRIGHT', 0,0)
        for _, target in ipairs({child,b}) do
            seed_anchor()
            local ok, err = pcall(function() child:SetAllPoints(target) end)
            assert(not ok and type(err) == 'string')
            retained_anchor()
        end
        "#,
    );
}

#[test]
fn f1_protected_denial_retains_anchors_and_returns_no_values() {
    check(
        r#"
        A_Admin.SetFrameProtected('F1Child', true)
        A_Admin.SetInCombat(true)
        forceinsecure()
        assert(select('#', child:SetAllPoints(other)) == 0)
        retained_anchor()
        "#,
    );
}

#[test]
fn f1_protected_denial_precedes_unknown_name_validation() {
    check(
        r#"
        A_Admin.SetFrameProtected('F1Child', true)
        A_Admin.SetInCombat(true)
        forceinsecure()
        assert(select('#', child:SetAllPoints('F1NeverExists')) == 0)
        retained_anchor()
        "#,
    );
}
