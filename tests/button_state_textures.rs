#![cfg(feature = "gui")]
//! Tests for button state-dependent texture visibility.
//!
//! WoW buttons have child Texture widgets (NormalTexture, PushedTexture,
//! HighlightTexture, DisabledTexture) that should only render when the
//! button is in the corresponding state.

use crate::common;

use common::env_with_shared_xml;
use wow_ui_sim::iced_app::{RegistryQuadBatchParams, build_quad_batch_for_registry};

/// Helper: build a quad batch for a named subtree with given button state.
fn build_batch_for_button(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    root: &str,
    pressed: Option<u64>,
    hovered: Option<u64>,
) -> wow_ui_sim::render::QuadBatch {
    env.set_screen_size(1024.0, 768.0);
    {
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
    }
    let buckets = {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    build_quad_batch_for_registry(
        RegistryQuadBatchParams::new(&state.widgets, (1024.0, 768.0), &buckets)
            .root_name(Some(root))
            .pressed_frame(pressed)
            .hovered_frame(hovered),
    )
}

fn build_text_batch_for_button(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    root: &str,
) -> wow_ui_sim::render::QuadBatch {
    use wow_ui_sim::render::font::WowFontSystem;
    use wow_ui_sim::render::glyph::GlyphAtlas;

    env.set_screen_size(1024.0, 768.0);
    {
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
    }
    let buckets = {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let mut font_sys = WowFontSystem::new();
    let mut glyph_atlas = GlyphAtlas::new();
    let state = env.state().borrow();
    build_quad_batch_for_registry(
        RegistryQuadBatchParams::new(&state.widgets, (1024.0, 768.0), &buckets)
            .root_name(Some(root))
            .text_ctx(Some((&mut font_sys, &mut glyph_atlas))),
    )
}

fn glyph_vertex_bounds(batch: &wow_ui_sim::render::QuadBatch) -> (f32, f32, f32, f32) {
    let glyph_tex_index = wow_ui_sim::render::shader::GLYPH_ATLAS_TEX_INDEX;
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for vertex in &batch.vertices {
        if vertex.tex_index != glyph_tex_index {
            continue;
        }
        min_x = min_x.min(vertex.position[0]);
        min_y = min_y.min(vertex.position[1]);
        max_x = max_x.max(vertex.position[0]);
        max_y = max_y.max(vertex.position[1]);
    }
    assert!(min_x.is_finite(), "Batch should contain glyph vertices");
    (min_x, min_y, max_x, max_y)
}

fn glyph_vertex_count(batch: &wow_ui_sim::render::QuadBatch) -> usize {
    let glyph_tex_index = wow_ui_sim::render::shader::GLYPH_ATLAS_TEX_INDEX;
    batch
        .vertices
        .iter()
        .filter(|vertex| vertex.tex_index == glyph_tex_index)
        .count()
}

/// In normal state, NormalTexture renders but PushedTexture does not.
/// In pressed state, PushedTexture renders but NormalTexture does not.
#[test]
fn normal_vs_pressed_texture() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestStateTex", UIParent)
        btn:SetPoint("CENTER")
        btn:SetSize(100, 30)
        btn:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
        btn:SetPushedTexture("Interface/Buttons/UI-Panel-Button-Down")
        btn:Show()
    "#,
    )
    .unwrap();

    let btn_id = {
        let state = env.state().borrow();
        state.widgets.get_id_by_name("TestStateTex").unwrap()
    };

    // Normal state: NormalTexture renders, PushedTexture does not
    let batch = build_batch_for_button(&env, "TestStateTex", None, None);
    assert!(
        batch
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains("button-up")),
        "Normal state should render NormalTexture"
    );
    assert!(
        !batch
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains("button-down")),
        "Normal state should NOT render PushedTexture"
    );

    // Pressed state: PushedTexture renders, NormalTexture does not
    let batch = build_batch_for_button(&env, "TestStateTex", Some(btn_id), None);
    assert!(
        !batch
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains("button-up")),
        "Pressed state should NOT render NormalTexture"
    );
    assert!(
        batch
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains("button-down")),
        "Pressed state should render PushedTexture"
    );
}

