use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn perks_activities_remove_tracked_updates_state() {
    let env = env();
    let (removed, remaining_one, remaining_two, remaining_count, remove_count, last_removed): (
        bool,
        i64,
        i64,
        i64,
        i64,
        i64,
    ) = env
        .eval(
            r#"
            C_PerksActivities._state.trackedIDs = { 101, 202, 303 }
            C_PerksActivities._state.removeCount = 0
            C_PerksActivities._state.lastRemovedID = nil
            C_PerksActivities._state.activityInfoByID = {
                [303] = { id = 303, name = "Trading Post Task" },
            }
            C_PerksActivities._state.chatLinkByID = {
                [303] = "|cffffff00|Hperk:303|h[Trading Post Task]|h|r",
            }

            local removed = C_PerksActivities.RemoveTrackedPerksActivity("202")
            local tracked = C_PerksActivities.GetTrackedPerksActivities().trackedIDs
            local info = C_PerksActivities.GetPerksActivityInfo(303)
            local link = C_PerksActivities.GetPerksActivityChatLink(303)

            assert(info and info.id == 303 and info.name == "Trading Post Task")
            assert(link ~= nil and string.find(link, "perk:303", 1, true) ~= nil)

            return removed == true,
                tracked[1],
                tracked[2],
                #tracked,
                C_PerksActivities._state.removeCount,
                C_PerksActivities._state.lastRemovedID
            "#,
        )
        .unwrap();

    assert!(
        removed,
        "RemoveTrackedPerksActivity should report successful removal"
    );
    assert_eq!(remaining_one, 101, "first tracked id should remain");
    assert_eq!(remaining_two, 303, "second tracked id should remain");
    assert_eq!(remaining_count, 2, "one tracked id should be removed");
    assert_eq!(remove_count, 1, "remove count should increment");
    assert_eq!(last_removed, 202, "lastRemovedID should store removed id");
}

#[test]
fn store_glue_methods_are_state_backed() {
    let env = env();
    let (
        disconnect_on_logout,
        vas_ready,
        purchase_state,
        product_id,
        result_tag,
        request_count,
        update_count,
        queued_count,
        last_queued_guid,
    ): (bool, bool, i64, i64, String, i64, i64, i64, String) = env
        .eval(
            r#"
            C_StoreGlue._state.disconnectOnLogout = true
            C_StoreGlue._state.vasProductReady = true
            C_StoreGlue._state.purchaseStateByGuid = {
                ["Player-111"] = {
                    purchaseState = 3,
                    productID = 77,
                    result = "READY",
                }
            }
            C_StoreGlue._state.requestedQueueGuids = {}
            C_StoreGlue._state.requestCharacterQueueTimeCount = 0
            C_StoreGlue._state.updateVASPurchaseStatesCount = 0
            C_StoreGlue._state.lastRequestedQueueGuid = nil

            local purchaseState, productID, resultTag = C_StoreGlue.GetVASPurchaseStateInfo("Player-111")
            local firstQueueRequest = C_StoreGlue.RequestCharacterQueueTime("Player-111")
            local secondQueueRequest = C_StoreGlue.RequestCharacterQueueTime("Player-111")
            local firstUpdate = C_StoreGlue.UpdateVASPurchaseStates()
            local secondUpdate = C_StoreGlue.UpdateVASPurchaseStates()

            assert(firstQueueRequest and secondQueueRequest)
            assert(firstUpdate and secondUpdate)

            return C_StoreGlue.GetDisconnectOnLogout(),
                C_StoreGlue.GetVASProductReady(),
                purchaseState,
                productID,
                resultTag,
                C_StoreGlue._state.requestCharacterQueueTimeCount,
                C_StoreGlue._state.updateVASPurchaseStatesCount,
                #C_StoreGlue._state.requestedQueueGuids,
                C_StoreGlue._state.lastRequestedQueueGuid
            "#,
        )
        .unwrap();

    assert!(
        disconnect_on_logout,
        "disconnectOnLogout should read from _state"
    );
    assert!(vas_ready, "vasProductReady should read from _state");
    assert_eq!(
        purchase_state, 3,
        "purchase state should read from map entry"
    );
    assert_eq!(product_id, 77, "product id should read from map entry");
    assert_eq!(result_tag, "READY", "result should read from map entry");
    assert_eq!(request_count, 2, "queue request count should increment");
    assert_eq!(update_count, 2, "update count should increment");
    assert_eq!(
        queued_count, 2,
        "requested guid list should record requests"
    );
    assert_eq!(
        last_queued_guid, "Player-111",
        "last requested guid should be tracked"
    );
}

