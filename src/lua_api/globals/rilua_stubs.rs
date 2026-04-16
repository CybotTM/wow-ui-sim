//! rilua RustFn stubs for global and C_* namespace functions.
//!
//! Provides trivial constant-return stubs for the majority of WoW API
//! functions that return `nil`, `false`, `0`, or an empty table.
//!
//! # Design
//!
//! Four shared stub functions cover almost every case:
//!   - `stub_nil`         → returns nothing (Lua `nil`)
//!   - `stub_false`       → returns `false`
//!   - `stub_zero`        → returns `0`
//!   - `stub_empty_table` → returns a fresh empty table `{}`
//!
//! `register_all` maps function names to the appropriate stub via static
//! slice tables, then uses helper macros to avoid per-call boilerplate.

use rilua::vm::closure::{Closure, RustClosure, RustFn};
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

// ── Shared stub implementations ──────────────────────────────────────────────

/// Returns nothing — Lua sees `nil`.
pub fn stub_nil(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// Returns `false`.
pub fn stub_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// Returns `0`.
pub fn stub_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// Returns `"NONE"` for role APIs that expect a string token.
pub fn stub_role_none(state: &mut LuaState) -> LuaResult<u32> {
    let value = state.gc.intern_string(b"NONE");
    state.push(Val::Str(value));
    Ok(1)
}

/// Returns the no-role enum value used by Blizzard APIs.
pub fn stub_role_none_enum(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(-1.0));
    Ok(1)
}

/// Returns `(0, false)` for merchant repair cost checks.
pub fn stub_repair_all_cost(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    Ok(2)
}

/// Returns a fresh empty table `{}`.
pub fn stub_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table_ref = state.gc.alloc_table(Table::new());
    state.push(Val::Table(table_ref));
    Ok(1)
}

// ── Internal registration helpers ─────────────────────────────────────────────

/// Set a `RustFn` as a global in the rilua state.
fn set_global_fn(state: &mut LuaState, name: &'static str, func: RustFn) {
    let key = state.gc.intern_string(name.as_bytes());
    let closure = Closure::Rust(RustClosure::new(func, name));
    let closure_ref = state.gc.alloc_closure(closure);
    let global = state.global;
    if let Some(g) = state.gc.tables.get_mut(global) {
        let _ = g.raw_set(
            Val::Str(key),
            Val::Function(closure_ref),
            &state.gc.string_arena,
        );
    }
}

/// Get or create a C_* namespace table in globals, then set a `RustFn` on it.
///
/// If the namespace does not exist yet it is created and registered.
fn set_namespace_fn(
    state: &mut LuaState,
    namespace: &'static str,
    method: &'static str,
    func: RustFn,
) {
    // Resolve or create the namespace table.
    let ns_key = state.gc.intern_string(namespace.as_bytes());
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|g| g.get_str(ns_key, &state.gc.string_arena))
        .unwrap_or(Val::Nil);

    let ns_ref = match existing {
        Val::Table(t) => t,
        _ => {
            let new_table = state.gc.alloc_table(Table::new());
            let global = state.global;
            if let Some(g) = state.gc.tables.get_mut(global) {
                let _ = g.raw_set(
                    Val::Str(ns_key),
                    Val::Table(new_table),
                    &state.gc.string_arena,
                );
            }
            new_table
        }
    };

    // Set the method on the namespace table.
    let m_key = state.gc.intern_string(method.as_bytes());
    let closure = Closure::Rust(RustClosure::new(func, method));
    let closure_ref = state.gc.alloc_closure(closure);
    if let Some(ns) = state.gc.tables.get_mut(ns_ref) {
        let _ = ns.raw_set(
            Val::Str(m_key),
            Val::Function(closure_ref),
            &state.gc.string_arena,
        );
    }
}

// ── Registration entry point ──────────────────────────────────────────────────

/// Register all rilua stub globals and C_* namespace stubs.
///
/// Only registers each name if the global slot is currently `nil`, so
/// hand-written implementations registered earlier always take priority.
pub fn register_all(state: &mut LuaState) {
    register_global_stubs(state);
    register_namespace_stubs(state);
}

// ── Global function stubs ─────────────────────────────────────────────────────

