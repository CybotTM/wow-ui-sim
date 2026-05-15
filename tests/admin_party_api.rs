//! Tests for A_Admin party simulation API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetPartySize
// ============================================================================

#[test]
fn test_set_party_size_nonzero_means_in_group() {
    let env = env();
    let in_group: bool = env
        .eval(
            r#"
            A_Admin.SetPartySize(3)
            return IsInGroup()
            "#,
        )
        .unwrap();
    assert!(
        in_group,
        "IsInGroup() should return true when party size > 0"
    );
}

#[test]
fn test_set_party_size_zero_means_not_in_group() {
    let env = env();
    let in_group: bool = env
        .eval(
            r#"
            A_Admin.SetPartySize(0)
            return IsInGroup()
            "#,
        )
        .unwrap();
    assert!(
        !in_group,
        "IsInGroup() should return false when party size == 0"
    );
}

#[test]
fn test_group_has_offline_member_defaults_false() {
    let env = env();
    let has_offline: bool = env
        .eval(
            r#"
            A_Admin.SetPartySize(3)
            return GroupHasOfflineMember(LE_PARTY_CATEGORY_HOME)
            "#,
        )
        .unwrap();
    assert!(
        !has_offline,
        "GroupHasOfflineMember should be false until offline party state is modeled"
    );
}

#[test]
fn test_get_num_group_members_includes_player() {
    let env = env();
    // GetNumGroupMembers returns party count + 1 (for the player) when in a group.
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(3)
            return GetNumGroupMembers()
            "#,
        )
        .unwrap();
    assert_eq!(
        count, 4,
        "GetNumGroupMembers() should return party size + 1 (player)"
    );
}

#[test]
fn test_get_num_group_members_zero_when_no_party() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(0)
            return GetNumGroupMembers()
            "#,
        )
        .unwrap();
    assert_eq!(
        count, 0,
        "GetNumGroupMembers() should return 0 when not in a group"
    );
}

#[test]
fn test_get_num_raid_members_zero_for_party_size_under_six() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(4)
            return GetNumRaidMembers()
            "#,
        )
        .unwrap();
    assert_eq!(
        count, 0,
        "GetNumRaidMembers() should be 0 while the group is still a party"
    );
}

#[test]
fn test_get_num_raid_members_counts_raid_including_player() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(9)
            return GetNumRaidMembers()
            "#,
        )
        .unwrap();
    assert_eq!(
        count, 10,
        "GetNumRaidMembers() should return party size + 1 once the raid threshold is crossed"
    );
}

#[test]
fn test_get_raid_roster_info_covers_player_and_all_party_members() {
    let env = env();
    let (all_have_roster_data, last_subgroup): (bool, i32) = env
        .eval(
            r#"
            A_Admin.SetPartySize(9)
            local count = GetNumGroupMembers()
            for i = 1, count do
                local name, _, subgroup, _, _, classFile, _, _, _, _, _, role = GetRaidRosterInfo(i)
                if name == nil or subgroup == nil or classFile == nil or role == nil then
                    return false, -1
                end
            end
            local _, _, lastSubgroup = GetRaidRosterInfo(count)
            return true, lastSubgroup
            "#,
        )
        .unwrap();

    assert!(
        all_have_roster_data,
        "raid roster should include the player plus every simulated party member"
    );
    assert_eq!(
        last_subgroup, 2,
        "ten raid members should fill two five-player subgroups"
    );
}

#[test]
fn test_get_raid_roster_info_covers_forced_party_raid_frames() {
    let env = env();
    let (all_have_roster_data, last_subgroup): (bool, i32) = env
        .eval(
            r#"
            A_Admin.SetPartySize(4)
            local count = GetNumGroupMembers()
            for i = 1, count do
                local name, _, subgroup, _, _, classFile, _, _, _, _, _, role = GetRaidRosterInfo(i)
                if name == nil or subgroup == nil or classFile == nil or role == nil then
                    return false, -1
                end
            end
            local _, _, lastSubgroup = GetRaidRosterInfo(count)
            return true, lastSubgroup
            "#,
        )
        .unwrap();

    assert!(
        all_have_roster_data,
        "Edit Mode can force raid frame layout while the simulated group is still a party"
    );
    assert_eq!(
        last_subgroup, 1,
        "five visible group members should remain in the first subgroup"
    );
}

