//! Static stub tables and registration for top-level global functions.

use rilua::vm::closure::RustFn;
use rilua::vm::state::LuaState;

use super::{
    is_nil_global, set_global_fn, stub_false, stub_nil, stub_repair_all_cost, stub_role_none,
    stub_role_none_enum, stub_zero,
};

static GLOBAL_NIL_STUBS: &[&str] = &[
    "AddFriend",
    "AgreeToSurvey",
    "AscendToRank",
    "AuctionHouseShowAuctionator",
    "CheckCharacterUndeleteCooldown",
    "ClearCursor",
    "ClearInspectPlayer",
    "CollapseSkillHeader",
    "ConfirmBossEmote",
    "DismissSummon",
    "DoBattlefieldMaintenance",
    "DoEmote",
    "ExpandSkillHeader",
    "ForceLogout",
    "ForceTaint",
    "InspectUnit",
    "LeaveMythicPlusGroup",
    "LogoutStatusFrame_StartLogout",
    "LootSlot",
    "MacroFrameTab_OnClick",
    "OpenWorldMap",
    "PlayMusic",
    "PlaySound",
    "PlaySoundFile",
    "RaidGroupSetRole",
    "RepairAllItems",
    "ReportCheating",
    "RequestInspectData",
    "RequestLFDPlayerLockInfo",
    "RequestPartyLootMethod",
    "RequestRaidInfo",
    "GetUnitPowerBarInfo",
    "GetInventoryItemID",
    "GetInventoryItemQuality",
    "ResetCameraPosition",
    "SetActionBarToggles",
    "SetChannelPassword",
    "SetInsertItemsLeftToRight",
    "SetLootThreshold",
    "SetPartyLeader",
    "SetRaidSubgroup",
    "SetSelectedFaction",
    "SetUnitCritterKillCount",
    "SetView",
    "SetWatchedFaction",
    "ShowingCinematic",
    "ShowUIPanel",
    "SortBags",
    "SortReagentBag",
    "StopCinematic",
    "StopMusic",
    "SwapActionSlots",
    "SwapRangedWeapon",
    "TaxiNodeSetFocus",
    "UnlearnSkill",
    "UnloadUnit",
    "UnmuteFriend",
    "UntrackAchievement",
    "UpdateTransmogrifyOutfit",
    "UseAction",
    "UseContainerItem",
    "UseInventoryItem",
    "UseItemByName",
    "WardrobeFrame_OpenTransmogToItem",
];

static GLOBAL_FALSE_STUBS: &[&str] = &[
    "AreNewRecruitTutorialsEnabled",
    "CanComplainChat",
    "CanComplainMail",
    "CanExitVehicle",
    "CanPartyLFGBackfill",
    "CanSendAuctionQuery",
    "CanShowAchievementUI",
    "CanSummonFriend",
    "CanUseLanguage",
    "DoesCurrentZoneHaveDungeon",
    "GetCVarBool",
    "GetLFGDungeonEncounterInfo",
    "GetLFGRoles",
    "GetLootMethod",
    "GetMasterLooterThreshold",
    "HasNewMail",
    "IsOnGroundFloorInJailersTower",
    // IsShiftKeyDown is SimState-backed in modifier_keys.rs, not a stub.
    "IsThreatWarningEnabled",
    "NeedToDisplayDisclaimer",
    "PetUsesPetFrame",
    // PlayerIsTimerunning is SimState-backed in rilua_timerunning.rs, not a stub.
    "ShouldShowLevelSquishDialog",
    "UnitCanAssist",
    "UnitDistanceSquared",
    "UnitInAura",
    "UnitIsCharmed",
    "UnitIsOwnerOrControllerOfUnit",
    "UnitIsPVP",
    "UnitHasVehicleUI",
    "UnitIsGameObject",
    "UnitIsPVPSanctioned",
    "UnitIsQuestBoss",
    "UnitIsTapDenied",
    "UnitOnTaxi",
    "UnitPVPName",
    "UnitPlayerControlled",
];

