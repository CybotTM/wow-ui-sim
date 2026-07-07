//! Startup-time global functions (UnitPower, GetTime, ACTIONBAR_HOTKEY_FONT_COLOR,
//! social/LFG, presence and return-shape checks).

use super::super::*;

#[test]
fn test_startup_utility_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        get_text_ty,
        get_text_value,
        game_time_ty,
        hour,
        minute,
        trial_ty,
        trial_value,
        restricted_ty,
        restricted_value,
    ): (String, String, String, i64, i64, String, bool, String, bool) = env
        .eval(
            r#"
            local hour, minute = GetGameTime()
            return type(GetText),
                GetText("ERR_FAKE_TOKEN"),
                type(GetGameTime),
                hour,
                minute,
                type(IsTrialAccount),
                IsTrialAccount(),
                type(IsRestrictedAccount),
                IsRestrictedAccount()
            "#,
        )
        .unwrap();

    assert_eq!(get_text_ty, "function");
    assert_eq!(get_text_value, "ERR_FAKE_TOKEN");
    assert_eq!(game_time_ty, "function");
    assert!((0..=23).contains(&hour));
    assert!((0..=59).contains(&minute));
    assert_eq!(trial_ty, "function");
    assert!(!trial_value);
    assert_eq!(restricted_ty, "function");
    assert!(!restricted_value);

    let (
        stream_ty,
        stream_value,
        callstack_ty,
        callstack_height,
        arena_ty,
        arena_specs,
        kiosk_ty,
        kiosk_enabled_ty,
        kiosk_enabled,
    ): (String, i64, String, i64, String, i64, String, String, bool) = env
        .eval(
            r#"
            return type(GetFileStreamingStatus),
                GetFileStreamingStatus(),
                type(GetErrorCallstackHeight),
                GetErrorCallstackHeight(),
                type(GetNumArenaOpponentSpecs),
                GetNumArenaOpponentSpecs(),
                type(Kiosk),
                type(Kiosk and Kiosk.IsEnabled),
                Kiosk and Kiosk.IsEnabled and Kiosk.IsEnabled() or false
            "#,
        )
        .unwrap();

    assert_eq!(stream_ty, "function");
    assert_eq!(stream_value, 0);
    assert_eq!(callstack_ty, "function");
    assert_eq!(callstack_height, 0);
    assert_eq!(arena_ty, "function");
    assert_eq!(arena_specs, 0);
    assert_eq!(kiosk_ty, "table");
    assert_eq!(kiosk_enabled_ty, "function");
    assert!(!kiosk_enabled);
}
#[cfg(feature = "retail-12-0-7")]
#[test]
fn test_patch_12_0_7_safe_global_bridges() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local initialBNetFriends = C_BattleNet.GetNumFriends()
            C_BattleNet.InviteFriend("Jaina#3000")
            if C_BattleNet.GetNumFriends() ~= initialBNetFriends + 1 then return "bnet-invite-count" end
            local invitedInfo = C_BattleNet.GetFriendAccountInfo(initialBNetFriends + 1)
            if type(invitedInfo) ~= "table" then return "bnet-invite-info" end
            if invitedInfo.battleTag ~= "Jaina#3000" then return "bnet-invite-tag" end
            if invitedInfo.accountName ~= "Jaina" then return "bnet-invite-account" end
            if invitedInfo.isFriend ~= true then return "bnet-invite-friend" end
            C_BattleNet.InviteFriend("Jaina#3000")
            if C_BattleNet.GetNumFriends() ~= initialBNetFriends + 1 then return "bnet-invite-duplicate" end
            if type(C_Container.CalculateTotalNumberOfFreeBagSlots()) ~= "number" then return "container-free" end
            if type(C_DelvesUI.GetDelveEntranceTitleString(1)) ~= "string" then return "delve-title" end
            if type(C_DelvesUI.GetWorldTierDifficultyForActivePlayer()) ~= "number" then return "delve-world-tier" end
            local clock = C_DurationUtil.CreateManualClock(5)
            if clock:GetTime() ~= 5 then return "clock-time" end
            clock:AdvanceTime(2)
            if clock:GetTime() ~= 7 then return "clock-advance" end
            clock:RewindTime(3)
            if clock:GetTime() ~= 4 then return "clock-rewind" end
            clock:ResetTime()
            if clock:GetTime() ~= 0 then return "clock-reset" end
            local binding = C_DurationUtil.CreateDurationTextBinding(10)
            if binding:GetDuration() ~= 10 then return "duration-value" end
            binding:SetDuration(3)
            if binding:GetFormattedText() ~= "3" then return "duration-text" end
            if binding:IsEnabled() ~= true then return "duration-enabled" end
            binding:Disable()
            if binding:IsActive() ~= false then return "duration-disabled" end
            binding:Enable()
            if binding:GetClock() == nil then return "duration-clock" end
            if type(binding.HasSecretValues) ~= "function" then return "duration-secret-method" end
            if binding:HasSecretValues() ~= false then return "duration-secret-values" end
            if type(binding.SetEnabled) ~= "function" then return "duration-set-enabled-method" end
            binding:SetEnabled(false)
            if binding:IsEnabled() ~= false or binding:IsActive() ~= false then return "duration-set-disabled" end
            binding:SetEnabled(true)
            if binding:IsEnabled() ~= true then return "duration-set-enabled" end
            local durationObject = C_DurationUtil.CreateDuration()
            binding:SetDuration(durationObject)
            if binding:GetDuration() ~= durationObject then return "duration-object" end
            if type(binding.SetToDefaults) ~= "function" then return "duration-defaults-method" end
            binding:SetExpiredText("expired")
            binding:SetZeroDurationText("zero")
            binding:SetToDefaults()
            if binding:IsEnabled() ~= true then return "duration-default-enabled" end
            if binding:GetExpiredText() ~= nil or binding:GetZeroDurationText() ~= nil then return "duration-default-text" end
            local r, g, b, a = C_EncounterTimeline.GetEventColor(1)
            if r ~= 1 or g ~= 1 or b ~= 1 or a ~= 1 then return "event-color" end
            C_EncounterEvents.SetEventColor(1, { r = 0.2, g = 0.4, b = 0.6, a = 0.8 })
            r, g, b, a = C_EncounterTimeline.GetEventColor(1)
            if r ~= 0.2 or g ~= 0.4 or b ~= 0.6 or a ~= 0.8 then return "event-color-state" end
            C_EncounterEvents.SetEventColor(1, nil)
            r, g, b, a = C_EncounterTimeline.GetEventColor(1)
            if r ~= 1 or g ~= 1 or b ~= 1 or a ~= 1 then return "event-color-cleared" end
            if C_HousingCatalog.GetCatalogCategoryAndSubcategoryNames(1) ~= nil then return "housing-catalog" end
            if C_HousingCustomizeMode.RoomConnectionSupportsDoorType(1, 1) ~= false then return "housing-door" end
            if C_HousingLayout.CanSetViewedFloor(1) ~= false then return "housing-floor" end
            if type(C_MerchantFrame.GetMerchantCurrencies()) ~= "table" then return "merchant-currencies" end
            local readyFrame = CreateFrame("Frame")
            readyFrame.events = {}
            readyFrame:RegisterEvent("READY_CHECK")
            readyFrame:RegisterEvent("READY_CHECK_CONFIRM")
            readyFrame:RegisterEvent("READY_CHECK_FINISHED")
            readyFrame:SetScript("OnEvent", function(self, event, ...)
                table.insert(self.events, { event, ... })
            end)
            if GetReadyCheckStatus("player") ~= nil then return "ready-initial-status" end
            C_PartyInfo.DoReadyCheck()
            if readyFrame.events[1] == nil or readyFrame.events[1][1] ~= "READY_CHECK" then return "ready-start-event" end
            if GetReadyCheckStatus("player") ~= "waiting" then return "ready-waiting-status" end
            if GetReadyCheckTimeLeft() <= 0 then return "ready-time-left" end
            C_PartyInfo.ConfirmReadyCheck(true)
            if readyFrame.events[2][1] ~= "READY_CHECK_CONFIRM" then return "ready-confirm-event" end
            if readyFrame.events[2][2] ~= "player" or readyFrame.events[2][3] ~= true then return "ready-confirm-args" end
            if readyFrame.events[3][1] ~= "READY_CHECK_FINISHED" then return "ready-finished-event" end
            if GetReadyCheckStatus("player") ~= "ready" then return "ready-confirmed-status" end
            if GetReadyCheckTimeLeft() ~= 0 then return "ready-finished-time" end
            C_PartyInfo.DoReadyCheck()
            C_PartyInfo.ConfirmReadyCheck(false)
            if GetReadyCheckStatus("player") ~= "notready" then return "ready-declined-status" end
            C_PartyInfo.DemoteAssistant("player")
            if C_PartyInfo.IsGUIDInGroup("Player-1") ~= false then return "party-guid" end
            C_PartyInfo.PromoteToAssistant("player")
            C_PartyInfo.PromoteToLeader("player")
            C_PartyInfo.SetEveryoneIsAssistant(true)
            C_PartyInfo.UninviteUnit("party1")
            C_PingSecure.SetPendingPingOffScreenCallback(function() return "ping" end)
            if type(GetSecurePendingPingOffScreenCallback()) ~= "function" then return "ping-callback" end
            C_PingSecure.ClearPendingPingOffScreenCallback()
            if GetSecurePendingPingOffScreenCallback() ~= nil then return "ping-clear" end
            if type(C_QuestHub.GetDragonridingRacesForAreaPOI(1)) ~= "table" then return "dragonraces" end
            if C_UIFileAsset.GetFileID(123) ~= 123 then return "file-id-number" end
            if C_UIFileAsset.GetFileID("Interface\\Icons\\Trade_Engineering.blp") ~= 136243 then return "file-id-known-path" end
            if C_UIFileAsset.GetFileID("Interface/Unknown") ~= nil then return "file-id-unknown-path" end
            if C_UIFileAsset.IsKnownFile("Interface/Icons/Trade_Engineering.blp") ~= true then return "file-known-path" end
            if C_UIFileAsset.IsKnownFile("Interface/Unknown") ~= false then return "file-known-unknown" end
            if C_UIFileAsset.IsLooseFile("Interface/Unknown") ~= false then return "file-loose" end
            if GetEventCPUUsage() ~= 0 then return "event-cpu" end
            if GetFunctionCPUUsage(function() end) ~= 0 then return "function-cpu" end
            if GetScriptCPUUsage(CreateFrame("Frame"), "OnShow") ~= 0 then return "script-cpu" end
            local buttonCallback = function() return "button" end
            SetSecurePendingButtonCallback(buttonCallback)
            if GetSecurePendingButtonCallback() ~= buttonCallback then return "button-callback" end
            local toggleCallback = function() return "toggle" end
            SetSecurePendingToggleRunCallback(toggleCallback)
            if GetSecurePendingToggleRunCallback() ~= toggleCallback then return "toggle-callback" end
            if type(GameTooltip_AddMoneyLine) ~= "function" then return "money-line" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_cvars_and_enums_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (screen_narration, discord_enabled, ping_target, group_buff, raid_warning, tiered_lairs): (
        String,
        String,
        String,
        String,
        String,
        String,
    ) = env
        .eval(
            r#"
            return type(GetCVar("accessibilityScreenNarrationEnabled")),
                type(GetCVar("discordClientEnabled")),
                type(GetCVar("pingTarget")),
                type(Enum.CooldownViewerCategory.GroupBuff),
                type(Enum.EditModeSystem.RaidWarning),
                type(Enum.TieredEntranceType.Lairs)
            "#,
        )
        .unwrap();

    assert_eq!(screen_narration, "string");
    assert_eq!(discord_enabled, "string");
    assert_eq!(ping_target, "string");
    assert_eq!(group_buff, "number");
    assert_eq!(raid_warning, "number");
    assert_eq!(tiered_lairs, "number");
}

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_strict_removed_symbols_are_hidden() {
    let env = WowLuaEnv::new().unwrap();
    crate::ptr::compat_bootstrap::apply_post_load(&env);
    crate::ptr::compat_bootstrap::apply_strict_removals(&env);
    let result: String = env
        .eval(
            r#"
            if type(C_DyeColor) == "table" and C_DyeColor.GetDyeColorForItemLocation ~= nil then return "dye-location" end
            if type(C_DyeColor) == "table" and C_DyeColor.GetDyeColorForItem ~= nil then return "dye-item" end
            if type(C_Housing) == "table" and C_Housing.IsInsideOwnHouse ~= nil then return "housing-own" end
            if type(C_Ping) == "table" and C_Ping.GetContextualPingTypeForUnit ~= nil then return "ping-context" end
            if type(C_RecruitAFriend) == "table" and C_RecruitAFriend.IsEnabled ~= nil then return "raf-enabled" end
            if type(C_SuperTrack) == "table" and C_SuperTrack.GetNextWaypointForMap ~= nil then return "supertrack-waypoint" end
            if type(C_UnitAuras) == "table" and C_UnitAuras.TriggerPrivateAuraShowDispelType ~= nil then return "private-aura-dispel" end
            if GetInventorySlotInfo ~= nil then return "inventory-global" end
            if Enum.EditModeUnitFrameSetting.IconSize ~= nil then return "editmode-iconsize" end
            if GetCVar("lastLockedDelvesCompanionAbilities") ~= nil then return "delves-cvar" end
            if GetCVar("SlugSupersampling") ~= nil then return "slug-cvar" end
            local frame = CreateFrame("Frame")
            if pcall(function() frame:RegisterEvent("BATTLETAG_INVITE_SHOW") end) then return "battletag-event" end
            if pcall(function() frame:RegisterUnitEvent("BATTLETAG_INVITE_SHOW", "player") end) then return "battletag-unit-event" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_safe_global_bridges() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            if C_CVar.AreCVarsLoaded() ~= true then return "cvars" end
            local durationBinding = C_DurationUtil.CreateDurationTextBinding(5)
            if type(durationBinding.ClearTextColorCurve) ~= "function" then return "duration-color-clear" end
            if type(durationBinding.GetFormattedTextColor) ~= "function" then return "duration-color-formatted" end
            if type(durationBinding.GetTextColorCurve) ~= "function" then return "duration-color-get" end
            if type(durationBinding.SetTextColorCurve) ~= "function" then return "duration-color-set" end
            local curve = { r = 1 }
            durationBinding:SetTextColorCurve(curve)
            if durationBinding:GetTextColorCurve() ~= curve then return "duration-color-curve" end
            local r, g, b, a = durationBinding:GetFormattedTextColor()
            if r ~= 1 or g ~= 1 or b ~= 1 or a ~= 1 then return "duration-color-rgba" end
            durationBinding:ClearTextColorCurve()
            if durationBinding:GetTextColorCurve() ~= nil then return "duration-color-cleared" end
            if C_CombatAudioAlert.SpeakText("test") ~= 0 then return "combat-audio" end
            if #C_CooldownViewer.GetGroupBuffItems() ~= 0 then return "cooldown-group-buffs" end
            if #C_DyeColor.GetDyeColorsForItem(1) ~= 0 then return "dye-item" end
            if #C_DyeColor.GetDyeColorsForItemLocation({}) ~= 0 then return "dye-location" end
            if type(C_DelvesUI.GetFlavorNodeForCompanion(1)) ~= "number" then return "delve-flavor-node" end
            if type(C_DelvesUI.GetFlavorNodeNameForCompanion(1)) ~= "string" then return "delve-flavor-name" end
            if C_GuildInfo.IsDiscordStreamSeparate() ~= false then return "guild-discord" end
            if C_Sound.PlaySoundWithOptions(1, { volumeOverride = 0.5 }) ~= nil then return "sound" end
            if C_Navigation.GetNextWaypointForMap(1) ~= nil then return "navigation" end
            if type(C_Ping.SendMacroPing({ type = Enum.PingSubjectType.ActionReady })) ~= "number" then return "ping-macro" end
            local _, _, _, _, _, _, _, _, _, _, _, speciesID = C_PetJournal.GetPetInfoByIndex(1)
            if speciesID then
                local petInfo = C_PetJournal.GetPetInfoTableBySpeciesID(speciesID)
                if type(petInfo) ~= "table" or petInfo.speciesID ~= speciesID then return "pet-table" end
                if petInfo.canAttachToDecor ~= false or petInfo.creatureModelScale ~= 1 then return "pet-12-1-fields" end
            end
            local delveTierInfo = C_DelvesUI.GetActiveDelveTier()
            if type(delveTierInfo) == "table" then
                if delveTierInfo.overrideTooltipSpellID ~= nil then return "delve-override-tooltip" end
                if delveTierInfo.isLFG ~= false then return "delve-is-lfg" end
            end
            local lfgInfo = C_LFGList.GetSearchResultInfo(7)
            if type(lfgInfo) == "table" and lfgInfo.censored ~= false then return "lfg-censored-field" end
            if C_PvP.CanSurrenderArena() ~= false then return "pvp" end
            if C_EncounterJournal.GetBaseDifficultyID(63) ~= 1 then return "ej-base-difficulty-dungeon" end
            if C_EncounterJournal.GetBaseDifficultyID(72) ~= 14 then return "ej-base-difficulty-raid" end
            if C_EncounterJournal.GetBaseDifficultyID(999999) ~= nil then return "ej-base-difficulty-invalid" end
            if C_EncounterJournal.InstanceHasDifficultyID(63, 1) ~= true then return "ej-instance-difficulty-dungeon" end
            if C_EncounterJournal.InstanceHasDifficultyID(63, 14) ~= false then return "ej-instance-difficulty-dungeon-raid" end
            if C_EncounterJournal.InstanceHasDifficultyID(72, 14) ~= true then return "ej-instance-difficulty-raid" end
            if C_EncounterJournal.InstanceHasDifficultyID(999999, 14) ~= false then return "ej-instance-difficulty-invalid" end
            if C_QuestHub.IsAreaPOICurrentlyRelatedToHub(1, 2) ~= false then return "quest-hub" end
            if C_RecruitAFriend.IsSystemEnabled() ~= false then return "raf-enabled" end
            if C_RecruitAFriend.IsSystemSupported() ~= false then return "raf-supported" end
            local canSummon, summonReason = C_RecruitAFriend.CanSummonFriend("player")
            if canSummon ~= false or summonReason ~= nil then return "raf-summon" end
            if C_Roleset.ApplyRolesetFilters({}) ~= true then return "roleset" end
            if C_SocialQueue.IsSystemEnabled() ~= false then return "social-queue-enabled" end
            if C_SocialQueue.IsSystemSupported() ~= false then return "social-queue-supported" end
            if C_FriendList.IsLegacyFriendSystemEnabled() ~= false then return "legacy-friends" end
            if C_SocialRestrictions.IsFriendsDisabled() ~= false then return "friends-disabled" end
            if C_SocialUI.IsSystemEnabled() ~= false then return "social-ui" end
            if C_Discord.IsEnabled() ~= false then return "discord-enabled" end
            SetCVar("discordClientEnabled", "1")
            if C_Discord.IsEnabled() ~= true then return "discord-enabled-cvar" end
            SetCVar("discordClientEnabled", "0")
            if C_Discord.IsEnabled() ~= false then return "discord-disabled-cvar" end
            if C_Discord.IsUserOAuthed() ~= false then return "discord-oauth" end
            if C_Discord.GetNumDiscordChannels() ~= 0 then return "discord-channels" end
            if C_Discord.GetNumDiscordServers() ~= 0 then return "discord-servers" end
            if type(C_Discord.GetServerLinkableChannels()) ~= "table" then return "discord-linkable" end
            C_Discord.Authorize()
            C_Discord.GuildLink()
            C_Discord.GuildUnlink()
            C_Discord.RefreshAuth()
            C_Discord.SetGuildSetting("setting", true)
            C_Discord.UpdateDiscordServers()
            C_Discord.UpdateGuildLobby()
            if C_Discord.GetDiscordChannelName(1) ~= nil then return "discord-channel-name" end
            if C_Discord.GetDiscordUserID() ~= nil then return "discord-user-id" end
            if C_Discord.GetDisplayNameType() ~= 0 then return "discord-display-type" end
            if C_Discord.GetGuildLinkStatus() ~= nil then return "discord-guild-status" end
            if C_Discord.GetServerName(1) ~= nil then return "discord-server-name" end
            if C_Discord.IsGuildChannelLinked(1) ~= false then return "discord-linked" end
            if C_Discord.IsGuildSettingSet("setting") ~= false then return "discord-setting" end
            C_LFGList.ConfirmCensoredActiveEntry()
            C_LFGList.RevealCensoredActiveEntry()
            C_LFGList.RevealCensoredSearchResult(1)
            if C_LFGList.DoesCensoredTextMatch("foo", "bar") ~= false then return "lfg-censored-match" end
            if C_LFGList.IsCensoredActiveEntryUnresolved() ~= false then return "lfg-censored-active" end
            if C_BattleNet.AreFriendTagsEnabled() ~= false then return "bnet-tags" end
            if C_BattleNet.AreTitleFriendsEnabled() ~= false then return "bnet-title" end
            if C_BattleNet.AreTitleFriendCustomNamesEnabled() ~= false then return "bnet-title-names" end
            if C_BattleNet.IsBattleNetFriendsListEnabled() ~= false then return "bnet-list-enabled" end
            if C_BattleNet.IsBattleNetFriendsListSupported() ~= false then return "bnet-list-supported" end
            if C_BattleNet.BNCheckTitleFriendInviteToUnit("player") ~= false then return "bnet-title-invite" end
            if C_BattleNet.GetCustomTitleFriendName(1) ~= nil then return "bnet-title-name" end
            if C_BattleNet.GetFriendInviteInfo(1) ~= nil then return "bnet-invite-info" end
            C_BattleNet.SendVerifiedBattleNetFriendInvite("Anduin#4000")
            local inviteInfo = C_BattleNet.GetFriendInviteInfo(1)
            if type(inviteInfo) ~= "table" then return "bnet-invite-created" end
            if inviteInfo.inviteID ~= 1 then return "bnet-invite-id" end
            if inviteInfo.accountName ~= "Anduin" then return "bnet-invite-account" end
            if inviteInfo.battleTag ~= "Anduin#4000" then return "bnet-invite-battletag" end
            if type(inviteInfo.friendLevel) ~= "number" then return "bnet-invite-level" end
            if type(inviteInfo.creationTimestamp) ~= "number" then return "bnet-invite-created-at" end
            C_BattleNet.SendVerifiedBattleNetFriendInvite("Anduin#4000")
            if C_BattleNet.GetFriendInviteInfo(2) ~= nil then return "bnet-invite-duplicate" end
            C_BattleNet.SetAppearOffline(false)
            C_BattleNet.SetCustomTitleFriendName(1, "Light")
            if C_BattleNet.GetCustomTitleFriendName(1) ~= "Light" then return "bnet-title-name-set" end
            C_BattleNet.SetCustomTitleFriendName(1, nil)
            if C_BattleNet.GetCustomTitleFriendName(1) ~= nil then return "bnet-title-name-cleared" end
            C_BattleNet.SetFriendTags(1, { "raid", "guild" })
            local bnetInfo = C_BattleNet.GetFriendAccountInfo(1)
            if type(bnetInfo) == "table" then
                if type(bnetInfo.friendTags) ~= "table" then return "bnet-friend-tags-field" end
                if bnetInfo.friendTags[1] ~= "raid" or bnetInfo.friendTags[2] ~= "guild" then return "bnet-friend-tags-values" end
                if type(bnetInfo.friendLevel) ~= "number" then return "bnet-friend-level-field" end
                if type(bnetInfo.gameAccountInfo) == "table" and type(bnetInfo.gameAccountInfo.classFilename) ~= "string" then return "bnet-class-filename-field" end
            end
            C_UnitAuras.SetGroupBuffVisualAlerts({ [123] = true })
            if C_UnitAuras.GetGroupBuffVisualAlerts()[123] ~= true then return "group-alerts" end
            C_UnitAuras.SetHiddenGroupBuffs({ [456] = true })
            if C_UnitAuras.GetHiddenGroupBuffs()[456] ~= true then return "hidden-buffs" end
            local slotID, icon, checkRelic = C_PaperDollInfo.GetInventorySlotInfo("MainHandSlot")
            if slotID ~= 16 or type(icon) ~= "number" or checkRelic ~= false then return "paperdoll-slot" end
            if C_HouseEditor.GetHouseEditorPlayerType() ~= nil then return "house-editor-player" end
            C_Housing.HouseFinderIgnoreNeighborhood(1)
            if C_Housing.IsInsideOwnedHouseOrPlot() ~= false then return "housing-owned-house-or-plot" end
            if C_Housing.IsInsideOwnedHouse() ~= false then return "housing-owned-house" end
            if C_Housing.IsInsideOwnedPlot() ~= false then return "housing-owned-plot" end
            C_Housing.ResetHouse()
            if C_HousingBlueprint.CanImportTypeFromCurrentLocation(1) ~= false then return "blueprint-can-import" end
            C_HousingBlueprint.DeleteBlueprint("id")
            C_HousingBlueprint.ExportBlueprint("id")
            C_HousingBlueprint.ExportRoomBlueprint("id")
            if C_HousingBlueprint.GetBlueprintHyperlink("code") ~= nil then return "blueprint-link" end
            if C_HousingBlueprint.GetBlueprintTypeForCode("code") ~= nil then return "blueprint-type" end
            if C_HousingBlueprint.GetExportAvailability() ~= nil then return "blueprint-export-availability" end
            if C_HousingBlueprint.GetFeatureAvailability() ~= nil then return "blueprint-feature-availability" end
            if C_HousingBlueprint.GetImportAvailability() ~= nil then return "blueprint-import-availability" end
            C_HousingBlueprint.ImportBlueprint("code")
            if C_HousingBlueprint.IsShareCodeValid("code") ~= false then return "blueprint-share-code" end
            C_HousingBlueprint.RenameBlueprint("id", "name")
            C_HousingBlueprint.RequestBlueprintCollection()
            C_HousingBlueprint.RequestBlueprintContents("id")
            C_HousingBlueprint.RequestBlueprintContentsForContext({})
            C_HousingBlueprint.StartImportRoomBlueprint("code")
            C_HousingCustomizeMode.ApplyPetToSelectedDecor("pet")
            if C_HousingCustomizeMode.GetSelectedDecorPetInfo() ~= nil then return "housing-pet-info" end
            if C_HousingDecor.AnyDecorPlacedInRoom(1) ~= false then return "housing-room-decor" end
            if C_HousingDecor.GetBothMaxPlacementBudgets() ~= nil then return "housing-max-budgets" end
            if C_HousingDecor.GetBothSpentPlacementBudgets() ~= nil then return "housing-spent-budgets" end
            if C_HousingDecor.GetDecorAssignedPetName("guid") ~= nil then return "housing-pet-name" end
            if C_HousingDecor.GetDecorCanAttachPet("guid") ~= false then return "housing-can-attach-pet" end
            if C_HousingDecor.GetMaxPetPlacementBudget() ~= nil then return "housing-max-pets" end
            if C_HousingDecor.GetSpentPetPlacementBudget() ~= nil then return "housing-spent-pets" end
            if C_HousingLayout.GetBaseRoomFloor(1) ~= nil then return "housing-base-floor" end
            if C_HousingLayout.GetRoomPlayerIsIn() ~= nil then return "housing-player-room" end
            if C_HousingLayout.GetSelectedBlueprintFloorplan() ~= nil then return "housing-blueprint-floorplan" end
            if C_HousingLayout.HasSelectedBlueprintFloorplan() ~= false then return "housing-has-blueprint-floorplan" end
            if C_Item.DoesItemMatchSpellItemCondition(6948, 6603) ~= false then return "item-condition" end
            if C_Spell.TargetSpellChecksItemCondition(6603) ~= false then return "spell-condition" end
            local _, spellIcon, conditionalIcon = C_Spell.GetSpellTexture(6603)
            if type(spellIcon) ~= "number" or conditionalIcon ~= nil then return "spell-texture" end
            local slotID2 = C_PaperDollInfo.GetInventorySlotInfoForInvSlot("MainHandSlot")
            if slotID2 ~= 16 then return "paperdoll-invslot" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[test]
fn test_startup_service_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (background_ty, background_status, debugstack_ty, debugstack_value, issecure_ty, secure): (
        String,
        i64,
        String,
        String,
        String,
        bool,
    ) = env
        .eval(
            r#"
            return type(GetBackgroundLoadingStatus),
                GetBackgroundLoadingStatus(),
                type(debugstack),
                debugstack(1),
                type(issecure),
                issecure()
            "#,
        )
        .unwrap();

    assert_eq!(background_ty, "function");
    assert_eq!(background_status, 0);
    assert_eq!(debugstack_ty, "function");
    assert!(
        debugstack_value.contains("in main chunk") || debugstack_value.contains("in function"),
        "expected debugstack output to contain stack frames, got: {debugstack_value:?}"
    );
    assert_eq!(issecure_ty, "function");
    assert!(secure);
}

#[test]
fn debugstack_uses_wow_bracketed_file_locations() {
    let env = WowLuaEnv::new().unwrap();

    let result = env.exec_named(
        r#"
        local function current_folder()
            local stack = debugstack()
            __debugstack_stack = stack
            local _, _, luafilepath = string.find(stack, "[%[](.-)[%]]")
            local i = 1
            local lastPart
            while string.find(luafilepath, "([/].+)", i) do
                local startPoint
                startPoint, _, lastPart = string.find(luafilepath, "([/].+)", i)
                i = startPoint + 1
            end
            return string.gsub(luafilepath, lastPart, "")
        end
        __debugstack_folder = current_folder()
        "#,
        "@Interface/AddOns/Angleur/angTemplates/LegolandoTemplates/Legolando_MouseScanAnim/Legolando_MouseScanAnim.lua",
    );
    if let Err(error) = result {
        let stack: String = env.eval("__debugstack_stack").unwrap_or_default();
        panic!("debugstack path extraction failed: {error}; stack={stack:?}");
    }

    let folder: String = env.eval("__debugstack_folder").unwrap();
    assert_eq!(
        folder,
        "Interface/AddOns/Angleur/angTemplates/LegolandoTemplates/Legolando_MouseScanAnim"
    );
}
#[test]
fn test_old_stack_startup_globals_exist_on_rilua_path() {
    let env = WowLuaEnv::new().unwrap();
    let (
        actionbar_color_method_ty,
        actionbar_r,
        actionbar_g,
        actionbar_b,
        red_rgba_ty,
        raid_class_rgb_ty,
        class_color_ty,
        class_color_rgb_ty,
        spec_get_ty,
        spec_info_ty,
        spec_class_ty,
        model_info_ty,
        timerunning_ty,
        timerunning_season,
        unit_casting_ty,
        unit_casting_nil,
        active_spec_id,
        active_spec_name,
    ): (
        String,
        f64,
        f64,
        f64,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        i64,
        String,
        bool,
        i64,
        String,
    ) = env
        .eval(
            r#"
            local hotkeyR, hotkeyG, hotkeyB = ACTIONBAR_HOTKEY_FONT_COLOR:GetRGB()
            local classColor = C_ClassColor.GetClassColor("PALADIN")
            local activeSpecIndex = C_SpecializationInfo.GetSpecialization()
            local activeSpecID, activeSpecName = C_SpecializationInfo.GetSpecializationInfo(activeSpecIndex)
            return type(ACTIONBAR_HOTKEY_FONT_COLOR.GetRGB),
                hotkeyR, hotkeyG, hotkeyB,
                type(RED_FONT_COLOR.GetRGBA),
                type(RAID_CLASS_COLORS.WARRIOR.GetRGB),
                type(C_ClassColor.GetClassColor),
                type(classColor and classColor.GetRGB),
                type(C_SpecializationInfo.GetSpecialization),
                type(C_SpecializationInfo.GetSpecializationInfo),
                type(C_SpecializationInfo.GetClassIDFromSpecID),
                type(C_ModelInfo.GetModelSceneInfoByID),
                type(PlayerGetTimerunningSeasonID),
                PlayerGetTimerunningSeasonID(),
                type(UnitCastingInfo),
                UnitCastingInfo("player") == nil,
                activeSpecID,
                activeSpecName
            "#,
        )
        .unwrap();

    assert_eq!(actionbar_color_method_ty, "function");
    assert_eq!((actionbar_r, actionbar_g, actionbar_b), (0.6, 0.6, 0.6));
    assert_eq!(red_rgba_ty, "function");
    assert_eq!(raid_class_rgb_ty, "function");
    assert_eq!(class_color_ty, "function");
    assert_eq!(class_color_rgb_ty, "function");
    assert_eq!(spec_get_ty, "function");
    assert_eq!(spec_info_ty, "function");
    assert_eq!(spec_class_ty, "function");
    assert_eq!(model_info_ty, "function");
    assert_eq!(timerunning_ty, "function");
    assert_eq!(timerunning_season, 0);
    assert_eq!(unit_casting_ty, "function");
    assert!(unit_casting_nil);
    assert!(active_spec_id > 0);
    assert!(!active_spec_name.is_empty());
}
#[test]
fn test_old_stack_power_lfg_and_cleanup_globals_exist_on_rilua_path() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("C_WowTokenSecure = nil").unwrap();
    env.exec("LE_GAME_ERR_SPELL_COOLDOWN = nil").unwrap();
    env.restore_post_cleanup_globals();

    let (
        unit_power,
        unit_power_with_type,
        unit_power_max,
        power_type,
        power_token,
        lfg_lfd_ty,
        lfg_lfr_ty,
        can_use_lfd,
        can_use_lfr,
        faction_rgba_ty,
        token_secure_ty,
        token_count,
        price_lock_duration,
        spell_cooldown_err,
    ): (
        i64,
        i64,
        i64,
        i64,
        String,
        String,
        String,
        bool,
        bool,
        String,
        String,
        i64,
        i64,
        i64,
    ) = env
        .eval(
            r#"
            local ptype, ptoken = UnitPowerType("player")
            local canUseLFD = C_LFGInfo.CanPlayerUseLFD()
            local canUseLFR = C_LFGInfo.CanPlayerUseLFR()
            return UnitPower("player"),
                UnitPower("player", 0),
                UnitPowerMax("player"),
                ptype,
                ptoken,
                type(C_LFGInfo.CanPlayerUseLFD),
                type(C_LFGInfo.CanPlayerUseLFR),
                canUseLFD,
                canUseLFR,
                type(FACTION_RED_COLOR.GetRGBA),
                type(C_WowTokenSecure.GetTokenCount),
                C_WowTokenSecure.GetTokenCount(),
                C_WowTokenSecure.GetPriceLockDuration(),
                LE_GAME_ERR_SPELL_COOLDOWN
            "#,
        )
        .unwrap();

    assert_eq!(unit_power, 50_000);
    assert_eq!(unit_power_with_type, 50_000);
    assert_eq!(unit_power_max, 100_000);
    assert_eq!(power_type, 0);
    assert_eq!(power_token, "MANA");
    assert_eq!(lfg_lfd_ty, "function");
    assert_eq!(lfg_lfr_ty, "function");
    assert!(can_use_lfd);
    assert!(can_use_lfr);
    assert_eq!(faction_rgba_ty, "function");
    assert_eq!(token_secure_ty, "function");
    assert_eq!(token_count, 2);
    assert_eq!(price_lock_duration, 900);
    assert_eq!(spell_cooldown_err, 61);
}
#[test]
fn test_startup_social_and_lfg_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        tutorial_ty,
        tutorial_flagged,
        lfg_ty,
        has_active_entry,
        premade_style,
        search_result_ty,
        queue_ty,
        group_count,
        queue_config_ty,
    ): (String, bool, String, bool, i64, String, String, i64, String) = env
        .eval(
            r#"
            local groups = C_SocialQueue.GetAllGroups()
            local config = C_SocialQueue.GetConfig()
            return type(IsTutorialFlagged),
                IsTutorialFlagged(42),
                type(C_LFGList.GetApplications),
                C_LFGList.HasActiveEntryInfo(),
                C_LFGList.GetPremadeGroupFinderStyle(),
                type(C_LFGList.GetSearchResultInfo(7)),
                type(C_SocialQueue.GetAllGroups),
                #groups,
                type(config)
            "#,
        )
        .unwrap();

    assert_eq!(tutorial_ty, "function");
    assert!(!tutorial_flagged);
    assert_eq!(lfg_ty, "function");
    assert!(!has_active_entry);
    assert_eq!(premade_style, 0);
    assert_eq!(search_result_ty, "table");
    assert_eq!(queue_ty, "function");
    assert_eq!(group_count, 0);
    assert_eq!(queue_config_ty, "table");
}

