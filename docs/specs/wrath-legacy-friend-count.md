# Wrath legacy friend count tuple

Wrath simulator projection of existing friend-list state, not native-client evidence.

## What it must do

- [x] Global GetNumFriends returns exactly total, online from existing C_FriendList social state, Wrath only.
- [x] Original FriendsMicroButton subtree displays 2, 1 and 0 for supplied three/two-online, three/one-online and empty records respectively.
- [x] Namespace and legacy counts agree; no independent friend list and no replacement count getters in tests.

## How it works

- [Lua API](../lua-api.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

`src/lua_api/globals/real/wrath_startup_queries.rs`: real non-C legacy projection; C_FriendList retains ownership.

## Tests asserting this spec

`tests/wrath_unmasked_friends.rs`; shared pinned originals in `tests/wrath_unmasked_support/`.

## Known gaps (current cycle)

- [x] Original FriendsMicroButton RED reaches generated bcdd9c3815ef6147:8 (FloatingChatFrame.xml:947). All supplied/empty count fixtures pass in the d11a25154 sealed Wrath43 gate; that startup record disappears. This is not complete FloatingChatFrame/profile acceptance.
- [ ] Existing BNGetNumFriends temporary zero policy is not a modeled Battle.net count. Combined two-domain populated behavior remains unsupported.

## Out of scope

Blizzard/vendor edits, native parity and full-profile acceptance.
