# Sealed receiver/RED verification

Scope: executable code sealed at 76cf33ef327bbd9305706d8dd3f375b609e824d4; runtime docs/data read from ad0bd92a8922437791c38b6ca5c20705ec70b09f checkout. Rust/Cargo diff was empty before runs. No producer implementations edited.

Compile: 742 JSON records read (666 compiler-artifact, 75 build-script-executed, 1 successful build-finished), terminal success/0. Six iced manifest deprecation warnings in full stderr. Both newly emitted test executables fresh=false, copied/authenticated once each under deps with exclusive per-artifact receipts. No old seals or libraries hashed.

Runtime: 46 exact-filter invocations; timeout 90 each; one test thread; CASC/sound/addons/savedvars disabled. 12 test PASS, 33 test FAIL, 1 fixture FAIL; 12 exit 0 and 34 exit 101; zero timeouts. Full per-test stdout/stderr and runtime-ledger.jsonl preserved.

item_stats_supplied: {'FAIL': 9}

item_set_bonuses_supplied: {'PASS': 1, 'FAIL': 8}

ping_cooldown_interval: {'FIXTURE_FAIL': 1, 'FAIL': 5}

party_ai_membership: {'FAIL': 9}

patch_10_2_5_secure_cvars: {'PASS': 3, 'FAIL': 1}

vertex_color_animation: {'PASS': 7}

method_diff_coverage: {'FAIL': 1, 'PASS': 1}

F1 Ping consumer: test line 129 uses ACTIVE.subdir() = Retail (client_profile.rs:121), not cache_subdir() = retail (line 133). Wrong path exists=False; lowercase cache source exists=True. Missing source fails before vendor consumer assertions; NOT behavior RED.

CVar reach from successful test loops and failure location: 28+28+4 = 60 completed cases; first denial case autoStand/SetCVar/lowercase/combat/tainted entered and failed unexpected permission result at test line 102. Remaining 27 denial cases not reached. 88 planned matrix cases is NOT 88 executed cases. These tests are simulator contracts, not native proof; denial control only checks pcall outcome, not exact native error class.

AI secure secret test stops at first secure wrapped string query with expected string, got userdata (test line 286). Later wrong-type and tainted-secret classification assertions NOT reached; message substring checks are not native proof.

VertexColor 7/7 PASS including receiver exclusion. Method missing snapshot PASS; extras snapshot FAIL at line 302 with exactly 32 new GetFileWidth/GetFileHeight entries across 16 receiver types, including Texture. No stale entries reported. Not 32 Texture-only entries. Four color-endpoint names absent from committed extras baseline; old 64 endpoint pollution not reported in current diff.

EditMode lib tests not rerun: user supplied prior 0805 PASS and declared broad verification unnecessary. No fresh EditMode runtime claim. Lib executable newly authenticated but not executed.

No build/check, source edits, baseline refresh, services, user config/WTF edits, or delegation performed. Seal remains 76cf33ef regardless of future producer fixes; runtime evidence is invalid for changed producer code until rebuilt/reverified.

## Exact per-test outcomes

### item_stats_supplied::item_stats_delta_is_first_minus_second_over_union_in_both_directions — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.45s

