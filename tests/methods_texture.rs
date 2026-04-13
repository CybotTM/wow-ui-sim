//! Tests for texture-related Lua methods (methods_texture.rs).
//!
//! Covers: SetTexture, GetTexture, SetTexCoord, SetVertexColor, GetVertexColor,
//! SetColorTexture, SetAtlas, GetAtlas, SetBlendMode, GetBlendMode,
//! SetHorizTile, GetHorizTile, SetVertTile, GetVertTile, SetDrawLayer, GetDrawLayer,
//! SetDesaturated, IsDesaturated, mask textures, pixel grid, texel snapping,
//! and nine-slice stub methods.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

/// Create a frame with a child texture. Returns `(frame_global, texture_global)` names.
fn setup_texture(env: &WowLuaEnv, prefix: &str) -> (String, String) {
    let frame_name = format!("{}Frame", prefix);
    let tex_name = format!("{}Tex", prefix);
    env.exec(&format!(
        r#"
        local f = CreateFrame("Frame", "{frame_name}", UIParent)
        f:CreateTexture("{tex_name}", "BACKGROUND")
    "#
    ))
    .unwrap();
    (frame_name, tex_name)
}

// ============================================================================
// SetTexture / GetTexture
// ============================================================================

#[test]
fn test_set_get_texture_path() {
    let env = env();
    let (_, tex) = setup_texture(&env, "TexPath");
    env.exec(&format!(
        r#"{tex}:SetTexture("Interface\\Buttons\\UI-Panel-Button-Up")"#
    ))
    .unwrap();
    let path: String = env.eval(&format!("return {tex}:GetTexture()")).unwrap();
    assert_eq!(path, "Interface\\Buttons\\UI-Panel-Button-Up");
}

#[test]
fn test_set_texture_nil_clears() {
    let env = env();
    let (_, tex) = setup_texture(&env, "TexNil");
    env.exec(&format!(
        r#"{tex}:SetTexture("Interface\\Buttons\\UI-Panel-Button-Up"); {tex}:SetTexture(nil)"#
    ))
    .unwrap();
    let is_nil: bool = env
        .eval(&format!("return {tex}:GetTexture() == nil"))
        .unwrap();
    assert!(is_nil, "SetTexture(nil) should clear the texture path");
}

#[test]
fn test_get_texture_default_nil() {
    let env = env();
    let (_, tex) = setup_texture(&env, "TexDef");
    let is_nil: bool = env
        .eval(&format!("return {tex}:GetTexture() == nil"))
        .unwrap();
    assert!(is_nil, "Texture path should be nil by default");
}

// ============================================================================
// SetVertexColor / GetVertexColor
// ============================================================================

#[test]
fn test_set_get_vertex_color() {
    let env = env();
    let (_, tex) = setup_texture(&env, "VC");
    let (r, g, b, a): (f64, f64, f64, f64) = env
        .eval(&format!(
            "{tex}:SetVertexColor(0.5, 0.6, 0.7, 0.8); return {tex}:GetVertexColor()"
        ))
        .unwrap();
    assert!((r - 0.5).abs() < 0.001);
    assert!((g - 0.6).abs() < 0.001);
    assert!((b - 0.7).abs() < 0.001);
    assert!((a - 0.8).abs() < 0.001);
}

#[test]
fn test_vertex_color_default_alpha() {
    let env = env();
    let (_, tex) = setup_texture(&env, "VCDef");
    let (r, g, b, a): (f64, f64, f64, f64) = env
        .eval(&format!(
            "{tex}:SetVertexColor(0.1, 0.2, 0.3); return {tex}:GetVertexColor()"
        ))
        .unwrap();
    assert!((r - 0.1).abs() < 0.001);
    assert!((g - 0.2).abs() < 0.001);
    assert!((b - 0.3).abs() < 0.001);
    assert!((a - 1.0).abs() < 0.001, "Alpha should default to 1.0");
}

