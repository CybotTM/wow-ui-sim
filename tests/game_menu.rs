//! Integration tests for GameMenuFrame.
//!
//! Verifies that GameMenuFrame exists, is hidden by default, creates its
//! buttons on Show (via InitButtons from MainMenuFrameMixin), lays them out
//! vertically, and lives in the DIALOG strata.

use crate::common;

use std::path::PathBuf;
use iced::Point;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::paths::default_blizzard_ui_addons_path;

fn blizzard_ui_dir() -> PathBuf {
    default_blizzard_ui_addons_path()
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI"))
}

/// Minimal addon set needed to load GameMenuFrame (mirrors test_keybindings.rs).
const BLIZZARD_ADDONS: &[(&str, &str)] = &[
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
    ("Blizzard_FrameEffects", "Blizzard_FrameEffects.toc"),
    ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
    ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
    ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
    ("Blizzard_Menu", "Blizzard_Menu.toc"),
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
    ("Blizzard_StaticPopup", "Blizzard_StaticPopup.toc"),
    ("Blizzard_SimpleCheckout", "Blizzard_SimpleCheckout.toc"),
    ("Blizzard_StoreUI", "Blizzard_StoreUI_Mainline.toc"),
    ("Blizzard_MicroMenu", "Blizzard_MicroMenu_Mainline.toc"),
    ("Blizzard_EditMode", "Blizzard_EditMode.toc"),
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
    (
        "Blizzard_MapCanvasSecureUtil",
        "Blizzard_MapCanvasSecureUtil.toc",
    ),
    ("Blizzard_MapCanvas", "Blizzard_MapCanvas.toc"),
    (
        "Blizzard_SharedMapDataProviders",
        "Blizzard_SharedMapDataProviders_Mainline.toc",
    ),
    ("Blizzard_WorldMap", "Blizzard_WorldMap_Mainline.toc"),
    ("Blizzard_ActionBar", "Blizzard_ActionBar_Mainline.toc"),
    ("Blizzard_GameMenu", "Blizzard_GameMenu_Mainline.toc"),
];

fn setup_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    let ui = blizzard_ui_dir();
    for (name, toc) in BLIZZARD_ADDONS {
        let toc_path = ui.join(name).join(toc);
        assert!(
            toc_path.exists(),
            "missing {name} fixture TOC at {}",
            toc_path.display()
        );
        load_addon(&env.loader_env(), &toc_path)
            .unwrap_or_else(|error| panic!("[load {name}] FAILED: {error}"));
    }

    env.apply_post_load_workarounds();

    common::fire_addon_loaded(&env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::fire_player_entering_world(&env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }

    env
}

// ── Existence ───────────────────────────────────────────────────────────────

#[test]
fn test_game_menu_frame_exists() {
    test_timeout! {
        let env = setup_env();
        let exists: bool = env.eval("return GameMenuFrame ~= nil").unwrap();
        assert!(exists, "GameMenuFrame should exist as a global");

        let obj_type: String = env.eval("return GameMenuFrame:GetObjectType()").unwrap();
        assert_eq!(obj_type, "Frame");

        let is_shown: bool = env.eval("return GameMenuFrame:IsShown()").unwrap();
        assert!(!is_shown, "GameMenuFrame should be hidden by default");
    }
}

// ── Buttons created on Show ──────────────────────────────────────────────────

#[test]
fn test_game_menu_buttons_created_on_show() {
    test_timeout! {
        let env = setup_env();

        // Before Show() the pool should be empty.
        let before: i32 = env
            .eval(
                r#"
                local n = 0
                for _ in GameMenuFrame.buttonPool:EnumerateActive() do
                    n = n + 1
                end
                return n
            "#,
            )
            .unwrap();
        assert_eq!(before, 0, "buttonPool should be empty before Show()");

        env.exec("GameMenuFrame:Show()").unwrap();

        // Collect button texts from the active pool entries.
        let texts: String = env
            .eval(
                r#"
                local found = {}
                for btn in GameMenuFrame.buttonPool:EnumerateActive() do
                    table.insert(found, btn:GetText() or "")
                end
                table.sort(found)
                return table.concat(found, "|")
            "#,
            )
            .unwrap();

        // These localisation keys resolve to English strings in the simulator.
        // Verify the expected set is a subset of what was created.
        // Note: "Edit Mode" is absent because EditModeManagerFrame:CanEnterEditMode()
        // returns false in the headless simulator.
        let expected = [
            "Options",
            "AddOns",
            "Support",
            "Macros",
            "Log Out",
            "Exit Game",
            "Return to Game",
        ];
        for label in &expected {
            assert!(
                texts.contains(label),
                "Expected button '{label}' in pool; got: {texts}"
            );
        }

        // At minimum 8 buttons (the required set above, Shop may be absent).
        let count: i32 = env
            .eval(
                r#"
                local n = 0
                for _ in GameMenuFrame.buttonPool:EnumerateActive() do n = n + 1 end
                return n
            "#,
            )
            .unwrap();
        assert!(
            count >= 8,
            "Expected at least 8 active buttons after Show(), got {count}"
        );
    }
}

