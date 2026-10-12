# Talent handler-log observation

Test-scoped observations associate existing handler logs with the canonical talent latency fixture. Logged records describe dispatch attempts and rounded intervals, not unique/top-level callbacks or exclusive callback-body cost. Logging may perturb latency; no repaired-performance or native claim follows.

## What it must do

- [ ] Mark each original tick outside its measured timer, preserving20 baseline/10 settle/24 steady samples, setup/order and existing35ms average/75ms max/+20ms delta assertions.
- [ ] Parse complete ordered begin/end windows and existing handler records. Preserve repeated OnUpdate/OnPostUpdate/other attempts, identities, zero intervals, grouped sums/maxima, and outside-window/unparsed-line counts.
- [ ] Reject malformed records, invalid durations, mismatched/overlapping/out-of-order/duplicate/missing/unclosed windows; emit deterministic JSON for a complete canonical log and explicit failure for invalid input.
- [ ] Retain actual diagnostic input/resource/source/command identity and original selector outcome. Completeness requires threshold-zero handler logging enabled before process initialization; a logged-run threshold result is not repair evidence.

## How it works

- [Event system](../event-system.md)
- [Frozen performance investigation](../wiki/investigations/fullsuite-ba-2026-10-11.md)

## Implementation inventory

- `tests/talent_panel_onupdate_perf.rs`: existing canonical fixture and test-only tick markers.
- `src/lua_api/script_helpers/event_dispatch.rs`: existing OnUpdate dispatch interval logger, unchanged.
- `src/lua_api/env_events.rs`: existing generic script/OnPostUpdate interval logger, unchanged.
- `tools/talent_handler_log.py`: standard-library-only log-to-JSON observation tool. Global grouping includes setup/outside-window records; phase grouping does not.

## Tests asserting this spec

- `tools/test_talent_handler_log.py`: concrete parser/CLI fixtures; development RED46 assertion failures then GREEN12 tests. Independent proof pending; initial syntax error excluded from RED credit.
- `tests/talent_panel_onupdate_perf.rs::talent_panel_open_onupdate_tick_latency_stays_under_budget`: original assertions retained; new marked/logged capture pending.

## Known gaps (current cycle)

- [ ] Execute genuine parser RED/GREEN, then authenticate one new targeted diagnostic and verify all54 sample windows and aggregate coherence.
- [ ] Source/readability, applicable checks, retention and documented interpretation limits.

## Out of scope

No vendor/runtime optimization, production metrics or new runtime configuration. Intervals include helper/error/nested work; summing nested intervals can double-count. Rounded log values and logging overhead are not a CPU profile or exclusive-body measurement. OnUpdate modes/visibility/budget filters still govern actual dispatch. Frozen16-thread full-suite failure and isolated single-case inputs/conditions are not established equivalent; historical/root-cause/full-handoff acceptance remains open.