#[test]
fn state_texture_child_stays_hidden_when_button_is_hidden() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestHiddenStateTex", UIParent)
        btn:SetPoint("CENTER")
        btn:SetSize(100, 30)
        btn:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
        btn:Hide()
    "#,
    )
    .unwrap();

    let batch = build_batch_for_button(&env, "TestHiddenStateTex", None, None);
    assert!(
        !batch
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains("button-up")),
        "hidden buttons should not render state texture children"
    );
}

/// HighlightTexture renders only when hovered.
#[test]
fn highlight_texture_only_when_hovered() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestHighlight", UIParent)
        btn:SetPoint("CENTER")
        btn:SetSize(100, 30)
        btn:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
        btn:SetHighlightTexture("Interface/Buttons/UI-Panel-Button-Highlight")
        btn:Show()
    "#,
    )
    .unwrap();

    let btn_id = {
        let state = env.state().borrow();
        state.widgets.get_id_by_name("TestHighlight").unwrap()
    };

    // Not hovered: no highlight texture
    let batch = build_batch_for_button(&env, "TestHighlight", None, None);
    assert!(
        !batch
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains("highlight")),
        "Non-hovered state should NOT render HighlightTexture"
    );

    // Hovered: highlight texture appears
    let batch = build_batch_for_button(&env, "TestHighlight", None, Some(btn_id));
    assert!(
        batch
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains("highlight")),
        "Hovered state should render HighlightTexture"
    );
}

#[test]
fn highlight_texture_child_stays_hidden_until_hover_render_path() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestHighlightVisibility", UIParent)
        btn:SetPoint("CENTER")
        btn:SetSize(100, 30)
        btn:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
        btn:SetHighlightTexture("Interface/Buttons/UI-Panel-Button-Highlight")
        btn:Show()
    "#,
    )
    .unwrap();

    let hidden: bool = env
        .eval("return TestHighlightVisibility:GetHighlightTexture():IsShown() == false")
        .unwrap();
    assert!(
        hidden,
        "HighlightTexture child should stay hidden outside the hover render path"
    );
}

fn red_highlight_quads(batch: &wow_ui_sim::render::QuadBatch) -> usize {
    batch
        .vertices
        .chunks_exact(4)
        .filter(|quad| {
            let color = quad[0].color;
            color[0] > 0.9 && color[1] < 0.1 && color[2] < 0.1
        })
        .count()
}

#[test]
fn locked_button_highlight_renders_once_without_hover_and_unlock_restores_hover() {
    let env = wow_ui_sim::lua_api::WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local parent = CreateFrame("Frame", "TestLockParent", UIParent)
        parent:SetPoint("CENTER")
        parent:SetSize(120, 40)
        local btn = CreateFrame("Button", "TestLockButton", parent)
        btn:SetAllPoints(parent)
        local highlight = btn:CreateTexture(nil, "HIGHLIGHT")
        highlight:SetAllPoints(btn)
        highlight:SetColorTexture(1, 0, 0, 1)
        btn:SetHighlightTexture(highlight)
        "#,
    )
    .unwrap();
    let button_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("TestLockButton")
        .unwrap();
    let count = |hovered| {
        red_highlight_quads(&build_batch_for_button(
            &env,
            "TestLockParent",
            None,
            hovered,
        ))
    };

    assert_eq!(count(None), 0, "unlocked nonhover must not emit highlight");
    assert_eq!(count(Some(button_id)), 1, "ordinary hover emits once");

    env.exec("TestLockButton:LockHighlight()").unwrap();
    assert_eq!(count(None), 1, "locked nonhover must emit highlight");
    assert_eq!(
        count(Some(button_id)),
        1,
        "locked hover must not double-emit"
    );

    let mouse_pressed = red_highlight_quads(&build_batch_for_button(
        &env,
        "TestLockParent",
        Some(button_id),
        Some(button_id),
    ));
    env.exec("TestLockButton:SetButtonState('PUSHED')").unwrap();
    let pushed = count(None);
    env.exec("TestLockButton:SetButtonState('NORMAL'); TestLockButton:Disable()")
        .unwrap();
    let disabled = count(None);
    let disabled_hovered = count(Some(button_id));
    env.exec("TestLockButton:Enable()").unwrap();
    let enabled_again = count(None);
    assert_eq!(
        (
            mouse_pressed,
            pushed,
            disabled,
            disabled_hovered,
            enabled_again
        ),
        (0, 0, 0, 0, 1),
        "locking must preserve pressed/disabled suppression and resume after enabling"
    );

    env.exec("TestLockButton:UnlockHighlight()").unwrap();
    assert_eq!(count(None), 0, "unlock restores nonhover suppression");
    assert_eq!(count(Some(button_id)), 1, "unlock restores hover emission");

    env.exec("TestLockButton:SetHighlightLocked(true)").unwrap();
    assert_eq!(
        count(None),
        1,
        "SetHighlightLocked also renders when not hovered"
    );
    assert_eq!(
        count(Some(button_id)),
        1,
        "SetHighlightLocked hover emits once"
    );
    env.exec("TestLockParent:Hide()").unwrap();
    assert_eq!(
        count(None),
        0,
        "hidden ancestor suppresses locked highlight"
    );
    assert_eq!(
        count(Some(button_id)),
        0,
        "hidden ancestor suppresses hover overlay"
    );
    env.exec("TestLockParent:Show(); TestLockButton:SetHighlightLocked(false)")
        .unwrap();
    assert_eq!(count(None), 0, "cleared lock restores nonhover suppression");
    assert_eq!(count(Some(button_id)), 1, "cleared lock restores hover");
}

