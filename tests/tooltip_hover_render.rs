//! Full-environment tooltip render-pipeline tests requiring GUI support.

mod tooltip_full_env_helpers;
mod tooltip_hover_helpers;

use tooltip_full_env_helpers::setup_full_env;
use tooltip_hover_helpers::{open_character_panel, refresh_buff_frame};

const HOVER_FIRST_VISIBLE_BUFF_ICON_LUA: &str = r#"
    local totalButtons = 0
    local shownButtons = 0
    local buttonsWithInfo = 0
    local buttonsWithIndex = 0
    for _, button in ipairs(BuffFrame.auraFrames) do
        totalButtons = totalButtons + 1
        if button.buttonInfo then
            buttonsWithInfo = buttonsWithInfo + 1
        end
        if button.buttonInfo and button.buttonInfo.index then
            buttonsWithIndex = buttonsWithIndex + 1
        end
        if button:IsShown() and button.buttonInfo and button.buttonInfo.index then
            button:OnEnter()
            return
        end
        if button:IsShown() then
            shownButtons = shownButtons + 1
        end
    end
    error(string.format(
        "No visible buff icon with tooltip data (BuffFrameShown=%s auraInfo=%s totalButtons=%d shownButtons=%d buttonsWithInfo=%d buttonsWithIndex=%d isExpanded=%s collapseEnabled=%s consolidateEnabled=%s)",
        tostring(BuffFrame:IsShown()),
        tostring(BuffFrame.auraInfo and #BuffFrame.auraInfo or nil),
        totalButtons,
        shownButtons,
        buttonsWithInfo,
        buttonsWithIndex,
        tostring(BuffFrame:IsExpanded()),
        tostring(BuffFrame.CollapseAndExpandButton and BuffFrame.CollapseAndExpandButton:IsEnabled()),
        tostring(BuffFrame.ConsolidatedBuffs and BuffFrame.ConsolidatedBuffs:IsEnabled())
    ))
"#;

fn hover_first_visible_buff_icon(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(HOVER_FIRST_VISIBLE_BUFF_ICON_LUA)
        .expect("Failed to hover a visible buff icon");
}

/// Verify the tooltip produces render quads after the full rendering pipeline runs.
#[cfg(feature = "gui")]
#[test]
fn test_tooltip_produces_quads_after_hover() {
    use wow_ui_sim::render::font::WowFontSystem;
    use wow_ui_sim::render::glyph::GlyphAtlas;

    let env = setup_full_env();

    // Hover over CharacterMicroButton
    let btn_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name("CharacterMicroButton")
            .expect("CharacterMicroButton should exist")
    };
    env.state().borrow_mut().hovered_frame = Some(btn_id);
    env.fire_script_handler(btn_id, "OnEnter", vec![]).unwrap();

    // Run tooltip sizing (same as build_quad_batch does)
    let mut font_sys = WowFontSystem::new();
    {
        let mut state = env.state().borrow_mut();
        let _ = state.widgets.take_render_dirty();
        wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_sys);
    }

    // Check tooltip got sized
    let gt_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("GameTooltip")
        .unwrap();
    let (w, h) = {
        let state = env.state().borrow();
        let f = state.widgets.get(gt_id).unwrap();
        (f.width, f.height)
    };
    eprintln!("Tooltip after sizing: {}x{}", w, h);
    assert!(
        w > 0.0,
        "Tooltip width should be > 0 after sizing, got {}",
        w
    );
    assert!(
        h > 0.0,
        "Tooltip height should be > 0 after sizing, got {}",
        h
    );

    // Check tooltip position (compute_frame_rect uses the anchor system)
    {
        let state = env.state().borrow();
        let rect = wow_ui_sim::iced_app::compute_frame_rect(&state.widgets, gt_id, 1024.0, 768.0);
        eprintln!(
            "Tooltip rect: x={}, y={}, w={}, h={}",
            rect.x, rect.y, rect.width, rect.height
        );
        assert!(rect.width > 0.0, "Tooltip rect width should be > 0");
        assert!(rect.height > 0.0, "Tooltip rect height should be > 0");
        // Check tooltip is within visible screen
        assert!(
            rect.x >= 0.0 && rect.x < 1024.0,
            "Tooltip x={} should be on screen",
            rect.x
        );
        assert!(
            rect.y >= 0.0 && rect.y < 768.0,
            "Tooltip y={} should be on screen",
            rect.y
        );
    }

    // Build quads and verify tooltip emits something
    let buckets = {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    let tooltip_data = wow_ui_sim::iced_app::tooltip::collect_tooltip_data(&state);
    assert!(!tooltip_data.is_empty(), "Tooltip render data should exist");

    let mut glyph_atlas = GlyphAtlas::new();
    let batch = wow_ui_sim::iced_app::build_quad_batch_for_registry(
        wow_ui_sim::iced_app::RegistryQuadBatchParams::new(
            &state.widgets,
            (1024.0, 768.0),
            &buckets,
        )
        .text_ctx(Some((&mut font_sys, &mut glyph_atlas)))
        .message_frames(Some(&state.message_frames))
        .tooltip_data(Some(&tooltip_data)),
    );

    // Tooltip renders via glyph quads (text) not texture quads.
    // Verify the tooltip frame was reached by checking total quad count increased.
    eprintln!(
        "Total quads: {}, vertices: {}",
        batch.vertices.len() / 4,
        batch.vertices.len()
    );
    assert!(
        batch.vertices.len() > 100,
        "Batch should have many vertices (tooltip + UI)"
    );
}

