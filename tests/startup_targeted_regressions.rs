mod common;

use std::path::PathBuf;

use tempfile::tempdir;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon, load_addon_with_saved_vars};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::SavedVariablesManager;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn damage_meter_saved_vars_shape(env: &WowLuaEnv) -> (String, String) {
    env.eval(
        r#"
        local saved_type = type(DamageMeterPerCharacterSettings)
        local list_type = "missing"
        if saved_type == "table" then
            list_type = type(DamageMeterPerCharacterSettings.windowDataList)
        end
        return saved_type, list_type
        "#,
    )
    .expect("damage meter saved vars probe should run")
}

fn run_standard_startup(env: &WowLuaEnv, mut after_step: impl FnMut()) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        env.fire_event(event).ok();
        after_step();
    }
    env.fire_edit_mode_layouts_updated().ok();
    after_step();
    common::call_global_if_present(env, "RequestTimePlayed");
    common::fire_player_entering_world(env, true, false);
    after_step();

    for event in [
        "UNIT_AURA",
        "BAG_UPDATE_DELAYED",
        "QUEST_LOG_UPDATE",
        "GROUP_ROSTER_UPDATE",
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
        "UPDATE_CHAT_WINDOWS",
    ] {
        env.fire_event(event).ok();
        after_step();
    }

    env.fire_on_update(0.016).ok();
    after_step();
}

fn load_and_startup_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);

    for (_name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path).expect("Failed to load Blizzard addon");
    }

    env.apply_post_load_workarounds();
    run_standard_startup(&env, || {});
    env
}

fn push_addon_load_messages(
    messages: &mut Vec<String>,
    name: &str,
    result: Result<wow_ui_sim::loader::LoadResult, wow_ui_sim::loader::LoadError>,
) {
    match result {
        Ok(result) => {
            for warning in result.warnings {
                messages.push(format!("[load {name}] {warning}"));
            }
        }
        Err(error) => messages.push(format!("[load {name}] FAILED: {error}")),
    }
}

fn drain_startup_errors(env: &WowLuaEnv, messages: &mut Vec<String>) {
    messages.extend(common::drain_string_table(env, "__targeted_startup_errors"));
}

fn load_targeted_startup_env(messages: &mut Vec<String>) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        let result = load_addon(&env.loader_env(), toc_path);
        push_addon_load_messages(messages, name, result);
    }

    env.apply_post_load_workarounds();
    common::install_error_collector(&env, "__targeted_startup_errors");
    env
}

fn load_with_early_error_collector(messages: &mut Vec<String>) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    common::install_error_collector(&env, "__targeted_startup_errors");

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        let result = load_addon(&env.loader_env(), toc_path);
        push_addon_load_messages(messages, name, result);
        drain_startup_errors(&env, messages);
    }

    env.apply_post_load_workarounds();
    env
}

fn collect_targeted_startup_messages(env: &WowLuaEnv, messages: &mut Vec<String>) {
    run_standard_startup(env, || {
        drain_startup_errors(env, messages);
    });
}

fn load_and_startup_collect_messages() -> Vec<String> {
    let mut messages = Vec::new();
    let env = load_targeted_startup_env(&mut messages);
    collect_targeted_startup_messages(&env, &mut messages);
    messages
}

#[test]
fn startup_omits_targeted_missing_global_errors() {
    test_timeout! {
        let messages = load_and_startup_collect_messages();
        let targeted: Vec<String> = messages
            .into_iter()
            .filter(|message| {
                message.contains("GetQuestLink")
                    || message.contains("GetWorldPVPQueueStatus")
                    || message.contains("CanHearthAndResurrectFromArea")
                    || message.contains("UnitIsOtherPlayersPet")
                    || message.contains("SupportsClipCursor")
                    || message.contains("GetNumBattlefieldFlagPositions")
                    || message.contains("GetWorldMapActionButtonSpellInfo")
                    || message.contains("PlayerIsPVPInactive")
                    || message.contains("GetMouseFoci")
                    || message.contains("QuestOfferDataProvider.lua:174")
                    || message.contains("ContentTrackingDataProvider.lua:51")
                    || message.contains("DigSiteDataProvider.lua:17")
                    || message.contains("GarrisonPlotDataProvider.lua:12")
                    || message.contains("DungeonEntranceDataProvider.lua:34")
                    || message.contains("BannerDataProvider.lua:12")
                    || message.contains("MapLinkDataProvider.lua:12")
                    || message.contains("SelectableGraveyardDataProvider.lua:30")
                    || message.contains("AreaPOIEventDataProvider.lua:46")
                    || message.contains("DelveEntranceDataProvider.lua:32")
                    || message.contains("EncounterJournalDataProvider.lua:35")
                    || message.contains("invalid script handler 'OnCooldownDone'")
                    || message.contains("attempt to index field 'savedVars' (a nil value)")
                    || message.contains("attempt to call local '(for index)'")
                    || message.contains("attempt to call global 'date'")
            })
            .collect();

        assert!(
            targeted.is_empty(),
            "Startup should not report the targeted missing-global regressions:\n  {}",
            targeted.join("\n  ")
        );
    }
}

