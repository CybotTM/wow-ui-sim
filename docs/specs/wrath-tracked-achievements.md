# Wrath tracked achievements

Wrath-only tracking globals operate on the simulator's existing achievement catalog and tracked IDs. The contract comes from the laptop's 2026-10-21 static reconstruction of build 12340 (AchievementInfo_Scripts.cpp:147–155, 569–596, 1023–1048; AchievementInfo.cpp:344–364, 524–537). This is source evidence, not native execution. Cached FrameXML consumer expectations without authenticated 12340 source remain **INFERRED**.

## What it must do

- [ ] `GetTrackedAchievements()` returns separate numeric IDs in insertion order, zero values when empty, ignoring arguments. Removing preserves remaining order; re-adding appends.
- [ ] `GetNumTrackedAchievements()` returns one numeric count. `IsTrackedAchievement(id)` returns exactly one boolean; unknown, zero and nonconvertible IDs are false.
- [ ] Add/remove use Lua numeric conversion followed by int32 truncation, ignoring extra arguments. Add rejects zero and absent catalog IDs with `Usage: AddTrackedAchievement(achievementID)`; remove rejects only zero with its corresponding usage error. Both return zero values.
- [ ] Add accepts completed catalog records. A distinct ID at ten entries is a silent no-op. Duplicate add leaves the list unchanged and still dispatches its event, including at capacity. The authenticated reconstruction states: “Already tracked: unchanged; event still fires.”
- [ ] Successful add and removal dispatch `TRACKED_ACHIEVEMENT_UPDATE(id)` synchronously after mutation and before return. Missing removal emits nothing. Original AchievementUI consumers remain installed and update their private tracking cache through that event.
- [ ] Existing shared `SetTrackedAchievement`/`UntrackAchievement` membership semantics remain unchanged; Retail behavior and registration stay unchanged. New native endpoints use the existing `client-wrath` module gate; the dedicated contract tests use the same profile gate.

## How it works

- [Lua API systems](../wiki/systems/lua-api.md#wrath-tracked-achievements)

## Implementation inventory

- `src/lua_api/globals/real/wrath_startup_queries.rs`: Wrath-only native readers and inline tracking mutations.
- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs`: existing unique tracked membership stored as an insertion-ordered vector.
- `src/lua_api/globals/quest_verbs.rs`: existing shared setters preserve uniqueness and remove matching IDs without reordering survivors.

## Tests asserting this spec

- `tests/wrath_achievement_tracking.rs`: ordered inline events, zero returns, count/membership, duplicate/capacity behavior, completed records and argument validation.
- `tests/blizzard_ui/blizzard_achievementui/behavior_track_toggle.rs`, `behavior_category_select.rs`: unchanged original native consumers reach and consume the tracking producer.
- `tests/wrath_residual_a.rs`: existing original consumer reads shared setter mutations.
- `tests/quest_verbs.rs`: existing shared setter controls run under Retail before and after.

## Known gaps (current cycle)

- [ ] Current-source RED: 54 Wrath cases each execute one test, 49 pass and five fail at missing tracking natives before the producer. GREEN and final profile preservation remain pending.
- [ ] Retail baseline: original 72 cases have 57 passes and 15 existing failures; four additional shared-setter controls pass. Existing failures are missing `C_AchievementTelemetry` and stale hard-coded cached TOC paths, not tracking producer failures. Preserve/report rather than repair them here.
- [ ] Modern-only guild/search/restricted rows remain unchanged report-only Wrath gate candidates, not supported Wrath features.

## Out of scope

Persistence, teardown/account-data synchronization, achievement server messages, timed criteria, other producer defects, vendor changes and modern-only Wrath feature emulation. No claim of native execution or complete client parity.
