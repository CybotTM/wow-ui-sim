# Patch 12.0.7 API Audit

Patch 12.0.7 API work in wow-ui-sim separates safe additive compatibility bridges from security, taint, and secret-value behavior that must be proven with live Blizzard observations before implementation.

## Content

### Source scope

The audit source was the Warcraft Wiki Patch 12.0.7 API changes page, saved locally as `/tmp/warcraft_patch_12_0_7_api_changes.txt` during the audit. The page covers the 12.0.5 `(67602)` to 12.0.7 `(68182)` API diff and 12.0.7 blue-post notes.

### Completed compatible bridge work

The simulator now provides safe 12.0.7 additive probes for API names that can be inert without pretending to model live game state:

- `C_BattleNet.InviteFriend`
- `C_DelvesUI.GetDelveEntranceTitleString`
- `C_DurationUtil.CreateManualClock`
- `C_DurationUtil.CreateDurationTextBinding`
- `C_EncounterTimeline.GetEventColor`
- `C_HousingCatalog.GetCatalogCategoryAndSubcategoryNames`
- `C_HousingCustomizeMode.RoomConnectionSupportsDoorType`
- `C_HousingLayout.CanSetViewedFloor`
- `C_MerchantFrame.GetMerchantCurrencies`
- `C_PartyInfo.ConfirmReadyCheck`, `DemoteAssistant`, `DoReadyCheck`, `IsGUIDInGroup`, `PromoteToAssistant`, `PromoteToLeader`, `SetEveryoneIsAssistant`, `UninviteUnit`
- `C_PingSecure.ClearPendingPingOffScreenCallback`
- `C_QuestHub.GetDragonridingRacesForAreaPOI`
- `C_UIFileAsset.GetFileID`, `IsKnownFile`, `IsLooseFile`
- `GetEventCPUUsage`, `GetFunctionCPUUsage`, `GetScriptCPUUsage`
- secure pending callback getters/setters: button, ping off-screen, toggle run
- `GameTooltip_AddMoneyLine`
- `ENCOUNTER_TIMELINE_EVENT_COLOR_CHANGED` registration under `retail-12-0-7`

Already-existing coverage from prior work included `C_Container.CalculateTotalNumberOfFreeBagSlots`, `C_DelvesUI.GetWorldTierDifficultyForActivePlayer`, `C_PingSecure.SetPendingPingOffScreenCallback`, and `URL_TEXTURE_REQUEST_RESULT` registration.

Key implementation locations:

- `src/lua_api/workarounds/temporary/patch_12_0_7_inert_defaults.rs` — version-gated inert 12.0.7 defaults.
- `src/lua_api/workarounds/mod.rs`, `src/lua_api/workarounds/temporary/mod.rs` — bootstrap registration.
- `src/event/valid_events.rs` — 12.0.7 event registration gate.
- `src/loader/tests/wow_api_globals/startup_globals.rs` — regression test for safe 12.0.7 global bridges.

### Verification

Targeted proof logs:

- `/tmp/wow_12_0_7_target5_test-safe-12-0-7.out`
- `/tmp/wow_12_0_7_target5_test-events-12-0-7.out`

Full proof logs after the final bridge pass:

- `/tmp/wow_12_0_7_full_fmt-check.out`
- `/tmp/wow_12_0_7_full_check-default.out`
- `/tmp/wow_12_0_7_full_check-retail-12-0-7.out`
- `/tmp/wow_12_0_7_full_check-retail-12-1.out`
- `/tmp/wow_12_0_7_full_test-safe-12-0-7.out`
- `/tmp/wow_12_0_7_full_test-events-12-0-7.out`
- `/tmp/wow_12_0_7_full_test-safe-12-1.out`
- `/tmp/wow_12_0_7_full_build-retail-12-0-7.out`
- `/tmp/wow_12_0_7_full_lua-retail-12-0-7.out`

