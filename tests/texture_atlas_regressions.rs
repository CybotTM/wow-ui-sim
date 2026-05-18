//! Focused regression tests for two texture-path regressions introduced during
//! the mlua→rilua migration.
//!
//! Regression 1: `SetTexCoord` ignores `atlas_tex_coords` — should remap UVs
//!   into the atlas slot coordinate space when an atlas is active.
//!
//! Regression 2: `SetAtlas` on child textures with standard parentKeys
//!   (`NormalTexture`, `PushedTexture`, `HighlightTexture`, `DisabledTexture`)
//!   should propagate atlas data to the parent Button's corresponding slot fields.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// Regression 1: SetTexCoord atlas remapping
// ============================================================================

/// Atlas-attached texture: SetTexCoord(0,1,0,1) with atlas region (0.25,0.75,0.1,0.9)
/// should store the atlas coords as tex_coords, not the raw (0,1,0,1).
#[test]
fn set_tex_coord_remaps_against_atlas_tex_coords() {
    let env = env();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "AtlasRemapFrame", UIParent)
        _G.AtlasRemapTex = frame:CreateTexture("AtlasRemapTex", "BACKGROUND")
    "#,
    )
    .unwrap();

    // Inject atlas_tex_coords via Rust: region is (0.25, 0.75, 0.1, 0.9)
    {
        let mut state = env.state().borrow_mut();
        let id = state.widgets.get_id_by_name("AtlasRemapTex").unwrap();
        let widget = state.widgets.get_mut(id).unwrap();
        widget.atlas_tex_coords = Some((0.25, 0.75, 0.1, 0.9));
    }

    // SetTexCoord(0, 1, 0, 1) should remap to the full atlas region
    env.exec("AtlasRemapTex:SetTexCoord(0, 1, 0, 1)").unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("AtlasRemapTex").unwrap();
    let coords = state.widgets.get(id).unwrap().tex_coords.unwrap();

    // Expected: (0.25 + 0*0.5, 0.25 + 1*0.5, 0.1 + 0*0.8, 0.1 + 1*0.8) = (0.25, 0.75, 0.1, 0.9)
    assert!(
        (coords.0 - 0.25).abs() < 0.001,
        "left should be remapped to atlas_left=0.25, got {}",
        coords.0
    );
    assert!(
        (coords.1 - 0.75).abs() < 0.001,
        "right should be remapped to atlas_right=0.75, got {}",
        coords.1
    );
    assert!(
        (coords.2 - 0.1).abs() < 0.001,
        "top should be remapped to atlas_top=0.1, got {}",
        coords.2
    );
    assert!(
        (coords.3 - 0.9).abs() < 0.001,
        "bottom should be remapped to atlas_bottom=0.9, got {}",
        coords.3
    );
}

/// No-atlas texture: SetTexCoord args pass through unchanged.
#[test]
fn set_tex_coord_without_atlas_passes_through() {
    let env = env();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "NoAtlasRemapFrame", UIParent)
        _G.NoAtlasRemapTex = frame:CreateTexture("NoAtlasRemapTex", "BACKGROUND")
        NoAtlasRemapTex:SetTexCoord(0.1, 0.9, 0.2, 0.8)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("NoAtlasRemapTex").unwrap();
    let coords = state.widgets.get(id).unwrap().tex_coords.unwrap();

    assert!(
        (coords.0 - 0.1).abs() < 0.001,
        "left unchanged: {}",
        coords.0
    );
    assert!(
        (coords.1 - 0.9).abs() < 0.001,
        "right unchanged: {}",
        coords.1
    );
    assert!(
        (coords.2 - 0.2).abs() < 0.001,
        "top unchanged: {}",
        coords.2
    );
    assert!(
        (coords.3 - 0.8).abs() < 0.001,
        "bottom unchanged: {}",
        coords.3
    );
}

