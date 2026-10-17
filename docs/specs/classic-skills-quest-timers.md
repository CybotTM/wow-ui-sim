# Classic skill and quest-timer startup boundaries

Era/Anniversary must satisfy the unchanged cached SkillFrame and QuestTimer
consumers. Skill projection is shared with Wrath; no native-parity claim.
See [client profiles](../wiki/systems/client-profiles.md).

## What it must do

- [ ] Empty skill inputs report zero rows/no selection; the original detail
  consumer may add rank and temporary points before checking absent skill name.
- [ ] Explicit skill records and selection render rank through original
  SkillFrame.lua, preserving existing Wrath tuple/selection behavior.
- [ ] With no modeled legacy timed quests, GetQuestTimers returns **no values**,
  not nil or numeric zero. Original QuestTimer events hide all timer rows/frame.
- [ ] Keep other profiles' API behavior unchanged.

## How it works

- [Lua API](../lua-api.md)
- [Wrath residual contract](wrath-residual-b.md)

## Implementation inventory

- `src/lua_api/globals/real/legacy_skills.rs`: shared skill type/projection.
- `src/lua_api/globals/real/wrath_legacy_state.rs`: existing explicit storage;
  other Wrath projections remain registered only in Wrath.
- `src/era/compat_bootstrap.rs`: Era/Anniversary publication.
- `src/lua_api/workarounds/temporary/legacy_quest_timers.rs`: shared empty
  domain until legacy quest countdown/expiry state exists.

## Tests asserting this spec

`tests/classic_r10r14.rs` loads hash-pinned original consumers from each profile;
`tests/wrath_residual_b.rs` retains its existing skill-consumer fixture unchanged.

## Known gaps (current cycle)

- [ ] GREEN, touched-profile checks and seven-profile startup comparison pending.
- [ ] QuestLogState has no countdown/expiry inputs. quest_surface's fixed
  world-quest time-left projection is not a legacy timed-quest model.

## Out of scope

R20, R16, populated timers without a backing model, native invalid-index tuples,
Blizzard/vendor changes, invented skill catalog/defaults, Retail/PTR changes.
