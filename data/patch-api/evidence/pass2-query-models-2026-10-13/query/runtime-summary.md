# Current runtime artifact gate — PASS

Epoch: `<VERIFICATION>/pass2-query-green-current/20261013T053532421312Z`. Read/followed verify/SKILL.md, adapted Mode B to caller's artifact-only runtime scope. No execution or delegation by verifier.

## Result and accounting

Initial 14 filters: **78 PASS, 1 FAIL, 1 ignored helper**; **79 unique executed cases**. Nine query modules **46/46 PASS**. Controls **31/31 PASS**: lifecycle 6, renamed Vertex probe 1, direct slots 17, Vertex animation 7. Missing-method comparison **PASS**; extra-method comparison **FAIL**, exit **101**. Ignored refresh helper is separate, not an executed case or failure.

Current baseline correction committed `72a8d83a82c73e8ad6371abd4f65faecf26e767c`: deletes exactly the 16 stale `CreateControlPoint` rows printed by initial stderr; no additional extra-method additions reported. Current default extra baseline contains no `:CreateControlPoint` rows. Commit also adds four spec lines. Initial failure remains recorded, never rewritten as initial PASS.

Corrected-extra followup: **PASS, one repeated execution; latest 79/79 unique cases PASS, 80 execution attempts total**. Followup uses the same extra comparison case; deduplicate against original 79. Do not rerun unaffected 78 cases. Runtime fs read in `tests/method_diff_coverage.rs:123-126`, selected default path at lines 91-97, extra test at lines 338-345: text baseline correction does not require recompilation. This statement covers baseline-only correction, not arbitrary later code changes.

| Filter | Expected executed | PASS | FAIL | ignored | Exit |
|---|---:|---:|---:|---:|---:|
| `cooldown_viewer_availability::` | 8 | 8 | 0 | 0 | 0 |
| `currency_transfer_activity::` | 3 | 3 | 0 | 0 | 0 |
| `zone_ability_icon::` | 4 | 4 | 0 | 0 | 0 |
| `c_ui_widget_manager_button_header::` | 9 | 9 | 0 | 0 | 0 |
| `assisted_action_input::` | 6 | 6 | 0 | 0 | 0 |
| `get_ticks_per_second::` | 3 | 3 | 0 | 0 | 0 |
| `voice_transcribing_input::` | 3 | 3 | 0 | 0 | 0 |
| `path_control_point::` | 4 | 4 | 0 | 0 | 0 |
| `ping_texture_kits::` | 6 | 6 | 0 | 0 | 0 |
| `animation_query_lifecycle::` | 6 | 6 | 0 | 0 | 0 |
| `patch_10_2_5_probe_factory::patch_10_2_5_vertex_color_probe_reaches_modeled_endpoint_methods` | 1 | 1 | 0 | 0 | 0 |
| `action_spell_slot_identifiers::` | 17 | 17 | 0 | 0 | 0 |
| `method_diff_coverage::` | 2 | 1 | 1 | 1 | 101 |
| `vertex_color_animation::` | 7 | 7 | 0 | 0 | 0 |

## Assertion reach (not inferred from names)

Read full source bodies of all 14 modules and full stdout/stderr, each result JSON and full runtime-results.json. Each per-filter JSON equals corresponding aggregate row. All executed named cases exist in source bodies. Named PASS and exit 0 establish completion through fail-fast assertions; no native WoW execution or assertion instrumentation claimed. Counts below mean unique passing test bodies reaching that boundary, not numbers of individual assertions; categories overlap and must not be summed.