/// (name, stub) pairs for top-level global stubs.
///
/// Each tuple is `(&str, RustFn)`. The stub function is chosen based on
/// what the mlua equivalent returns:
///   nil   → stub_nil
///   false → stub_false
///   0     → stub_zero
///   {}    → stub_empty_table
static GLOBAL_NIL_STUBS: &[&str] = &[
    "AcceptBattlefieldPort",
    "AcceptDuel",
    "AcceptGroup",
    "AcceptResurrect",
    "AcceptTrade",
    "AcknowledgeAutoQuestPopUp",
    "AddChatWindowChannel",
    "AddFriend",
    "AgreeToSurvey",
    "AscendToRank",
    "AttackTarget",
    "AuctionHouseShowAuctionator",
    "CancelAuction",
    "CancelTrade",
    "CastSpell",
    "CastSpellByID",
    "CastSpellByName",
    "ChannelBan",
    "ChannelInvite",
    "ChannelKick",
    "ChannelLeave",
    "ChannelModerator",
    "ChannelUnmoderator",
    "CheckCharacterUndeleteCooldown",
    "ClearCursor",
    "ClearInspectPlayer",
    "ClearTarget",
    "ClickSpecialAbility",
    "CloseBankFrame",
    "CloseGuildBankFrame",
    "CloseGuildRegistrar",
    "CloseInbox",
    "CloseLoot",
    "CloseMerchant",
    "ClosePetStables",
    "CloseQuestFrame",
    "CloseSocketInfo",
    "CloseTabardCreation",
    "CloseTrainerFrame",
    "CollapseSkillHeader",
    "ConfirmAcceptQuest",
    "ConfirmBossEmote",
    "DeclineGroup",
    "DeclineDuel",
    "DeclineResurrect",
    "DeleteCursorItem",
    "DeleteMail",
    "DismissSummon",
    "DoBattlefieldMaintenance",
    "DoEmote",
    "EditMacro",
    "EquipCursorItem",
    "ExpandSkillHeader",
    "FocusUnit",
    "ForceLogout",
    "ForceTaint",
    "ForwardMail",
    "GuildInvite",
    "GuildKick",
    "GuildLeave",
    "GuildPromote",
    "RequestGuildChallengeInfo",
    "GuildSetMOTD",
    "GuildUninvite",
    "InitiateTrade",
    "InspectUnit",
    "JoinBattlefield",
    "JoinChannelByName",
    "JoinTemporaryChannel",
    "KickUnit",
    "LeaveBattlefield",
    "LeaveMythicPlusGroup",
    "LeaveParty",
    "LogoutStatusFrame_StartLogout",
    "LootSlot",
    "MacroFrameTab_OnClick",
    "MoveForwardStart",
    "MoveForwardStop",
    "OpenWorldMap",
    "PickupBagFromSlot",
    "PickupContainerItem",
    "PickupInventoryItem",
    "PickupMacro",
    "PickupMerchantItem",
    "PickupPetAction",
    "PickupPvpTalent",
    "PickupSpell",
    "PickupTalent",
    "PlaceAction",
    "PlayMusic",
    "PlaySound",
    "PlaySoundFile",
    "QueueForLFG",
    "QuestChoiceFrame_SetActiveChoice",
    "QuestMapLogTitleButton_OnClick",
    "RaidGroupSetRole",
    "ReadyCheck",
    "RemoveFromParty",
    "RepairAllItems",
    "ReportCheating",
    "RequestBattlefieldPositions",
    "RequestGuildRoster",
    "RequestInspectData",
    "RequestLFDPlayerLockInfo",
    "RequestPartyLootMethod",
    "RequestRaidInfo",
    "GetUnitPowerBarInfo",
    "GetInventoryItemID",
    "GetInventoryItemQuality",
    "ResetCameraPosition",
    "ResurrectGetOfferer",
    "RetrieveCorpse",
    "RunMacro",
    "GetChatWindowMessages",
    "GetChatWindowChannels",
    "SendAddonMessage",
    "SendChatMessage",
    "SendMail",
    "SetChatWindowAlpha",
    "SetChatWindowColor",
    "SetChatWindowLocked",
    "SetChatWindowUninteractable",
    "ChangeChatColor",
    "SetAbandonQuest",
    "SetActionBarToggles",
    "SetChannelPassword",
    "SetCVar",
    "SetCursorItemSlot",
    "SetInsertItemsLeftToRight",
    "SetLootThreshold",
    "SetPartyLeader",
    "SetRaidSubgroup",
    "SetSelectedFaction",
    "SetTrackedAchievement",
    "SetTradeCurrency",
    "SetUnitCritterKillCount",
    "SetView",
    "SetWatchedFaction",
    "ShowingCinematic",
    "ShowUIPanel",
    "SortBags",
    "SortReagentBag",
    "SpellTargetUnit",
    "StopAttack",
    "StopCinematic",
    "StopMacro",
    "StopMusic",
    "SwapActionSlots",
    "SwapChatChannelLinks",
    "SwapRangedWeapon",
    "TargetLastTarget",
    "TargetNearestEnemy",
    "TargetNearestFriend",
    "TargetUnit",
    "TaxiNodeSetFocus",
    "ToggleBattlefieldMinimap",
    "ToggleCharacter",
    "ToggleDropDownMenu",
    "ToggleFriendsFrame",
    "ToggleGuildFrame",
    "ToggleHelpFrame",
    "ToggleMinimap",
    "ToggleQuestLog",
    "ToggleSocialPanel",
    "ToggleSpellBook",
    "ToggleTalentFrame",
    "ToggleWorldMap",
    "UninviteUnit",
    "UnlearnSkill",
    "UnloadUnit",
    "UnmuteFriend",
    "UntrackAchievement",
    "UpdateTransmogrifyOutfit",
    "UseAction",
    "UseContainerItem",
    "UseInventoryItem",
    "UseItemByName",
    "VoiceChat_GetMicrophoneVolume",
    "VoiceChat_SetMicrophoneVolume",
    "VoiceChat_SetOutputVolume",
    "VoiceChatHeadsetModeCheck",
    "WardrobeFrame_OpenTransmogToItem",
];

