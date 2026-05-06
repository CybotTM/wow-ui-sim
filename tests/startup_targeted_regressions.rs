mod common;

use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

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

fn new_targeted_startup_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env
}

fn load_blizzard_addons_collecting(
    env: &WowLuaEnv,
    messages: &mut Vec<String>,
    mut after_load: impl FnMut(&WowLuaEnv, &mut Vec<String>),
) {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        let result = load_addon(&env.loader_env(), toc_path);
        push_addon_load_messages(messages, name, result);
        after_load(env, messages);
    }
}

fn load_targeted_startup_env(messages: &mut Vec<String>) -> WowLuaEnv {
    let env = new_targeted_startup_env();
    load_blizzard_addons_collecting(&env, messages, |_, _| {});

    env.apply_post_load_workarounds();
    common::install_error_collector(&env, "__targeted_startup_errors");
    env
}

fn load_with_early_error_collector(messages: &mut Vec<String>) -> WowLuaEnv {
    let env = new_targeted_startup_env();
    common::install_error_collector(&env, "__targeted_startup_errors");
    load_blizzard_addons_collecting(&env, messages, |env, messages| {
        drain_startup_errors(env, messages);
    });

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
fn startup_collections_journal_closes_on_escape() {
    test_timeout! {
        let env = load_and_startup_env();

        env.exec("ToggleCollectionsJournal(COLLECTIONS_JOURNAL_TAB_INDEX_APPEARANCES)")
            .expect("collections journal should open");
        let opened: bool = env
            .eval("return CollectionsJournal and CollectionsJournal:IsShown()")
            .expect("collections journal open state should be readable");
        assert!(opened, "CollectionsJournal should be shown before Escape");

        env.send_key_press("ESCAPE", None)
            .expect("Escape dispatch should succeed");
        let closed_without_menu: bool = env
            .eval(
                "return (not CollectionsJournal:IsShown()) \
                    and (GameMenuFrame == nil or not GameMenuFrame:IsShown())",
            )
            .expect("collections journal closed state should be readable");
        assert!(
            closed_without_menu,
            "Escape should close CollectionsJournal through the UIPanel close path"
        );
    }
}

#[test]
fn startup_adventure_guide_closes_on_escape() {
    test_timeout! {
        let env = load_and_startup_env();

        env.exec("ToggleEncounterJournal()")
            .expect("adventure guide should open");
        let opened: bool = env
            .eval("return EncounterJournal and EncounterJournal:IsShown()")
            .expect("adventure guide open state should be readable");
        assert!(opened, "EncounterJournal should be shown before Escape");

        env.send_key_press("ESCAPE", None)
            .expect("Escape dispatch should succeed");
        let closed_without_menu: bool = env
            .eval(
                "return (not EncounterJournal:IsShown()) \
                    and (GameMenuFrame == nil or not GameMenuFrame:IsShown())",
            )
            .expect("adventure guide closed state should be readable");
        assert!(
            closed_without_menu,
            "Escape should close EncounterJournal through the UIPanel close path"
        );
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

#[path = "startup_targeted_regressions/late.rs"]
mod late;
