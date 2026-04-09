//! Tests for system_api.rs: type(), rawget(), xpcall(), BN*, build type stubs.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// type() override (frame userdata awareness)
// ============================================================================

#[test]
fn test_type_number() {
    let env = env();
    let t: String = env.eval("return type(42)").unwrap();
    assert_eq!(t, "number");
}

#[test]
fn test_type_string() {
    let env = env();
    let t: String = env.eval("return type('hello')").unwrap();
    assert_eq!(t, "string");
}

#[test]
fn test_type_table() {
    let env = env();
    let t: String = env.eval("return type({})").unwrap();
    assert_eq!(t, "table");
}

#[test]
fn test_type_nil() {
    let env = env();
    let t: String = env.eval("return type(nil)").unwrap();
    assert_eq!(t, "nil");
}

#[test]
fn test_type_boolean() {
    let env = env();
    let t: String = env.eval("return type(true)").unwrap();
    assert_eq!(t, "boolean");
}

#[test]
fn test_type_function() {
    let env = env();
    let t: String = env.eval("return type(print)").unwrap();
    assert_eq!(t, "function");
}

#[test]
fn test_type_frame_returns_table() {
    let env = env();
    let t: String = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "TestTypeFrame", UIParent)
        return type(f)
    "#,
        )
        .unwrap();
    assert_eq!(t, "table", "type() on frame userdata should return 'table'");
}

// ============================================================================
// rawget()
// ============================================================================

#[test]
fn test_rawget_table() {
    let env = env();
    let val: i32 = env
        .eval(
            r#"
        local t = {a = 42}
        return rawget(t, "a")
    "#,
        )
        .unwrap();
    assert_eq!(val, 42);
}

#[test]
fn test_rawget_bypasses_metatable() {
    let env = env();
    let is_nil: bool = env
        .eval(
            r#"
        local t = setmetatable({}, {__index = function() return 99 end})
        return rawget(t, "missing") == nil
    "#,
        )
        .unwrap();
    assert!(is_nil, "rawget should bypass metatable __index");
}

// ============================================================================
// xpcall()
// ============================================================================

#[test]
fn test_xpcall_success() {
    let env = env();
    let (ok, result): (bool, i32) = env
        .eval(
            r#"
        return xpcall(function() return 42 end, function(err) return err end)
    "#,
        )
        .unwrap();
    assert!(ok);
    assert_eq!(result, 42);
}

#[test]
fn test_xpcall_error() {
    let env = env();
    let (ok, msg): (bool, String) = env
        .eval(
            r#"
        return xpcall(function() error("boom") end, function(err) return "handled: " .. err end)
    "#,
        )
        .unwrap();
    assert!(!ok);
    assert!(msg.contains("handled:"), "Error handler should be called");
}

#[test]
fn test_xpcall_passes_args() {
    let env = env();
    let (ok, result): (bool, i32) = env
        .eval(
            r#"
        return xpcall(function(a, b) return a + b end, function(err) return err end, 10, 20)
    "#,
        )
        .unwrap();
    assert!(ok);
    assert_eq!(result, 30);
}

// ============================================================================
// SlashCmdList
// ============================================================================

#[test]
fn test_slash_cmd_list_is_table() {
    let env = env();
    let is_table: bool = env.eval("return type(SlashCmdList) == 'table'").unwrap();
    assert!(is_table);
}

// ============================================================================
// Build type stubs
// ============================================================================

#[test]
fn test_is_public_test_client() {
    let env = env();
    let val: bool = env.eval("return IsPublicTestClient()").unwrap();
    assert!(!val);
}

#[test]
fn test_is_beta_build() {
    let env = env();
    let val: bool = env.eval("return IsBetaBuild()").unwrap();
    assert!(!val);
}

#[test]
fn test_is_public_build() {
    let env = env();
    let val: bool = env.eval("return IsPublicBuild()").unwrap();
    assert!(val);
}