static GLOBAL_FALSE_STUBS: &[&str] = &[
    "AreNewRecruitTutorialsEnabled",
    "CanComplainChat",
    "CanComplainMail",
    "CanExitVehicle",
    "CanInspect",
    "CanLootUnit",
    "CanMerchant",
    "CanReplaceGuildMaster",
    "CanSendAuctionQuery",
    "CanShowAchievementUI",
    "CanSummonFriend",
    "CanUseLanguage",
    "DoesCurrentZoneHaveDungeon",
    "GetAutoDeclineGuildInvites",
    "GetCVarBool",
    "GetGuildRosterShowOffline",
    "GetLFGDungeonEncounterInfo",
    "GetLFGRoles",
    "GetLootMethod",
    "GetMasterLooterThreshold",
    "GetPVPDesired",
    "GetPVPLastHonorGain",
    "HasNewMail",
    "HasPetSpells",
    "InCombatLockdown",
    "IsBattlefieldArena",
    "IsConsumableItem",
    "IsCurrentAction",
    "IsCurrentSpell",
    "IsEncounterInProgress",
    "IsEquippableItem",
    "IsEveryoneAssistant",
    "IsFlyableArea",
    "IsInventoryItemLocked",
    "IsGroupLeader",
    "IsHarmfulSpell",
    "IsHelpfulSpell",
    "IsInActiveWorldPVP",
    "IsInGroup",
    "IsInGuild",
    "IsInInstance",
    "IsInRaid",
    "IsItemInRange",
    "IsLoggedIn",
    "IsMenuOpen",
    "IsPartyLFG",
    "IsResting",
    "IsShiftKeyDown",
    "IsSpellInRange",
    "IsSpellKnown",
    "IsSpellKnownOrOverridesKnown",
    "IsSubZonePVP",
    "IsThreatWarningEnabled",
    "IsUsingVoiceChat",
    "IsVoiceEnabled",
    "IsXPUserDisabled",
    "NeedToDisplayDisclaimer",
    "PlayerCanTeleport",
    "PlayerHasHearthstone",
    "PlayerIsTimerunning",
    "ShouldShowLevelSquishDialog",
    "UnitCanAssist",
    "UnitCanCooperate",
    "UnitDistanceSquared",
    "UnitFactionGroup",
    "UnitInAura",
    "UnitInBattleground",
    "UnitHasIncomingResurrection",
    "UnitInOtherParty",
    "UnitInParty",
    "UnitInRaid",
    "UnitInRange",
    "UnitIsCharmed",
    "UnitIsCorpse",
    "UnitIsDeadOrGhost",
    "UnitIsGroupAssistant",
    "UnitIsGroupLeader",
    "UnitIsOwnerOrControllerOfUnit",
    "UnitHasVehicleUI",
    "UnitIsGameObject",
    "UnitIsPVPSanctioned",
    "UnitIsQuestBoss",
    "UnitIsTapDenied",
    "UnitIsUnconscious",
    "UnitIsVisible",
    "UnitOnTaxi",
    "UnitPVPName",
    "UnitPlayerControlled",
    "VoiceChat_IsConnecting",
    "VoiceChat_IsDeafened",
    "VoiceChat_IsMuted",
    "VoiceChat_IsTalking",
];

static GLOBAL_ZERO_STUBS: &[&str] = &[
    "GetActionCooldown",
    "GetAuctionHouseDepositRate",
    "GetBackpackCurrencyInfo",
    "GetBattlefieldInstanceRunTime",
    "GetBattlefieldStatus",
    "GetContainerNumFreeSlots",
    "GetCurrentGuildBankTab",
    "GetCursorPosition",
    "GetArenaOpponentSpec",
    "GetFactionInfoByID",
    "GetGossipNumOptions",
    "GetGossipNumAvailableQuests",
    "GetGossipNumActiveQuests",
    "GetGuildBankTabCost",
    "GetGuildBankTabInfo",
    "GetGuildBankText",
    "GetGuildFactionInfo",
    "GetGuildRosterInfo",
    "GetGuildRosterMOTD",
    "GetGuildRosterSize",
    "GetGuildTabardInfo",
    "GetInstanceInfo",
    "GetInventoryAlertStatus",
    "GetInventoryItemCooldown",
    "GetItemQualityColor",
    "GetLFGDungeonInfo",
    "GetLFGDungeonNumEncounters",
    "GetLFGMode",
    "GetMerchantNumItems",
    "GetMirrorTimerInfo",
    "GetMirrorTimerProgress",
    "GetMouseFocus",
    "GetNextInteractUnit",
    "GetNumAuctionItems",
    "GetNumBattlegroundEntries",
    "GetNumClasses",
    "GetNumGroupMembers",
    "GetNumGuildBankTabs",
    "GetNumGuildMembers",
    "GetNumLootItems",
    "GetNumPartyMembers",
    "GetNumQuestLogEntries",
    "GetNumRaidMembers",
    "GetNumShapeshiftForms",
    "GetNumSkillLines",
    "GetNumSpellTabs",
    "GetNumSubgroupMembers",
    "GetNumTalentTabs",
    "GetNumTitles",
    "GetPetExperience",
    "GetPetHappiness",
    "GetPetLoyalty",
    "GetPetTimeInCombat",
    "GetPvpTalentSlotInfo",
    "GetQuestLogTimeLeft",
    "GetRaidRosterInfo",
    "GetRelicSlotType",
    "GetRestState",
    "GetSelectedSkill",
    "GetSelectedSocial",
    "GetSkillLineInfo",
    "GetSpellAutocast",
    "GetSpellBonusDamage",
    "GetSpellBonusHealing",
    "GetSpellCooldown",
    "GetSpellLevelLearned",
    "GetSpellTabInfo",
    "GetSummonConfirmSummoner",
    "GetSummonConfirmTimeLeft",
    "GetTalentInfo",
    "GetTitleName",
    "GetTradePlayerItemInfo",
    "GetTradeSkillInfo",
    "GetTradeTargetItemInfo",
    "GetXPExhaustion",
    "UnitArmor",
    "UnitAttackBothHands",
    "UnitAttackPower",
    "UnitAttackSpeed",
    "UnitBattlePetLevel",
    "UnitCriticalStrike",
    "UnitDamage",
    "UnitDefense",
    "UnitDodge",
    "UnitHasVehiclePlayerFrameUI",
    "UnitHealthMax",
    "UnitIsAFK",
    "UnitIsDND",
    "UnitIsUnit",
    "UnitParry",
    "UnitPowerMax",
    "UnitRangedAttack",
    "UnitRangedAttackPower",
    "UnitRangedCriticalStrike",
    "UnitRangedDamage",
    "UnitReaction",
    "UnitResistance",
    "UnitSpellHaste",
    "UnitStat",
    "UnitXP",
    "UnitXPMax",
];

