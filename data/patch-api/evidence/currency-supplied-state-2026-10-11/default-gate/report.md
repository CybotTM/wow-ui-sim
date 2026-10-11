# Independent bounded default currency verification

**PASS — bounded default Retail slice at HEAD `1dcc59c0788d0e09ee3d02516123df965ba8dc63`.** Not whole-branch readiness.

## Artifact authentication

| Evidence | Observed result |
|---|---|
| Initial RED `20261011T073545Z`, revision `085f7d7999f44870d87001c9dc493f5586c4e048` | Compile exit 101; Cargo build-finished false; two E0277 unsupported `u32` decoder errors; zero cases executed. Preserved, not counted as behavioral RED. |
| Actual RED `20261011T073849Z`, revision `f4a6fb2e47278ea30cbdce8c7edea71ab0091953` | Compile exit 0/build-finished true; integration test profile with default/client-retail features; **16 executed: 12 PASS / 4 FAIL**, exit 101; 0 ignored, 0 measured, 10706 filtered. |
| GREEN `20261011T074304Z`, revision current HEAD | Compile exit 0/build-finished true; same target/features; **16 executed: 16 PASS / 0 FAIL**, exit 0; 0 ignored, 0 measured, 10706 filtered. |
| Format/check | Saved `cargo fmt --all -- --check` exit 0, empty stdout/stderr; saved default `cargo check --offline --locked --jobs 12` exit 0, Finished dev profile. |
| Cached stock startup | Sealed normal `wow-sim`, compiler kind bin/profile.test false, default/client-retail/sound/gui/casc features; stdout `[]`, exit 0; stderr CLEAN, 0 unique / 0 occurrence Lua errors, 0 unattributed. |

All full saved streams read privately; compiler JSON independently parsed rather than trusting narrative worker reports. Compiler artifact paths/features/profiles match seals. Normal wow-sim seal's compiler record exactly matches compile.stdout. RED and GREEN integration binaries and normal GREEN binary rehashed once each: all three digest/size matches. RED seal SHA256 `9caf5587e1e5f293fc849a5c6c2ab620491f4eedc6a6f4187ff8f44af4a7d9b8`; GREEN test `33d343733564d85a2c1bc7ac9a688d98661f7bc498475832c362ab3019c2a595`; GREEN normal `4856833e442267186e9dbfd42c5eeb017a3e81cb421816d9da7f72e00972b60c`.

Seven RED scoped inputs match before/after and revision blobs. Eight GREEN inputs match before/after/current working files/HEAD blobs. Same corrected regression tests on actual RED and GREEN. RED producer differs from GREEN; git ancestor check confirms RED revision precedes producer commit `adf26ff83`. Only producer/spec/Mists test differ from actual RED to HEAD. Build config, static currency catalog and Mists adapter hashes unchanged. This authenticates the recorded scoped chain, not all transitive build inputs or a cryptographically signed execution environment. Exits are preserved execution receipts, corroborated by successful Cargo records and harness/startup summaries; no reruns performed.

## Named behavior coverage

| Test | Actual RED | Default GREEN |
|---|---|---|
| get_backpack_currency_info_reflects_successive_existing_currency_updates | FAIL | PASS |
| get_backpack_currency_info_returns_nothing_when_existing_currency_row_is_removed | FAIL | PASS |
| get_basic_currency_info_returns_display_info_for_saved_instances_hover | PASS | PASS |
| get_currency_container_info_returns_display_info_with_passed_quantity | PASS | PASS |
| get_currency_container_info_returns_nothing_for_unknown_id | PASS | PASS |
| get_currency_info_covers_saved_instances_currency_ids | PASS | PASS |
| get_currency_info_exposes_all_retail_fields | PASS | PASS |
| get_currency_info_from_link_parses_currency_hyperlink | PASS | PASS |
| get_currency_info_from_link_returns_nothing_for_non_currency_link | PASS | PASS |
| get_currency_info_from_link_returns_nothing_for_unknown_currency_id | PASS | PASS |
| get_currency_info_matches_new_saved_instances_currency_metadata | PASS | PASS |
| get_currency_info_reflects_sim_state_mutation | PASS | PASS |
| get_currency_info_returns_nothing_for_unknown_id | PASS | PASS |
| get_currency_info_returns_seeded_valorstones_row | PASS | PASS |
| get_currency_list_info_reflects_successive_existing_currency_updates | FAIL | PASS |
| get_currency_list_info_returns_nil_when_existing_currency_row_is_removed | FAIL | PASS |

Four failing RED tests panic at `tests/currency_info_probes.rs:313`, `:364`, `:396`, `:417` with Lua assertion failures. These are executed failures at the API boundary, not source-substring checks; logs do not identify individual Lua assertion lines or exact old return values.

