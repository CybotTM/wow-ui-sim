# Wrath legacy calendar projections

Source-only proposed Wrath simulator contract (2026-10-16), not native-client evidence.

## What it must do

- [ ] Publish CalendarGetDate before original GameTime.lua captures it; return weekday, month, monthDay, year from the existing C_DateAndTime producer.
- [ ] Publish CalendarGetNumPendingInvites from C_Calendar, without independent defaults or UI-field seeding.
- [ ] Original GameTime.xml OnLoad displays the existing default day; original PLAYER_ENTERING_WORLD clears empty invitations and glow state.

## How it works

- [Lua API](../lua-api.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

`src/lua_api/workarounds/temporary/date_and_time_defaults.rs`: Wrath projections over existing temporary defaults.

## Tests asserting this spec

`tests/wrath_unmasked_calendar.rs`; shared pinned originals in `tests/wrath_unmasked_support/`.

## Known gaps (current cycle)

- [ ] No executed RED/GREEN or startup acceptance.
- [ ] Clock and invitation state are unmodeled: fixed 2026-04-14, zero pending invitations inherited from existing producers. Changed-date/populated-invite behavior is not covered; retire these adapters alongside the defaults when server calendar state is modeled.

## Out of scope

Blizzard/vendor edits, native parity, full profile acceptance, builds and runtime execution during staging.
