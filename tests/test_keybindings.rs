//! Integration tests for keybinding dispatch against the real Blizzard UI.
//!
//! Loads the full Blizzard addon set, fires startup events, then presses each
//! default keybind and verifies the corresponding panel frame is shown.
//!
//! These tests exercise the real Blizzard toggle functions (ToggleAllBags,
//! ToggleCharacter, etc.) — not stubs. Failures surface real missing APIs,
//! nil widget errors, and broken on-demand addon loads.
//!
//! Panel interaction tests (spellbook, talents, world map, escape menu, etc.)
//! are in `test_keybindings_panels.rs`.
//!
//! Targeting tests (TargetFrame, F2–F6) are in `test_keybindings_targeting.rs`.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::globals::global_frames;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Blizzard addons in dependency order (same as micro_menu.rs).
const BLIZZARD_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    ("Blizzard_SharedXMLGame", "Blizzard_SharedXMLGame.toc"),
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
    ("Blizzard_FrameXMLUtil", "Blizzard_FrameXMLUtil.toc"),
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
    ("Blizzard_UIWidgets", "Blizzard_UIWidgets_Mainline.toc"),
    ("Blizzard_Minimap", "Blizzard_Minimap_Mainline.toc"),
    ("Blizzard_AddOnList", "Blizzard_AddOnList.toc"),
    ("Blizzard_TimerunningUtil", "Blizzard_TimerunningUtil.toc"),
    ("Blizzard_Communities", "Blizzard_Communities_Mainline.toc"),
];

/// Create a fully loaded environment with Blizzard addons and startup events.
fn setup_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    // Set addon_base_paths for runtime on-demand loading
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    // Load base Blizzard addons
    let ui = blizzard_ui_dir();
    for (name, toc) in BLIZZARD_ADDONS {
        let toc_path = ui.join(name).join(toc);
        if !toc_path.exists() {
            continue;
        }
        if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }

    load_token_ui(&env);
    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

/// Fire startup events (same sequence as main.rs).
fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::call_global_if_present(env, "RequestTimePlayed");
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
        "PLAYER_LEAVING_WORLD",
    ] {
        let _ = env.fire_event(event);
    }
}

fn load_token_ui(env: &WowLuaEnv) {
    env.exec(
        r#"
        local loaded, reason = LoadAddOn("Blizzard_TokenUI")
        assert(loaded, "LoadAddOn(Blizzard_TokenUI) failed: " .. tostring(reason))
        if ContainerFrameSettingsManager and not ContainerFrameSettingsManager.TokenTracker then
            ContainerFrameSettingsManager:OnAddonLoaded("Blizzard_TokenUI")
        end
        assert(BackpackTokenFrame, "BackpackTokenFrame should exist after loading Blizzard_TokenUI")
        assert(
            ContainerFrameSettingsManager and ContainerFrameSettingsManager.TokenTracker == BackpackTokenFrame,
            "ContainerFrameSettingsManager should own BackpackTokenFrame after loading Blizzard_TokenUI"
        )
        "#,
    )
    .expect("Failed to runtime-load Blizzard_TokenUI for keybinding bag tests");
}

/// Check whether a global frame exists and is shown.
fn frame_is_shown(env: &WowLuaEnv, frame_name: &str) -> bool {
    let code = format!("return {frame_name} ~= nil and {frame_name}:IsShown() == true");
    env.eval::<bool>(&code).unwrap_or(false)
}

fn visible_bag_frames_debug(env: &WowLuaEnv) -> String {
    env.eval::<String>(
        r#"
        local parts = {}
        local function add(name)
            local frame = _G[name]
            if not frame then
                return
            end
            local shown = frame.IsShown and frame:IsShown()
            local bagID = frame.GetBagID and frame:GetBagID()
            table.insert(parts, string.format("%s(shown=%s, bagID=%s)", name, tostring(shown), tostring(bagID)))
        end

        add("ContainerFrameCombinedBags")
        for i = 1, 6 do
            add("ContainerFrame" .. i)
        end
        return table.concat(parts, ", ")
    "#,
    )
    .unwrap_or_else(|_| "<bag frame introspection failed>".to_string())
}