#[test]
fn test_get_num_party_members_matches_subgroup_count() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(3)
            return GetNumPartyMembers()
            "#,
        )
        .unwrap();
    assert_eq!(count, 3, "GetNumPartyMembers aliases GetNumSubgroupMembers");
}

#[test]
fn test_get_num_subgroup_members_matches_party_size() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(3)
            return GetNumSubgroupMembers()
            "#,
        )
        .unwrap();
    assert_eq!(count, 3, "GetNumSubgroupMembers() should match party size");
}

#[test]
fn test_group_queries_switch_between_solo_and_grouped_states() {
    let env = env();
    let (solo_in_group, solo_group_count, solo_subgroup_count, grouped_in_group, grouped_group_count, grouped_subgroup_count): (
        bool,
        i32,
        i32,
        bool,
        i32,
        i32,
    ) = env
        .eval(
            r#"
            A_Admin.SetPartySize(0)
            local solo_in_group = IsInGroup()
            local solo_group_count = GetNumGroupMembers()
            local solo_subgroup_count = GetNumSubgroupMembers()

            A_Admin.SetPartySize(4)
            return solo_in_group, solo_group_count, solo_subgroup_count, IsInGroup(LE_PARTY_CATEGORY_HOME), GetNumGroupMembers(LE_PARTY_CATEGORY_INSTANCE), GetNumSubgroupMembers()
            "#,
        )
        .unwrap();
    assert!(!solo_in_group, "solo state should report not in group");
    assert_eq!(
        solo_group_count, 0,
        "solo state should have zero group members"
    );
    assert_eq!(
        solo_subgroup_count, 0,
        "solo state should have zero subgroup members"
    );
    assert!(
        grouped_in_group,
        "grouped state should report in-group even when a party category is passed"
    );
    assert_eq!(
        grouped_group_count, 5,
        "group member count should include the player after grouping"
    );
    assert_eq!(
        grouped_subgroup_count, 4,
        "subgroup member count should match grouped party size"
    );
}

#[test]
fn set_party_size_defaults_to_player_leader() {
    let env = env();
    let (player_leads, party1_leads): (bool, bool) = env
        .eval(
            r#"
            A_Admin.SetPartySize(4)
            return UnitIsGroupLeader("player", LE_PARTY_CATEGORY_HOME),
                   UnitIsGroupLeader("party1", LE_PARTY_CATEGORY_HOME)
            "#,
        )
        .unwrap();
    assert!(
        player_leads,
        "party-size fixtures should let the local player queue as party leader"
    );
    assert!(
        !party1_leads,
        "party1 should only lead after explicit A_Admin.SetPartyLeader(1)"
    );
}

#[test]
fn test_set_party_leader_to_party_member_updates_group_leader_queries() {
    let env = env();
    let (player_leads, party1_leads, party2_leads): (bool, bool, bool) = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyLeader(1)
            return IsGroupLeader(), UnitIsGroupLeader("party1"), UnitIsGroupLeader("party2")
            "#,
        )
        .unwrap();
    assert!(
        !player_leads,
        "player should stop leading after SetPartyLeader(1)"
    );
    assert!(party1_leads, "party1 should lead after SetPartyLeader(1)");
    assert!(
        !party2_leads,
        "party2 should not lead after SetPartyLeader(1)"
    );
}

#[test]
fn test_set_party_size_defaults_leadership_to_player() {
    let env = env();
    let (player_leads, party1_leads): (bool, bool) = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            return IsGroupLeader(), UnitIsGroupLeader("party1")
            "#,
        )
        .unwrap();
    assert!(
        player_leads,
        "SetPartySize should default the local player as leader"
    );
    assert!(
        !party1_leads,
        "party1 should only lead after explicit A_Admin.SetPartyLeader(1)"
    );
}