- **actual_vm_secret assertionReachCount=5**: `zone_ability_icon.rs:64` `zone_ability_icon_authenticates_native_host_secrets_by_caller_taint`; `c_ui_widget_manager_button_header.rs:374` `button_header_documented_secret_arguments_require_an_untainted_caller`; `ping_texture_kits.rs:147` `pass2_ping_kit_untainted_secret_key_returns_exact_public_string`; `ping_texture_kits.rs:170` `pass2_ping_kit_tainted_secret_denial_precedes_validation_and_lookup`; `action_spell_slot_identifiers.rs:381` `actual_vm_secrets_reject_after_gc_without_mutation_and_public_calls_recover`
- **actual_cached_consumer assertionReachCount=4**: `cooldown_viewer_availability.rs:199` `availability_actual_settings_and_bounded_visibility_consume_live_state`; `currency_transfer_activity.rs:69` `currency_transfer_activity_drives_actual_bounded_immediate_spinner`; `c_ui_widget_manager_button_header.rs:321` `button_header_actual_cached_provider_filters_hidden_but_preserves_visible_data`; `ping_texture_kits.rs:204` `pass2_ping_kit_actual_cached_consumer_formats_both_atlas_names`
- **xml_load_and_playback assertionReachCount=2**: `path_control_point.rs:118` `path_xml_declared_points_reach_public_queries_and_none_playback`; `vertex_color_animation.rs:217` `vertex_color_animation_xml_matches_lua_source_example`
- **computed_layout assertionReachCount=3**: `path_control_point.rs:118` `path_xml_declared_points_reach_public_queries_and_none_playback`; `path_control_point.rs:173` `path_none_walks_sorted_supplied_points_without_moving_owner_or_sibling`; `path_control_point.rs:208` `path_stop_and_natural_finish_restore_each_playbacks_existing_offset`
- **secret_rejection_public_or_secure_recovery assertionReachCount=4**: `zone_ability_icon.rs:64` `zone_ability_icon_authenticates_native_host_secrets_by_caller_taint`; `c_ui_widget_manager_button_header.rs:374` `button_header_documented_secret_arguments_require_an_untainted_caller`; `ping_texture_kits.rs:170` `pass2_ping_kit_tainted_secret_denial_precedes_validation_and_lookup`; `action_spell_slot_identifiers.rs:381` `actual_vm_secrets_reject_after_gc_without_mutation_and_public_calls_recover`

Secret evidence uses actual rilua host wrappers, rooted globals, caller taint and explicit rejection/recovery checks; zone and direct-slot cases also collect garbage. Cache evidence reads active-profile cached vendor source without replacing queried APIs: bounded settings/visibility, immediate spinner, registered ButtonHeader provider and PingManager atlas sinks. Provider preserves exact data and filters hidden/unknown records; ping checks both exact atlas names. XML cases create real temporary TOC/XML and load_addon, then advance animation state. Layout assertions use compute_frame_rect and check positions, size, sibling isolation and restored pre-play offsets. Recovery category intentionally counts secret rejection/public-or-secure recovery only; broader lifecycle restoration also passes in controls and Path tests. Profiler proof covers fixed nanosecond frequency/conversions, not production elapsed timing.

Stale source comments saying staged/unrun do not override current named results; they likewise confer no native proof. Synthetic IDs/kits, NONE path interpolation, ordering/misses, public outputs and taint policies prove simulator contracts only. Cached consumers are bounded fixtures, not full startup or complete UI integration.

## Provenance and remaining scope

Reused all **3 current auth receipts**, PASS, under `compile-auth-current/`; **zero new executable hash reads**. Receipt artifact consistency is not independently signed historical source/build attestation. Runtime rows declare compiled source `6b522c595274906b7304dd302123e2e1f3561038`; correction is a later external baseline edit. Existing compile/auth report retains six manifest deprecation warnings; no warning-free claim.

**Current startup Lua-error gate now PASS with model-error diagnostics (below); default check and PTR remain pending.** No tests, builds, checks, formatting, startup, runtime operations, binary reads/hashes, tracked edits or delegation performed here. Read-only Git inspection confirmed correction content and no current helper diff relative to correction commit. Reports written only in epoch/runtime-independent.

