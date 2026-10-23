# CF-2 PvP and map consumers

CF-2 slice 4 covers the original cached Blizzard match-results event condition, battlefield vehicle provider, and group-map icon visibility consumer. Modeled namespace behavior belongs in `src/c_api/`; Blizzard Lua remains unchanged. This cycle stages consumer tests only: no observed RED or producer implementation yet.

## What it must do

### Match results — Retail, PTR, WowForever

- [ ] The original `PVPMatchResultsMixin:OnEvent("PLAYER_ENTERING_WORLD")` leaves completion/reward events unregistered when explicitly supplied active and completed inputs are both false.
- [ ] An explicitly active match registers completion/reward events; after clearing frame registrations, an explicitly completed, non-active match also registers them.
- [ ] Reading completion uses supplied simulator match state, not a constant chosen to bypass the event condition.

### Group map icons — Era, Anniversary

- [ ] Original `GroupMembersPinMixin:OnMapChanged` hides its real UnitPositionFrame when the selected map's supplied `hideIcons` value is true.
- [ ] Explicit false shows the pin and binds its UI map ID to the selected map.
- [ ] Switching between maps 84/85 preserves independent inputs; live updates on either map change visibility without overwriting the other's input. IDs are fixture selectors, not native map-policy claims.

### Battlefield vehicles — Mists

- [ ] PENDING-NATIVE-ORACLE: populated map-scoped records must reach the original `VehicleDataProviderMixin:RefreshAllData`; it acquires pins only for records with `x`, `isAlive`, no `isPlayer`, and `atlas`. Consumer proves this filter, not a native default, input schema, or enumeration-order policy.
- [ ] PENDING-NATIVE-ORACLE: establish an authoritative normal state path before writing a populated executable fixture; no API replacement or invented vehicle defaults.

## How it works

- [C API and Lua architecture](../lua-api.md)
- [Existing supplied map display contract](map-display-info.md)

## Implementation inventory

- `tests/cf2_pvpmap.rs`: original-cache closure loading, match event subscription and group-pin visibility fixtures; no producer changes.
- `src/c_api/c_pvp.rs`: existing `IsMatchActive` projection of `SimState.pvp_match_active`; completion getter remains missing.
- `src/c_api/c_pvp/catalog.rs`: existing `PvpCatalog.match_completed` supplied snapshot.
- `src/c_api/c_map.rs`: existing map display projection, currently excluded from Era/Anniversary by its epoch gate.
- `src/lua_api/state/sim_state.rs`: existing per-map `map_display_hide_icons` input; no battlefield vehicle catalog found.

## Tests asserting this spec

`tests/cf2_pvpmap.rs` (integration harness discovers this module):

- `original_match_results_enter_world_ignores_explicit_inactive_match`
- `original_match_results_enter_world_subscribes_for_active_and_completed_matches`
- `original_group_pin_tracks_map_display_visibility_and_live_updates`

Authored, uncompiled and unrun. Runtime `ACTIVE` reports N/A elsewhere; returned non-applicable tests are not passing compatibility evidence. Fixtures call complete original functions loaded through the profile-cache addon closure. Real widget outputs are minimal presentation fixtures, not alternate consumers. Existing addon-load errors remain recorded; direct consumer calls must succeed without adding errors.

## Known gaps (current cycle)

- [ ] Observe intended original-consumer RED on each applicable profile before producer work.
- [ ] Completion API publication and Era/Anniversary map-display publication need producer changes after RED.
- [ ] Mists vehicles: no existing seedable record state or indexed vehicle-info API found. Native questions: record fields/types, map ownership, stable one-based enumeration and relation to `GetBattlefieldVehicleInfo(index,mapID)`, removal/update lifetime, absent-coordinate/atlas policy. Original provider lines 21–38 and pin refresh lines 57–72 consume these contracts. No populated vehicle test or passing claim yet.
- [ ] Native completion lifecycle/reset and conflicting active/completed inputs remain unresolved. Tests use explicit non-conflicting snapshots only; they prescribe no native defaults or transitions.

## Out of scope

Matchmaking, rewards, native match acquisition/reset, native map-ID defaults, malformed/secret selectors, player ping, full map templates, startup health, and whole-profile acceptance. No vendor patches, private tracked paths, test-only API overrides, new hiding cfg gates, or producer code before observed RED.
