# d21 saved full-suite observation — PASS

Revision `d21ca3f64388d49b367803434ff1b0404b86c640`; unit `full-suite-1791833706`. Started October 12, 2026 19:35:07 UTC; finished 20:11:50 UTC. Complete JSON and full log read, not extrapolated from partial integration results.

| Phase | PASS executions | FAIL | skipped | exit |
|---|---:|---:|---:|---:|
| Integration | 10786 | 0 | 19 | 0 |
| Prefork primary | 2321 | 0 | not separately reported | 0 |
| Prefork auxiliary groups 2+1+1 | 4 | 0 | not separately reported | same phase |
| Library | 2001 | 0 | 0 | 0 |
| Total | 15112 | 0 | 19 reported | all phases 0 |

Terminal execution rows: 15112. Distinct exact name strings: 15112; not a claim of semantically distinct behaviors. Aux four counted once. SLOW notices, wrapper recaps, summaries and failure duplicates do not add executions.

## Exact selected outcomes
All following rows individually retained in saved log; no omitted-selector credit. Elapsed values are runner durations, not test-body measurements.

- `casc_asset_fallback::legacy_casc_asset_fallbacks_cover_known_missing_texture_paths`: PASS, log:5138 (0.054s runner elapsed)
- `casc_loading::casc_resolves_baseline_textures`: PASS, log:5145 (0.198s runner elapsed)
- `casc_loading::casc_resolves_legacy_encoding_key_fallback_textures`: PASS, log:5148 (0.234s runner elapsed)
- `casc_loading::casc_resolves_baseline_fonts`: PASS, log:5216 (2.447s runner elapsed)
- `guid_is_player::guid_is_player_active_populated_party_lifecycle`: PASS, log:6160 (0.204s runner elapsed)
- `guid_is_player::guid_is_player_authenticates_real_secrets_before_type_and_lookup`: PASS, log:6161 (0.201s runner elapsed)
- `guid_is_player::guid_is_player_exact_local_identity_and_closed_world_misses`: PASS, log:6163 (0.201s runner elapsed)
- `guid_is_player::guid_is_player_does_not_leak_records_between_environments`: PASS, log:6164 (0.325s runner elapsed)
- `guid_is_player::guid_is_player_requires_string_without_numeric_coercion`: PASS, log:6165 (0.224s runner elapsed)
- `guid_is_player::guid_is_player_same_env_kind_updates_and_unit_removal_are_live`: PASS, log:6166 (0.234s runner elapsed)
- `guid_is_player::guid_is_player_original_consumer_preserves_actual_account_predicate_for_misses`: PASS, log:6168 (0.620s runner elapsed)
- `guid_is_player::guid_is_player_target_and_focus_follow_explicit_kind_not_prefix`: PASS, log:6169 (0.268s runner elapsed)
- `guid_is_player::guid_is_player_original_object_api_consumer_accepts_represented_player`: PASS, log:6172 (0.604s runner elapsed)
- `startup_perf::observation::tests::addon_observation_roundtrip_preserves_values_and_only_diagnostic_counts`: PASS, log:8978 (0.036s runner elapsed)
- `talent_panel_onupdate_perf::visible_onupdate_collector_tracks_child_parent_visibility`: PASS, log:9142 (0.157s runner elapsed)
- `startup_perf::full_game_startup_stays_under_budget`: PASS, log:9787 (259.673s runner elapsed)
- `talent_panel_onupdate_perf::talent_panel_open_onupdate_tick_latency_stays_under_budget`: PASS, log:9918 (199.506s runner elapsed)
- `talent_panel_onupdate_perf::talent_panel_open_visible_onupdate_handlers_stay_within_ceiling`: PASS, log:10061 (207.988s runner elapsed)
- `patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep`: PASS, log:13136

GUID current nine: 9/9 PASS. Original 8.1.0 publication sweep: exact prefork case PASS. Original full-game startup, both talent performance selectors and three `casc_loading` selectors PASS. No CASC example/smoke-command execution credited.

## Raw failures, skips, warnings
JSON failures/new_failures arrays empty for all three phases; raw terminal failure scan empty. Integration19 skips are aggregate only: individual names/reasons absent, no behavioral credit. Integration38 slow cases reported, not failures.
Dependency-manifest warnings: six unique deprecated Clippy key diagnostics repeated in each of three phases (18 diagnostic occurrences), plus three “generated 6 warnings” recaps. No warning-free claim.
Keys: `large-enum-variant`, `map-entry`, `match-wildcard-for-single-variants`, `redundant-closure-for-method-calls`, `trivially-copy-pass-by-ref`, `type-complexity` in `iced-wgpu-patched/Cargo.toml`. Exact warning lines retained in proof.
Prefork startup diagnostic text is retained, but cannot be attributed to the selected startup/talent bodies. Nextest runner elapsed timings do not prove handler body timings or causal performance improvements.

## Source applicability
Read-only comparison d21 → HEAD `bd4fe8059f350ebae34b670a29e2ba9ff5a0c484`: no changes in src/tests/tools/Cargo.toml/Cargo.lock/build.rs/.cargo/scripts; scoped worktree clean. All later committed paths are docs/evidence. Exact paths and command exits retained in proof. No binary authentication, source-to-executable attestation, native classification or causal claim.

## Historical contrast
Caller-provided old533c:15102PASS/1FAIL,15103 executions,19 skips. New:15112PASS/0FAIL,15112 executions,19 skips (+9 executions, +10 PASS, −1 FAIL). Frozen BA:15038PASS/3FAIL remains unchanged. Neither historical failure is rewritten or retroactively passed. Historical seals/artifacts not read or hashed.

Only the two requested exclusive outputs written. No test/build/CLI job polling, rerun, cancellation, delegation or source edits.