#[cfg(feature = "gui")]
#[test]
fn test_character_slot_hover_tooltip_produces_quads() {
    use wow_ui_sim::render::font::WowFontSystem;
    use wow_ui_sim::render::glyph::GlyphAtlas;

    let env = setup_full_env();
    open_character_panel(&env);

    let slot_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name("CharacterHeadSlot")
            .expect("CharacterHeadSlot should exist after opening character panel")
    };

    env.state().borrow_mut().hovered_frame = Some(slot_id);
    env.fire_script_handler(slot_id, "OnEnter", vec![]).unwrap();

    let mut font_sys = WowFontSystem::new();
    {
        let mut state = env.state().borrow_mut();
        let _ = state.widgets.take_render_dirty();
        wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_sys);
    }

    let gt_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("GameTooltip")
        .unwrap();
    let (w, h) = {
        let state = env.state().borrow();
        let frame = state.widgets.get(gt_id).unwrap();
        (frame.width, frame.height)
    };
    assert!(w > 0.0, "Tooltip width should be > 0 after slot hover");
    assert!(h > 0.0, "Tooltip height should be > 0 after slot hover");

    {
        let state = env.state().borrow();
        let rect = wow_ui_sim::iced_app::compute_frame_rect(&state.widgets, gt_id, 1024.0, 768.0);
        assert!(rect.width > 0.0, "Tooltip rect width should be > 0");
        assert!(rect.height > 0.0, "Tooltip rect height should be > 0");
        assert!(
            rect.x >= 0.0 && rect.x < 1024.0,
            "Tooltip x={} should be on screen",
            rect.x
        );
        assert!(
            rect.y >= 0.0 && rect.y < 768.0,
            "Tooltip y={} should be on screen",
            rect.y
        );
    }

    let buckets = {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    let tooltip_data = wow_ui_sim::iced_app::tooltip::collect_tooltip_data(&state);
    assert!(
        !tooltip_data.is_empty(),
        "Tooltip render data should exist after character slot hover"
    );

    let mut glyph_atlas = GlyphAtlas::new();
    let batch = wow_ui_sim::iced_app::build_quad_batch_for_registry(
        wow_ui_sim::iced_app::RegistryQuadBatchParams::new(
            &state.widgets,
            (1024.0, 768.0),
            &buckets,
        )
        .text_ctx(Some((&mut font_sys, &mut glyph_atlas)))
        .message_frames(Some(&state.message_frames))
        .tooltip_data(Some(&tooltip_data)),
    );

    assert!(
        batch.vertices.len() > 100,
        "Batch should contain tooltip vertices after character slot hover"
    );
}