/// 8-arg quad form with atlas active: the bounding-box tex_coords should be
/// remapped into the atlas region; the raw quad vertices are preserved as-is.
#[test]
fn set_tex_coord_8_arg_quad_form_remaps() {
    let env = env();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "QuadAtlasFrame", UIParent)
        _G.QuadAtlasTex = frame:CreateTexture("QuadAtlasTex", "BACKGROUND")
    "#,
    )
    .unwrap();

    // Atlas region: left=0.0, right=0.5, top=0.0, bottom=0.5 (top-left quadrant)
    {
        let mut state = env.state().borrow_mut();
        let id = state.widgets.get_id_by_name("QuadAtlasTex").unwrap();
        let widget = state.widgets.get_mut(id).unwrap();
        widget.atlas_tex_coords = Some((0.0, 0.5, 0.0, 0.5));
    }

    // 8-arg: UL(0,0) LL(0,1) UR(1,0) LR(1,1) — full unit quad
    env.exec("QuadAtlasTex:SetTexCoord(0,0, 0,1, 1,0, 1,1)")
        .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("QuadAtlasTex").unwrap();
    let widget = state.widgets.get(id).unwrap();

    // Bounding box: left=min(0,0,1,1)=0, right=max=1, top=0, bottom=1
    // Remapped into atlas (0, 0.5, 0, 0.5):
    //   left  = 0.0 + 0*0.5 = 0.0
    //   right = 0.0 + 1*0.5 = 0.5
    //   top   = 0.0 + 0*0.5 = 0.0
    //   bottom= 0.0 + 1*0.5 = 0.5
    let coords = widget.tex_coords.unwrap();
    assert!(
        (coords.0 - 0.0).abs() < 0.001,
        "remapped left should be 0.0, got {}",
        coords.0
    );
    assert!(
        (coords.1 - 0.5).abs() < 0.001,
        "remapped right should be 0.5, got {}",
        coords.1
    );
    assert!(
        (coords.2 - 0.0).abs() < 0.001,
        "remapped top should be 0.0, got {}",
        coords.2
    );
    assert!(
        (coords.3 - 0.5).abs() < 0.001,
        "remapped bottom should be 0.5, got {}",
        coords.3
    );
}

// ============================================================================
// Regression 2: SetAtlas propagates to parent button slots
// ============================================================================

