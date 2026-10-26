# Housing spatial presence

The existing `HousingArea` enum has `HouseInterior` and `PlotExterior`. `HousingState.current_housing_area` is optional supplied world state, independent of `inside_owned_house` / `inside_owned_plot`. The existing `IsInsideHouse` getter matches only the interior. The broader query uses that same area without adding another plot flag. See the [house-or-plot contract](../../specs/house-or-plot-presence.md) and [interior contract](../../specs/house-interior-presence.md).

## Profile selection (static, 2026-10-24)

| Profile | Selected original consumer | Generated housing documentation |
|---|---|---|
| Retail/PTR | `Blizzard_Game_Mainline.toc`, Game allowed, `AllowLoadGameType: mainline`; Shared then Mainline EventImplementation/EventRouting, Startup | Generic `Blizzard_APIDocumentationGenerated.toc`, LoadOnDemand, HousingUIDocumentation.lua listed |
| Forever | Generic `Blizzard_Game.toc`; `[Family] -> Mainline`, `[Game] -> Camelot`; Camelot inline entries allowed, Classic/TBC/Wrath/Cata/Mists entries excluded; Camelot wraps and calls inherited Mainline world handler | Same generic LoadOnDemand documentation list |
| Wrath | No Blizzard_Game or generated-documentation TOC in inspected profile cache | Not present in inspected cache |
| Mists/Era/Anniversary | No Blizzard_Game TOC in inspected cache; not evidence of API absence everywhere | Generic LoadOnDemand documentation includes HousingUIDocumentation.lua, `AllowLoad: Both`; documentation presence alone does not select the consumer |

Selection authority: `src/loader/mod.rs::active_profile_toc_suffixes` and `src/toc/mod.rs::{active_game_types,family_subdir,game_subdir,is_allowed_game_type}`. Retail/PTR suffix order is Mainline, generic, Standard. Forever order is Camelot, generic, Mainline; no Camelot TOC exists for Game, so generic wins. Its active types are camelot/mainline, not standard.

Retail/PTR original Mainline handler lines769–848 include query at843. Forever Mainline handler starts782 and queries at851; Camelot handler line3 invokes its inherited Mainline closure. Shared EventRouting registers PLAYER_ENTERING_WORLD on a private frame; Startup initializes routing. Generated docs are available on demand, not loaded merely because the Game consumer runs.

## Producer

`src/c_api/c_housing/presence.rs` registers both spatial getters under one shared Retail/PTR/Forever module gate. The existing interior predicate is unchanged. The broader query returns `current_housing_area.is_some()` because the existing enum has exactly interior and plot-exterior variants. It returns one `Val::Bool`; no mutation, default fabrication or ownership fallback occurs. `location.inside_plot` remains an older independent input and is not silently reconciled by a getter. Retail original-consumer proof is complete; PTR acceptance depends on the separately owned input-interface-style lane (see the accepted-proof section).

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

These rows describe the planned contradictions. The accepted Retail proof below records actual failure boundaries, including nil/no-return rejection by the post-consumer shape check rather than a later visiting branch. A setup/earlier-handler failure never counts as endpoint RED.

## Sources

- `src/c_api/c_housing.rs`, `src/lua_api/state/support_types.rs`, `src/c_api/c_housing/location.rs` — inspected existing model; older independent location flags are not fallback spatial inputs.
- Profile-scoped Blizzard runtime cache TOCs, Mainline/Shared/Camelot handlers and HousingUIDocumentation.lua — source hashes recorded in `<handoff>/cf2-housing-plot/consumer-source-hashes.json`.
- [House-or-plot contract](../../specs/house-or-plot-presence.md) — assertions and exclusions.

## See Also

- [Consumer selection and RED proof](../investigations/consumer-selection-and-red-proof.md).
- [House-interior presence](../../specs/house-interior-presence.md).

## Current-house shape and PTR startup

The continuation uses authenticated Retail 12.1.5.70448 and private PTR 12.1.7.70493 caches, unchanged from the prior slot run. Both declare nullable `HouseInfo`; the exact fields and host-input contract are in the [current-house spec](../../specs/current-house-info.md). `HousingState.current_house_info` is an optional supplied row, projected by `src/c_api/c_housing/current_house.rs` on Retail/PTR only. No row is inferred from spatial presence, ownership or tracked selection. Native acquisition, nil-selection, validation and coherence remain **native-unknown**.