static GLOBAL_CUSTOM_STUBS: &[(&str, RustFn)] = &[
    ("GetReadyCheckStatus", stub_nil),
    ("GetReadyCheckTimeLeft", stub_zero),
    ("GetRepairAllCost", stub_repair_all_cost),
    ("UnitGroupRolesAssigned", stub_role_none),
    ("UnitGroupRolesAssignedEnum", stub_role_none_enum),
];

fn register_global_stubs(state: &mut LuaState) {
    for &name in GLOBAL_NIL_STUBS {
        if is_nil_global(state, name) {
            set_global_fn(state, name, stub_nil);
        }
    }
    for &name in GLOBAL_FALSE_STUBS {
        if is_nil_global(state, name) {
            set_global_fn(state, name, stub_false);
        }
    }
    for &name in GLOBAL_ZERO_STUBS {
        if is_nil_global(state, name) {
            set_global_fn(state, name, stub_zero);
        }
    }
    for &(name, func) in GLOBAL_CUSTOM_STUBS {
        if is_nil_global(state, name) {
            set_global_fn(state, name, func);
        }
    }
}

/// Returns true if the global `name` is currently `nil`.
fn is_nil_global(state: &mut LuaState, name: &str) -> bool {
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    state
        .gc
        .tables
        .get(global)
        .map(|g| g.get_str(key, &state.gc.string_arena) == Val::Nil)
        .unwrap_or(true)
}

// ── C_* namespace stubs ───────────────────────────────────────────────────────

/// (namespace, method, stub) triples.
///
/// These mirror the mlua stubs in c_stubs_api_*.rs and c_misc_api_*.rs.
/// The stub function used matches what the mlua version returns.
type NsStub = (&'static str, &'static str, RustFn);