/// Disabled button shows DisabledTexture instead of NormalTexture.
/// Pressed/hovered state has no effect while disabled.
#[test]
fn disabled_button_shows_disabled_texture() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestDisabled", UIParent)
        btn:SetPoint("CENTER")
        btn:SetSize(100, 30)
        btn:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
        btn:SetPushedTexture("Interface/Buttons/UI-Panel-Button-Down")
        btn:SetDisabledTexture("Interface/Buttons/UI-Panel-Button-Disabled")
        btn:SetHighlightTexture("Interface/Buttons/UI-Panel-Button-Highlight")
        btn:Disable()
        btn:Show()
    "#,
    )
    .unwrap();

    let btn_id = {
        let state = env.state().borrow();
        state.widgets.get_id_by_name("TestDisabled").unwrap()
    };

    let has_path = |batch: &wow_ui_sim::render::QuadBatch, substr: &str| -> bool {
        batch
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains(substr))
    };

    // Disabled + not interacted: DisabledTexture shows, NormalTexture hidden
    let batch = build_batch_for_button(&env, "TestDisabled", None, None);
    assert!(
        has_path(&batch, "button-disabled"),
        "Disabled state should render DisabledTexture"
    );
    assert!(
        !has_path(&batch, "button-up"),
        "Disabled state should NOT render NormalTexture"
    );
    assert!(
        !has_path(&batch, "button-down"),
        "Disabled state should NOT render PushedTexture"
    );
    assert!(
        !has_path(&batch, "highlight"),
        "Disabled state should NOT render HighlightTexture"
    );

    // Disabled + pressed: still shows DisabledTexture (pressing disabled button is a no-op)
    let batch = build_batch_for_button(&env, "TestDisabled", Some(btn_id), None);
    assert!(
        has_path(&batch, "button-disabled"),
        "Disabled+pressed should still show DisabledTexture"
    );
    assert!(
        !has_path(&batch, "button-down"),
        "Disabled+pressed should NOT show PushedTexture"
    );

    // Re-enable: NormalTexture returns, DisabledTexture hidden
    env.exec("TestDisabled:Enable()").unwrap();
    let batch = build_batch_for_button(&env, "TestDisabled", None, None);
    assert!(
        has_path(&batch, "button-up"),
        "Re-enabled should render NormalTexture"
    );
    assert!(
        !has_path(&batch, "button-disabled"),
        "Re-enabled should NOT render DisabledTexture"
    );
}