#[test]
fn startup_omits_arena_over_heal_absorb_glow_nil_error() {
    test_timeout! {
        let messages = load_and_startup_collect_messages();
        let targeted: Vec<String> = messages
            .into_iter()
            .filter(|message| {
                message.contains("ArenaEnemyMatchFrame1")
                    && message.contains("overHealAbsorbGlow")
            })
            .collect();

        assert!(
            targeted.is_empty(),
            "Startup should not report the ArenaEnemyMatchFrame1 overHealAbsorbGlow nil regression:\n  {}",
            targeted.join("\n  ")
        );
    }
}

#[test]
fn startup_omits_followup_blizzard_lua_errors() {
    test_timeout! {
        let env = load_and_startup_env();
        let state = env.state();
        let targeted: Vec<String> = state
            .borrow()
            .lua_error_records
            .iter()
            .filter(|record| {
                record.message.contains("CheckButton")
                    || record.message.contains("GetItemLevelColor")
                    || record.message.contains("ClearCursorHoveredItem")
                    || record.message.contains("SetCursorHoveredItem")
                    || record.message.contains("UnitInSubgroup")
                    || record.message.contains("GetNumGuildPerks")
                    || record.message.contains("RequestGuildRewards")
                    || record.message.contains("GetGuildRenameRequired")
                    || record.message.contains("GetAvailableBandwidth")
                    || record.message.contains("overHealAbsorbGlow")
                    || record.message.contains("transmogLocation")
                    || record.message.contains("CommunitiesUtil.lua:217")
                    || record.message.contains("WarbandSceneCollection.lua:54")
                    || record.message.contains("expected number, got nil at argument 1")
                    || record.message.contains("expected number, got string at argument 1")
            })
            .map(|record| {
                let addon = record.addon_name.as_deref().unwrap_or("<none>");
                format!("[{addon}] {}", record.message)
            })
            .collect();

        assert!(
            targeted.is_empty(),
            "Startup should not report the follow-up Blizzard Lua regressions:\n  {}",
            targeted.join("\n  ")
        );
    }
}

#[test]
fn startup_followup_surfaces_expose_safe_defaults() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: (
            i32,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
        ) = env
            .eval(
                r##"
                local r, g, b = GetItemLevelColor()
                local appearanceSlotInfo, illusionSlotInfo = C_TransmogOutfitInfo.GetAllSlotLocationInfo()
                return
                    select("#", GetItemLevelColor()),
                    type(r) == "number" and type(g) == "number" and type(b) == "number",
                    type(C_Club.GetClubStreamNotificationSettings("guild-0")) == "table",
                    type(C_WarbandScene.SearchWarbandSceneEntries({})) == "table",
                    type(appearanceSlotInfo) == "table",
                    type(illusionSlotInfo) == "table",
                    UnitInSubgroup("player") == false,
                    GetNumGuildPerks() == 0,
                    GetGuildRenameRequired() == false,
                    type(GetAvailableBandwidth()) == "number",
                    type(GetDownloadedPercentage()) == "number",
                    pcall(ClearCursorHoveredItem),
                    pcall(SetCursorHoveredItem, nil),
                    pcall(SetCursorHoveredItemTradeItem, true),
                    pcall(RequestGuildRewards)
                "##,
            )
            .expect("follow-up startup surfaces should return safe defaults");

        let catalog_shop_nav_soundkit_is_number: bool = env
            .eval(
                r#"
                return type(SOUNDKIT.CATALOG_SHOP_SELECT_NAV_MENU) == "number"
                "#,
            )
            .expect("catalog shop nav soundkit probe should run");

        let (
            color_count,
            item_level_color_ok,
            club_stream_ok,
            warband_scene_ok,
            appearance_slot_info_ok,
            illusion_slot_info_ok,
            unit_in_subgroup_player_ok,
            guild_perks_ok,
            guild_rename_required_ok,
            available_bandwidth_ok,
            downloaded_percentage_ok,
            clear_cursor_hovered_item_ok,
            set_cursor_hovered_item_ok,
            set_cursor_hovered_trade_item_ok,
            request_guild_rewards_ok,
        ) = result;
        assert_eq!(color_count, 3, "GetItemLevelColor should return three values");
        assert!(
            item_level_color_ok
                && club_stream_ok
                && warband_scene_ok
                && appearance_slot_info_ok
                && illusion_slot_info_ok
                && unit_in_subgroup_player_ok
                && guild_perks_ok
                && guild_rename_required_ok
                && available_bandwidth_ok
                && downloaded_percentage_ok
                && clear_cursor_hovered_item_ok
                && set_cursor_hovered_item_ok
                && set_cursor_hovered_trade_item_ok
                && request_guild_rewards_ok,
            "Follow-up startup surfaces should expose safe defaults for Blizzard callers"
        );
        assert!(
            catalog_shop_nav_soundkit_is_number,
            "CatalogShop nav soundkit should be seeded during startup"
        );
    }
}