#[test]
fn test_set_party_leader_zero_restores_player_leadership() {
    let env = env();
    let (player_leads, party1_leads): (bool, bool) = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyLeader(1)
            A_Admin.SetPartyLeader(0)
            return IsGroupLeader(), UnitIsGroupLeader("party1")
            "#,
        )
        .unwrap();
    assert!(player_leads, "player should lead after SetPartyLeader(0)");
    assert!(
        !party1_leads,
        "party1 should stop leading after SetPartyLeader(0)"
    );
}

// ============================================================================
// SetPartyMember
// ============================================================================

#[test]
fn test_set_party_member_name_readable_via_unit_name() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMember(1, "Tank", 1, 80)
            return UnitName("party1")
            "#,
        )
        .unwrap();
    assert_eq!(name, "Tank");
}

#[test]
fn test_set_party_member_class_readable_via_unit_class() {
    let env = env();
    // class_index 2 = Paladin
    let (class_name, class_file, class_id): (String, String, i32) = env
        .eval(
            r#"
            A_Admin.SetPartySize(1)
            A_Admin.SetPartyMember(1, "Holydin", 2, 80)
            return UnitClass("party1")
            "#,
        )
        .unwrap();
    assert_eq!(class_name, "Paladin");
    assert_eq!(class_file, "PALADIN");
    assert_eq!(class_id, 2);
}

#[test]
fn test_set_party_member_level_readable_via_unit_level() {
    let env = env();
    let level: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(1)
            A_Admin.SetPartyMember(1, "Tanker", 1, 70)
            return UnitLevel("party1")
            "#,
        )
        .unwrap();
    assert_eq!(level, 70);
}

#[test]
fn test_set_party_member_second_slot() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMember(1, "Alpha", 1, 80)
            A_Admin.SetPartyMember(2, "Beta", 5, 80)
            return UnitName("party2")
            "#,
        )
        .unwrap();
    assert_eq!(name, "Beta");
}

#[test]
fn test_set_party_member_does_not_affect_other_slots() {
    let env = env();
    let name1: String = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMember(1, "Alpha", 1, 80)
            A_Admin.SetPartyMember(2, "Beta", 5, 80)
            return UnitName("party1")
            "#,
        )
        .unwrap();
    assert_eq!(name1, "Alpha");
}

// ============================================================================
// SetPartyMemberHealth
// ============================================================================

#[test]
fn test_set_party_member_health_current() {
    let env = env();
    let hp: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMemberHealth(1, 5000, 10000)
            return UnitHealth("party1")
            "#,
        )
        .unwrap();
    assert_eq!(hp, 5000);
}

#[test]
fn test_set_party_member_health_max() {
    let env = env();
    let hp_max: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMemberHealth(1, 5000, 10000)
            return UnitHealthMax("party1")
            "#,
        )
        .unwrap();
    assert_eq!(hp_max, 10000);
}

#[test]
fn test_set_party_member_health_second_member() {
    let env = env();
    let (hp, hp_max): (i32, i32) = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMemberHealth(1, 1000, 2000)
            A_Admin.SetPartyMemberHealth(2, 3000, 6000)
            return UnitHealth("party2"), UnitHealthMax("party2")
            "#,
        )
        .unwrap();
    assert_eq!(hp, 3000);
    assert_eq!(hp_max, 6000);
}

#[test]
fn test_set_party_member_health_does_not_affect_other_member() {
    let env = env();
    // Set health for member 2, member 1 retains its value.
    let hp1: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMemberHealth(1, 9000, 10000)
            A_Admin.SetPartyMemberHealth(2, 3000, 6000)
            return UnitHealth("party1")
            "#,
        )
        .unwrap();
    assert_eq!(hp1, 9000);
}
