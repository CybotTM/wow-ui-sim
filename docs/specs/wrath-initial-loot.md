# Wrath initial loot method

Approved 2026-10-16: `freeforall` is the documented simulator default, backed by
wowless Wrath Classic data, not a native observation. Decision receipt:
`~/.local/state/wow-ui-sim/handoff/user-decisions-wrath-loot-20261016.json`.

Documented simulator fixture: a fresh client-wrath LootMethodState starts with
method `freeforall`. Other profiles retain their existing initial token.
Existing master indices, thresholds, state-backed readers and mutations remain
unchanged. No solo-time override, setter change, or group-leave behavior is added.
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

Behavioral RED reached PartyMemberFrame1 creation at original UnitPopup.lua:223
before producer `6dab184f6`. After that approved producer, both original party
fixtures fail at UnitPopup.lua:226 on the absent GetLootThreshold call. Explicit
master/RAF inputs and original consumers remain unchanged; no green party-flow
claim. Combined sound/loot startup falls from 22 records/30 occurrences to 19/22,
including nine newly unmasked :226 records (2026-10-17).

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/sim_substates/mod.rs`: approved initial token selected by active profile.

## Tests asserting this spec

- `tests/wrath_loot_initial.rs`

## Known gaps (current cycle)

- [ ] Resolve the missing GetLootThreshold surface before completing either
  party-flow test. Adding an adapter or temporary producer is not covered by
  the approved initial-token patch; no additional implementation was inferred.

## Out of scope

Native parity, complete startup recovery, broader aliases and group-transition semantics.