#[test]
fn startup_wardrobe_tab_has_transmog_locations() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: String = env
            .eval(
                r#"
                local appearanceSlotInfo, illusionSlotInfo = C_TransmogOutfitInfo.GetAllSlotLocationInfo()
                if type(appearanceSlotInfo) ~= "table" or #appearanceSlotInfo == 0 then
                    return "missing_appearance_slot_info"
                end
                if type(illusionSlotInfo) ~= "table" then
                    return "missing_illusion_slot_info"
                end

                local transmogLocation = TransmogUtil.GetTransmogLocation("HEADSLOT", Enum.TransmogType.Appearance, false)
                if not transmogLocation then
                    return "missing_head_transmog_location"
                end
                if transmogLocation:GetSlotName() ~= "HEADSLOT" then
                    return "wrong_slot_name:" .. tostring(transmogLocation:GetSlotName())
                end

                return "ok"
                "#,
            )
            .expect("wardrobe transmog location probe should run");

        assert_eq!(result, "ok");
    }
}

#[test]
fn startup_wardrobe_can_switch_from_armor_to_weapon_slot() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: String = env
            .eval(
                r#"
                ToggleCollectionsJournal(5)
                if CollectionsJournal and CollectionsJournal_SetTab then
                    CollectionsJournal_SetTab(CollectionsJournal, 5)
                end

                local itemsFrame = WardrobeCollectionFrame and WardrobeCollectionFrame.ItemsCollectionFrame
                if not itemsFrame then
                    return "missing_items_frame"
                end

                local headLocation = TransmogUtil.GetTransmogLocation("HEADSLOT", Enum.TransmogType.Appearance, false)
                local mainHandLocation = TransmogUtil.GetTransmogLocation("MAINHANDSLOT", Enum.TransmogType.Appearance, false)
                if not headLocation or not mainHandLocation then
                    return "missing_location"
                end
                if mainHandLocation:GetArmorCategoryID() ~= nil then
                    return "weapon_has_armor_category"
                end

                local headOk, headErr = pcall(function()
                    itemsFrame:SetActiveSlot(headLocation)
                end)
                if not headOk then
                    return "head_error:" .. tostring(headErr)
                end

                local weaponOk, weaponErr = pcall(function()
                    itemsFrame:SetActiveSlot(mainHandLocation)
                end)
                if not weaponOk then
                    return "weapon_error:" .. tostring(weaponErr)
                end

                return "ok"
                "#,
            )
            .expect("wardrobe armor-to-weapon slot switch probe should run");

        assert_eq!(result, "ok");
    }
}

#[test]
fn startup_wardrobe_head_appearances_are_displayable() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: String = env
            .eval(
                r#"
                ToggleCollectionsJournal(5)
                if CollectionsJournal and CollectionsJournal_SetTab then
                    CollectionsJournal_SetTab(CollectionsJournal, 5)
                end

                local itemsFrame = WardrobeCollectionFrame and WardrobeCollectionFrame.ItemsCollectionFrame
                if not itemsFrame then
                    return "missing_items_frame"
                end

                local headLocation = TransmogUtil.GetTransmogLocation("HEADSLOT", Enum.TransmogType.Appearance, false)
                if not headLocation then
                    return "missing_head_location"
                end

                itemsFrame:SetActiveSlot(headLocation)
                local model = itemsFrame.Models and itemsFrame.Models[1]
                if not model or not model.visualInfo then
                    return "missing_visual_info"
                end
                if not model.visualInfo.canDisplayOnPlayer then
                    return "not_displayable"
                end
                if model.SlotInvalidTexture:IsShown() then
                    return "invalid_overlay_shown"
                end

                return "ok"
                "#,
            )
            .expect("wardrobe displayability probe should run");

        assert_eq!(result, "ok");
    }
}

