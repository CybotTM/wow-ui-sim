//! Missing global function stubs referenced during startup events.
//!
//! Split from c_stubs_api.rs — contains register_missing_globals and all
//! its callees: server/timerunning stubs, PlayerLocation mixin, secure env,
//! timer/bar globals, paperdoll/container stubs, LFG/guild stubs, and
//! ActionButtonUtil.

use mlua::{Lua, MultiValue, Result, Value};

/// Missing global functions referenced during startup events.
pub(crate) fn register_missing_globals(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    super::c_stubs_api_glue::initialize_globals(lua)?;
    register_timer_and_bar_globals(lua, &g)?;
    register_lfg_and_guild_stubs(lua, &g)?;
    register_action_button_util(lua, &g)?;
    register_player_location_stub(lua, &g)?;
    super::c_stubs_api_glue::register_login_state_globals(lua, &g)?;
    super::c_stubs_api_glue::register_character_select_globals(lua, &g)?;
    register_server_and_timerunning_stubs(lua, &g)?;
    register_misc_startup_stubs(lua, &g)?;
    register_paperdoll_container_and_misc_stubs(lua, &g)?;
    register_secure_env_globals(lua, &g)?;
    register_former_workaround_stubs(lua, &g)?;
    Ok(())
}

/// Server info, character undelete, timerunning, and system requirements stubs.
fn register_server_and_timerunning_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "CheckCharacterUndeleteCooldown",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetCharacterUndeleteStatus",
        lua.create_function(|_, ()| Ok((true, false, 0i32, 0i32)))?,
    )?;
    g.set(
        "GetServerName",
        lua.create_function(|_, ()| {
            Ok((
                String::from("Burning Blade"),
                String::new(),
                false,
                false,
                1i32,
            ))
        })?,
    )?;
    g.set(
        "IsConnectedToServer",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    g.set(
        "ShouldShowLevelSquishDialog",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetActiveTimerunningSeasonID",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetPlayersOnServer",
        lua.create_function(|_, ()| Ok((false, 0i32, 0i32)))?,
    )?;
    g.set(
        "GetCharacterTimerunningSeasonID",
        lua.create_function(|_, _guid: Value| Ok(Value::Nil))?,
    )?;
    g.set(
        "IsCharacterTimerunning",
        lua.create_function(|_, _guid: Value| Ok(false))?,
    )?;
    g.set(
        "IsCharacterTimerunningConversionAllowed",
        lua.create_function(|_, _guid: Value| Ok(false))?,
    )?;
    g.set(
        "IsTimerunningEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "HasCheckedSystemRequirements",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    g.set(
        "SetCheckedSystemRequirements",
        lua.create_function(|_, _checked: bool| Ok(()))?,
    )?;
    Ok(())
}

/// AlertFrame, unit roles, LFG role update stubs.
fn register_misc_startup_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "AlertFrame_SetDuration",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    g.set(
        "UnitGetAvailableRoles",
        lua.create_function(|_, _unit: Value| Ok((true, true, true)))?,
    )?;
    g.set(
        "UnitIsGameObject",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetLFGRoleUpdate",
        lua.create_function(|_, ()| Ok((false, 0i32, 0i32, 0i32, 0i32, false)))?,
    )?;
    Ok(())
}

/// C API stubs previously in workarounds.rs Lua patches.
fn register_former_workaround_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetArenaOpponentSpec",
        lua.create_function(|_, _slot: Value| Ok(0i32))?,
    )?;
    g.set(
        "GetLFGStringFromEnum",
        lua.create_function(|_, _enum_val: Value| Ok(String::new()))?,
    )?;
    g.set("UpdateMicroButtons", lua.create_function(|_, ()| Ok(()))?)?;
    g.set(
        "CompactUnitFrame_GetOptionDisplayOnlyDispellableDebuffs",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(())
}

