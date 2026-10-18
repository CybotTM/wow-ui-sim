# GUI startup yielding

GUI startup settling yields between app updates without changing timer callback identity or invocation. Source: `src/iced_app/app.rs` and `src/iced_app/update_runtime.rs`; [mechanics](../event-system.md#startup-event-sequence).

## What it must do

- [ ] Draw records the first real canvas size without executing startup or resize Lua; startup waits until that size is available and runs its event sequence once.
- [ ] Service queued IPC before advancing startup on a timer update. A zero-delay callback that schedules another callback yields to the next app update; queued Exec can run between those generations and executes once.
- [ ] Carry the initial ten-pass timer budget and sixteen settling OnUpdates, each followed by up to ten timer passes, across turns. A turn performs at most one timer processing pass and one settling OnUpdate; an empty pass ends its timer group.
- [ ] Keep frame-rate startup ticks active after the first canvas is known until startup finishes, even with no timers pending. Complete cleanup, hidden-frame policy, layout/render initialization and initial texture warmup before drawing UI quads.
- [ ] After startup, dispatch resize Lua from timer updates rather than draw. Preserve ordinary timer behavior and headless settling.

## How it works

- [Event-system startup mechanics](../event-system.md#startup-event-sequence).
- [Wiki event system](../wiki/systems/event-system.md#startup-event-sequence).

## Implementation inventory

- `src/iced_app/app.rs` — first-canvas capture, settling progress, completion and tick subscription.
- `src/iced_app/update.rs` — IPC processing before message dispatch.
- `src/iced_app/update_runtime.rs` — startup advancement and subsequent resize dispatch.
- `src/iced_app/render_draw_frame.rs` — records bounds and returns empty startup batches.

## Tests asserting this spec

`src/iced_app/app_tests.rs`: `gui_startup_zero_delay_chain_yields_to_queued_exec`, `gui_startup_uses_first_real_canvas_size_for_display_size_changed`, `gui_startup_draw_defers_lua_and_keeps_event_order_once`, `gui_startup_branching_zero_delay_chain_spans_turns`, `gui_startup_waits_for_canvas_before_running_lua`, `gui_startup_settles_ready_timers_at_completion`, `gui_startup_settles_bounded_on_update_work_at_completion` and `gui_startup_closes_windows_created_by_startup_timers`. Existing timer/container regressions own ordinary invocation preservation.

## Known gaps (current cycle)

- [ ] Attach final-revision execution receipts before marking requirements verified; this documentation audit inspected source/tests only.
- [ ] Real Ellesmere GUI first-draw and IPC acceptance remains separate from the bounded App fixtures.

## Out of scope

Preempting a synchronous Lua callback, changing timer identity/arguments/taint/cancellation, headless startup policy, or claiming native scheduling parity.