#[test]
fn test_startup_pvp_queue_surfaces_are_safe() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local weeklyProgress = C_WeeklyRewards.GetConquestWeeklyProgress()
            if type(weeklyProgress) ~= "table" then return "weekly_type" end
            if type(weeklyProgress.progress) ~= "number" then return "weekly_progress" end
            if type(weeklyProgress.maxProgress) ~= "number" then return "weekly_max" end
            if type(weeklyProgress.displayType) ~= "number" then return "weekly_display" end
            if type(weeklyProgress.unlocksCompleted) ~= "number" then return "weekly_unlocks" end
            if type(weeklyProgress.maxUnlocks) ~= "number" then return "weekly_max_unlocks" end

            local randomBGInfo = C_PvP.GetRandomBGInfo()
            if type(randomBGInfo) ~= "table" then return "random_bg_type" end
            if type(randomBGInfo.canQueue) ~= "boolean" then return "random_bg_queue" end
            if type(randomBGInfo.minLevel) ~= "number" then return "random_bg_min" end
            if randomBGInfo.name ~= "Random Battleground" then return "random_bg_name" end

            local epicBGInfo = C_PvP.GetRandomEpicBGInfo()
            if type(epicBGInfo) ~= "table" then return "epic_bg_type" end
            if type(epicBGInfo.canQueue) ~= "boolean" then return "epic_bg_queue" end
            if type(epicBGInfo.minLevel) ~= "number" then return "epic_bg_min" end
            if epicBGInfo.name ~= "Random Epic Battleground" then return "epic_bg_name" end

            SetPVPRoles(true, false, true)
            local tank, healer, dps = GetPVPRoles()
            if tank ~= true or healer ~= false or dps ~= true then return "roles" end
            if ClearBattlemaster() ~= nil then return "clear" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[test]