Merged-master original-consumer RED (fixture 4abbc60a0) reaches `Blizzard_HousingControls.lua:105` through original Game Mainline:844 and fails on missing `GetCurrentHouseInfo`. Original `Blizzard_HousingTutorialsData.lua` and `Blizzard_HousingTutorialsUtil.lua` loads remove the independent missing-utility fixture error without changing assertions. PTR separately fails Game startup registration on `SHARD_TRANSFER`, leaving original world completion at zero. Its pinned `SystemDocumentation.lua:175–178` explicitly declares that event. Root `src/event/valid_events.rs` extends registration and validity only when `client_profile::ACTIVE` is PTR; the feature-neutral `sim-event` crate remains unchanged.

Source hashes and original-consumer RED receipts are in `<handoff>/cf2-housing-plot/slot1-run2/`. GREEN, paired missing-producer proof and mutants remain pending at this implementation checkpoint; no native-parity claim.

## PTR keyring startup boundary

After SHARD_TRANSFER admission, the original PTR world event reaches `CloseAllWindows` in `UIParentPanelManager.lua:1180`, but `CloseAllBags` is nil: original `ContainerFrame.lua:10` aborted earlier on missing `C_ActionBar.ShouldShowKeyring`. This is the observed original-consumer RED, not a missing fixture load. The pinned `ActionBarFrameDocumentation.lua:1022–1028` declares a non-null boolean without arguments. User authorization covers this exact namespace, not a C_Container alias.

The [keyring capability contract](../../specs/ptr-keyring-capability.md) projects an explicit input in existing `ActionBarStateInfo`; no key-count, preference or native eligibility rule is guessed. The C API owns the runtime ACTIVE gate and getter. Native eligibility remains unknown. At that implementation checkpoint, GREEN was pending. The accepted-proof section records the later results; further unrelated missing APIs are dependencies, not permission to expand scope.

## Bootstrap observer lifetime

Retail 21f338ac1 reaches None and visiting-interior decisions without event diagnostics, then fails the third visiting-plot loader count (0 rather than 1). The fixture's direct bootstrap-file execution omitted loader bookkeeping. The first real controls load re-executes its `[Bootstrap]` file and replaces `HousingControls_LoadUI`, discarding the previously installed hook. This is an observer-lifetime fixture failure, not evidence that spatial presence rejects plots.

The approved fixture repair replaces direct HousingControls bootstrap execution with existing `loader::load_addon_bootstrap_from_toc` before observers. The real bootstrap phase records `bootstrap_loaded`; production's normal load already skips previously loaded bootstrap entries. Assertions, vendor bytes and production code are unchanged. RED/GREEN use identical repaired tests. The paired Retail rerun and mutants are proved below. PTR's separate C_InputInterfaceStyle dependency remains deferred.

## Accepted Retail proof and PTR dependency

Verified 2026-10-25. Final master input is f1f3c930d, merged once at 59a8f8acc. Executed repaired fixture GREEN 03ee43f44 and producer-only RED 6ed438196 have identical test trees and housing test SHA256 `cdf5c6c4cf8bac9869ed0248d6a54b3732ea234ddd65f927b7aaae99d55828b5`. Only the endpoint registration/function is subtracted in RED. No assertion, production loader or vendor change repairs the fixture.

| Capability / boundary | Retail | PTR 70493 private cache |
|---|---|---|
| Original eight-state world consumer | PASS, loader counts 0,1,1,0,1,1,0,1 | BLOCKED-ON `ptr-input-interface-style` merge; separate short rerun scheduled |
| Original missing-producer RED | Original Mainline:843 rejects missing IsInsideHouseOrPlot | Deferred with dependency; prior startup failures are not endpoint RED |
| Interior preservation | Both exact selectors pass on repaired GREEN and RED | Both exact selectors pass on production 21f338ac1; their source/production is unchanged by 03ee fixture repair |
| Nullable current-house shape, event gate, keyring capability | Concrete shape and Retail publication tests pass | Concrete shape, SHARD registration and false→true→false keyring capability pass |
| Final all-target check at 03ee43f44 | Exit 0, zero warnings | Exit 0, only three allowed fast_handler_late_bind dead_code warnings |

All six Retail callable mutants are rejected by the same exact selector with passing GREEN control. Always-true fails at None; always-false and ownership-only fail at visiting interior; house-only fails at visiting plot. Nil and no-return fail the one-non-nil-boolean check after the first original None event, not a visiting-branch assertion. Unique sealed binaries and source SHAs are recorded. PTR mutants are not acceptance evidence without PTR GREEN.

