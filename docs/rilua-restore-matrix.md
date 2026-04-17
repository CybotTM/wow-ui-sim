# Rilua Migration — Restore Matrix

The rilua migration dropped a large master-era compatibility-shim surface
instead of porting it. This file inventories the deleted master files and the
global / namespace names they registered, so the restore work can proceed by
subsystem instead of warning-by-warning.

**Merge base:** `f165d64634` (last common ancestor of `rilua-migration` and `master`).

**Methodology:**
- `git log --diff-filter=D --name-only master..HEAD -- src/lua_api/globals/` lists deleted master-era modules.
- For each, `git show f165d64634:<path>` was grepped for `.set("Name"...)` calls to recover the registered surface.
- State-of-port decided by spot-checking: is a name still registered on HEAD (via a rilua-era module or `stubs/*.rs`), or is it missing?

Tri-state per function:
- **ported** — already registered on HEAD with real (or at least named) implementation.
- **stubbed** — registered via `stubs/{global,namespace}_stubs.rs` with a constant-return placeholder. Upgrade to real impl is a separate PLAN task.
- **missing** — not registered on HEAD at all. Addons that call this name see nil and fall into error handlers.

## Process for each subsystem

1. Start from the master file (`git show f165d64634:src/lua_api/globals/<file>.rs`) and harvest the full registered surface.
2. `rg '"Name"'` on HEAD to check which subset is already wired.
3. Move ported items to the rilua-era owner module, stubbed items get a task to upgrade, missing items become a new registrar.
4. Do this one subsystem at a time — do NOT open all 200+ names simultaneously.

---

## Subsystem inventory (30 highest-surface modules)

Register-call counts from master. Names show the full set each file touched;
many are state fields (lowercase) mixed with globals (CamelCase). Focus on the
CamelCase entries when deciding what a restore owes.

### player_api.rs (60 set-calls)

Movement / combat stats:
`Ambiguate AreTalentsLocked BNConnected BNFeaturesEnabled BNGetInfo GetAvoidance GetBlockChance GetBonusBarIndex GetCursorMoney GetDodgeChance GetHaste GetLifesteal GetMeleeHaste GetNumBuybackItems GetNumTitles GetParryChance GetPlayerFacing GetSheathState GetShieldBlock GetSpeed IsAccountSecured IsDrivableArea IsFalling IsFlyableArea IsFlying IsMounted IsOutOfBounds IsPlayerMoving IsSubmerged IsSwimming IsXPUserDisabled PetUsesPetFrame` — most are nil-stubbed on HEAD; the movement probes partly map to `MovementState` but the stat accessors (`GetAvoidance`, `GetBlockChance`, …) need a player-stats SimState struct.

### c_misc_api_game.rs (51 set-calls)

`C_AdventureJournal C_ArtifactUI C_AzeriteEmpoweredItem C_AzeriteItem C_CatalogShop C_Commentator C_CraftingOrders C_EquipmentSet C_ExternalEventURL C_Garrison C_ItemUpgrade C_SplashScreen C_StorePublic C_SummonInfo C_UI CanBeShown ExpansionLandingPage GameRulesUtil GetArtifactItemID GetArtifactTier GetGameTime GetMode GetPrimaryOffset GetSummonReason HasNewMail HasURL IsAtForge IsEnabled IsNew IsOverlayApplied IsShop2Enabled IsSpectating Kiosk LaunchURL MinimapUtil ShouldShowAddOns UpdateSuggestions`

### system_api_runtime.rs (33 set-calls)

`AnimateCallout AnimateMouse GetCallstackHeight GetTime IsAltKeyDown IsControlKeyDown IsModifierKeyDown IsPlayerInRPE IsShiftKeyDown OnLoad RequestTimePlayed Start Stop` — modifier keys are ported to `modifier_keys.rs`; the `AnimateCallout` / `AnimateMouse` / `WowStyle1DropdownMixin` table stubs are MISSING and likely drive nil-index errors in Blizzard UI. Restore them as pass-through mixins (`Start`, `Stop`, `OnLoad` no-ops).

### c_misc_api_ui.rs (30 set-calls)

`C_AreaPoiInfo C_Calendar C_ContributionCollector C_CovenantCallings C_GameRules C_Glue C_GossipInfo C_MajorFactions C_PlayerChoice C_Scenario C_ScriptedAnimations C_UIWidgetManager C_VignetteInfo C_WeeklyRewards CloseCalendar CloseGossip ForceGossip GetActiveGameMode GetMonthInfo GetNumActiveQuests GetNumOptions GetState GetText IsHardcoreActive IsInScenario IsStandard IsWoWHack OpenCalendar RequestCallings SetMonth`

### c_stubs_api_namespaces.rs (25 set-calls)