Rust readability metrics are under `/tmp/rust_readability_12_0_7` with no high-complexity findings.

### Paused / blocked items

Do not implement these as guesses. They need live Blizzard behavior, generated docs, or targeted probe addons:

- **Unit identity restricted-token behavior** — 12.0.7 changed restricted unit APIs such as `UnitGUID`, `UnitAura`, and health/power APIs from Lua errors to nil/default returns for unsupported PvP-restricted tokens. Need exact token matrix, return values, and addon-vs-Blizzard behavior.
- **Encounter payloads** — `ENCOUNTER_END` now includes `encounterUnitStatus` tables with `creatureID`, `creatureName`, and `remainingHealthPercent`. Need real event payload shape and simulator encounter state backing before modeling.
- **Encounter Events color state** — `C_EncounterEvents` gained color configuration and alpha support, plus `ENCOUNTER_TIMELINE_EVENT_COLOR_CHANGED`. Only timeline event color probing is bridged; persistent color state and event firing need live behavior.
- **SimulateMouse taint and focus restrictions** — 12.0.7 changed taint propagation and imposed forbidden/locked/script-inaccessible/protected focus restrictions. This overlaps secure input and combat lockdown; implement only after exact behavior is known.
- **debugstack/debuglocals secret propagation** — returning secret values based on current/caller stack secret access requires rilua secret-value semantics, not a simple Lua stub.
- **Secure `raidtarget` `set-unmarked` and `/tm ~N` behavior** — secure action and macro execution behavior needs real secure-state tests.
- **C_MythicPlus CalendarTime return structs** — `GetRunHistory`, `GetWeeklyBestForMap`, and `GetSeasonBestForMap` changed return struct shape. Need backing M+ data and exact CalendarTime fields.
- **GROUP_FORMED solo follower dungeon/delve behavior** — needs group/follower-dungeon state, not just a registerable event.
- **AuraData vehicle ownership and AddPrivateAuraAppliedSound allowance** — aura state/security behavior needs real aura model and M+ combat state.
- **SetFrameStrata secret-value error fix** — secret argument behavior needs live tests before changing method guards.
- **Button/scroll secret aspects and font asset validation** — widget change flags mention secret aspects and font validation. Existing methods remain compatible, but exact secret aspect and asset-validation semantics need probes.
- **Removed Minimap texture setters** — simulator method availability must be checked against active Blizzard sources before hiding anything; removing too early could break cached UI code.
- **Deprecated wrappers / removed globals** — 12.0.7 deprecates globals into namespaces. The simulator currently favors startup compatibility; strict removal timing should only change if current Blizzard UI no longer needs the legacy names.

### Practical next step

If the exact-behavior work resumes, create live PTR probe addons for restricted-unit returns, encounter payloads, SimulateMouse taint/focus restrictions, debug secret propagation, secure `raidtarget` actions, DurationTextBinding object semantics, and widget secret-aspect behavior.

## Sources

- `/tmp/warcraft_patch_12_0_7_api_changes.txt` — local snapshot of the patch API-change source.
- `src/lua_api/workarounds/temporary/patch_12_0_7_inert_defaults.rs` — implemented inert 12.0.7 bridge defaults.
- `src/event/valid_events.rs` — 12.0.7 event gate.
- `src/loader/tests/wow_api_globals/startup_globals.rs` — 12.0.7 safe bridge regression coverage.
- `docs/wiki/investigations/patch-12-1-api-audit.md` — adjacent 12.1 audit workflow and blocked-item pattern.

## See Also

- [[patch-12-1-api-audit]] — same audit pattern for Patch 12.1.
- [[client-profiles]] — retail epoch features used to gate patch-specific API surface.
- [[lua-api]] — Lua runtime surface and C API bridge context.
- [[event-system]] — event registration/dispatch behavior.
- [[taint-system]] — secure/taint behavior related to SimulateMouse, debug secret propagation, and secure actions.