#[test]
fn test_game_menu_exit_button_requests_simulator_quit() {
    test_timeout! {
        let env = setup_env();

        env.exec("GameMenuFrame:Show()").unwrap();

        let result: String = env
            .eval(
                r#"
                local quitButton
                for btn in GameMenuFrame.buttonPool:EnumerateActive() do
                    if btn:GetText() == EXIT_GAME then
                        quitButton = btn
                        break
                    end
                end
                if not quitButton then
                    return "missing-quit-button"
                end

                local onclick = quitButton:GetScript("OnClick")
                if type(onclick) ~= "function" then
                    return "missing-onclick"
                end

                local ok, err = pcall(onclick, quitButton, "LeftButton", false)
                if not ok then
                    return "onclick-error:" .. tostring(err)
                end

                return A_Admin.IsSimulatorExitRequested() and "quit-requested" or "quit-not-requested"
            "#,
            )
            .unwrap();

        assert_eq!(result, "quit-requested");
    }
}

#[test]
fn test_game_menu_options_button_hitbox_matches_visual_center() {
    test_timeout! {
        let env = setup_env();
        env.exec("GameMenuFrame:Show()").unwrap();

        let button_debug_name: String = env
            .eval(
                r#"
                for btn in GameMenuFrame.buttonPool:EnumerateActive() do
                    if btn:GetText() == "Options" then
                        return btn:GetDebugName()
                    end
                end
                return ""
            "#,
            )
            .unwrap();
        let button_id: u64 = button_debug_name
            .rsplit_once(':')
            .and_then(|(_, id)| id.parse().ok())
            .unwrap_or_else(|| panic!("Options button debug name should end with an internal id, got {button_debug_name:?}"));

        let center = {
            let mut state = env.state().borrow_mut();
            state.ensure_layout_rects();
            let rect = state
                .widgets
                .get(button_id)
                .and_then(|frame| frame.layout_rect)
                .expect("Options game-menu button should have a resolved rect");
            let center = Point::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0);
            state.set_mouse_position(Some((center.x, center.y)));
            state.hovered_frame = Some(button_id);
            center
        };

        let result: String = env
            .eval(
                r#"
                for btn in GameMenuFrame.buttonPool:EnumerateActive() do
                    if btn:GetText() == "Options" then
                        if not btn:IsMouseOver() then
                            return "not_mouse_over"
                        end
                        if not btn:IsMouseMotionFocus() then
                            return "not_mouse_motion_focus"
                        end
                        return "ok"
                    end
                end
                return "missing_options_button"
            "#,
            )
            .unwrap();

        assert_eq!(
            result, "ok",
            "Options visual center should hit and hover its button at ({}, {}) for widget id {button_id}: {result}",
            center.x, center.y
        );
    }
}

// ── Vertical layout ──────────────────────────────────────────────────────────

