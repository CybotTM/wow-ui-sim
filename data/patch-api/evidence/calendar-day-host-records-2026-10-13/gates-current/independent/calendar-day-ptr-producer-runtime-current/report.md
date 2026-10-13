# Calendar day PTR producer: bounded runtime gate — PASS

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/calendar-day-ptr-green-20261013T214624254173Z`

Source metadata: docs/evidence HEAD `eb453a078ffc1ba3b6c76de35ba4d27300566ef4`; producer `396f0c9a6ba705f9fd610d0a2c198ef910f67c4f`. Independent read-only saved-evidence gate; no reruns/builds/delegation/tracked edits.

## Runtime evidence

| Saved retry1 | Result | Exact evidence |
|---|---|---|
| calendar-day-green | PASS | 9 passed; 0 failed/ignored/measured; 10171 filtered out; exit 0 |
| calendar-index-preservation | PASS | 11 passed; 0 failed/ignored/measured; 10169 filtered out; exit 0 |
| normal-calendar-defaults | PASS | `CALENDAR_DAY_EMPTY_DEFAULT_INDEX_PRESERVED`, `[]`, exit 0; `Status: CLEAN`; `Lua errors: 0 unique, 0 occurrence(s)` |

All six files in each retry1 directory read in full: admission.json, argv.json, launcher.txt, result.json, stdout, stderr. Admissions allowed; each launcher says `Finished with result: success` and `code=exited, status=0/SUCCESS`. Full verbatim files embedded in proof.json. Normal startup is not a 21st Rust test.

## Initial attempts: separate admission blocks

Each original non-retry directory contains only blocked-before-admission.json: `BLOCKED_ZERO_LAUNCH`. Receipt fields dev/inode/size/mtime_ns did not match generic harness st_* attribute names. Saved boundary: before admission, argv, or launch. These are neither RED nor behavioral failures. Separate records preserved. Explicit epoch runners map those four actual once-auth fields to st_dev/st_ino/st_size/st_mtime_ns and reuse existing receipts; no hash/authentication stream is performed by the runners.

## Stat-only and compilation evidence

Current stats match receipt stat_before, stat_after, and normalized seal for both copies:

| Copy | dev | inode | size | mtime_ns |
|---|---:|---:|---:|---:|
| integration | 66306 | 147491566 | 470927408 | 1791928176621260906 |
| wow-sim | 66306 | 147491569 | 415674312 | 1791928120053808561 |

No executable bytes read or new hashes computed. Prior receipts record PASS and exactly one authentication hash read each; this gate only consumes that metadata. Runner source compares current stat to receipt stat_after and receipt path to seal path; this independent gate additionally compares both receipt snapshots to the seal stat.

compile.stdout parsed fully: 741 records (665 compiler-artifact, 75 build-script-executed, 1 build-finished success=true), zero compiler-message records. Terminal: SubState=exited, Result=success, ExecMainStatus=0. Full compile.stderr read: six iced-wgpu-patched manifest deprecations, no source diagnostics; not warning-free. Normal/integration Cargo artifacts carry client-ptr and retail-12-1-5 (including retail-12-1-0 secrets); metadata association is not source-byte/native-library provenance.

## Exact 20-case coverage

Paths below refer to epoch/source, not mutable current checkout. Every row is matched to its saved `test … ok` line.

| Source | Test | Proven behavior (bounded) |
|---|---|---|
| `tests/calendar_day_events/consumer.rs:46` | `calendar_day_events_cached_itemref_link_tracks_title_replacement_and_deletion` | Whole cached ItemRef.lua loaded, no extracted function; exact public link before/after title replacement and nil after erase. |
| `tests/calendar_day_events/consumer.rs:76` | `calendar_day_events_cached_ongoing_broadcast_uses_separate_caller_numeric_index_fixture` | Real cached CalendarUtil/LinkUtil/GameTime/colors/constants closure; exact ongoing broadcast after title replacement; caller numeric 910001 separate from opaque string day IDs. |
| `tests/calendar_day_events/queries.rs:121` | `calendar_day_events_all_required_selectors_reject_non_numeric_types` | All required slots; missing/nil/string/boolean/table/function rejection, including absent-key calls. |
| `tests/calendar_day_events/queries.rs:5` | `calendar_day_events_complete_snapshots_survive_replacement_and_erase` | Exact 21/20 outer fields; both six-field times; primitive types/values, false/fractional values; caller mutation, fresh independent tables, host replacement/erase and retained snapshots. |
| `tests/calendar_day_events/queries.rs:57` | `calendar_day_events_counts_exact_keys_and_dense_positions_without_default_rows` | Empty default map; positive count 2; dense positions 1/2; absent/empty/adjacent keys; negative month offset. |
| `tests/calendar_day_events/queries.rs:145` | `calendar_day_events_nonrepresented_numbers_miss_without_aliasing_or_native_bounds_claim` | NaN/infinities/huge/fractional/negative/zero/out-of-range selectors do not alias; valid row remains accessible. Model policy only. |
| `tests/calendar_day_events/queries.rs:97` | `calendar_day_events_same_key_is_isolated_between_environments` | Two simultaneous environments; replace second, erase first, assert independent count/DTO. |
| `tests/calendar_day_events/security.rs:34` | `calendar_day_events_tainted_secret_authenticates_every_argument_before_type_or_lookup` | Object-tainted addon; each secret slot denied before earlier malformed/absent selectors; untainted-caller error; wrappers and taint preserved; no declassification; untainted return works. |
| `tests/calendar_day_events/security.rs:6` | `calendar_day_events_untainted_secret_selectors_preserve_original_wrappers` | Actual secretwrap, debug taint mode and nil stack taint; each/all slots permitted; wrappers/value and caller context retained. |
| `tests/calendar_event_index_info.rs:46` | `calendar_event_index_default_session_returns_three_exact_dtos` | Three numeric default IDs 910001/910002/910003 and exact three-field positions. |
| `tests/calendar_event_index_info.rs:77` | `calendar_event_index_each_mismatching_filter_returns_one_nil` | Each/both mismatching filters return exactly one nil; matching lookup survives. |
| `tests/calendar_event_index_info.rs:214` | `calendar_event_index_invalid_numbers_cannot_alias_fixture_positions` | Fractional IDs/filters, NaN/infinite/huge numbers cannot alias fixtures. |
| `tests/calendar_event_index_info.rs:132` | `calendar_event_index_observes_owned_state_updates_and_removal` | Owned index state update/removal observed; old snapshots/other IDs retained. |
| `tests/calendar_event_index_info.rs:57` | `calendar_event_index_omitted_nil_and_matching_filters_return_dtos` | Omitted/nil/offset/day/both matching filters for all three defaults. |
| `tests/calendar_event_index_info.rs:194` | `calendar_event_index_required_and_optional_numbers_reject_wrong_types` | Required ID and optional filters reject wrong types including unknown IDs. |
| `tests/calendar_event_index_info.rs:173` | `calendar_event_index_restoration_preserves_namespace_and_unrelated_functions` | Post-cleanup restoration retains namespace identity and unrelated function sentinel. |
| `tests/calendar_event_index_info.rs:111` | `calendar_event_index_result_tables_are_independent_snapshots` | Fresh independent index DTOs; mutations cannot affect later or unrelated fixtures. |
| `tests/calendar_event_index_info.rs:265` | `calendar_event_index_tainted_secret_inputs_are_denied_before_lookup` | Actual tainted secrets denied before type/lookup, even malformed earlier slots; context/wrappers retained. |
| `tests/calendar_event_index_info.rs:95` | `calendar_event_index_unknown_identity_returns_one_nil` | Unknown numeric identities with omitted/nil/offset/day/both filters return one nil. |
| `tests/calendar_event_index_info.rs:241` | `calendar_event_index_untainted_secret_inputs_preserve_wrappers` | Actual untainted secret ID/offset/day calls; wrappers and stack context retained. |

Projection source: day_event_projection.rs:9-81 creates rooted fresh outer and nested tables: 8 strings, 6 required numbers, 1 optional icon, 4 booleans, 2 time tables = 21/20 outer fields. Each time has six numeric fields. fixtures.rs:70-118 recursively asserts exact field inventory/type/value and return arity, including opaque IDs 9007199254740997/9007199254740995, false flags and 7.5/2.5 numbers. No forced-GC stress claim. Query source day_event_queries.rs:44-86 authenticates all slots before numeric validation, checks representability, and clones owned data before Lua allocation.

## Artifact checks

[EXIST] PASS — concrete source inventory with byte/line counts in proof.json.
[SUBSTANTIVE] PASS — real DTOs, host map queries/projection and behavioral assertions; no placeholder bodies.
[WIRED] PASS — C API registration, SimState map/default and executed integration cases; consumer closure loads entire cached ItemRef.lua and real CalendarUtil dependencies.
[ANTI-PATTERN] PASS — zero TODO/FIXME/HACK/XXX markers in inspected ten-file source scope; no empty placeholder/catch bodies observed.

Numeric index preservation: producer diff leaves existing query/filter implementation and demo IDs unchanged; 11 saved regressions pass. Separate caller 910001 fixture is never derived from opaque day IDs. docs HEAD over producer adds docs and historical RED evidence, not implementation changes.

## Limits / parent remains open

- No new runtime/build; saved evidence only, not freshly executed tests.
- Stats only compare dev/inode/size/mtime_ns to both once-auth receipts and seals; not renewed byte authenticity or cryptographic source association.
- No source.tar access, library hashing, executable byte reads, reauthentication, delegation, tracked edits, runtime or build.
- External caches/libraries/settings not authenticated; current cached consumer source inspected, historical cache byte identity not independently proven.
- CASC/sound disabled, no third-party addons or SavedVariables; read-only WTF EditMode cache loaded; not native CASC/render/editor proof.
- No native bounds/date/order/arity, Calendar ID conversion, full Calendar/Club producer, lockdown output secrecy, full parent goal or Retail GREEN proof.
- Prior preproducer RED is separate history, not reverified by this gate.

Runtime argv uses read-only root/source/target binds, unshared network, temporary /tmp, CASC=0 and sound disabled. Normal stderr confirms 283 cached Blizzard addons, SavedVariables/addon loading disabled, and read-only WTF EditMode cache use. No editor or rendered/native CASC observation. Whole cached consumer loading is source/runtime evidence, not independent historical cache byte authentication. Full Calendar output secrecy (SecretInChatMessagingLockdown) and full parent completion remain UNPROVED.
