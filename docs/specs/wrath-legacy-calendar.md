# Wrath legacy calendar projections

Wrath simulator projections over existing temporary sources, not native-client evidence.

## What it must do

- [x] Publish CalendarGetDate before original GameTime.lua captures it; return weekday, month, monthDay, year from the existing C_DateAndTime producer.
- [x] Publish CalendarGetNumPendingInvites from C_Calendar, without independent defaults or UI-field seeding.
- [x] Original GameTime.xml OnLoad displays the existing default day; original PLAYER_ENTERING_WORLD clears empty invitations and glow state.

## How it works

- [Lua API](../lua-api.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

`src/lua_api/workarounds/temporary/date_and_time_defaults.rs`: Wrath projections over existing temporary defaults.

## Tests asserting this spec

`tests/wrath_unmasked_calendar.rs`; shared pinned originals in `tests/wrath_unmasked_support/`.

## Known gaps (current cycle)

- [x] Original-consumer RED reaches GameTime.lua:247 and independently :157. Both tests pass in the d11a25154 sealed Wrath43 gate; both startup records disappear in the 22/30 → 18/24 delta. This remains a failing whole startup.
- [ ] Clock and invitation state are unmodeled: fixed 2026-04-14, zero pending invitations inherited from existing producers. Changed-date/populated-invite behavior is not covered; retire these adapters alongside the defaults when server calendar state is modeled.

## Out of scope

Blizzard/vendor edits, native parity and full-profile acceptance.
