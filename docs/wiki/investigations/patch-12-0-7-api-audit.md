# Patch 12.0.7 API Audit

Patch 12.0.7 API work in wow-ui-sim separates safe additive compatibility bridges from security, taint, and secret-value behavior that must be proven with live Blizzard observations before implementation.

## Content

### Source scope

The audit source was the Warcraft Wiki Patch 12.0.7 API changes page, saved locally as `/tmp/warcraft_patch_12_0_7_api_changes.txt` during the audit. The page covers the 12.0.5 `(67602)` to 12.0.7 `(68182)` API diff and 12.0.7 blue-post notes.

### Completed compatible bridge work

The simulator now provides modeled 12.0.7 social/party mutations:

- `C_BattleNet.InviteFriend` appends a queryable offline Battle.net friend to `SimState.bnet_friends`, ignores empty/duplicate invites, and is visible through `GetNumFriends`/`GetFriendAccountInfo`.
- `C_PartyInfo.DoReadyCheck` / `ReadyCheck` start a ready-check state, dispatch `READY_CHECK`, and expose `GetReadyCheckStatus("player") == "waiting"` with positive `GetReadyCheckTimeLeft()`.
- `C_PartyInfo.ConfirmReadyCheck(ready)` records ready/not-ready state, dispatches `READY_CHECK_CONFIRM` and `READY_CHECK_FINISHED`, and clears the time-left value.
- `C_PartyInfo.IsGUIDInGroup(guid)` now reads the existing simulator party roster and synthetic `UnitGUID("partyN")` values instead of always returning false.
- `C_PartyInfo.SetEveryoneIsAssistant`, `PromoteToAssistant`, `DemoteAssistant`, and `PromoteToLeader` now mutate existing party assistant/leader state used by `IsEveryoneAssistant`, `IsGroupLeader`, and `UnitIsGroupLeader`.

The simulator also provides safe 12.0.7 additive probes for API names that can be inert without pretending to model live game state:
- `C_DelvesUI.GetDelveEntranceTitleString`
- `C_DurationUtil.CreateManualClock`
- `C_DurationUtil.CreateDurationTextBinding` (now best-effort tracks documented binding methods, enabled/default state, duration objects, formatter/text-format storage, and font-string update hooks)
- `C_EncounterTimeline.GetEventColor` (now best-effort delegated to `C_EncounterEvents` color state)
- `C_HousingCatalog.GetCatalogCategoryAndSubcategoryNames`
- `C_HousingCustomizeMode.RoomConnectionSupportsDoorType`
- `C_HousingLayout.CanSetViewedFloor`
- `C_MerchantFrame.GetMerchantCurrencies`
- `C_PartyInfo.ConfirmReadyCheck`, `DoReadyCheck`, `UninviteUnit`
- `C_PingSecure.ClearPendingPingOffScreenCallback`
- `C_QuestHub.GetDragonridingRacesForAreaPOI`
- `C_UIFileAsset.GetFileID`, `IsKnownFile`, `IsLooseFile` (now best-effort modeled from the bundled limited listfile)
- `GetEventCPUUsage`, `GetFunctionCPUUsage`, `GetScriptCPUUsage`
- secure pending callback getters/setters: button, ping off-screen, toggle run
- `GameTooltip_AddMoneyLine` (now best-effort formats copper through the simulator money formatter before adding the tooltip line)
- `ENCOUNTER_TIMELINE_EVENT_COLOR_CHANGED` registration under `retail-12-0-7`

Already-existing coverage from prior work included `C_Container.CalculateTotalNumberOfFreeBagSlots`, `C_DelvesUI.GetWorldTierDifficultyForActivePlayer`, `C_PingSecure.SetPendingPingOffScreenCallback`, and `URL_TEXTURE_REQUEST_RESULT` registration.

Key implementation locations:

- `src/c_api/c_battle_net.rs` — modeled `C_BattleNet.InviteFriend` backed by `SimState.bnet_friends`.
- `src/c_api/c_party_info.rs`, `src/lua_api/globals/group_verbs.rs`, `src/lua_api/state/support_types.rs` — modeled ready-check state, C_PartyInfo ready-check methods, GUID-in-group membership, party assistant/leader mutators, global ready-check status/time probes, and immediate ready-check event dispatch.
- `src/c_api/c_ui_file_asset.rs` — best-effort `C_UIFileAsset` path/fileDataID lookup backed by the bundled limited listfile.
- `src/lua_api/workarounds/temporary/patch_12_0_7_inert_defaults.rs` — version-gated inert 12.0.7 defaults for still-unmodeled APIs.
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

### Best-effort modeled guesses needing later probes

- **C_UIFileAsset path semantics** — `GetFileID` and `IsKnownFile` are backed by `data/wow-ui-sim-listfile.csv` through `limited_listfile`, with slash/case normalization and numeric IDs passed through. `IsLooseFile` currently returns false because loose-file install/source semantics are not modeled. Replace this with exact client behavior if PTR probes show different extension handling or loose-file rules.
- **Encounter timeline color state** — `C_EncounterTimeline.GetEventColor` mirrors the existing temporary `C_EncounterEvents` color table, including alpha, and falls back to white when no event color is configured. Exact five-second-warning/custom-color behavior and event firing still need probes.
- **DurationTextBinding formatting** — the binding object now follows the documented method surface for non-secret state and stores duration objects from `C_DurationUtil.CreateDuration`, but exact Blizzard formatting/component semantics and secret-value handling still need live probes.
- **Tooltip money line formatting** — `GameTooltip_AddMoneyLine` uses the existing `GetMoneyString` fallback with thousands grouping and optional prefix text. Exact embedded-atlas/MoneyFormatter output remains a later fidelity improvement if addon screenshots require it.
- **Party GUID membership** — `C_PartyInfo.IsGUIDInGroup` treats the local player and synthetic party member GUIDs as in-group only while `SimState.party_group_active` is true. Exact instance-party category filtering and cross-realm GUID details remain future fidelity work if addons depend on them.
- **Party role mutators** — `C_PartyInfo` leader/assistant mutators write the existing simulator group-role fields. Individual assistant tracking is not modeled yet, so `PromoteToAssistant` and `DemoteAssistant` map to the coarse `everyone_assistant` flag until a per-member assistant model is needed.

### Paused / blocked items

Security/error-shape-sensitive items still need live Blizzard behavior, generated docs, or targeted probe addons:

- **Unit identity restricted-token behavior** — 12.0.7 changed restricted unit APIs such as `UnitGUID`, `UnitAura`, and health/power APIs from Lua errors to nil/default returns for unsupported PvP-restricted tokens. Need exact token matrix, return values, and addon-vs-Blizzard behavior.
- **Encounter payloads** — `ENCOUNTER_END` now includes `encounterUnitStatus` tables with `creatureID`, `creatureName`, and `remainingHealthPercent`. Need real event payload shape and simulator encounter state backing before modeling.
- **Encounter Events color event semantics** — `C_EncounterEvents` color state and timeline color reads are best-effort bridged, but exact five-second-warning custom-color behavior, persistence rules, and `ENCOUNTER_TIMELINE_EVENT_COLOR_CHANGED` firing need live behavior.
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
- `src/c_api/c_battle_net.rs` — modeled Battle.net friend-list APIs.
- `src/c_api/c_party_info.rs`, `src/lua_api/globals/group_verbs.rs`, `src/lua_api/state/support_types.rs` — modeled ready-check behavior.
- `src/c_api/c_ui_file_asset.rs` — best-effort UI file-asset lookup.
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
