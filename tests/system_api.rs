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