`C_ConfigurationWarnings C_IncomingSummon C_LobbyMatchmakerInfo C_PerksActivities C_PlayerMentorship C_RecentAllies C_SharedCharacterServices C_SocialQueue C_SocialRestrictions C_SpectatingUI C_StoreGlue C_VideoOptions CanReceiveChat GetChannelList IsChatDisabled IsInQueue IsMuted IsParticipating IsPartyLFG IsPartyWorldPVP IsSilenced IsSpectating IsSquelched IsSystemEnabled IsTargetLoose`

### c_stubs_api_missing.rs (23 set-calls)

`ActionBarType ActionButtonUtil C_SecureTransfer DAMAGER GetDefaultScale GetHonorLevel GetLFGProposal GetSendMailPrice GetWebTicket HasBonusActionBar HEALER LE_PARTY_CATEGORY_HOME LE_PARTY_CATEGORY_INSTANCE Normal NOROLE Override Possess PutItemInBackpack PutItemInBag RequestRaidInfo ResetCursor TANK UpdateMicroButtons`

### c_map_api.rs (22 set-calls)

`C_DateAndTime C_DeathInfo C_FogOfWar C_InvasionInfo C_Map C_MapExplorationInfo C_Minimap C_Navigation C_TaxiMap GetAreaInfo GetCurrentMapID GetMapArtID GetMapArtLayers GetMapGroupID GetMapInfo UiMapPoint` — WorldMap-heavy surface, needs `SimState.map` with seeded map IDs (Stormwind + continents).

### c_quest_api.rs (21 set-calls)

`C_QuestLine C_QuestLog C_QuestOffer C_QuestSession C_TaskQuest GetAbandonQuestName GetAvailableWorldQuestsByMapID GetClassName GetCompleteQuestOfferRewardInfo GetLockedTitleDescription GetNumQuestPOIWorldEffects GetQuestInfo GetQuestLogIndexByID GetQuestOfferInfoForQuestID GetRaceName GetSpecializationRoleByID GetSpellInfo` — needs a real `SimState.quest_log`.

### c_system_api.rs (19 set-calls)

`C_System GetFrameStack debugprofilestop ForEachFrame GetBuildInfo GetCVarBitfield GetFramerate GetItemRawIDFromGUID GetLatency GetLocale GetMacroIconInfo GetNetStats GetOSVersion HasAlternateForm IsAttunedZone IsGMClient IsOnGlueScreen SetPortraitTextureFromCreatureDisplayID SetPortraitToTexture UnitFullName` — some ported (`GetNetStats`, `HasAlternateForm` maps to player_info); the rest need restoring.

### c_stubs_api_combat.rs (17 set-calls)

`__activeOutfitID C_ColorUtil C_CombatAudioAlert C_CombatText C_DamageMeter C_DeathRecap C_HousingPhotoSharing ClearAuthorization C_RestrictedActions C_TransmogOutfitInfo __currentlyViewedOutfitID GetCropRatio GetSpeakerSpeed HasRecapEvents IsAuthorized IsEnabled NamePlateConstants __pendingSheatheCategories sheatheCategory SpeakText`

### c_misc_api_core.rs (17 set-calls)

`C_DateAndTime C_LFGInfo C_MythicPlus C_ScenarioInfo C_TradeSkillUI GetBaseProfessionInfo GetChildProfessionInfo GetCriteriaInfo GetCurrentAffixes GetCurrentSeason GetProfessions GetRunHistory GetScenarioInfo IsInScenario IsNPCCrafting IsRuneforging IsTradeSkillReady`

### c_stubs_api_social.rs (16 set-calls)

`C_ActivityContent C_ContributionCollector CloseGossip GetAdventureMapQuestDataByQuestID GetContributionCollectorsForMap GetContributionInfo GetCurrentConversation GetGossipOptions GetGossipActiveQuests GetNumActiveQuests GetNumAvailableQuests GetScenarioInfo InitiateVote IsSameMapSelected SelectGossipOption`

### utility_api.rs (15 set-calls)

`abs ceil floor format GetHash hash min max random ScheduleInterval ScheduleIntervalCancel SetFloat SetMultiplier SetVector Wrap` — math helpers and hash helpers; most are `math.*` already; the `Schedule*` helpers are probably missing and drive `C_Timer` adjacent failures.

### c_item_api_globals.rs (14 set-calls)

`C_Item C_ItemSocketInfo CanIMogItRun GetActiveSessionName GetBestItemForSlot GetDetailedItemLevelInfo GetInventoryItemLink GetItemCreationContext GetItemFamily GetItemGem GetItemSetInfo GetItemSpell GetItemStats IsItemDataCachedByID`

### c_stubs_api.rs (11 set-calls)

Non-trivial C_* namespace surface — `C_BarberShop`, `C_Console`, `C_FriendList`, etc. Each is a table with its own subset of stub methods.

### c_stubs_api_guild_delves.rs (10 set-calls)

`C_DelvesUI C_Delves C_GuildFollowers C_GuildInfo ...` — Delves season content; needs the delves state first.

### c_stubs_api_unit_frame.rs (9 set-calls)

`C_UnitFrame C_UnitProfession ...` — profession probes for unit frames.

### c_collection_api.rs (8 set-calls)