#[test]
fn test_game_menu_button_positions_vertical_layout() {
    test_timeout! {
        let env = setup_env();
        env.exec("GameMenuFrame:Show()").unwrap();

        // Trigger layout so GetTop()/GetBottom() are populated.
        {
            let mut state = env.state().borrow_mut();
            state.ensure_layout_rects();
        }

        // Collect (layoutIndex, top) for each active button and verify ordering.
        // In a VerticalLayoutFrame, higher layoutIndex means lower on screen
        // (smaller Top value in WoW's coordinate system where Y grows upward).
        let result: String = env
            .eval(
                r#"
                local buttons = {}
                for btn in GameMenuFrame.buttonPool:EnumerateActive() do
                    table.insert(buttons, {idx = btn.layoutIndex, top = btn:GetTop()})
                end
                table.sort(buttons, function(a, b) return a.idx < b.idx end)

                -- Build a verification string: "ok" if each successive button's
                -- top is <= the previous top, "FAIL:i" if not.
                local result = "ok"
                for i = 2, #buttons do
                    local prev = buttons[i-1].top
                    local curr = buttons[i].top
                    if prev ~= nil and curr ~= nil and curr > prev + 1 then
                        result = "FAIL:" .. i .. " prev=" .. tostring(prev) .. " curr=" .. tostring(curr)
                        break
                    end
                end
                return result
            "#,
            )
            .unwrap();
        assert_eq!(result, "ok", "Buttons are not laid out top-to-bottom: {result}");

        // All buttons should be within GameMenuFrame bounds (when layout is available).
        // GetLeft()/GetTop() return nil if the frame has no resolved layout yet;
        // skip the bounds check in that case.
        let bounds_result: String = env
            .eval(
                r#"
                local frameLeft   = GameMenuFrame:GetLeft()
                local frameRight  = GameMenuFrame:GetRight()
                local frameTop    = GameMenuFrame:GetTop()
                local frameBottom = GameMenuFrame:GetBottom()

                -- Layout not yet resolved — skip check.
                if not (frameLeft and frameRight and frameTop and frameBottom) then
                    return "skip"
                end

                for btn in GameMenuFrame.buttonPool:EnumerateActive() do
                    local l = btn:GetLeft()
                    local r = btn:GetRight()
                    local t = btn:GetTop()
                    local b = btn:GetBottom()
                    -- Button not yet laid out — skip this one.
                    if l and r and t and b then
                        -- Allow 2px tolerance for padding/border.
                        if l < frameLeft - 2 or r > frameRight + 2
                           or t > frameTop + 2 or b < frameBottom - 2 then
                            return "FAIL:btn out of bounds l=" .. l .. " r=" .. r
                                .. " t=" .. t .. " b=" .. b
                                .. " frame=" .. frameLeft .. ".." .. frameRight
                                .. " " .. frameBottom .. ".." .. frameTop
                        end
                    end
                end
                return "ok"
            "#,
            )
            .unwrap();
        assert!(
            bounds_result == "ok" || bounds_result == "skip",
            "One or more buttons lie outside GameMenuFrame bounds: {bounds_result}"
        );
    }
}

// ── Strata ───────────────────────────────────────────────────────────────────

#[test]
fn test_game_menu_frame_strata() {
    test_timeout! {
        let env = setup_env();
        let strata: String = env
            .eval("return GameMenuFrame:GetFrameStrata()")
            .unwrap();
        assert_eq!(strata, "DIALOG", "GameMenuFrame should be in DIALOG strata");
    }
}

// ── Hide ─────────────────────────────────────────────────────────────────────

#[test]
fn test_game_menu_hide() {
    test_timeout! {
        let env = setup_env();

        env.exec("GameMenuFrame:Show()").unwrap();
        let shown: bool = env.eval("return GameMenuFrame:IsShown()").unwrap();
        assert!(shown, "GameMenuFrame should be shown after Show()");

        env.exec("GameMenuFrame:Hide()").unwrap();
        let hidden: bool = env.eval("return not GameMenuFrame:IsShown()").unwrap();
        assert!(hidden, "GameMenuFrame should be hidden after Hide()");

        // Pool is released on Reset() which is called on next Show(); buttons
        // remain in the pool after a simple Hide() but are still "active".
        // Verify the frame itself reports hidden.
        let is_visible: bool = env.eval("return GameMenuFrame:IsVisible()").unwrap();
        assert!(!is_visible, "GameMenuFrame:IsVisible() should be false after Hide()");
    }
}

#[test]
fn test_game_menu_shop_button_opens_store_and_menu_can_reopen_after_store_closes() {
    test_timeout! {
        let env = setup_env();

        let result: String = env
            .eval(
                r#"
                GameMenuFrame:Show()

                local shopButton
                for button in GameMenuFrame.buttonPool:EnumerateActive() do
                    local text = button:GetText()
                    if text == BLIZZARD_STORE then
                        shopButton = button
                        break
                    end
                end
                if not shopButton then
                    return "missing-shop-button"
                end

                local onclick = shopButton:GetScript("OnClick")
                if type(onclick) ~= "function" then
                    return "missing-onclick"
                end

                local ok, err = pcall(onclick, shopButton, "LeftButton", false)
                if not ok then
                    return "onclick-error:" .. tostring(err)
                end

                if not StoreFrame:IsShown() then
                    return "store-not-shown action=" .. tostring(StoreFrame:GetAttribute("action"))
                end

                StoreFrame_SetShown(false, G_GameMenuFrameContextKey)
                if StoreFrame:IsShown() then
                    return "store-still-shown-after-close"
                end

                if not GameMenuFrame:IsShown() then
                    ToggleGameMenu()
                    if not GameMenuFrame:IsShown() then
                        return "menu-did-not-reopen"
                    end
                    return "menu-reopened-only-after-toggle"
                end

                return "ok"
                "#,
            )
            .unwrap();

        assert_eq!(result, "ok");
    }
}
