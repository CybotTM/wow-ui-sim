//! Full-environment tooltip tests requiring Blizzard addon loading.
//!
//! Tests that need the micro menu, OnEnter scripts, or the render pipeline
//! live here. Basic tooltip API tests stay in tooltip.rs.

mod tooltip_full_env_helpers;
mod tooltip_hover_helpers;

use tooltip_full_env_helpers::setup_full_env;
use tooltip_hover_helpers::{
    hover_first_visible_buff_icon, open_character_panel, refresh_buff_frame,
};

#[test]
fn test_tooltip_mixins_load_in_full_env() {
    let env = setup_full_env();

    let tooltip_data_handler_mixin_type: String =
        env.eval("return type(TooltipDataHandlerMixin)").unwrap();
    let game_tooltip_data_mixin_type: String =
        env.eval("return type(GameTooltipDataMixin)").unwrap();
    let status_bar_onload_type: String = env
        .eval("return type(GameTooltipStatusBar.OnLoad)")
        .unwrap();
    let tooltip_onload_type: String = env.eval("return type(GameTooltip.OnLoad)").unwrap();

    assert_eq!(tooltip_data_handler_mixin_type, "table");
    assert_eq!(game_tooltip_data_mixin_type, "table");
    assert_eq!(status_bar_onload_type, "function");
    assert_eq!(tooltip_onload_type, "function");
}

#[test]
fn test_game_tooltip_process_info_with_constructed_tooltip_info_populates_lines() {
    let env = setup_full_env();

    let processed: bool = env
        .eval(
            r#"
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            local tooltipInfo = {
                getterName = "GetItemByID",
                getterArgs = { [1] = 6948, n = 1 },
            }
            return GameTooltip:ProcessInfo(tooltipInfo)
        "#,
        )
        .unwrap();

    assert!(
        processed,
        "GameTooltip:ProcessInfo should succeed for a valid item tooltipInfo"
    );

    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    assert!(
        num_lines > 0,
        "GameTooltip:ProcessInfo should populate tooltip lines for a valid tooltipInfo"
    );

    let first_line: String = env
        .eval("return GameTooltip:GetLeftLine(1):GetText()")
        .unwrap();
    assert_eq!(
        first_line, "Hearthstone",
        "GameTooltip:ProcessInfo should set the first tooltip line from the constructed tooltipInfo"
    );
}

/// Test that hovering a micro menu button shows the tooltip with text.
///
/// Uses the full Blizzard UI environment so OnEnter scripts run properly.
#[test]
fn test_micro_menu_hover_shows_tooltip() {
    let env = setup_full_env();

    // Find the CharacterMicroButton frame ID
    let btn_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name("CharacterMicroButton")
            .expect("CharacterMicroButton should exist")
    };

    // Set hovered_frame so IsMouseMotionFocus() returns true
    env.state().borrow_mut().hovered_frame = Some(btn_id);

    // Fire OnEnter (this is what handle_mouse_move does)
    env.fire_script_handler(btn_id, "OnEnter", vec![]).unwrap();

    // Check tooltip state
    let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();

    assert!(
        visible,
        "GameTooltip should be visible after micro menu hover"
    );
    assert!(
        num_lines > 0,
        "GameTooltip should have at least one line, got {}",
        num_lines
    );

    // Verify the tooltip text content
    {
        let state = env.state().borrow();
        let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
        let td = state
            .tooltips
            .get(&gt_id)
            .expect("tooltip data should exist");
        assert!(!td.lines.is_empty(), "tooltip should have line data");
        eprintln!("Tooltip text: {:?}", td.lines[0].left_text);
    }

    // Propagate effective_alpha via get_strata_buckets, then verify
    {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
    }
    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    assert!(
        state
            .widgets
            .get(gt_id)
            .is_some_and(|f| f.effective_alpha > 0.0),
        "GameTooltip should be ancestor-visible (effective_alpha > 0)"
    );

    // Check frame dimensions (tooltip should not be 0x0)
    let frame = state.widgets.get(gt_id).unwrap();
    eprintln!(
        "Tooltip frame: visible={}, width={}, height={}",
        frame.visible, frame.width, frame.height
    );
}

#[test]
fn test_character_slot_hover_shows_inventory_tooltip() {
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

    let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();

    assert!(
        visible,
        "GameTooltip should be visible after hovering CharacterHeadSlot"
    );
    assert!(
        num_lines >= 3,
        "Character slot hover should populate item tooltip lines, got {}",
        num_lines
    );

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state
        .tooltips
        .get(&gt_id)
        .expect("tooltip data should exist after character slot hover");
    assert_eq!(td.lines[0].left_text, "Entombed Seraph's Casque");
    assert_eq!(td.lines[1].left_text, "Item Level 571");
    assert_eq!(td.lines[2].left_text, "Head");
}