Mostly ported to `WorldState.collected_*`; audit remaining gaps.

### c_container_api.rs (7 set-calls)

`C_Container.*` bag helpers — ported partially; `GetContainerItemInfo`, `GetContainerNumFreeSlots` etc. are nil-stubbed.

### c_collection_transmog.rs (7 set-calls)

Ported to `WorldState.transmog_appearances` + `applied_transmog_slots`; audit.

### traits_api.rs (5 set-calls)

Talent/traits registration — some ported via `talent_state`.

### targeting_api.rs (3 set-calls)

`TargetDirection` / `InteractWithTarget` / targeting helpers — missing.

### c_stubs_achievement.rs (5 set-calls)

`C_AchievementInfo.*` — partially ported via `stubs/namespace_stubs.rs`.

### c_quest_api_tasks.rs (3 set-calls)

World-quest tasks API — missing.

### c_mail_api.rs (3 set-calls)

`GetInboxItem` / `ReplyInboxItem` / `TakeInboxItem` — partial.

### c_stubs_api_pet_battles.rs (3 set-calls)

Ported in part via `pet_battles.rs`.

### c_stubs_api_professions.rs (4 set-calls)

Partial — `TrackedRecipes` covers tracking; recipe DB missing.

### c_stubs_api_lfg.rs (7 set-calls)

Partial — `lfg_list_counts` + `can_use_premade_group` cover the new scaffold; dungeon info missing.

### c_stubs_api_chat_quest.rs (8 set-calls)

Chat / quest crossover surface — missing.

### c_stubs_api_combat_log.rs (and `_curve`, `_encounter`, `_delves`, `_store`, `_shop`)

Each 4-8 entries; surfaces are mostly small C_* namespaces. Each deserves its own PLAN task (already partially enumerated under the stub-upgrade section).

### Other notable deleted modules (not in top 30 by count but structurally important)

- `targeting_api.rs` — targeting globals.
- `event_query_api.rs` — `C_EventUtils.NotifySettingsLoaded`, event shape helpers.
- `fading_frame_api.rs` — fade-in/fade-out frame mixin.
- `lua_duration_object.rs` — `LuaDurationObject` table.
- `abbreviate_config.rs` — abbreviation helpers for number display.
- `addon_api_runtime.rs` / `addon_api.rs` — addon metadata helpers (`GetAddOnMetadata`, etc.).
- `aura_api.rs` — aura utilities (partial: `C_UnitAuras.GetAuraSlots` etc. stubbed).
- `c_editmode_api.rs` — EditMode persistence.
- `c_event_utils_api.rs` — event-utils helpers.
- `early_globals.rs` — globals registered before the main registrar.
- `settings_api.rs` — Settings API (likely HUGE — needs full audit).
- `cursor_api.rs` — cursor pickup helpers (partial: `CursorInfo` state exists).
- `constants_api.rs` — LE_* enum constants.
- `dropdown_api.rs` — dropdown mixin; blocks Menu.lua path (see Menu fallback in loader_env.rs).
- `mixin_api.rs` — mixin helpers (partial: `Mixin`, `CreateFromMixins`).
- `c_unit_auras_api.rs` — unit auras (partial).
- `protected_call.rs` — `pcall` / `xpcall` shims.
- `locale_api.rs` — locale info (partial via `locale_info.rs`).
- `sound_api.rs` — sound playback (sound system exists on SimState).
- `tooltip_api.rs` — tooltip frame registration (partial).
- `unit_heal_prediction.rs` — heal prediction values.
- `timer_api.rs` — `C_Timer.*` (ported via `timer_layout.rs` but verify full surface).
- `generated_stubs.rs` — the 962-entry auto-generated stub file; rilua regeneration is noted in admin.rs docstring.

---

## Suggested restore order

1. **Player stats** (`player_api.rs` — 60 names). Biggest coverage win; `GetAvoidance` / `GetHaste` / `GetDodgeChance` etc. are surfaced throughout stat-frame UI.
2. **Map info** (`c_map_api.rs` — 22 names, plus `C_Map.*` namespace). Affects WorldMap, Minimap, quest POIs.
3. **System runtime mixins** (`system_api_runtime.rs` — `AnimateCallout` / `AnimateMouse` / `WowStyle1DropdownMixin`). Cheap restores, kill a lot of nil-dispatch errors.
4. **Settings API** (`settings_api.rs`). Blocks large chunks of Settings UI.
5. **C_Misc Game** (`c_misc_api_game.rs` — 51 names). Shop / Artifact / Garrison / ExpansionLandingPage surface.
6. **C_Stubs** cluster (`c_stubs_api*.rs`, 10+ files). Low per-file cost; clear in bulk.
7. **Everything else** — per-module audits in the order shown above.

Items 1-5 are big "branch-unblocking" wins. Items 6-7 are cleanup.

Each row in this matrix is a candidate PLAN task. The intent is to open ONE
subsystem at a time: fetch its master file, diff against HEAD, port or upgrade,
commit before starting the next.
