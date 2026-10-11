# Final WowForever saved-artifact verification

2026-10-11. **PASS — newly authenticated WowForever pure-model/bootstrap scope, 9/9 positive executions. Matrix FINISHED; full handoff remains OPEN.**

Followed `/home/osso/AgentConfig/skills/verify/SKILL.md` as assigned verifier, without delegation. Evidence root: `/home/osso/.local/state/wow-ui-sim/verification/shared-query-profiles-current/20261011T181742Z`. Only WowForever independently authenticated here. Prior five profile seals/authentications retained from `/home/osso/.local/state/wow-ui-sim/verification/shared-query-profiles-current/partial-report.md` and `/home/osso/.local/state/wow-ui-sim/verification/shared-query-profiles-current/remaining-report.md`; no duplicate executable hashes or reruns. No builds, checks, runtime execution, operations, source edits, or cwd changes. Git reads used explicit `git -C /home/osso/Projects/wow/wow-ui-sim`.

## WowForever authentication

All profile receipts/streams below live at `/home/osso/.local/state/wow-ui-sim/verification/shared-query-profiles-current/20261011T181742Z/wowforever`.

- `compile-submission.json`: revision **c7d0c5ac326e3258ae90a16422df990c4db85d02**, started **2026-10-11T18:36:52.268584+00:00**. Saved Cargo argv uses offline/locked, no default features, `sound,gui,casc,client-wowforever`, integration test, no-run, JSON output. `compile-result.json`: **exit 0**, ended **2026-10-11T18:39:21.471600+00:00**. These commands were not executed by this verifier.
- Entire `compile.stdout` parsed: **741 Cargo JSON messages**, exactly **one integration executable artifact**. Executable source path, full feature array and test-profile metadata match `artifact.json`; final record exactly `{"reason":"build-finished","success":true}`.
- Features: `aura-containers, aura-instance-enumeration, aura-xml-widgets, base-spell-relationships, casc, client-wowforever, forbidden-animation-aspects, forbidden-aspects, gui, native-duration-formatting, numeric-rule-formatters, on-update-modes, player-cast-durations, rodio, sound, timed-signal-maps`. Exactly one client feature, `client-wowforever`; no retail epoch feature. Test profile: opt_level **1**, debuginfo **line-tables-only**, debug_assertions/overflow_checks/test **true**.
- **First independent sealed executable SHA-256, computed exactly once:** `/home/osso/.local/state/wow-ui-sim/verification/shared-query-profiles-current/20261011T181742Z/wowforever/integration-test`, **438981496 bytes**, **27aac848c0948b73dcc83374ce351fe07479da944adbf2dd7165a8dc0f844596**. Size/digest match seal. Mutable source `/home/osso/Projects/wow/wow-ui-sim/target/debug/deps/integration-9eb63ae53ac458d0` was neither hashed nor executed.
- `execution-results.json`: both argv target that exact sealed copy; **exit 0 / expected=observed=6** achievement, **exit 0 / expected=observed=3** Garrison pure; positive_expected_count true for both. Nine named stdout successes and final summaries independently agree. `outcome.json`: EXECUTED, inputs_unchanged true, before/after manifest identical.

Six existing iced-wgpu manifest lint-key deprecation warnings remain in saved compile stderr. Exit 0 is not warning-free proof. Test stderr fully read: achievement **66** startup lines, Garrison **33** startup lines; no error diagnostics.

## Eleven input hashes

Each SHA-256 independently matches submitted before, saved after, current file bytes at authentication, and exact committed bytes at submitted revision c7d0c5ac326e3258ae90a16422df990c4db85d02. **11/11 PASS.** This is the declared eleven-input boundary, not whole-tree/dependency/environment authentication. Later read-only HEAD observation: **5a4fdc6a20e022fc784a5b9e1047fa2d4946afb0**; no claim that saved executable was built at that later HEAD.

| Absolute input path | SHA-256 |
|---|---|
| `/home/osso/Projects/wow/wow-ui-sim/Cargo.lock` | `5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1` |
| `/home/osso/Projects/wow/wow-ui-sim/Cargo.toml` | `91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9` |
| `/home/osso/Projects/wow/wow-ui-sim/build.rs` | `cdebff6b6dec0e19e1124e734b2603432e2c47a7b956f9a4568ae412de182741` |
| `/home/osso/Projects/wow/wow-ui-sim/src/c_api/c_achievement_info_relations.rs` | `b45e91cff1c775fdf500b42be066b0034b0ca80948b1ba1bd44681a4f86895b3` |
| `/home/osso/Projects/wow/wow-ui-sim/src/c_api/c_garrison_damage_classes.rs` | `4319131c280f4cda7c6a75bd5b1fddf4704b5ba338fada122aa2361b0f6e1186` |
| `/home/osso/Projects/wow/wow-ui-sim/src/c_api/mod.rs` | `7c7a1b3b1b01063285bd6275f72a6f5f540a5a6c8cd1febf265e7521662368b3` |
| `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/globals/font_strings_collection/colors.rs` | `9aa469106b6b23fdbe6a0074108c7213dacc0578d4af2e57356b7661a62b4811` |
| `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/state.rs` | `0a9603dcb351ab356f41bb6f1fd8d6e6a7e3a114d790cea1236fe0157253e351` |
| `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/state/sim_state.rs` | `1454e312e6996cd9b7bb64ed4a37bdd85ff881e3a67bd5ef10d19306e11e9cd4` |
| `/home/osso/Projects/wow/wow-ui-sim/tests/c_achievement_info_probes/superceding.rs` | `665a09046d8a5e1d4508a0d9a61711378c2732df4f8de3bd569afb40423ca641` |
| `/home/osso/Projects/wow/wow-ui-sim/tests/garrison_damage_classes.rs` | `e103b6725fa8f731d8292c06772168bd2b78fcdd37395a7db3147451dbf86146` |

