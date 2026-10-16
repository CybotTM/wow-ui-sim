# Wrath residual startup readers (stage A)

Bounded simulator contracts for six source-established Wrath startup failures.
Modeled legacy globals live in `src/lua_api/globals/real/wrath_startup_queries.rs`;
unmodeled values remain in `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`.
This is a source-only staged delta: tests have not been compiled or executed.

## What it must do

### Tracking and aura projection

- [ ] `GetTrackedAchievements()` projects existing tracked IDs as separate numeric returns, including zero returns when empty; existing setters and removers remain observable through the original achievement ADDON_LOADED/TRACKED_ACHIEVEMENT_UPDATE consumer.
- [ ] Wrath `UnitAura`, `UnitBuff`, and `UnitDebuff` retain the rank slot: name, rank, icon, applications, dispel, duration, expiration, source, stealable, consolidation, spell ID. Modeled fields retain their values; missing aura terminates enumeration with nil name.
- [ ] Original AuraButton_Update displays the supplied icon/count and computes numeric timeLeft from a supplied deadline for helpful and harmful auras; removing the aura hides its button and duration.
- [ ] Other profiles retain their existing legacy tuple, and `C_UnitAuras` retains its existing table contract. Only a Wrath table-preservation case is staged; cross-profile execution remains open.

### Equipment, trainer, and group projections

- [ ] `OffhandHasWeapon()` uses equipped slot 17 and catalogued inventory type: 13/17/21/22 are weapons, shield 14 and holdable 23 are not. Empty equipment returns false. Original direct durability selection must reset the selected weapon/shield texture and hide the alternative. Complete XML/event/vehicle recovery is not asserted.
- [ ] Wrath trainer getters/setters retain independent available/unavailable/used visibility flags. Numeric 0 disables and numeric 1 enables; original dropdown initialization and click callbacks must reflect persisted changes. Initial simulator policy `[true, true, false]` follows the cached trainer panel constants, not a measured native default.
- [ ] `IsRaidLeader()` uses the same active group/leader inputs as existing group queries, including their current simulated raid threshold of six companions. Wrath `IsPartyLeader()` uses those same inputs for party-only leadership instead of unrelated matchmaking state. Original LFD/LFR empowerment and find-group buttons must distinguish local/nonlocal raid leader, local/nonlocal party leader, and no group.

### Explicit temporary limits

- [ ] `GetQuestTimers()` returns zero values for the currently unmodeled timed-quest domain. Original WatchFrame_HandleDisplayQuestTimers must produce zero dimensions and no timer lines.
- [ ] Missing aura rank remains nil and missing consolidation policy remains false, without shifting modeled fields. Unknown equipped-item classification remains false only in the named temporary workaround. Retire each temporary policy when its backing system is modeled.

## How it works

- [Lua API architecture](../lua-api.md)
- [Client profile architecture](../wiki/systems/client-profiles.md)

## Implementation inventory

- `src/lua_api/globals/real/wrath_startup_queries.rs` — existing-state legacy readers and persistent trainer filter endpoints.
- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs` — passive Wrath trainer filter input and initial simulator policy.
- `src/lua_api/globals/real/mod.rs`, `src/lua_api/globals/register.rs` — Wrath-only registration after existing group registration.
- `src/lua_api/globals/auras.rs` — profile-scoped tuple projection; no C_* model relocated.
- `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`, `src/lua_api/workarounds/temporary/mod.rs` — named missing-domain defaults and retirement conditions.
- `src/wrath/compat_bootstrap.lua` — removes replaced one-nil quest timer definition; this shared directory does not imply enabling the new registration for Mists/Era/Anniversary.

## Tests asserting this spec

- `tests/wrath_residual_a.rs` — six original-consumer cases; SHA256-pinned cache files, callback and direct error audits, explicit fixtures.
- `tests/wrath_residual_a_consumers.json` — exact cached source bytes required by those tests.

## Known gaps (current cycle)

- [ ] All behavioral RED/GREEN, compilation and cross-profile preservation runs remain unexecuted under the staging-only/no-build instruction.
- [ ] Full original startup ordering and all residual errors remain parent-goal acceptance work.
- [ ] Trainer service catalog filtering is not modeled by this endpoint-only delta; menu state roundtrips do not prove filtered service rows.
- [ ] Positive durability alert severity is unmodeled; staged original-consumer tests use the existing zero-alert producer and assert selected texture/color reset, not positive damage alerts.
- [ ] Aura spell rank, consolidation policy, uncatalogued item classification and timed quests remain named temporary gaps.

## Out of scope

Native-client parity, complete historical tuples beyond the bounded projection,
server behavior, timed quest population/expiry, trainer service catalogs, raid model
redesign, other residual clusters, vendor Lua changes, builds and deployment.