#[test]
fn startup_wardrobe_filter_dropdown_click_toggles_not_collected() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: String = env
            .eval(
                r#"
                ToggleCollectionsJournal(5)
                if CollectionsJournal and CollectionsJournal_SetTab then
                    CollectionsJournal_SetTab(CollectionsJournal, 5)
                end

                local wardrobeFrame = WardrobeCollectionFrame
                local itemsFrame = wardrobeFrame and wardrobeFrame.ItemsCollectionFrame
                local filterButton = wardrobeFrame and wardrobeFrame.FilterButton
                if not itemsFrame or not filterButton then
                    return "missing_wardrobe_filter"
                end

                local headLocation = TransmogUtil.GetTransmogLocation("HEADSLOT", Enum.TransmogType.Appearance, false)
                if not headLocation then
                    return "missing_head_location"
                end
                itemsFrame:SetActiveSlot(headLocation)

                if not filterButton:IsEnabled() then
                    return "filter_disabled"
                end
                local onMouseDown = filterButton:GetScript("OnMouseDown")
                if type(onMouseDown) ~= "function" then
                    return "missing_on_mouse_down"
                end
                onMouseDown(filterButton, "LeftButton")
                if not filterButton:IsMenuOpen() then
                    return "menu_not_open"
                end

                local notCollectedButton
                local function inspectButton(button)
                    local text = button:GetText()
                    if (text == nil or text == "") and button.fontString then
                        text = button.fontString:GetText()
                    end
                    if text == NOT_COLLECTED then
                        notCollectedButton = button
                    end
                end

                for _, button in ipairs(filterButton.__wow_menu_buttons or {}) do
                    inspectButton(button)
                end
                if not notCollectedButton and filterButton.menu then
                    for _, child in ipairs({ filterButton.menu:GetChildren() }) do
                        if child.GetText then
                            inspectButton(child)
                        end
                    end
                end
                if not notCollectedButton then
                    return "missing_not_collected_button"
                end

                C_TransmogCollection.SetUncollectedShown(true)
                notCollectedButton:Click()
                if C_TransmogCollection.GetUncollectedShown() then
                    return "not_collected_still_enabled"
                end

                return "ok"
                "#,
            )
            .expect("wardrobe filter dropdown probe should run");

        assert_eq!(result, "ok");
    }
}

#[test]
fn startup_wardrobe_class_dropdown_uses_localized_radio_rows() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: String = env
            .eval(
                r#"
                ToggleCollectionsJournal(5)
                if CollectionsJournal and CollectionsJournal_SetTab then
                    CollectionsJournal_SetTab(CollectionsJournal, 5)
                end

                local dropdown = WardrobeCollectionFrame and WardrobeCollectionFrame.ClassDropdown
                if not dropdown then
                    return "missing_class_dropdown"
                end

                dropdown:Show()
                dropdown:Refresh()
                dropdown:OpenMenu()

                local selectedText = dropdown.Text and dropdown.Text:GetText() or dropdown:GetText()
                if type(selectedText) ~= "string" or not selectedText:find("|c") then
                    return "selected_not_colored:" .. tostring(selectedText)
                end
                if selectedText:find("PALADIN", 1, true) then
                    return "selected_uppercase:" .. selectedText
                end
                local paladinColor = GetClassColorObj("PALADIN")
                local r, g, b = dropdown.Text:GetTextColor()
                if math.abs(r - paladinColor.r) > 0.01
                    or math.abs(g - paladinColor.g) > 0.01
                    or math.abs(b - paladinColor.b) > 0.01 then
                    return "selected_color=" .. tostring(r) .. "," .. tostring(g) .. "," .. tostring(b)
                end

                local firstButton
                local function inspectButton(button)
                    local text = button:GetText()
                    if (text == nil or text == "") and button.fontString then
                        text = button.fontString:GetText()
                    end
                    if text == "Warrior" then
                        firstButton = button
                    end
                end

                for _, button in ipairs(dropdown.__wow_menu_buttons or {}) do
                    inspectButton(button)
                end
                if not firstButton and dropdown.menu then
                    for _, child in ipairs({ dropdown.menu:GetChildren() }) do
                        if child.GetText then
                            inspectButton(child)
                        end
                    end
                end
                if not firstButton then
                    return "missing_warrior_button"
                end
                if not firstButton.leftTexture1 then
                    return "missing_radio_texture"
                end

                return "ok"
                "#,
            )
            .expect("wardrobe class dropdown probe should run");

        assert_eq!(result, "ok");
    }
}

#[test]
fn cursor_hovered_item_globals_are_callable() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        let result: (bool, bool, bool, bool, bool) = env
            .eval(
                r#"
                return
                    type(ClearCursorHoveredItem) == "function",
                    type(SetCursorHoveredItem) == "function",
                    type(SetCursorHoveredItemTradeItem) == "function",
                    pcall(SetCursorHoveredItem, nil),
                    pcall(SetCursorHoveredItemTradeItem, true)
                "#,
            )
            .expect("cursor hovered globals probe should run");

        let (
            clear_cursor_hovered_item_is_fn,
            set_cursor_hovered_item_is_fn,
            set_cursor_hovered_trade_item_is_fn,
            set_cursor_hovered_item_ok,
            set_cursor_hovered_trade_item_ok,
        ) = result;

        assert!(
            clear_cursor_hovered_item_is_fn
                && set_cursor_hovered_item_is_fn
                && set_cursor_hovered_trade_item_is_fn
                && set_cursor_hovered_item_ok
                && set_cursor_hovered_trade_item_ok,
            "cursor hovered globals should exist and be callable"
        );
    }
}