fn test_startup_cursor_surface_accepts_texture_paths() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            if type(SetCursor) ~= "function" then return "set_type" end
            if type(ResetCursor) ~= "function" then return "reset_type" end
            if SetCursor("Interface\\CURSOR\\UI-Cursor-Move.blp") ~= true then return "set_result" end
            ResetCursor()
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}
#[test]
fn test_startup_time_and_service_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        get_time_ty,
        now,
        action_info_ty,
        action_type_value,
        action_id,
        web_ticket_ty,
        web_ticket_value_ty,
        dungeon_ty,
        dungeon_id,
        unit_in_vehicle_ty,
        in_vehicle,
        roles_ty,
        can_tank,
        can_heal,
        can_dps,
    ): (
        String,
        f64,
        String,
        String,
        i64,
        String,
        String,
        String,
        i64,
        String,
        bool,
        String,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local actionType, actionId = GetActionInfo(1)
            local canTank, canHeal, canDps = UnitGetAvailableRoles("player")
            return type(GetTime),
                GetTime(),
                type(GetActionInfo),
                actionType,
                actionId,
                type(GetWebTicket),
                type(GetWebTicket()),
                type(GetDungeonDifficultyID),
                GetDungeonDifficultyID(),
                type(UnitInVehicle),
                UnitInVehicle("player"),
                type(UnitGetAvailableRoles),
                canTank,
                canHeal,
                canDps
            "#,
        )
        .unwrap();

    assert_eq!(get_time_ty, "function");
    assert!(now >= 0.0);
    assert_eq!(action_info_ty, "function");
    assert_eq!(action_type_value, "spell");
    assert!(action_id > 0);
    assert_eq!(web_ticket_ty, "function");
    assert_eq!(web_ticket_value_ty, "nil");
    assert_eq!(dungeon_ty, "function");
    assert_eq!(dungeon_id, 1);
    assert_eq!(unit_in_vehicle_ty, "function");
    assert!(!in_vehicle);
    assert_eq!(roles_ty, "function");
    assert!(can_tank);
    assert!(can_heal);
    assert!(can_dps);

    let (
        auth_ty,
        auth_succeeded,
        class_trial_ty,
        is_class_trial,
        logout_seconds,
        character_services_ty,
        has_trial_boost,
    ): (String, bool, String, bool, i64, String, bool) = env
        .eval(
            r#"
            return type(C_AuthChallenge.SetFrame),
                C_AuthChallenge.DidChallengeSucceed(),
                type(C_ClassTrial.IsClassTrialCharacter),
                C_ClassTrial.IsClassTrialCharacter(),
                C_ClassTrial.GetClassTrialLogoutTimeSeconds(),
                type(C_CharacterServices.HasRequiredBoostForClassTrial),
                C_CharacterServices.HasRequiredBoostForClassTrial()
            "#,
        )
        .unwrap();

    assert_eq!(auth_ty, "function");
    assert!(!auth_succeeded);
    assert_eq!(class_trial_ty, "function");
    assert!(!is_class_trial);
    assert_eq!(logout_seconds, 0);
    assert_eq!(character_services_ty, "function");
    assert!(!has_trial_boost);
}

