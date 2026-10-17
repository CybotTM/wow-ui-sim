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
Wrath `GetPartyMember(index)` reads the existing active-group flag and supplied
party-member list. Only occupied indices 1–4 in an active group return true;
other indices/inactive groups return false. No new party state or default is added.
`GetPartyLeaderIndex()` converts existing zero-based member-leader state to party
indices 1–4; player leadership (`None`), inactive groups, and unsupplied/invalid
member indices return 0. It agrees with existing leader queries for supplied members.
Wrath `GetVoiceStatus(unit, mode)` / `GetMuteStatus(unit, mode)` read explicit
per-unit/channel-mode presentation entries beside `VoiceChatState`; absent entries
return false. Modes remain distinct (`party`, `raid`, `Battleground`), as selected
by the original party-frame consumer. Both values are booleans, not numeric flags.
Existing channel snapshots have no unit-to-member mapping carrying both flags;
this small legacy input does not implement voice transport or native parity.
`UnitIsTalking(name)` reads existing channel member `is_active_speaker` entries
by exact character-name bytes; no match returns false. Non-UTF-8 names remain
valid string inputs and cannot match the modeled UTF-8 names.
No solo-time override, setter change, or group-leave behavior is added.
This fixture is not a native initial-state or tuple-parity claim.

Evidence: research-loot-sound/report.md in the private staging handoff identifies
wowless Wrath Classic 3.4.2 commit 6698b2a98835f8e65e2e347cc19f943cb4313dd8,
data/products/wow_classic/apis.yaml, GetLootMethod `stub: freeforall`, alongside
original Wrath FrameXML/UnitPopup.lua's accepted tokens. No native capture.

## What it must do

- [x] Fresh state reaches original UIDropDownMenu/UnitPopup through complete
  PartyFrame.xml sequential loading; four real party frames/dropdowns and labels.
- [x] Empty RAF links and explicitly supplied links are separately exercised.
- [x] Explicit master state preserves party index 1 and raid index 3.
- [x] Original RaidOptionsFrame_UpdatePartyFrames hides unused real siblings
  and shows the represented party member without captured Lua errors.
- [x] All four dropdowns and the original loot-threshold menu resolve to
  `ITEM_QUALITY2_DESC`; an explicit threshold 4 update resolves to `ITEM_QUALITY4_DESC`.
- [x] Original opt-out menu text reflects unsupplied false and explicit true state.
- [x] A second supplied member reaches the original shown-frame path; disabling
  the group makes the same frame hide.
- [x] Supplying party2 as leader returns index 2 and shows only that member's
  original leader icon; player leadership returns 0 and hides member leader icons.
- [x] Supplied voice-enabled/muted flags reach the original speaker/mute icons;
  absent flags hide them, and another mode does not inherit the party selection.
- [x] An explicitly active speaker named like the supplied party member reaches
  the original talking-frame path; changing it inactive hides that frame.
- [x] Original RAF summon text has no cooldown suffix for unsupplied `(0, 0)`;
  supplied start/duration reaches its formatted remaining-time label.
- [x] Retail C API returns documented start/duration/enabled fields; original
  FriendsListTemplates RefreshCooldown clears for unsupplied/disabled state and
  applies the supplied cooldown when enabled.

Historical RED reached PartyMemberFrame1 creation at original UnitPopup.lua:223
before producer `6dab184f6`, then :226 on absent GetLootThreshold. Subsequent REDs
exposed opt-out, RAF cooldown, membership, voice, and leader readers. Original
consumers and explicit master/RAF fixtures were preserved.

## Combined proof — 2026-10-17

Sealed source `a83be2c64`, rebased onto `241ce290f`, uses rilua `b2629b6`.
All 58 distinct Wrath names PASS: our 46 plus u-resid's 48 share 36 controls,
so no duplicate executions are credited. This includes the unchanged original
unsupplied/binary-name voice consumer and the supplied speaking/party2-leader paths.
All eight Retail controls PASS: cooldown three, existing combo/speed four, and
an unchanged 535-byte admin/query trace. Retail startup is `[]`; default/Wrath
library and full root-bin checks plus formatting PASS. These are bounded proofs,
not full-suite, native-parity or all-profile acceptance.

The [current failing startup inventory](../baselines/README.md#wrath-lua-errors)
owns exact deltas and the newly exposed error-display cursor record. Its
[original-source investigation](../wiki/investigations/wrath-startup-u-residuals.md#error-display-after-lootsound-repair)
separates exposure from a cursor fix; no cursor/vendor behavior is patched.
Exact commands, names, seals, CVar guards and raw attribution are retained under
`~/.local/state/wow-ui-sim/handoff/wrath-sound-loot/` in `proof-ledger.json`,
`wrath-combined-startup-vs-*.json` and `wrath-combined-classification.json`.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/sim_substates/mod.rs`: approved initial token and threshold fixture.
- `src/lua_api/globals/real/loot_method.rs`: Wrath-only legacy threshold and opt-out readers.
- `src/c_api/c_recruit_a_friend.rs`: shared explicit cooldown snapshot and reader.
- `src/lua_api/globals/real/wrath_startup_queries.rs`: existing-state party membership reader.
- `src/lua_api/globals/real/wrath_voice.rs`: explicit legacy voice presentation input/readers.
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