static GLOBAL_ZERO_STUBS: &[&str] = &[
    // GetActionCooldown is SimState-backed in cooldown_probes.rs, not a stub.
    "GetAuctionHouseDepositRate",
    "GetBackpackCurrencyInfo",
    "GetBattlefieldInstanceRunTime",
    "GetBattlefieldStatus",
    // GetContainerNumFreeSlots is SimState-backed in inventory_counts.rs,
    // not a stub.
    "GetCurrentGuildBankTab",
    "GetCursorPosition",
    // GetArenaOpponentSpec is SimState-backed in talent_spec_probes.rs,
    // not a stub.
    "GetFactionInfoByID",
    "GetGossipNumOptions",
    "GetGossipNumAvailableQuests",
    "GetGossipNumActiveQuests",
    "GetChannelName",
    "GetGuildBankTabCost",
    "GetGuildBankTabInfo",
    "GetGuildBankText",
    "GetGuildFactionInfo",
    // GetGuildRosterInfo / GetGuildRosterMOTD / GetGuildRosterSize are
    // SimState-backed in guild_probes.rs, not stubs.
    "GetGuildTabardInfo",
    "GetInstanceInfo",
    "GetInventoryAlertStatus",
    // GetInventoryItemCooldown is SimState-backed in cooldown_probes.rs,
    // not a stub.
    "GetItemQualityColor",
    "GetLFGDungeonInfo",
    "GetLFGDungeonNumEncounters",
    "GetLFGMode",
    // GetMerchantNumItems is SimState-backed in inventory_counts.rs, not a stub.
    "GetMirrorTimerInfo",
    "GetMirrorTimerProgress",
    "GetMouseFocus",
    "GetNextInteractUnit",
    // GetNumAuctionItems is SimState-backed in inventory_counts.rs, not a stub.
    "GetNumBattlegroundEntries",
    "GetNumClasses",
    // GetNumGroupMembers / GetNumPartyMembers / GetNumRaidMembers /
    // GetNumSubgroupMembers are SimState-backed in group_queries.rs,
    // not stubs.
    "GetNumGuildBankTabs",
    // GetNumGuildMembers is SimState-backed in guild_probes.rs, not a stub.
    // GetNumLootItems is SimState-backed in inventory_counts.rs, not a stub.
    // GetNumQuestLogEntries is SimState-backed in quest_surface.rs, not a stub.
    "GetNumShapeshiftForms",
    // GetNumSkillLines / GetNumSpellTabs / GetNumTalentTabs are
    // SimState-backed in talent_spec_probes.rs, not stubs.
    "GetNumTitles",
    // GetPetExperience / GetPetHappiness / GetPetLoyalty /
    // GetPetTimeInCombat are SimState-backed in pet_stats.rs, not stubs.
    // GetPvpTalentSlotInfo is SimState-backed in talent_spec_probes.rs,
    // not a stub.
    // GetQuestLogTimeLeft / QuestMapUpdateAllQuests are SimState-backed in
    // quest_surface.rs, not stubs.
    "GetRaidRosterInfo",
    "GetRelicSlotType",
    "GetRestState",
    // GetSelectedSkill / GetSkillLineInfo are SimState-backed in
    // talent_spec_probes.rs, not stubs.
    "GetSelectedSocial",
    // GetSpellAutocast / GetSpellBonusDamage / GetSpellBonusHealing /
    // GetSpellCooldown / GetSpellLevelLearned are SimState-backed in
    // cooldown_probes.rs, not stubs.
    // GetSpellTabInfo / GetTalentInfo are SimState-backed in
    // talent_spec_probes.rs, not stubs.
    "GetSummonConfirmSummoner",
    "GetSummonConfirmTimeLeft",
    "GetTitleName",
    "GetTradePlayerItemInfo",
    "GetTradeSkillInfo",
    "GetTradeTargetItemInfo",
    "GetXPExhaustion",
    // UnitArmor / UnitAttackPower / UnitCriticalStrike / UnitDamage /
    // UnitDefense / UnitDodge / UnitParry / UnitSpellHaste / UnitStat /
    // UnitResistance / UnitRangedAttackPower / UnitRangedCriticalStrike /
    // UnitRangedDamage / UnitReaction / UnitHealthMax / UnitPowerMax /
    // UnitXP / UnitXPMax are SimState-backed in unit_stats.rs, not stubs.
    "UnitAttackBothHands",
    "UnitAttackSpeed",
    "UnitBattlePetLevel",
    "UnitHasVehiclePlayerFrameUI",
    "UnitIsAFK",
    "UnitIsDND",
    "UnitIsUnit",
    "UnitRangedAttack",
];

static GLOBAL_CUSTOM_STUBS: &[(&str, RustFn)] = &[
    ("GetReadyCheckStatus", stub_nil),
    ("GetReadyCheckTimeLeft", stub_zero),
    ("GetRepairAllCost", stub_repair_all_cost),
    ("UnitGroupRolesAssigned", stub_role_none),
    ("UnitGroupRolesAssignedEnum", stub_role_none_enum),
];

pub(super) fn register_global_stubs(state: &mut LuaState) {
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
