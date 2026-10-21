//! Static 12340 tuple/selection contracts over supplied simulator friends.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn wrath_friends_legacy_tuple_and_name_lookup() {
    let env = WowLuaEnv::new().unwrap();
    env.state().borrow_mut().social_friends[0].note = "Tank friend".into();
    env.exec(
        r#"
        local function check(selector)
            assert(select('#', GetFriendInfo(selector)) == 8)
            local name, level, class, area, connected, status, note, raf = GetFriendInfo(selector)
            assert(name == 'Arthax' and level == 70 and class == 'Paladin')
            assert(area == 'Stormwind City' and connected == 1 and status == '')
            assert(note == 'Tank friend' and raf == nil)
        end
        check(1)
        check('1')
        check('aRtHaX')
        local name, _, _, _, connected, _, note = GetFriendInfo('Durotan')
        assert(name == 'Durotan' and connected == nil and note == nil)
        for _, selector in ipairs({0, 102, 'not-a-friend'}) do
            local name, level, class, area, connected, status, note, raf = GetFriendInfo(selector)
            assert(select('#', GetFriendInfo(selector)) == 8)
            assert(name == nil and level == 0 and class == UNKNOWN and area == UNKNOWN)
            assert(connected == nil and status == '' and note == nil and raf == nil)
        end
        for _, invalid in ipairs({false, {}, function() end}) do
            local ok, message = pcall(GetFriendInfo, invalid)
            assert(not ok and tostring(message):find('Usage: GetFriendInfo(index or name)', 1, true))
        end
        assert(not pcall(GetFriendInfo))
        "#,
    )
    .unwrap();
}

#[test]
fn wrath_friends_legacy_selection_tracks_guid_across_projection_changes() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(GetSelectedFriend() == 0)
        assert(select('#', SetSelectedFriend('1')) == 0)
        assert(GetSelectedFriend() == 1)
        assert(GetFriendInfo(GetSelectedFriend()) == 'Arthax')
        for _, invalid in ipairs({false, 'not-an-index', {}}) do
            local ok, message = pcall(SetSelectedFriend, invalid)
            assert(not ok and tostring(message):find('Usage: SetSelectedFriend(index)', 1, true))
            assert(GetSelectedFriend() == 1, 'invalid type preserves selected GUID')
        end
        assert(not pcall(SetSelectedFriend))
        assert(GetSelectedFriend() == 1)
        for _, index in ipairs({0, 102, 100}) do
            SetSelectedFriend(index)
            assert(GetSelectedFriend() == 0, 'out-of-range or empty slot clears selection')
        end
        SetSelectedFriend(1.9)
        assert(GetSelectedFriend() == 1)
        "#,
    )
    .unwrap();
    env.state().borrow_mut().social_friends[0].is_online = false;
    env.exec(
        r#"
        assert(GetSelectedFriend() == 2, 'online-first projection retains selected GUID')
        assert(GetFriendInfo(GetSelectedFriend()) == 'Arthax')
        "#,
    )
    .unwrap();
    env.state().borrow_mut().social_friends.remove(0);
    assert_eq!(env.eval::<i32>("return GetSelectedFriend()").unwrap(), 0);
}

#[test]
fn wrath_friends_legacy_show_requests_refresh_without_changing_selection_or_rows() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        SetSelectedFriend(2)
        assert(select('#', ShowFriends('ignored')) == 0)
        assert(GetSelectedFriend() == 2)
        assert(GetNumFriends() == 3)
        assert(GetFriendInfo(2) == 'Sylvara')
        "#,
    )
    .unwrap();
}
