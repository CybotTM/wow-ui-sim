//! Tests for alpha: SetAlpha, GetAlpha, GetEffectiveAlpha, propagation, clamping.

use super::*;

#[test]
fn test_alpha_defaults() {
    let (t, _) = load_test_lua(
        "layout-alpha-def",
        r#"
        local f = CreateFrame("Frame", nil, UIParent)
        f:SetAllPoints(UIParent)
        ALPHA = f:GetAlpha()
        EFF_ALPHA = f:GetEffectiveAlpha()
    "#,
    );
    assert_eq!(t.env.eval::<f64>("return ALPHA").unwrap(), 1.0);
    assert_eq!(t.env.eval::<f64>("return EFF_ALPHA").unwrap(), 1.0);
}

#[test]
fn test_set_alpha() {
    let (t, _) = load_test_lua(
        "layout-set-alpha",
        r#"
        local f = CreateFrame("Frame")
        f:SetAlpha(0.5)
        ALPHA = f:GetAlpha()
    "#,
    );
    let a = t.env.eval::<f64>("return ALPHA").unwrap();
    assert!((a - 0.5).abs() < 0.001, "expected 0.5, got {}", a);
}

#[test]
fn test_set_alpha_zero() {
    let (t, _) = load_test_lua(
        "layout-alpha-zero",
        r#"
        local f = CreateFrame("Frame")
        f:SetAlpha(0)
        ALPHA = f:GetAlpha()
    "#,
    );
    assert_eq!(t.env.eval::<f64>("return ALPHA").unwrap(), 0.0);
}

#[test]
fn test_set_alpha_clamps_above_1() {
    let (t, _) = load_test_lua(
        "layout-alpha-clamp-hi",
        r#"
        local f = CreateFrame("Frame")
        f:SetAlpha(2.0)
        ALPHA = f:GetAlpha()
    "#,
    );
    assert_eq!(t.env.eval::<f64>("return ALPHA").unwrap(), 1.0);
}

#[test]
fn test_set_alpha_clamps_below_0() {
    let (t, _) = load_test_lua(
        "layout-alpha-clamp-lo",
        r#"
        local f = CreateFrame("Frame")
        f:SetAlpha(-0.5)
        ALPHA = f:GetAlpha()
    "#,
    );
    assert_eq!(t.env.eval::<f64>("return ALPHA").unwrap(), 0.0);
}

#[test]
fn test_effective_alpha_parent_child() {
    let (t, _) = load_test_lua(
        "layout-effalpha",
        r#"
        local parent = CreateFrame("Frame", nil, UIParent)
        parent:SetAllPoints(UIParent); parent:SetAlpha(0.5)
        local child = CreateFrame("Frame", nil, parent)
        child:SetAllPoints(parent); child:SetAlpha(0.8)
        PARENT_EFF = parent:GetEffectiveAlpha()
        CHILD_EFF = child:GetEffectiveAlpha()
    "#,
    );
    let pe = t.env.eval::<f64>("return PARENT_EFF").unwrap();
    let ce = t.env.eval::<f64>("return CHILD_EFF").unwrap();
    assert!((pe - 0.5).abs() < 0.001, "parent: {}", pe);
    assert!((ce - 0.4).abs() < 0.001, "child: {}", ce);
}

#[test]
fn test_effective_alpha_three_levels() {
    let (t, _) = load_test_lua(
        "layout-effalpha3",
        r#"
        local a = CreateFrame("Frame", nil, UIParent)
        a:SetAllPoints(UIParent); a:SetAlpha(0.5)
        local b = CreateFrame("Frame", nil, a)
        b:SetAllPoints(a); b:SetAlpha(0.6)
        local c = CreateFrame("Frame", nil, b)
        c:SetAllPoints(b); c:SetAlpha(0.8)
        A_EFF = a:GetEffectiveAlpha()
        B_EFF = b:GetEffectiveAlpha()
        C_EFF = c:GetEffectiveAlpha()
    "#,
    );
    let a = t.env.eval::<f64>("return A_EFF").unwrap();
    let b = t.env.eval::<f64>("return B_EFF").unwrap();
    let c = t.env.eval::<f64>("return C_EFF").unwrap();
    assert!((a - 0.5).abs() < 0.001, "a: {}", a);
    assert!((b - 0.3).abs() < 0.001, "b: {}", b);
    assert!((c - 0.24).abs() < 0.001, "c: {}", c);
}

