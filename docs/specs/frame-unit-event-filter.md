# Frame unit-event filter

`Frame:RegisterUnitEvent(event, unit1[, unit2])` must retain both supplied valid literal unit tokens for frame-script delivery. Registration lives under `src/lua_api/frame/methods/text_attribute_event/`; see the [event system](../event-system.md) for dispatch architecture. This bounded repair is approved for `player` and `target`; it is not native-client proof.

## What it must do

- [ ] Match the first event payload against either registered literal token (OR), delivering the original payload to `OnEvent`. With `"player", "target"`, both player and target health events must arrive; an unmatched token must not arrive.
- [ ] Preserve single-unit filtering: player-only registration suppresses target events and target-only registration suppresses player events.
- [ ] Re-registering with one unit replaces the two-unit filter; the former second unit no longer matches.
- [ ] `UnregisterEvent` and `UnregisterAllEvents` remove the complete filter and prevent delivery for both units.
- [ ] Keep existing `IsEventRegistered` first-unit reporting and query return arity unchanged. Keep the existing invalid-first-unit policy unchanged; neither behavior is newly established native parity.

## How it works

- [Event system](../event-system.md)

## Implementation inventory

- `src/lua_api/frame/methods/text_attribute_event/` — frame event registration and query methods.
- `src/lua_api/frame/methods/text_attribute_event/unit_event.rs` — existing literal-token validation and registered-unit query helper.

## Tests asserting this spec

- `tests/unit_event_second_unit.rs::second_unit_filter_receives_modeled_player_and_target_health_events` — modeled player healing and target damage, single-unit controls, original first payload, and existing first-unit query control.
- `tests/unit_event_second_unit.rs::reregistering_and_unregistering_clear_the_second_unit_filter` — unmatched-token suppression, one-unit replacement, unregister, and unregister-all controls.

## Known gaps (current cycle)

- [ ] RED/GREEN and verification for `tests/unit_event_second_unit.rs` remain pending; this document records no passing execution.

## Out of scope

Generic unit aliases, invalid-second-unit behavior, third-argument contracts, changes to invalid-first-unit policy or query arity, and native-client parity claims. These exceed the approved valid `player`/`target` second-unit repair.
