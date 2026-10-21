# Wrath legacy friends globals

Wrath's original FriendsFrame consumes legacy tuples and GUID-based selection over the existing social friend model. The modeled globals live in `src/lua_api/globals/real/wrath_friends.rs`; [projection notes](../wiki/systems/lua-api.md#wrath-legacy-friends) explain shared ownership.

## What it must do

- [ ] Publish `ShowFriends`, `GetFriendInfo`, `GetSelectedFriend` and `SetSelectedFriend` on Wrath only; preserve Retail friends behavior.
- [ ] Original `ToggleFriendsFrame(1)` / `FriendsList_Update()` displays populated online/offline rows, the offline header, and selection-dependent tell-button state without replacing the original consumers.
- [ ] `GetFriendInfo(index or name)` returns exactly name, level, localized class, area, numeric 1-or-nil connected, status, nonempty notes-or-nil, numeric 1-or-nil recruit-a-friend flag. Numeric strings use the index path; nonnumeric names match case-insensitively.
- [ ] Explicit null-info selectors (0, 102, unknown name) return exactly nil, 0, UNKNOWN, UNKNOWN, nil, empty string, nil, nil. Reject invalid types with `Usage: GetFriendInfo(index or name)`.
- [ ] Selection starts at 0, retains GUID identity across display-order changes, and resolves the current ordinal or 0 if absent. Numeric strings and representable positive fractional indexes convert before subtraction. Invalid types error without changing selection; out-of-range numeric selectors clear it. Setter returns zero values and emits no event.
- [ ] `ShowFriends` returns zero values and does not change supplied rows or selected GUID. Reuse the existing namespace refresh handler: queued `FRIENDLIST_UPDATE` is **INFERRED simulator response policy**, not synchronous native wrapper behavior.

### Evidence and limits

Laptop answer `friends-legacy-12340.md`, host `aso`, session `01a140f8-bf71-7014-8fdc-5eb7b11d0e7f`, received 2026-10-20: static build-12340 inspection, not runtime execution. Source citations relative to `WoW/Source/WowSvcs/WowSvcsClient/`: `FriendList_Scripts.cpp:166–278` (getter/selection), `:38–45` (ShowFriends request), `FriendList.cpp:160–294,1197–1207` (response/name completion), `:1999–2027` (name/selection lookup), `:353–386` (native sorting).

Native ShowFriends sends `CMSG_CONTACT_LIST` mask 1; the separate response path sorts and signals `FRIENDLIST_UPDATE`. Existing simulator projection preserves online-first stable input order, not native case-insensitive name sorting. Status is empty because the supplied model has no AFK/DND flags; this is **INFERRED model policy**, not complete native status support. The eighth return projects the existing linked-GUID input. ASCII case-insensitive lookup is covered; broader Unicode matching is unverified. Empty simulator slots use null-info output; real unused/stale native slots can retain different fields. Native index 101 crosses a reconstructed storage boundary and is deliberately not a parity contract. Exotic numeric conversions and native localized class/area DB resolution remain unverified.

## How it works

- [Lua API ownership and projection](../wiki/systems/lua-api.md#wrath-legacy-friends)
- [Existing count tuple](wrath-legacy-friend-count.md)

## Implementation inventory

- `src/lua_api/globals/real/wrath_friends.rs`: Wrath legacy tuple/selection adapters.
- `src/c_api/c_friend_list.rs`: shared online-first projection and refresh response handler.
- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs`: Wrath-only selected GUID initialization/storage.
- `src/lua_api/globals/real/{mod,wrath_startup_queries}.rs`: profile-scoped registration.
- `src/lua_api/globals/missing_surface/friend_list.rs`: existing namespace uses the same shared projection/refresh handler.

## Tests asserting this spec

- `tests/wrath_friends_legacy.rs`: original FriendsFrame rows/header/tell selection.
- `tests/wrath_friends_legacy_contract.rs`: tuple, coercion, selection identity and refresh preservation.
- `tests/friends_panel.rs`, `tests/system_api_seeded.rs`: Retail preservation and shared friend reads.

## Known gaps (current cycle)

- [ ] GREEN and profile-preservation proof pending; four-test RED reaches missing ShowFriends at original FriendsFrame.lua:258 plus missing API globals.

## Out of scope

Ignore APIs/state, AddFriend/RemoveFriend mutations (not called by default list path), vendor/rilua changes, native network transport, native stale-slot behavior, status-flag modeling, full-profile acceptance and other-profile publication.
