# Wrath initial loot method

Approved 2026-10-16: `freeforall` is the documented simulator default, backed by
wowless Wrath Classic data, not a native observation. Decision receipt:
`~/.local/state/wow-ui-sim/handoff/user-decisions-wrath-loot-20261016.json`.

Documented simulator fixture: a fresh client-wrath LootMethodState starts with
method `freeforall`. Other profiles retain their existing initial token.
Existing master indices, state-backed readers and mutations remain unchanged.
Wrath `GetLootThreshold()` reads the existing `LootMethodState.threshold`, settable
through the existing Rust admin/test state path. Its initial 2 (Uncommon) is an
approved temporary simulator default, not native parity (2026-10-17 receipt:
`~/.local/state/wow-ui-sim/handoff/user-decisions-wrath-loot-threshold-20261017.json`).
Retire this fixture when native group loot threshold state is modeled.
Wrath `GetOptOutOfLoot()` reads `LootMethodState.opt_out`, settable through the
same Rust test-state path. Unsupplied state is false (not opted out), per the
standing approved Wrath empty/zero/no-selection policy applied by the parent on
2026-10-17. This is a simulator default, not a native observation.
Wrath's legacy `GetSummonFriendCooldown()` and the existing
`C_RecruitAFriend.GetSummonFriendCooldown` read one explicit `SimState` snapshot:
start time in seconds, duration in seconds, and an enabled-timer bool. Unsupplied
state is `(0, 0, false)`, per the approved zero/no-selection policy (parent
correction 2026-10-17); supplied values must be preserved by both readers.
This corrects the C API's previously missing third result to match pinned generated
documentation; it does not claim native cooldown lifecycle parity.
No solo-time override, setter change, or group-leave behavior is added.
This fixture is not a native initial-state or tuple-parity claim.

Evidence: research-loot-sound/report.md in the private staging handoff identifies
wowless Wrath Classic 3.4.2 commit 6698b2a98835f8e65e2e347cc19f943cb4313dd8,
data/products/wow_classic/apis.yaml, GetLootMethod `stub: freeforall`, alongside
original Wrath FrameXML/UnitPopup.lua's accepted tokens. No native capture.

## What it must do

- [ ] Fresh state reaches original UIDropDownMenu/UnitPopup through complete
  PartyFrame.xml sequential loading; four real party frames/dropdowns and labels.
- [ ] Empty RAF links and explicitly supplied links are separately exercised.
- [ ] Explicit master state preserves party index 1 and raid index 3.
- [ ] Original RaidOptionsFrame_UpdatePartyFrames hides unused real siblings
  and shows the represented party member without captured Lua errors.
- [ ] All four dropdowns and the original loot-threshold menu resolve to
  `ITEM_QUALITY2_DESC`; an explicit threshold 4 update resolves to `ITEM_QUALITY4_DESC`.
- [ ] Original opt-out menu text reflects unsupplied false and explicit true state.
- [ ] Original RAF summon text has no cooldown suffix for unsupplied `(0, 0)`;
  supplied start/duration reaches its formatted remaining-time label.
- [ ] Retail C API returns documented start/duration/enabled fields; original
  FriendsListTemplates RefreshCooldown clears for unsupplied/disabled state and
  applies the supplied cooldown when enabled.

Behavioral RED reached PartyMemberFrame1 creation at original UnitPopup.lua:223
before producer `6dab184f6`. After that approved producer, both original party
fixtures fail at UnitPopup.lua:226 on the absent GetLootThreshold call. Explicit
master/RAF inputs and original consumers remain unchanged; no green party-flow
claim. Combined sound/loot startup falls from 22 records/30 occurrences to 19/22,
including nine newly unmasked :226 records (2026-10-17).

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/sim_substates/mod.rs`: approved initial token and threshold fixture.
- `src/lua_api/globals/real/loot_method.rs`: Wrath-only legacy threshold and opt-out readers.
- `src/c_api/c_recruit_a_friend.rs`: shared explicit cooldown snapshot.
- `src/lua_api/globals/real/recruit_a_friend.rs`: Wrath legacy cooldown registration.

## Tests asserting this spec

- `tests/wrath_loot_initial.rs`
- `tests/recruit_a_friend_cooldown.rs`

## Known gaps (current cycle)

Native initial threshold and group-transition parity remain unverified. The
2026-10-17 decision authorizes the state-backed reader with temporary default 2,
not a new group-loot lifecycle model.

## Out of scope

Native parity, complete startup recovery, broader aliases and group-transition semantics.