#[test]
fn pressed_button_text_child_uses_pushed_text_offset() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestPressedTextOffset", UIParent)
        btn:SetPoint("CENTER")
        btn:SetSize(140, 30)
        btn:SetText("Push Me")
        btn:SetPushedTextOffset(4, 3)
        btn:Show()
    "#,
    )
    .unwrap();

    let normal_batch = build_text_batch_for_button(&env, "TestPressedTextOffset");
    assert!(
        !normal_batch.vertices.is_empty(),
        "Normal button text should emit glyph vertices"
    );
    let normal_bounds = glyph_vertex_bounds(&normal_batch);

    env.exec(r#"TestPressedTextOffset:SetButtonState("PUSHED")"#)
        .unwrap();
    let pushed_batch = build_text_batch_for_button(&env, "TestPressedTextOffset");
    assert!(
        !pushed_batch.vertices.is_empty(),
        "Pressed button text should emit glyph vertices"
    );
    let pushed_bounds = glyph_vertex_bounds(&pushed_batch);

    assert!(
        (pushed_bounds.0 - (normal_bounds.0 + 4.0)).abs() < 0.01,
        "Pressed text min_x should shift by +4; normal={:?} pushed={:?}",
        normal_bounds,
        pushed_bounds
    );
    assert!(
        (pushed_bounds.1 - (normal_bounds.1 + 3.0)).abs() < 0.01,
        "Pressed text min_y should shift by +3; normal={:?} pushed={:?}",
        normal_bounds,
        pushed_bounds
    );
    assert!(
        (pushed_bounds.2 - (normal_bounds.2 + 4.0)).abs() < 0.01,
        "Pressed text max_x should shift by +4; normal={:?} pushed={:?}",
        normal_bounds,
        pushed_bounds
    );
    assert!(
        (pushed_bounds.3 - (normal_bounds.3 + 3.0)).abs() < 0.01,
        "Pressed text max_y should shift by +3; normal={:?} pushed={:?}",
        normal_bounds,
        pushed_bounds
    );
}

#[test]
fn lowercase_button_text_child_suppresses_direct_button_text_rendering() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local childOnly = CreateFrame("Button", "TestLowercaseChildOnlyText", UIParent)
        childOnly:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -20)
        childOnly:SetSize(100, 30)
        local childOnlyText = childOnly:CreateFontString(nil, "ARTWORK", "GameFontNormal")
        childOnlyText:SetPoint("LEFT", 20, 0)
        childOnlyText:SetText("World")
        childOnly.text = childOnlyText
        childOnly:Show()

        local withButtonText = CreateFrame("Button", "TestLowercaseChildAndButtonText", UIParent)
        withButtonText:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -60)
        withButtonText:SetSize(100, 30)
        local explicitText = withButtonText:CreateFontString(nil, "ARTWORK", "GameFontNormal")
        explicitText:SetPoint("LEFT", 20, 0)
        explicitText:SetText("World")
        withButtonText.text = explicitText
        withButtonText:SetText("World")
        withButtonText:Show()
    "#,
    )
    .unwrap();

    let child_only_batch = build_text_batch_for_button(&env, "TestLowercaseChildOnlyText");
    let with_button_text_batch =
        build_text_batch_for_button(&env, "TestLowercaseChildAndButtonText");
    let child_only_glyphs = glyph_vertex_count(&child_only_batch);
    let with_button_text_glyphs = glyph_vertex_count(&with_button_text_batch);

    assert!(
        child_only_glyphs > 0,
        "Lowercase text child should render its label"
    );
    assert_eq!(
        with_button_text_glyphs, child_only_glyphs,
        "Button:SetText should not add a second rendered label when a lowercase text child exists"
    );
}

#[test]
fn clearing_button_texture_hides_rendered_child_texture() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestClearNormalTextureRender", UIParent)
        btn:SetPoint("CENTER")
        btn:SetSize(100, 30)
        btn:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
        btn:Show()
    "#,
    )
    .unwrap();

    let before = build_batch_for_button(&env, "TestClearNormalTextureRender", None, None);
    assert!(
        before
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains("button-up")),
        "Normal texture should render before ClearNormalTexture"
    );

    env.exec(r#"TestClearNormalTextureRender:ClearNormalTexture()"#)
        .unwrap();
    let after = build_batch_for_button(&env, "TestClearNormalTextureRender", None, None);
    assert!(
        !after
            .texture_requests
            .iter()
            .any(|r| r.path.to_lowercase().contains("button-up")),
        "Normal texture should not render after ClearNormalTexture"
    );
}

