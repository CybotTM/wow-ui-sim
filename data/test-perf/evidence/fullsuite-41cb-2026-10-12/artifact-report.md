# suite41cb artifact verification

Evidence verification: **PASS**. Actual suite: **FAIL** (library exit 100, two failures).
Scope: existing completed artifacts only. No replay, polling, build/check, executable/library hashes, operations, delegation, repository edits or publication performed.

## Binding
Ref/SHA: `41cbf5005d4fec8dba98538e425e6d8fd7f0f5c3`; JSON ref and SHA match source basenames. Log section command/exits match JSON exactly. This is recorded-artifact binding, not independently authenticated checkout/binary provenance.
Start: `2026-10-12T18:26:49-0500`. Finish: `2026-10-12T18:59:39-0500`. Wall duration 1970 seconds; recorded step durations total 1969.5 seconds. The 0.5-second difference does not prove step boundary times.
Per-step start/finish timestamps are absent in both artifacts. Exact wall-clock step boundaries cannot be established; no interpolated timestamps used.

| Step | Passed | Failed | Skipped | Exit | Recorded seconds |
|---|---:|---:|---:|---:|---:|
| integration | 10796 | 0 | 19 | 0 | 1625.6 |
| prefork primary | 2324 | 0 | 0 | shared 0 | shared 195.4 |
| prefork aux chat_frame | 2 | 0 | 0 | shared 0 | shared 195.4 |
| prefork aux spell_casting | 1 | 0 | 0 | shared 0 | shared 195.4 |
| prefork aux blizzard_player_spells_loads | 1 | 0 | 0 | shared 0 | shared 195.4 |
| prefork subtotal | 2328 | 0 | 0 | 0 | 195.4 |
| lib | 1999 | 2 | 0 | 100 | 148.5 |
| total (subtotal counted once) | 15123 | 2 | 19 | SUITE FAIL | 1969.5 |

15125 executed test invocations, 15144 including skips. Totals count invocations across steps, not globally unique test names. Auxiliary groups are part of the same prefork command; no separate exit or elapsed values recorded. Prefork zero skips derived from all announced tests having ok terminals; custom summaries omit an explicit skipped field.

## Exact recorded commands

- `integration`: `cargo nextest run --test integration --no-fail-fast --test-threads 16 --offline --locked`; exit `0`, seconds `1625.6`; start/finish **not recorded**; log lines 1–10860.

- `prefork`: `cargo test --test prefork_full_ui --offline --locked`; exit `0`, seconds `195.4`; start/finish **not recorded**; log lines 10861–13692.

- `lib`: `cargo nextest run --lib --no-fail-fast --test-threads 16 --offline --locked`; exit `100`, seconds `148.5`; start/finish **not recorded**; log lines 13693–15770.

`WOW_SIM_TIMEOUT_FIXTURE_CAPACITY` is JSON null for every step; null alone does not establish the full process environment.

## Full-output accounting
All 15770 lines / 2087378 bytes decoded strictly and parsed. Integration: 10796 unique terminal PASS rows with all ordinals 1–10796; summary reports 19 skips (skip names not recorded). Lib: 2001 unique terminal results, 1999 PASS and two FAIL; repeated two FAIL summary rows excluded. Prefork: all 2328 ok terminal records reconcile four announced groups and summaries. Nested per-failure 1-test summaries are not extra test invocations.

47 SLOW notices retained in proof and relevant-output; summary reports 27 slow tests, not 47 distinct tests. 21 warning lines retained verbatim: six manifest lint deprecations repeated in each of three steps (18 occurrences), plus three “generated 6 warnings” summaries. No warning suppressed. Runtime Startup/Workarounds/EditMode lines retained in relevant-output; “Skipping EDIT_MODE_LAYOUTS_UPDATED” is runtime event behavior, not a test skip.

## Exact failing assertions/panics

`lua_api::workarounds_editmode::tests::setup_layout_info::basic::setup_layout_info_preserves_distinct_saved_action_bar_profile_settings`

Log lines 15450–15478; panic: `thread 'lua_api::workarounds_editmode::tests::setup_layout_info::basic::setup_layout_info_preserves_distinct_saved_action_bar_profile_settings' (3131425) panicked at src/lua_api/workarounds_editmode_tests/setup_layout_info/basic.rs:603:10:`

```text
read distinct saved action bar profile settings: Lua(Runtime(RuntimeError { message: "(string):17: attempt to index field '?' (a nil value)", level: 0, traceback: [] }))
```

`lua_api::workarounds_editmode::tests::setup_layout_info::basic::setup_layout_info_remaps_active_saved_layout_after_prepending_presets`

Log lines 15493–15521; panic: `thread 'lua_api::workarounds_editmode::tests::setup_layout_info::basic::setup_layout_info_remaps_active_saved_layout_after_prepending_presets' (3131428) panicked at src/lua_api/workarounds_editmode_tests/setup_layout_info/basic.rs:387:10:`

```text
read remapped active layout: Lua(Runtime(RuntimeError { message: "(string):8: attempt to index local 'party' (a nil value)", level: 0, traceback: [] }))
```

These are recorded expect/read panics caused by Lua nil-index runtime errors, not recorded assert_eq expected/actual comparisons. No root-cause or fix verification claimed. JSON failures and new_failures lists both match these exact two names; “new” is runner metadata, not independently verified history.

## Applicability and privacy
41cb predates 1887 Vertex tests/spec (user-supplied chronology). This historical suite provides no acceptance proof for those later tests/spec or current HEAD. No source chronology inspection or current-HEAD full acceptance claim.

Raw source copies and proof remain private; timestamp directory 0700, all files 0600, exclusive creation. Selected summary/failure excerpts reviewed for private paths, user/account values and credentials; four private checkout-path lines sanitized to [PRIVATE_PATH]. Relative source failure references retained. Provenance records source log hash, inclusive source line ranges, raw data-slice hashes and sanitized selected hashes. Data streams only: no executable/library hash.

Files: source.json/source.log (private exact data sources); proof.json (counts, all terminal records, warnings, slow notices); relevant-output.sanitized.log (all non-pass output); excerpts.sanitized.log (selected summaries/warnings/failures); provenance.json (data stream linkage/privacy); report.md.
