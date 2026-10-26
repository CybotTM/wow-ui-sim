# Mists scenario finder eligibility

Mists `C_LFGInfo.CanPlayerUseScenarioFinder` reads `SimState.scenario_finder_availability: Option<(bool, String)>`. The [contract](../../specs/mists-scenario-finder.md) requires a supplied tuple, not an eligibility inference from level, expansion or scenario catalog.

## Publication and state

The namespace registrar publishes the query only when `client_profile::ACTIVE` is Mists. The producer returns exactly two values, preserving the supplied failure-reason string even when eligibility is true. None raises the explicit named missing-input error. That is simulator policy, not an assertion about native defaults.

## Selected original consumer

Pinned Mists 5.5.4.70268 `Blizzard_APIDocumentationGenerated/LFGInfoDocumentation.lua:51` declares both returns non-nil. The selected Classic GroupFinder TOC loads the unchanged PVEFrame Lua/XML. Its original XML-bound OnEvent callback reads the tuple at270. The separate `ScenariosList` visibility branch hides an empty list; with entries, false disables the original button and copies the failure reason, while true enables it and clears the tooltip.

The test supplies only the Lua presentation list to exercise both branches; it does not replace any query, callback, widget or source file. Allowed-first and denied-first selectors each reverse eligibility, change denial reasons and player levels, and assert isolation across two environments. Direct return-count and tuple checks cover the string that the allowed consumer branch discards.

## LFR successor advance

The [LFR regression](../../specs/mists-lfr-availability.md) keeps its pre270 observer, all visibility and hidden-gap anchors, and environment/unset assertions. It now supplies scenario eligibility and requires the entire original callback to return successfully. The former exact missing-function acceptance is removed identically in both siblings. This advances the successor boundary; it does not reduce the LFR assertions.

## Proof

Allowed-first and denied-first selectors each fail RED at original270 with the missing query and pass GREEN with ten original-button observations, two environments and unset recovery. The byte-identical advanced LFR regression also rejects missing270 and passes full-callback GREEN with its ten LFR observations. No new successor occurs inside the PVEFrame callback.

Pinned-master and GREEN Mists stock maps are identical at6 keys/12 occurrences, with no additions, removals or count changes. Stock still stops at the earlier unsupplied LFR query at229; separate stock GetMinLevel at ScenarioFinder_Shared.lua:58 is also pre-existing. Startup cleanliness and native defaults are not claimed. All14 executable invocations use protected fresh child copies of the current persisted CVar file; all original stat/hash guards pass.

All seven profiles have separate full-feature all-target checks with zero Cargo JSON and plain compiler warnings. Formatting and changed-line manual Rust readability pass. Other-profile runtime execution and full-suite/CI acceptance are not claimed.

## Sources

- [Supplied eligibility contract](../../specs/mists-scenario-finder.md)
- `tests/mists_scenario_finder.rs` — original-consumer transitions, tuple shape and isolation.
- `src/c_api/scenario_finder_availability.rs` — supplied tuple query and explicit unset error.
- `src/c_api/c_lfg_info.rs` — profile publication boundary.

## Supplied active-party minimum

The [Mists minimum contract](../../specs/mists-party-minimum.md) adds `SimState.mists_party_min_level: Option<f64>`, initially None. The Mists-only `C_PartyInfo.GetMinLevel()` producer returns that observation exactly. It rejects explicit arguments and raises a named native-unknown error for None; it does not infer from player levels, roster or eligibility. Retail/PTR keep their existing producer.

Pinned Mists 5.5.4.70268 `PartyInfoDocumentation.lua:125–135` declares the query; line130 specifies the active party when category is omitted. The selected original `ScenarioFinder_Shared.lua:58` fails without the producer. Tests supply85/90 and exercise the original empty-catalog cover and label, plus direct value/count/isolation and clearing. Empty-catalog output is preservation, not minimum-sensitive presentation. Catalog discrimination remains blocked pending native contracts for the four legacy catalog queries. Runtime GREEN and profile checks are pending.

## See Also

- [Mists LFR availability](mists-lfr-availability.md)
- [Party minimum](../../specs/party-min-level-scenario-availability.md) — separate Retail/PTR catalog contract.
