# Independent bounded Calendar GREEN verification

**OVERALL: PASS — bounded producer/spec only.** Code `fa66dd878420fec0f67aab473cc91eebadf1c090`; later `10c520fd83a830dccf3e48ada4d690457efd9cb9` changes five docs only; `296b9ac77516b36761b8492e6b1a8354d3643917` changes current publication-gap fixture/spec only. Source hashes and exact committed-source matches: `green-source-attribution.json`. Current spec acceptance checkboxes still pending; no repo edits made.

## Exact command evidence (saved execution; no reruns)

- Compile **exit0**, build-finished success; integration and normal Cargo records both **fresh=false** (new artifacts).
- Integration **exit0: 11 passed / 0 failed / 0 ignored / 0 measured / 10,736 filtered; 1.38s**. Eleven unique per-test status lines, not recap duplicates. Earlier `outcome.json` compile-only zero-case receipt is superseded by runtime receipt/full logs.
- Normal `wow-sim --no-addons --no-saved-vars --exec-lua <rawget/demo assertions> lua-errors`: **exit0, stdout `[]`, 0 unique Lua errors / 0 occurrences**. 291 Blizzard addons loaded. Tests raw registered getter plus `(0,15,1)`, offset mismatch nil and ID910003 matching-filter position1 after startup. This is not a loaded repetition of all eleven cases or ID910002.
- Saved `final-fmt.json`: **cargo fmt --check exit0**. `producer-readability.json`: manual scoped audit, **0 violations**. Final default `cargo check --offline --locked -j12` receipt **exit0**, source hashes unchanged, finished34.65s at docs-only10c. Compile/check retain **six existing iced_wgpu manifest deprecation warnings**; no claim of warning-free project. Normal stderr audio-device absence is nonfatal, not a Calendar failure.

## First NEW sealed authentication (one read each)

| Artifact | Bytes | SHA256 | Metadata / compile record |
|---|---:|---|---|
| integration-sealed | 467715784 | fa3b1e1fd1a59fc2dd97e1941f041a2fd20ece43b98bb62cc84eecc2dc6c0fa4 | match / match |
| wow-sim-sealed | 407464472 | 94f428fd8ea90d18d744b3081503f9373abc53c7599274774a61ad71a8342b41 | match / match |

No old binary/library reads. `green-independent-auth.json` retains receipts. Compile receipt reports source_unchanged; not an independently reconstructed historical working tree.

## All eleven logical cases reached

| Case | Result and reached behavior |
|---|---|
| default_session_returns_three_exact_dtos | PASS — Three seeded identities return exactly one table with exactly offsetMonths/monthDay/eventIndex and expected values. |
| each_mismatching_filter_returns_one_nil | PASS — All 18 mismatching-filter combinations yield one nil; positive control returns exact DTO. |
| invalid_numbers_cannot_alias_fixture_positions | PASS — Fractions/nonfinite/extreme identities and filters cannot alias live records; both final positive controls execute. |
| observes_owned_state_updates_and_removal | PASS — Owned record changes to (2,20,4) affect subsequent query, old filter misses, saved snapshot remains (0,15,1); removal yields nil without affecting other identities. |
| omitted_nil_and_matching_filters_return_dtos | PASS — All 21 omitted/nil/matching filter combinations across three fixtures return exact DTOs. |
| required_and_optional_numbers_reject_wrong_types | PASS — Missing/nil required ID and wrong-type ID/optional filters raise contextual errors, including unknown-ID validation; final valid control executes. |
| restoration_preserves_namespace_and_unrelated_functions | PASS — After getter clearing and public restoration: real DTO, same namespace identity, same GetNumInvites sentinel identity and sentinel return7. |
| result_tables_are_independent_snapshots | PASS — Fresh result-table identities, caller mutation isolation, third snapshot freshness and other records preserved; mutated filter misses. |
| tainted_secret_inputs_are_denied_before_lookup | PASS — CalendarIndexProbe-tainted addon: plain controls succeed; eight secret-bearing argument combinations denied with untainted-caller message before lookup/type validation; wrappers remain secret, declassification denied, taint preserved and untainted caller restored. |
| unknown_identity_returns_one_nil | PASS — 15 unknown identity/filter calls return exactly one nil; configured positive control executes. |
| untainted_secret_inputs_preserve_wrappers | PASS — Untainted wrapped required/optional arguments return DTOs; wrapped unknown/mismatch return nil; original wrappers and numeric unwrap values preserved; caller remains untainted. |

