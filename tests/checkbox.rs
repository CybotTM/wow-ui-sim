#![cfg(feature = "gui")]
//! Tests for MinimalCheckboxTemplate and button mouse-enable defaults.

use crate::common;

use common::env_with_shared_xml;
use wow_ui_sim::iced_app::{RegistryQuadBatchParams, build_quad_batch_for_registry};
use wow_ui_sim::lua_api::WowLuaEnv;

#[cfg(feature = "client-wrath")]
mod checkbox_art {
    // INFERRED from cached profile FrameXML, not native 12340 runtime proof.
    pub const TEMPLATE: &str = "UICheckButtonTemplate";
    pub const SIZE: (f32, f32) = (32.0, 32.0);
    pub const PATH_FRAGMENT: &str = "ui-checkbox";

    pub fn assert_source(path: &str, _coords: Option<(f32, f32, f32, f32)>, file: &str) {
        assert_eq!(path, format!("Interface\\Buttons\\{file}"));
    }
}

#[cfg(not(feature = "client-wrath"))]
mod checkbox_art {
    pub const TEMPLATE: &str = "MinimalCheckboxTemplate";
    pub const SIZE: (f32, f32) = (30.0, 29.0);
    pub const PATH_FRAGMENT: &str = "minimalcheckbox";

    pub fn assert_source(path: &str, coords: Option<(f32, f32, f32, f32)>, _file: &str) {
        assert!(
            path.to_lowercase().contains(PATH_FRAGMENT),
            "atlas path: {path}"
        );
        assert!(
            coords.is_some(),
            "texture coordinates should be set from atlas"
        );
    }
}

fn create_checkbox(env: &WowLuaEnv, name: &str) {
    let template = checkbox_art::TEMPLATE;
    env.exec(&format!(
        r#"local cb = CreateFrame("CheckButton", {name:?}, UIParent, {template:?})
        cb:SetPoint("CENTER")"#
    ))
    .unwrap();
}

// ============================================================================
// MinimalCheckboxTemplate
// ============================================================================

#[test]
fn minimal_checkbox_size() {
    let env = env_with_shared_xml();

    create_checkbox(&env, "TestMinCheckbox");

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestMinCheckbox").unwrap();
    let frame = state.widgets.get(id).unwrap();

    assert_eq!(
        frame.width,
        checkbox_art::SIZE.0,
        "native checkbox template width"
    );
    assert_eq!(
        frame.height,
        checkbox_art::SIZE.1,
        "native checkbox template height"
    );
}

#[test]
fn minimal_checkbox_normal_texture_atlas() {
    let env = env_with_shared_xml();

    create_checkbox(&env, "TestMinCbNormal");

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestMinCbNormal").unwrap();
    let frame = state.widgets.get(id).unwrap();

    assert!(
        frame.normal_texture.is_some(),
        "normal_texture should be set from checkbox-minimal atlas"
    );
    let path = frame.normal_texture.as_ref().unwrap();
    checkbox_art::assert_source(path, frame.normal_tex_coords, "UI-CheckBox-Up");
}

#[test]
fn minimal_checkbox_pushed_texture_atlas() {
    let env = env_with_shared_xml();

    create_checkbox(&env, "TestMinCbPushed");

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestMinCbPushed").unwrap();
    let frame = state.widgets.get(id).unwrap();

    assert!(
        frame.pushed_texture.is_some(),
        "pushed_texture should be set from checkbox-minimal atlas"
    );
    checkbox_art::assert_source(
        frame.pushed_texture.as_ref().unwrap(),
        frame.pushed_tex_coords,
        "UI-CheckBox-Down",
    );
}

#[test]
fn minimal_checkbox_highlight_texture_atlas() {
    let env = env_with_shared_xml();

    create_checkbox(&env, "TestMinCbHighlight");

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestMinCbHighlight").unwrap();
    let frame = state.widgets.get(id).unwrap();

    assert!(
        frame.highlight_texture.is_some(),
        "highlight_texture should be set from checkbox-minimal atlas"
    );
    checkbox_art::assert_source(
        frame.highlight_texture.as_ref().unwrap(),
        frame.highlight_tex_coords,
        "UI-CheckBox-Highlight",
    );
}

#[test]
fn minimal_checkbox_checked_texture_atlas() {
    let env = env_with_shared_xml();

    create_checkbox(&env, "TestMinCbChecked");

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestMinCbChecked").unwrap();
    let frame = state.widgets.get(id).unwrap();

    assert!(
        frame.checked_texture.is_some(),
        "checked_texture should be set from checkmark-minimal atlas"
    );
    let path = frame.checked_texture.as_ref().unwrap();
    checkbox_art::assert_source(path, frame.checked_tex_coords, "UI-CheckBox-Check");
}

#[test]
fn minimal_checkbox_disabled_checked_texture_atlas() {
    let env = env_with_shared_xml();

    create_checkbox(&env, "TestMinCbDisChecked");

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestMinCbDisChecked").unwrap();
    let frame = state.widgets.get(id).unwrap();

    assert!(
        frame.disabled_checked_texture.is_some(),
        "disabled_checked_texture should be set from checkmark-minimal-disabled atlas"
    );
    let path = frame.disabled_checked_texture.as_ref().unwrap();
    checkbox_art::assert_source(
        path,
        frame.disabled_checked_tex_coords,
        "UI-CheckBox-Check-Disabled",
    );
}

