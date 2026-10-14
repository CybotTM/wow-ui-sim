# Terminal broad-suite artifact verification

**PASS — recorded run `4fc115b9357e39794423a7da2145960f27273e4e` only. Historical `983f02fbf46363d182828517632ec7b54426df36` remains FAIL. No native or parent-goal closure.**

Read `<private-skill-path>`; verifier performed artifact accounting directly, without delegation, tests, builds, or new suite launches. All 15,918 log lines processed (2,115,397 bytes).

## Primary counts

| Step | Passed | Failed | Skipped | Exit | Step seconds | Log evidence |
|---|---:|---:|---:|---:|---:|---|
| integration | 11,039 | 0 | 19 | 0 | 855.9 | lines 14, 11070 |
| prefork | 2,324 | 0 | not emitted | 0 | 92.1 | lines 11073, 13399 |
| lib | 1,999 | 0 | 0 | 0 | 91.4 | lines 13916, 15917 |

Nextest integration execution 755.806s (9 slow tests); lib execution 41.617s. Step durations include compilation. Primary totals are 15,362 passed, zero failed, plus 19 known skips; prefork emits no skip total. Do not interpret missing prefork skip metadata as proven zero.

Prefork additionally emits separate successful groups of 2, 1, 1 tests (lines 13400–13412), **four supplemental passes**, not part of primary 2,324:
- `chat_frame::test_chat_editbox_click_type_and_submit` — log line 13401
- `chat_frame::test_chat_editbox_text_color_after_activation` — log line 13402
- `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix` — log line 13406
- `blizzard_player_spells_loads::mainline_spellbook_keybind_opens_and_closes_without_runtime_errors` — log line 13410

No FAIL/SIGSEGV/SIGABRT/TIMEOUT result, error-prefix or panic diagnostics found. Current JSON failure/new_failure arrays empty. Integration skip IDs unavailable: log reports 19 skipped but prints no SKIP IDs. Lib reports zero skipped.

## Diagnostics

Six distinct `iced-wgpu-patched/Cargo.toml` lint-name deprecations repeat in each step: 18 individual warnings plus three “generated 6 warnings” records. Keys: `large-enum-variant`, `map-entry`, `match-wildcard-for-single-variants`, `redundant-closure-for-method-calls`, `trivially-copy-pass-by-ref`, `type-complexity`; replacements use underscores. Evidence lines 2–8, 13413–13419, 13905–13911.

15 SLOW threshold records correspond to 9 eventually passing integration IDs; exact records included in proof.json. Prefork emits 480 timing/trace records: 240 Workarounds, 136 Startup, 104 EditMode. Exact trace records preserved in proof.json, including deliberate “Skipping EDIT_MODE_LAYOUTS_UPDATED”; this is startup tracing, not a skipped test. No warning-free claim.

## Argv and scope

- **integration**: `cargo nextest run --test integration --no-fail-fast --test-threads 16 --offline --locked`
- **prefork**: `cargo test --test prefork_full_ui --offline --locked`
- **lib**: `cargo nextest run --lib --no-fail-fast --test-threads 16 --offline --locked`

JSON ref and SHA both `4fc115b9357e39794423a7da2145960f27273e4e`. Started 2026-10-14T11:57:54-0500; finished 2026-10-14T12:15:13-0500 (17m19s wall clock). Checkout path shown in compile records and runner source: `<private-checkout>`. Runner forces checkout and cleans while preserving target. Supplied source runs steps sequentially under a lock; captures stdout then stderr, so prefork diagnostic placement is not chronological.

Recorded nextest concurrency 16; timeout-fixture-capacity override null for all steps. Source config sets CARGO_BUILD_JOBS=12 and CARGO_TERM_COLOR=never while inheriting environment. Submit source requests agents.slice, CPUQuota=1200%, MemoryHigh=16G, MemoryMax=16G. **These are source defaults, not observed resource enforcement or consumption.** Actual unit, host, cgroup limits, peak RSS, toolchain and complete environment not recorded. No explicit feature/profile flags in argv; cannot establish native/profile coverage. Runner source at execution, clean-tree identity and binary-to-revision provenance are not independently attested by these artifacts. No binaries hashed.

## Historical comparison — preserve FAIL

Historical JSON `983f02fbf46363d182828517632ec7b54426df36`: 2026-10-14T06:34:42-0500 to 2026-10-14T07:18:04-0500. Same recorded argv and null capacity override. Integration exit 100 (2023.2s), prefork exit 1 (257.8s), lib exit 100 (320.7s). Historical JSON contains no primary counts/skips; no counts invented.

### integration
- `encounter_timeline_view::encounter_tracks_real_blizzard_track_layout_and_view` — historical FAIL; current PASS at log line(s) 5757
- `zone_ability::zone_ability_frame_displays_seeded_active_ability` — historical FAIL; current PASS at log line(s) 11067
### prefork
- `patch_10_0_0_publication_sweep::patch_10_0_0_publication_sweep` — historical FAIL; current PASS at log line(s) 13213
- `patch_11_1_5_publication_sweep::patch_11_1_5_publication_sweep` — historical FAIL; current PASS at log line(s) 13239
- `patch_5_1_0_publication_sweep::patch_5_1_0_publication_sweep` — historical FAIL; current PASS at log line(s) 13277
- `patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep` — historical FAIL; current PASS at log line(s) 13333
- `patch_9_0_1_publication_sweep::patch_9_0_1_publication_sweep` — historical FAIL; current PASS at log line(s) 13349
- `patch_9_1_0_publication_sweep::patch_9_1_0_publication_sweep` — historical FAIL; current PASS at log line(s) 13351
### lib
- `loader::tests::layout_alpha::test_set_alpha_clamps_above_1` — historical FAIL; current no matching PASS ID found
- `loader::tests::layout_alpha::test_set_alpha_clamps_below_0` — historical FAIL; current no matching PASS ID found

Historical lib clamping IDs are absent, not demonstrated fixed by exact-ID matches. Related current IDs `loader::tests::layout_alpha::test_set_alpha_above_1_obeys_profile_policy` and `loader::tests::layout_alpha::test_set_alpha_below_0_obeys_profile_policy` pass at lines 14513–14514; replacement/equivalence is not proven by these artifacts. Eight historical failing IDs have exact current PASS matches; two do not.

Current passes do not rewrite the historical outcome or independently explain its failures. Empty new_failures is baseline-relative subtraction, not standalone success evidence. PASS rests on three zero exits, terminal summaries and independently counted pass records.

## Evidence and boundaries

Current JSON/log and historical JSON: `<private-result-store>/` with exact SHA filenames. Runner: `<private-project-root>/wow-ui-sim/tools/full_suite.py`. `proof.json` retains original JSONs, exact summary/diagnostic records, line references and comparison IDs.

Artifact-only PASS. Not current HEAD verification, native compatibility proof, all-profile proof, parent-goal completion, or a new test run. Input artifacts unchanged. Outputs directory 0700; report.md and proof.json 0600.