The PTR keyring fix gets past original ContainerFrame:10. Its next unconditional input-style query at line14 reaches missing `C_InputInterfaceStyle` in original SharedXML/Mainline/InputUtil.lua:12; the file aborts before publishing CloseAllBags, and the original Game handler subsequently fails UIParentPanelManager:1180. The separate `ptr-input-interface-style` lane owns this dependency. User direction is to report it and schedule a later PTR rerun, without another master merge or unrelated API expansion here.

Every runtime proof uses an exact listing, one exact run, a fresh byte-exact current CVar copy and O_NOATIME original-file guards. Retail 4028 and private PTR 4580 pinned cache files remain byte-identical. Full source/runtime/check/CPU receipts and the final matrix are in `<handoff>/cf2-housing-plot/slot1-run2/REPORT.md`. No new stock-startup comparison, Forever acceptance or native housing-policy parity is claimed.

## PTR talent-award startup boundary

After merging the input-interface-style lane, PTR's original world event passes the ContainerFrame input dependency but aborts at `MainMenuBarMicroButtons.lua:1016`, called by Shared Game `HandlePlayerEnteringWorld:311` through Mainline `:783`. The actual endpoint is `C_SpecializationInfo.HasPlayerEarnedATalentPoint`, not C_ClassTalents. Pinned 70493 `SpecializationInfoDocumentation.lua:331–337` declares no arguments and one non-null bool `hasEarned`; the prior PTR runtime inventory already lists this missing endpoint. Retail's pinned microbutton has no such call; no Retail producer exists to enable.

The [shape-only contract](../../specs/ptr-talent-award-history.md) supplies lifetime history in existing talent state and projects it through a PTR ACTIVE gate. Current ranks, unspent points and level do not establish lifetime awards. Empty host history is false; native acquisition and policy remain unknown. The new registered-world-event selector first fails at the original missing call before any standalone probe. Its producer GREEN and housing rerun are pending at this commit.

## PTR microbutton region lifetime

The talent producer exposes the next original-world boundary: `LegacyMicroButton:UpdateMicroButton:1195` calls original `SetNormal:504`, which dereferences a nil highlight. PTR XML inherits `MainMenuBarMicroButton` and does not declare HighlightTexture; original `LoadMicroButtonTextures:32–38` creates slots with the button atlas setters. The active `apply_atlas_setter` returned on missing atlas lookup before creating a region. The older method implementation created the region first. Legacy atlas entries are absent from active atlas data; this explains the lookup miss without authorizing invented pixels.

The [region-lifetime contract](../../specs/button-atlas-region-lifetime.md) restores creation and Lua publication before optional atlas application in the shared simulator setter. Original-template RED fails in unchanged vendor `SetNormal:504`; GREEN is pending at this commit. No vendor behavior or atlas catalog is changed. Native invalid-atlas semantics remain unproved.

## Texture fix acceptance and remaining PTR dependency

Executed c029ad36c: original PTR template GREEN follows the 4c6e7ae04 RED at SetNormal:504. Retail's original eight-state house/plot consumer and both interior selectors pass after the master merge; 10 atlas regressions and 19 button-texture tests also pass. Retail/PTR all-target checks pass with zero/three known dead-code warnings, and formatting passes. The texture fix does not fabricate missing atlas pixels or claim native invalid-atlas parity.

PTR housing/talent acceptance remains blocked before the housing branch. Original MicroMenu OnLoad aborts on missing C_GameRules.IsWoWHack at GameRulesUtil:207; InitializeButtons/RefreshButtons therefore never writes addedButtons (MicroMenuContainer:139), and UpdateMicroButtons:230 later rejects nil. This producer is separately owned. No local workaround or assertion change is authorized.

Raw stock comparison preserves every full-message key: Retail exact master f4387c7bf has 16 keys/18 occurrences; the lane has 14/16. Its two additions are existing dividerVerticalPool messages with only Game frame identity changed. PTR's requested 298/412 reference becomes 286/408, with 10 individually traced additions and 22 removals. Four additions are identity-only churn; the remaining six expose original MicroMenu/raid startup gaps after earlier boundaries are removed. The separately merged green1-adjusted reference is 296/410 and contributes two known GhostFrame keys retained on this lane's unmerged f438 base. No clean-stock or complete PTR GREEN claim is made. Full per-key traces, source/runtime receipts and CPU/release evidence live in the local ptr-final handoff REPORT.