#[test]
fn test_set_alpha_updates_child_effective() {
    let (t, _) = load_test_lua(
        "layout-alpha-update",
        r#"
        local parent = CreateFrame("Frame", nil, UIParent)
        parent:SetAllPoints(UIParent); parent:SetAlpha(1.0)
        local child = CreateFrame("Frame", nil, parent)
        child:SetAllPoints(parent); child:SetAlpha(0.5)
        EFF_BEFORE = child:GetEffectiveAlpha()
        parent:SetAlpha(0.4)
        EFF_AFTER = child:GetEffectiveAlpha()
    "#,
    );
    let before = t.env.eval::<f64>("return EFF_BEFORE").unwrap();
    let after = t.env.eval::<f64>("return EFF_AFTER").unwrap();
    assert!((before - 0.5).abs() < 0.001, "before: {}", before);
    assert!((after - 0.2).abs() < 0.001, "after: {}", after);
}

#[test]
fn test_parent_alpha_zero_zeroes_child() {
    let (t, _) = load_test_lua(
        "layout-alpha-zero-prop",
        r#"
        local parent = CreateFrame("Frame", nil, UIParent)
        parent:SetAllPoints(UIParent); parent:SetAlpha(0)
        local child = CreateFrame("Frame", nil, parent)
        child:SetAllPoints(parent); child:SetAlpha(1.0)
        CHILD_EFF = child:GetEffectiveAlpha()
    "#,
    );
    assert_eq!(t.env.eval::<f64>("return CHILD_EFF").unwrap(), 0.0);
}

#[test]
fn test_alpha_independent_of_scale() {
    let (t, _) = load_test_lua(
        "layout-alpha-indep",
        r#"
        local f = CreateFrame("Frame", nil, UIParent)
        f:SetAllPoints(UIParent)
        f:SetAlpha(0.7); f:SetScale(3.0)
        ALPHA = f:GetAlpha()
        EFF_ALPHA = f:GetEffectiveAlpha()
    "#,
    );
    let a = t.env.eval::<f64>("return ALPHA").unwrap();
    let ea = t.env.eval::<f64>("return EFF_ALPHA").unwrap();
    assert!((a - 0.7).abs() < 0.001, "alpha: {}", a);
    assert!((ea - 0.7).abs() < 0.001, "eff alpha: {}", ea);
}

#[test]
fn test_ignore_parent_alpha_flag_round_trip() {
    let (t, _) = load_test_lua(
        "layout-ignore-parent-alpha-flag",
        r#"
        local f = CreateFrame("Frame")
        BEFORE = f:GetIgnoreParentAlpha()
        BEFORE_ALIAS = f:IsIgnoringParentAlpha()
        f:SetIgnoreParentAlpha(true)
        AFTER_ENABLE = f:GetIgnoreParentAlpha()
        AFTER_ENABLE_ALIAS = f:IsIgnoringParentAlpha()
        f:SetIgnoreParentAlpha(false)
        AFTER_DISABLE = f:GetIgnoreParentAlpha()
        AFTER_DISABLE_ALIAS = f:IsIgnoringParentAlpha()
    "#,
    );
    assert!(
        !t.env.eval::<bool>("return BEFORE").unwrap(),
        "frames should default to GetIgnoreParentAlpha() == false",
    );
    assert!(
        !t.env.eval::<bool>("return BEFORE_ALIAS").unwrap(),
        "frames should default to IsIgnoringParentAlpha() == false",
    );
    assert!(
        t.env.eval::<bool>("return AFTER_ENABLE").unwrap(),
        "SetIgnoreParentAlpha(true) should persist on the frame",
    );
    assert!(
        t.env.eval::<bool>("return AFTER_ENABLE_ALIAS").unwrap(),
        "IsIgnoringParentAlpha() should reflect SetIgnoreParentAlpha(true)",
    );
    assert!(
        !t.env.eval::<bool>("return AFTER_DISABLE").unwrap(),
        "SetIgnoreParentAlpha(false) should clear the frame state",
    );
    assert!(
        !t.env.eval::<bool>("return AFTER_DISABLE_ALIAS").unwrap(),
        "IsIgnoringParentAlpha() should reflect SetIgnoreParentAlpha(false)",
    );
}