## Exact saved positive output

```text
running 6 tests
test c_achievement_info_probes::superceding::get_superceding_achievements_does_not_alias_unrepresented_numbers ... ok
test c_achievement_info_probes::superceding::get_superceding_achievements_observes_updates_and_removal ... ok
test c_achievement_info_probes::superceding::get_superceding_achievements_projects_distinct_supplied_sequences ... ok
test c_achievement_info_probes::superceding::get_superceding_achievements_requires_a_number ... ok
test c_achievement_info_probes::superceding::get_superceding_achievements_returns_isolated_tables ... ok
test c_achievement_info_probes::superceding::get_superceding_achievements_returns_one_empty_table_for_absent_relation ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 8347 filtered out; finished in 0.56s

running 3 tests
test garrison_damage_classes::adventures_combat_log_colors_bootstrap_with_mists_row_rgb_palette ... ok
test garrison_damage_classes::garrison_damage_classes_returns_fresh_tables_and_rows_without_mutation_leaks ... ok
test garrison_damage_classes::garrison_damage_classes_returns_one_table_with_en_us_pure_school_masks ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 8350 filtered out; finished in 0.30s
```

## Matrix and aggregate consistency

`/home/osso/.local/state/wow-ui-sim/verification/shared-query-profiles-current/20261011T181742Z/outcome.json`: **FINISHED**, results exactly equal sibling `results.json`. Six expected unique profiles present; all EXECUTED, inputs_unchanged true, both exits 0, expected/observed positive counts equal. WowForever matrix row exactly matches its execution receipts. Sibling `fmt.json`: **exit 0, stdout empty, stderr empty**. This authenticates saved fmt result only; no formatter/check was run here.

| Configuration | Achievement | Garrison pure | Pure executions | Authentication |
|---|---:|---:|---:|---|
| PTR | 8 | 3 | 11 | Prior retained |
| Wrath | 6 | 3 | 9 | Prior retained |
| Mists | 6 | 3 | 9 | Prior retained |
| Era | 6 | 3 | 9 | Prior retained |
| Anniversary | 6 | 3 | 9 | Prior retained |
| WowForever | 6 | 3 | 9 | Newly authenticated here |
| Earlier Retail | 8 | 3 | 11 | Earlier retained; not reauthenticated |

**56 new matrix executions = 11 + 9 + 9 + 9 + 9 + 9. Earlier Retail11 = eight relation cases plus three pure Garrison/bootstrap cases. Total 67 pure passing executions across seven configurations**, not 67 distinct tests and not a single contemporaneous seven-profile build. Prior five matrix profiles contribute47; newly sealed Forever contributes9. Earlier Retail relation proof is retained under `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/achievement-garrison-policy-2026-10-11/partial-green`; pure Garrison/bootstrap proof under `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/adventures-color-green-2026-10-11/runtime-report.md`. Consumer/prefork/startup/publication/reward cases are excluded from67; child output is not counted twice.

## Coverage and exclusions

| Contract | Newly verified proof |
|---|---|
| Supplied achievement sequences, updates/removal, absent relation, isolated tables, numeric input, non-aliasing identifiers | Six saved positive assertions; pure simulator-state boundary |
| enUS seven pure-school mask labels, fresh outer/row tables and mutation isolation | Two saved positive assertions; simulator catalog policy |
| Four combat-log globals, three numeric GetRGB channels, shared Mists-row palette | One saved positive bootstrap assertion |
| Native surface/parity/catalog/order/palette, cached vendor consumer, source/input acquisition/refresh, whole-profile startup/compatibility, broad/full suite | **No new credit** |
| Full handoff | **OPEN**; FINISHED matrix does not close excluded gates |

No native/cache/input-acquisition/full-handoff credit. No source/readability re-review. Prior failures and resource blocks remain historical evidence; earlier pending Forever snapshot is superseded only for this newly completed pure lane.

Authentication output, including sole independent executable hash: `/tmp/pi-tool-1c8ccd49be1d68ed.log`. Full saved streams were loaded/parsed despite display truncation. Subsequent bookkeeping encountered an unavailable cross-evaluation Python variable and a regex match-slicing error; both corrected by reading saved receipts only. Neither caused a build, runtime execution, or repeated executable/input hash.
