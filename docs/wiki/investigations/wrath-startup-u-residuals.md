# Wrath calendar, friends and empty-history startup boundaries

The deferred Wrath queue exposes missing legacy publishers and an empty-history validity defect. Original cached Lua is the unchanged consumer; the bounded simulator policies are not native-parity evidence. The [final bounded gate](../../specs/wrath-startup-capability-defaults.md#final-bounded-proof--2026-10-17) passes 48 named Wrath cases; startup drops from 22/30 to 15/21 with no new records, but still fails. Retail/Forever startup preservation remains separately bounded.

## Producer ownership

Calendar adapters project the existing temporary date and empty invitations rather than inventing another clock. Friend counts project existing C_FriendList state. Wrath combo counts reuse the existing target-bound secondary pool, leaving Retail player-owned behavior separate. The absent map-overlay catalog has a Wrath-only temporary zero count and explicit retirement condition.

## Four nil-string occurrences

The sealed diagnostic scratch executable loads the separate Wrath FrameXML TOC, then original startup addons and `collect_lua_error_startup`. The three deferred ticks matter: login alone produces only one occurrence, not the startup command's four.

All four stacks contain `Blizzard_CombatLog.lua:2841` in `CombatLog_OnEvent` and its caller at :745. Original :2841 executes `format("ACTION_%s_FULL_TEXT", event)` with nil event because the temporary current-entry getter returned scalar cursor zero, truthy in Lua. The first call is the login refilter; the remaining three are `CombatLogUpdateFrame` OnUpdate callbacks originating at :738. The shared empty-history repair makes absent entries falsy and empty advance invalid; it does not select populated replay policy. Do not attribute these to an unidentified native string consumer or to `strsub`.

## Two generated chunk identities

Runtime `debug.getinfo` on the original closures establishes source identity, not a general bytecode-instruction source map:

- `generated/loader-exec-no-global-slots/ac261bb011cbb2cd`: `VideoOptionsEffectsPanelTextureResolution.GetCurrentValue`, original VideoOptionsPanels.xml:685–689. It subtracts the absent BaseMip value at :688. Explicit BaseMip test input is not a default oracle.
- `generated/loader-exec-no-global-slots/b3d87b1c6f9a6505`: GammaSlider OnValueChanged, original VideoOptionsPanels.xml:384–389. It calls absent `SetGamma(value)` at :387. No gamma-setting/host-display policy is selected.

## Approval boundaries

Main explicitly extended the documented empty/zero/no-selection policy to existing-model publication, no-target/unsupplied counts and shared empty history. The user separately approved HasDebugZoneMap=false, UnitIsTalking=false for any name string, and IsPlayerResolutionAvailable=true on 2026-10-17; [their owning contract](../../specs/wrath-startup-capability-defaults.md) distinguishes temporary UI policy from native capability. Empty debug-object/floor counts use the standing zero-when-unsupplied policy, never a total inferred from a selected-floor scalar. BaseMip, gamma operations and populated combat-log replay remain blocked. The approvals-packet U1 and u1-geometry receipt concern the unrelated Retail tooltip divider.

## Sources

- [Calendar contract](../../specs/wrath-legacy-calendar.md)
- [Friend count contract](../../specs/wrath-legacy-friend-count.md)
- [Combo contract](../../specs/wrath-combo-points.md)
- [Empty overlay contract](../../specs/wrath-empty-map-overlays.md)
- [Startup capability/default contract](../../specs/wrath-startup-capability-defaults.md)
- [Shared empty-history contract](../../specs/classic-empty-combatlog-history.md)

## See Also

- [[forever-combo-points]] — existing target-bound pool and ownership policy
- [[client-profiles]] — cache and publication boundaries