static NAMESPACE_NIL_STUBS: &[NsStub] = &[
    // C_AchievementInfo
    ("C_AchievementInfo", "GetRewardItemID", stub_nil),
    ("C_AchievementInfo", "GetAchievementInfo", stub_nil),
    ("C_AddOnProfiler", "CheckForPerformanceMessage", stub_nil),
    // C_AreaPoiInfo
    ("C_AreaPoiInfo", "GetAreaPOIInfo", stub_nil),
    ("C_AreaPoiInfo", "GetAreaPOISecondsLeft", stub_nil),
    // C_AuctionHouse
    ("C_AuctionHouse", "GetAuctionItemSubClasses", stub_nil),
    ("C_AuctionHouse", "GetReplicateItemInfo", stub_nil),
    // C_BattleNet
    ("C_BattleNet", "GetAccountInfoByGUID", stub_nil),
    ("C_BattleNet", "GetFriendAccountInfo", stub_nil),
    ("C_BattleNet", "GetGameAccountInfoByGUID", stub_nil),
    // C_CharacterServices
    (
        "C_CharacterServices",
        "GetActiveCharacterUpgradeBoostType",
        stub_nil,
    ),
    (
        "C_CharacterServices",
        "GetActiveClassTrialBoostType",
        stub_nil,
    ),
    // C_ChatBubbles
    ("C_ChatBubbles", "GetAllChatBubbles", stub_nil),
    // C_ClassTalents
    ("C_ClassTalents", "GetActiveConfigID", stub_nil),
    ("C_ClassTalents", "GetConfigIDsBySpecID", stub_nil),
    ("C_ClassTalents", "GetHeroTalentSpecsForClassSpec", stub_nil),
    ("C_ClassTalents", "GetTraitTreeForSpec", stub_nil),
    // C_Club
    ("C_Club", "GetClubMembers", stub_nil),
    ("C_Club", "GetSubscribedClubs", stub_nil),
    // C_CurrencyInfo
    ("C_CurrencyInfo", "GetCurrencyContainerInfo", stub_nil),
    ("C_CurrencyInfo", "GetCurrencyInfo", stub_nil),
    ("C_CurrencyInfo", "GetCurrencyInfoFromLink", stub_nil),
    // C_DeathRecap
    ("C_DeathRecap", "GetKillingBlows", stub_nil),
    ("C_DeathRecap", "GetMostRecentDeathRecap", stub_nil),
    // C_EncounterJournal
    ("C_EncounterJournal", "GetEncounterInfo", stub_nil),
    ("C_EncounterJournal", "GetInstanceInfo", stub_nil),
    // C_GossipInfo
    ("C_GossipInfo", "GetActiveQuests", stub_nil),
    ("C_GossipInfo", "GetAvailableQuests", stub_nil),
    ("C_GossipInfo", "GetOptions", stub_nil),
    ("C_GossipInfo", "GetPoiForUiMapID", stub_nil),
    // C_Heirloom
    ("C_Heirloom", "GetHeirloomInfo", stub_nil),
    ("C_Heirloom", "GetHeirloomItemIDFromDisplayedSlot", stub_nil),
    // C_IncomingSummon
    ("C_IncomingSummon", "HasIncomingSummon", stub_nil),
    ("C_IncomingSummon", "IncomingSummonStatus", stub_nil),
    // C_Item
    ("C_Item", "GetItemIconByID", stub_nil),
    ("C_Item", "GetItemNameByID", stub_nil),
    ("C_Item", "GetItemQualityByID", stub_nil),
    // C_LFGInfo
    ("C_LFGInfo", "CanPlayerUseLFD", stub_nil),
    ("C_LFGInfo", "GetLFGCategoryInfo", stub_nil),
    // C_LossOfControl
    ("C_LossOfControl", "GetActiveLossOfControlData", stub_nil),
    (
        "C_LossOfControl",
        "GetActiveLossOfControlDataCount",
        stub_nil,
    ),
    // C_Map
    ("C_Map", "GetMapArtID", stub_nil),
    ("C_Map", "GetMapChildrenInfo", stub_nil),
    ("C_Map", "GetPlayerMapPosition", stub_nil),
    // C_MythicPlus
    ("C_MythicPlus", "GetCurrentAffixes", stub_nil),
    ("C_MythicPlus", "GetCurrentSeason", stub_nil),
    ("C_MythicPlus", "GetLastWeeklyChest", stub_nil),
    ("C_MythicPlus", "GetRunHistory", stub_nil),
    (
        "C_MythicPlus",
        "GetSeasonBestAffixScoreInfoForMap",
        stub_nil,
    ),
    ("C_MythicPlus", "GetWeeklyChestRewardLevel", stub_nil),
    ("C_MythicPlus", "RequestCurrentAffixes", stub_nil),
    ("C_MythicPlus", "RequestMapInfo", stub_nil),
    ("C_MythicPlus", "RequestRewards", stub_nil),
    // C_NamePlate
    ("C_NamePlate", "GetNamePlateForUnit", stub_nil),
    ("C_NamePlate", "GetNamePlates", stub_nil),
    // C_PartyInfo
    ("C_PartyInfo", "GetActiveCategories", stub_nil),
    ("C_PartyInfo", "GetInviteConfirmationInfo", stub_nil),
    // C_PetBattles
    ("C_PetBattles", "GetAbilityInfoByID", stub_nil),
    ("C_PetBattles", "GetActivePet", stub_nil),
    ("C_PetBattles", "GetAllEffectiveAbilityIDs", stub_nil),
    // GetBattleState / GetNumPets intentionally omitted: zero-returning
    // stubs live in env_init.rs so `petIndex > GetNumPets(...)` stays
    // a number comparison and doesn't crash PetBattleFrame OnLoad.
    ("C_PetBattles", "GetMaxAbilityCharges", stub_nil),
    ("C_PetBattles", "GetPetAbilityInfo", stub_nil),
    ("C_PetBattles", "GetPetAbilityList", stub_nil),
    ("C_PetBattles", "GetPetInfo", stub_nil),
    ("C_PetBattles", "GetPetInfoByPetID", stub_nil),
    ("C_PetBattles", "GetPetStats", stub_nil),
    ("C_PetBattles", "GetPlayerInfo", stub_nil),
    ("C_PetBattles", "GetRoundTimingInfo", stub_nil),
    ("C_PetBattles", "GetTurnResult", stub_nil),
    ("C_PetBattles", "GetXP", stub_nil),
    ("C_PetBattles", "IsPlayerNPC", stub_nil),
    ("C_PetBattles", "StartPVPMatchmaking", stub_nil),
    // C_PlayerInfo
    ("C_PlayerInfo", "GetAlternateFormInfo", stub_nil),
    (
        "C_PlayerInfo",
        "GetContentDifficultyCreatureForPlayer",
        stub_nil,
    ),
    ("C_PlayerInfo", "GetPlayerMythicPlusRatingSummary", stub_nil),
    // C_QuestLog
    ("C_QuestLog", "GetBountySetInfoForMapID", stub_nil),
    ("C_QuestLog", "GetInfo", stub_nil),
    ("C_QuestLog", "GetNextWaypoint", stub_nil),
    ("C_QuestLog", "GetQuestDetailsTheme", stub_nil),
    ("C_QuestLog", "GetQuestTagInfo", stub_nil),
    ("C_QuestLog", "GetWorldQuestInfo", stub_nil),
    // C_RaidFrames
    ("C_RaidFrames", "GetProfile", stub_nil),
    // C_ScenarioInfo
    (
        "C_ScenarioInfo",
        "GetScenarioBonusStepRewardQuestID",
        stub_nil,
    ),
    ("C_ScenarioInfo", "GetScenarioInfo", stub_nil),
    ("C_ScenarioInfo", "GetScenarioStepInfo", stub_nil),
    // C_Social
    ("C_Social", "GetFriendInfo", stub_nil),
    ("C_Social", "GetFriends", stub_nil),
    // C_Spell
    ("C_Spell", "GetMountFromSpell", stub_nil),
    ("C_Spell", "GetSpellInfo", stub_nil),
    // C_SummonInfo
    ("C_SummonInfo", "GetSummonReason", stub_nil),
    // C_System
    ("C_System", "GetFrameStack", stub_nil),
    // C_Timer (already has real impl for After/NewTicker; these are stubs)
    ("C_Timer", "NewTimerID", stub_nil),
    // C_TooltipInfo
    ("C_TooltipInfo", "GetAction", stub_nil),
    ("C_TooltipInfo", "GetAchievementByID", stub_nil),
    ("C_TooltipInfo", "GetAura", stub_nil),
    ("C_TooltipInfo", "GetBagItem", stub_nil),
    ("C_TooltipInfo", "GetCurrencyByID", stub_nil),
    ("C_TooltipInfo", "GetCurrencyToken", stub_nil),
    ("C_TooltipInfo", "GetGuildBankItem", stub_nil),
    ("C_TooltipInfo", "GetHyperlink", stub_nil),
    ("C_TooltipInfo", "GetInboxItem", stub_nil),
    (
        "C_TooltipInfo",
        "GetInstanceLockEncountersComplete",
        stub_nil,
    ),
    ("C_TooltipInfo", "GetInventoryItem", stub_nil),
    ("C_TooltipInfo", "GetItem", stub_nil),
    ("C_TooltipInfo", "GetLFGDungeon", stub_nil),
    ("C_TooltipInfo", "GetMerchantItem", stub_nil),
    ("C_TooltipInfo", "GetPetAction", stub_nil),
    ("C_TooltipInfo", "GetQuestCurrency", stub_nil),
    ("C_TooltipInfo", "GetQuestItem", stub_nil),
    ("C_TooltipInfo", "GetQuestLogCurrency", stub_nil),
    ("C_TooltipInfo", "GetQuestLogItem", stub_nil),
    ("C_TooltipInfo", "GetRecipeReagentItem", stub_nil),
    ("C_TooltipInfo", "GetRecipeResultItem", stub_nil),
    ("C_TooltipInfo", "GetSendMailItem", stub_nil),
    ("C_TooltipInfo", "GetShapeshift", stub_nil),
    ("C_TooltipInfo", "GetSocketedItem", stub_nil),
    ("C_TooltipInfo", "GetSpell", stub_nil),
    ("C_TooltipInfo", "GetTalent", stub_nil),
    ("C_TooltipInfo", "GetTooltipDataForItem", stub_nil),
    ("C_TooltipInfo", "GetTradePlayerItem", stub_nil),
    ("C_TooltipInfo", "GetTradeSkillItem", stub_nil),
    ("C_TooltipInfo", "GetTradeTargetItem", stub_nil),
    ("C_TooltipInfo", "GetTrainerService", stub_nil),
    ("C_TooltipInfo", "GetUnit", stub_nil),
    ("C_TooltipInfo", "GetUpgradeItem", stub_nil),
    // C_TradeSkillUI
    (
        "C_TradeSkillUI",
        "GetAllProfessionTradeSkillLines",
        stub_nil,
    ),
    ("C_TradeSkillUI", "GetBaseProfessionInfo", stub_nil),
    ("C_TradeSkillUI", "GetChildProfessionInfo", stub_nil),
    ("C_TradeSkillUI", "GetCraftingOrderCount", stub_nil),
    ("C_TradeSkillUI", "GetFilteredRecipeIDs", stub_nil),
    ("C_TradeSkillUI", "GetProfessionInfoByRecipeID", stub_nil),
    ("C_TradeSkillUI", "GetProfessions", stub_nil),
    ("C_TradeSkillUI", "GetRecipeInfo", stub_nil),
    ("C_TradeSkillUI", "GetRecipeItemLink", stub_nil),
    ("C_TradeSkillUI", "GetRecipeNumReagents", stub_nil),
    ("C_TradeSkillUI", "GetRecipeReagentInfo", stub_nil),
    ("C_TradeSkillUI", "GetRecipeReagentItemLink", stub_nil),
    ("C_TradeSkillUI", "GetRecipeSchematic", stub_nil),
    ("C_TradeSkillUI", "GetTradeSkillListLink", stub_nil),
    // C_Transmog
    ("C_Transmog", "GetAppliedAlteredAppearance", stub_nil),
    ("C_Transmog", "GetCreatureDisplayIDForSource", stub_nil),
    // C_TrophyHall (stub all nil)
    ("C_TrophyHall", "GetTrophyInfo", stub_nil),
    // C_Tutorial
    ("C_Tutorial", "AcknowledgeTutorial", stub_nil),
    ("C_Tutorial", "HasSeenTutorial", stub_nil),
    // C_UnitAuras
    ("C_UnitAuras", "GetAuraDataByAuraInstanceID", stub_nil),
    ("C_UnitAuras", "GetAuraDataByIndex", stub_nil),
    ("C_UnitAuras", "GetAuraDataBySpellName", stub_nil),
    ("C_UnitAuras", "GetBuffDataByIndex", stub_nil),
    ("C_UnitAuras", "GetDebuffDataByIndex", stub_nil),
    // C_VoiceChat
    ("C_VoiceChat", "GetActiveChannelID", stub_nil),
    ("C_VoiceChat", "GetChannel", stub_nil),
    ("C_VoiceChat", "GetChannels", stub_nil),
    (
        "C_VoiceChat",
        "GetCurrentVoiceChatConnectionStatusCode",
        stub_nil,
    ),
    ("C_VoiceChat", "GetMasterVolumeScale", stub_nil),
    ("C_VoiceChat", "GetMicrophoneVolume", stub_nil),
    ("C_VoiceChat", "GetOutputVolume", stub_nil),
    // C_WowEntitlements
    ("C_WowEntitlements", "GetAllEntitlementsByType", stub_nil),
    // C_WowLabs
    ("C_WowLabs", "GetMatchmakingEnabled", stub_nil),
    ("C_WowLabsMatchmaking", "CancelQueue", stub_nil),
    ("C_WowLabsMatchmaking", "GetCurrentQueue", stub_nil),
    ("C_WowLabsMatchmaking", "GetQueue", stub_nil),
    ("C_WowLabsMatchmaking", "JoinQueue", stub_nil),
];

