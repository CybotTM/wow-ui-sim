# Housing spatial presence

The existing `HousingArea` enum has `HouseInterior` and `PlotExterior`. `HousingState.current_housing_area` is optional supplied world state, independent of `inside_owned_house` / `inside_owned_plot`. The existing `IsInsideHouse` getter matches only the interior. This lane stages the broader query without adding another plot flag. See the [house-or-plot contract](../../specs/house-or-plot-presence.md) and [interior contract](../../specs/house-interior-presence.md).

## Profile selection (static, 2026-10-24)

| Profile | Selected original consumer | Generated housing documentation |
|---|---|---|
| Retail/PTR | `Blizzard_Game_Mainline.toc`, Game allowed, `AllowLoadGameType: mainline`; Shared then Mainline EventImplementation/EventRouting, Startup | Generic `Blizzard_APIDocumentationGenerated.toc`, LoadOnDemand, HousingUIDocumentation.lua listed |
| Forever | Generic `Blizzard_Game.toc`; `[Family] -> Mainline`, `[Game] -> Camelot`; Camelot inline entries allowed, Classic/TBC/Wrath/Cata/Mists entries excluded; Camelot wraps and calls inherited Mainline world handler | Same generic LoadOnDemand documentation list |
| Wrath | No Blizzard_Game or generated-documentation TOC in inspected profile cache | Not present in inspected cache |
| Mists/Era/Anniversary | No Blizzard_Game TOC in inspected cache; not evidence of API absence everywhere | Generic LoadOnDemand documentation includes HousingUIDocumentation.lua, `AllowLoad: Both`; documentation presence alone does not select the consumer |

Selection authority: `src/loader/mod.rs::active_profile_toc_suffixes` and `src/toc/mod.rs::{active_game_types,family_subdir,game_subdir,is_allowed_game_type}`. Retail/PTR suffix order is Mainline, generic, Standard. Forever order is Camelot, generic, Mainline; no Camelot TOC exists for Game, so generic wins. Its active types are camelot/mainline, not standard.

Retail/PTR original Mainline handler lines769–848 include query at843. Forever Mainline handler starts782 and queries at851; Camelot handler line3 invokes its inherited Mainline closure. Shared EventRouting registers PLAYER_ENTERING_WORLD on a private frame; Startup initializes routing. Generated docs are available on demand, not loaded merely because the Game consumer runs.

## Producer staging

`src/c_api/c_housing/presence.rs` registers both spatial getters under one shared Retail/PTR/Forever module gate. The existing interior predicate is unchanged. The broader query returns `current_housing_area.is_some()` because the existing enum has exactly interior and plot-exterior variants. It returns one `Val::Bool`; no mutation, default fabrication or ownership fallback occurs. `location.inside_plot` remains an older independent input and is not silently reconciled by a getter. Runtime proof is pending.

## State and original-consumer selectors

`tests/cf2_housing_plot.rs` loads real panel dependencies and the selected Game closure. It checks TOC eligibility and original handler source, then fires registered PLAYER_ENTERING_WORLD. `hooksecurefunc` observers count original handler completion and actual original `HousingControls_LoadUI` calls without changing return values. The selected public housing bootstrap executes with its original addon vararg. Shape checks run AFTER the event, so missing API RED must originate in the real handler, not a direct getter probe.

Forever's controls TOC has `AllowLoadGameType: standard`, which current Forever selection rejects. Its consumer applies, but its complete true-branch fixture is blocked at that downstream loader boundary; the Retail/PTR selector is deliberately not registered there. Do not bypass TOC selection to manufacture Forever GREEN. Producer publication can still follow existing IsInsideHouse's three-profile scope.

## Static contradiction table

Selector: `cf2_housing_plot::original_world_event_branches_on_house_plot_absence_and_reentry` (Retail and PTR). Each row refers to that ONE selector alone, not pooled assertions across selectors.

| Candidate producer | Contradicting supplied state / observable outcome |
|---|---|
| Always true | None -> original housing-loader count must be0, not1 |
| Always false | Visiting HouseInterior AND visiting PlotExterior -> count must be1, not0 |
| Nil / no return / no-op | Same visiting states -> count1; post-consumer exact-one-boolean check also rejects missing/nil result |
| House-only (`IsInsideHouse`) | Visiting PlotExterior -> count1 |
| Ownership-only | Visiting interior and visiting plot with both ownership flags false -> count1 |
| Stale first result | None -> interior -> plot -> None -> owned plot -> owned interior -> None -> visiting plot; expected0,1,1,0,1,1,0,1 |
| Missing/unimplemented API | Original event diagnostic or incomplete original handler; expected boundary is Mainline EventImplementation:843 |

These are static contradictions, NOT observed mutation or RED proof. A setup/closure/earlier-handler failure does not count as RED. The deferred slot run must show the original :843 boundary and actual branch counts.

## Sources

- `src/c_api/c_housing.rs`, `src/lua_api/state/support_types.rs`, `src/c_api/c_housing/location.rs` — inspected existing model; older independent location flags are not fallback spatial inputs.
- Profile-scoped Blizzard runtime cache TOCs, Mainline/Shared/Camelot handlers and HousingUIDocumentation.lua — source hashes recorded in `/home/osso/.local/state/wow-ui-sim/handoff/cf2-housing-plot/consumer-source-hashes.json`.
- [House-or-plot contract](../../specs/house-or-plot-presence.md) — assertions and exclusions.

## See Also

- [Consumer selection and RED proof](../investigations/consumer-selection-and-red-proof.md).
- [House-interior presence](../../specs/house-interior-presence.md).
