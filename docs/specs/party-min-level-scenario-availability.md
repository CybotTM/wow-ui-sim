# Supplied party minimum and scenario availability (CF-2)

`C_PartyInfo.GetMinLevel` must derive from existing supplied player/active-party levels. Only scenario catalog projections needed by the original ScenarioFinder availability consumer belong in this slice. C namespace model/query code belongs in `src/c_api/`; legacy catalog globals may delegate there. [Consumer proof notes](../wiki/investigations/consumer-selection-and-red-proof.md#cf-2-party-minimum-staging) describe selection and proof boundaries.

## What it must do

- [ ] Retail and PTR normally select the GroupFinder Game TOC and its shared ScenarioFinder Lua/XML. Use the original XML-created frame, cover, label and script bindings; prove Lua source provenance before calling availability. Never execute an unselected file or replace vendor functions/widgets.
- [ ] With supplied player60 and an active connected member49, original availability shows the cover and the original localized next-level50 label. Moving that member to50 opens the cover at the inclusive minimum without changing player level.
- [ ] Player48/member50 yields48; player100/member71 yields71 and next80. Members80 and90 are available; member91 has no available scenario or next threshold. Each standalone selector rejects constant party getters.
- [ ] Project supplied `lfd_dungeons` scenario rows only: specific/random ranges50–70 and80–90, plus an ordinary-dungeon distractor1–100. Replacement lower thresholds55 then65 change the original label/cover; each standalone selector rejects constant catalogs and non-scenario inclusion.
- [ ] Clearing the supplied roster and active flag recovers supplied player60. This is a bounded simulator policy, not established native ungrouped/category/disconnected semantics.

## How it works

- [Runtime selection and RED proof](../wiki/investigations/consumer-selection-and-red-proof.md#cf-2-party-minimum-staging).
- [C API boundary and profile rules](../../AGENTS.md#c-api-boundary).

## Implementation inventory

- `tests/cfactory_cf2_social.rs` — staged original selected-consumer selectors; no observed RED yet.
- `src/c_api/c_party_info.rs` — existing party query integration point; GetMinLevel remains absent at staging.
- `src/lua_api/globals/battlefield_lfg_probes.rs` — existing scenario globals return empty placeholders; replacement awaits observed RED.
- `src/lua_api/state_types/collections.rs` — existing `LfdDungeonInfo` catalog records, reused without a second catalog.
- `src/lua_api/game_data.rs`, `src/lua_api/state/sim_state.rs` — existing player/party inputs, no second roster.

## Tests asserting this spec

Both selectors in `tests/cfactory_cf2_social.rs`, automatically discovered by the integration harness:

- `cfactory_cf2_social::original_scenario_specific_catalog_uses_lowest_party_level`
- `cfactory_cf2_social::original_scenario_random_catalog_uses_lowest_party_level`

Each runs all roster, boundary, replacement-catalog and supplied-solo observations independently. Registered on Retail/PTR/Mists; no runtime success-return for unsupported profiles. Compilation and executable listing remain pending admission.

## Known gaps (current cycle)

- [ ] Acquire an authorized build-slot marker and literal capacity `| ADD:`; compile/list selectors and observe each Retail/PTR RED at the original GetMinLevel call before producer changes.
- [ ] Implement the smallest shared minimum and scenario catalog projections, then prove both independent selectors GREEN on each applicable profile, with zero new warnings including test targets.
- [ ] Mists remains part of the goal, but explicitly BLOCKED if original GroupFinder setup hits unresolved `C_LFGInfo.IsLFREnabled` policy. Preserve diagnostics; do not call that setup failure producer RED or install an availability shim.
- [ ] Compare complete own-base and producer stock message→count maps for each applicable profile, including additions and surviving count changes. Historical opportunity2 keys/2 occurrences is not a guaranteed delta or prefork pass.

## Out of scope

- Squelch/LFR availability defaults, club state and generic LFG redesign.
- Unknown/disconnected roster, argument coercion and category semantics without separate evidence; native solo parity remains unproven.
- Vendor changes, test-only producer overrides, ambient/environment gameplay inputs, new availability shims and invented native defaults.
- Full-suite/prefork acceptance or seven-profile compatibility claims from this bounded slice.