#[test]
fn test_get_net_stats_returns_seeded_latency_values() {
    let env = env();
    let (bandwidth_in, bandwidth_out, latency_home, latency_world): (f64, f64, f64, f64) =
        env.eval("return GetNetStats()").unwrap();
    assert!(
        bandwidth_in > 0.0,
        "GetNetStats should seed incoming bandwidth for network stats UI"
    );
    assert!(
        bandwidth_out > 0.0,
        "GetNetStats should seed outgoing bandwidth for network stats UI"
    );
    assert!(
        latency_home > 0.0,
        "GetNetStats should seed home latency for network stats UI"
    );
    assert!(
        latency_world > 0.0,
        "GetNetStats should seed world latency for network stats UI"
    );
}

#[test]
fn test_c_encounter_timeline_returns_seeded_visible_event() {
    let env = env();
    let result: String = env
        .eval(
            r#"
        if not C_EncounterTimeline.IsFeatureAvailable() then
            return "feature_unavailable"
        end
        if not C_EncounterTimeline.IsFeatureEnabled() then
            return "feature_disabled"
        end

        local eventIDs = C_EncounterTimeline.GetEventList()
        if #eventIDs ~= 1 then
            return "event_count=" .. tostring(#eventIDs)
        end

        local eventID = eventIDs[1]
        local info = C_EncounterTimeline.GetEventInfo(eventID)
        if not info then
            return "missing_event_info"
        end
        if info.spellID ~= 19750 then
            return "spell_id=" .. tostring(info.spellID)
        end
        if info.spellName ~= "Flash of Light" then
            return "spell_name=" .. tostring(info.spellName)
        end

        local timer = C_EncounterTimeline.GetEventTimer(eventID)
        if not timer then
            return "missing_timer"
        end
        if timer:GetRemainingDuration() <= 0 then
            return "remaining_duration=" .. tostring(timer:GetRemainingDuration())
        end

        local track, trackSortIndex = C_EncounterTimeline.GetEventTrack(eventID)
        if track ~= Enum.EncounterTimelineTrack.Short then
            return "track=" .. tostring(track)
        end
        if trackSortIndex ~= 1 then
            return "track_sort_index=" .. tostring(trackSortIndex)
        end

        if not C_EncounterTimeline.HasActiveEvents() then
            return "missing_active_events"
        end
        if not C_EncounterTimeline.HasVisibleEvents() then
            return "missing_visible_events"
        end

        return "ok"
    "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "C_EncounterTimeline should expose a seeded visible event with timer data: {result}"
    );
}

#[test]
fn test_c_damage_meter_returns_seeded_session_and_source_data() {
    let env = env();
    let result: String = env
        .eval(
            r#"
        local isAvailable, failureReason = C_DamageMeter.IsDamageMeterAvailable()
        if not isAvailable then
            return "unavailable:" .. tostring(failureReason)
        end

        local availableSessions = C_DamageMeter.GetAvailableCombatSessions()
        if #availableSessions ~= 1 then
            return "session_count=" .. tostring(#availableSessions)
        end

        local sessionInfo = availableSessions[1]
        if sessionInfo.sessionID ~= 1 then
            return "session_id=" .. tostring(sessionInfo.sessionID)
        end

        local overallSession = C_DamageMeter.GetCombatSessionFromType(
            Enum.DamageMeterSessionType.Overall,
            Enum.DamageMeterType.DamageDone
        )
        if not overallSession then
            return "missing_overall_session"
        end
        if overallSession.totalAmount <= 0 then
            return "overall_total=" .. tostring(overallSession.totalAmount)
        end
        if #overallSession.combatSources ~= 2 then
            return "source_count=" .. tostring(#overallSession.combatSources)
        end

        local topSource = overallSession.combatSources[1]
        if topSource.name ~= "Player" then
            return "top_source=" .. tostring(topSource.name)
        end
        if not topSource.isLocalPlayer then
            return "top_source_not_local"
        end

        local sourceSession = C_DamageMeter.GetCombatSessionSourceFromType(
            Enum.DamageMeterSessionType.Overall,
            Enum.DamageMeterType.DamageDone,
            topSource.sourceGUID,
            topSource.sourceCreatureID
        )
        if not sourceSession then
            return "missing_source_session"
        end
        if sourceSession.totalAmount ~= topSource.totalAmount then
            return "source_total=" .. tostring(sourceSession.totalAmount)
        end
        if #sourceSession.combatSpells == 0 then
            return "spell_count=0"
        end
        if sourceSession.combatSpells[1].spellID ~= 19750 then
            return "spell_id=" .. tostring(sourceSession.combatSpells[1].spellID)
        end

        local durationSeconds = C_DamageMeter.GetSessionDurationSeconds(Enum.DamageMeterSessionType.Overall)
        if durationSeconds <= 0 then
            return "duration=" .. tostring(durationSeconds)
        end

        return "ok"
    "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "C_DamageMeter should expose seeded session, source, and spell data: {result}"
    );
}

// ============================================================================
// Battle.net stubs
// ============================================================================

#[test]
fn test_bn_features_enabled() {
    let env = env();
    let val: bool = env.eval("return BNFeaturesEnabled()").unwrap();
    assert!(!val);
}

#[test]
fn test_bn_features_enabled_and_connected() {
    let env = env();
    let val: bool = env.eval("return BNFeaturesEnabledAndConnected()").unwrap();
    assert!(!val);
}

#[test]
fn test_bn_connected() {
    let env = env();
    let val: bool = env.eval("return BNConnected()").unwrap();
    assert!(val);
}

#[test]
fn test_bn_get_num_friends() {
    let env = env();
    let (total, online): (i32, i32) = env.eval("return BNGetNumFriends()").unwrap();
    assert_eq!(total, 0);
    assert_eq!(online, 0);
}

#[test]
fn test_bn_get_friend_info_nil() {
    let env = env();
    let is_nil: bool = env.eval("return BNGetFriendInfo(1) == nil").unwrap();
    assert!(is_nil);
}

#[test]
fn test_c_friend_list_returns_seeded_wow_friends() {
    let env = env();
    let result: String = env
        .eval(
            r#"
        if C_FriendList.GetNumFriends() ~= 2 then
            return "friend_count=" .. tostring(C_FriendList.GetNumFriends())
        end
        if C_FriendList.GetNumOnlineFriends() ~= 1 then
            return "online_count=" .. tostring(C_FriendList.GetNumOnlineFriends())
        end

        local info = C_FriendList.GetFriendInfoByIndex(1)
        if not info then
            return "missing_index_1"
        end
        if info.name ~= "Alyth" then
            return "name=" .. tostring(info.name)
        end
        if not info.connected then
            return "friend_should_be_online"
        end
        if info.level ~= 80 then
            return "level=" .. tostring(info.level)
        end
        if info.className ~= "Paladin" then
            return "class=" .. tostring(info.className)
        end
        if info.area ~= "Stormwind City" then
            return "area=" .. tostring(info.area)
        end
        if info.notes ~= "Testing the FriendsFrame list" then
            return "notes=" .. tostring(info.notes)
        end
        if info.guid ~= "Player-11-00000001" then
            return "guid=" .. tostring(info.guid)
        end

        local by_name = C_FriendList.GetFriendInfoByName("Alyth")
        if not by_name or by_name.guid ~= info.guid then
            return "name_lookup_failed"
        end
        if not C_FriendList.IsFriend("Alyth") then
            return "is_friend_failed"
        end
        if C_FriendList.GetFriendInfoByIndex(99) ~= nil then
            return "unexpected_friend_at_99"
        end
        return "ok"
    "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "C_FriendList should expose seeded WoW friends: {result}"
    );
}

#[test]
fn test_c_loss_of_control_returns_seeded_active_event() {
    let env = env();
    let result: String = env
        .eval(
            r#"
        if C_LossOfControl.GetActiveLossOfControlDataCount() ~= 1 then
            return "global_count=" .. tostring(C_LossOfControl.GetActiveLossOfControlDataCount())
        end

        local data = C_LossOfControl.GetActiveLossOfControlData(1)
        if not data then
            return "missing_global_data"
        end
        if data.spellID ~= 408 then
            return "spell=" .. tostring(data.spellID)
        end
        if data.displayText ~= "Kidney Shot" then
            return "text=" .. tostring(data.displayText)
        end
        if data.iconTexture ~= "Interface\\Icons\\Ability_Rogue_KidneyShot" then
            return "icon=" .. tostring(data.iconTexture)
        end
        if data.displayType ~= 2 then
            return "display_type=" .. tostring(data.displayType)
        end
        if data.timeRemaining ~= 4 then
            return "time_remaining=" .. tostring(data.timeRemaining)
        end

        if C_LossOfControl.GetActiveLossOfControlData(2) ~= nil then
            return "unexpected_global_index_2"
        end

        if C_LossOfControl.GetActiveLossOfControlDataCountByUnit("player") ~= 1 then
            return "player_count=" .. tostring(C_LossOfControl.GetActiveLossOfControlDataCountByUnit("player"))
        end
        if C_LossOfControl.GetActiveLossOfControlDataByUnit("player", 1) == nil then
            return "missing_player_data"
        end
        if C_LossOfControl.GetActiveLossOfControlDuration("player", 1) ~= 4 then
            return "duration=" .. tostring(C_LossOfControl.GetActiveLossOfControlDuration("player", 1))
        end

        if C_LossOfControl.GetActiveLossOfControlDataCountByUnit("focus") ~= 0 then
            return "unexpected_focus_count"
        end
        if C_LossOfControl.GetActiveLossOfControlDataByUnit("focus", 1) ~= nil then
            return "unexpected_focus_data"
        end
        if C_LossOfControl.GetActiveLossOfControlDuration("focus", 1) ~= nil then
            return "unexpected_focus_duration"
        end

        return "ok"
    "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_LossOfControl should expose seeded active loss-of-control data: {result}"
    );
}

#[test]
fn test_macro_apis_return_seeded_macros() {
    let env = env();
    let result: String = env
        .eval(
            r#"
        local accountCount, characterCount = GetNumMacros()
        if accountCount ~= 2 or characterCount ~= 1 then
            return "counts=" .. tostring(accountCount) .. "," .. tostring(characterCount)
        end

        local accountName, accountIcon, accountBody = GetMacroInfo(1)
        if accountName ~= "Raid Beacon" then
            return "account_name=" .. tostring(accountName)
        end
        if accountIcon ~= "Interface\\Icons\\INV_Misc_QuestionMark" then
            return "account_icon=" .. tostring(accountIcon)
        end
        if accountBody ~= "/rw Stack on star" then
            return "account_body=" .. tostring(accountBody)
        end

        local characterName, characterIcon, characterBody = GetMacroInfo(121)
        if characterName ~= "Crusader" then
            return "character_name=" .. tostring(characterName)
        end
        if characterIcon ~= "Interface\\Icons\\Spell_Holy_CrusaderAura" then
            return "character_icon=" .. tostring(characterIcon)
        end
        if characterBody ~= "/cast Crusader Aura" then
            return "character_body=" .. tostring(characterBody)
        end

        if GetMacroInfo(999) ~= nil then
            return "unexpected_macro_999"
        end
        if C_Macro.GetMacroName(1) ~= "Raid Beacon" then
            return "c_macro_name=" .. tostring(C_Macro.GetMacroName(1))
        end
        if C_Macro.GetSelectedMacroIcon(121) ~= "Interface\\Icons\\Spell_Holy_CrusaderAura" then
            return "c_macro_icon=" .. tostring(C_Macro.GetSelectedMacroIcon(121))
        end
        local cAccountCount, cCharacterCount = C_Macro.GetNumMacros()
        if cAccountCount ~= 2 or cCharacterCount ~= 1 then
            return "c_counts=" .. tostring(cAccountCount) .. "," .. tostring(cCharacterCount)
        end
        return "ok"
    "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Macro APIs should expose seeded macros: {result}"
    );
}

// ============================================================================
// PlayerLocation
// ============================================================================

#[test]
fn test_player_location_guid_uses_guid_validity_and_clear_invalidates() {
    let env = env();
    let (is_guid, is_unit, guid_matches, valid_before, valid_after): (
        bool,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
        C_PlayerInfo.GUIDIsPlayer = function(guid)
            return guid == "Player-3676-00000001"
        end
        C_AccountInfo.IsGUIDBattleNetAccountType = function()
            return false
        end

        local location = PlayerLocation:CreateFromGUID("Player-3676-00000001")
        local valid_before = location:IsValid()
        local guid_matches = location:GetGUID() == "Player-3676-00000001"
        location:Clear()

        return location:IsGUID(), location:IsUnit(), guid_matches, valid_before, location:IsValid()
    "#,
        )
        .unwrap();
    assert!(!is_guid, "Clear should remove the GUID source kind");
    assert!(
        !is_unit,
        "GUID locations should not report as unit locations"
    );
    assert!(
        guid_matches,
        "GUID locations should preserve the original GUID"
    );
    assert!(
        valid_before,
        "GUID validity should come from the GUID resolver"
    );
    assert!(!valid_after, "Cleared locations should become invalid");
}

#[test]
fn test_player_location_unit_uses_unit_validity() {
    let env = env();
    let (player_valid, pet_valid, player_is_unit, pet_is_unit): (bool, bool, bool, bool) = env
        .eval(
            r#"
        UnitIsHumanPlayer = function(unit)
            return unit == "player"
        end

        local player_location = PlayerLocation:CreateFromUnit("player")
        local pet_location = PlayerLocation:CreateFromUnit("pet")
        return player_location:IsValid(), pet_location:IsValid(), player_location:IsUnit(), pet_location:IsUnit()
    "#,
        )
        .unwrap();
    assert!(
        player_valid,
        "player unit should be valid when UnitIsHumanPlayer says so"
    );
    assert!(!pet_valid, "non-human unit locations should be invalid");
    assert!(player_is_unit);
    assert!(pet_is_unit);
}

#[test]
fn test_player_location_preserves_non_guid_source_kinds() {
    let env = env();
    let (community_valid, bnet_valid, community_kind, bnet_kind, clear_invalid): (
        bool,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
        C_Club.CanResolvePlayerLocationFromClubMessageData = function(clubID, streamID, epoch, position)
            return clubID == 7 and streamID == 11 and epoch == 13 and position == 17
        end

        local community = PlayerLocation:CreateFromCommunityChatData(7, 11, 13, 17)
        local bnet = PlayerLocation:CreateFromBattleNetID(42)
        local cleared = PlayerLocation:CreateFromVoiceID(3, 9)
        cleared:Clear()

        return community:IsValid(), bnet:IsValid(), community:IsCommunityData(), bnet:IsBattleNetID(), cleared:IsValid()
    "#,
        )
        .unwrap();
    assert!(
        community_valid,
        "community chat locations should use the resolver result"
    );
    assert!(
        bnet_valid,
        "battle.net locations should remain valid while their source kind is set"
    );
    assert!(
        community_kind,
        "community locations should preserve their source kind"
    );
    assert!(
        bnet_kind,
        "battle.net locations should preserve their source kind"
    );
    assert!(
        !clear_invalid,
        "cleared voice locations should become invalid"
    );
}

#[test]
fn test_bn_get_info() {
    let env = env();
    // BNGetInfo returns mock data, just verify it doesn't error
    env.exec("local info = BNGetInfo()").unwrap();
}

// ============================================================================
// FireEvent / ReloadUI
// ============================================================================

#[test]
fn test_fire_event_no_error() {
    let env = env();
    env.exec(r#"FireEvent("PLAYER_ENTERING_WORLD")"#).unwrap();
}

#[test]
fn test_reload_ui_exists() {
    let env = env();
    let is_func: bool = env.eval("return type(ReloadUI) == 'function'").unwrap();
    assert!(is_func);
}

// ============================================================================
// Generated stub overrides
// ============================================================================

#[test]
fn test_ambiguate_preserves_plain_names() {
    let env = env();
    let value: String = env.eval(r#"return Ambiguate("Thrall", "all")"#).unwrap();
    assert_eq!(value, "Thrall");
}

#[test]
fn test_ambiguate_strips_realm_for_common_contexts() {
    let env = env();
    let (all_name, short_name, guild_name): (String, String, String) = env
        .eval(
            r#"
        return Ambiguate("Thrall-Area52", "all"),
               Ambiguate("Thrall-Area52", "short"),
               Ambiguate("Thrall-Area52", "guild")
    "#,
        )
        .unwrap();
    assert_eq!(all_name, "Thrall");
    assert_eq!(short_name, "Thrall");
    assert_eq!(guild_name, "Thrall");
}

#[test]
fn test_ambiguate_none_keeps_full_name() {
    let env = env();
    let value: String = env
        .eval(r#"return Ambiguate("Thrall-Area52", "none")"#)
        .unwrap();
    assert_eq!(value, "Thrall-Area52");
}

#[test]
fn test_are_talents_locked_returns_false() {
    let env = env();
    let value: bool = env.eval("return AreTalentsLocked()").unwrap();
    assert!(!value);
}
