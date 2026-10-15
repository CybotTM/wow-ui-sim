# Forever quest difficulty query

Forever's `C_QuestLog.GetQuestDifficultyLevel` must expose the supplied rich quest-log row's `level`. Implementation belongs to `src/c_api/c_quest_log.rs`; [quest loading context](../addon-loading-pipeline.md). This equality and fail-fast missing-input policy are bounded simulator assumptions, not claims about native behavior.

## What it must do

- [x] Publish a raw callable `GetQuestDifficultyLevel` through the Forever modeled registration.
- [x] Return exactly one number from the exact current environment's `quest_log_entries.entries` row matching `quest_id`; observe non-80 live mutations and never consult a static catalog or fabricate a level.
- [x] Fail explicitly with API/questID context for nonnumeric, nonfinite, fractional, out-of-i32-range or unrepresented identities and for removed rows. Missing valid integer identities include that ID in the error.
- [x] Support unchanged whole cached Mainline DifficultyUtil plus Camelot QuestUtilsOverrides with explicit test fixtures for elite status, CVar selection and color dependencies; produce concrete ordinary/elite, colored/plain title prefixes.
- [ ] Preserve other profiles' existing surface and Retail/PTR `QuestCanHaveWarModeBonus` behavior in runtime verification.

## Ordinary-watch coherence follow-up

The existing simulator policy watches every ordinary quest. Forever's count and indexed IDs now project that policy from current rich rows, excluding `is_world_quest` rows and preserving row order; this is not native watch-subset parity. Source704's normal Lua RED enumerated80000/80001/80002: the first two resolved to log rows and difficulty80, while80002 had no rich row. New source removes the stale watch producer, not the strict missing-level error.

- [ ] Project `C_QuestLog.GetNumQuestWatches` and `GetQuestIDForQuestWatchIndex` from the same current ordinary-row sequence; global `GetNumQuestWatches` matches the count. Seed IDs are80000/80001, not static-only80002.
- [ ] Observe identity/level mutation, insertion, removal, reorder and world-flag changes immediately; fresh environments remain independent.
- [ ] Preserve existing watch-index conversion and arity: nil/missing index means zero, numeric conversion truncates/saturates before bounds checking, valid IDs return one number, misses zero results, nonnumbers error, extra arguments ignored.
- [ ] Resolve every returned ID through current `GetLogIndexForQuestID`, `GetInfo`, `IsOnQuest` and difficulty metadata without a static fallback or invented rich record.

Four authored `tests/wowforever_quest_watch_projection.rs` cases cover these requirements; all UNEXECUTED. The later legacy registration omits these three replaced Forever writes, so fresh publication and cleanup restoration use the same C API producers; other profiles retain their old registrations. Watch membership/mutators, legacy index helpers, world-watch projection and broader quest query reconciliation remain parent gaps to investigate where actual callers require them; this correction does not mark them complete or exclude them from the parent.

## How it works

- [C API boundary](../../AGENTS.md#c-api-boundary)
- [Lua API system](../wiki/systems/lua-api.md)

## Implementation inventory

- `src/c_api/c_quest_log.rs`: supplied-state lookup and Forever registration; existing Retail/PTR eligibility remains separate.
- `src/c_api/mod.rs`: narrowly extends the module and utility call gate to Forever.
- `src/lua_api/globals/quest_surface/{register,handlers}.rs`: omit replaced static watch handlers only on Forever, preserving initial modeled publication.
- `tests/wowforever_quest_difficulty.rs`: fresh-environment and unchanged whole-consumer behavior assertions.
- `tests/wowforever_quest_watch_projection.rs`: ordinary-watch/live rich-state coherence, conversion and isolation assertions.

## Tests asserting this spec

`tests/wowforever_quest_difficulty.rs`: all three PASS at runtime source `70459ec5a`; [central receipt](../baselines/README.md#current-providerquest-execution--2026-10-15) owns build/seals, normal probes and failing startup. Original sealed Lua REDs precede this producer; no authored Rust RED is claimed.

- `forever_quest_difficulty_raw_publication_reads_seed_and_live_state`: all four seed IDs, exact one-result raw endpoint, level37 mutation, new identity456789/level23 and independent fresh seed.
- `forever_quest_difficulty_rejects_invalid_and_unrepresented_ids_explicitly`: invalid input shapes, absent IDs, removed row, useful errors; 80002 is deliberately absent from rich state despite the older static quest catalog.
- `forever_quest_difficulty_whole_vendor_title_consumer_uses_live_level`: exact `[37] ` / `[37+] ` prefixes, `|cff336699` fixture wrapping, CVar disabled plain output.

## Known gaps (current cycle)

- [ ] Source704 startup requests quest80002, present in an older static catalog but absent from rich state; the explicit missing-record error is captured, not suppressed. The ordinary-watch producer correction has original Lua RED but no new Rust/normal-startup GREEN yet.
- [ ] Other-profile preservation, default check, native parity and whole-startup acceptance remain open. Current no-run compilation, `cargo fmt --check`, three model tests, four-known-row normal coherence and fixture-controlled whole-consumer probe passed; these do not close the missing80002 case.

## Out of scope

Native unknown-ID/error/secrecy parity; defining a separate difficulty-level model; changing vendor code; new globals, missing-surface shims, retail epochs or build configuration. Forever follows ordinary numeric validation; the Retail/PTR-only secret-unwrapping branch is unchanged, not promoted into a new Forever security policy.
