# Quest offer to rich-log coherence

Public acceptance must make the accepted offer visible through the existing rich quest-log queries. Current regression scope is Forever's existing `A_Admin.OpenMultiQuestNpc()` offer, selected by legacy offer index and confirmed through `ConfirmAcceptQuest()`. Source lives in `src/lua_api/globals/{admin_quests,quest_verbs}.rs`; see [Lua API](../wiki/systems/lua-api.md) and [watch contract](forever-quest-difficulty.md). This is a bounded simulator coherence contract, not native parity.

## What it must do

- [ ] Discover the existing offer through public gossip APIs, preserving its actual ID, title and level; do not inject pending state, construct a rich fixture row or fabricate metadata.
- [ ] After legacy `SelectAvailableQuest(1)` and `ConfirmAcceptQuest()`, consume the pending offer once and expose the accepted ID through `IsOnQuest`, `GetLogIndexForQuestID` and `GetInfo`, with the offered title/level. Existing difficulty projection remains equal to that rich level.
- [ ] Allow explicit ordinary watch insertion for the accepted row, with coherent count/type/watch-to-log mapping and exactly one occurrence. No automatic-watch policy is assumed; an existing watch remains intact.

## How it works

- [Event dispatch](../event-system.md)
- [Existing quest confirmation scope](quest-accept-confirmation.md) — separate Retail 12.0.5 contract; not expanded here.

## Implementation inventory

- `src/lua_api/globals/admin_quests.rs`: existing public seeded multi-quest NPC and offered metadata.
- `src/lua_api/globals/quest_surface/legacy_globals.rs`: existing offer-index selection and public metadata.
- `src/lua_api/globals/quest_verbs.rs`: current lightweight acceptance transition; no producer change in this test-only checkpoint.
- `src/lua_api/globals/missing_surface/{gossip_info,quest_log}.rs`: public offer and rich-log queries.
- `src/lua_api/sim_substates/quest_log.rs`: current rich rows.
- `src/c_api/c_quest_log_watches.rs`: explicit ordinary-watch membership over represented rich rows.

## Tests asserting this spec

`tests/wowforever_quest_offer_acceptance.rs` and `.lua`: fresh Forever environment, existing public NPC offer80002 (`Supply Run`), actual selection/confirmation, public rich/watch observations and final Rust assertion outside callbacks. Expected title/level come from the public offer. Rust also records lightweight occurrences, pending/selected state and queued event count; no native event timing/payload policy is asserted.

**UNEXECUTED:** authored RED candidate only. Existing sourcef91's39 model passes and normal watch/header GREEN do not cover offer acceptance. No production fix or RED/GREEN result is claimed.

## Known gaps (current cycle)

- [ ] Compile and execute the candidate, distinguishing a coherence failure from an unreachable or malformed fixture.
- [ ] Before a production fix, establish policy for rich-only fields absent from offered metadata; do not reuse seed defaults or invent native data.
- [ ] Abandonment, completion/turn-in, other-profile preservation and full lifecycle acceptance remain parent gaps, not covered by this insertion probe.

## Out of scope

This probe does not define confirmation prompts, native validation, automatic watches, native event timing/payloads or rich-only field defaults. Existing Retail confirmation policies remain unchanged. Broader quest lifecycle work remains open rather than inferred from this one case.