fn register_player_location_stub(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if !g.get::<Value>("PlayerLocation")?.is_nil() {
        return Ok(());
    }

    lua.load(
        r#"
        PlayerLocation = {};
        PlayerLocationMixin = {};

        local function CreatePlayerLocation(fieldName, ...)
            local playerLocation = CreateFromMixins(PlayerLocationMixin);
            if fieldName == "guid" then
                playerLocation:SetGUID(...);
            elseif fieldName == "unit" then
                playerLocation:SetUnit(...);
            elseif fieldName == "chatLineID" then
                playerLocation:SetChatLineID(...);
            elseif fieldName == "communityData" then
                playerLocation:SetCommunityData(...);
            elseif fieldName == "communityInvitation" then
                playerLocation:SetCommunityInvitation(...);
            elseif fieldName == "battlefieldScoreIndex" then
                playerLocation:SetBattlefieldScoreIndex(...);
            elseif fieldName == "voiceID" then
                playerLocation:SetVoiceID(...);
            elseif fieldName == "battleNetID" then
                playerLocation:SetBattleNetID(...);
            end
            return playerLocation;
        end

        function PlayerLocation:CreateFromGUID(guid)
            return CreatePlayerLocation("guid", guid);
        end

        function PlayerLocation:CreateFromUnit(unit)
            return CreatePlayerLocation("unit", unit);
        end

        function PlayerLocation:CreateFromChatLineID(lineID)
            return CreatePlayerLocation("chatLineID", lineID);
        end

        function PlayerLocation:CreateFromCommunityChatData(clubID, streamID, epoch, position)
            return CreatePlayerLocation("communityData", clubID, streamID, epoch, position);
        end

        function PlayerLocation:CreateFromCommunityInvitation(clubID, guid)
            return CreatePlayerLocation("communityInvitation", clubID, guid);
        end

        function PlayerLocation:CreateFromBattlefieldScoreIndex(index)
            return CreatePlayerLocation("battlefieldScoreIndex", index);
        end

        function PlayerLocation:CreateFromVoiceID(memberID, channelID)
            return CreatePlayerLocation("voiceID", memberID, channelID);
        end

        function PlayerLocation:CreateFromBattleNetID(battleNetID)
            return CreatePlayerLocation("battleNetID", battleNetID);
        end

        function PlayerLocationMixin:SetGUID(guid)
            self:ClearAndSetField("guid", guid);
        end

        function PlayerLocationMixin:IsGUID()
            return self.guid ~= nil;
        end

        function PlayerLocationMixin:IsBattleNetGUID()
            return false;
        end

        function PlayerLocationMixin:GetGUID()
            return self.guid or self.communityClubInviterGUID;
        end

        function PlayerLocationMixin:SetUnit(unit)
            self:ClearAndSetField("unit", unit);
        end

        function PlayerLocationMixin:IsUnit()
            return self.unit ~= nil;
        end

        function PlayerLocationMixin:GetUnit()
            return self.unit;
        end

        function PlayerLocationMixin:SetChatLineID(lineID)
            self:ClearAndSetField("chatLineID", lineID);
        end

        function PlayerLocationMixin:IsChatLineID()
            return self.chatLineID ~= nil;
        end

        function PlayerLocationMixin:GetChatLineID()
            return self.chatLineID;
        end

        function PlayerLocationMixin:SetBattlefieldScoreIndex(index)
            self:ClearAndSetField("battlefieldScoreIndex", index);
        end

        function PlayerLocationMixin:IsBattlefieldScoreIndex()
            return self.battlefieldScoreIndex ~= nil;
        end

        function PlayerLocationMixin:GetBattlefieldScoreIndex()
            return self.battlefieldScoreIndex;
        end

        function PlayerLocationMixin:SetVoiceID(memberID, channelID)
            self:Clear();
            self.voiceMemberID = memberID;
            self.voiceChannelID = channelID;
        end

        function PlayerLocationMixin:IsVoiceID()
            return self.voiceMemberID ~= nil and self.voiceChannelID ~= nil;
        end

        function PlayerLocationMixin:GetVoiceID()
            return self.voiceMemberID, self.voiceChannelID;
        end

        function PlayerLocationMixin:SetBattleNetID(battleNetID)
            self:Clear();
            self.battleNetID = battleNetID;
        end

        function PlayerLocationMixin:IsBattleNetID()
            return self.battleNetID ~= nil;
        end

        function PlayerLocationMixin:GetBattleNetID()
            return self.battleNetID;
        end

        function PlayerLocationMixin:SetCommunityData(clubID, streamID, epoch, position)
            self:Clear();
            self.communityClubID = clubID;
            self.communityStreamID = streamID;
            self.communityEpoch = epoch;
            self.communityPosition = position;
        end

        function PlayerLocationMixin:IsCommunityData()
            return self.communityClubID ~= nil and self.communityStreamID ~= nil and self.communityEpoch ~= nil and self.communityPosition ~= nil;
        end

        function PlayerLocationMixin:SetCommunityInvitation(clubID, guid)
            self:Clear();
            self.communityClubID = clubID;
            self.communityClubInviterGUID = guid;
        end

        function PlayerLocationMixin:IsCommunityInvitation()
            return self.communityClubID ~= nil and self.communityClubInviterGUID ~= nil;
        end

        function PlayerLocationMixin:IsValid()
            return true;
        end

        function PlayerLocationMixin:Clear()
            self.guid = nil;
            self.unit = nil;
            self.chatLineID = nil;
            self.battlefieldScoreIndex = nil;
            self.voiceMemberID = nil;
            self.voiceChannelID = nil;
            self.communityClubID = nil;
            self.communityStreamID = nil;
            self.communityEpoch = nil;
            self.communityPosition = nil;
            self.communityClubInviterGUID = nil;
            self.battleNetID = nil;
        end

        function PlayerLocationMixin:ClearAndSetField(fieldName, field)
            self:Clear();
            self[fieldName] = field;
        end
        "#,
    )
    .exec()?;

    Ok(())
}