#[test]
fn startup_player_life_bar_matches_player_health() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: (
            Option<f64>,
            Option<f64>,
            String,
            bool,
            String,
            i32,
            i32,
        ) = env
            .eval(
                r#"
                local healthBar = PlayerFrame_GetHealthBar and PlayerFrame_GetHealthBar()
                local playerFrameState = PlayerFrame and PlayerFrame.state or "nil"
                local vehicleUi = type(UnitHasVehiclePlayerFrameUI) == "function"
                    and UnitHasVehiclePlayerFrameUI("player")
                    or false
                if not healthBar then
                    return nil, nil, playerFrameState, vehicleUi, "nil", UnitHealth("player"), UnitHealthMax("player")
                end
                local _, maxValue = healthBar:GetMinMaxValues()
                return healthBar:GetValue(), maxValue, playerFrameState, vehicleUi, tostring(healthBar.unit), UnitHealth("player"), UnitHealthMax("player")
                "#,
            )
            .expect("player health bar probe should run");

        let (
            bar_value,
            bar_max,
            player_frame_state,
            vehicle_ui,
            bar_unit,
            current_health,
            max_health,
        ) = result;
        assert!(
            current_health > 0,
            "player health should be initialized at startup"
        );
        assert!(
            max_health > 0,
            "player health max should be initialized at startup"
        );
        assert_eq!(
            bar_max,
            Some(max_health as f64),
            "player health bar max should match player max health"
        );
        assert_eq!(
            player_frame_state,
            "player",
            "player frame should stay on player art at startup"
        );
        assert!(
            !vehicle_ui,
            "vehicle player-frame UI should be disabled in the simulator startup surface"
        );
        assert_eq!(
            bar_unit,
            "player",
            "player health bar should stay bound to the player unit"
        );
        assert_eq!(
            bar_value,
            Some(current_health as f64),
            "player health bar should reflect current player health"
        );
    }
}

#[test]
fn startup_player_buffs_show_duration_text() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: (i32, i32, Option<String>) = env
            .eval(
                r#"
                if not BuffFrame or not BuffFrame.auraFrames then
                    return 0, 0, nil
                end

                local visible_buffs = 0
                local visible_durations = 0
                local first_duration = nil

                for _, button in ipairs(BuffFrame.auraFrames) do
                    if button:IsShown()
                        and button.buttonInfo
                        and button.buttonInfo.auraType == "Buff"
                        and button.buttonInfo.expirationTime
                        and button.buttonInfo.expirationTime > 0
                    then
                        visible_buffs = visible_buffs + 1
                        if button.Duration and button.Duration:IsShown() then
                            visible_durations = visible_durations + 1
                            if not first_duration and button.Duration:GetText() then
                                first_duration = button.Duration:GetText()
                            end
                        end
                    end
                end

                return visible_buffs, visible_durations, first_duration
                "#,
            )
            .expect("buff duration probe should run");

        let (visible_buffs, visible_durations, first_duration) = result;
        assert!(
            visible_buffs > 0,
            "startup should expose at least one visible player buff with a duration"
        );
        assert_eq!(
            visible_buffs,
            visible_durations,
            "visible player buffs with durations should render their duration labels"
        );
        assert!(
            first_duration.is_some(),
            "at least one visible buff duration should have text"
        );
    }
}

#[test]
fn startup_keeps_action_bar_deprecation_fallbacks_non_recursive() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: (bool, bool, bool, bool) = env
            .eval(
                r#"
                local texture_ok, texture = pcall(C_ActionBar.GetActionTexture, 13)
                local has_action_ok, has_action = pcall(C_ActionBar.HasAction, 13)
                return texture_ok, texture == nil, has_action_ok, has_action == false
            "#,
            )
            .expect("C_ActionBar probes should return values");

        assert_eq!(
            result,
            (true, true, true, true),
            "Deprecated action-bar fallbacks should not recurse through C_ActionBar"
        );
    }
}

#[test]
fn c_action_bar_matches_master_default_bar_indices() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        let result: (
            i32,
            i32,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            i32,
            i32,
            i32,
        ) = env
            .eval(
                r#"
                return C_ActionBar.GetCurrentActionBarByClass(),
                       C_ActionBar.GetExtraBarIndex(),
                       C_ActionBar.GetVehicleBarIndex(),
                       C_ActionBar.GetOverrideBarIndex(),
                       C_ActionBar.GetTempShapeshiftBarIndex(),
                       C_ActionBar.GetMultiCastBarIndex(),
                       C_ActionBar.GetBonusBarIndex(),
                       C_ActionBar.GetBonusBarOffset()
            "#,
            )
            .expect("C_ActionBar default bar indices should evaluate");

        assert_eq!(
            result,
            (1, 13, None, None, None, 7, 0, 0),
            "C_ActionBar should match master default bar index semantics"
        );
    }
}

#[test]
fn blizzard_console_saved_variables_machine_seed_without_saved_vars_manager() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let toc_path = blizzard_ui_dir().join("Blizzard_Console/Blizzard_Console.toc");
        load_addon(&env.loader_env(), &toc_path)
            .expect("Blizzard_Console should load without a saved vars manager");

        let saved_vars_type: String = env
            .eval("return type(Blizzard_Console_SavedVars)")
            .expect("saved vars probe should run");

        assert_eq!(
            saved_vars_type, "table",
            "SavedVariablesMachine globals should still be seeded when persistence is disabled"
        );
    }
}

