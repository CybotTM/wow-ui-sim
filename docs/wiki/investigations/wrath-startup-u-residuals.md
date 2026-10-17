# Wrath startup boundaries and error-display exposure

Original cached Lua is the unchanged consumer for legacy publishers and startup error reporting. The [u-resid proof](../../specs/wrath-startup-capability-defaults.md#final-bounded-proof--2026-10-17) is historical; the [combined loot/party proof](../../specs/wrath-initial-loot.md#combined-proof--2026-10-17) and [current failing inventory](../../baselines/README.md#wrath-lua-errors) own current acceptance and exact counts. Simulator input policies are not native-parity evidence.

## Producer ownership

Calendar adapters project the existing temporary date and empty invitations rather than inventing another clock. Friend counts project existing C_FriendList state. Wrath combo counts reuse the existing target-bound secondary pool, leaving Retail player-owned behavior separate. The absent map-overlay catalog has a Wrath-only temporary zero count and explicit retirement condition. Loot/party support owns the state-backed legacy readers and the shared C_RecruitAFriend cooldown snapshot; its contract distinguishes temporary defaults from supplied state.

## Four historical nil-string occurrences

The sealed diagnostic scratch loads the separate Wrath FrameXML TOC, then original startup addons and `collect_lua_error_startup`. The three deferred ticks matter: login alone produces only one occurrence, not the startup command's four.

All four stacks contain `Blizzard_CombatLog.lua:2841` in `CombatLog_OnEvent` and its caller at :745. Original :2841 executes `format("ACTION_%s_FULL_TEXT", event)` with nil event because the temporary current-entry getter returned scalar cursor zero, truthy in Lua. The first call is the login refilter; the remaining three are `CombatLogUpdateFrame` OnUpdate callbacks originating at :738. The shared empty-history repair makes absent entries falsy and empty advance invalid; it does not select populated replay policy. Do not attribute these to an unidentified native string consumer or to `strsub`.

## Generated chunk identities

Original-closure `debug.getinfo` established the following identities on the old VM, not a general bytecode-instruction source map:

- `generated/loader-exec-no-global-slots/ac261bb011cbb2cd`: `VideoOptionsEffectsPanelTextureResolution.GetCurrentValue`, original VideoOptionsPanels.xml:685–689. It subtracts the absent BaseMip value at :688. Explicit BaseMip test input is not a default oracle.
- `generated/loader-exec-no-global-slots/b3d87b1c6f9a6505`: GammaSlider OnValueChanged, original VideoOptionsPanels.xml:384–389. It calls absent `SetGamma(value)` at :387. No gamma-setting/host-display policy is selected.

The [re-pin disclosure](../../baselines/README.md#rilua-re-pin-baseline-refresh--2026-10-17) owns the new generated IDs and extra host-frame changes. Normalize generated identities for comparison, not the committed raw stdout.

## Error display after loot/sound repair

With the earlier loot/sound aborts fixed, the first selected error is Gamma. While handling the following BaseMip failure, unchanged `Blizzard_DebugTools.lua:449` takes its unchanged-text branch: `ScriptErrorsFrame_Update` → `ScrollingEdit_OnTextChanged` at UIPanelTemplates.lua:342 → `ScrollingEdit_OnUpdate` at :365. The observed `ScriptErrorsFrameScrollFrameText` has `handleCursorChange=true`, but `cursorOffset` and `cursorHeight` are nil. This occurs with the font system installed, not just in the earlier fontless fixture's secondary error path.

A read-only line hook reproduces the fresh CLI's exact raw error messages/counts and captures that call chain. A private counterfactual using the same sealed simulator library, VM and current CVar bytes restores only the pre-fix `personalloot` token and numeric-only simulator sound binding. It reproduces the prior frontier without the cursor error, with initialized cursor fields at every observed read. This demonstrates joint earlier-abort exposure; it does not isolate which restoration initializes cursor state or fix the missing initial notification. No production/vendor handler or EditBox behavior is changed.

`lua-errors` aggregates by normalized first line and retains one representative stack. Its numeric sound aggregate contains distinct TargetFrame and CombatLog callers; do not attribute every occurrence to the representative TargetFrame stack. The [inventory disclosure](../../baselines/README.md#wrath-lua-errors) owns raw caller counts, retained-record deltas and the counterfactual receipt. Private evidence lives in `~/.local/state/wow-ui-sim/handoff/wrath-sound-loot/`: `cursor-startup-attribution.json`, `prior-abort-raw.json`, and `wrath-combined-classification.json`.

## Approval boundaries

Main extended the standing empty/zero/no-selection policy to existing-model publication, no-target/unsupplied counts and shared empty history. The user separately approved HasDebugZoneMap=false, the former unsupplied UnitIsTalking=false policy, and IsPlayerResolutionAvailable=true on 2026-10-17. [Loot/party support](../../specs/wrath-initial-loot.md) now replaces the constant speaking probe with existing named-member activity while preserving unsupplied/binary controls. Empty debug-object/floor counts never infer a total from the selected-floor scalar. BaseMip defaults, gamma operations, populated combat-log replay and the cursor fix remain outside this work. The U1/u1-geometry approvals concern the unrelated Retail tooltip divider.

## Sources

- [Calendar contract](../../specs/wrath-legacy-calendar.md) and [friend count contract](../../specs/wrath-legacy-friend-count.md)
- [Combo contract](../../specs/wrath-combo-points.md) and [shared empty history](../../specs/classic-empty-combatlog-history.md)
- [Empty overlays](../../specs/wrath-empty-map-overlays.md) and [startup defaults](../../specs/wrath-startup-capability-defaults.md)
- [Loot/party support](../../specs/wrath-initial-loot.md)
- [Current failing inventory](../../baselines/README.md#wrath-lua-errors)

## See Also

- [[forever-combo-points]] — existing target-bound pool and ownership policy
- [[client-profiles]] — cache and publication boundaries
