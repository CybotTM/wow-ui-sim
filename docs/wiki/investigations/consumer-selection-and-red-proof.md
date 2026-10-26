# Consumer Selection and RED Proof

Lessons learned on 2026-10-23: cached Blizzard source is not proof of a runtime-selected consumer, and a passing consumer boundary is not necessarily producer acceptance. Findings below came from static reachability and test reviews; they do not claim executed RED/GREEN.

## Runtime-selected consumers

`data/blizzard-ui-files/<profile>.txt` defines synchronized cache inputs, not the implementation selected for a run. Trace the selected TOC, its filters/substitutions, and XML/Lua chain before claiming consumer applicability:

- `src/loader/mod.rs`: `find_toc_file`, `active_profile_toc_suffixes` and `select_profile_specific_toc_suffix` select TOC flavor; `discover_blizzard_addon_toc_pools_for_screen` applies screen/game-type restrictions and separates startup roots from the dependency/LoD pool.
- `src/toc/mod.rs`: `ParsedFileEntries::push_file_entry` applies `is_allowed_game_type`, substitutes `family_subdir`/`game_subdir`, then uses `selected_file_path`. `TocFile::is_game_type_restricted` handles addon-level restrictions.
- `src/loader/addon.rs`: `load_addon_files` visits parsed entries subject to phase, profile and environment checks. Selected source still needs a callable consumer path and its prerequisites.

**Forever StableUI:** `Blizzard_StableUI/Blizzard_StableUI.toc` selects `[Game]/Blizzard_StableUI.lua` and XML; `[Game]` resolves to `Camelot`. The selected Camelot implementation never calls `C_StableInfo.GetActivePetList`. The cached Standard implementation does, but is unselected. Its top-level arithmetic requires `PetConsts.STABLED_PETS_FIRST_SLOT_INDEX` and `EXTRA_PET_STABLE_SLOT`, absent from Forever's `PetConsts`; whole-file execution is therefore predicted to fail before that consumer. This was not runtime-tested. Do not force-load Standard or substitute modern constants to manufacture a Forever gap.

The upgrade/store audit found the same trap: cached Mainline upgrade calls were not selected on Mists/Era/Anniversary, and cached shared secure-store source was absent from the selected Retail/PTR/Forever TOC.

## Combat-log processor filter staging

[Filter-record contract](../../specs/combat-log-event-filters.md) separates the original ApplyFilterSettings lifecycle from native event matching. The cached secure Add/Clear declarations provide restriction markers but no argument or return signatures; missing signatures do not establish validation or default values. The staged producer retains opaque requests and exposes a simulator-only snapshot, without interpreting event/mask semantics. Retail/PTR execution is pending; case032/033 remain preservation evidence, not producer acceptance.

Do not classify Forever as inapplicable merely because this TOC says `mainline`: `src/toc/mod.rs::active_game_types` admits both `camelot` and `mainline` for Forever. Its cached CombatLogProcessor TOC selects the same secure Lua file. The current Retail/PTR-only stage does not cover that selected Forever consumer. Mists/Era/Anniversary reject the mainline-only TOC; Wrath also rejects it and has no processor in its cache.

## CF-2 supplied PvP match completion

The [completion contract](../../specs/pvp-match-completion.md) uses the existing `SimState.pvp_catalog.match_completed` snapshot. GREEN's `src/c_api/c_pvp.rs` callback pushes this boolean without mutation; namespace publication checks runtime ACTIVE Retail/PTR inside the backing field's epoch gate. `IsMatchActive` and `HasMatchStarted` remain separate and unchanged. No Forever producer or native lifecycle/default behavior is inferred. The paired RED removes only this callback and registration, retaining all test and documentation bytes.

The unchanged `cf2_pvp_completion` selector invokes the normal XML-created frame's installed script. It checks original method identity and Lua-file provenance, then observes exactly one call to that original method. Incomplete/active/completed/reset snapshots assert all eight actual event registrations, hidden presentation and no new Lua errors. Constant true contradicts snapshot 1; constant false and zero-return/nil no-ops contradict snapshot 3. Missing publication can error at the original completion call. The catalog and locklist selectors are preservation only, not independent producer RED.

This is static staging, not observed RED/GREEN or runtime selection proof. Historical PTR cache evidence predates the current 12.1.7 pin. Future PTR execution must first synchronize and verify a lane-private matching cache and use it in every phase; old cache evidence cannot establish current PTR selection. See the contract for bounded native gaps and the handoff for source-bound replay.

## Applicable registration and standalone RED

Register consumer tests only on evidenced applicable profiles, using the established file-level `#![cfg(any(feature = "client-…"))]` pattern or `post_wrath_ui_fixture!` when its six non-Wrath profiles match the consumer. The macro is defined in `tests/common/profile_fixtures.rs`; it is not a universal applicability predicate. Never return success at runtime for an inapplicable profile: libtest reports a vacuous PASS, even if stderr says N/A.

Each independently claimed RED selector must reject a constant/no-op producer on its own. A missing-member failure alone does not establish this. In the CF-2 reviews, separate match selectors admitted constant false and constant true respectively; combining inactive/incomplete, active/incomplete and inactive/complete observations in one selector defeated both. TTS checks needed two distinct preference pairs and reopen observations to reject constant getters/no-op setters.

Empty-catalog, empty-invitation, notification-only and already-saved-marker checks can preserve useful boundaries while allowing no-op producers. Label and count them as preservation, not producer RED; keep missing native or backing-state evidence explicit.

## Supplied social squelch

The [squelch contract](../../specs/social-squelch.md) adds one environment-local boolean input, not an account or voice-provider policy. `SocialRestrictionsDocumentation.lua` declares the getter on Retail/PTR at line 96 and Mists at line 62. The original selected Mainline/Classic `LFGList.lua` uses it in the local find/start restriction functions.

`tests/social_squelch.rs` loads the discovered game-screen GroupFinder closure without source overrides. It checks the loaded function's Mainline/Classic provenance and uses the original XML-created CategorySelection buttons. One selector supplies false/true/false and asserts both enabled states and both restriction tooltips. Constant false fails the middle state; constant true fails the first state; a nil/no-op producer cannot satisfy the original calls. A second environment holds true while the first changes, checking isolation. This is staged coverage until runtime receipts establish RED/GREEN.

## Sources

- [TOC resolver](../../../src/toc/mod.rs) and [loader selection](../../../src/loader/mod.rs) — selection rules verified against source.
- [Addon execution](../../../src/loader/addon.rs) — parsed-file execution gates.
- [Forever manifest](../../../data/blizzard-ui-files/wowforever.txt) — synchronized inputs, not runtime selection.
- [Forever constants](../../../src/c_api/forever_finite_constants.rs) — StableUI prerequisite mismatch.
- [Profile fixtures](../../../tests/common/profile_fixtures.rs) — compile-time registration mechanism.

## See Also

- [[addon-loading]] — runtime loading and dependency closure.
- [[blizzard-ui-test-lanes]] — unit versus addon-bootstrap boundaries.
