# Wrath manual quest watch

Wrath build-12340 manual watch calls project current rich quest-log rows into the unchanged original quest log and watch frame. The legacy Lua surface owns this seam, not the Forever quest-ID API. See [Wrath legacy state](../wiki/systems/lua-api.md#wrath-selected-quest-legacy-surface) and [abandon response](wrath-quest-abandon.md).

## What it must do

- [x] Start with no implicit watches. Store quest identity separately from current log row; retain append order for manual sorting, preserve survivor order on removal, and enforce 25 watches with a silent 26th-add no-op. Duplicate add and absent remove are inert.
- [x] `AddQuestWatch`/`RemoveQuestWatch` take a numeric-convertible current log row, return zero values and emit no direct events. Missing/non-numeric input uses the selected active identity; explicit invalid/header rows do nothing. No modern watch-list-change event is introduced.
- [x] `GetNumQuestWatches` returns one count. `IsQuestWatched` returns exactly one `1`/nil; `GetQuestIndexForWatch` returns exactly one current row/nil. Both queries reject non-numeric-convertible input with their `Usage: NAME(index)` error. Lookup scans all supplied rows by ID without a visibility filter; log reorder does not resort watches.
- [x] Abandon keeps watch membership through the immediate unit-change notification. The existing deferred/coalesced argumentless `QUEST_LOG_UPDATE` prunes missing identities before listeners consume the rebuilt projection, compacting survivors in watch order. No additional abandon producer or watch event is introduced.
- [x] Unchanged original shifted log clicks add/remove watches, shifted tracker clicks remove them, and actual check textures/visible title FontStrings agree after reorder and deferred abandon. Exact startup-error snapshot stays unchanged; consumers add no errors and never clear existing errors.

## How it works

- [Wrath legacy state](../wiki/systems/lua-api.md#wrath-selected-quest-legacy-surface).
- [Existing abandoned-quest notification boundary](wrath-quest-abandon.md).

## Implementation inventory

- `src/lua_api/globals/quest_surface/wrath_quest_watch.rs` — five Wrath-only manual bindings; C numeric-string conversion matches the Lua numeric input boundary.
- `src/lua_api/globals/quest_surface/register.rs` — overrides only those five legacy Wrath globals.
- `src/lua_api/sim_substates/quest_log.rs` — ordered manual IDs and refresh-time missing-ID pruning.
- `src/lua_api/env_events.rs` — prune before original deferred log-refresh listeners; existing abandon notifier remains unchanged.

## Tests asserting this spec

- `tests/wrath_quest_watch_manual.rs` — pinned original full-startup log/tracker, real shifted clicks, visible check/title membership, retained identity and abandon survivors.
- `tests/wrath_manual_watch_contract.rs` — source-relayed 12340 cap, query/return/argument semantics, identity/order, event silence and deferred pruning.
- `tests/fixtures/wrath_quest_watch_manual.lua` — authored input/oracles, no API or original-consumer replacement.
- `tests/fixtures/wrath_manual_watch_startup_errors.json` — exact startup-error precondition; no clearing.

## Known gaps (current cycle)

- [ ] Sort modes 1–3 and manual watch shifting; manual append-order seam only.
- [ ] Cold-cache omission and multi-missing prune anomaly; no invented oracle.
- [ ] Progress-driven `QUEST_WATCH_UPDATE`; no manual-list-change equivalent.
- [ ] Automatic and timed watches; no durations, expiry or seeding policy.
- [ ] Pre-existing generic `quest_verbs::set_abandon_quest_removes_selected_and_fires_removed` assumes destructive marking, contrary to the modeled Wrath contract. Leave unchanged in N3; see proof below.

## Out of scope

Other profiles and quest acceptance/turn-in/objective producers. Existing flat rich-log scope does not add header grouping/collapse production or cache lifecycle; ID lookup does not filter retained rows by visibility. The original header/collapse lifecycle is not certified by flat-row tests.

Source limits: laptop-relayed, independently verified source findings, not native execution captures. Runtime proof below remains distinct from this authority.

## Bounded proof — 2026-10-19

At test-only `b9368c270`, all twelve N3 cases genuinely fail on missing watch state/current-row identity, including original-consumer cardinality failures; every full-startup snapshot is exactly `[[],{}]`. Producer `8a3d5d0b3` rebases to `5d8663ca3` on `b88ef4c9e` (including N2); unchanged test/fixture patches precede it in both histories. Follow-up `ed84204e7` initializes the explicit seeded constructor's empty watch vector.

At code `ed84204e7`: **12/12 N3 PASS** (four source-derived manual contracts, eight unchanged original log/tracker cases), with no new direct/handler/simulator errors and all eight startup snapshots still `[[],{}]`. Source seals pin the full original Lua/XML files; no API or consumer replacements are installed. Deferred-abandon cases deliver only the producer's actually queued refresh through original listeners. This is bounded original-consumer execution, not native-client capture or header/collapse lifecycle proof.

The same executable runs **59/60 existing quest controls PASS**: rewards14, verbs12/13, original abandon14, alpha2, log projection6, objectives5 and offer selection6. The sole generic destructive-marking failure reproduces identically on both the retained pre-producer `b9368c270` executable and a fresh full-feature build at exact base `b88ef4c9e`: `assertion left == right failed`, left `[1, 2, 3]`, right `[1, 3]`. Its producer and test sources are byte-identical between base and N3 and unchanged from `66f280e63`. No existing Wrath-specific case regresses, and no test is disabled or rewritten to hide that baseline failure.

Stock current `wow-sim --no-addons --no-saved-vars lua-errors` exits 0 with `[]` (zero unique/total). Full `cargo check --no-default-features --features sound,gui,casc,client-wrath -j1`, formatting and manual Rust readability pass; compiler, harness and profile-check warnings are zero. Each runtime uses a fresh child-only byte copy of persisted CVars; original stat/hash stays unchanged. Sealed binaries, exact commands, admission/resource receipts and source-bound logs are retained in the private N3 handoff. These results do not claim a full suite or another profile.
