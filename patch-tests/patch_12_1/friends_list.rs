use std::{fs, path::Path};

use super::{blizzard_ui_dir, load_game_ui_without_player_choice};

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

fn assert_source_tree_omits_methods(path: &Path) {
    for entry in fs::read_dir(path).expect("PTR AddOns directory should be readable") {
        let entry = entry.expect("PTR source entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            assert_source_tree_omits_methods(&path);
            continue;
        }

        let extension = path.extension().and_then(|value| value.to_str());
        if !matches!(extension, Some("lua" | "xml" | "toc")) {
            continue;
        }

        let source = fs::read_to_string(&path).expect("PTR source file should be UTF-8 text");
        for method in SNAPSHOT_ONLY_METHODS {
            let qualified_method = format!("FriendsListUtil.{method}");
            assert!(
                !source.contains(&qualified_method),
                "snapshot-only method {qualified_method} unexpectedly appears in {}",
                path.display(),
            );
        }
    }
}

/// Proves all proposed FriendsListUtil additions are absent from PTR source and runtime.
#[test]
fn snapshot_only_friends_list_methods_remain_absent() {
    assert_source_tree_omits_methods(&blizzard_ui_dir());

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
