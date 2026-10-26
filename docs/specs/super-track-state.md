# Supplied SuperTrack state

CF-2 supplies tracked targets through `src/c_api/c_super_track.rs` and per-quest highlights through `src/c_api/c_quest_log.rs`. [System notes](../wiki/systems/super-track-state.md) describe ownership and dispatch. Edit-only staging: all runtime requirements remain unchecked.

## What it must do

- [ ] Register the modeled namespace on Retail, PTR and Forever beside C_QuestLog through one shared runtime profile predicate; preserve unrelated namespace members.
- [ ] Start with absent quest/content/map-pin/vignette. Untracked quest returns nil: **documented-optional representation**, not established native sentinel parity. Pair queries return no results when absent; populated zero enum values remain valid.
- [ ] Round-trip quest and content selection, replace targets within each category and clear them; quest setter argument 0 clears. Clear-all removes all supported targets. IsSuperTrackingAnything reflects supported stored targets without priority arbitration.
- [ ] Publish committed state to the original selected Blizzard_QuestSuperTracking consumer through SUPER_TRACKING_CHANGED; its cached quest/content/highlight outputs must change and clear, including matching/nonmatching turn-in/watch events and eligible/ineligible auto-track branches.
- [ ] Return all five highlight outputs only from supplied per-quest rows. Unknown/0-ID calls without a row fail explicitly under simulator policy; **native evidence gap**, not a native empty-tuple contract. No fallback tuple.

## How it works

- [Ownership, consumer selection and proof bounds](../wiki/systems/super-track-state.md)

## Implementation inventory

- `src/c_api/c_super_track.rs` — SuperTrackState and modeled getters/mutators.
- `src/c_api/c_quest_log.rs` — QuestAdditionalHighlights supplied-row projection.
- `src/c_api/mod.rs` — shared runtime Retail/PTR/Forever publication predicate.
- `src/lua_api/state{,/sim_state}.rs` — environment-owned `super_track`, all fields None by default.
- `src/lua_api/sim_substates/quest_log.rs` — quest-log-owned additional_highlights rows, empty by default.
- `src/lua_api/workarounds/temporary/super_track_defaults.rs` — only unsupported priority query remains; no target fallback installers.

## Tests asserting this spec

- `tests/cf2_supertrack_consumers.rs` — five original-consumer selectors, each with state transitions; original complete chunk loaded via selected TOC, no consumer/API replacements.
- `tests/c_super_track_probes.rs` — public shapes after bootstrap, supplied map-pin/vignette state and explicit missing-highlight-row policy.
- Existing `tests/c_factory_m4_contracts.rs` absent-target assertions remain preservation, not producer RED.

## Known gaps (current cycle)

- [ ] Execute frozen RED, relevant constant/no-op mutants, GREEN, stock error-set diff, PTR/Forever preservation and checks using the handoff SLOT-RECIPE. No runtime evidence produced during staging.
- [ ] Native untracked quest sentinel, unknown/zero highlight semantics, event timing/deduplication and cross-target coupling remain unestablished. Mutation calls synchronously notify after releasing state borrow; this is simulator policy, not native timing parity.
- [ ] IsQuestTask is currently coupled to the static world-quest catalog. World/task rejection is covered together; independent task-only classification remains held, not mocked.

## Out of scope

Corpse, waypoint, scenario/party targets, priority arbitration, persistence, broader map-provider catalogs and user-facing map-pin/vignette setters. Existing highest-priority query remains a temporary unsupported nil response, retired only when native arbitration is modeled; namespace is not fully modeled. PTR readiness=false is preservation, not readiness RED.

## Deferred supertracking coverage restoration

`tests/cf2_supertracking_restored.rs` restores only the supertracking obligations from the deferred housing shape/boundary tests on Retail, PTR and Forever. Both exact selectors exercise supplied Appearance/101 and Achievement/202 pairs, clear, and re-entry through the selected original `Blizzard_QuestSuperTracking.lua` event/cache refresh. An external listener records the pair during notification; the original helper reads that cache. The shape selector also checks the documented zero-result absence versus populated two-number binding contract. The cache helper itself returns two fields, including two nils after clear.

Declarations were re-authenticated per profile at `Blizzard_APIDocumentationGenerated/SuperTrackManagerDocumentation.lua:32–40` and `ContentTrackingTypesDocumentation.lua:60–70`. These tests preserve an existing supplied producer; they do not establish native target policy, event deduplication or arbitration, and do not restore unrelated housing tests. Runtime proof is recorded separately in the lane report.