fn register_secure_env_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "C_SecureTransfer",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "CanAutoSetFKeyBinding",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumShapeshiftForms",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetShapeshiftFormInfo",
        lua.create_function(|_, _index: i32| Ok((Value::Nil, false, false, 0i32)))?,
    )?;
    g.set("GetBonusBarOffset", lua.create_function(|_, ()| Ok(0i32))?)?;
    Ok(())
}

fn register_timer_and_bar_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set("GetDefaultScale", lua.create_function(|_, ()| Ok(1.0f64))?)?;
    g.set(
        "HasVehicleActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "HasOverrideActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetMaxBattlefieldID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set("RequestRaidInfo", lua.create_function(|_, ()| Ok(()))?)?;
    g.set(
        "RequestLFDPlayerLockInfo",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "RequestLFDPartyLockInfo",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetQuestTimers",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetMirrorTimerInfo",
        lua.create_function(|_, _timer: Value| Ok(("UNKNOWN", 0i32, 0i32, -1i32, false, "")))?,
    )?;
    g.set(
        "GetInventoryAlertStatus",
        lua.create_function(|_, _slot: i32| Ok(0i32))?,
    )?;
    g.set(
        "GetWorldElapsedTimers",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetWorldElapsedTime",
        lua.create_function(|_, _id: i32| Ok((0i32, 0i32, 0i32)))?,
    )?;
    g.set("HasBonusActionBar", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "HasTempShapeshiftActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set("PutItemInBackpack", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("PutItemInBag", lua.create_function(|_, _bag: i32| Ok(()))?)?;
    Ok(())
}

/// PaperDoll, container frame, group roster, and miscellaneous stubs.
fn register_paperdoll_container_and_misc_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "PaperDollFrame_SetLevel",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "ContainerFrame_CanContainerUseFilterMenu",
        lua.create_function(|_, _container: Value| Ok(false))?,
    )?;
    g.set(
        "ContainerFrame_IsMainBankBag",
        lua.create_function(|_, _id: Value| Ok(false))?,
    )?;
    g.set(
        "ContainerFrame_IsReagentBag",
        lua.create_function(|_, _id: Value| Ok(false))?,
    )?;
    g.set(
        "IsDisplayChannelLinked",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetDisplayedInviteType",
        lua.create_function(|_, _guid: Value| Ok("INVITE"))?,
    )?;
    g.set(
        "GetNumGroupMembers",
        lua.create_function(|_, _category: Value| Ok(0i32))?,
    )?;
    g.set(
        "GetNumSubgroupMembers",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set("IsInGroup", lua.create_function(|_, _le: Value| Ok(false))?)?;
    if g.get::<Value>("LE_PARTY_CATEGORY_HOME")?.is_nil() {
        g.set("LE_PARTY_CATEGORY_HOME", 1)?;
        g.set("LE_PARTY_CATEGORY_INSTANCE", 2)?;
    }
    Ok(())
}

