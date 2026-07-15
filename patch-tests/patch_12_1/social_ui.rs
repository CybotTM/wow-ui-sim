use super::{assert_ptr_source_omits_qualified_methods, load_game_ui_without_player_choice};

const SNAPSHOT_ONLY_METHODS: &[&str] = &[
    "AddSeparatorToTooltip",
    "GetBattleNetFriendTagInterestsUIOrder",
    "GetBattleNetFriendTagRoleUIOrder",
    "GetBlockedName",
    "GetIconForPresenceType",
    "GetLabelForBattleNetFriendTag",
    "GetLabelForPresenceType",
    "GetPresenceTypeForBattleNetAccountInfo",
    "GetPresenceTypeSelf",
    "InitializeUserScaledDropdownButton",
    "InitializeUserScaledDropdownMainTitle",
    "InitializeUserScaledDropdownTitle",
    "SetBattleNetPresenceFromSocialUIPresence",
];

/// Proves all proposed SocialUIUtil additions are absent from PTR source and runtime.
#[test]
fn snapshot_only_social_ui_methods_remain_absent() {
    assert_ptr_source_omits_qualified_methods("SocialUIUtil", SNAPSHOT_ONLY_METHODS);

    let env = load_game_ui_without_player_choice();
    let (namespace_type, absent_count): (String, i32) = env
        .eval(
            r#"
            local names = {
                "AddSeparatorToTooltip",
                "GetBattleNetFriendTagInterestsUIOrder",
                "GetBattleNetFriendTagRoleUIOrder",
                "GetBlockedName",
                "GetIconForPresenceType",
                "GetLabelForBattleNetFriendTag",
                "GetLabelForPresenceType",
                "GetPresenceTypeForBattleNetAccountInfo",
                "GetPresenceTypeSelf",
                "InitializeUserScaledDropdownButton",
                "InitializeUserScaledDropdownMainTitle",
                "InitializeUserScaledDropdownTitle",
                "SetBattleNetPresenceFromSocialUIPresence",
            }
            local absentCount = 0
            for _, name in ipairs(names) do
                if SocialUIUtil == nil or SocialUIUtil[name] == nil then
                    absentCount = absentCount + 1
                end
            end
            return type(SocialUIUtil), absentCount
            "#,
        )
        .expect("SocialUIUtil runtime probe succeeds");

    assert_eq!(namespace_type, "nil");
    assert_eq!(absent_count, SNAPSHOT_ONLY_METHODS.len() as i32);
}
