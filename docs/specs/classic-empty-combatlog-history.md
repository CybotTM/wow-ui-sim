# Classic empty combat-log history

Era/Anniversary/Wrath public history queries must not turn an empty cursor into a fabricated event. Existing temporary fixture ownership is `src/lua_api/workarounds/temporary/combat_log_state.rs`; no second history model is introduced. See [Lua API](../lua-api.md).

## What it must do

- [x] Empty `C_CombatLog.GetCurrentEntryInfo()` returns no values (therefore nil in the original validity check), regardless of the stored cursor number.
- [x] Empty `SeekToNewestEntry` and `SeekToPreviousEntry` return false. Cached `Blizzard_APIDocumentationGenerated/CombatLogDocumentation.lua:109-137` declares boolean validity for both seek methods.
- [x] Original Vanilla UIParent PLAYER_LOGIN loads the real CombatLog and completes its empty refilter without passing nil event to `strsub`.
- [ ] Preserve non-admitted profiles' existing public scalar contract and secure fixture behavior; no populated replay contract is selected.
- [x] Wrath's original login and three lua-errors update ticks must not format an empty cursor as an event; explicit refilter clears stale messages, completes, and removes its OnUpdate callback.
- [x] Wrath legacy CombatLogAdvanceEntry returns false on empty history, including a stale scalar cursor. Era/Anniversary share this path; fresh Era coverage is pending.

## How it works

- [Lua API](../lua-api.md)
- [A acceptance boundaries](classic-startup-a.md)

## Implementation inventory

- `src/lua_api/workarounds/temporary/combat_log_state.rs`: selected-profile empty validity/advance repair in the existing fixture.

## Tests asserting this spec

- `tests/classic_combatlog_startup.rs`: Era/Anniversary original cache/TOC startup and startup events; empty query, stale cursor 37, original refilter completion and OnUpdate removal.
- `tests/wrath_unmasked_combatlog.rs`: Wrath's separate original FrameXML TOC, addon discovery, login and deferred refilter ticks; same shared empty validity, stale cursor, original refilter output/completion and legacy advance.

## Known gaps (current cycle)

- [x] Wrath original-consumer RED reproduces all four formatter occurrences (login + three deferred ticks). d11a25154 sealed Wrath43 GREEN completes original refilter, clears stale messages and removes OnUpdate; startup removes the normalized record/count4. The overall startup is still failing at18/24.

- [x] Original-login empty-history test GREEN in both profiles on production `0c6636255`; exact receipts in the [A ledger](classic-startup-a.md#rebased-acceptance-after-combatlog-rejection). Both startups report 36 unique / 38 occurrences, with the string.sub pair removed; only the disclosed marker-active-state pair is new after StoreUI-ID normalization.
- [ ] Progress-bar visibility is not asserted: real secure startup does not expose that frame through the tested public globals or `getfenv` path. Parent approved removing only this lookup assertion; startup error rejection, empty queries, refilter completion and OnUpdate removal remain mandatory.
- [ ] Populated history remains policy-blocked. Handoff `stage-wrath-unmasked/README.md` U4 and `stage-wrath-unmasked/U4-combatlog/POLICY.md` explicitly leave row order, cursor-zero semantics, tuple schema/arity and populated replay undecided. This fix makes none of those choices. Retire this temporary path when the authorized history model exists.

## Attribution and grounding

R3 quality/count publication permits the original `Blizzard_UIParent/Vanilla/UIParent.lua:24-29` color loop to finish, so `UIParent_OnEvent` is defined and PLAYER_LOGIN at :381 calls `CombatLog_LoadUI`. Original `Blizzard_CombatLog/Classic/Blizzard_CombatLog.lua:602-610` uses `GetCurrentEntryInfo` first as validity, then as the formatter's event tuple. The fixture instead returned cursor zero, truthy in Lua; `event` becomes nil at the formatter's :1705 `strsub(event, 1, 5)`. This is an unmasked producer defect, not a vendor bug.

Cached `CombatLogDocumentation.lua:49-57` declares `GetCurrentEntryInfo` with an empty Returns list, not an explicit event schema or native nil-count measurement. Returning no values for absent history follows the original consumer's falsy-entry boundary; no native populated tuple is inferred. U4 now admits Wrath at the same shared fixture: its legacy getter aliases the admitted namespace getter, so both reject empty cursor values. Non-admitted profiles retain their existing scalar fixture. Wrath original `Blizzard_CombatLog.lua:745` passes `CombatLogGetCurrentEntry()` to `CombatLog_OnEvent`; its formatter at :2841 previously received nil `event` and called `format("ACTION_%s_FULL_TEXT", event)`. Sealed original-consumer RED reproduces the initial login failure and three deferred OnUpdate occurrences. No populated tuple is selected.

## Out of scope

Populated replay, fabricated events, cursor/traversal policy, complete C API history model, native parity, suppression/removal of startup error records, and Blizzard Lua edits.