/// Child texture with parentKey "NormalTexture" calling SetAtlas should
/// propagate atlas data to parent button's normal_texture / normal_tex_coords.
#[test]
fn set_atlas_propagates_to_parent_button_normal_texture() {
    let env = env();
    env.exec(
        r#"
        local btn = CreateFrame("Button", "PropNormalBtn", UIParent)
        btn:SetSize(32, 32)
        local tex = btn:CreateTexture()
        btn:SetNormalTexture(tex)
        tex:SetAtlas("checkbox-minimal")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("PropNormalBtn").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert!(
        btn.normal_texture.is_some(),
        "SetAtlas on NormalTexture child should propagate to parent.normal_texture"
    );
    assert!(
        btn.normal_tex_coords.is_some(),
        "SetAtlas on NormalTexture child should populate parent.normal_tex_coords"
    );
}

#[test]
fn empty_set_atlas_clears_texture_instead_of_using_empty_atlas_db_entry() {
    let env = env();
    env.exec(
        r#"
        local tex = UIParent:CreateTexture("EmptyAtlasClearsTexture", "BACKGROUND")
        tex:SetTexture("Interface\\Buttons\\WHITE8X8")
        tex:SetAtlas("")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let tex_id = state
        .widgets
        .get_id_by_name("EmptyAtlasClearsTexture")
        .unwrap();
    let tex = state.widgets.get(tex_id).unwrap();
    assert_eq!(
        tex.texture, None,
        "SetAtlas(\"\") should clear texture path"
    );
    assert_eq!(tex.atlas, None, "SetAtlas(\"\") should clear atlas name");
}

#[test]
fn empty_set_atlas_clears_parent_button_slot_texture() {
    let env = env();
    env.exec(
        r#"
        local btn = CreateFrame("Button", "EmptyAtlasClearsButtonSlot", UIParent)
        btn:SetSize(32, 32)
        local tex = btn:CreateTexture(nil, "BACKGROUND", nil, nil, "NormalTexture")
        btn:SetNormalTexture(tex)
        tex:SetAtlas("checkbox-minimal")
        tex:SetAtlas("")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state
        .widgets
        .get_id_by_name("EmptyAtlasClearsButtonSlot")
        .unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert_eq!(
        btn.normal_texture, None,
        "SetAtlas(\"\") on NormalTexture child should clear parent normal_texture"
    );
    assert_eq!(
        btn.normal_tex_coords, None,
        "SetAtlas(\"\") on NormalTexture child should clear parent normal_tex_coords"
    );
}

/// Child texture with parentKey "PushedTexture" calling SetAtlas should
/// propagate to parent button's pushed_texture / pushed_tex_coords.
#[test]
fn set_atlas_propagates_to_parent_button_pushed_texture() {
    let env = env();
    env.exec(
        r#"
        local btn = CreateFrame("Button", "PropPushedBtn", UIParent)
        btn:SetSize(32, 32)
        local tex = btn:CreateTexture()
        btn:SetPushedTexture(tex)
        tex:SetAtlas("checkbox-minimal")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("PropPushedBtn").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert!(
        btn.pushed_texture.is_some(),
        "SetAtlas on PushedTexture child should propagate to parent.pushed_texture"
    );
    assert!(
        btn.pushed_tex_coords.is_some(),
        "SetAtlas on PushedTexture child should populate parent.pushed_tex_coords"
    );
}

/// Child texture with parentKey "HighlightTexture" calling SetAtlas should
/// propagate to parent button's highlight_texture / highlight_tex_coords.
#[test]
fn set_atlas_propagates_to_parent_button_highlight_texture() {
    let env = env();
    env.exec(
        r#"
        local btn = CreateFrame("Button", "PropHighlightBtn", UIParent)
        btn:SetSize(32, 32)
        local tex = btn:CreateTexture()
        btn:SetHighlightTexture(tex)
        tex:SetAtlas("checkbox-minimal")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("PropHighlightBtn").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert!(
        btn.highlight_texture.is_some(),
        "SetAtlas on HighlightTexture child should propagate to parent.highlight_texture"
    );
    assert!(
        btn.highlight_tex_coords.is_some(),
        "SetAtlas on HighlightTexture child should populate parent.highlight_tex_coords"
    );
}

/// Child texture with parentKey "DisabledTexture" calling SetAtlas should
/// propagate to parent button's disabled_texture / disabled_tex_coords.
#[test]
fn set_atlas_propagates_to_parent_button_disabled_texture() {
    let env = env();
    env.exec(
        r#"
        local btn = CreateFrame("Button", "PropDisabledBtn", UIParent)
        btn:SetSize(32, 32)
        local tex = btn:CreateTexture()
        btn:SetDisabledTexture(tex)
        tex:SetAtlas("checkbox-minimal")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("PropDisabledBtn").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert!(
        btn.disabled_texture.is_some(),
        "SetAtlas on DisabledTexture child should propagate to parent.disabled_texture"
    );
    assert!(
        btn.disabled_tex_coords.is_some(),
        "SetAtlas on DisabledTexture child should populate parent.disabled_tex_coords"
    );
}

/// Child texture with a custom parentKey should NOT propagate to any button slot.
#[test]
fn set_atlas_on_custom_parent_key_does_not_propagate() {
    let env = env();
    env.exec(
        r#"
        local btn = CreateFrame("Button", "PropCustomBtn", UIParent)
        btn:SetSize(32, 32)
        local tex = btn:CreateTexture()
        tex:SetParentKey("MyCustomThing")
        tex:SetAtlas("checkbox-minimal")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("PropCustomBtn").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert!(
        btn.normal_texture.is_none(),
        "custom parentKey SetAtlas must NOT touch parent.normal_texture"
    );
    assert!(
        btn.pushed_texture.is_none(),
        "custom parentKey SetAtlas must NOT touch parent.pushed_texture"
    );
    assert!(
        btn.highlight_texture.is_none(),
        "custom parentKey SetAtlas must NOT touch parent.highlight_texture"
    );
    assert!(
        btn.disabled_texture.is_none(),
        "custom parentKey SetAtlas must NOT touch parent.disabled_texture"
    );
}
