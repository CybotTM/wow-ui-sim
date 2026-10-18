# Unit existence state and aura visibility

`UnitExists` reads simulator state, and shared aura collection suppresses records for units without modeled existence. This records commits `1a5e8f348`, `1681029d7` and `35822d5c7`; the producers live in `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/globals/group_queries.rs` and `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/globals/auras.rs`. See [Lua API architecture](/home/osso/Projects/wow/wow-ui-sim/docs/wiki/systems/lua-api.md). These are simulator contracts, not native-parity claims.

## What it must do

### Modeled existence

- [x] Keep `player` present. Report `pet` present exactly when `pet.icon` is populated; summon/dismiss state changes must update the result.
- [x] Report `vehicle` present exactly while `player.in_vehicle` is true; entering/exiting must update the result.
- [x] Report `target` absent before selection and after clearing, and present while `current_target` is populated.
- [ ] Report `focus` present exactly when `current_focus` is populated. Party tokens require both `party_group_active` and an in-range member index (`unit_exists_in_state`).

### Aura visibility and result boundaries

- [x] Absent targets expose no legacy UnitAura/UnitDebuff record, indexed/instance record or aura slots; selecting a target exposes its stored debuffs, and clearing it hides them again.
- [x] Inactive party membership hides stored party auras from legacy and indexed queries without deleting them; activating membership exposes the stored records.
- [ ] All retrieval/enumeration paths using the shared collector must suppress records for missing units. This does not bypass existing argument validation or access checks.
- [ ] Preserve these specific missing-record results for otherwise accepted inputs, where the API is registered: `GetAuraApplicationDisplayCount` returns `""`; `IsAuraFilteredOutByInstanceID` returns **true** (missing means filtered out); `GetAuraBaseDuration` and `GetRefreshExtendedDuration` return nil; `DoesAuraHaveExpirationTime` returns false.
- [ ] Preserve existing errors rather than describing every aura query as empty: curve-based `GetAuraDispelTypeColor` raises `aura instance not found`; `GetAuraDuration` raises `no such aura instance`.

The checked items have regression assertions in the test file below and historical Retail/Forever GREEN evidence in `/home/osso/.local/state/wow-ui-sim/verification/unit-exists-state-verify/report.md`. Its section 4 records the broader source-level impact. Unchecked items remain source-grounded without claiming dedicated transition-test coverage. No tests are rerun for this docs-only change.

## How it works

- [Lua API architecture](/home/osso/Projects/wow/wow-ui-sim/docs/wiki/systems/lua-api.md).
- [Unit GUID presence](/home/osso/Projects/wow/wow-ui-sim/docs/specs/unit-guid-presence.md) — separate identity-query contract.

## Implementation inventory

- `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/globals/group_queries.rs`: `UnitExists` and shared `unit_exists_in_state` predicate.
- `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/globals/auras.rs`: existence gate before aura-store reads; shared collection and instance lookup.
- `/home/osso/Projects/wow/wow-ui-sim/src/c_api/c_unit_aura_display_count.rs`: empty missing-record display count.
- `/home/osso/Projects/wow/wow-ui-sim/src/c_api/c_unit_aura_filter_query.rs`: missing records are filtered out.
- `/home/osso/Projects/wow/wow-ui-sim/src/c_api/c_unit_aura_dispel_color.rs`: curve query's missing-instance error.
- `/home/osso/Projects/wow/wow-ui-sim/src/c_api/aura_duration.rs`: base/refresh nil results, expiration false and duration-object error on missing records.
- `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/globals/utility_system_spell/spell_api.rs`: unchanged pet/vehicle player-vitals shortcut.

## Tests asserting this spec

`/home/osso/Projects/wow/wow-ui-sim/tests/unit_exists_state.rs`:

- `pet_existence_tracks_summon_and_dismiss_from_rust_state` and `vehicle_existence_tracks_player_occupancy_from_rust_state`: absent/present/absent transitions and player controls.
- `target_aura_queries_follow_selection_and_clear`: existence, legacy/index/instance/slot absence and populated target controls.
- `party_aura_queries_require_active_membership_without_deleting_stored_auras`: inactive/active/inactive query visibility and retained stored aura.
- Retail-only `compact_unit_frame_pet_visibility_tracks_summon_and_dismiss` and `blizzard_aura_util_visits_only_selected_target_debuffs`: cached Blizzard consumers through simulator state transitions.

The first four have historical Retail and Forever passes; consumer tests have Retail passes only. This does not establish all-profile coverage.

## Known gaps (current cycle)

- [ ] Pet/vehicle vitals still return player vitals before checking existence in `lookup_unit_vitals`; health/power behavior is unchanged and inconsistent with absent pet/vehicle existence.
- [ ] The six focused regressions do not directly cover focus transitions or missing-unit display-count/filter/duration/dispel transitions; broader collector effects are source-grounded, not a universal empty-result guarantee.

## Out of scope

New pet/vehicle vitals or identity models, new aura stores, changes to argument/security policy, native-client equivalence and unmeasured GUI effects. This slice changes existence and aura visibility, not every API that accepts a unit token.
