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

---

## Deleted Helper Module Audit (PLAN.md line 32)

**Summary:** Four master-era helper modules (script_helpers, secure_env, key_dispatch, keybindings) account for 37 public symbols. The rilua tree has ported 11 (script_helpers: 7 + secure_env: 4), regressed 1 (send_key_press), and left 25 missing (script_helpers: 12 + key_dispatch internal methods absorbed, keybindings: 7). **Suggested restore order by subsystem priority (lowest surface-area first):** (1) secure_env polish (apply_secure_env absorbed into mark_secure; small gap), (2) script_helpers frame/error helpers (add_frame_unit_event_callback, dispatch_frame_unit_event_callbacks, get_frame_ref, get_stack_taint, LuaApiError, lua_error variants), (3) keybindings full table (init_keybindings, dispatch_key_binding, get_binding_*), (4) key_dispatch rewrite (send_key_press + internal dispatch tree).

### script_helpers.rs (401 lines, 19 public symbols)

| Symbol | Master lines | Rilua location | Status | Notes |
|--------|--------------|----------------|--------|-------|
| get_scripts_table | 20 | — | missing | Registry table reader; no equivalent in rilua |
| get_or_create_scripts_table | 25 | — | missing | Registry table creation; no equivalent |
| get_script | 35 | src/lua_api/script_helpers.rs:78 | ported | Lookup handler; mlua::Function → Val |
| set_script | 42 | src/lua_api/script_helpers.rs:88 | ported | Store handler; signature adapted |
| remove_script | 50 | src/lua_api/script_helpers.rs:96 | ported | Delete handler; Val::Nil signature |
| clear_on_update_script_caches | 58 | src/lua_api/script_helpers.rs:104-116 (sync_on_update_cache) | ported | Cache sync; merged into sync_on_update_cache |
| get_frame_fields_table | 96 | — | missing | Registry table reader |
| get_or_create_frame_fields_table | 101 | — | missing | Registry creation for frame fields |
| get_or_create_frame_fields | 112 | — | missing | Per-frame field table creation |
| add_frame_unit_event_callback | 133 | — | missing | Frame event callback registration; 30-line impl |
| dispatch_frame_unit_event_callbacks | 164 | — | missing | Frame event callback dispatch; 43-line impl |
| get_frame_ref | 250 | — | missing | Frame userdata lookup; likely maps to frame_ref in methods.rs |
| call_error_handler | 260 | src/lua_api/script_helpers.rs:121 | ported | Error handler invocation; both mlua + rilua variants |
| get_stack_taint | 283 | — | missing | Addon taint detection from Lua stack; 24-line master impl |
| collect_lua_error | 325 | src/lua_api/script_helpers.rs:351 | ported | Collect error into SimState; signature preserved |
| get_event_listeners_lua_order | 353 | src/lua_api/script_helpers.rs:380 (get_event_listeners) | ported | Event listener query; renamed (Lua order assumed) |
| LuaApiError | 384 | — | missing | Error struct; used by lua_error macros |
| lua_error | 393 | — | missing | mlua::Error creation wrapper; no rilua equivalent |
| lua_error_val | 398 | — | missing | mlua::Error creation without Lua ref; no rilua equivalent |

**Porting priorities:** Frame callbacks (unit events) and frame_ref lookup are high-value. Error struct can be deferred (lua_error is only used internally by script dispatch).

### secure_env.rs (69 lines, 3 public symbols)

| Symbol | Master lines | Rilua location | Status | Notes |
|--------|--------------|----------------|--------|-------|
| create_secure_environment | 21 | src/lua_api/globals/security.rs:400 | ported | Shallow copy + __index fallback; logic preserved |
| apply_secure_env | 53 | src/lua_api/globals/security.rs:432 (mark_secure) | regressed | Calls setfenv on a function; rilua uses mark_secure(lua, func) instead; signature change is API-surface regression |
| set_in_both_envs | 62 | src/lua_api/globals/security.rs:448 (set_in_both_envs_rilua) | ported | Set in _G + secureenv; name changed; functionality preserved |

**Porting priorities:** apply_secure_env is the regression; callers currently use mark_secure but should use apply_secure_env for API consistency with master. Small rename/wrapper task.

### key_dispatch.rs (383 lines, 1 public method in WowLuaEnv impl)

| Symbol | Master lines | Rilua location | Status | Notes |
|--------|--------------|----------------|--------|-------|
| send_key_press | 25 (WowLuaEnv method) | src/lua_api/env.rs:390 | regressed | Key dispatch root; rilua implementation is a no-op stub returning Ok(()), master has 350+ lines of dispatch tree (escape → focus → editbox → keybinding → OnKeyDown) |

**Porting priorities:** send_key_press is the top blocker for interactive testing; full dispatch tree required (dispatch_escape, dispatch_key, clear_target_if_any, close_special_windows, close_all_windows, toggle_game_menu, editbox_insert_text, editbox_backspace, editbox_delete, editbox_move_cursor, editbox_cursor_home, editbox_cursor_end, fire_handler_returns_truthy, fire_on_key_down, dispatch_on_key_down). ~350 lines of plumbing.

### keybindings.rs (424 lines, 7 public functions)

| Symbol | Master lines | Rilua location | Status | Notes |
|--------|--------------|----------------|--------|-------|
| init_keybindings | 333 | — | missing | Initialize __wow_binding_actions + __wow_key_bindings registry tables; 20-line const-driven setup |
| dispatch_key_binding | 358 | — | missing | Dispatch a key binding to Lua code; 15-line function |
| get_binding_key | 378 | — | missing | Query key(s) for an action; 10-line pairs loop |
| get_binding_action | 391 | — | missing | Query action for a key; 3-line registry lookup |
| set_binding | 397 | — | missing | Set or clear a binding; 5-line registry mutation |
| get_num_bindings | 407 | — | missing | Return BINDING_ACTIONS.len(); 1-line |
| get_binding_at | 412 | — | missing | Get binding by index (1-based); 10-line lookup |

**Porting priorities:** init_keybindings + dispatch_key_binding unlock keybinding support; the query/set functions are low-cost add-ons. All 7 functions are straightforward registry-table wrappers (60 lines total).

**Grand total status:** 37 symbols: 11 ported, 1 regressed, 25 missing. Porting work is three discrete chunks:
1. **script_helpers frame/error missing** (add_frame_unit_event_callback, dispatch_frame_unit_event_callbacks, get_frame_ref, get_stack_taint, LuaApiError, lua_error): ~100 lines.
2. **keybindings full module** (all 7 functions): ~60 lines.
3. **key_dispatch rewrite** (send_key_press + 10 internal helpers): ~350 lines (highest effort).
4. **secure_env polish** (apply_secure_env wrapper): ~5 lines.

Recommend tackling in order: 4 → 1 → 2 → 3 (lowest-surface first to unblock other work).