#[test]
fn test_vertex_color_default_white() {
    let env = env();
    let (_, tex) = setup_texture(&env, "VCWhite");
    let (r, g, b, a): (f64, f64, f64, f64) =
        env.eval(&format!("return {tex}:GetVertexColor()")).unwrap();
    assert_eq!((r, g, b, a), (1.0, 1.0, 1.0, 1.0));
}

// ============================================================================
// SetColorTexture
// ============================================================================

#[test]
fn test_set_color_texture() {
    let env = env();
    let (_, tex) = setup_texture(&env, "CT");
    env.exec(&format!(
        r#"{tex}:SetTexture("Interface\\Buttons\\something"); {tex}:SetColorTexture(1, 0, 0, 0.5)"#
    ))
    .unwrap();

    let is_nil: bool = env
        .eval(&format!("return {tex}:GetTexture() == nil"))
        .unwrap();
    assert!(is_nil, "GetTexture should return nil after SetColorTexture");

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(&tex).unwrap();
    let color = state.widgets.get(id).unwrap().color_texture.unwrap();
    assert!((color.r - 1.0).abs() < 0.001);
    assert!((color.g - 0.0).abs() < 0.001);
    assert!((color.b - 0.0).abs() < 0.001);
    assert!((color.a - 0.5).abs() < 0.001);
}

#[test]
fn test_set_color_texture_default_alpha() {
    let env = env();
    let (_, tex) = setup_texture(&env, "CTDef");
    env.exec(&format!("{tex}:SetColorTexture(0.2, 0.3, 0.4)"))
        .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(&tex).unwrap();
    let color = state.widgets.get(id).unwrap().color_texture.unwrap();
    assert!((color.a - 1.0).abs() < 0.001, "Alpha should default to 1.0");
}

// ============================================================================
// SetTexCoord / atlas-relative tex coords
// ============================================================================

#[test]
fn test_set_tex_coord_basic() {
    let env = env();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "TCFrame", UIParent)
        local tex = frame:CreateTexture("TCTex", "BACKGROUND")
        tex:SetTexCoord(0.1, 0.9, 0.2, 0.8)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TCTex").unwrap();
    let widget = state.widgets.get(id).unwrap();
    let coords = widget.tex_coords.unwrap();
    assert!((coords.0 - 0.1).abs() < 0.001);
    assert!((coords.1 - 0.9).abs() < 0.001);
    assert!((coords.2 - 0.2).abs() < 0.001);
    assert!((coords.3 - 0.8).abs() < 0.001);
}

#[test]
fn test_set_tex_coord_with_atlas_remaps() {
    let env = env();
    // Manually set up atlas_tex_coords via Rust, then call SetTexCoord
    // The atlas sub-region is (0.25, 0.75, 0.1, 0.9)
    // SetTexCoord(0, 1, 0, 1) should produce the atlas coords themselves
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "TCAtlasFrame", UIParent)
        local tex = frame:CreateTexture("TCAtlasTex", "BACKGROUND")
    "#,
    )
    .unwrap();

    // Set atlas_tex_coords directly in Rust state
    {
        let mut state = env.state().borrow_mut();
        let id = state.widgets.get_id_by_name("TCAtlasTex").unwrap();
        let widget = state.widgets.get_mut(id).unwrap();
        widget.atlas_tex_coords = Some((0.25, 0.75, 0.1, 0.9));
    }

    // Now call SetTexCoord(0, 1, 0, 1) - should remap to atlas sub-region
    env.exec("TCAtlasTex:SetTexCoord(0, 1, 0, 1)").unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TCAtlasTex").unwrap();
    let widget = state.widgets.get(id).unwrap();
    let coords = widget.tex_coords.unwrap();
    // 0.25 + 0 * 0.5 = 0.25, 0.25 + 1 * 0.5 = 0.75
    // 0.1 + 0 * 0.8 = 0.1, 0.1 + 1 * 0.8 = 0.9
    assert!((coords.0 - 0.25).abs() < 0.001);
    assert!((coords.1 - 0.75).abs() < 0.001);
    assert!((coords.2 - 0.1).abs() < 0.001);
    assert!((coords.3 - 0.9).abs() < 0.001);
}