/// LFG, dungeon finder, guild, and honor global stubs.
fn register_lfg_and_guild_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetLFGMode",
        lua.create_function(|_, _cat: Value| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetLFGQueuedList",
        lua.create_function(|_, _cat: Value| Ok(Value::Nil))?,
    )?;
    // inParty, joined, queued, noPartialClear, achievements, lfgComment, slotCount,
    // category, leader, tank, healer, dps
    g.set(
        "GetLFGInfoServer",
        lua.create_function(|_, (_cat, _id): (Value, Value)| {
            Ok((false, false, false, false, false, "", 0i32, 0i32, false, false, false, false))
        })?,
    )?;
    g.set(
        "GetLFGBootProposal",
        lua.create_function(|_, ()| {
            // inProgress, didVote, myVote, targetName, totalVotes, bootVotes, reason, timeLeft
            Ok((false, false, false, "", 0i32, 0i32, "", 0i32))
        })?,
    )?;
    g.set("GetLFGProposal", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetLFGCompletionReward",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetLFGRandomDungeonInfo",
        lua.create_function(|_, _index: i32| Ok((0i32, false)))?,
    )?;
    g.set(
        "GetNumRandomDungeons",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GuildControlGetNumRanks",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetAvailableLocaleInfo",
        lua.create_function(|lua, _ignore_restrictions: Option<bool>| {
            let entry = lua.create_table()?;
            entry.set("localeId", 1)?;
            entry.set("localeName", "enUS")?;
            let list = lua.create_table()?;
            list.set(1, entry)?;
            Ok(list)
        })?,
    )?;
    g.set(
        "GetGuildFactionGroup",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetGroupMemberCounts",
        lua.create_function(|lua, ()| {
            let t = lua.create_table()?;
            t.set("TANK", 0)?;
            t.set("HEALER", 0)?;
            t.set("DAMAGER", 0)?;
            t.set("NOROLE", 0)?;
            Ok(t)
        })?,
    )?;
    g.set(
        "UnitGroupRolesAssigned",
        lua.create_function(|_, _unit: String| Ok("NONE"))?,
    )?;
    g.set(
        "GetDungeonDifficultyID",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    g.set(
        "RequestGuildChallengeInfo",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GuildControlSetRank",
        lua.create_function(|_, _rank: i32| Ok(()))?,
    )?;
    g.set("IsInGuild", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetGuildInfo",
        lua.create_function(|_, _unit: Value| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetPVPLifetimeStats",
        lua.create_function(|_, ()| Ok((0i32, 0i32)))?,
    )?;
    g.set("GetHonorLevel", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set(
        "UnitPrestige",
        lua.create_function(|_, _unit: Value| Ok(0i32))?,
    )?;
    Ok(())
}

/// ActionButtonUtil enum tables needed by Blizzard_SpellSearch at load time.
/// Blizzard_ActionBar will overwrite this with the full version when it loads.
fn register_action_button_util(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if !g.get::<Value>("ActionButtonUtil")?.is_nil() {
        return Ok(());
    }
    let abu = lua.create_table()?;
    let bar_type = lua.create_table()?;
    bar_type.set("Normal", 1)?;
    bar_type.set("Possess", 2)?;
    bar_type.set("Override", 3)?;
    abu.set("ActionBarType", bar_type)?;
    g.set("ActionButtonUtil", abu)?;
    Ok(())
}