#[test]
fn test_addon_startup_time_and_difficulty_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        precise_ty,
        precise_now,
        server_time_ty,
        server_time,
        raid_difficulty_ty,
        raid_difficulty_id,
        legacy_raid_difficulty_ty,
        legacy_raid_difficulty_id,
    ): (String, f64, String, f64, String, i64, String, i64) = env
        .eval(
            r#"
            return type(GetTimePreciseSec),
                GetTimePreciseSec(),
                type(GetServerTime),
                GetServerTime(),
                type(GetRaidDifficultyID),
                GetRaidDifficultyID(),
                type(GetLegacyRaidDifficultyID),
                GetLegacyRaidDifficultyID()
            "#,
        )
        .unwrap();

    assert_eq!(precise_ty, "function");
    assert!(precise_now >= 0.0);
    assert_eq!(server_time_ty, "function");
    assert!(server_time > 1_700_000_000.0);
    assert_eq!(raid_difficulty_ty, "function");
    assert_eq!(raid_difficulty_id, 14);
    assert_eq!(legacy_raid_difficulty_ty, "function");
    assert_eq!(legacy_raid_difficulty_id, 3);
}

#[test]
fn test_addon_startup_frame_scale_and_merchant_namespaces_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (dpi_ty, dpi_scale, merchant_ty, merchant_item_ty, raid_locks_ty, encounter_complete): (
        String,
        f64,
        String,
        String,
        String,
        bool,
    ) = env
        .eval(
            r#"
            local itemInfo = C_MerchantFrame.GetItemInfo(1)
            return type(GetScreenDPIScale),
                GetScreenDPIScale(),
                type(C_MerchantFrame),
                type(itemInfo),
                type(C_RaidLocks),
                C_RaidLocks.IsEncounterComplete(1)
            "#,
        )
        .unwrap();

    assert_eq!(dpi_ty, "function");
    assert_eq!(dpi_scale, 1.0);
    assert_eq!(merchant_ty, "table");
    assert_eq!(merchant_item_ty, "table");
    assert_eq!(raid_locks_ty, "table");
    assert!(!encounter_complete);
}

