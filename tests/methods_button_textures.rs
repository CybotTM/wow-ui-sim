//! Texture-focused tests for button methods.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_get_normal_texture_nil_on_fresh_button() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"local btn = CreateFrame("Button", "TestGetNormTex", UIParent)"#)
        .unwrap();

    let is_nil: bool = env
        .eval("return TestGetNormTex:GetNormalTexture() == nil")
        .unwrap();
    assert!(is_nil, "Fresh button GetNormalTexture should return nil");
}

#[test]
fn test_get_normal_texture_after_set() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestGetNormTex2", UIParent)
        btn:SetNormalTexture("Interface\\Buttons\\UI-Panel-Button-Up")
    "#,
    )
    .unwrap();

    let obj_type: String = env
        .eval("return TestGetNormTex2:GetNormalTexture():GetObjectType()")
        .unwrap();
    assert_eq!(
        obj_type, "Texture",
        "GetNormalTexture should return a Texture after Set"
    );
}

#[test]
fn test_get_texture_returns_child_of_button() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestTexChild", UIParent)
        btn:SetNormalTexture("Interface\\Buttons\\UI-Panel-Button-Up")
    "#,
    )
    .unwrap();

    let parent_name: String = env
        .eval("return TestTexChild:GetNormalTexture():GetParent():GetName()")
        .unwrap();
    assert_eq!(
        parent_name, "TestTexChild",
        "Texture child should have button as parent"
    );
}

#[test]
fn test_set_normal_texture_with_path() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestSetNormTex", UIParent)
        btn:SetNormalTexture("Interface\\Buttons\\UI-Panel-Button-Up")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestSetNormTex").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    assert!(
        btn.normal_texture.is_some(),
        "Button normal_texture should be set after SetNormalTexture with path"
    );
    assert!(
        btn.normal_texture
            .as_ref()
            .unwrap()
            .contains("UI-Panel-Button-Up"),
        "normal_texture should contain the texture path"
    );
}

#[test]
fn test_set_pushed_texture_with_path() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestSetPushTex", UIParent)
        btn:SetPushedTexture("Interface\\Buttons\\UI-Panel-Button-Down")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestSetPushTex").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    assert!(
        btn.pushed_texture.is_some(),
        "Button pushed_texture should be set after SetPushedTexture with path"
    );
}

#[test]
fn test_set_highlight_texture_with_path() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestSetHlTex", UIParent)
        btn:SetHighlightTexture("Interface\\Buttons\\UI-Panel-Button-Highlight")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestSetHlTex").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    assert!(
        btn.highlight_texture.is_some(),
        "Button highlight_texture should be set after SetHighlightTexture with path"
    );
}

#[test]
fn test_set_disabled_texture_with_path() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestSetDisTex", UIParent)
        btn:SetDisabledTexture("Interface\\Buttons\\UI-Panel-Button-Disabled")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestSetDisTex").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    assert!(
        btn.disabled_texture.is_some(),
        "Button disabled_texture should be set after SetDisabledTexture with path"
    );
}

#[test]
fn test_set_texture_with_userdata_does_not_overwrite_path() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestSetTexUD", UIParent)
        btn:SetNormalTexture("Interface\\Buttons\\UI-Panel-Button-Up")
        local tex = btn:GetNormalTexture()
        btn:SetNormalTexture(tex)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestSetTexUD").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    assert!(
        btn.normal_texture.is_some(),
        "normal_texture should still be set after SetNormalTexture with userdata"
    );
    assert!(
        btn.normal_texture
            .as_ref()
            .unwrap()
            .contains("UI-Panel-Button-Up"),
        "normal_texture should still contain original path after userdata set"
    );
}

#[test]
fn test_set_checked_texture() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestSetChkTex", UIParent)
        cb:SetCheckedTexture("Interface\\Buttons\\CheckButtonCheck")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let cb_id = state.widgets.get_id_by_name("TestSetChkTex").unwrap();
    let cb = state.widgets.get(cb_id).unwrap();

    assert!(
        cb.checked_texture.is_some(),
        "checked_texture should be set after SetCheckedTexture"
    );

    let tex_id = cb.children_keys.get("CheckedTexture").unwrap();
    let tex = state.widgets.get(*tex_id).unwrap();
    assert!(!tex.visible, "CheckedTexture child should start hidden");
}

#[test]
fn test_set_disabled_checked_texture() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestSetDisChkTex", UIParent)
        cb:SetDisabledCheckedTexture("Interface\\Buttons\\CheckButtonCheckDisabled")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let cb_id = state.widgets.get_id_by_name("TestSetDisChkTex").unwrap();
    let cb = state.widgets.get(cb_id).unwrap();

    assert!(
        cb.disabled_checked_texture.is_some(),
        "disabled_checked_texture should be set after SetDisabledCheckedTexture"
    );

    let tex_id = cb.children_keys.get("DisabledCheckedTexture").unwrap();
    let tex = state.widgets.get(*tex_id).unwrap();
    assert!(
        !tex.visible,
        "DisabledCheckedTexture child should start hidden"
    );
}

