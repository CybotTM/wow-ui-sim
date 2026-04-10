#![cfg(feature = "gui")]
//! Tests for button state-dependent texture visibility.
//!
//! WoW buttons have child Texture widgets (NormalTexture, PushedTexture,
//! HighlightTexture, DisabledTexture) that should only render when the
//! button is in the corresponding state.

mod common;

use common::env_with_shared_xml;
use wow_ui_sim::iced_app::build_quad_batch_for_registry;

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
        &state.widgets,
        (1024.0, 768.0),
        Some(root),
        pressed,
        hovered,
        None,
        None,
        None,
        &buckets,
    )
}

fn build_text_batch_for_button(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    root: &str,
) -> wow_ui_sim::render::QuadBatch {
    use std::path::PathBuf;
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
    let mut font_sys = WowFontSystem::new(&PathBuf::from("./fonts"));
    let mut glyph_atlas = GlyphAtlas::new();
    let state = env.state().borrow();
    build_quad_batch_for_registry(
        &state.widgets,
        (1024.0, 768.0),
        Some(root),
        None,
        None,
        Some((&mut font_sys, &mut glyph_atlas)),
        None,
        None,
        &buckets,
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
