# Classic empty combat-log history

Era/Anniversary public history queries must not turn an empty cursor into a fabricated event. Existing temporary fixture ownership is `src/lua_api/workarounds/temporary/combat_log_state.rs`; no second history model is introduced. See [Lua API](../lua-api.md).

## What it must do

- [ ] Empty `C_CombatLog.GetCurrentEntryInfo()` returns no values (therefore nil in the original validity check), regardless of the stored cursor number.
- [ ] Empty `SeekToNewestEntry` and `SeekToPreviousEntry` return false. Cached `Blizzard_APIDocumentationGenerated/CombatLogDocumentation.lua:109-137` declares boolean validity for both seek methods.
- [ ] Original Vanilla UIParent PLAYER_LOGIN loads the real CombatLog and completes its empty refilter without passing nil event to `strsub`.
- [ ] Preserve other profiles' existing public scalar contract and secure fixture behavior; no populated replay contract is selected.

## How it works

- [Lua API](../lua-api.md)
- [A acceptance boundaries](classic-startup-a.md)

## Implementation inventory

- `src/lua_api/workarounds/temporary/combat_log_state.rs`: selected-profile empty validity/advance repair in the existing fixture.

## Tests asserting this spec

- `tests/classic_combatlog_startup.rs`: original cache/TOC startup and startup events; empty query, stale cursor 37, original refilter completion and OnUpdate removal.

## Known gaps (current cycle)

- [ ] GREEN/preservation/startup reconciliation pending.
- [ ] Populated history remains policy-blocked. Handoff `stage-wrath-unmasked/README.md` U4 and `stage-wrath-unmasked/U4-combatlog/POLICY.md` explicitly leave row order, cursor-zero semantics, tuple schema/arity and populated replay undecided. This fix makes none of those choices. Retire this temporary path when the authorized history model exists.

## Attribution and grounding

R3 quality/count publication permits the original `Blizzard_UIParent/Vanilla/UIParent.lua:24-29` color loop to finish, so `UIParent_OnEvent` is defined and PLAYER_LOGIN at :381 calls `CombatLog_LoadUI`. Original `Blizzard_CombatLog/Classic/Blizzard_CombatLog.lua:602-610` uses `GetCurrentEntryInfo` first as validity, then as the formatter's event tuple. The fixture instead returned cursor zero, truthy in Lua; `event` becomes nil at the formatter's :1705 `strsub(event, 1, 5)`. This is an unmasked producer defect, not a vendor bug.

Cached `CombatLogDocumentation.lua:49-57` declares `GetCurrentEntryInfo` with an empty Returns list, not an explicit event schema or native nil-count measurement. Returning no values for absent history follows the original consumer's falsy-entry boundary; no native populated tuple is inferred. The existing U4 Wrath proposal repairs its legacy-global validity/advance at the same shared fixture and leaves the modern scalar intact. Era/Anniversary need the public namespace route; U4 must retain its distinct legacy route when integrated.

## Out of scope

Populated replay, fabricated events, cursor/traversal policy, complete C API history model, native parity, suppression/removal of startup error records, and Blizzard Lua edits.