fn c7_font_env() -> wow_ui_sim::lua_api::WowLuaEnv {
    let env = env_with_shared_xml();
    env.exec(
        r#"
        local function font(name, path, size, flags, r, g, b)
            local object = CreateFont(name)
            object:SetFont(path, size, flags)
            object:SetTextColor(r, g, b, 1)
            return { object = object, path = path, size = size, flags = flags, color = {r, g, b, 1} }
        end
        C7Normal = font("C7NormalFont", "Fonts\\FRIZQT__.TTF", 12, "", 1, 0, 0)
        C7Highlight = font("C7HighlightFont", "Fonts\\ARIALN.TTF", 20, "OUTLINE", 0, 1, 0)
        C7Disabled = font("C7DisabledFont", "Fonts\\FRIZQT__.TTF", 16, "", 0, 0, 1)
        C7Seed = font("C7SeedFont", "Fonts\\FRIZQT__.TTF", 9, "", 1, 1, 1)

        C7Button = CreateFrame("Button", "C7Button", UIParent)
        C7Button:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
        C7Button:SetSize(260, 70)
        C7Label = C7Button:CreateFontString("C7OwnedLabel", "ARTWORK")
        C7Label:SetFontObject(C7Seed.object)
        C7Button:SetFontString(C7Label)
        C7Label:ClearAllPoints()
        C7Label:SetPoint("CENTER", C7Button, "CENTER", 2, -1)
        C7Label:SetSize(240, 60)
        C7Label:SetText("Wm8")
        C7Anchor = {C7Label:GetPoint(1)}

        C7NormalTexture = C7Button:CreateTexture(nil, "BACKGROUND")
        C7NormalTexture:SetAllPoints(C7Button)
        C7NormalTexture:SetTexture("Interface/Buttons/UI-Panel-Button-Up")
        C7Button:SetNormalTexture(C7NormalTexture)
        C7HighlightTexture = C7Button:CreateTexture(nil, "HIGHLIGHT")
        C7HighlightTexture:SetAllPoints(C7Button)
        C7HighlightTexture:SetTexture("Interface/Buttons/UI-Panel-Button-Highlight")
        C7Button:SetHighlightTexture(C7HighlightTexture)
        C7NormalSource = C7NormalTexture:GetTexture()
        C7HighlightSource = C7HighlightTexture:GetTexture()
        assert(C7NormalSource ~= nil and C7HighlightSource ~= nil, 'texture sources assigned')
        C7Button:SetPushedTextOffset(4, 3)
        C7Button:SetNormalFontObject(C7Normal.object)
        C7Button:SetHighlightFontObject(C7Highlight.object)
        C7Button:SetDisabledFontObject(C7Disabled.object)
        C7Button:Show()

        -- Independent ordinary FontString: expected glyph geometry is rendered,
        -- never guessed from a nominal font height or hardcoded pixel bounds.
        C7Reference = CreateFrame("Frame", "C7Reference", UIParent)
        C7Reference:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
        C7Reference:SetSize(260, 70)
        C7ReferenceLabel = C7Reference:CreateFontString(nil, "ARTWORK")
        C7ReferenceLabel:SetPoint("CENTER", C7Reference, "CENTER", 2, -1)
        C7ReferenceLabel:SetSize(240, 60)
        C7ReferenceLabel:SetText("Wm8")
        C7Reference:Show()

        function C7Check(expected)
            assert(C7Button:GetFontString() == C7Label, "owned FontString identity")
            assert(C7Label:GetParent() == C7Button, "owned FontString parent")
            assert(C7Label:GetText() == "Wm8", "owned text unchanged")
            assert(C7Label:GetDrawLayer() == "ARTWORK", "owned draw layer unchanged")
            assert(C7Label:GetNumPoints() == 1, "owned anchor count unchanged")
            local anchor = {C7Label:GetPoint(1)}
            for i = 1, 5 do assert(anchor[i] == C7Anchor[i], "owned anchor " .. i) end
            assert(C7Button:GetNormalTexture() == C7NormalTexture, "normal texture identity")
            assert(C7Button:GetHighlightTexture() == C7HighlightTexture, "highlight texture identity")
            assert(C7NormalTexture:GetTexture() == C7NormalSource, 'normal texture source unchanged')
            assert(C7HighlightTexture:GetTexture() == C7HighlightSource, 'highlight texture source unchanged')
            assert(C7NormalTexture:GetTextureFilePath() == "Interface/Buttons/UI-Panel-Button-Up",
                'normal texture path unchanged')
            assert(C7HighlightTexture:GetTextureFilePath() == "Interface/Buttons/UI-Panel-Button-Highlight",
                'highlight texture path unchanged')
            local path, size, flags = C7Label:GetFont()
            assert(path == expected.path, "selected font path")
            assert(size == expected.size and flags == expected.flags, "selected font size/flags")
            local color = {C7Label:GetTextColor()}
            for i = 1, 4 do assert(color[i] == expected.color[i], "selected color " .. i) end
            C7ReferenceLabel:SetFontObject(expected.object)
        end
        "#,
    )
    .unwrap();
    env
}