#[test]
fn test_get_disabled_checked_texture_nil_when_unset() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestGetDisChkTexNil", UIParent)
    "#,
    )
    .unwrap();

    let is_nil: bool = env
        .eval("return TestGetDisChkTexNil:GetDisabledCheckedTexture() == nil")
        .unwrap();
    assert!(is_nil, "Fresh checkbutton getter should return nil");
}

#[test]
fn test_get_disabled_checked_texture_returns_child_texture() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cb = CreateFrame("CheckButton", "TestGetDisChkTex", UIParent)
        cb:SetDisabledCheckedTexture("Interface\\Buttons\\CheckButtonCheckDisabled")
    "#,
    )
    .unwrap();

    let obj_type: String = env
        .eval("return TestGetDisChkTex:GetDisabledCheckedTexture():GetObjectType()")
        .unwrap();
    assert_eq!(
        obj_type, "Texture",
        "Getter should return the disabled checked texture child"
    );

    let parent_name: String = env
        .eval("return TestGetDisChkTex:GetDisabledCheckedTexture():GetParent():GetName()")
        .unwrap();
    assert_eq!(
        parent_name, "TestGetDisChkTex",
        "Disabled checked texture child should stay parented to the checkbutton"
    );
}

#[test]
fn test_set_left_texture() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestLeftTex", UIParent)
        btn:SetLeftTexture("Interface\\Buttons\\Left")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestLeftTex").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    assert!(btn.left_texture.is_some(), "left_texture should be set");
    assert!(btn.left_texture.as_ref().unwrap().contains("Left"));
}

#[test]
fn test_set_middle_texture() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestMidTex", UIParent)
        btn:SetMiddleTexture("Interface\\Buttons\\Middle")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestMidTex").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    assert!(btn.middle_texture.is_some(), "middle_texture should be set");
}

#[test]
fn test_set_right_texture() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestRightTex", UIParent)
        btn:SetRightTexture("Interface\\Buttons\\Right")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestRightTex").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    assert!(btn.right_texture.is_some(), "right_texture should be set");
}

#[test]
fn test_set_three_slice_nil_clears() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestSliceNil", UIParent)
        btn:SetLeftTexture("Interface\\Buttons\\Left")
        btn:SetLeftTexture(nil)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestSliceNil").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    assert!(
        btn.left_texture.is_none(),
        "left_texture should be nil after setting nil"
    );
}

#[test]
fn test_set_highlight_atlas_creates_texture() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestHlAtlas", UIParent)
        btn:SetHighlightAtlas("checkbox-minimal")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestHlAtlas").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert!(
        btn.children_keys.contains_key("HighlightTexture"),
        "SetHighlightAtlas should create HighlightTexture child"
    );
}

#[test]
fn test_texture_children_have_fill_parent_anchors() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestTexAnchors", UIParent)
        btn:SetSize(100, 30)
        btn:SetNormalTexture("Interface\\Buttons\\UI-Panel-Button-Up")
        btn:SetPushedTexture("Interface\\Buttons\\UI-Panel-Button-Down")
        btn:SetHighlightTexture("Interface\\Buttons\\UI-Panel-Button-Highlight")
        btn:SetDisabledTexture("Interface\\Buttons\\UI-Panel-Button-Disabled")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestTexAnchors").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();

    for key in &[
        "NormalTexture",
        "PushedTexture",
        "HighlightTexture",
        "DisabledTexture",
    ] {
        let tex_id = btn
            .children_keys
            .get(*key)
            .unwrap_or_else(|| panic!("{} should exist", key));
        let tex = state.widgets.get(*tex_id).unwrap();
        assert!(
            !tex.anchors.is_empty(),
            "{} should have fill-parent anchors",
            key
        );
    }
}

#[test]
fn test_set_normal_texture_same_calendar_atlas_does_not_mark_render_dirty() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestRepeatCalendarAtlas", UIParent)
        btn:SetNormalTexture("ui-hud-calendar-1-up")
    "#,
    )
    .unwrap();

    let (btn_id, tex_id) = {
        let state = env.state().borrow();
        let btn_id = state
            .widgets
            .get_id_by_name("TestRepeatCalendarAtlas")
            .unwrap();
        let tex_id = state.widgets.get(btn_id).unwrap().children_keys["NormalTexture"];
        (btn_id, tex_id)
    };

    let _ = env.state().borrow().widgets.take_render_dirty_with_ids();

    env.exec(r#"TestRepeatCalendarAtlas:SetNormalTexture("ui-hud-calendar-1-up")"#)
        .unwrap();

    let (dirty_mask, dirty_ids) = env.state().borrow().widgets.take_render_dirty_with_ids();
    let dirty_ids = dirty_ids.unwrap_or_default();

    assert_eq!(
        dirty_mask, 0,
        "repeating the same atlas-backed SetNormalTexture should not dirty rendering"
    );
    assert!(
        !dirty_ids.contains(&btn_id) && !dirty_ids.contains(&tex_id),
        "repeat SetNormalTexture should not mark button or texture child dirty (got {:?})",
        dirty_ids
    );
}
