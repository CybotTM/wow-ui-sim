# Wrath UnitHasRelicSlot

Wrath exposes the modeled legacy non-C_* global in
`src/lua_api/globals/real/wrath_relic_slot.rs`. Native authority: accepted laptop
answer Q1 in `laptop-answers-req-11.md` (private artifact location in lane handoff).
See [Lua API implementation notes](../wiki/systems/lua-api.md#wrath-relic-class-capability).
Status: staged, edit-only; no compilation or runtime acceptance claimed.

## What it must do

- [ ] Register only when runtime `client_profile::ACTIVE == ClientProfile::Wrath`;
  leave Forever and all other profiles unchanged.
- [ ] Accept strings and Lua 5.1 numbers; missing/nil, boolean, table and function
  arguments raise `Usage: UnitHasRelicSlot("unit")`. Ignore extra arguments.
- [ ] Return exactly one value: numeric `1` for a resolved player with supplied
  class record flags containing bit `8`; otherwise nil, including nonplayers,
  unknown units and missing class records. Never inspect equipped gear.
- [ ] Recognize case-insensitive `player`, `vehicle`, `pet`, `target`, indexed
  `partypetN`, `partyN`, `raidpetN`, `raidN`, `bossN`, `arenapetN`, `arenaN`,
  `commentatorN`, plus `mouseover`, `focus`, `npc`, `questnpc`, `none`, and
  trailing `-target`/`-pet` chains. Resolve supplied known GUID strings too.
  Recognition alone does not establish a unit exists. `none` resolves to nil.
  Do not import simulator-only `self`, `enemyN` or `softinteract` aliases.
- [ ] Original ToggleCharacter/PaperDollFrame consumer hides ammo for a capable
  fixture, shows it for an incapable fixture, and keeps ranged visible; reopening
  follows current class input with an empty recorded-error list.

## How it works

- [Lua API wiki](../wiki/systems/lua-api.md#wrath-relic-class-capability) — backing
  state, resolution and explicit input policy.

## Implementation inventory

- `src/lua_api/globals/real/wrath_relic_slot.rs` — class flags, unit inputs and global.
- `src/lua_api/globals/real/mod.rs`, `src/lua_api/globals/register.rs` — shared module/registration.
- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs` — environment-local state and empty initialization.
- `tests/wrath_relic_slot.rs` — original consumer and native contract specimens.

## Tests asserting this spec

`tests/wrath_relic_slot.rs`: original selector at RED `62b1b1f01` plus three
producer contract tests. All unexecuted in this edit-only stage. GREEN adds
explicit fixture class flags `(2, 8)` and `(1, 0)`; these are authored inputs,
not a discovered native class-name mapping. Original assertions remain intact.
The original RED revision cannot contain those new backing fields.

## Known gaps (current cycle)

- [ ] Parent executes source-bound RED/GREEN, Wrath check/formatter, stock startup
  against the tracked baseline, and exact Retail stock preservation (36 messages,
  42 occurrences) following private `wrath-relic-slot/SLOT-RECIPE.md`.
- [ ] Native class-name-to-flag mapping remains UNKNOWN in Q1. Default class table
  is empty; stock player capability is not asserted. No inferred class mask.
- [ ] Unit chains and tokens without shared backing require explicit resolved
  snapshots. No automatic relationship graph or complete native unit roster.

## Out of scope

Gear-derived capability, inferred class defaults, other-profile semantic changes,
Inspect/icon branches, S49 fixture redesign, builds/tests/slots during staging,
and decompiled-source access. S49's original consumer was also blocked by this
API at `PaperDollFrame.lua:1094`; unblocking is not yet runtime-proven.