| Contract | Proof and boundary |
|---|---|
| List display | Successive 2245 mutations: name, quantity 73 then 0, cap 250 then 0, icon 134400 then 134401, quality 4 then 0; concrete tuple comparison and equality with GetCurrencyInfo. Exactly 9 keys, static ID/status flags. |
| Backpack display | Same successive supplied row changes; concrete name/quantity/icon comparison and equality with GetCurrencyInfo; exactly 4 keys, no cap/quality/status/currencyID fields. |
| Missing supplied row | Remove 2245 from existing map: list returns exactly one nil; backpack exactly zero returns; direct detail has zero returns. No static display resurrection. |
| Preservation | After each update/removal: list size 19, exact static header 1, unchanged list neighbor 6 and watched neighbor 2/ID 3089, watched 3/ID 1792, invalid list indices 0/20 and backpack 0/4. This is sampled preservation, not exhaustive catalog/order coverage. |
| Existing controls | Twelve controls cover seeded Valorstones, SavedInstances IDs and pinned metadata, Retail fields, direct supplied-state mutation, basic display/container info, unknown IDs, currency links and invalid/non-currency links. See named results above. |

## Source inspection

[EXIST] PASS — substantive files present: `src/c_api/item_spell/c_currency.rs` 519 lines, `tests/currency_info_probes.rs` 492 lines, `tests/mists_currency_list.rs` 351 lines; tracked spec present/read.

[SUBSTANTIVE] PASS — `c_currency_get_list_info` selects static entry first, resolves header display from that static entry or ordinary display via existing map, writes five display fields, static ID and three static status flags. `read_currency_list_display_info` has no ordinary-row fallback. Backpack selects existing static watched iterator, resolves same map, returns no data when absent and writes only its original four fields. No setter, events, catalog/default/API-shape/registration/vendor changes in producer diff.

[WIRED] PASS — existing namespace dispatch names map to modified producers in the same method table. Registration is called externally at `src/c_api/item_spell/mod.rs:33`. Existing state seeding is wired at `src/lua_api/state.rs:491`; unchanged static selection at `src/lua_api/globals/currency_data.rs:95-109`. Helpers are called by the existing producers, not orphaned. Named sealed integration execution establishes tests are in the actual default integration artifact.

[ANTI-PATTERN] PASS — zero added TODO/FIXME/HACK/XXX markers or `#[allow(...)]` suppressions in scoped changed Rust. No placeholder, swallowed-error or commented-out implementation found by manual inspection.

## Manual Rust readability audit

**0 findings**, current changed Rust only: producer and four regressions/shared preservation helper, plus new Mists preservation probe. No metrics command used. Modified/new producer function bodies 27/14/9/13/22 lines; no excessive cognitive/cyclomatic complexity, nesting, function length, hidden effects, state accumulation, substantive duplicate non-test body, suppressions, opaque naming, bumpy road or primitive parameter overload found. Test mutation scopes explicitly release state borrows before Lua calls; fixtures are concrete data. Existing API function names follow the registered contract, not a newly introduced generic getter abstraction.

## Warnings and acceptance limits

Six unique `iced-wgpu-patched/Cargo.toml` deprecated Clippy manifest names: `large-enum-variant`, `map-entry`, `match-wildcard-for-single-variants`, `redundant-closure-for-method-calls`, `trivially-copy-pass-by-ref`, `type-complexity`. Six deprecation lines each in initial RED compile, actual RED compile, GREEN compile and GREEN check (24 occurrences total); summary lines are not additional warnings. Actual RED/GREEN compiler JSON: 0 compiler warnings, 0 compiler errors. Initial failed compile: 2 compiler errors plus one failure-note. Format stream empty. GREEN integration stderr contains startup timings, no warning/error diagnostics; RED additionally contains four expected panic blocks.

Stock startup has **8 ALSA diagnostic lines + 1 no-audio-device line**; this is not an audio success claim. Audio-enabled normal default binary executed with `WOW_SIM_CASC=0`, `--no-addons`, `--no-saved-vars`; warm cached Blizzard UI/bytecode (1569/1569 hits), not live CASC, cold cache, third-party addons, rendering or native behavior.

Source inspection proves unchanged static membership/status sourcing. Executed default regressions mutate display values/remove the row, not conflicting supplied status flags. The new Mists probe specifically sets supplied `is_show_in_backpack=false` while asserting preserved static watched membership and exact legacy tuple positions/arity, but **its execution belongs to the separate queued Mists gate and is not accepted here**.

Spec's Known gaps still labels GREEN pending at this HEAD; evidence above closes only the bounded default gate in this report. No tracked spec/code edits made. No Mists/native/full-profile/full-suite/full-handoff/whole-branch ready claim. No builds/tests/checks/full suites/ops/source acquisition/delegation or Bash/rg/fd invoked; report files are the only writes.

Artifact roots:
- `/home/osso/.local/state/wow-ui-sim/verification/currency-projection-red-current/20261011T073849Z`
- `/home/osso/.local/state/wow-ui-sim/verification/currency-projection-green-current/20261011T074304Z`

Detailed receipts, source hashes, seals, full compiler records, exact test names, diagnostics and limits: sibling `report.json`.