fn c7_assert_projection(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    expected: &str,
    color: [f32; 4],
) -> wow_ui_sim::render::QuadBatch {
    // Observe the public child before rendering: a render-only tint is not enough.
    env.exec(&format!("C7Check({expected})")).unwrap();
    let actual = build_text_batch_for_button(env, "C7Button");
    let reference = build_text_batch_for_button(env, "C7Reference");
    let actual_bounds = glyph_vertex_bounds(&actual);
    let reference_bounds = glyph_vertex_bounds(&reference);
    let glyph_index = wow_ui_sim::render::shader::GLYPH_ATLAS_TEX_INDEX;
    let actual_glyphs: Vec<_> = actual
        .vertices
        .iter()
        .filter(|vertex| vertex.tex_index == glyph_index)
        .collect();
    let reference_glyphs: Vec<_> = reference
        .vertices
        .iter()
        .filter(|vertex| vertex.tex_index == glyph_index)
        .collect();
    assert!(!actual_glyphs.is_empty(), "selected label must emit glyphs");
    assert_eq!(actual_glyphs.len(), reference_glyphs.len());
    assert!(
        actual_glyphs.iter().any(|vertex| vertex.color == color),
        "{expected}: selected foreground color must render"
    );
    for (actual_vertex, reference_vertex) in actual_glyphs.iter().zip(&reference_glyphs) {
        for channel in 0..4 {
            assert!(
                (actual_vertex.color[channel] - reference_vertex.color[channel]).abs() < 0.001,
                "{expected}: glyph color {:?}",
                actual_vertex.color
            );
        }
        // Normalize translation only: width, height and each glyph's placement
        // must match real shaping of the expected font path and size.
        for (axis, actual_origin, reference_origin) in [
            (0, actual_bounds.0, reference_bounds.0),
            (1, actual_bounds.1, reference_bounds.1),
        ] {
            assert!(
                ((actual_vertex.position[axis] - actual_origin)
                    - (reference_vertex.position[axis] - reference_origin))
                    .abs()
                    < 0.01,
                "{expected}: rendered glyph geometry differs on axis {axis}"
            );
        }
    }
    actual
}

fn c7_assert_reference_projection(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    expected: &str,
    color: [f32; 4],
) {
    env.exec(&format!(
        "assert(C7Label:GetFontObject() == {expected}.object, 'selected FontObject reference')"
    ))
    .unwrap();
    c7_assert_projection(env, expected, color);
}