#[test]
fn test_buff_icon_hover_shows_aura_tooltip() {
    let env = setup_full_env();
    refresh_buff_frame(&env);

    let expected_name: String = env
        .eval(
            r#"
            for _, button in ipairs(BuffFrame.auraFrames) do
                if button:IsShown() and button.buttonInfo and button.buttonInfo.index then
                    button:OnEnter()
                    return C_TooltipInfo.GetUnitAura("player", button.buttonInfo.index, button:GetFilter()).lines[1].leftText
                end
            end
            error("No visible buff icon with tooltip data")
            "#,
        )
        .unwrap();

    let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();

    assert!(
        visible,
        "GameTooltip should be visible after hovering a buff icon"
    );
    assert!(
        num_lines >= 1,
        "Buff icon hover should populate aura tooltip lines, got {}",
        num_lines
    );

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state
        .tooltips
        .get(&gt_id)
        .expect("tooltip data should exist after buff hover");
    assert_eq!(td.lines[0].left_text, expected_name);
}

#[test]
fn test_world_loot_object_tooltip_shows_world_loot_data() {
    let env = setup_full_env();

    env.exec(
        r#"
        GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
        assert(GameTooltip.SetWorldLootObject, "GameTooltip:SetWorldLootObject should exist")
        local shown = GameTooltip:SetWorldLootObject("player")
        assert(shown == true, "SetWorldLootObject should report success for the supported unit")
        "#,
    )
    .expect("Failed to show the world loot object tooltip");

    let visible: bool = env.eval("return GameTooltip:IsVisible()").unwrap();
    let num_lines: i32 = env.eval("return GameTooltip:NumLines()").unwrap();
    let getter_name: String = env
        .eval(
            r#"
            local info = GameTooltip:GetPrimaryTooltipInfo()
            return info and info.getterName or ""
            "#,
        )
        .unwrap();

    assert!(
        visible,
        "GameTooltip should be visible after SetWorldLootObject"
    );
    assert!(
        num_lines >= 1,
        "GameTooltip should have world-loot tooltip lines, got {}",
        num_lines
    );
    assert_eq!(getter_name, "GetWorldLootObject");

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state
        .tooltips
        .get(&gt_id)
        .expect("tooltip data should exist after SetWorldLootObject");
    assert!(!td.lines[0].left_text.is_empty());
}

#[test]
fn test_tooltip_comparison_manager_get_comparison_item_data_uses_guid_lookup() {
    let env = setup_full_env();

    let has_real_tooltip: bool = env
        .eval(
            r#"
            assert(type(TooltipComparisonManager) == "table", "TooltipComparisonManager should be loaded")

            local guid = C_Item.GetItemGUID({ bagID = 0, slotIndex = 1 })
            local tooltipData = TooltipComparisonManager:GetComparisonItemData({ guid = guid })
            if not guid or guid == "" or not tooltipData or tooltipData.type ~= Enum.TooltipDataType.Item or not tooltipData.lines then
                return false
            end

            local nameLine = tooltipData.lines[1]
            return tooltipData.guid == guid
                and nameLine
                and nameLine.type == Enum.TooltipDataLineType.ItemName
                and nameLine.leftText == "Hearthstone"
            "#,
        )
        .unwrap();

    assert!(
        has_real_tooltip,
        "TooltipComparisonManager should resolve GUID-backed comparison items through C_TooltipInfo.GetItemByGUID",
    );
}

#[test]
fn test_tooltip_data_handler_set_owned_item_by_id_uses_owned_item_getter() {
    let env = setup_full_env();

    let has_real_tooltip: bool = env
        .eval(
            r#"
            assert(GameTooltip.SetOwnedItemByID, "GameTooltip:SetOwnedItemByID should exist")
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            GameTooltip:SetOwnedItemByID(6948)

            local info = GameTooltip:GetPrimaryTooltipInfo()
            local tooltipData = GameTooltip:GetPrimaryTooltipData()
            if not info or not tooltipData or tooltipData.type ~= Enum.TooltipDataType.Item or not tooltipData.lines then
                return false
            end

            local nameLine = tooltipData.lines[1]
            return info.getterName == "GetOwnedItemByID"
                and nameLine
                and nameLine.type == Enum.TooltipDataLineType.ItemName
                and nameLine.leftText == "Hearthstone"
            "#,
        )
        .unwrap();

    assert!(
        has_real_tooltip,
        "TooltipDataHandler should populate GameTooltip:SetOwnedItemByID through C_TooltipInfo.GetOwnedItemByID",
    );
}

#[test]
fn test_tooltip_data_handler_set_inventory_item_uses_inventory_getter() {
    let env = setup_full_env();

    let has_real_tooltip: bool = env
        .eval(
            r#"
            assert(GameTooltip.SetInventoryItem, "GameTooltip:SetInventoryItem should exist")
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            local ok = GameTooltip:SetInventoryItem("player", 1)
            if not ok then
                return false
            end

            local info = GameTooltip:GetPrimaryTooltipInfo()
            local tooltipData = GameTooltip:GetPrimaryTooltipData()
            if not info or not tooltipData or tooltipData.type ~= Enum.TooltipDataType.Item or not tooltipData.lines then
                return false
            end

            local nameLine = tooltipData.lines[1]
            return info.getterName == "GetInventoryItem"
                and nameLine
                and nameLine.type == Enum.TooltipDataLineType.ItemName
                and nameLine.leftText == "Entombed Seraph's Casque"
            "#,
        )
        .unwrap();

    assert!(
        has_real_tooltip,
        "TooltipDataHandler should populate GameTooltip:SetInventoryItem through C_TooltipInfo.GetInventoryItem",
    );
}

