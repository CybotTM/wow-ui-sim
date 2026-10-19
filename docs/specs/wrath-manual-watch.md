# Wrath manual quest watch

Wrath build-12340 manual watch calls project current rich quest-log rows into the unchanged original quest log and watch frame. The legacy Lua surface owns this seam, not the Forever quest-ID API. See [Wrath legacy state](../wiki/systems/lua-api.md#wrath-selected-quest-legacy-surface) and [abandon response](wrath-quest-abandon.md).

## What it must do

- [ ] Start with no implicit watches. Store quest identity separately from current log row; retain append order for manual sorting, preserve survivor order on removal, and enforce 25 watches with a silent 26th-add no-op. Duplicate add and absent remove are inert.
- [ ] `AddQuestWatch`/`RemoveQuestWatch` take a numeric-convertible current log row, return zero values and emit no direct events. Missing/non-numeric input uses the selected active identity; explicit invalid/header rows do nothing. No modern watch-list-change event is introduced.
- [ ] `GetNumQuestWatches` returns one count. `IsQuestWatched` returns exactly one `1`/nil; `GetQuestIndexForWatch` returns exactly one current row/nil. Both queries reject non-numeric-convertible input with their `Usage: NAME(index)` error. Lookup scans all supplied rows by ID without a visibility filter; log reorder does not resort watches.
- [ ] Abandon keeps watch membership through the immediate unit-change notification. The existing deferred/coalesced argumentless `QUEST_LOG_UPDATE` prunes missing identities before listeners consume the rebuilt projection, compacting survivors in watch order. No additional abandon producer or watch event is introduced.
- [ ] Unchanged original shifted log clicks add/remove watches, shifted tracker clicks remove them, and actual check textures/visible title FontStrings agree after reorder and deferred abandon. Exact startup-error snapshot stays unchanged; consumers add no errors and never clear existing errors.

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

- [ ] Prove GREEN, existing Wrath quest preservation, formatting/readability and zero-warning Wrath check. Genuine missing-state RED is recorded for all twelve tests; startup snapshot is exactly `[[],{}]`.

## Out of scope

- Sort modes 1–3 and manual watch shifting; manual append-order seam only.
- Cold-cache omission and multi-missing prune anomaly; no invented oracle.
- Progress-driven `QUEST_WATCH_UPDATE`; no manual-list-change equivalent.
- Automatic and timed watches; no durations, expiry or seeding policy.
- Other profiles and quest acceptance/turn-in/objective producers. Existing flat rich-log scope does not add header grouping/collapse production or cache lifecycle; ID lookup must not filter retained rows by visibility.

Source limits: laptop-relayed, independently verified source findings, not native execution captures. Runtime proof below must remain distinct from this authority.
