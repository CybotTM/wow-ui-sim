#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn friends_frame_onload_reproduces_missing_guild_roster_selection() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let (ok, err): (bool, String) = env
        .eval(
            r#"
            rawset(_G, "SetGuildRosterSelection", nil)

            GuildFrame = {}

            local function friendsFrameOnLoadGuildSelection()
                GuildFrame.selectedGuildMember = 0
                SetGuildRosterSelection(0)
            end

            local ok, err = pcall(friendsFrameOnLoadGuildSelection)
            return ok, tostring(err)
            "#,
        )
        .expect("FriendsFrame guild selection reproduction should return a pcall status");

    assert!(!ok, "missing SetGuildRosterSelection should fail");
    assert!(
        err.contains("SetGuildRosterSelection"),
        "expected SetGuildRosterSelection nil failure, got: {err}"
    );
}

#[test]
fn guild_roster_selection_setter_is_not_a_noop() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let selected_indices: (i32, i32, i32) = env
        .eval(
            r#"
            local initialIndex = GetGuildRosterSelection()
            SetGuildRosterSelection(7)
            local selectedIndex = GetGuildRosterSelection()
            SetGuildRosterSelection(0)
            local clearedIndex = GetGuildRosterSelection()
            return initialIndex, selectedIndex, clearedIndex
            "#,
        )
        .expect("Guild roster selection should round-trip through setter and getter");

    assert_eq!(
        selected_indices,
        (0, 7, 0),
        "SetGuildRosterSelection must update state that GetGuildRosterSelection observes"
    );
}