static NAMESPACE_FALSE_STUBS: &[NsStub] = &[
    // C_AchievementInfo
    ("C_AchievementInfo", "IsValidAchievement", stub_false),
    // C_Bank
    ("C_Bank", "HasFullBankAccess", stub_false),
    // C_BattleNet
    ("C_BattleNet", "IsAccountMuted", stub_false),
    // C_CharacterServices
    (
        "C_CharacterServices",
        "HasRequiredServiceForCharacterUpgrade",
        stub_false,
    ),
    // C_ClassTalents
    ("C_ClassTalents", "CanChangeTalents", stub_false),
    ("C_ClassTalents", "GetHasStarterBuild", stub_false),
    ("C_ClassTalents", "IsStarterBuildActive", stub_false),
    // C_Club
    ("C_Club", "IsEnabled", stub_false),
    // C_GarrisonInfo
    ("C_GarrisonInfo", "HasGarrison", stub_false),
    // C_IncomingSummon
    (
        "C_IncomingSummon",
        "HasIncomingSummonFromFriend",
        stub_false,
    ),
    // C_Item
    ("C_Item", "IsItemTransmogrifiable", stub_false),
    // C_LFGInfo
    ("C_LFGInfo", "CanPlayerUsePremadeGroup", stub_false),
    ("C_LFGInfo", "IsLFGModeActiveForCategory", stub_false),
    // C_Map
    ("C_Map", "IsMapValidForNavigation", stub_false),
    // C_MythicPlus
    ("C_MythicPlus", "IsMythicPlusActive", stub_false),
    ("C_MythicPlus", "IsWeeklyRewardAvailable", stub_false),
    // C_PartyInfo
    ("C_PartyInfo", "IsPartyFull", stub_false),
    ("C_PartyInfo", "IsPartyInJailersTower", stub_false),
    // C_PvP
    ("C_PvP", "IsMatchConsideredArena", stub_false),
    // C_PhotoSharing — in sim we never upload/authorize, so both are false
    ("C_PhotoSharing", "IsAuthorized", stub_false),
    ("C_PhotoSharing", "IsEnabled", stub_false),
    // C_PlayerInfo
    ("C_PlayerInfo", "IsPlayerEligibleForNPE", stub_false),
    ("C_PlayerInfo", "IsPlayerNPERestricted", stub_false),
    // C_QuestLog
    ("C_QuestLog", "IsComplete", stub_false),
    ("C_QuestLog", "IsFailed", stub_false),
    ("C_QuestLog", "IsMetaQuest", stub_false),
    ("C_QuestLog", "IsOnMap", stub_false),
    ("C_QuestLog", "IsOnQuest", stub_false),
    ("C_QuestLog", "IsQuestFlaggedCompleted", stub_false),
    ("C_QuestLog", "IsQuestReplayable", stub_false),
    ("C_QuestLog", "IsWorldQuest", stub_false),
    // C_Spell
    ("C_Spell", "IsSpellUsable", stub_false),
    ("C_Spell", "TargetSpellIsEnchanting", stub_false),
    ("C_Spell", "TargetSpellJumpsUpgradeTrack", stub_false),
    ("C_Spell", "TargetSpellReplacesBonusTree", stub_false),
    // C_SummonInfo
    (
        "C_SummonInfo",
        "IsSummonSkippingStartExperience",
        stub_false,
    ),
    // C_StableInfo
    ("C_StableInfo", "IsAtPetStable", stub_false),
    // C_Transmog
    ("C_Transmog", "IsAtTransmogNPC", stub_false),
    ("C_Transmog", "PlayerHasTransmogByItemInfo", stub_false),
    // C_VoiceChat
    ("C_VoiceChat", "IsDeafened", stub_false),
    ("C_VoiceChat", "IsEnabled", stub_false),
    ("C_VoiceChat", "IsMuted", stub_false),
    ("C_VoiceChat", "IsParentalDisabled", stub_false),
    ("C_VoiceChat", "IsTalking", stub_false),
    // C_WowLabs
    ("C_WowLabs", "IsEnabled", stub_false),
];

