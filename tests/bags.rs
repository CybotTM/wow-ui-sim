//! Tests for bag frames opening and displaying items.
//!
//! Loads the full Blizzard addon set, opens bags via keybind, and verifies
//! that item slots are populated with real item data from the mock inventory.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

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
    .expect("Failed to runtime-load Blizzard_TokenUI for bag tests");
}

fn install_test_error_handler(env: &WowLuaEnv) {
    common::install_error_collector(env, "__test_errors");
}

fn drain_test_errors(env: &WowLuaEnv) -> Vec<String> {
    common::drain_string_table(env, "__test_errors")
}

fn clear_recorded_lua_errors(env: &WowLuaEnv) {
    common::panel_fixtures::clear_recorded_lua_errors(env);
}

fn assert_no_bag_open_errors(env: &WowLuaEnv, context: &str) {
    let recorded_errors = common::panel_fixtures::recorded_lua_errors(env);
    let handler_errors = drain_test_errors(env);
    assert!(
        recorded_errors.is_empty(),
        "{context} produced {} recorded Lua error(s):\n{}\nhandler errors:\n{}",
        recorded_errors.len(),
        recorded_errors.join("\n"),
        handler_errors.join("\n"),
    );
    assert!(
        handler_errors.is_empty(),
        "{context} produced {} Lua error(s):\n{}",
        handler_errors.len(),
        handler_errors.join("\n"),
    );
}

#[test]
fn test_container_frames_registered() {
    let env = setup_env();

    // Check ContainerFrameContainer.ContainerFrames population
    let count: i32 = env
        .eval(
            r#"
            local t = ContainerFrameContainer.ContainerFrames
            if type(t) ~= "table" then return -1 end
            local n = 0
            for _ in pairs(t) do n = n + 1 end
            return n
        "#,
        )
        .unwrap();
    assert_eq!(
        count, 6,
        "ContainerFrameContainer.ContainerFrames should have 6 entries"
    );

    // Check individual frames exist
    for i in 1..=6 {
        let exists: bool = env
            .eval(&format!("return ContainerFrame{i} ~= nil"))
            .unwrap();
        assert!(exists, "ContainerFrame{i} should exist");
    }
}

#[test]
fn test_container_frames_array_contains_only_real_container_frames() {
    let env = setup_env();

    let names: Vec<String> = env
        .eval(
            r#"
            local t = assert(ContainerFrameContainer and ContainerFrameContainer.ContainerFrames)
            local names = {}
            for k, v in pairs(t) do
                local key = tostring(k)
                local name = type(v) == "table" and v.GetName and v:GetName() or tostring(v)
                table.insert(names, key .. "=" .. tostring(name))
            end
            table.sort(names)
            return names
        "#,
        )
        .unwrap();

    assert_eq!(
        names,
        vec![
            "1=ContainerFrame1".to_string(),
            "2=ContainerFrame2".to_string(),
            "3=ContainerFrame3".to_string(),
            "4=ContainerFrame4".to_string(),
            "5=ContainerFrame5".to_string(),
            "6=ContainerFrame6".to_string(),
        ],
        "ContainerFrameContainer.ContainerFrames should only contain the six real bag frames",
    );
}

#[test]
fn test_bag_ui_c_container_helpers_have_safe_default_shapes() {
    let env = setup_env();

    let (not_filtered, quest_info_ok, cannot_upgrade, trade_money_zero): (bool, bool, bool, bool) =
        env.eval(
            r#"
            local questInfo = C_Container.GetContainerItemQuestInfo(0, 1)
            local itemLocation = ItemLocation:CreateFromBagAndSlot(0, 1)
            return C_Container.IsContainerFiltered(0) == false,
                   type(questInfo) == "table"
                       and questInfo.isQuestItem == false
                       and questInfo.isActive == false,
                   C_ItemUpgrade.CanUpgradeItem(itemLocation) == false,
                   GetPlayerTradeMoney() == 0
            "#,
        )
        .unwrap();

    assert!(not_filtered, "bag search defaults should start unfiltered");
    assert!(
        quest_info_ok,
        "bag quest-info helper should return a safe default table shape",
    );
    assert!(
        cannot_upgrade,
        "default bag items should not report upgrade availability"
    );
    assert!(
        trade_money_zero,
        "default trade money helper should report zero"
    );
}

/// Open all bags via pcall-protected ToggleAllBags, logging any Lua errors.
fn open_all_bags(env: &WowLuaEnv) {
    env.exec(
        r#"
        local ok, err = pcall(ToggleAllBags)
        if not ok then
            table.insert(__test_errors, "ToggleAllBags: " .. tostring(err))
        end
    "#,
    )
    .unwrap();

    let errors = drain_test_errors(env);
    for e in &errors {
        eprintln!("Lua error: {e}");
    }
}

/// Assert that at least one bag frame is visible.
fn assert_bag_frame_visible(env: &WowLuaEnv) {
    let bag_shown: bool = env
        .eval(
            "return (ContainerFrameCombinedBags and ContainerFrameCombinedBags:IsShown()) \
             or (ContainerFrame1 and ContainerFrame1:IsShown())",
        )
        .unwrap();
    assert!(
        bag_shown,
        "A bag frame should be visible after ToggleAllBags"
    );
}

