//! Render order tests: strata bucket ordering and z-order correctness.

mod common;

use common::env_with_shared_xml;

/// Build strata buckets from a WowLuaEnv (mutable borrow), then return a clone.
fn build_strata_buckets(env: &wow_ui_sim::lua_api::WowLuaEnv) -> Vec<Vec<u64>> {
    let mut state = env.state().borrow_mut();
    let _ = state.get_strata_buckets();
    state.strata_buckets.as_ref().unwrap().clone()
}

// ============================================================================
// High frame_level border must not cover lower-level content
// ============================================================================

/// Reproduces the world map quest log bug: a decorative BorderFrame at
/// frame_level 100 covers quest POI icons at level 5-7 because the DFS
/// emits the border's texture AFTER the content children.
///
/// In WoW, a border frame's textures (edges/corners) render as part of
/// that frame's draw layer — they should not occlude child content at
/// lower frame_levels in the same parent.
#[test]
fn high_level_border_does_not_cover_lower_level_content() {
    let env = env_with_shared_xml();

    // Replicate QuestScrollFrame structure:
    // - ScrollFrame with a Background texture (BACKGROUND layer)
    // - A content child with an icon texture (ARTWORK layer)
    // - A BorderFrame child at frame_level 100 with a covering texture
    env.exec(
        r#"
        local panel = CreateFrame("Frame", "TestPanel", UIParent)
        panel:SetSize(300, 400)
        panel:SetPoint("CENTER")
        panel:Show()

        -- Background texture on the panel (like QuestLog-main-background)
        local bg = panel:CreateTexture("TestPanelBg", "BACKGROUND")
        bg:SetAllPoints()
        bg:SetColorTexture(0.1, 0.1, 0.1, 1)

        -- Content child at default frame_level (like quest entries)
        local content = CreateFrame("Frame", "TestContent", panel)
        content:SetAllPoints()
        content:Show()

        -- Icon texture on content (like POI button icon at ARTWORK layer)
        local icon = content:CreateTexture("TestIcon", "ARTWORK")
        icon:SetSize(20, 20)
        icon:SetPoint("CENTER")
        icon:SetColorTexture(1, 0, 0, 1)

        -- Decorative border at high frame_level (like ScrollFrameTemplate BorderFrame)
        local border = CreateFrame("Frame", "TestBorder", panel)
        border:SetAllPoints()
        border:SetFrameLevel(100)
        border:Show()

        -- Border texture covers the whole area (like the Border texture at level 100)
        local borderTex = border:CreateTexture("TestBorderTex", "ARTWORK")
        borderTex:SetAllPoints()
        borderTex:SetColorTexture(0, 0, 0, 0.8)
    "#,
    )
    .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();

    let icon_id = state.widgets.get_id_by_name("TestIcon").unwrap();
    let border_tex_id = state.widgets.get_id_by_name("TestBorderTex").unwrap();

    // Find both IDs in the strata bucket and check their order.
    // The icon MUST render AFTER the border texture so it appears on top.
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];

    let icon_pos = medium_bucket.iter().position(|&id| id == icon_id);
    let border_pos = medium_bucket.iter().position(|&id| id == border_tex_id);

    assert!(
        icon_pos.is_some(),
        "TestIcon should be in the MEDIUM strata bucket"
    );
    assert!(
        border_pos.is_some(),
        "TestBorderTex should be in the MEDIUM strata bucket"
    );

    let icon_pos = icon_pos.unwrap();
    let border_pos = border_pos.unwrap();

    assert!(
        icon_pos > border_pos,
        "Content icon (pos={icon_pos}) must render AFTER border texture (pos={border_pos}). \
         A decorative border at high frame_level should not cover lower-level content."
    );
}