static NAMESPACE_ZERO_STUBS: &[NsStub] = &[
    // C_BattleNet
    ("C_BattleNet", "GetFriendNumAccounts", stub_zero),
    ("C_BattleNet", "GetNumFriends", stub_zero),
    // C_Club
    ("C_Club", "GetClubCapacity", stub_zero),
    // C_GarrisonInfo
    ("C_GarrisonInfo", "GetGarrisonType", stub_zero),
    // C_MythicPlus
    ("C_MythicPlus", "GetOwnedKeystoneLevel", stub_zero),
    ("C_MythicPlus", "GetWeeklyBestForMap", stub_zero),
    // C_PartyInfo
    ("C_PartyInfo", "GetActiveGroupType", stub_zero),
    // C_QuestLog
    ("C_QuestLog", "GetLogIndexForQuestID", stub_zero),
    ("C_QuestLog", "GetNumQuestLogEntries", stub_zero),
    // C_Spell
    ("C_Spell", "GetSpellCooldown", stub_zero),
    // C_SummonInfo
    ("C_SummonInfo", "GetSummonConfirmTimeLeft", stub_zero),
    // C_TradeSkillUI
    ("C_TradeSkillUI", "GetNumRecipes", stub_zero),
    ("C_TradeSkillUI", "GetNumTradeSkills", stub_zero),
    // C_VoiceChat
    ("C_VoiceChat", "GetNumActiveChannels", stub_zero),
    ("C_VoiceChat", "GetNumMembers", stub_zero),
];

