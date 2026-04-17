//! Tests for `C_Social` probes backed by `SimState.social_friends`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::SocialFriend;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_friend_info_returns_seeded_count_via_get_friends() {
    let env = env();
    let count: i32 = env
        .eval("return #C_Social.GetFriends()")
        .unwrap();
    assert_eq!(count, 3, "three seeded friends");
}

#[test]
fn get_friend_info_returns_table_for_valid_index() {
    let env = env();
    let (name, level, area, class, connected): (String, i32, String, String, bool) = env
        .eval(
            r#"
            local info = C_Social.GetFriendInfo(1)
            return info.name, info.level, info.area, info.className, info.connected
            "#,
        )
        .unwrap();
    assert_eq!(name, "Arthax");
    assert_eq!(level, 70);
    assert_eq!(area, "Stormwind City");
    assert_eq!(class, "Paladin");
    assert!(connected);
}

#[test]
fn get_friend_info_offline_friend() {
    let env = env();
    let (name, connected, note): (String, bool, String) = env
        .eval(
            r#"
            local info = C_Social.GetFriendInfo(2)
            return info.name, info.connected, info.notes
            "#,
        )
        .unwrap();
    assert_eq!(name, "Durotan");
    assert!(!connected);
    assert_eq!(note, "old guildie");
}

#[test]
fn get_friend_info_returns_nil_for_index_zero() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Social.GetFriendInfo(0) == nil")
        .unwrap();
    assert!(is_nil, "index 0 is out of range (1-based)");
}

#[test]
fn get_friend_info_returns_nil_for_out_of_range_index() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Social.GetFriendInfo(999) == nil")
        .unwrap();
    assert!(is_nil, "index beyond list length returns nil");
}

#[test]
fn get_friends_returns_array_of_tables() {
    let env = env();
    let (count, first_name, third_class): (i32, String, String) = env
        .eval(
            r#"
            local friends = C_Social.GetFriends()
            return #friends, friends[1].name, friends[3].className
            "#,
        )
        .unwrap();
    assert_eq!(count, 3);
    assert_eq!(first_name, "Arthax");
    assert_eq!(third_class, "Mage");
}

#[test]
fn get_friends_entries_have_guid_field() {
    let env = env();
    let guid: String = env
        .eval(
            r#"
            local friends = C_Social.GetFriends()
            return friends[1].guid
            "#,
        )
        .unwrap();
    assert_eq!(guid, "Player-1-0000A001");
}

#[test]
fn social_friends_reflect_sim_state_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.social_friends.clear();
        state.social_friends.push(SocialFriend {
            name: "TestFriend".into(),
            level: 80,
            area: "Dalaran".into(),
            class_name: "Priest".into(),
            note: "test".into(),
            is_online: true,
            guid: "Player-1-TESTGUID".into(),
        });
    }
    let (count, name, level): (i32, String, i32) = env
        .eval(
            r#"
            local friends = C_Social.GetFriends()
            local info = C_Social.GetFriendInfo(1)
            return #friends, info.name, info.level
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(name, "TestFriend");
    assert_eq!(level, 80);
}
