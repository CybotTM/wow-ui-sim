//! Integration tests for keybinding dispatch — panel interaction tests.
//!
//! Covers spellbook, talents, collections, world map, escape menu, and social panels.

mod common;

use std::path::PathBuf;
use wow_ui_sim::iced_app::build_quad_batch_for_registry;
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

fn frame_is_shown(env: &WowLuaEnv, frame_name: &str) -> bool {
    let code = format!("return {frame_name} ~= nil and {frame_name}:IsShown() == true");
    env.eval::<bool>(&code).unwrap_or(false)
}

fn frame_is_visible(env: &WowLuaEnv, frame_name: &str) -> bool {
    let code = format!("return {frame_name} ~= nil and {frame_name}:IsVisible() == true");
    env.eval::<bool>(&code).unwrap_or(false)
}

fn build_batch_for_root(env: &WowLuaEnv, root_name: &str) -> wow_ui_sim::render::QuadBatch {
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
        Some(root_name),
        None,
        None,
        None,
        None,
        None,
        &buckets,
    )
}

fn install_test_error_handler(env: &WowLuaEnv) {
    common::install_error_collector(env, "__test_errors");
}

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

// ── S → PlayerSpellsUtil.ToggleSpellBookFrame() ─────────────────────────

#[test]
fn keybind_s_opens_spellbook() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("S", None).expect("S keybind failed");
        assert!(
            frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be shown after pressing S"
        );
        // ShowUIPanel should scale-to-fit; PlayerSpellsFrame keeps its default strata.
        let scale: f64 = env
            .eval("return PlayerSpellsFrame:GetScale()")
            .expect("GetScale failed");
        assert!(
            scale < 1.0,
            "1618px-wide frame at 1024px screen should be scaled down, got {scale}"
        );
        let strata: String = env
            .eval("return PlayerSpellsFrame:GetFrameStrata()")
            .expect("GetFrameStrata failed");
        assert_eq!(
            strata, "MEDIUM",
            "PlayerSpellsFrame should keep its default MEDIUM strata"
        );
    }
}

// ── N → PlayerSpellsUtil.ToggleClassTalentFrame() ───────────────────────

#[test]
fn keybind_n_opens_talents() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("N", None).expect("N keybind failed");
        assert!(
            frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be shown after pressing N (talents tab)"
        );
        assert!(
            !frame_is_shown(&env, "ClassTalentLoadoutImportDialog"),
            "ClassTalentLoadoutImportDialog should stay hidden until the Import action is clicked"
        );
        assert!(
            !frame_is_shown(&env, "ClassTalentLoadoutCreateDialog"),
            "ClassTalentLoadoutCreateDialog should stay hidden until the New Loadout action is clicked"
        );
        assert!(
            !frame_is_visible(&env, "ClassTalentLoadoutImportDialogImportControl"),
            "Import dialog content should not become visible when opening the talents tab"
        );
        assert!(
            !frame_is_visible(&env, "ClassTalentLoadoutImportDialogNameControl"),
            "Import dialog name control should not become visible when opening the talents tab"
        );
        assert!(
            !frame_is_visible(&env, "ClassTalentLoadoutCreateDialogNameControl"),
            "Create dialog content should not become visible when opening the talents tab"
        );
    }
}

#[test]
fn hidden_talent_dialogs_do_not_emit_quads_after_opening_talents() {
    test_timeout! {
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
        wow_ui_sim::startup::settle_headless_startup(&env);
        env.send_key_press("N", None).expect("N keybind failed");
        wow_ui_sim::startup::run_extra_update_ticks(&env, 3);

        let import_batch = build_batch_for_root(&env, "ClassTalentLoadoutImportDialog");
        assert_eq!(
            import_batch.quad_count(),
            12,
            "hidden import dialog should not emit quads"
        );
        assert_eq!(
            import_batch.texture_requests.len(),
            1,
            "hidden import dialog should only contribute the tiled background"
        );

        let hero_batch = build_batch_for_root(&env, "HeroTalentsSelectionDialog");
        assert_eq!(
            hero_batch.quad_count(),
            12,
            "hidden hero talents dialog should not emit quads"
        );
        assert_eq!(
            hero_batch.texture_requests.len(),
            1,
            "hidden hero talents dialog should only contribute the tiled background"
        );
    }
}