#[test]
fn damage_meter_saved_variables_default_without_partial_empty_seed() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let edit_mode_toc = blizzard_ui_dir().join("Blizzard_EditMode/Blizzard_EditMode.toc");
        load_addon(&env.loader_env(), &edit_mode_toc).expect("Blizzard_EditMode should load");

        let toc_path = blizzard_ui_dir().join("Blizzard_DamageMeter/Blizzard_DamageMeter.toc");
        load_addon(&env.loader_env(), &toc_path)
            .expect("Blizzard_DamageMeter should load without a saved vars manager");

        let (saved_vars_type, window_data_list_type) = damage_meter_saved_vars_shape(&env);

        assert!(
            saved_vars_type == "nil" || window_data_list_type == "table",
            "DamageMeter saved vars should stay nil or expose windowDataList, not a partially-seeded table"
        );
    }
}

#[test]
fn damage_meter_saved_variables_default_with_empty_saved_vars_storage() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);
        let temp = tempdir().expect("tempdir");
        let mut saved_vars = SavedVariablesManager::with_storage_dir(temp.path().to_path_buf());

        let edit_mode_toc = blizzard_ui_dir().join("Blizzard_EditMode/Blizzard_EditMode.toc");
        load_addon_with_saved_vars(&env.loader_env(), &edit_mode_toc, &mut saved_vars)
            .expect("Blizzard_EditMode should load with an empty saved vars manager");

        let toc_path = blizzard_ui_dir().join("Blizzard_DamageMeter/Blizzard_DamageMeter.toc");
        load_addon_with_saved_vars(&env.loader_env(), &toc_path, &mut saved_vars)
            .expect("Blizzard_DamageMeter should load with an empty saved vars manager");

        let (saved_vars_type, window_data_list_type) = damage_meter_saved_vars_shape(&env);

        assert!(
            saved_vars_type == "nil" || window_data_list_type == "table",
            "DamageMeter saved vars should stay nil or expose windowDataList, not a partially-seeded table"
        );
    }
}

#[test]
fn startup_chat_config_checkbox_frames_keep_checkbutton_children() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: (bool, bool, bool, bool, bool, bool) = env
            .eval(
                r##"
                return
                    type(ChatConfigChatSettingsLeftCheckbox1) == "table",
                    ChatConfigChatSettingsLeftCheckbox1
                        and ChatConfigChatSettingsLeftCheckbox1.CheckButton ~= nil
                        or false,
                    ChatConfigChatSettingsLeftCheckbox1Check ~= nil,
                    type(ChatConfigChannelSettingsLeftCheckbox1) == "table",
                    ChatConfigChannelSettingsLeftCheckbox1
                        and ChatConfigChannelSettingsLeftCheckbox1.CheckButton ~= nil
                        or false,
                    ChatConfigChannelSettingsLeftCheckbox1Check ~= nil
                "##,
            )
            .expect("chat config checkbox probes should run");

        assert_eq!(
            result,
            (true, true, true, true, true, true),
            "Chat config checkbox frames should keep their inherited CheckButton children"
        );
    }
}