#[test]
fn test_set_tex_coord_with_atlas_partial() {
    let env = env();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "TCPartialFrame", UIParent)
        local tex = frame:CreateTexture("TCPartialTex", "BACKGROUND")
    "#,
    )
    .unwrap();

    // Atlas region: left=0.0, right=1.0, top=0.0, bottom=1.0
    {
        let mut state = env.state().borrow_mut();
        let id = state.widgets.get_id_by_name("TCPartialTex").unwrap();
        let widget = state.widgets.get_mut(id).unwrap();
        widget.atlas_tex_coords = Some((0.0, 1.0, 0.0, 1.0));
    }

    // SetTexCoord(0.5, 1.0, 0.5, 1.0) - should produce (0.5, 1.0, 0.5, 1.0)
    env.exec("TCPartialTex:SetTexCoord(0.5, 1.0, 0.5, 1.0)")
        .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TCPartialTex").unwrap();
    let widget = state.widgets.get(id).unwrap();
    let coords = widget.tex_coords.unwrap();
    assert!((coords.0 - 0.5).abs() < 0.001);
    assert!((coords.1 - 1.0).abs() < 0.001);
    assert!((coords.2 - 0.5).abs() < 0.001);
    assert!((coords.3 - 1.0).abs() < 0.001);
}

// ============================================================================
// SetHorizTile / GetHorizTile / SetVertTile / GetVertTile
// ============================================================================

#[test]
fn test_horiz_tile() {
    let env = env();
    let (_, tex) = setup_texture(&env, "HT");
    let (before, after): (bool, bool) = env
        .eval(&format!("local b = {tex}:GetHorizTile(); {tex}:SetHorizTile(true); return b, {tex}:GetHorizTile()"))
        .unwrap();
    assert!(!before, "HorizTile should default to false");
    assert!(after, "HorizTile should be true after SetHorizTile(true)");
}

#[test]
fn test_vert_tile() {
    let env = env();
    let (_, tex) = setup_texture(&env, "VT");
    let (before, after): (bool, bool) = env
        .eval(&format!(
            "local b = {tex}:GetVertTile(); {tex}:SetVertTile(true); return b, {tex}:GetVertTile()"
        ))
        .unwrap();
    assert!(!before, "VertTile should default to false");
    assert!(after, "VertTile should be true after SetVertTile(true)");
}

// ============================================================================
// SetBlendMode / GetBlendMode
// ============================================================================

#[test]
fn test_blend_mode_default() {
    let env = env();
    let (_, tex) = setup_texture(&env, "BM");
    let mode: String = env.eval(&format!("return {tex}:GetBlendMode()")).unwrap();
    assert_eq!(mode, "BLEND");
}