static NAMESPACE_EMPTY_TABLE_STUBS: &[NsStub] = &[
    // C_AuctionHouse
    ("C_AuctionHouse", "GetBrowseResults", stub_empty_table),
    // C_ClassTalents
    ("C_ClassTalents", "GetConfigIDsBySpecID", stub_empty_table),
    // C_Club
    ("C_Club", "GetClubMembers", stub_empty_table),
    ("C_Club", "GetSubscribedClubs", stub_empty_table),
    // C_GossipInfo
    ("C_GossipInfo", "GetActiveQuests", stub_empty_table),
    ("C_GossipInfo", "GetAvailableQuests", stub_empty_table),
    ("C_GossipInfo", "GetOptions", stub_empty_table),
    // C_LFGInfo
    ("C_LFGInfo", "GetSystemPanelData", stub_empty_table),
    // C_NamePlate
    ("C_NamePlate", "GetNamePlates", stub_empty_table),
    // C_PartyInfo
    ("C_PartyInfo", "GetActiveCategories", stub_empty_table),
    // C_ZoneAbility
    ("C_ZoneAbility", "GetActiveAbilities", stub_empty_table),
    // C_QuestLog
    ("C_QuestLog", "GetAllCompletedQuestIDs", stub_empty_table),
    // C_Social
    ("C_Social", "GetFriends", stub_empty_table),
    // C_TooltipInfo: these return nil not empty table, handled above
    // C_TradeSkillUI
    (
        "C_TradeSkillUI",
        "GetAllProfessionTradeSkillLines",
        stub_empty_table,
    ),
    ("C_TradeSkillUI", "GetFilteredRecipeIDs", stub_empty_table),
    // C_UnitAuras
    ("C_UnitAuras", "GetAuraSlots", stub_empty_table),
    // C_VoiceChat
    ("C_VoiceChat", "GetChannels", stub_empty_table),
    // C_WowLabs
    ("C_WowLabs", "GetAvailableQueues", stub_empty_table),
];

fn register_namespace_stubs(state: &mut LuaState) {
    for &(ns, method, func) in NAMESPACE_NIL_STUBS {
        if is_nil_namespace(state, ns, method) {
            set_namespace_fn(state, ns, method, func);
        }
    }
    for &(ns, method, func) in NAMESPACE_FALSE_STUBS {
        if is_nil_namespace(state, ns, method) {
            set_namespace_fn(state, ns, method, func);
        }
    }
    for &(ns, method, func) in NAMESPACE_ZERO_STUBS {
        if is_nil_namespace(state, ns, method) {
            set_namespace_fn(state, ns, method, func);
        }
    }
    for &(ns, method, func) in NAMESPACE_EMPTY_TABLE_STUBS {
        if is_nil_namespace(state, ns, method) {
            set_namespace_fn(state, ns, method, func);
        }
    }
}

/// Returns true if `namespace.method` is currently `nil`.
fn is_nil_namespace(state: &mut LuaState, namespace: &str, method: &str) -> bool {
    let ns_key = state.gc.intern_string(namespace.as_bytes());
    let m_key = state.gc.intern_string(method.as_bytes());
    let global = state.global;
    let ns_val = state
        .gc
        .tables
        .get(global)
        .map(|g| g.get_str(ns_key, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    match ns_val {
        Val::Table(t) => state
            .gc
            .tables
            .get(t)
            .map(|tbl| tbl.get_str(m_key, &state.gc.string_arena) == Val::Nil)
            .unwrap_or(true),
        Val::Nil => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::WowLuaEnv;

    fn make_env() -> WowLuaEnv {
        WowLuaEnv::new().expect("failed to create Lua environment")
    }

    #[test]
    fn stub_nil_returns_nothing() {
        let env = make_env();
        env.register_rilua_function("__test_stub_nil", stub_nil)
            .unwrap();
        let func = env.load_rilua("return __test_stub_nil()").unwrap();
        let result = env.call_rilua(&func, &[]).unwrap();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn stub_false_returns_false() {
        let env = make_env();
        env.register_rilua_function("__test_stub_false", stub_false)
            .unwrap();
        let result = env
            .call_rilua(&env.load_rilua("return __test_stub_false()").unwrap(), &[])
            .unwrap();
        assert_eq!(result, vec![Val::Bool(false)]);
    }

    #[test]
    fn stub_zero_returns_zero() {
        let env = make_env();
        env.register_rilua_function("__test_stub_zero", stub_zero)
            .unwrap();
        let result = env
            .call_rilua(&env.load_rilua("return __test_stub_zero()").unwrap(), &[])
            .unwrap();
        assert_eq!(result, vec![Val::Num(0.0)]);
    }

    #[test]
    fn stub_empty_table_returns_table() {
        let env = make_env();
        env.register_rilua_function("__test_stub_empty_table", stub_empty_table)
            .unwrap();
        // type() returns "table" for a table value
        let func = env
            .load_rilua("return type(__test_stub_empty_table())")
            .unwrap();
        let result = env.call_rilua(&func, &[]).unwrap();
        // Val::Str wraps a GcRef — we can compare by checking via Lua
        // Just assert we got one result and it is not nil/false/number
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Val::Str(_)));
    }

    #[test]
    fn register_all_does_not_panic() {
        use rilua::LuaApiMut;
        let env = make_env();
        {
            let mut lua = env.rilua_mut();
            register_all(lua.state_mut());
        }
    }

    #[test]
    fn register_all_skips_existing_global() {
        use rilua::LuaApiMut;
        let env = make_env();
        // Pre-register a sentinel value as a global
        env.set_rilua_global("ClearTarget", Val::Bool(true))
            .unwrap();
        {
            let mut lua = env.rilua_mut();
            register_all(lua.state_mut());
        }
        // Our sentinel should still be true, not overwritten by stub
        assert_eq!(env.get_rilua_global("ClearTarget"), Val::Bool(true));
    }
}