#[test]
fn startup_frames_missing_checkbutton_parent_key() {
    test_timeout! {
        let env = load_and_startup_env();
        let missing: String = env
            .eval(
                r##"
                local missing = {}
                for key, value in pairs(_G) do
                    if type(key) == "string" and type(value) == "table" and _G[key .. "Check"] ~= nil then
                        local ok, objectType = pcall(function()
                            return type(value.GetObjectType) == "function" and value:GetObjectType() or nil
                        end)
                        if ok and type(objectType) == "string" and value.CheckButton == nil then
                            missing[#missing + 1] = key .. " [" .. objectType .. "]"
                        end
                    end
                end
                table.sort(missing)
                return table.concat(missing, "\n")
                "##,
            )
            .expect("missing CheckButton frame probe should run");

        assert!(
            missing.is_empty(),
            "Startup frames missing CheckButton parent keys:
    {missing}"
        );
    }
}

#[test]
fn startup_widget_tree_missing_checkbutton_parent_keys() {
    test_timeout! {
        let env = load_and_startup_env();
        let state = env.state();
        let sim = state.borrow();
        let mut missing = Vec::new();
        for frame_id in sim.widgets.iter_ids() {
            let Some(frame) = sim.widgets.get(frame_id) else {
                continue;
            };
            if frame.children_keys.contains_key("CheckButton") {
                continue;
            }
            let Some(child_id) = frame.children.iter().copied().find(|child_id| {
                sim.widgets
                    .get(*child_id)
                    .and_then(|child| child.name.as_deref())
                    .is_some_and(|name| name.ends_with("Check"))
            }) else {
                continue;
            };
            let child_name = sim
                .widgets
                .get(child_id)
                .and_then(|child| child.name.clone())
                .unwrap_or_else(|| format!("#{child_id}"));
            missing.push(format!(
                "id={frame_id} name={:?} child={child_name}",
                frame.name
            ));
        }
        missing.sort();

        assert!(
            missing.is_empty(),
            "Startup widget tree is missing CheckButton parent keys:
    {}",
            missing.join("
    ")
        );
    }
}

#[test]
fn startup_chat_config_dynamic_wide_checkboxes_keep_checkbutton_parent_key() {
    test_timeout! {
        let env = load_and_startup_env();
        let result: (bool, bool, bool, bool, bool) = env
            .eval(
                r##"
                local ok, err = pcall(function()
                    CreateFrame("CheckButton", "StartupWideCheckboxProbe", UIParent, "MovableChatConfigWideCheckboxWithSwatchTemplate")
                end)
                local frame = StartupWideCheckboxProbe
                local check = StartupWideCheckboxProbeCheck
                return
                    ok,
                    type(err) == "nil",
                    frame ~= nil,
                    check ~= nil,
                    frame ~= nil and frame.CheckButton == check
                "##,
            )
            .expect("dynamic chat checkbox probe should run");

        assert_eq!(
            result,
            (true, true, true, true, true),
            "Dynamic chat checkboxes should keep their CheckButton child wired to the parent frame"
        );
    }
}

#[test]
fn chat_config_create_checkboxes_does_not_emit_checkbutton_error() {
    test_timeout! {
        let env = load_and_startup_env();
        let before = env.state().borrow().lua_errors.len();
        env.exec(
            r##"
            ChatConfig_CreateCheckboxes(ChatConfigChatSettingsLeft, CHAT_CONFIG_CHAT_LEFT, "ChatConfigWideCheckboxWithSwatchTemplate", PLAYER_MESSAGES)
            "##,
        )
        .expect("chat checkbox creation should succeed");
        let after = env.state().borrow().lua_errors.clone();
        let targeted: Vec<String> = after
            .into_iter()
            .skip(before)
            .filter(|message| message.contains("CheckButton"))
            .collect();

        assert!(
            targeted.is_empty(),
            "ChatConfig_CreateCheckboxes should not emit CheckButton errors:
    {}",
            targeted.join("
    ")
        );
    }
}

#[test]
fn startup_chat_config_channel_checkbox_creation_keeps_checkbutton_children() {
    test_timeout! {
        let mut messages = Vec::new();
        let env = load_targeted_startup_env(&mut messages);
        env.exec(
            r##"
            local created = {}
            local originalCreateFrame = CreateFrame
            __chat_config_original_create_frame = originalCreateFrame
            CreateFrame = function(frameType, name, parent, template, ...)
                local frame = originalCreateFrame(frameType, name, parent, template, ...)
                local nameStr = tostring(name)
                local templateStr = tostring(template)
                if nameStr:find("Checkbox") or nameStr:find("Check") or templateStr:find("Checkbox") or templateStr:find("Check") then
                    created[#created + 1] = table.concat({
                        tostring(frameType),
                        nameStr,
                        templateStr,
                        tostring(frame ~= nil and frame.CheckButton ~= nil),
                        tostring(_G[nameStr .. "Check"] ~= nil),
                    }, "|")
                end
                return frame
            end
            __chat_config_created = created
            "##,
        )
        .expect("chat config CreateFrame wrapper should install");
        {
            let mut state = env.state().borrow_mut();
            state.lua_errors.clear();
            state.lua_error_records.clear();
            state.lua_error_counts.clear();
        }
        collect_targeted_startup_messages(&env, &mut messages);
        env.exec(
            r##"
            if __chat_config_original_create_frame ~= nil then
                CreateFrame = __chat_config_original_create_frame
            end
            "##,
        )
        .ok();
        let created: String = env
            .eval(
                r##"
                return table.concat(__chat_config_created or {}, "\n")
                "##,
            )
            .expect("created checkbox log should stringify");
        let targeted: Vec<String> = env
            .state()
            .borrow()
            .lua_errors
            .clone()
            .into_iter()
            .filter(|message| message.contains("CheckButton"))
            .collect();

        assert!(
            targeted.is_empty(),
            "Startup channel checkbox creation should not emit CheckButton errors.\ncreated:\n{created}\nerrors:\n  {}",
            targeted.join("\n  ")
        );
    }
}

#[test]
fn startup_expected_number_error_has_traceback() {
    test_timeout! {
        let env = load_and_startup_env();
        let targeted: Vec<String> = env
            .state()
            .borrow()
            .lua_error_records
            .iter()
            .filter(|record| record.message.contains("expected number, got nil at argument 1"))
            .map(|record| {
                let addon = record.addon_name.as_deref().unwrap_or("<none>");
                format!("[{addon}] {}", record.message)
            })
            .collect();

        assert!(
            targeted.is_empty(),
            "Startup should not report the numeric-argument regression:
    {}",
            targeted.join("
    ")
        );
    }
}

#[test]
fn startup_catalog_shop_numeric_error_after_load_clear() {
    test_timeout! {
        let mut messages = Vec::new();
        let env = load_targeted_startup_env(&mut messages);
        {
            let mut state = env.state().borrow_mut();
            state.lua_errors.clear();
            state.lua_error_records.clear();
            state.lua_error_counts.clear();
        }
        collect_targeted_startup_messages(&env, &mut messages);
        let targeted: Vec<String> = env
            .state()
            .borrow()
            .lua_errors
            .clone()
            .into_iter()
            .filter(|message| message.contains("expected number, got nil at argument 1"))
            .collect();
        let traced: Vec<String> = messages
            .into_iter()
            .filter(|message| message.contains("expected number, got nil at argument 1"))
            .collect();

        assert!(
            targeted.is_empty(),
            "CatalogShop numeric error should be absent after clearing load-time errors if it is load-only.\nstate errors:\n  {}\ntracebacks:\n  {}",
            targeted.join("\n  "),
            traced.join("\n  ")
        );
    }
}

#[test]
fn loading_blizzard_addons_does_not_emit_catalog_shop_numeric_error() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);

        for (name, toc_path) in &addons {
            let before = env.state().borrow().lua_error_records.len();
            load_addon(&env.loader_env(), toc_path)
                .unwrap_or_else(|error| panic!("{name} should load: {error}"));
            let records = env.state().borrow().lua_error_records.clone();
            let targeted: Vec<String> = records
                .into_iter()
                .skip(before)
                .filter(|record| {
                    record.addon_name.as_deref() == Some("Blizzard_CatalogShop")
                        && record
                            .message
                            .contains("expected number, got nil at argument 1")
                })
                .map(|record| {
                    let addon = record.addon_name.unwrap_or_else(|| "<none>".to_string());
                    format!("[{addon}] {}", record.message)
                })
                .collect();

            assert!(
                targeted.is_empty(),
                "{name} load introduced the CatalogShop numeric error:\n  {}",
                targeted.join("\n  ")
            );
        }
    }
}