```text
thread 'item_stats_supplied::item_stats_delta_is_first_minus_second_over_union_in_both_directions' (<PID>) panicked at $REPO/tests/item_stats_supplied.rs:70:22:
item-stat contract assertion: Lua(Runtime(RuntimeError { message: "expected stat table", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_stats_supplied::item_stats_delta_omits_equal_and_explicit_zero_keys_but_returns_empty_table — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.45s

```text
thread 'item_stats_supplied::item_stats_delta_omits_equal_and_explicit_zero_keys_but_returns_empty_table' (<PID>) panicked at $REPO/tests/item_stats_supplied.rs:70:22:
item-stat contract assertion: Lua(Runtime(RuntimeError { message: "(string):3: attempt to index local 'delta' (a nil value)", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_stats_supplied::item_stats_missing_exact_link_or_either_delta_operand_returns_nil — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.44s

```text
thread 'item_stats_supplied::item_stats_missing_exact_link_or_either_delta_operand_returns_nil' (<PID>) panicked at $REPO/tests/item_stats_supplied.rs:70:22:
item-stat contract assertion: Lua(Runtime(RuntimeError { message: "expected stat table", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_stats_supplied::item_stats_plain_invalid_types_error_in_every_required_position — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.46s

```text
thread 'item_stats_supplied::item_stats_plain_invalid_types_error_in_every_required_position' (<PID>) panicked at $REPO/tests/item_stats_supplied.rs:70:22:
item-stat contract assertion: Lua(Runtime(RuntimeError { message: "invalid type must error", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_stats_supplied::item_stats_results_are_fresh_and_mutation_does_not_change_later_queries — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.43s

```text
thread 'item_stats_supplied::item_stats_results_are_fresh_and_mutation_does_not_change_later_queries' (<PID>) panicked at $REPO/tests/item_stats_supplied.rs:70:22:
item-stat contract assertion: Lua(Runtime(RuntimeError { message: "every call must return an independent table", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_stats_supplied::item_stats_secure_genuine_secrets_unwrap_then_validate_each_position — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.40s

```text
thread 'item_stats_supplied::item_stats_secure_genuine_secrets_unwrap_then_validate_each_position' (<PID>) panicked at $REPO/tests/item_stats_supplied.rs:70:22:
item-stat contract assertion: Lua(Runtime(RuntimeError { message: "expected stat table", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_stats_supplied::item_stats_supplied_links_return_both_exact_numeric_maps — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.37s

```text
thread 'item_stats_supplied::item_stats_supplied_links_return_both_exact_numeric_maps' (<PID>) panicked at $REPO/tests/item_stats_supplied.rs:70:22:
item-stat contract assertion: Lua(Runtime(RuntimeError { message: "expected stat table", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_stats_supplied::item_stats_supplied_state_is_isolated_between_environments — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.54s

```text
thread 'item_stats_supplied::item_stats_supplied_state_is_isolated_between_environments' (<PID>) panicked at $REPO/tests/item_stats_supplied.rs:70:22:
item-stat contract assertion: Lua(Runtime(RuntimeError { message: "expected stat table", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_stats_supplied::item_stats_tainted_secrets_are_denied_before_parsing_any_original_argument — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.38s

```text
thread 'item_stats_supplied::item_stats_tainted_secrets_are_denied_before_parsing_any_original_argument' (<PID>) panicked at $REPO/tests/item_stats_supplied.rs:70:22:
item-stat contract assertion: Lua(Runtime(RuntimeError { message: "secret must be denied", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_set_bonuses_supplied::item_set_absent_pairs_return_nil_without_derived_defaults — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.24s

### item_set_bonuses_supplied::item_set_both_arguments_require_numbers_without_string_coercion — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.29s

```text
thread 'item_set_bonuses_supplied::item_set_both_arguments_require_numbers_without_string_coercion' (<PID>) panicked at $REPO/tests/item_set_bonuses_supplied.rs:57:22:
actual item-set query behavior: Lua(Runtime(RuntimeError { message: "invalid type must error", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_set_bonuses_supplied::item_set_numeric_selectors_do_not_truncate_or_saturate_to_supplied_pairs — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.37s

```text
thread 'item_set_bonuses_supplied::item_set_numeric_selectors_do_not_truncate_or_saturate_to_supplied_pairs' (<PID>) panicked at $REPO/tests/item_set_bonuses_supplied.rs:57:22:
actual item-set query behavior: Lua(Runtime(RuntimeError { message: "spell ID list", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_set_bonuses_supplied::item_set_pair_lookup_preserves_exact_spell_ids_order_and_duplicates — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.43s

```text
thread 'item_set_bonuses_supplied::item_set_pair_lookup_preserves_exact_spell_ids_order_and_duplicates' (<PID>) panicked at $REPO/tests/item_set_bonuses_supplied.rs:57:22:
actual item-set query behavior: Lua(Runtime(RuntimeError { message: "spell ID list", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_set_bonuses_supplied::item_set_results_are_fresh_and_mutation_cannot_change_later_queries — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.36s

```text
thread 'item_set_bonuses_supplied::item_set_results_are_fresh_and_mutation_cannot_change_later_queries' (<PID>) panicked at $REPO/tests/item_set_bonuses_supplied.rs:57:22:
actual item-set query behavior: Lua(Runtime(RuntimeError { message: "spell ID list", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_set_bonuses_supplied::item_set_supplied_empty_list_is_distinct_from_absence — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.42s

```text
thread 'item_set_bonuses_supplied::item_set_supplied_empty_list_is_distinct_from_absence' (<PID>) panicked at $REPO/tests/item_set_bonuses_supplied.rs:57:22:
actual item-set query behavior: Lua(Runtime(RuntimeError { message: "spell ID list", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_set_bonuses_supplied::item_set_supplied_pairs_are_environment_local — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.57s

```text
thread 'item_set_bonuses_supplied::item_set_supplied_pairs_are_environment_local' (<PID>) panicked at $REPO/tests/item_set_bonuses_supplied.rs:57:22:
actual item-set query behavior: Lua(Runtime(RuntimeError { message: "spell ID list", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_set_bonuses_supplied::item_set_tainted_secrets_deny_before_either_argument_validation_or_miss — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.35s

```text
thread 'item_set_bonuses_supplied::item_set_tainted_secrets_deny_before_either_argument_validation_or_miss' (<PID>) panicked at $REPO/tests/item_set_bonuses_supplied.rs:57:22:
actual item-set query behavior: Lua(Runtime(RuntimeError { message: "permission denial", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### item_set_bonuses_supplied::item_set_untainted_callers_accept_genuine_secrets_in_either_original_argument — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.36s

```text
thread 'item_set_bonuses_supplied::item_set_untainted_callers_accept_genuine_secrets_in_either_original_argument' (<PID>) panicked at $REPO/tests/item_set_bonuses_supplied.rs:57:22:
actual item-set query behavior: Lua(Runtime(RuntimeError { message: "spell ID list", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### ping_cooldown_interval::actual_vendor_onenter_uses_absolute_end_ms_and_requests_three_point_five_seconds — FIXTURE_FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.43s

```text
thread 'ping_cooldown_interval::actual_vendor_onenter_uses_absolute_end_ms_and_requests_three_point_five_seconds' (<PID>) panicked at $REPO/tests/ping_cooldown_interval.rs:129:47:
cached active-profile PingUI source: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### ping_cooldown_interval::default_interval_is_nonnil_zero_and_accessible_to_tainted_zeroarg_caller — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.40s

```text
thread 'ping_cooldown_interval::default_interval_is_nonnil_zero_and_accessible_to_tainted_zeroarg_caller' (<PID>) panicked at $REPO/tests/ping_cooldown_interval.rs:31:6:
public getter projects exact interval: Lua(Runtime(RuntimeError { message: "required nonnil DTO", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### ping_cooldown_interval::every_read_is_a_fresh_dto_and_mutation_does_not_change_state — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.43s

```text
thread 'ping_cooldown_interval::every_read_is_a_fresh_dto_and_mutation_does_not_change_state' (<PID>) panicked at $REPO/tests/ping_cooldown_interval.rs:86:6:
DTO identity and observable mutation independence: Lua(Runtime(RuntimeError { message: "each read owns a fresh DTO", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### ping_cooldown_interval::live_state_replacement_and_explicit_reset_affect_the_next_read — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.45s

```text
thread 'ping_cooldown_interval::live_state_replacement_and_explicit_reset_affect_the_next_read' (<PID>) panicked at $REPO/tests/ping_cooldown_interval.rs:31:6:
public getter projects exact interval: Lua(Runtime(RuntimeError { message: "required nonnil DTO", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### ping_cooldown_interval::simultaneous_environments_do_not_share_interval_or_reset_state — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.61s

```text
thread 'ping_cooldown_interval::simultaneous_environments_do_not_share_interval_or_reset_state' (<PID>) panicked at $REPO/tests/ping_cooldown_interval.rs:31:6:
public getter projects exact interval: Lua(Runtime(RuntimeError { message: "required nonnil DTO", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### ping_cooldown_interval::supplied_interval_is_returned_as_absolute_milliseconds — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.40s

```text
thread 'ping_cooldown_interval::supplied_interval_is_returned_as_absolute_milliseconds' (<PID>) panicked at $REPO/tests/ping_cooldown_interval.rs:31:6:
public getter projects exact interval: Lua(Runtime(RuntimeError { message: "required nonnil DTO", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### party_ai_membership::aliases_follow_existing_guid_not_target_spelling_or_display_name — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.37s

```text
thread 'party_ai_membership::aliases_follow_existing_guid_not_target_spelling_or_display_name' (<PID>) panicked at $REPO/tests/party_ai_membership.rs:84:5:
assertion failed: is_ai(&env, "target")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### party_ai_membership::cached_name_consumer_uses_real_ai_query_and_live_membership — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.48s

```text
thread 'party_ai_membership::cached_name_consumer_uses_real_ai_query_and_live_membership' (<PID>) panicked at $REPO/tests/party_ai_membership.rs:324:5:
assertion `left == right` failed: unit=party1, prefixed=true
  left: "Supplied Ally"
 right: "Follower: Supplied Ally"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### party_ai_membership::documented_secret_argument_authentication_preserves_caller_and_wrapper — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.44s

```text
thread 'party_ai_membership::documented_secret_argument_authentication_preserves_caller_and_wrapper' (<PID>) panicked at $REPO/tests/party_ai_membership.rs:286:6:
cached AllowedWhenUntainted contract; source stub currently lacks decoding: Lua(Runtime(RuntimeError { message: "expected string, got userdata at argument 1", level: 1, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### party_ai_membership::follower_dungeon_does_not_supply_ai_and_ai_does_not_require_dungeon — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.46s

```text
thread 'party_ai_membership::follower_dungeon_does_not_supply_ai_and_ai_does_not_require_dungeon' (<PID>) panicked at $REPO/tests/party_ai_membership.rs:216:5:
AI is not dungeon classification
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### party_ai_membership::live_inactivation_and_roster_removal_clear_effective_membership — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.46s

```text
thread 'party_ai_membership::live_inactivation_and_roster_removal_clear_effective_membership' (<PID>) panicked at $REPO/tests/party_ai_membership.rs:135:5:
assertion failed: is_ai(&env, "focus")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### party_ai_membership::missing_unknown_and_supplied_nonparty_identity_are_false — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.42s

```text
thread 'party_ai_membership::missing_unknown_and_supplied_nonparty_identity_are_false' (<PID>) panicked at $REPO/tests/party_ai_membership.rs:121:5:
negative controls do not clear real input
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### party_ai_membership::nullable_argument_and_plain_type_policy_are_preserved — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.52s

```text
thread 'party_ai_membership::nullable_argument_and_plain_type_policy_are_preserved' (<PID>) panicked at $REPO/tests/party_ai_membership.rs:241:6:
existing nullable strict-string extraction and ignored extras: Lua(Runtime(RuntimeError { message: "assertion failed!", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### party_ai_membership::supplied_membership_is_reversible_and_environment_owned — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.77s

```text
thread 'party_ai_membership::supplied_membership_is_reversible_and_environment_owned' (<PID>) panicked at $REPO/tests/party_ai_membership.rs:170:5:
assertion failed: is_ai(&first, "party1")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### party_ai_membership::supplied_party_guid_is_ai_ordinary_member_is_not — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.53s

```text
thread 'party_ai_membership::supplied_party_guid_is_ai_ordinary_member_is_not' (<PID>) panicked at $REPO/tests/party_ai_membership.rs:60:5:
assertion failed: is_ai(&env, "party1")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### patch_10_2_5_secure_cvars::patch_10_2_5_extension_preserves_existing_nameplate_combat_protection — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 1.21s

### patch_10_2_5_secure_cvars::patch_10_2_5_seven_secure_cvars_accept_insecure_out_of_combat_writes — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 7.07s

### patch_10_2_5_secure_cvars::patch_10_2_5_seven_secure_cvars_accept_secure_combat_writes — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 5.79s

### patch_10_2_5_secure_cvars::patch_10_2_5_seven_secure_cvars_deny_insecure_combat_writes — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.44s

```text
thread 'patch_10_2_5_secure_cvars::patch_10_2_5_seven_secure_cvars_deny_insecure_combat_writes' (<PID>) panicked at $REPO/tests/patch_10_2_5_secure_cvars.rs:102:9:
autoStand, SetCVar, uppercase=false, combat=true, tainted=true: Lua error: autoStand: unexpected permission result
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### vertex_color_animation::vertex_color_animation_child_target_is_isolated_and_restored_on_stop_hide_and_replay — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.40s

### vertex_color_animation::vertex_color_animation_endpoints_are_independent_color_objects — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.41s

### vertex_color_animation::vertex_color_animation_endpoints_are_scoped_to_vertex_color_receivers — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.40s

### vertex_color_animation::vertex_color_animation_interpolates_all_channels_and_restores_on_finish — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.40s

### vertex_color_animation::vertex_color_animation_smoothing_and_delays_follow_modeled_time_progress — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.42s

### vertex_color_animation::vertex_color_animation_source_bounce_and_reverse_use_owner_timeline — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.38s

### vertex_color_animation::vertex_color_animation_xml_matches_lua_source_example — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 3.52s

### method_diff_coverage::diff_methods_extra_snapshot_matches_current_metatable_surface — FAIL / exit 101

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.51s

```text
thread 'method_diff_coverage::diff_methods_extra_snapshot_matches_current_metatable_surface' (<PID>) panicked at $REPO/tests/method_diff_coverage.rs:302:5:
diff_methods_extra.txt is out of sync with the current metatable surface.

Add new entries:
Button:GetFileHeight
Button:GetFileWidth
CheckButton:GetFileHeight
CheckButton:GetFileWidth
ColorSelect:GetFileHeight
ColorSelect:GetFileWidth
Cooldown:GetFileHeight
Cooldown:GetFileWidth
EditBox:GetFileHeight
EditBox:GetFileWidth
FontString:GetFileHeight
FontString:GetFileWidth
Frame:GetFileHeight
Frame:GetFileWidth
GameTooltip:GetFileHeight
GameTooltip:GetFileWidth
MessageFrame:GetFileHeight
MessageFrame:GetFileWidth
Model:GetFileHeight
Model:GetFileWidth
PlayerModel:GetFileHeight
PlayerModel:GetFileWidth
ScrollFrame:GetFileHeight
ScrollFrame:GetFileWidth
SimpleHTML:GetFileHeight
SimpleHTML:GetFileWidth
Slider:GetFileHeight
Slider:GetFileWidth
StatusBar:GetFileHeight
StatusBar:GetFileWidth
Texture:GetFileHeight
Texture:GetFileWidth
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### method_diff_coverage::diff_methods_missing_snapshot_matches_current_metatable_surface — PASS / exit 0

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10899 filtered out; finished in 0.62s