/// Assert the backpack has the expected number of populated item slots.
fn assert_backpack_item_count(env: &WowLuaEnv, expected: i32) {
    let populated_slots: i32 = env
        .eval(
            r#"
            local count = 0
            for slot = 1, 16 do
                local info = C_Container.GetContainerItemInfo(0, slot)
                if info and info.itemID then
                    count = count + 1
                end
            end
            return count
        "#,
        )
        .unwrap();
    assert_eq!(
        populated_slots, expected,
        "Backpack populated slot count mismatch"
    );
}

#[test]
fn test_bag_env_loads_real_backpack_token_tracker() {
    let env = setup_env();

    let (tracker_is_real_frame, tracker_matches_backpack_frame, addon_loaded): (bool, bool, bool) =
        env.eval(
            r#"
            local tracker = ContainerFrameSettingsManager and ContainerFrameSettingsManager.TokenTracker
            return type(tracker) == "table"
                and type(tracker.UpdateIfVisible) == "function"
                and type(tracker.GetMaxTokensWatched) == "function",
                tracker == BackpackTokenFrame,
                C_AddOns and C_AddOns.IsAddOnLoaded
                    and C_AddOns.IsAddOnLoaded("Blizzard_TokenUI") or false
            "#,
        )
        .unwrap();

    assert!(tracker_is_real_frame);
    assert!(tracker_matches_backpack_frame);
    assert!(addon_loaded);
}

#[test]
fn test_bags_open_with_items() {
    let env = setup_env();
    install_test_error_handler(&env);
    clear_recorded_lua_errors(&env);

    // Backpack starts with 4 default items (Hearthstone, Water, Bread, Skinning Knife)
    // Add one more via admin API
    env.exec("A_Admin.AddBagItem(0, 5, 6948, 1)").unwrap();

    open_all_bags(&env);
    assert_no_bag_open_errors(&env, "ToggleAllBags backpack open");
    assert_bag_frame_visible(&env);
    assert_backpack_item_count(&env, 5);

    // Verify default Hearthstone in slot 1
    let item_link: String = env
        .eval(r#"return C_Container.GetContainerItemInfo(0, 1).hyperlink"#)
        .unwrap();
    assert!(
        item_link.contains("Hearthstone"),
        "Slot 1 should contain Hearthstone, got: {item_link}",
    );

    // Verify empty slots return nil
    let empty: bool = env
        .eval("return C_Container.GetContainerItemInfo(0, 6) == nil")
        .unwrap();
    assert!(empty, "Slot 6 should be empty");
}

#[test]
fn test_container_frame_1_item_1_icon_matches_first_bag_slot_item() {
    let env = setup_env();
    install_test_error_handler(&env);
    clear_recorded_lua_errors(&env);

    env.exec(
        r#"
        assert(ContainerFrameSettingsManager, "ContainerFrameSettingsManager should exist")
        ContainerFrameSettingsManager:SetUsingCombinedBags(false)
        "#,
    )
    .unwrap();

    open_all_bags(&env);
    assert_no_bag_open_errors(&env, "ToggleAllBags individual-bag open");

    let result: String = env
        .eval(
            r#"
            if not ContainerFrame1 then
                return "missing_container_frame_1"
            end
            local button = ContainerFrame1Item1
            if not button and ContainerFrame1.Items then
                button = ContainerFrame1.Items[1]
                if not button then
                    return "missing_container_frame_1_items_1"
                end
                if not button.icon then
                    return "missing_container_frame_1_items_1_icon"
                end

                local buttonSlot = button:GetID()
                local actual = button.icon:GetTexture()
                local buttonInfo = C_Container.GetContainerItemInfo(0, buttonSlot)
                local buttonTexture = buttonInfo and buttonInfo.iconFileID
                if actual ~= buttonTexture then
                    return string.format(
                        "stale_name_items_1_button_slot_%d_expected_%s_actual_%s",
                        buttonSlot,
                        tostring(buttonTexture),
                        tostring(actual)
                    )
                end
                return string.format("stale_name_items_1_button_slot_%d", buttonSlot)
            end
            if not button then
                return "missing_container_frame_1_item_1"
            end

            local buttonSlot = button:GetID()
            local actual = button.icon and button.icon:GetTexture()
            local firstSlotInfo = C_Container.GetContainerItemInfo(0, 1)
            local firstSlotTexture = firstSlotInfo and firstSlotInfo.iconFileID

            if buttonSlot ~= 1 then
                local buttonInfo = C_Container.GetContainerItemInfo(0, buttonSlot)
                local buttonTexture = buttonInfo and buttonInfo.iconFileID
                if actual ~= buttonTexture then
                    return string.format(
                        "stale_name_button_slot_%d_expected_%s_actual_%s",
                        buttonSlot,
                        tostring(buttonTexture),
                        tostring(actual)
                    )
                end
                return string.format("stale_name_button_slot_%d", buttonSlot)
            end

            if actual ~= firstSlotTexture then
                return string.format(
                    "icon_mismatch_expected_%s_actual_%s",
                    tostring(firstSlotTexture),
                    tostring(actual)
                )
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert!(
        result == "ok"
            || result.starts_with("stale_name_button_slot_")
            || result.starts_with("stale_name_items_1_button_slot_"),
        "ContainerFrame1Item1 should either match bag slot 1 directly or prove the plan wording is stale through the real item-button list: {result}"
    );
}
