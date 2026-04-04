//! Tests for CheckButton creation behavior and child management.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_create_checkbutton_basic() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestCheckButton", UIParent)
        cb:SetSize(24, 24)
        cb:SetPoint("CENTER")
    "#,
    )
    .unwrap();

    let obj_type: String = env.eval("return TestCheckButton:GetObjectType()").unwrap();
    assert_eq!(obj_type, "CheckButton");

    let has_normal: bool = env
        .eval("return TestCheckButton:GetNormalTexture() ~= nil")
        .unwrap();
    let has_pushed: bool = env
        .eval("return TestCheckButton:GetPushedTexture() ~= nil")
        .unwrap();
    let has_highlight: bool = env
        .eval("return TestCheckButton:GetHighlightTexture() ~= nil")
        .unwrap();

    assert!(
        !has_normal,
        "Fresh CheckButton should not have NormalTexture"
    );
    assert!(
        !has_pushed,
        "Fresh CheckButton should not have PushedTexture"
    );
    assert!(
        !has_highlight,
        "Fresh CheckButton should not have HighlightTexture"
    );
}

#[test]
fn test_checkbutton_checked_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestCheckButtonState", UIParent)
        cb:SetSize(24, 24)
    "#,
    )
    .unwrap();

    let initially_checked: bool = env
        .eval("return TestCheckButtonState:GetChecked()")
        .unwrap();
    assert!(!initially_checked, "CheckButton should start unchecked");

    env.exec("TestCheckButtonState:SetChecked(true)").unwrap();
    let now_checked: bool = env
        .eval("return TestCheckButtonState:GetChecked()")
        .unwrap();
    assert!(
        now_checked,
        "CheckButton should be checked after SetChecked(true)"
    );

    env.exec("TestCheckButtonState:SetChecked(false)").unwrap();
    let now_unchecked: bool = env
        .eval("return TestCheckButtonState:GetChecked()")
        .unwrap();
    assert!(
        !now_unchecked,
        "CheckButton should be unchecked after SetChecked(false)"
    );
}

#[test]
fn test_checkbutton_settext_creates_text_child() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestCheckBoxTemplate", UIParent)
        cb:SetSize(24, 24)
        cb:SetText("Test Label")
    "#,
    )
    .unwrap();

    let has_text: bool = env.eval("return TestCheckBoxTemplate.Text ~= nil").unwrap();
    assert!(
        has_text,
        "SetText should lazily create Text FontString child"
    );

    let text_type: String = env
        .eval("return TestCheckBoxTemplate.Text:GetObjectType()")
        .unwrap();
    assert_eq!(text_type, "FontString");
}

#[test]
fn test_checkbutton_with_label() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestCheckBoxWithLabel", UIParent)
        cb:SetSize(24, 24)
        cb:SetText("Enable Feature")
    "#,
    )
    .unwrap();

    let label_text: String = env.eval("return TestCheckBoxWithLabel:GetText()").unwrap();
    assert_eq!(label_text, "Enable Feature");
}

#[test]
fn test_checkbutton_template_no_orphaned_children() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestCbOrphans", UIParent)
        cb:SetSize(30, 29)
        cb:SetPoint("CENTER")

        cb:SetNormalTexture("Interface\\common\\minimalcheckbox")
        cb:SetPushedTexture("Interface\\common\\minimalcheckbox")
        cb:SetHighlightTexture("Interface\\common\\minimalcheckbox")
        cb:SetCheckedTexture("Interface\\common\\minimalcheckbox")
        cb:SetDisabledCheckedTexture("Interface\\common\\minimalcheckbox")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let registry = &state.widgets;

    let cb_id = registry.get_id_by_name("TestCbOrphans").unwrap();
    let cb = registry.get(cb_id).unwrap();
    let referenced_ids: std::collections::HashSet<u64> =
        cb.children_keys.values().copied().collect();

    let orphaned: Vec<_> = cb
        .children
        .iter()
        .copied()
        .filter(|child_id| !referenced_ids.contains(child_id))
        .filter_map(|child_id| {
            let child = registry.get(child_id).unwrap();
            let is_empty_child = child.anchors.is_empty()
                && child.texture.is_none()
                && child.text.is_none()
                && child.width == 0.0
                && child.height == 0.0;
            is_empty_child.then_some((child_id, child.widget_type))
        })
        .collect();

    assert!(
        orphaned.is_empty(),
        "CheckButton has {} orphaned children (0x0, no anchors, no content) \
         that will render as ghost elements centered in the parent: {:?}",
        orphaned.len(),
        orphaned
    );
}

#[test]
fn test_checkbutton_setatlas_propagates_to_parent() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestCbAtlasProp", UIParent)
        cb:SetSize(30, 29)
        cb:SetPoint("CENTER")

        local tex = cb:CreateTexture()
        cb:SetNormalTexture(tex)
        tex:SetAtlas("checkbox-minimal")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let registry = &state.widgets;

    let cb_id = registry.get_id_by_name("TestCbAtlasProp").unwrap();
    let cb = registry.get(cb_id).unwrap();

    assert!(
        cb.normal_texture.is_some(),
        "CheckButton's normal_texture should be set via SetAtlas propagation, got None"
    );
    assert!(
        cb.normal_texture
            .as_ref()
            .unwrap()
            .contains("minimalcheckbox"),
        "normal_texture should contain the atlas file path, got: {:?}",
        cb.normal_texture
    );

    let normal_tex_id = cb.children_keys.get("NormalTexture").unwrap();
    let normal_tex = registry.get(*normal_tex_id).unwrap();
    assert!(
        normal_tex.texture.is_some(),
        "NormalTexture child should have texture set via SetAtlas"
    );
    assert!(
        !normal_tex.anchors.is_empty(),
        "NormalTexture child should have anchors (fill-parent)"
    );
}

#[test]
fn test_checkbutton_text_from_global_string() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestCbGlobalStr", UIParent)
        cb:SetSize(24, 24)
        cb:SetText(ADDON_FORCE_LOAD)
    "#,
    )
    .unwrap();

    let label: String = env.eval("return TestCbGlobalStr:GetText()").unwrap();
    assert_eq!(label, "Load out of date AddOns");
}