Passing Rust `.expect` boundaries and complete functions establish every unconditional/loop assertion in these concrete tests reached, including assertions after tainted addon return. Nine RED cases plus two owned-state/restoration additions, not eleven historical RED failures.

## `[]` assertion proof boundary

`src/lua_errors.rs:36` fires startup before probe. Lines68–70 route a probe error to canonical error handling, rather than swallowing an assertion. `src/lua_api/script_helpers.rs:531–537` records original error **before** invoking custom handler; lines587–602 append raw error/attribution and normalized count. `src/lua_errors.rs:264–280` includes counted errors; lines81–86 print JSON and return errors.is_empty. `src/bin/wow_sim/main.rs:549–558` exits1 if errors exist. `WowLuaEnv` installs app_data; exec returns before sink and no state borrow survives this non-reentrant probe. Sink has missing-app-data/borrow-failure branches, so `[]` is not a universal guarantee for arbitrary reentrant scenarios; neither applies to this supplied probe. The supplied rawget/demo assertions therefore have valid exit0/[] evidence.

## Mode B

[EXIST] **PASS** — dataset5 lines; producer101; c_api/mod389; state1052; sim_state704; temporary defaults177; tests297; spec37. Detailed file inventory in proof.json.

[SUBSTANTIVE] **PASS** — `CalendarEventIndexInfo` has offset_months/month_day/event_index i32 fields; owned HashMap keyed by numeric fixture ID. Committed dataset seeds three positions. Real query authenticates all arguments first (retail-12-1-0 active), validates numeric types, rejects ID aliasing via finite/integer/i32 checks, matches optional filters against copied owned record, then allocates fresh table with exactly offsetMonths/monthDay/eventIndex. Miss pushes one nil. No selected/open-event hidden input; no synthetic native claim.

[WIRED] **PASS** — c_api/mod.rs:55/:344 declares/registers; globals/utility_system_spell/mod.rs:646 initializes utility tables; environment_cleanup_restore.rs:20 registers again. SimState::default calls seed_default_game_state (state.rs:941–951); WowLuaEnv::new uses default (env.rs:75–79). Frame-independent owned field at sim_state.rs:582. table_builder.rs:47–81 raw-installs Rust closure. Normal rawget excludes fabricated lookup. Eleven observed integration statuses prove discovery via tests/integration.rs harness macro. Restoration test proves same namespace and unrelated sentinel preserved while real getter restored.

[ANTI-PATTERN] **PASS** — 172 added Rust/test lines scanned: TODO0/FIXME0/HACK0/XXX0; no added empty catches/commented-out-code blocks. Existing catalog/invite inert defaults are explicitly outside this getter and not new implementation placeholders.

[SPEC POLICY] **PASS** — numeric IDs and nil-on-miss are explicitly fixture policy; filters and normal seeding are user-selected policy. Current retained generated-contract receipt declares AllowedWhenUntainted and DTO fields but leaves native behavior unknown. Security tests cover actual wrapper/taint/authentication paths, not native parity. Scope excludes catalog/editor/invitations/dates/native IDs/historical8.1 fidelity/vendor edits.

## RED reused; remaining gates

Reused independently verified `red-compile-20261012T014953198487Z/report.md` and `proof.json`: **exit101, 0PASS/9FAIL**. Eight first DTO-positive failures and one input-validation failure. No RED reauthentication or execution; never promote its unreached security branches to proof. Two new GREEN cases cover owned mutation/removal and restoration.

**Pending:** relevant publication sweep plus its separate independent proof; parent final acceptance/full handoff. Publication worker not polled or inspected here. Only current event-index gap row removed by296b; historical accounting preserved. This producer verification does not establish whole Calendar, native parity, all-profile or broad-suite completion.

No tests/builds/checks/reruns/operations/polling/delegation/repo changes. Private proof/report writes only. Initial wrong-cwd git attempts and ambiguous10c lookup yielded no evidence; explicit-cwd/full-revision distinct inspections resolved them without rerunning proof commands. Machine-readable evidence: `proof.json`.
