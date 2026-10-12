# Friend note mutation

Retail/PTR `C_FriendList.SetFriendNotes` contract over existing environment-owned friends. Cached `Blizzard_APIDocumentationGenerated/FriendListDocumentation.lua` declares two non-nil `cstring` arguments, one non-nil `found: bool`, `RequiresFriendList`, and `AllowedWhenUntainted`. Local exact matching, synchronous note mutation, and strict UTF-8 validation below are bounded model policy, not independently established native behavior. Ordinary unchanged `SET_FRIENDNOTE` Accept/Enter callers motivate the seam. Setter implementation uses the existing owned records; focused GREEN and consumer/security proof remain pending.

## What it must do

- [ ] Publish exact `C_FriendList.SetFriendNotes(name, notes)` on Retail/PTR; return exactly one boolean. Find by the same exact supplied name as the getter; return true even when the matching note is unchanged, update its existing note, and clear on empty string. Miss returns false without creating/reordering/mutating records.
- [ ] Expose updated `notes` through existing `C_FriendList.GetFriendInfo`, `GetFriendInfoByIndex`, `C_Social.GetFriendInfo`, and `C_Social.GetFriends`; preserve unrelated represented friend fields and independent environments. Respect current indexed display sorting, not vector-index assumptions.
- [ ] Require two actual UTF-8 strings without numeric/other coercion. Authenticate both original arguments before any type validation, lookup, or mutation. Genuine untainted secret strings work in either/both positions; wrapped nonstrings/invalid UTF-8 produce positional string errors without destroying wrappers.
- [ ] Deny any tainted secret argument on hit, miss, or invalid payload, retaining wrappers/caller taint and friend state. In particular, tainted invalid plain arg1 plus secret arg2 must fail for secret access before arg1 type. Plain strings remain usable under taint.
- [ ] Load the whole unchanged StaticPopup_Game/FriendsFrame closure. Use real edit-box SetText and actual Accept button Click/original whole Enter handler, preserving vendor handler identity and void return/root-hide behavior; reopen and read updated/cleared notes through the source getter.

## How it works

- [Lua API architecture](../lua-api.md); existing shared model boundary.
- [Existing query contract](friend-info-name-query.md).

## Implementation inventory

- `src/lua_api/state_types/social.rs`: existing `SocialFriend.name`/`note` backing; no new state proposed.
- `src/c_api/c_friend_list.rs`: existing exact getter/shared simulator projection; exact getter and setter boundary.
- `src/lua_api/globals/missing_surface/friend_list.rs`: existing indexed getter and online-first display order.
- `src/c_api/c_social.rs`: existing `notes` projections over the same vector.
- Cached `Blizzard_StaticPopup_Game/{GameDialogDefs.lua,Mainline/GameDialogDefs.lua,GameDialog.lua}` and `Blizzard_StaticPopup/StaticPopup.lua`: unchanged real popup handlers and Accept routing.

## Tests asserting this spec

- [friend_note_mutation.rs](../../tests/friend_note_mutation.rs): eight integrated Rust cases; genuine Retail RED failed8/8 at missing raw setter on `d61c78a2b`. Downstream assertions were unreached; GREEN remains pending.

## Known gaps (current cycle)

- [ ] Complete focused Retail/PTR GREEN after authenticated Retail RED at the absent raw registration boundary; downstream mutation/security/popup assertions have no proof until executed.
- [ ] Whole-closure readiness, actual Click dispatch, genuine-secret/type-error assertions, and successful reopen remain runtime-unproved; report dependency failures separately, never fabricate dependencies or patch vendor handlers.

## Out of scope

`SetFriendNotesByIndex`, friend catalogs/seeding, network/persistence/native normalization/length policy, asynchronous refresh, event behavior (including absence assertions), earlier profiles. Native service semantics require separate evidence.
