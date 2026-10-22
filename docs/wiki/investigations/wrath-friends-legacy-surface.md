# Wrath friends consumer needs legacy global adapters

F02 retains original Wrath friends-panel coverage rather than excluding the entire mixed modern-UI/seeded-data test. The native consumer exists; missing modeled legacy entry points are not evidence that the feature is absent.

## Observed boundary

The unchanged `FrameXML/FriendsFrame.lua` uses `ToggleFriendsFrame`, `FriendsList_Update`, `ShowFriends` and `GetFriendInfo`. A source-bound Wrath CLI diagnostic reaches `FriendsFrame_Update` at line 258, where calling `ShowFriends` fails.

Raw global `ShowFriends` and `GetFriendInfo` entries are absent. Calling their looked-up values fails, while raw `C_FriendList.ShowFriends` is registered and callable without error. Native `GetNumFriends` returns the supplied counts `3, 2`. Callability synthesized by global lookup therefore does not establish a working legacy adapter or native UI compatibility.

The fixture loads the shared native FrameXML helper before addons and retains the authored online/offline identities through original free functions. Its source-bound Rust case executes and fails at the original consumer's missing-producer boundary; it is not gated. Build12340 findings additionally require the offline connected value to remain nil, not modern false. That final expectation correction is test-only and awaits the joint gate with the separate producer lane; it was not rebuilt in isolation. No namespace shim, mock consumer or producer correction is added here. Producer repair belongs to the separate Wrath friends legacy lane.

## Sources

- [Native fixture](../../../tests/friends_panel.rs) and [original-consumer assertions](../../../tests/friends_panel/wrath.lua).
- Clean-room `friends-legacy-12340.md` findings (2026-10-20): static reconstruction, not native runtime execution; connected is numeric1 or nil.
- [Wrath manifest](../../../data/blizzard-ui-files/wrath.txt), including original `FrameXML/FriendsFrame.lua`.
- [Legacy count adapter](../../../src/lua_api/globals/real/wrath_startup_queries.rs).
- [Backing friend-list registration and model reads](../../../src/lua_api/globals/missing_surface/friend_list.rs).

## See Also

- [Lua API system](../systems/lua-api.md).
- [Native chat preparation boundary](wrath-chat-layout.md).