#[test]
fn video_options_set_window_size_updates_current_size() {
    let env = env();
    let (
        default_x,
        default_y,
        current_x,
        current_y,
        size_count,
        set_count,
        last_set_x,
        last_set_y,
    ): (i64, i64, i64, i64, i64, i64, i64, i64) = env
        .eval(
            r#"
            C_VideoOptions._state.defaultGameWindowSize = { x = 2560, y = 1440 }
            C_VideoOptions._state.currentGameWindowSize = { x = 1920, y = 1080 }
            C_VideoOptions._state.availableGameWindowSizes = {
                { x = 1280, y = 720 },
                { x = 1920, y = 1080 },
                { x = 2560, y = 1440 },
            }
            C_VideoOptions._state.setGameWindowSizeCount = 0
            C_VideoOptions._state.lastSetWindowSize = nil

            local defaultSize = C_VideoOptions.GetDefaultGameWindowSize(1)
            local sizes = C_VideoOptions.GetGameWindowSizes()
            local currentBefore = C_VideoOptions.GetCurrentGameWindowSize()
            assert(currentBefore.x == 1920 and currentBefore.y == 1080)
            assert(#sizes == 3)

            local changed = C_VideoOptions.SetGameWindowSize(1600, 900)
            assert(changed == true)

            local currentAfter = C_VideoOptions.GetCurrentGameWindowSize()
            local lastSet = C_VideoOptions._state.lastSetWindowSize

            return defaultSize.x,
                defaultSize.y,
                currentAfter.x,
                currentAfter.y,
                #sizes,
                C_VideoOptions._state.setGameWindowSizeCount,
                lastSet.x,
                lastSet.y
            "#,
        )
        .unwrap();

    assert_eq!(default_x, 2560, "default width should read from _state");
    assert_eq!(default_y, 1440, "default height should read from _state");
    assert_eq!(
        current_x, 1600,
        "current width should update after SetGameWindowSize"
    );
    assert_eq!(
        current_y, 900,
        "current height should update after SetGameWindowSize"
    );
    assert_eq!(
        size_count, 3,
        "GetGameWindowSizes should return configured sizes"
    );
    assert_eq!(set_count, 1, "setGameWindowSizeCount should increment");
    assert_eq!(last_set_x, 1600, "last set width should be tracked");
    assert_eq!(last_set_y, 900, "last set height should be tracked");
}

#[test]
fn video_options_official_api_surface_returns_stable_shapes() {
    let env = env();
    let (
        current_x,
        current_y,
        default_x,
        default_y,
        size_count,
        first_size_x,
        first_size_y,
        adapter_count,
        first_adapter_name,
        first_adapter_low_power,
        second_adapter_external,
        spell_density_supported,
    ): (
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        String,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            C_VideoOptions._state.defaultGameWindowSize = { x = 2560, y = 1440 }
            C_VideoOptions._state.currentGameWindowSize = { x = 1920, y = 1080 }
            C_VideoOptions._state.availableGameWindowSizes = {
                { x = 1280, y = 720 },
                { x = 1920, y = 1080 },
            }
            C_VideoOptions._state.gxAdapterInfo = {
                { name = "Integrated GPU", isLowPower = true, isExternal = false },
                { name = "Dock GPU", isLowPower = false, isExternal = true },
            }

            local current = C_VideoOptions.GetCurrentGameWindowSize(0, true)
            local defaultSize = C_VideoOptions.GetDefaultGameWindowSize(1)
            local sizes = C_VideoOptions.GetGameWindowSizes(1, false)
            local adapters = C_VideoOptions.GetGxAdapterInfo()
            local spellDensitySupported = C_VideoOptions.IsSpellVisualDensitySystemSupported()

            assert(type(current) == "table" and current.x == 1920 and current.y == 1080)
            assert(type(defaultSize) == "table" and defaultSize.x == 2560 and defaultSize.y == 1440)
            assert(type(sizes) == "table" and #sizes == 2)
            assert(type(adapters) == "table" and #adapters == 2)
            assert(type(spellDensitySupported) == "boolean")

            return current.x,
                current.y,
                defaultSize.x,
                defaultSize.y,
                #sizes,
                sizes[1].x,
                sizes[1].y,
                #adapters,
                adapters[1].name,
                adapters[1].isLowPower,
                adapters[2].isExternal,
                spellDensitySupported
            "#,
        )
        .unwrap();

    assert_eq!(current_x, 1920, "current width should come from _state");
    assert_eq!(current_y, 1080, "current height should come from _state");
    assert_eq!(default_x, 2560, "default width should come from _state");
    assert_eq!(default_y, 1440, "default height should come from _state");
    assert_eq!(
        size_count, 2,
        "window sizes should preserve configured entries"
    );
    assert_eq!(
        first_size_x, 1280,
        "first window size width should match _state"
    );
    assert_eq!(
        first_size_y, 720,
        "first window size height should match _state"
    );
    assert_eq!(
        adapter_count, 2,
        "adapter list should preserve configured entries"
    );
    assert_eq!(
        first_adapter_name, "Integrated GPU",
        "adapter info should preserve names"
    );
    assert!(
        first_adapter_low_power,
        "adapter info should preserve low-power flags"
    );
    assert!(
        second_adapter_external,
        "adapter info should preserve external flags"
    );
    assert!(
        !spell_density_supported,
        "spell density support should remain disabled in the simulator"
    );
}

#[test]
fn perks_activities_monthly_accessors_return_stable_shapes() {
    let env = env();
    let (
        tag_type,
        tag_count,
        activities_type,
        activities_count,
        thresholds_type,
        thresholds_count,
        active_month,
        seconds_remaining,
        pending_type,
        pending_count,
    ): (String, i64, String, i64, String, i64, i64, i64, String, i64) = env
        .eval(
            r#"
            C_PerksActivities._state.activitiesInfo = nil
            C_PerksActivities._state.allTags = nil
            C_PerksActivities._state.pendingCompletion = nil

            local tags = C_PerksActivities.GetAllPerksActivityTags()
            local info = C_PerksActivities.GetPerksActivitiesInfo()
            local pending = C_PerksActivities.GetPerksActivitiesPendingCompletion()

            return type(tags.tagName),
                #tags.tagName,
                type(info.activities),
                #info.activities,
                type(info.thresholds),
                #info.thresholds,
                info.activePerksMonth,
                info.secondsRemaining,
                type(pending.pendingIDs),
                #pending.pendingIDs
            "#,
        )
        .unwrap();

    assert_eq!(tag_type, "table", "tagName should be a table");
    assert_eq!(tag_count, 0, "default tag list should be empty");
    assert_eq!(
        activities_type, "table",
        "activities should be a table for pairs/ipairs safety"
    );
    assert_eq!(
        activities_count, 0,
        "default activities list should be empty"
    );
    assert_eq!(
        thresholds_type, "table",
        "thresholds should be a table for pairs/ipairs safety"
    );
    assert_eq!(
        thresholds_count, 0,
        "default threshold list should be empty"
    );
    assert!(
        active_month >= 1,
        "activePerksMonth should default to a positive integer"
    );
    assert!(
        seconds_remaining >= 0,
        "secondsRemaining should default to a non-negative integer"
    );
    assert_eq!(
        pending_type, "table",
        "pendingIDs should be a table for ipairs safety"
    );
    assert_eq!(pending_count, 0, "pendingIDs should default to empty");
}

#[test]
fn encounter_journal_global_filters_and_tier_are_numeric() {
    let env = env();
    let (
        tier_type,
        class_before,
        spec_before,
        class_after,
        spec_after,
        tier_after,
        has_valid_difficulty,
    ): (String, i64, i64, i64, i64, i64, bool) = env
        .eval(
            r#"
            local tierBefore = EJ_GetCurrentTier()
            local classBefore, specBefore = EJ_GetLootFilter()

            EJ_SetLootFilter("3", nil)
            local classAfter, specAfter = EJ_GetLootFilter()

            EJ_SelectTier("11")
            local tierAfter = EJ_GetCurrentTier()

            return type(tierBefore),
                classBefore,
                specBefore,
                classAfter,
                specAfter,
                tierAfter,
                EJ_IsValidInstanceDifficulty(14)
            "#,
        )
        .unwrap();

    assert_eq!(tier_type, "number", "EJ_GetCurrentTier should be numeric");
    assert!(class_before >= 0, "default class filter should be numeric");
    assert!(spec_before >= 0, "default spec filter should be numeric");
    assert_eq!(
        class_after, 3,
        "class filter should normalize numeric input"
    );
    assert_eq!(spec_after, 0, "spec filter should normalize nil input to 0");
    assert_eq!(tier_after, 11, "EJ_SelectTier should update current tier");
    assert!(
        has_valid_difficulty,
        "numeric difficulties should be treated as valid"
    );
}

#[test]
fn combat_log_globals_have_stable_stub_behavior() {
    let env = env();
    let (
        reset_ok,
        add_filter_ok,
        set_entry_ok,
        clear_ok,
        set_retention_ok,
        current_entry,
        num_entries,
        show_current,
        advance_result,
        retention_time,
        event_info_is_nil,
        object_match,
        object_miss,
    ): (
        bool,
        bool,
        bool,
        bool,
        bool,
        i64,
        i64,
        bool,
        bool,
        f64,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local resetOk = pcall(CombatLogResetFilter)
            local addFilterOk = pcall(CombatLogAddFilter, "anything")
            local setEntryOk = pcall(CombatLogSetCurrentEntry, 5)
            local clearOk = pcall(CombatLogClearEntries)
            local setRetentionOk = pcall(CombatLogSetRetentionTime, 120)

            local currentEntry = CombatLogGetCurrentEntry()
            local numEntries = CombatLogGetNumEntries()
            local showCurrent = CombatLogShowCurrentEntry()
            local advanceResult = CombatLogAdvanceEntry(1)
            local retentionTime = CombatLogGetRetentionTime()
            local eventInfoIsNil = CombatLogGetCurrentEventInfo() == nil

            local objectMatch = CombatLog_Object_IsA(0x21, 0x01)
            local objectMiss = CombatLog_Object_IsA(0x20, 0x01)

            return resetOk,
                addFilterOk,
                setEntryOk,
                clearOk,
                setRetentionOk,
                currentEntry,
                numEntries,
                showCurrent,
                advanceResult,
                retentionTime,
                eventInfoIsNil,
                objectMatch,
                objectMiss
            "#,
        )
        .unwrap();

    assert!(reset_ok, "CombatLogResetFilter should be callable");
    assert!(add_filter_ok, "CombatLogAddFilter should be callable");
    assert!(set_entry_ok, "CombatLogSetCurrentEntry should be callable");
    assert!(clear_ok, "CombatLogClearEntries should be callable");
    assert!(
        set_retention_ok,
        "CombatLogSetRetentionTime should be callable"
    );
    assert_eq!(current_entry, 0, "current entry stub should default to 0");
    assert_eq!(num_entries, 0, "entry count stub should default to 0");
    assert!(!show_current, "show current entry stub should be false");
    assert!(!advance_result, "advance entry stub should be false");
    assert_eq!(
        retention_time, 300.0,
        "retention time stub should default to 300s"
    );
    assert!(event_info_is_nil, "current event info stub should be nil");
    assert!(object_match, "bitmask check should match overlapping flags");
    assert!(
        !object_miss,
        "bitmask check should fail for non-overlapping flags"
    );
}