#[test]
fn apply_post_load_workarounds_does_not_introduce_catalog_shop_numeric_error() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);
        for (_name, toc_path) in &addons {
            load_addon(&env.loader_env(), toc_path).expect("Failed to load Blizzard addon");
        }

        {
            let mut state = env.state().borrow_mut();
            state.lua_errors.clear();
            state.lua_error_records.clear();
            state.lua_error_counts.clear();
        }

        env.apply_post_load_workarounds();
        let targeted: Vec<String> = env
            .state()
            .borrow()
            .lua_errors
            .clone()
            .into_iter()
            .filter(|message| message.contains("expected number, got nil at argument 1"))
            .collect();

        assert!(
            targeted.is_empty(),
            "apply_post_load_workarounds should not introduce the CatalogShop numeric error:\n  {}",
            targeted.join("\n  ")
        );
    }
}

#[test]
fn startup_checkbutton_errors_report_addon_names() {
    test_timeout! {
        let env = load_and_startup_env();
        let mut targeted: Vec<String> = env
            .state()
            .borrow()
            .lua_error_records
            .iter()
            .filter(|record| record.message.contains("CheckButton"))
            .map(|record| {
                let addon = record.addon_name.as_deref().unwrap_or("<none>");
                format!("[{addon}] {}", record.message)
            })
            .collect();
        targeted.sort();
        targeted.dedup();

        assert!(
            targeted.is_empty(),
            "Startup should not report CheckButton regressions:
    {}",
            targeted.join("
    ")
        );
    }
}

#[test]
fn startup_targeted_errors_have_tracebacks() {
    test_timeout! {
        let mut messages = Vec::new();
        let env = load_with_early_error_collector(&mut messages);
        collect_targeted_startup_messages(&env, &mut messages);
        let targeted: Vec<String> = messages
            .into_iter()
            .filter(|message| {
                message.contains("expected number, got nil at argument 1")
                    || message.contains("CheckButton")
            })
            .collect();

        assert!(
            targeted.is_empty(),
            "Startup targeted errors should be gone:
    {}",
            targeted.join("
    ")
        );
    }
}

#[test]
fn load_time_targeted_errors_have_tracebacks() {
    test_timeout! {
        let mut messages = Vec::new();
        let _env = load_with_early_error_collector(&mut messages);
        let targeted: Vec<String> = messages
            .into_iter()
            .filter(|message| {
                message.contains("CheckButton")
                    || message.contains("expected number, got nil at argument 1")
                    || message.contains("expected number, got string at argument 1")
            })
            .collect();

        assert!(
            targeted.is_empty(),
            "Load-time targeted errors should be gone:
    {}",
            targeted.join("
    ")
        );
    }
}
