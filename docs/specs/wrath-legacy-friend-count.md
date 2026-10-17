# Wrath legacy friend count tuple

Source-only proposed Wrath simulator contract (2026-10-16), not native-client evidence.

## What it must do

- [ ] Global GetNumFriends returns exactly total, online from existing C_FriendList social state, Wrath only.
- [ ] Original FriendsMicroButton subtree displays 2, 1 and 0 for supplied three/two-online, three/one-online and empty records respectively.
- [ ] Namespace and legacy counts agree; no independent friend list and no replacement count getters in tests.

## How it works

- [Lua API](../lua-api.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

`src/lua_api/globals/real/wrath_startup_queries.rs`: real non-C legacy projection; C_FriendList retains ownership.

## Tests asserting this spec

`tests/wrath_unmasked_friends.rs`; shared pinned originals in `tests/wrath_unmasked_support/`.

## Known gaps (current cycle)

- [ ] No executed RED/GREEN or whole FloatingChatFrame startup acceptance.
- [ ] Existing BNGetNumFriends temporary zero policy is not a modeled Battle.net count. Combined two-domain populated behavior remains unsupported.

## Out of scope

Blizzard/vendor edits, native parity, full profile acceptance, builds and runtime execution during staging.
