use super::{assert_ptr_source_omits_qualified_methods, load_game_ui_without_player_choice};

const SNAPSHOT_ONLY_METHODS: &[&str] = &[
    "BuildCharacterClassDisplayText",
    "BuildCharacterLevelDisplayText",
    "BuildCharacterNameDisplayText",
    "BuildFriendNameDisplayText",
    "BuildLocationDisplayText",
    "BuildTooltipBroadcastText",
    "GameStateUsesFactions",
    "GetBattleNetFriendGameAccountInfoIfExactlyOneDirectInviteTargetExists",
    "GetBattleNetFriendInviteInfo",
    "GetBattleNetFriendInviteTypeLabel",
    "GetBattleNetFriendPartyInviteRestrictionText",
    "GetBattleNetFriendPartyInviteRestriction",
    "GetFormattedCharacterName",
    "GetFriendAccountNameText",
    "GetFriendNameColorForFriendType",
    "GetFriendNameDisplayColor",
    "GetFriendNameOfflineDisplayColor",
    "GetGameAccountPartyInviteRestriction",
    "GetLastOnlineText",
    "GetRegionName",
    "GetRelativeTimeText",
    "HasMultipleGameAccounts",
    "InviteOrRequestToJoin",
    "IsPlayingDifferentWoWProject",
    "IsPlayingSameWoWProject",
    "IsPlayingWoW",
    "IsRequestInviteType",
    "IsTitleFriend",
    "ShouldShowRichPresenceOnly",
];

/// Proves all proposed FriendsListUtil additions are absent from PTR source and runtime.
#[test]
fn snapshot_only_friends_list_methods_remain_absent() {
    assert_ptr_source_omits_qualified_methods("FriendsListUtil", SNAPSHOT_ONLY_METHODS);

    let env = load_game_ui_without_player_choice();
    let (namespace_type, absent_count): (String, i32) = env
        .eval(
            r#"
            local names = {
                "BuildCharacterClassDisplayText",
                "BuildCharacterLevelDisplayText",
                "BuildCharacterNameDisplayText",
                "BuildFriendNameDisplayText",
                "BuildLocationDisplayText",
                "BuildTooltipBroadcastText",
                "GameStateUsesFactions",
                "GetBattleNetFriendGameAccountInfoIfExactlyOneDirectInviteTargetExists",
                "GetBattleNetFriendInviteInfo",
                "GetBattleNetFriendInviteTypeLabel",
                "GetBattleNetFriendPartyInviteRestrictionText",
                "GetBattleNetFriendPartyInviteRestriction",
                "GetFormattedCharacterName",
                "GetFriendAccountNameText",
                "GetFriendNameColorForFriendType",
                "GetFriendNameDisplayColor",
                "GetFriendNameOfflineDisplayColor",
                "GetGameAccountPartyInviteRestriction",
                "GetLastOnlineText",
                "GetRegionName",
                "GetRelativeTimeText",
                "HasMultipleGameAccounts",
                "InviteOrRequestToJoin",
                "IsPlayingDifferentWoWProject",
                "IsPlayingSameWoWProject",
                "IsPlayingWoW",
                "IsRequestInviteType",
                "IsTitleFriend",
                "ShouldShowRichPresenceOnly",
            }
            local absentCount = 0
            for _, name in ipairs(names) do
                if FriendsListUtil == nil or FriendsListUtil[name] == nil then
                    absentCount = absentCount + 1
                end
            end
            return type(FriendsListUtil), absentCount
            "#,
        )
        .expect("FriendsListUtil runtime probe succeeds");

    assert_eq!(namespace_type, "nil");
    assert_eq!(absent_count, SNAPSHOT_ONLY_METHODS.len() as i32);
}