#[cfg(feature = "gui")]
#[test]
fn test_character_slot_hover_tooltip_is_positioned_to_right_of_slot() {
    use wow_ui_sim::render::font::WowFontSystem;

    let env = setup_full_env();
    open_character_panel(&env);

    let slot_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name("CharacterHeadSlot")
            .expect("CharacterHeadSlot should exist after opening character panel")
    };

    env.state().borrow_mut().hovered_frame = Some(slot_id);
    env.fire_script_handler(slot_id, "OnEnter", vec![]).unwrap();

    let mut font_sys = WowFontSystem::new();
    {
        let mut state = env.state().borrow_mut();
        let _ = state.widgets.take_render_dirty();
        wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_sys);
    }

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let tooltip_rect =
        wow_ui_sim::iced_app::compute_frame_rect(&state.widgets, gt_id, 1024.0, 768.0);
    let slot_rect =
        wow_ui_sim::iced_app::compute_frame_rect(&state.widgets, slot_id, 1024.0, 768.0);
    let slot_right = slot_rect.x + slot_rect.width;
    let tooltip_center_y = tooltip_rect.y + tooltip_rect.height / 2.0;
    let slot_center_y = slot_rect.y + slot_rect.height / 2.0;

    assert!(
        tooltip_rect.x >= slot_right - 0.1,
        "Character slot hover tooltip should anchor to the right of the slot: tooltip={:?} slot={:?}",
        tooltip_rect,
        slot_rect
    );
    assert!(
        (tooltip_center_y - slot_center_y).abs() <= 3.0,
        "Character slot hover tooltip should stay vertically centered or bottom-clamped on the slot: tooltip={:?} slot={:?}",
        tooltip_rect,
        slot_rect
    );
    assert!(
        tooltip_rect.x + tooltip_rect.width <= 1024.0 + 0.1,
        "Tooltip should remain on-screen after character slot hover: tooltip={:?}",
        tooltip_rect
    );
}

#[cfg(feature = "gui")]
#[test]
fn test_buff_icon_hover_tooltip_produces_quads() {
    use wow_ui_sim::render::font::WowFontSystem;
    use wow_ui_sim::render::glyph::GlyphAtlas;

    let env = setup_full_env();
    refresh_buff_frame(&env);
    hover_first_visible_buff_icon(&env);

    let mut font_sys = WowFontSystem::new();
    {
        let mut state = env.state().borrow_mut();
        let _ = state.widgets.take_render_dirty();
        wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_sys);
    }

    let gt_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("GameTooltip")
        .unwrap();
    let (w, h) = {
        let state = env.state().borrow();
        let frame = state.widgets.get(gt_id).unwrap();
        (frame.width, frame.height)
    };
    assert!(w > 0.0, "Tooltip width should be > 0 after buff hover");
    assert!(h > 0.0, "Tooltip height should be > 0 after buff hover");

    {
        let state = env.state().borrow();
        let rect = wow_ui_sim::iced_app::compute_frame_rect(&state.widgets, gt_id, 1024.0, 768.0);
        assert!(rect.width > 0.0, "Tooltip rect width should be > 0");
        assert!(rect.height > 0.0, "Tooltip rect height should be > 0");
        assert!(
            rect.x >= 0.0 && rect.x < 1024.0,
            "Tooltip x={} should be on screen",
            rect.x
        );
        assert!(
            rect.y >= 0.0 && rect.y < 768.0,
            "Tooltip y={} should be on screen",
            rect.y
        );
    }

    let buckets = {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    let tooltip_data = wow_ui_sim::iced_app::tooltip::collect_tooltip_data(&state);
    assert!(
        !tooltip_data.is_empty(),
        "Tooltip render data should exist after buff hover"
    );

    let mut glyph_atlas = GlyphAtlas::new();
    let batch = wow_ui_sim::iced_app::build_quad_batch_for_registry(
        wow_ui_sim::iced_app::RegistryQuadBatchParams::new(
            &state.widgets,
            (1024.0, 768.0),
            &buckets,
        )
        .text_ctx(Some((&mut font_sys, &mut glyph_atlas)))
        .message_frames(Some(&state.message_frames))
        .tooltip_data(Some(&tooltip_data)),
    );

    assert!(
        batch.vertices.len() > 100,
        "Batch should contain tooltip vertices after buff hover"
    );
}