fn bag_id_is_shown(env: &WowLuaEnv, bag_id: i32) -> bool {
    env.eval::<bool>(&format!(
        r#"
        if ContainerFrameCombinedBags and ContainerFrameCombinedBags:IsShown() then
            return true
        end
        for i = 1, 6 do
            local frame = _G["ContainerFrame" .. i]
            if frame and frame:IsShown() and frame.GetBagID and frame:GetBagID() == {bag_id} then
                return true
            end
        end
        return false
    "#
    ))
    .unwrap_or(false)
}

/// Check whether a global frame exists.
#[allow(dead_code)]
fn frame_exists(env: &WowLuaEnv, frame_name: &str) -> bool {
    let code = format!("return {frame_name} ~= nil");
    env.eval::<bool>(&code).unwrap_or(false)
}

/// Install a Lua error handler that collects errors into `__test_errors`.
fn install_test_error_handler(env: &WowLuaEnv) {
    common::install_error_collector(env, "__test_errors");
}

/// Read collected errors from `__test_errors` and clear it.
fn drain_test_errors(env: &WowLuaEnv) -> Vec<String> {
    common::drain_string_table(env, "__test_errors")
}

/// Create environment with ALL Blizzard addons (including Blizzard_UnitFrame).
fn setup_full_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        if let Err(e) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }
    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env.apply_post_event_workarounds();
    let _ = global_frames::hide_runtime_hidden_frames(&*env.rilua());
    env
}

// ── B → ToggleAllBags() ─────────────────────────────────────────────────

#[test]
fn keybind_b_opens_bags() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("B", None).expect("B keybind failed");
        assert!(
            frame_is_shown(&env, "ContainerFrameCombinedBags")
                || frame_is_shown(&env, "ContainerFrame1"),
            "A bag frame should be visible after pressing B"
        );
    }
}

// ── BACKSPACE → ToggleBackpack() ────────────────────────────────────────

#[test]
fn keybind_backspace_opens_backpack() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("BACKSPACE", None).expect("BACKSPACE keybind failed");
        assert!(
            bag_id_is_shown(&env, 0),
            "Backpack should be visible after pressing BACKSPACE; {}",
            visible_bag_frames_debug(&env)
        );
    }
}

// ── F8 → ToggleBag(4) ──────────────────────────────────────────────

#[test]
fn keybind_f8_opens_bag4() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("F8", None).expect("F8 keybind failed");
        assert!(
            bag_id_is_shown(&env, 4),
            "A bag frame should be visible after pressing F8; {}",
            visible_bag_frames_debug(&env)
        );
    }
}

// ── F9 → ToggleBag(3) ──────────────────────────────────────────────

#[test]
fn keybind_f9_opens_bag3() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("F9", None).expect("F9 keybind failed");
        assert!(
            bag_id_is_shown(&env, 3),
            "A bag frame should be visible after pressing F9; {}",
            visible_bag_frames_debug(&env)
        );
    }
}

// ── F10 → ToggleBag(2) ─────────────────────────────────────────────

#[test]
fn keybind_f10_opens_bag2() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("F10", None).expect("F10 keybind failed");
        assert!(
            bag_id_is_shown(&env, 2),
            "A bag frame should be visible after pressing F10; {}",
            visible_bag_frames_debug(&env)
        );
    }
}

// ── F11 → ToggleBag(1) ─────────────────────────────────────────────

#[test]
fn keybind_f11_opens_bag1() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("F11", None).expect("F11 keybind failed");
        assert!(
            bag_id_is_shown(&env, 1),
            "A bag frame should be visible after pressing F11; {}",
            visible_bag_frames_debug(&env)
        );
    }
}

// ── C → ToggleCharacter("PaperDollFrame") ───────────────────────────────

#[test]
fn keybind_c_opens_character() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("C", None).expect("C keybind failed");
        assert!(
            frame_is_shown(&env, "CharacterFrame"),
            "CharacterFrame should be shown after pressing C"
        );
    }
}