#[test]
fn minimal_checkbox_set_checked_visibility() {
    let env = env_with_shared_xml();

    create_checkbox(&env, "TestMinCbVis");

    // CheckedTexture child should exist and start hidden
    {
        let state = env.state().borrow();
        let id = state.widgets.get_id_by_name("TestMinCbVis").unwrap();
        let frame = state.widgets.get(id).unwrap();
        let checked_tex_id = frame.children_keys.get("CheckedTexture");
        assert!(
            checked_tex_id.is_some(),
            "CheckedTexture child should exist after template application"
        );
        let tex = state.widgets.get(*checked_tex_id.unwrap()).unwrap();
        assert!(!tex.visible, "CheckedTexture should start hidden");
    }

    // SetChecked(true) should show it
    env.exec("TestMinCbVis:SetChecked(true)").unwrap();
    {
        let state = env.state().borrow();
        let id = state.widgets.get_id_by_name("TestMinCbVis").unwrap();
        let frame = state.widgets.get(id).unwrap();
        let checked_tex_id = *frame.children_keys.get("CheckedTexture").unwrap();
        let tex = state.widgets.get(checked_tex_id).unwrap();
        assert!(
            tex.visible,
            "CheckedTexture should be visible after SetChecked(true)"
        );
    }

    // SetChecked(false) should hide it
    env.exec("TestMinCbVis:SetChecked(false)").unwrap();
    {
        let state = env.state().borrow();
        let id = state.widgets.get_id_by_name("TestMinCbVis").unwrap();
        let frame = state.widgets.get(id).unwrap();
        let checked_tex_id = *frame.children_keys.get("CheckedTexture").unwrap();
        let tex = state.widgets.get(checked_tex_id).unwrap();
        assert!(
            !tex.visible,
            "CheckedTexture should be hidden after SetChecked(false)"
        );
    }
}

#[test]
fn minimal_checkbox_quad_batch() {
    let env = env_with_shared_xml();

    create_checkbox(&env, "TestMinCbQuad");
    env.exec("TestMinCbQuad:Show()").unwrap();

    let buckets = {
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    let batch = build_quad_batch_for_registry(
        RegistryQuadBatchParams::new(&state.widgets, (1024.0, 768.0), &buckets)
            .root_name(Some("TestMinCbQuad")),
    );

    // Should have texture requests referencing the checkbox atlas path
    let checkbox_requests: Vec<_> = batch
        .texture_requests
        .iter()
        .filter(|r| r.path.to_lowercase().contains(checkbox_art::PATH_FRAGMENT))
        .collect();

    assert!(
        !checkbox_requests.is_empty(),
        "Quad batch should contain texture requests for minimalcheckbox atlas. Got paths: {:?}",
        batch
            .texture_requests
            .iter()
            .map(|r| &r.path)
            .collect::<Vec<_>>()
    );
}

#[test]
fn minimal_checkbox_click_toggles_checked() {
    let env = env_with_shared_xml();

    // WoW toggles CheckButtons before dispatching OnClick, so the handler observes
    // the new checked state.
    create_checkbox(&env, "TestMinCbClick");
    env.exec(
        r#"
        TestMinCbClickObserved = {}
        local cb = TestMinCbClick
        cb:SetScript("OnClick", function(self)
            table.insert(TestMinCbClickObserved, self:GetChecked())
        end)
    "#,
    )
    .unwrap();

    // Starts unchecked
    let checked: bool = env.eval("return TestMinCbClick:GetChecked()").unwrap();
    assert!(!checked, "Should start unchecked");

    // Click() toggles to checked before firing OnClick.
    env.exec("TestMinCbClick:Click()").unwrap();
    let (checked, observed): (bool, bool) = env
        .eval("return TestMinCbClick:GetChecked(), TestMinCbClickObserved[1]")
        .unwrap();
    assert!(checked, "Should be checked after first Click()");
    assert!(observed, "OnClick should observe the checked state");

    // CheckedTexture should now be visible
    assert_checked_texture_visible(&env, "TestMinCbClick", true);

    // Click() again toggles to unchecked before firing OnClick.
    env.exec("TestMinCbClick:Click()").unwrap();
    let (checked, observed): (bool, bool) = env
        .eval("return TestMinCbClick:GetChecked(), TestMinCbClickObserved[2]")
        .unwrap();
    assert!(!checked, "Should be unchecked after second Click()");
    assert!(!observed, "OnClick should observe the unchecked state");

    // CheckedTexture should be hidden again
    assert_checked_texture_visible(&env, "TestMinCbClick", false);
}

fn assert_checked_texture_visible(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    name: &str,
    expected: bool,
) {
    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(name).unwrap();
    let frame = state.widgets.get(id).unwrap();
    let tex_id = *frame.children_keys.get("CheckedTexture").unwrap();
    let tex = state.widgets.get(tex_id).unwrap();
    assert_eq!(
        tex.visible,
        expected,
        "CheckedTexture should be {} for {}",
        if expected { "visible" } else { "hidden" },
        name,
    );
}

#[test]
fn minimal_checkbox_mouse_enabled_by_default() {
    let env = env_with_shared_xml();

    create_checkbox(&env, "TestMinCbMouse");

    // CheckButton should have mouse enabled by default (like WoW)
    let mouse_enabled: bool = env.eval("return TestMinCbMouse:IsMouseEnabled()").unwrap();
    assert!(
        mouse_enabled,
        "CheckButton should have mouse enabled by default"
    );

    // Also verify via Rust state
    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestMinCbMouse").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(
        frame.mouse_enabled,
        "CheckButton frame.mouse_enabled should be true"
    );
}

#[test]
fn button_mouse_enabled_by_default() {
    let env = wow_ui_sim::lua_api::WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestBtnMouse", UIParent)
        btn:SetPoint("CENTER")
        btn:SetSize(100, 30)
    "#,
    )
    .unwrap();

    let mouse_enabled: bool = env.eval("return TestBtnMouse:IsMouseEnabled()").unwrap();
    assert!(mouse_enabled, "Button should have mouse enabled by default");
}