#[test]
fn test_addon_startup_table_time_and_string_helpers_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        find_ty,
        found_index,
        found_value,
        secure_found_index,
        secure_found_value,
        secure_pair_count,
        difftime_ty,
        time_delta,
        parsed_time,
        colored,
        replaced,
        unmute_result,
    ): (
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        i64,
        String,
        String,
        bool,
    ) = env
        .eval(
            r#"
            local index, value = FindInTableIf({"a", "b"}, function(v) return v == "b" end)
            local secureArray = SecureTypes.CreateSecureArray()
            secureArray:Insert("a")
            secureArray:Insert("b")
            local secureIndex, secureValue = secureArray:FindInTableIf(function(v) return v == "b" end)
            local securePairCount = 0
            for _ in pairs(secureArray) do
                securePairCount = securePairCount + 1
            end
            return type(FindInTableIf),
                index,
                value,
                secureIndex,
                secureValue,
                securePairCount,
                type(difftime),
                difftime(10, 3),
                time({ year = "2024", month = "6", day = "3", hour = "0", min = "41", sec = "7" }),
                ("Requires a reload"):SetColorOrange(),
                ("Hello {name}"):K_ReplaceVars({ name = "Palaky" }),
                UnmuteSoundFile(123)
            "#,
        )
        .unwrap();

    assert_eq!(find_ty, "function");
    assert_eq!(found_index, 2);
    assert_eq!(found_value, "b");
    assert_eq!(secure_found_index, 2);
    assert_eq!(secure_found_value, "b");
    assert_eq!(secure_pair_count, 2);
    assert_eq!(difftime_ty, "function");
    assert_eq!(time_delta, 7);
    assert!(parsed_time > 0);
    assert!(colored.starts_with("|c"));
    assert!(colored.ends_with("|r"));
    assert_eq!(replaced, "Hello Palaky");
    assert!(unmute_result);
}