#[test]
fn test_set_blend_mode_no_error() {
    let env = env();
    let (_, tex) = setup_texture(&env, "BMSet");
    env.exec(&format!(r#"{tex}:SetBlendMode("ADD"); {tex}:SetBlendMode("ALPHAKEY"); {tex}:SetBlendMode("DISABLE"); {tex}:SetBlendMode("MOD")"#)).unwrap();
}

#[test]
fn test_set_blend_mode_persists_raw_mode_on_frame() {
    let env = env();
    let (_, tex) = setup_texture(&env, "BMState");
    env.exec(&format!(r#"{tex}:SetBlendMode("MOD")"#)).unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(&tex).unwrap();
    let widget = state.widgets.get(id).unwrap();
    assert_eq!(widget.alpha_mode.as_deref(), Some("MOD"));
    assert_eq!(widget.blend_mode, wow_ui_sim::BlendMode::Alpha);
}

// ============================================================================
// SetDesaturated / IsDesaturated
// ============================================================================

#[test]
fn test_desaturated_default() {
    let env = env();
    let (_, tex) = setup_texture(&env, "Desat");
    let desat: bool = env.eval(&format!("return {tex}:IsDesaturated()")).unwrap();
    assert!(!desat, "IsDesaturated should default to false");
}

#[test]
fn test_set_desaturated_no_error() {
    let env = env();
    let (_, tex) = setup_texture(&env, "DesatSet");
    env.exec(&format!(
        "{tex}:SetDesaturated(true); {tex}:SetDesaturated(false)"
    ))
    .unwrap();
}

// ============================================================================
// SetAtlas / GetAtlas
// ============================================================================

#[test]
fn test_set_atlas_known() {
    let env = env();
    let (_, tex) = setup_texture(&env, "Atlas");
    env.exec(&format!(r#"{tex}:SetAtlas("checkbox-minimal")"#))
        .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(&tex).unwrap();
    let widget = state.widgets.get(id).unwrap();
    assert_eq!(widget.atlas.as_deref(), Some("checkbox-minimal"));
    assert!(
        widget.texture.is_some(),
        "Known atlas should set texture path"
    );
    assert!(
        widget.tex_coords.is_some(),
        "Known atlas should set tex_coords"
    );
    assert!(
        widget.atlas_tex_coords.is_some(),
        "Known atlas should set atlas_tex_coords"
    );
}

#[test]
fn test_set_atlas_tile_slice_uses_direct_atlas_entry() {
    let env = env();
    let (_, tex) = setup_texture(&env, "QuestlogAtlas");
    env.exec(&format!(r#"{tex}:SetAtlas("questlog-frame")"#))
        .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(&tex).unwrap();
    let widget = state.widgets.get(id).unwrap();

    assert_eq!(widget.atlas.as_deref(), Some("questlog-frame"));
    assert!(
        widget
            .texture
            .as_ref()
            .is_some_and(|path| path.contains("questlogframe")),
        "questlog-frame should resolve to questlogframe texture, got: {:?}",
        widget.texture
    );
    assert!(
        widget.tex_coords.is_some(),
        "questlog-frame should set tex_coords"
    );
    assert!(
        widget.atlas_tex_coords.is_some(),
        "questlog-frame should set atlas_tex_coords"
    );
    assert!(
        widget.nine_slice_atlas.is_none(),
        "questlog-frame should stay a direct atlas slice, not a nine-slice kit"
    );
}

#[test]
fn test_set_atlas_unknown() {
    let env = env();
    let (_, tex) = setup_texture(&env, "AtlasUnk");
    env.exec(&format!(
        r#"{tex}:SetAtlas("nonexistent-atlas-name-12345")"#
    ))
    .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(&tex).unwrap();
    let widget = state.widgets.get(id).unwrap();
    assert_eq!(
        widget.atlas.as_deref(),
        Some("nonexistent-atlas-name-12345")
    );
    assert!(
        widget.texture.is_none(),
        "Unknown atlas should not set texture path"
    );
}

#[test]
fn test_get_atlas_default_nil() {
    let env = env();
    let (_, tex) = setup_texture(&env, "AtlasNil");
    let is_nil: bool = env
        .eval(&format!("return {tex}:GetAtlas() == nil"))
        .unwrap();
    assert!(is_nil, "GetAtlas should return nil by default");
}

#[test]
fn test_get_atlas_returns_name() {
    let env = env();
    let (_, tex) = setup_texture(&env, "AtlasGet");
    env.exec(&format!(r#"{tex}:SetAtlas("checkbox-minimal")"#))
        .unwrap();
    let name: String = env.eval(&format!("return {tex}:GetAtlas()")).unwrap();
    assert_eq!(name, "checkbox-minimal");
}

// ============================================================================
// SetAtlas - button parent propagation
// ============================================================================

#[test]
fn test_set_atlas_propagates_to_button_normal_texture() {
    let env = env();
    env.exec(
        r#"
        local btn = CreateFrame("Button", "AtlasBtnFrame", UIParent)
        btn:SetSize(30, 30)
        local normalTex = btn:CreateTexture()
        btn:SetNormalTexture(normalTex)
        normalTex:SetAtlas("checkbox-minimal")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("AtlasBtnFrame").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert!(
        btn.normal_texture.is_some(),
        "SetAtlas on NormalTexture child should propagate to parent button"
    );
    assert!(
        btn.normal_tex_coords.is_some(),
        "SetAtlas on NormalTexture child should set parent's normal_tex_coords"
    );
}

// ============================================================================
// Pixel grid and texel snapping stubs
// ============================================================================

#[test]
fn test_snap_to_pixel_grid() {
    let env = env();
    let (_, tex) = setup_texture(&env, "Snap");
    env.exec(&format!("{tex}:SetSnapToPixelGrid(true)"))
        .unwrap();
    let snap: bool = env
        .eval(&format!("return {tex}:IsSnappingToPixelGrid()"))
        .unwrap();
    assert!(snap);
}

#[test]
fn test_texel_snapping_bias() {
    let env = env();
    let (_, tex) = setup_texture(&env, "Bias");
    env.exec(&format!("{tex}:SetTexelSnappingBias(0.5)"))
        .unwrap();
    let bias: f64 = env
        .eval(&format!("return {tex}:GetTexelSnappingBias()"))
        .unwrap();
    assert_eq!(bias, 0.5);
}

// ============================================================================
// Nine-slice stubs
// ============================================================================

#[test]
fn test_nine_slice_margins() {
    let env = env();
    let (_, tex) = setup_texture(&env, "NS");
    env.exec(&format!("{tex}:SetTextureSliceMargins(10, 20, 30, 40)"))
        .unwrap();
    let (l, r, t, b): (f64, f64, f64, f64) = env
        .eval(&format!("return {tex}:GetTextureSliceMargins()"))
        .unwrap();
    assert_eq!((l, r, t, b), (10.0, 20.0, 30.0, 40.0));
}

#[test]
fn test_nine_slice_mode() {
    let env = env();
    let (_, tex) = setup_texture(&env, "NSMode");
    env.exec(&format!("{tex}:SetTextureSliceMode(1)")).unwrap();
    let mode: i32 = env
        .eval(&format!("return {tex}:GetTextureSliceMode()"))
        .unwrap();
    assert_eq!(mode, 1);
}

#[test]
fn test_clear_texture_slice_resets_margins_and_mode() {
    let env = env();
    let (_, tex) = setup_texture(&env, "NSClear");
    let (l, r, t, b, mode): (f64, f64, f64, f64, i32) = env
        .eval(&format!(
            r#"
            {tex}:SetTextureSliceMargins(10, 20, 30, 40)
            {tex}:SetTextureSliceMode(1)
            {tex}:ClearTextureSlice()
            local l, r, t, b = {tex}:GetTextureSliceMargins()
            return l, r, t, b, {tex}:GetTextureSliceMode()
        "#
        ))
        .unwrap();
    assert_eq!((l, r, t, b), (0.0, 0.0, 0.0, 0.0));
    assert_eq!(mode, 0);
}

// ============================================================================
// Mask textures
// ============================================================================

#[test]
fn test_add_and_remove_mask_texture() {
    let env = env();
    let (added, removed): (i32, i32) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", nil, UIParent)
        local tex = f:CreateTexture(nil, "BACKGROUND")
        local mask = f:CreateMaskTexture(nil, "BACKGROUND")
        tex:AddMaskTexture(mask)
        local a = tex:GetNumMaskTextures()
        tex:RemoveMaskTexture(mask)
        return a, tex:GetNumMaskTextures()
    "#,
        )
        .unwrap();
    assert_eq!(added, 1);
    assert_eq!(removed, 0);
}

#[test]
fn test_add_mask_texture_no_duplicates() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", nil, UIParent)
        local tex = f:CreateTexture(nil, "BACKGROUND")
        local mask = f:CreateMaskTexture(nil, "BACKGROUND")
        tex:AddMaskTexture(mask); tex:AddMaskTexture(mask)
        return tex:GetNumMaskTextures()
    "#,
        )
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn test_get_mask_texture_nil() {
    let env = env();
    let (_, tex) = setup_texture(&env, "MaskNil");
    let is_nil: bool = env
        .eval(&format!("return {tex}:GetMaskTexture(1) == nil"))
        .unwrap();
    assert!(is_nil);
}

// ============================================================================
// SetPortraitToTexture
// ============================================================================

#[test]
fn test_set_portrait_to_texture_applies_circle_mask() {
    let env = env();
    let mask_count: i32 = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "PortraitMaskFrame", UIParent)
            local tex = frame:CreateTexture("PortraitMaskTex", "BORDER")
            SetPortraitToTexture(tex, "Interface\\Icons\\Ability_Mount_RidingHorse")
            return tex:GetNumMaskTextures()
            "#,
        )
        .unwrap();
    assert_eq!(
        mask_count, 1,
        "SetPortraitToTexture should apply a circular mask"
    );
}

#[test]
fn test_set_portrait_to_texture_no_double_mask() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "PortraitNoDoubleFrame", UIParent)
            local tex = frame:CreateTexture("PortraitNoDoubleTex", "BORDER")
            SetPortraitToTexture(tex, "Interface\\Icons\\Ability_Mount_RidingHorse")
            SetPortraitToTexture(tex, "Interface\\Icons\\INV_Misc_QuestionMark")
            return tex:GetNumMaskTextures()
            "#,
        )
        .unwrap();
    assert_eq!(
        count, 1,
        "calling SetPortraitToTexture twice should not add a second mask"
    );
}

// ============================================================================
// SetDrawLayer / GetDrawLayer
// ============================================================================

#[test]
fn test_draw_layer_default() {
    let env = env();
    let (_, tex) = setup_texture(&env, "DL");
    let (layer, sublayer): (String, i32) =
        env.eval(&format!("return {tex}:GetDrawLayer()")).unwrap();
    assert_eq!(layer, "BACKGROUND");
    assert_eq!(sublayer, 0);
}

#[test]
fn test_set_draw_layer_no_error() {
    let env = env();
    let (_, tex) = setup_texture(&env, "DLSet");
    env.exec(&format!(
        r#"{tex}:SetDrawLayer("OVERLAY", 2); {tex}:SetDrawLayer("BORDER")"#
    ))
    .unwrap();
}

// ============================================================================
// SetGradient / SetCenterColor stubs
// ============================================================================

#[test]
fn test_set_gradient_no_error() {
    let env = env();
    let (_, tex) = setup_texture(&env, "Grad");
    env.exec(&format!(
        r#"{tex}:SetGradient("HORIZONTAL", {{r=1, g=0, b=0, a=1}}, {{r=0, g=0, b=1, a=1}})"#
    ))
    .unwrap();
}

#[test]
fn test_set_center_color_no_error() {
    let env = env();
    let (_, tex) = setup_texture(&env, "Center");
    env.exec(&format!("{tex}:SetCenterColor(1, 0, 0, 1)"))
        .unwrap();
}

// ============================================================================
// SetAtlas with useAtlasSize
// ============================================================================

#[test]
fn test_set_atlas_use_atlas_size() {
    let env = env();
    let (_, tex) = setup_texture(&env, "AtlasSize");
    env.exec(&format!(r#"{tex}:SetAtlas("checkbox-minimal", true)"#))
        .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(&tex).unwrap();
    let widget = state.widgets.get(id).unwrap();
    assert!(
        widget.width > 0.0 || widget.height > 0.0,
        "useAtlasSize=true should set non-zero dimensions from atlas"
    );
}
