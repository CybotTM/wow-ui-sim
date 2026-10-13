# Calendar day Retail runtime acceptance — PASS

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/calendar-day-retail-green-20261013T214113790544Z`
Source association: `396f0c9a6ba705f9fd610d0a2c198ef910f67c4f`. Existing evidence only; no reruns, builds, delegation, tracked edits, source-archive reads, library/executable hashing, or re-authentication.

## Observed results

| Existing run | Result | Actual wait/process status | Filtered outside scope |
|---|---|---|---|
| runtime-calendar-day-green | 9/9 PASS; 0 failed/ignored/measured; 2.39s | launcher exit 0; --wait; code=exited status=0/SUCCESS | 10994 |
| runtime-calendar-index-preservation | 11/11 PASS; 0 failed/ignored/measured; 1.93s | launcher exit 0; --wait; code=exited status=0/SUCCESS | 10992 |
| runtime-normal-calendar-defaults | CALENDAR_DAY_EMPTY_DEFAULT_INDEX_PRESERVED + []; CLEAN; 0 unique/occurrence Lua errors | launcher exit 0; --wait; code=exited status=0/SUCCESS | not a test harness |

Full stdout/stderr, argv, admissions, result and launcher inspected for all three. Each admission allowed=true before launch. Captured byte lengths match results (1191/6404, 1260/7052, 46/14702). No runtime warnings, failed tests, ignored selected tests or hidden selected skips. Filtered counts explicitly exclude the rest of the suite; not full-suite acceptance. Day assertions check accumulated Lua errors after each phase without clearing; index tests assert public behavior but do not separately inspect the accumulator. Normal CLI reports full observed zero-error summary.

## Compilation, separate from runtime

All compile.stdout parsed: 741 JSON records = 665 compiler-artifact + 75 build-script-executed + 1 build-finished(success=true); zero compiler-message records. Terminal SubState=exited, Result=success, ExecMainStatus=0 (ActiveState=active is RemainAfterExit=yes, not an unfinished compile). Both integration and normal wow-sim artifact metadata are fresh=false, Retail client + retail-12-1-0 features; local Cargo/rustc 1.99.0, per saved toolchain.

Full compile.stderr read: six iced-wgpu-patched manifest key deprecations (large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity) plus the generated-six-warnings summary; Finished test profile in 3m17s. These are real compile warnings, not hidden, and not runtime warnings. No source compiler diagnostics recorded.

## Prior copy receipts — metadata reuse only

Both prior receipt/seal/auth-report PASS records read. Current stat(dev,ino,size,mtime_ns) matches each receipt, seal and auth-report: integration 66306/147460100/471349272/1791927871222455744; wow-sim 66306/147460103/408614192/1791927809161859746. Previously recorded SHA metadata agrees, but no hash read repeated. Prior two-copy authentication proves copy integrity only. Cargo features/profile/mtime and source revision declarations are association, not new source/archive/library/cache cryptographic authentication or native provenance.

## Precise 20-case coverage

Paths below are relative to epoch/source; every row is an observed selected runtime PASS, mapped to inspected test and production implementation.

| Test | Evidence | Coverage |
|---|---|---|
| `calendar_day_events_cached_itemref_link_tracks_title_replacement_and_deletion` | `tests/calendar_day_events/consumer.rs:46` | PASS — Entire cached ItemRef.lua executes unchanged; real GetCalendarEventLink gives exact Host raid link, updated title, old string retained, nil after deletion. |
| `calendar_day_events_cached_ongoing_broadcast_uses_separate_caller_numeric_index_fixture` | `tests/calendar_day_events/consumer.rs:76` | PASS — Real TOC-loaded CalendarUtil ongoing broadcast exact color/localized formatting, updated title; separate numeric caller ID 910001 resolves day position; opaque day/club string IDs unchanged and never coerced. |
| `calendar_day_events_all_required_selectors_reject_non_numeric_types` | `tests/calendar_day_events/queries.rs:121` | PASS — All 3 getter/2 count slots reject omitted/nil/string/bool/table/function inputs; later malformed slot validated even with earlier missing key. |
| `calendar_day_events_complete_snapshots_survive_replacement_and_erase` | `tests/calendar_day_events/queries.rs:5` | PASS — Exact 21-field DTO with icon, 20 without; both 6-field time tables; fresh outer/start/end identity; caller mutation isolated; host replacement/removal preserves prior snapshots and changes later reads. |
| `calendar_day_events_counts_exact_keys_and_dense_positions_without_default_rows` | `tests/calendar_day_events/queries.rs:57` | PASS — Empty default map; count 2 and positions 1/2/3; exact offset/day keys; previous-month row and explicit empty adjacent vector; no synthetic rows. |
| `calendar_day_events_nonrepresented_numbers_miss_without_aliasing_or_native_bounds_claim` | `tests/calendar_day_events/queries.rs:145` | PASS — NaN, infinities, huge +/- values, fractions, negative/zero/out-of-vector positions and width-overflow candidates miss without truncation/alias; valid row remains. |
| `calendar_day_events_same_key_is_isolated_between_environments` | `tests/calendar_day_events/queries.rs:97` | PASS — Two owned environments, same key; replacing second and deleting first cannot cross-contaminate count or DTO. |
| `calendar_day_events_tainted_secret_authenticates_every_argument_before_type_or_lookup` | `tests/calendar_day_events/security.rs:34` | PASS — VM-tainted closure denies secret in every slot, including after malformed earlier argument or missing key; exact untainted-caller error; taint/wrappers retained and unwrap denied; untainted caller restored. |
| `calendar_day_events_untainted_secret_selectors_preserve_original_wrappers` | `tests/calendar_day_events/security.rs:6` | PASS — Actual VM wrappers in each slot/all slots allowed for untainted caller; exact DTO/count and miss; wrappers/payloads and caller taint preserved. |
| `calendar_event_index_default_session_returns_three_exact_dtos` | `tests/calendar_event_index_info.rs:46` | PASS — Numeric demo IDs 910001/2/3 return exact 3-field DTOs at (0,15,1)/(0,15,2)/(1,4,1). |
| `calendar_event_index_each_mismatching_filter_returns_one_nil` | `tests/calendar_event_index_info.rs:77` | PASS — Each mismatched offset/day/both combination returns exactly one nil; matching fixture still works. |
| `calendar_event_index_invalid_numbers_cannot_alias_fixture_positions` | `tests/calendar_event_index_info.rs:214` | PASS — Fractional identities/filters, NaN/infinities/huge values cannot alias known fixtures; valid rows retained. |
| `calendar_event_index_observes_owned_state_updates_and_removal` | `tests/calendar_event_index_info.rs:132` | PASS — Host update/removal visible to subsequent query/filter; previous snapshot and other IDs retained. |
| `calendar_event_index_omitted_nil_and_matching_filters_return_dtos` | `tests/calendar_event_index_info.rs:57` | PASS — Omitted, nil, and matching optional offset/day filters all preserve each fixture. |
| `calendar_event_index_required_and_optional_numbers_reject_wrong_types` | `tests/calendar_event_index_info.rs:194` | PASS — Required identity and optional offset/day reject wrong types, including with unknown identity. |
| `calendar_event_index_restoration_preserves_namespace_and_unrelated_functions` | `tests/calendar_event_index_info.rs:173` | PASS — Post-cleanup restores getter while preserving C_Calendar identity and unrelated GetNumInvites sentinel. |
| `calendar_event_index_result_tables_are_independent_snapshots` | `tests/calendar_event_index_info.rs:111` | PASS — Fresh index tables; caller mutation cannot alter later results or other IDs. |
| `calendar_event_index_tainted_secret_inputs_are_denied_before_lookup` | `tests/calendar_event_index_info.rs:265` | PASS — Genuine tainted VM secrets denied before lookup/type validation even after malformed earlier slots; wrappers/taint retained, unwrap denied, untainted caller restored. |
| `calendar_event_index_unknown_identity_returns_one_nil` | `tests/calendar_event_index_info.rs:95` | PASS — Unknown numeric identities with omitted/nil/matching filters return exactly one nil; known identity preserved. |
| `calendar_event_index_untainted_secret_inputs_preserve_wrappers` | `tests/calendar_event_index_info.rs:241` | PASS — Genuine untainted VM secrets accepted in ID/optional filters; misses, wrappers/payloads, caller taint preserved. |

## Implementation/declaration/consumer grounding

- `src/lua_api/state/sim_state.rs:617-624`: independent numeric index map and host-owned (i32,u32)->Vec<CalendarDayEventRecord>; `state.rs:522` empty day defaults; `state.rs:976` seeds numeric index defaults from data/calendar/demo-event-indices.json. Numeric IDs 910001/2/3 preserved independently from opaque day eventID/clubID strings 9007199254740997/9007199254740995.
- `src/c_api/mod.rs:375` registers c_calendar; `c_calendar.rs:27-47` publishes existing index plus both producers. day_event_queries.rs authenticates all required slots before validating types or accessing state; numeric representation failures return count zero/one nil. Owned row cloned before Lua allocation. day_event_projection.rs publishes fresh rooted outer/start/end tables, 21 keys with optional icon and 20 without, time tables exactly six keys.
- `tests/calendar_day_events/fixtures.rs:70-138` recursively checks exact keys, values, types and result arity; 7.5 difficulty and 2.5 numSequenceDays retain numeric values. Positive producer assertion precedes consumer/security phases. `src/lua_api/env_init/mod.rs:73` installs native VM secretwrap/unwrap under enabled retail-12-1-0; globals/security/secret_values.rs:31-56 calls rilua table_security, not a Lua stand-in.
- Existing Retail cache CalendarDocumentation.lua:1005 declares 21 day fields, optional icon; TimeDocumentation.lua:6 declares six time fields. GetDayEvent declaration:544 includes MayReturnNothing and SecretInChatMessagingLockdown; :717 count permits secret arguments when untainted. Declarations ground shape, not native runtime semantics.
- Consumer `tests/calendar_day_events/consumer.rs:34-42` reads and executes entire cached Blizzard_UIPanels_Game/Mainline/ItemRef.lua, not an extracted/recreated function; cache ItemRef.lua:319 calls real producer and LinkUtil. No overrides passed to Blizzard closure. Real Blizzard_FrameXMLUtil TOC:19 loads CalendarUtil.lua; CalendarUtil.lua:8-33 calls numeric index getter, GetCalendarEventLink and real formatting. Inspected actual cached source. Day string identifiers are not converted to caller numeric ID; ongoing broadcast deliberately uses separate caller fixture 910001. ItemRef deletion is tested; ongoing broadcast replacement is tested, not deletion.

## Runtime conditions and proof boundary

CASC disabled (WOW_SIM_CASC=0), sound disabled (WOW_SIM_NO_SOUND=1). Normal CLI --no-addons/--no-saved-vars; logs explicitly show third-party addon and SavedVariables loading disabled. Blizzard cache remains used (291 Blizzard addons, 1569/1569 bytecode cache hits). Root/source/target read-only, network unshared, /tmp temporary. Existing WTF EditMode cache read-only source and loaded layout are explicitly observed; no-vars does not mean no WTF reads.

- Native WoW behavior/parity, native identifier representation, bounds, miss arity and any native installed-CASC execution are unproved. Cached declarations are not runtime/native evidence.
- Secret input authentication and wrapper preservation are proved in simulator VM. SecretInChatMessagingLockdown output secrecy is not exercised or claimed.
- Host row replacement/deletion and snapshot lifetime are proved; native calendar creation/edit/invite/event lifecycles are not proved.
- Real consumers run from existing external profile cache without overrides/extracted function recreation; cache bytes and external libraries were not authenticated. Whole ItemRef execution is evidenced by read_to_string plus env.exec and successful observed consumer tests.
- No full-parent/full-suite, GUI/render, third-party addon or SavedVariables acceptance. CLI reads existing WTF EditMode cache read-only even with --no-saved-vars; no installed-CASC proof.

No in-scope counterexample found. PASS applies only to this source-associated simulator producer/index/normal-startup gate; full-parent acceptance remains unproved.