#[test]
fn c7_locked_highlight_font_survives_pushed_with_independent_text_offset() {
    let env = c7_font_env();
    c7_assert_projection(&env, "C7Normal", [1.0, 0.0, 0.0, 1.0]);
    env.exec("A_Admin.SetMouseOverFrame(C7Button)").unwrap();
    c7_assert_projection(&env, "C7Highlight", [0.0, 1.0, 0.0, 1.0]);
    env.exec("A_Admin.SetMouseOverFrame(nil)").unwrap();
    c7_assert_projection(&env, "C7Normal", [1.0, 0.0, 0.0, 1.0]);
    env.exec("C7Button:LockHighlight()").unwrap();
    let highlighted = c7_assert_projection(&env, "C7Highlight", [0.0, 1.0, 0.0, 1.0]);
    env.exec("C7Button:SetButtonState('PUSHED')").unwrap();
    let pushed = c7_assert_projection(&env, "C7Highlight", [0.0, 1.0, 0.0, 1.0]);
    let before = glyph_vertex_bounds(&highlighted);
    let after = glyph_vertex_bounds(&pushed);
    for (observed, expected) in [
        (after.0, before.0 + 4.0),
        (after.1, before.1 + 3.0),
        (after.2, before.2 + 4.0),
        (after.3, before.3 + 3.0),
    ] {
        assert!((observed - expected).abs() < 0.01, "pushed offset");
    }
    env.exec("C7Button:UnlockHighlight()").unwrap();
    c7_assert_projection(&env, "C7Normal", [1.0, 0.0, 0.0, 1.0]);
    env.exec("C7Button:SetButtonState('NORMAL'); C7Button:SetHighlightLocked(true)")
        .unwrap();
    c7_assert_projection(&env, "C7Highlight", [0.0, 1.0, 0.0, 1.0]);
    env.exec("C7Button:SetHighlightLocked(false)").unwrap();
    c7_assert_projection(&env, "C7Normal", [1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn c7_disabled_font_precedes_highlight_and_normal_returns_after_enable() {
    let env = c7_font_env();
    c7_assert_projection(&env, "C7Normal", [1.0, 0.0, 0.0, 1.0]);
    env.exec("C7Button:Disable()").unwrap();
    c7_assert_projection(&env, "C7Disabled", [0.0, 0.0, 1.0, 1.0]);
    env.exec("C7Button:LockHighlight()").unwrap();
    c7_assert_projection(&env, "C7Disabled", [0.0, 0.0, 1.0, 1.0]);
    env.exec("C7Button:Enable()").unwrap();
    c7_assert_projection(&env, "C7Highlight", [0.0, 1.0, 0.0, 1.0]);
    env.exec("C7Button:Disable(); C7Button:UnlockHighlight()")
        .unwrap();
    c7_assert_projection(&env, "C7Disabled", [0.0, 0.0, 1.0, 1.0]);
    env.exec("C7Button:Enable()").unwrap();
    c7_assert_projection(&env, "C7Normal", [1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn c7_replacing_each_active_font_slot_preserves_owned_children() {
    let env = c7_font_env();
    env.exec(
        r#"
        local object = CreateFont("C7ReplacementFont")
        object:SetFont("Fonts\\ARIALN.TTF", 18, "")
        object:SetTextColor(1, 0, 1, 1)
        C7Replacement = {object = object, path = "Fonts\\ARIALN.TTF", size = 18,
                         flags = "", color = {1, 0, 1, 1}}
        "#,
    )
    .unwrap();
    c7_assert_reference_projection(&env, "C7Normal", [1.0, 0.0, 0.0, 1.0]);
    env.exec("C7Button:SetNormalFontObject(C7Replacement.object)")
        .unwrap();
    c7_assert_reference_projection(&env, "C7Replacement", [1.0, 0.0, 1.0, 1.0]);
    env.exec("C7Button:LockHighlight()").unwrap();
    c7_assert_reference_projection(&env, "C7Highlight", [0.0, 1.0, 0.0, 1.0]);
    env.exec("C7Button:SetHighlightFontObject(C7Replacement.object)")
        .unwrap();
    c7_assert_reference_projection(&env, "C7Replacement", [1.0, 0.0, 1.0, 1.0]);
    // An inactive normal-slot setter must not overwrite active highlight style.
    env.exec("C7Button:SetNormalFontObject(C7Normal.object)")
        .unwrap();
    c7_assert_reference_projection(&env, "C7Replacement", [1.0, 0.0, 1.0, 1.0]);
    env.exec("C7Button:Disable()").unwrap();
    c7_assert_reference_projection(&env, "C7Disabled", [0.0, 0.0, 1.0, 1.0]);
    env.exec("C7Button:SetDisabledFontObject(C7Replacement.object)")
        .unwrap();
    c7_assert_reference_projection(&env, "C7Replacement", [1.0, 0.0, 1.0, 1.0]);
    env.exec("C7Button:Enable(); C7Button:UnlockHighlight()")
        .unwrap();
    c7_assert_reference_projection(&env, "C7Normal", [1.0, 0.0, 0.0, 1.0]);
}
