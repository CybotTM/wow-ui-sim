# Wrath tracked achievements

Wrath-only tracking globals operate on the simulator's existing achievement catalog and tracked IDs. The contract comes from the laptop's 2026-10-21 static reconstruction of build 12340 (AchievementInfo_Scripts.cpp:147–155, 569–596, 1023–1048; AchievementInfo.cpp:344–364, 524–537). This is source evidence, not native execution. Cached FrameXML consumer expectations without authenticated 12340 source remain **INFERRED**.

## What it must do

- [x] `GetTrackedAchievements()` returns separate numeric IDs in insertion order, zero values when empty, ignoring arguments. Removing preserves remaining order; re-adding appends.
- [x] `GetNumTrackedAchievements()` returns one numeric count. `IsTrackedAchievement(id)` returns exactly one boolean; unknown, zero and nonconvertible IDs are false.
- [x] Add/remove use Lua numeric conversion followed by int32 truncation, ignoring extra arguments. Add rejects zero and absent catalog IDs with `Usage: AddTrackedAchievement(achievementID)`; remove rejects only zero with its corresponding usage error. Both return zero values.
- [x] Add accepts completed catalog records. A distinct ID at ten entries is a silent no-op. Duplicate add leaves the list unchanged and still dispatches its event, including at capacity. The authenticated reconstruction states: “Already tracked: unchanged; event still fires.”
- [x] Successful add and removal dispatch `TRACKED_ACHIEVEMENT_UPDATE(id)` synchronously after mutation and before return. Missing removal emits nothing. Original AchievementUI consumers remain installed and update their private tracking cache through that event.
- [x] Existing shared `SetTrackedAchievement`/`UntrackAchievement` membership semantics remain unchanged; Retail behavior and registration stay unchanged. New native endpoints use the existing `client-wrath` module gate; the dedicated contract tests use the same profile gate.

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

- [ ] Retail retains 15 existing failures: missing `C_AchievementTelemetry` and stale hard-coded cached TOC paths, not tracking producer failures. Their exact per-case statuses and normalized failure messages are unchanged; repair belongs to separate work.
- [ ] Ten modern-only guild/search/restricted rows remain excluded under Wrath by the existing `post_wrath_ui_fixture!` gate (`tests/common/profile_fixtures.rs:17–29`). Gate commit `70dabc87f` predates the first F08 fixture commit `83bc7b815`; no gate or modern-only row was changed. These rows were not executed under Wrath and are not counted as passes.

## Out of scope

Persistence, teardown/account-data synchronization, achievement server messages, timed criteria, other producer defects, vendor changes and modern-only Wrath feature emulation. No claim of native execution or complete client parity.

## Bounded proof — 2026-10-21

Current-source baseline `19bf3b80e` precedes production changes; final runtime proof is bound to merged `e90ab137e` (including requested master `342acab31`). Each named Rust case below executes exactly one test.

| Scope | Result |
|---|---|
| Wrath original consumers plus raw tracking contracts | RED54: 49 PASS / 5 missing-native FAIL → GREEN54: 54 PASS. Original category selection, tracking toggles and inline event consumers remain installed. |
| Existing shared model preservation | Five Wrath controls PASS: four shared-setter cases plus the original achievement consumer reading their mutations. |
| Zero-ID query contract | Public-API RED at `a297ab5dd`: a supplied shared-setter zero made `IsTrackedAchievement(0)` true. Regression assertions precede the Wrath-only zero predicate correction; final contract also checks exact false for nil and nonconvertible input in that state. |
| Retail preservation | Original72: 57 PASS / 15 existing FAIL before and after; additional four shared-setter controls PASS. All 76 per-case statuses and normalized failure messages match. |
| Stock and checks | Wrath/Retail stock `lua-errors` are exactly `[]` before and after. Final Wrath default `cargo check` has zero compiler or stderr warnings; `cargo fmt --check` passes, with direct rustfmt proof for all 28 changed test files, including macro-included files. |

Executables are sealed with current-worktree test-source binding. Capacity-admitted asynchronous user units use one build job, incremental compilation disabled, a shared target slot, 9 GiB memory and no swap. Child-only byte-exact CVar copies preserve the original stat/hash, including a recorded diagnostic launcher exception before binary execution. Exact commands, binary hashes, per-case results, gate ancestry and CVar receipts are retained in the F08 proof ledger. Static 12340 authority remains distinct from simulator execution and INFERRED cached-UI expectations.