// ── A → ToggleAchievementFrame() ────────────────────────────────────────

#[test]
fn keybind_a_opens_achievements() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("A", None).expect("A keybind failed");
        assert!(
            frame_is_shown(&env, "AchievementFrame"),
            "AchievementFrame should be shown after pressing A"
        );
    }
}

// ── L → PVEFrame_ToggleFrame() ──────────────────────────────────────────

#[test]
fn keybind_l_opens_group_finder() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("L", None).expect("L keybind failed");
        assert!(
            frame_is_shown(&env, "PVEFrame"),
            "PVEFrame should be shown after pressing L"
        );
    }
}

// ── O → ToggleFriendsFrame() ────────────────────────────────────────────

#[test]
fn keybind_o_opens_social() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("O", None).expect("O keybind failed");
        assert!(
            frame_is_shown(&env, "FriendsFrame"),
            "FriendsFrame should be shown after pressing O"
        );
    }
}

#[test]
fn keybind_o_populates_friends_list_from_c_friend_list() {
    test_timeout! {
        let env = setup_env();

        env.send_key_press("O", None).expect("O keybind failed");
        let _ = build_batch_for_root(&env, "FriendsFrame");

        let result: String = env.eval(r#"
            if not FriendsFrame or not FriendsFrame:IsShown() then
                return "friends_frame_not_shown"
            end
            if C_FriendList.GetNumFriends() ~= 2 then
                return "friend_count=" .. tostring(C_FriendList.GetNumFriends())
            end
            if C_FriendList.GetNumOnlineFriends() ~= 1 then
                return "online_count=" .. tostring(C_FriendList.GetNumOnlineFriends())
            end

            local data_provider = FriendsListFrame.ScrollBox:GetDataProvider()
            if not data_provider then
                return "missing_data_provider"
            end
            if data_provider:GetSize() ~= 3 then
                return "data_provider_size=" .. tostring(data_provider:GetSize())
            end

            local online_friend = data_provider:FindElementDataByPredicate(function(elementData)
                return elementData.buttonType == FRIENDS_BUTTON_TYPE_WOW and elementData.id == 1
            end)
            if not online_friend then
                return "missing_online_friend"
            end
            local offline_friend = data_provider:FindElementDataByPredicate(function(elementData)
                return elementData.buttonType == FRIENDS_BUTTON_TYPE_WOW and elementData.id == 2
            end)
            if not offline_friend then
                return "missing_offline_friend"
            end
            local divider = data_provider:FindElementDataByPredicate(function(elementData)
                return elementData.buttonType == FRIENDS_BUTTON_TYPE_DIVIDER
            end)
            if not divider then
                return "missing_divider"
            end

            local online_info = C_FriendList.GetFriendInfoByIndex(online_friend.id)
            local offline_info = C_FriendList.GetFriendInfoByIndex(offline_friend.id)
            if not online_info or online_info.name ~= "Alyth" or online_info.area ~= "Stormwind City" then
                return "online_info_mismatch"
            end
            if not offline_info or offline_info.name ~= "Brom" or offline_info.connected then
                return "offline_info_mismatch"
            end
            return "ok"
        "#).unwrap();

        assert_eq!(
            result,
            "ok",
            "FriendsFrame should render the seeded WoW friend row from C_FriendList: {result}"
        );
    }
}

// ── J → ToggleGuildFrame() ──────────────────────────────────────────────

#[test]
fn keybind_j_opens_guild() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("J", None).expect("J keybind failed");
        assert!(
            frame_is_shown(&env, "CommunitiesFrame"),
            "CommunitiesFrame should be shown after pressing J"
        );
    }
}