#[test]
fn test_ignore_parent_alpha_changes_effective_alpha_propagation() {
    let (t, _) = load_test_lua(
        "layout-ignore-parent-alpha-prop",
        r#"
        local parent = CreateFrame("Frame", nil, UIParent)
        parent:SetAllPoints(UIParent)
        parent:SetAlpha(0.5)

        local child = CreateFrame("Frame", nil, parent)
        child:SetAllPoints(parent)
        child:SetAlpha(0.8)

        EFF_BEFORE = child:GetEffectiveAlpha()
        child:SetIgnoreParentAlpha(true)
        EFF_IGNORING = child:GetEffectiveAlpha()

        parent:SetAlpha(0.25)
        EFF_AFTER_PARENT_CHANGE = child:GetEffectiveAlpha()

        child:SetIgnoreParentAlpha(false)
        EFF_AFTER_REINHERIT = child:GetEffectiveAlpha()
    "#,
    );
    let before = t.env.eval::<f64>("return EFF_BEFORE").unwrap();
    let ignoring = t.env.eval::<f64>("return EFF_IGNORING").unwrap();
    let after_parent_change = t.env.eval::<f64>("return EFF_AFTER_PARENT_CHANGE").unwrap();
    let after_reinherit = t.env.eval::<f64>("return EFF_AFTER_REINHERIT").unwrap();

    assert!((before - 0.4).abs() < 0.001, "before: {}", before);
    assert!(
        (ignoring - 0.8).abs() < 0.001,
        "ignoring parent alpha should use only child alpha: {}",
        ignoring
    );
    assert!(
        (after_parent_change - 0.8).abs() < 0.001,
        "ignored child alpha should not change when parent alpha changes: {}",
        after_parent_change
    );
    assert!(
        (after_reinherit - 0.2).abs() < 0.001,
        "disabling ignore-parent-alpha should reinherit parent alpha: {}",
        after_reinherit
    );
}

#[test]
fn test_xml_alpha_zero_on_button() {
    let t = load_test_xml(
        "test-xml-alpha-zero",
        r#"<Ui>
            <Frame name="TestAlphaParent" parent="UIParent">
                <Size x="200" y="200"/>
                <Anchors><Anchor point="CENTER"/></Anchors>
                <Frames>
                    <Button name="TestAlphaZeroBtn" alpha="0">
                        <Size x="60" y="60"/>
                        <Anchors><Anchor point="BOTTOMRIGHT"/></Anchors>
                    </Button>
                </Frames>
            </Frame>
        </Ui>"#,
    );

    let alpha: f64 = t.env.eval("return TestAlphaZeroBtn:GetAlpha()").unwrap();
    assert!(
        alpha.abs() < 0.001,
        "XML alpha=\"0\" on Button should result in GetAlpha()=0, got {alpha}"
    );

    let eff: f64 = t
        .env
        .eval("return TestAlphaZeroBtn:GetEffectiveAlpha()")
        .unwrap();
    assert!(
        eff.abs() < 0.001,
        "XML alpha=\"0\" should propagate to effective alpha=0, got {eff}"
    );
}

#[test]
fn test_xml_alpha_zero_child_textures_invisible() {
    let t = load_test_xml(
        "test-xml-alpha-zero-children",
        r#"<Ui>
            <Frame name="TestAlphaParent2" parent="UIParent">
                <Size x="200" y="200"/>
                <Anchors><Anchor point="CENTER"/></Anchors>
                <Frames>
                    <Button name="TestAlphaZeroBtn2" alpha="0">
                        <Size x="60" y="60"/>
                        <Anchors><Anchor point="BOTTOMRIGHT"/></Anchors>
                        <NormalTexture name="TestAlphaZeroNormal" file="Interface/Buttons/test"/>
                    </Button>
                </Frames>
            </Frame>
        </Ui>"#,
    );

    let child_eff: f64 = t
        .env
        .eval("return TestAlphaZeroNormal:GetEffectiveAlpha()")
        .unwrap();
    assert!(
        child_eff.abs() < 0.001,
        "Child texture of alpha=0 button should have effective alpha=0, got {child_eff}"
    );
}

/// Regression: template child frames with alpha="0" must have alpha applied
/// when instantiated via CreateFrame. Before the fix, `apply_inline_frame_content`
/// in the template path skipped alpha, so the child kept the default alpha=1.
#[test]
fn test_template_child_alpha_zero() {
    let t = load_test_xml(
        "test-tpl-alpha-zero",
        r#"<Ui>
            <Frame name="TplAlphaTestTemplate" virtual="true">
                <Size x="400" y="200"/>
                <Frames>
                    <Button parentKey="ResizeBtn" alpha="0">
                        <Size x="60" y="60"/>
                        <Anchors><Anchor point="BOTTOMRIGHT"/></Anchors>
                    </Button>
                </Frames>
            </Frame>
        </Ui>"#,
    );

    // Instantiate the template via Lua (template child creation path)
    t.env
        .exec(
            r#"
        local f = CreateFrame("Frame", "TplAlphaInstance", UIParent, "TplAlphaTestTemplate")
        f:SetPoint("CENTER")
    "#,
        )
        .unwrap();

    let alpha: f64 = t
        .env
        .eval("return TplAlphaInstance.ResizeBtn:GetAlpha()")
        .unwrap();
    assert!(
        alpha.abs() < 0.001,
        "Template child with alpha=\"0\" should have GetAlpha()=0 after instantiation, got {alpha}"
    );
}
