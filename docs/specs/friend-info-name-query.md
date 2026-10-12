# Friend info by name

Current cached Retail/PTR `FriendListDocumentation.lua` declares `C_FriendList.GetFriendInfo(name: cstring)` with `AllowedWhenUntainted`. Unchanged Mainline `GameDialogDefs.lua` uses it to prefill `SET_FRIENDNOTE` from `info.notes`. Existing `SocialFriend` records and name lookup are modeled under a differently named simulator member; this slice exposes the exact query and verifies that public consumer, not native friend-system parity.

## What it must do

- [ ] Publish the exact current-profile query and project supplied friend name/note from the owning environment; live note changes must be visible without leaking across environments.
- [ ] Return one info table for a matching supplied name and one nil for a miss. Exact string matching and miss arity preserve existing simulator lookup policy, not independently established native normalization or arity.
- [ ] Retain the existing strict string-input policy. Authenticate wrapped values before validation: untainted secret names work, tainted secret arguments fail without clearing caller taint, ordinary names remain usable by tainted callers.
- [ ] Load the whole unchanged current-profile popup/FriendsFrame dependency closure and exercise real `StaticPopup_Show("SET_FRIENDNOTE", name)`: the actual Mainline handler must prefill the real edit box and reflect a changed supplied note on reopening, without recorded Lua errors.
- [ ] Preserve existing indexed/earlier-provider behavior while replacing the current non-native alternate-name provider; do not add an alternate-name fallback.

## How it works

- [Lua API architecture](../lua-api.md)
- [Historical/current API investigation](../wiki/investigations/patch-8-1-0-api-audit.md)

## Implementation inventory

- `src/lua_api/state_types/social.rs`: existing supplied `SocialFriend` name/note and other represented fields.
- `src/lua_api/globals/missing_surface/friend_list.rs`: existing lookup/DTO and registration; exact current query absent at test/spec stage.
- `tests/friend_info_exact.rs`: focused current-profile behavioral and real public-popup fixtures.

## Tests asserting this spec

- `tests/friend_info_exact.rs`, registered by `tests/integration.rs`: exact query, live environment-owned state, miss/type/security controls and actual popup prefill/reopen.
- `tests/friend_list_who.rs`: existing indexed/alias-backed state baseline; not exact-name acceptance yet.

## Known gaps (current cycle)

- [x] Initial pre-producer RED executed and authenticated: four cases0PASS/4FAIL; three stop at absent exact query, popup stops at an overbroad diagnostic-readiness assertion. Downstream assertions remain unreached.
- [ ] Establish real popup closure readiness and actual failure boundary. Initial `C_Reveal` observation is a guarded optional debug probe, not an actual load error; retain observations without requiring every nil probe to exist. Keep real load/error/provenance/prefill assertions, then re-execute before consumer credit.
- [ ] Implement the exact C API over existing state after RED, then independently verify affected behavior, loaded startup, formatting/default check and readability; retain scoped proof.

## Out of scope

This bounded caller fix does not establish native catalogs, name normalization/coercion, complete native `FriendInfo` fields, note mutation APIs, network friend lifecycle or new earlier-profile contracts. Those remain separate parent-handoff gaps; no new normal-session seeding follows from fixtures. Existing synthetic records are simulator inputs, not native facts.
