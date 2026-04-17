//! Static stub tables and registration for C_* namespace functions.

use rilua::vm::closure::RustFn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::{
    is_nil_namespace, set_namespace_fn, stub_empty_table, stub_false, stub_nil, stub_zero,
};

type NsStub = (&'static str, &'static str, RustFn);

fn stub_false_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    Ok(2)
}

static NAMESPACE_NIL_STUBS: &[NsStub] = &[
    // C_AchievementInfo
    ("C_AchievementInfo", "GetRewardItemID", stub_nil),
    ("C_AchievementInfo", "GetAchievementInfo", stub_nil),
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
    ("C_PlayerInfo", "GetAlternateFormInfo", stub_false_false),
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
    ("C_TooltipInfo", "GetGuildBankItem", stub_nil),
    ("C_TooltipInfo", "GetHyperlink", stub_nil),
    (
        "C_TooltipInfo",
        "GetInstanceLockEncountersComplete",
        stub_nil,
    ),
    ("C_TooltipInfo", "GetInventoryItem", stub_nil),
    ("C_TooltipInfo", "GetLFGDungeon", stub_nil),
    ("C_TooltipInfo", "GetPetAction", stub_nil),
    ("C_TooltipInfo", "GetQuestCurrency", stub_nil),
    ("C_TooltipInfo", "GetQuestItem", stub_nil),
    ("C_TooltipInfo", "GetQuestLogCurrency", stub_nil),
    ("C_TooltipInfo", "GetQuestLogItem", stub_nil),
    ("C_TooltipInfo", "GetRecipeResultItem", stub_nil),
    ("C_TooltipInfo", "GetShapeshift", stub_nil),
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
    // C_UIWidgetManager
    ("C_UIWidgetManager", "GetBelowMinimapWidgetSetID", stub_nil),
    (
        "C_UIWidgetManager",
        "GetObjectiveTrackerWidgetSetID",
        stub_nil,
    ),
    ("C_UIWidgetManager", "GetPowerBarWidgetSetID", stub_zero),
    ("C_UIWidgetManager", "GetTopCenterWidgetSetID", stub_nil),
    ("C_UIWidgetManager", "GetWidgetSetInfo", stub_nil),
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
    // C_LFGInfo — CanPlayerUsePremadeGroup is SimState-backed in lfg_info.rs.
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
    // C_PhotoSharing.IsAuthorized / IsEnabled are SimState-backed in photo_sharing.rs.
    // C_PlayerInfo
    ("C_PlayerInfo", "IsPlayerEligibleForNPE", stub_false),
    ("C_PlayerInfo", "IsPlayerInRPE", stub_false),
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
    // C_ScenarioInfo
    ("C_ScenarioInfo", "IsTieredEntranceScenario", stub_false),
    // C_Spell
    ("C_Spell", "GetVisibilityInfo", stub_false),
    ("C_Spell", "IsPriorityAura", stub_false),
    ("C_Spell", "IsSelfBuff", stub_false),
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
    // C_CinematicList
    ("C_CinematicList", "GetUICinematicList", stub_empty_table),
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
    // GetAuraSlots returns (continuationToken, slot1, slot2, ...). Callers
    // drive AuraUtil.ForEachAura via a `repeat ... until token == nil` loop
    // (Blizzard_FrameXMLUtil/AuraUtil.lua:114-117). Returning an empty table
    // as the first value makes that token truthy and loops forever.
    ("C_UnitAuras", "GetAuraSlots", stub_nil),
    // C_VoiceChat
    ("C_VoiceChat", "GetChannels", stub_empty_table),
    // C_WowLabs
    ("C_WowLabs", "GetAvailableQueues", stub_empty_table),
];

pub(super) fn register_namespace_stubs(state: &mut LuaState) {
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
