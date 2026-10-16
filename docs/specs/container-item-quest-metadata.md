# Container item quest metadata

Status: corrected tests/passive scaffold staged; RED/GREEN unexecuted. Wrath legacy calls assert arity and values directly; positive cases predict value RED, negative legacy cases are preservation PASS.

Simulator-authored supplied-state policy for `C_Container.GetContainerItemQuestInfo`
and Wrath's legacy `GetContainerItemQuestInfo`. Public Blizzard UI consumers
motivate fields and return shapes; association lookup, invalid-slot policy and
membership authority below are simulator contracts, **not native-parity claims**.
Backing model: `src/c_api/container_quest.rs`.

## What it must do

- [ ] Host-supplied metadata keyed by item ID contains an independent quest-item
  boolean and optional quest ID. Query the item currently occupying the requested
  bag slot; replacement/removal cannot leave stale slot-based metadata.
- [ ] C_Container returns a fresh non-nil table with boolean `isQuestItem`, optional
  numeric `questID`, and boolean `isActive`. A quest flag without an ID remains true.
- [ ] `isActive` reflects membership of that quest ID in
  `SimState.quest_log_entries.entries`, including seeded active entries; completion
  history alone does not make an item active. Do not OR in the lightweight list.
- [ ] Existing `ConfirmAcceptQuest` / `SetAbandonQuest` transitions change the next
  query false → true → false without changing supplied item metadata.
- [ ] Ordinary items without supplied metadata, empty slots, unknown bags, nonpositive
  slots and slots beyond supplied bag capacity return `(false, nil, false)` fields.
  The documentation specifies the table/field types but does **not** specify these
  edge-case values. These values are explicitly simulator-authored policy.
- [ ] On Wrath, the legacy global returns exactly three positional values:
  `isQuestItem, questId, isActive`, from the same backing model. No table wrapper.

## How it works

- [Lua API/state architecture](../wiki/systems/lua-api.md)
- [Admin quest verbs](../admin-api/quests.md)

## Implementation inventory

- `src/c_api/container_quest.rs` — passive supplied record and state-backed query.
- `src/c_api/mod.rs` — public backing-model module.
- `src/c_api/item_spell/c_container.rs` — C_Container registration hook.
- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs` — host-facing map/default.
- `src/lua_api/globals/real/container_legacy.rs` — Wrath positional projection.
- `src/lua_api/workarounds/temporary/container_default_shapes.rs` — quest entry retired;
  remaining no-state helpers unchanged.
- `src/lua_api/sim_substates/quest_log.rs`, `src/lua_api/globals/quest_verbs.rs` —
  existing membership authority and transitions; unchanged by this feature.

## Tests asserting this spec

`tests/container_item_quest_metadata.rs` (new top-level integration module):
supplied lifecycle, flag-only item, rich/history distinction, negative shapes,
slot identity, Wrath legacy lifecycle and legacy negative shapes.
`tests/bags.rs::test_bag_ui_c_container_helpers_have_safe_default_shapes`
remains unchanged as a no-supplied-metadata preservation test.

## Known gaps (current cycle)

- [ ] Execute behavioral RED on HEAD plus passive scaffold, before producer. Passive legacy `GetContainerItemQuestInfo` returns exactly `(false, nil, false)`; no pcall masks call/setup failures.
- [ ] Execute focused GREEN and default-shape preservation on required profiles.
- [ ] Run format/check and startup gates through the integrating session.

## Out of scope

Native item-to-quest catalogue discovery, quest acceptance on item use, new admin
Lua verbs, visual marker rendering/reachability, secret-argument/error parity,
legacy publication beyond directly evidenced Wrath, Cataclysm.
