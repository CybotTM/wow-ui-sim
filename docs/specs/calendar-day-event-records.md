# Host-supplied Calendar day-event records

`C_Calendar.GetDayEvent` and `GetNumDayEvents` expose host-owned day records with empty defaults. Passive DTO source: `src/c_api/c_calendar/day_event_record.rs`; storage: `SimState.calendar_day_events`. This test-first, unexecuted contract is distinct from [numeric event-index lookup](calendar-event-index-info.md). See [Calendar source boundary](../wiki/investigations/patch-8-1-0-api-audit.md#current-retail-calendar-query-contract--2026-10-10) for provenance limits.

Primary identifier reference: upstream wowless `data/products/wow/structures.yaml`, `C_Calendar.CalendarDayEvent`, inspected 2026-10-13: `eventID` and `clubID` are strings. Cached Retail/PTR `Blizzard_APIDocumentationGenerated/CalendarDocumentation.lua` declares the same 21-field inventory via named ID typedefs; `TimeDocumentation.lua` declares six CalendarTime fields. These sources are declarations, not native behavior evidence. Local profile cache files remain external runtime inputs, not tracked implementation.

## What it must do

### Supplied records and lifetime

- [ ] Keep day storage empty in normal defaults; only host-supplied records make day queries positive. Do not add a Lua editor, calendar producer, or default day rows.
- [ ] Address host rows by exact `(offset_months: i32, month_day: u32)` key and dense Vec order. `GetNumDayEvents` returns the matching length, zero for absent/empty keys. `GetDayEvent` uses 1-based positions, one DTO for a supplied row, one nil on miss. Ordering, representability and one-nil arity are explicit simulator model policies, not native bounds/arity proof; `MayReturnNothing` does not establish arity.
- [ ] Return independent outer and start/end tables. Caller mutation must not change host data or other results; replacement and erase must affect later lookups while retained snapshots stay unchanged. Environments must not share day rows.

### Projection inventory

- [ ] Project exactly 21 outer fields with `iconTexture` present, exactly 20 when absent; both required CalendarTime tables contain exactly six fields. Preserve concrete values and declared primitive types, including false booleans and fractional declared numbers.
- [ ] Preserve `eventID='9007199254740997'` and `clubID='9007199254740995'` byte-for-byte as strings. Never convert these to Calendar event-index fixture IDs.

Outer field inventory:

| Lua fields | Host fields / representation |
| --- | --- |
| eventID, clubID | event_id, club_id: String |
| title, calendarType, sequenceType, modStatus, invitedBy, difficultyName | title, calendar_type, sequence_type, mod_status, invited_by, difficulty_name: String |
| isCustomTitle, dontDisplayBanner, dontDisplayEnd, isLocked | is_custom_title, dont_display_banner, dont_display_end, is_locked: bool |
| eventType, inviteStatus, inviteType | event_type, invite_status, invite_type: i32 |
| iconTexture | icon_texture: Option<u32> |
| difficulty, numSequenceDays | difficulty, num_sequence_days: f64 |
| sequenceIndex | sequence_index: u32 |
| startTime, endTime | start_time, end_time: separate CalendarTimeRecord |

Time field inventory, independently required for each time:

| Lua fields | Host fields / representation |
| --- | --- |
| monthDay, month, weekday | month_day, month, weekday: u32 |
| year, hour, minute | year, hour, minute: f64 |

### Inputs and secrets

- [ ] Require numeric month offset/day for both functions, plus numeric position for GetDayEvent; reject missing/nil/wrong types without string coercion. Validate every required slot even for absent keys.
- [ ] Nonintegral, nonfinite or nonrepresentable selectors must not alias a represented key/position. Counts zero and rows one nil are model miss policy; this does not assert native date/index bounds.
- [ ] Apply AllowedWhenUntainted to authentic VM secret inputs for both functions: permit untainted calls, deny tainted secret calls, authenticate every slot before type validation or lookup. Preserve original wrappers and caller taint. Secret tests require the existing retail-12-1-0 feature and actual VM secret machinery.

### Real cached consumers

- [ ] Unchanged cached complete `Mainline/ItemRef.lua` GetCalendarEventLink must consume day.title, reflect host title replacement, and return nil after erase; no mock, extracted replacement, or ID conversion.
- [ ] Unchanged cached CalendarUtil ongoing broadcast must consume a separate caller-owned numeric index fixture 910001 at `(0,15,1)`, optional monthDay=15, and supplied day title. Caller event's numeric ID is not the day DTO's string ID. Use real GameTime/LinkUtil/colors/constants/localization dependencies, not a whole Club producer.
- [ ] Keep existing numeric event-index map/default fixture policy unchanged; day tests may explicitly insert their own numeric fixture only inside their environment.

## How it works

- [Calendar source boundary](../wiki/investigations/patch-8-1-0-api-audit.md#current-retail-calendar-query-contract--2026-10-10)
- [Event-index contract](calendar-event-index-info.md)
- [C API boundary](../../AGENTS.md#c-api-boundary)

## Implementation inventory

- `src/c_api/c_calendar/day_event_record.rs`: passive owned DTOs; no projection/query behavior.
- `SimState.calendar_day_events`: `HashMap<(i32,u32),Vec<CalendarDayEventRecord>>`, empty default.
- `src/c_api/c_calendar.rs`: owning Calendar namespace; existing event-index lookup unchanged; day-query producers pending.
- Active profile cache `Blizzard_UIPanels_Game/Mainline/ItemRef.lua` and `Blizzard_FrameXMLUtil/CalendarUtil.lua`: unchanged consumer dependencies.

## Tests asserting this spec

Integrated tests, execution pending:

- `tests/calendar_day_events.rs`: Retail/PTR-only root module, automatically discovered by test harness.
- `tests/calendar_day_events/queries.rs`: five cases for projection/lifetime, exact count/order/defaults, isolation, required types and nonrepresented numeric policy.
- `tests/calendar_day_events/security.rs`: two actual-VM secret cases under retail-12-1-0.
- `tests/calendar_day_events/consumer.rs`: two real-cache consumer cases.
- `tests/calendar_day_events/fixtures.rs`: explicit public passive DTO fixtures and exact Lua value assertions.

Each seeded case first requires a supplied DTO before security/consumer setup. Count case first requires a positive count, then a supplied DTO. Function guard distinguishes missing registration from MissingSuppliedRecord. No case earns behavior credit with only function existence or absence/default assertions.

## Known gaps (current cycle)

- [x] Integrate passive DTO/map and nine tests before the day-query producer; formatting/integration are not executed RED evidence.
- [ ] Compile and execute nine targeted cases separately on Retail and PTR after main integrates tests; no build/runtime execution occurred here.
- [ ] Implement day-query producers only after genuine behavioral RED; GREEN remains unproved.
- [ ] Consumer closure dependencies must load unchanged and real required localization/colors must exist; fail visibly rather than skip or patch them.

## Out of scope

Native validation/bounds/return arity, CalendarID conversions, native date/order parity, whole Calendar/Club event producer, normal day seeding, selected/open-event/edit/invitation lifecycle, index-policy changes, native-client observation and broad startup acceptance.

`SecretInChatMessagingLockdown` output secrecy has no established backing model here: UNPROVED, not tested or claimed. Only unlocked host-data output is covered by this bounded contract.