#[test]
fn keybind_c_toggles_character_without_errors() {
    test_timeout! {
        let env = setup_full_env();
        install_test_error_handler(&env);

        env.send_key_press("C", None).expect("first C keybind failed");

        let open_errors = drain_test_errors(&env);
        assert!(
            open_errors.is_empty(),
            "Opening character panel produced {} Lua error(s):\n{}",
            open_errors.len(),
            open_errors.join("\n"),
        );
        assert!(
            frame_is_shown(&env, "CharacterFrame"),
            "CharacterFrame should be shown after first C press"
        );

        env.send_key_press("C", None).expect("second C keybind failed");

        let close_errors = drain_test_errors(&env);
        assert!(
            close_errors.is_empty(),
            "Closing character panel produced {} Lua error(s):\n{}",
            close_errors.len(),
            close_errors.join("\n"),
        );
        assert!(
            !frame_is_shown(&env, "CharacterFrame"),
            "CharacterFrame should be hidden after second C press"
        );
    }
}

#[test]
fn character_panel_inventory_tooltip_has_lines_and_closes_without_errors() {
    test_timeout! {
        let env = setup_full_env();
        install_test_error_handler(&env);

        let result: String = env
            .eval(
                r#"
                if type(ToggleCharacter) ~= "function" then
                    return "missing_toggle_character"
                end

                ToggleCharacter("PaperDollFrame")

                if not (CharacterFrame and CharacterFrame:IsShown()) then
                    return "character_not_open"
                end

                local hasItem = GameTooltip:SetInventoryItem("player", 1)
                if not hasItem then
                    return "no_inventory_item"
                end

                if GameTooltip:NumLines() == 0 then
                    return "tooltip_has_no_lines"
                end

                ToggleCharacter("PaperDollFrame")

                if CharacterFrame and CharacterFrame:IsShown() then
                    return "character_not_closed"
                end

                return "ok"
            "#,
            )
            .unwrap();

        let errors = drain_test_errors(&env);
        assert!(
            errors.is_empty(),
            "Character panel inventory tooltip flow produced {} Lua error(s):\n{}",
            errors.len(),
            errors.join("\n"),
        );
        assert_eq!(
            result,
            "ok",
            "Character panel inventory tooltip flow should open, populate tooltip lines, and close: {result}"
        );
    }
}

// ── U → ToggleCharacter("ReputationFrame") ──────────────────────────────

#[test]
fn keybind_u_opens_reputation() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("U", None).expect("U keybind failed");
        assert!(
            frame_is_shown(&env, "CharacterFrame"),
            "CharacterFrame should be shown after pressing U (reputation tab)"
        );
    }
}

#[test]
fn reputation_first_visible_line_matches_first_faction_name() {
    test_timeout! {
        let env = setup_env();

        env.send_key_press("U", None).expect("U keybind failed");

        let result: String = env
            .eval(
                r#"
                if not (CharacterFrame and CharacterFrame:IsShown()) then
                    return "character_frame_not_shown"
                end
                if not (ReputationFrame and ReputationFrame:IsShown()) then
                    return "reputation_frame_not_shown"
                end
                if not ReputationFrame.ScrollBox then
                    return "missing_reputation_scroll_box"
                end

                local expectedData = C_Reputation.GetFactionDataByIndex(1)
                if not expectedData then
                    return "missing_first_faction_data"
                end

                local firstVisible
                for _, frame in ReputationFrame.ScrollBox:EnumerateFrames() do
                    if frame and frame:IsShown() then
                        firstVisible = frame
                        break
                    end
                end

                if not firstVisible then
                    return "missing_first_visible_reputation_line"
                end

                local actualIndex = firstVisible.factionIndex
                    or (firstVisible.elementData and firstVisible.elementData.factionIndex)
                if actualIndex ~= 1 then
                    return string.format(
                        "first_visible_line_index_mismatch_expected_1_actual_%s",
                        tostring(actualIndex)
                    )
                end

                local nameRegion = firstVisible.Content and firstVisible.Content.Name or firstVisible.Name
                if not nameRegion then
                    return "missing_first_visible_line_name"
                end

                local actualName = nameRegion:GetText()
                if actualName ~= expectedData.name then
                    return string.format(
                        "first_visible_line_name_mismatch_expected_%s_actual_%s",
                        tostring(expectedData.name),
                        tostring(actualName)
                    )
                end

                return "ok"
            "#,
            )
            .unwrap();

        assert_eq!(
            result,
            "ok",
            "The first visible reputation line should match C_Reputation.GetFactionDataByIndex(1).name: {result}"
        );
    }
}
