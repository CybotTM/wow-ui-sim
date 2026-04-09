//! Full-environment tooltip tests requiring Blizzard addon loading.
//!
//! Tests that need the micro menu, OnEnter scripts, or the render pipeline
//! live here. Basic tooltip API tests stay in tooltip.rs.

use wow_ui_sim::lua_api::WowLuaEnv;

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
fn test_world_cursor_tooltip_shows_world_cursor_data() {
    let env = setup_full_env();

    env.exec(
        r#"
        assert(GameTooltip.SetWorldCursor, "GameTooltip:SetWorldCursor should exist")
        GameTooltip:SetWorldCursor(Enum.WorldCursorAnchorType.Cursor)
        "#,
    )
    .expect("Failed to show the world cursor tooltip");

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
        "GameTooltip should be visible after SetWorldCursor"
    );
    assert!(
        num_lines >= 1,
        "GameTooltip should have world-cursor tooltip lines, got {}",
        num_lines
    );
    assert_eq!(getter_name, "GetWorldCursor");

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let td = state
        .tooltips
        .get(&gt_id)
        .expect("tooltip data should exist after SetWorldCursor");
    assert!(!td.lines[0].left_text.is_empty());
}

/// Verify the tooltip produces render quads after the full rendering pipeline runs.
#[cfg(feature = "gui")]
#[test]
fn test_tooltip_produces_quads_after_hover() {
    use std::path::PathBuf;
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
    let mut font_sys = WowFontSystem::new(&PathBuf::from("./fonts"));
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
        &state.widgets,
        (1024.0, 768.0),
        None,
        None,
        None,
        Some((&mut font_sys, &mut glyph_atlas)),
        Some(&state.message_frames),
        Some(&tooltip_data),
        &buckets,
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
    use std::path::PathBuf;
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

    let mut font_sys = WowFontSystem::new(&PathBuf::from("./fonts"));
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
        &state.widgets,
        (1024.0, 768.0),
        None,
        None,
        None,
        Some((&mut font_sys, &mut glyph_atlas)),
        Some(&state.message_frames),
        Some(&tooltip_data),
        &buckets,
    );

    assert!(
        batch.vertices.len() > 100,
        "Batch should contain tooltip vertices after character slot hover"
    );
}

#[cfg(feature = "gui")]
#[test]
fn test_buff_icon_hover_tooltip_produces_quads() {
    use std::path::PathBuf;
    use wow_ui_sim::render::font::WowFontSystem;
    use wow_ui_sim::render::glyph::GlyphAtlas;

    let env = setup_full_env();
    refresh_buff_frame(&env);
    hover_first_visible_buff_icon(&env);

    let mut font_sys = WowFontSystem::new(&PathBuf::from("./fonts"));
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
        &state.widgets,
        (1024.0, 768.0),
        None,
        None,
        None,
        Some((&mut font_sys, &mut glyph_atlas)),
        Some(&state.message_frames),
        Some(&tooltip_data),
        &buckets,
    );

    assert!(
        batch.vertices.len() > 100,
        "Batch should contain tooltip vertices after buff hover"
    );
}

const TOOLTIP_TEST_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    (
        "Blizzard_SharedXMLGame",
        "Blizzard_SharedXMLGame_Mainline.toc",
    ),
    (
        "Blizzard_UIPanelTemplates",
        "Blizzard_UIPanelTemplates_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLBase",
        "Blizzard_FrameXMLBase_Mainline.toc",
    ),
    ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
    ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
    ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
    (
        "Blizzard_AccessibilityTemplates",
        "Blizzard_AccessibilityTemplates.toc",
    ),
    ("Blizzard_ObjectAPI", "Blizzard_ObjectAPI_Mainline.toc"),
    ("Blizzard_UIParent", "Blizzard_UIParent_Mainline.toc"),
    ("Blizzard_TextStatusBar", "Blizzard_TextStatusBar.toc"),
    ("Blizzard_MoneyFrame", "Blizzard_MoneyFrame_Mainline.toc"),
    ("Blizzard_POIButton", "Blizzard_POIButton.toc"),
    ("Blizzard_Flyout", "Blizzard_Flyout.toc"),
    ("Blizzard_StoreUI", "Blizzard_StoreUI_Mainline.toc"),
    ("Blizzard_MicroMenu", "Blizzard_MicroMenu_Mainline.toc"),
    ("Blizzard_EditMode", "Blizzard_EditMode.toc"),
    ("Blizzard_Minimap", "Blizzard_Minimap_Mainline.toc"),
    ("Blizzard_BuffFrame", "Blizzard_BuffFrame.toc"),
    ("Blizzard_GarrisonBase", "Blizzard_GarrisonBase.toc"),
    ("Blizzard_GameTooltip", "Blizzard_GameTooltip_Mainline.toc"),
    (
        "Blizzard_UIParentPanelManager",
        "Blizzard_UIParentPanelManager_Mainline.toc",
    ),
    (
        "Blizzard_Settings_Shared",
        "Blizzard_Settings_Shared_Mainline.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Shared",
        "Blizzard_SettingsDefinitions_Shared.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Frame",
        "Blizzard_SettingsDefinitions_Frame_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLUtil",
        "Blizzard_FrameXMLUtil_Mainline.toc",
    ),
    ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
    ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    (
        "Blizzard_UIPanels_Game",
        "Blizzard_UIPanels_Game_Mainline.toc",
    ),
    ("Blizzard_TokenUI", "Blizzard_TokenUI.toc"),
    ("Blizzard_ActionBar", "Blizzard_ActionBar_Mainline.toc"),
];