#[test]
fn test_tooltip_data_handler_set_recipe_result_item_uses_recipe_getters() {
    let env = setup_full_env();

    let recipe_result_ok: bool = env
        .eval(
            r#"
            assert(GameTooltip.SetRecipeResultItem, "GameTooltip:SetRecipeResultItem should exist")
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            GameTooltip:SetRecipeResultItem(100005, {}, nil, nil, nil)

            local info = GameTooltip:GetPrimaryTooltipInfo()
            local tooltipData = GameTooltip:GetPrimaryTooltipData()
            if not info or not tooltipData or tooltipData.type ~= Enum.TooltipDataType.Item or not tooltipData.lines then
                return false
            end

            local nameLine = tooltipData.lines[1]
            return info.getterName == "GetRecipeResultItem"
                and nameLine
                and nameLine.type == Enum.TooltipDataLineType.ItemName
                and nameLine.leftText == "Ordained Forge Maul"
            "#,
        )
        .unwrap();

    let order_result_ok: bool = env
        .eval(
            r#"
            assert(GameTooltip.SetRecipeResultItemForOrder, "GameTooltip:SetRecipeResultItemForOrder should exist")
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            GameTooltip:SetRecipeResultItemForOrder(100005, {}, 1, nil, nil)

            local info = GameTooltip:GetPrimaryTooltipInfo()
            local tooltipData = GameTooltip:GetPrimaryTooltipData()
            if not info or not tooltipData or tooltipData.type ~= Enum.TooltipDataType.Item or not tooltipData.lines then
                return false
            end

            local nameLine = tooltipData.lines[1]
            return info.getterName == "GetRecipeResultItemForOrder"
                and nameLine
                and nameLine.leftText == "Ordained Forge Maul"
            "#,
        )
        .unwrap();

    assert!(
        recipe_result_ok && order_result_ok,
        "TooltipDataHandler should populate both recipe result item accessors through C_TooltipInfo",
    );
}

#[test]
fn test_tooltip_data_handler_set_minimap_mouseover_uses_minimap_getter() {
    let env = setup_full_env();

    let has_real_tooltip: bool = env
        .eval(
            r#"
            assert(GameTooltip.SetMinimapMouseover, "GameTooltip:SetMinimapMouseover should exist")
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            GameTooltip:SetMinimapMouseover()

            local info = GameTooltip:GetPrimaryTooltipInfo()
            local tooltipData = GameTooltip:GetPrimaryTooltipData()
            if not info or not tooltipData or tooltipData.type ~= Enum.TooltipDataType.MinimapMouseover or not tooltipData.lines then
                return false
            end

            local zoneLine = tooltipData.lines[1]
            local subZoneLine = tooltipData.lines[2]
            return info.getterName == "GetMinimapMouseover"
                and zoneLine
                and zoneLine.leftText == "Stormwind City"
                and subZoneLine
                and subZoneLine.leftText == "Trade District"
            "#,
        )
        .unwrap();

    assert!(
        has_real_tooltip,
        "TooltipDataHandler should populate GameTooltip:SetMinimapMouseover through C_TooltipInfo.GetMinimapMouseover",
    );
}

#[test]
fn test_tooltip_data_handler_set_upgrade_item_uses_upgrade_getter() {
    let env = setup_full_env();

    let has_real_tooltip: bool = env
        .eval(
            r#"
            C_ItemUpgrade.SetItemUpgradeFromLocation({ bagID = 0, slotIndex = 1 })
            assert(GameTooltip.SetUpgradeItem, "GameTooltip:SetUpgradeItem should exist")
            GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
            GameTooltip:SetUpgradeItem()

            local info = GameTooltip:GetPrimaryTooltipInfo()
            local tooltipData = GameTooltip:GetPrimaryTooltipData()
            if not info or not tooltipData or tooltipData.type ~= Enum.TooltipDataType.Item or not tooltipData.lines then
                return false
            end

            local nameLine = tooltipData.lines[1]
            local itemLevelLine = tooltipData.lines[2]
            return info.getterName == "GetUpgradeItem"
                and nameLine
                and nameLine.leftText == "Hearthstone"
                and itemLevelLine
                and itemLevelLine.leftText == "Item Level 1"
            "#,
        )
        .unwrap();

    assert!(
        has_real_tooltip,
        "TooltipDataHandler should populate GameTooltip:SetUpgradeItem through C_TooltipInfo.GetUpgradeItem",
    );
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