fn setup_full_env() -> WowLuaEnv {
    use std::path::PathBuf;

    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1024.0, 768.0);

    let ui = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI");
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];

    load_blizzard_addons(&env, &ui);
    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

fn load_blizzard_addons(env: &WowLuaEnv, ui: &std::path::Path) {
    use wow_ui_sim::loader::{find_toc_file, load_addon};

    for (name, toc) in TOOLTIP_TEST_ADDONS {
        let addon_dir = ui.join(name);
        let requested_toc = addon_dir.join(toc);
        let toc_path = if requested_toc.exists() {
            requested_toc
        } else if let Some(discovered_toc) = find_toc_file(&addon_dir) {
            discovered_toc
        } else {
            continue;
        };
        if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }
}

fn fire_startup_events(env: &WowLuaEnv) {
    ensure_player_frame_for_aura_tests(env);

    let lua = env.lua();
    let _ = env.fire_event_with_args(
        "ADDON_LOADED",
        &[mlua::Value::String(lua.create_string("WoWUISim").unwrap())],
    );
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    let _ = env.fire_event_with_args(
        "PLAYER_ENTERING_WORLD",
        &[mlua::Value::Boolean(true), mlua::Value::Boolean(false)],
    );
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }

    refresh_aura_frames(env);
}

fn open_character_panel(env: &WowLuaEnv) {
    env.exec(
        r#"
        local btn = CharacterMicroButton
        assert(btn, "CharacterMicroButton should exist")
        local onclick = btn:GetScript("OnClick")
        assert(onclick, "CharacterMicroButton should have an OnClick handler")
        onclick(btn, "LeftButton", false)
        assert(CharacterFrame and CharacterFrame:IsShown(), "CharacterFrame should be shown")
        assert(CharacterHeadSlot ~= nil, "CharacterHeadSlot should exist")
        "#,
    )
    .expect("Failed to open character panel");
}

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

fn hover_first_visible_buff_icon(env: &WowLuaEnv) {
    env.exec(HOVER_FIRST_VISIBLE_BUFF_ICON_LUA)
        .expect("Failed to hover a visible buff icon");
}

fn refresh_buff_frame(env: &WowLuaEnv) {
    refresh_aura_frames(env);
    wow_ui_sim::startup::seed_buff_durations(env);
}

fn ensure_player_frame_for_aura_tests(env: &WowLuaEnv) {
    env.exec(
        r#"
        if not PlayerFrame then
            PlayerFrame = CreateFrame("Frame", "PlayerFrame", UIParent)
        end
        PlayerFrame.unit = "player"
        "#,
    )
    .expect("Failed to create PlayerFrame stub for aura tests");
}

fn refresh_aura_frames(env: &WowLuaEnv) {
    env.exec(
        r#"
        assert(BuffFrame, "BuffFrame should exist")
        assert(BuffFrame.UpdateAuras, "BuffFrame should expose UpdateAuras")
        local updateInfo = { isFullUpdate = true }
        if BuffFrame:IsEventRegistered("UNIT_AURA") and BuffFrame:GetScript("OnEvent") then
            BuffFrame:GetScript("OnEvent")(BuffFrame, "UNIT_AURA", "player", updateInfo)
        end
        BuffFrame:UpdateAuras()
        BuffFrame:Update()
        "#,
    )
    .expect("Failed to refresh BuffFrame after seeding buffs");
}
