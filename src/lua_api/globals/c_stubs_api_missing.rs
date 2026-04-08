//! Missing global function stubs referenced during startup events.
//!
//! Split from c_stubs_api.rs — contains register_missing_globals and all
//! its callees: server/timerunning stubs, PlayerLocation mixin, secure env,
//! timer/bar globals, paperdoll/container stubs, LFG/guild stubs, and
//! ActionButtonUtil.

use crate::lua_api::SimState;
use mlua::{Lua, MultiValue, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Missing global functions referenced during startup events.
pub(crate) fn register_missing_globals(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
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
    register_paperdoll_container_and_misc_stubs(lua, &g, state)?;
    register_secure_env_globals(lua, &g)?;
    register_former_workaround_stubs(lua, &g)?;
    Ok(())
}

/// Server info, character undelete, timerunning, and system requirements stubs.
fn register_server_and_timerunning_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_character_undelete_stubs(lua, g)?;
    register_server_info_stubs(lua, g)?;
    register_timerunning_stubs(lua, g)?;
    register_system_requirements_stubs(lua, g)?;
    Ok(())
}

fn register_character_undelete_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "CheckCharacterUndeleteCooldown",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetCharacterUndeleteStatus",
        lua.create_function(|_, ()| Ok((true, false, 0i32, 0i32)))?,
    )?;
    Ok(())
}

fn register_server_info_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
        "GetPlayersOnServer",
        lua.create_function(|_, ()| Ok((false, 0i32, 0i32)))?,
    )?;
    Ok(())
}

fn register_timerunning_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetActiveTimerunningSeasonID",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
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
    Ok(())
}

fn register_system_requirements_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
    if player_location_already_registered(g)? {
        return Ok(());
    }

    install_player_location_bootstrap(lua)?;
    install_player_location_factories(lua)?;
    install_player_location_source_methods(lua)?;
    install_player_location_state_methods(lua)?;
    Ok(())
}

fn player_location_already_registered(g: &mlua::Table) -> Result<bool> {
    Ok(!g.get::<Value>("PlayerLocation")?.is_nil())
}

fn install_player_location_bootstrap(lua: &Lua) -> Result<()> {
    lua.load(PLAYER_LOCATION_BOOTSTRAP_LUA).exec()
}

fn install_player_location_factories(lua: &Lua) -> Result<()> {
    lua.load(PLAYER_LOCATION_FACTORIES_LUA).exec()
}

fn install_player_location_source_methods(lua: &Lua) -> Result<()> {
    lua.load(PLAYER_LOCATION_SOURCE_METHODS_LUA).exec()
}

fn install_player_location_state_methods(lua: &Lua) -> Result<()> {
    lua.load(PLAYER_LOCATION_STATE_METHODS_LUA).exec()
}

const PLAYER_LOCATION_BOOTSTRAP_LUA: &str = r#"
    PlayerLocation = {};
    PlayerLocationMixin = {};
"#;

const PLAYER_LOCATION_FACTORIES_LUA: &str = r#"
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
"#;

const PLAYER_LOCATION_SOURCE_METHODS_LUA: &str = r#"
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
"#;

const PLAYER_LOCATION_STATE_METHODS_LUA: &str = r#"
    function PlayerLocationMixin:IsValid()
        if self:IsGUID() then
            local guid = self:GetGUID();
            return guid ~= nil and (C_PlayerInfo.GUIDIsPlayer(guid) or C_AccountInfo.IsGUIDBattleNetAccountType(guid));
        elseif self:IsCommunityData() then
            return C_Club.CanResolvePlayerLocationFromClubMessageData(self.communityClubID, self.communityStreamID, self.communityEpoch, self.communityPosition);
        elseif self:IsUnit() then
            local unit = self:GetUnit();
            return unit ~= nil and UnitIsHumanPlayer(unit);
        end

        return self:IsChatLineID() or self:IsBattlefieldScoreIndex() or self:IsVoiceID() or self:IsBattleNetID() or self:IsCommunityInvitation();
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
"#;

const SECURE_TRANSFER_LUA: &str = r#"
    C_SecureTransfer = C_SecureTransfer or {}
    local api = C_SecureTransfer

    api._state = api._state or {
        shouldShowTradeOfferWarning = false,
        tradePartner = nil,
        mailInfo = {
            target = "",
            sendMoney = 0,
        },
        housingPurchaseCost = 0,
        housingPurchaseQuantity = 1,
        housingVCPurchaseProductID = 0,
        acceptTradeCount = 0,
        sendMailCount = 0,
        completeHousingPurchaseCount = 0,
        completeHousingVCPurchaseCount = 0,
        cancelCount = 0,
        lastAction = nil,
    }

    local function normalizeMoney(value)
        local n = tonumber(value)
        if n == nil then
            return 0
        end
        return math.max(0, math.floor(n))
    end

    local function normalizeQuantity(value)
        local n = tonumber(value)
        if n == nil then
            return 1
        end
        return math.max(1, math.floor(n))
    end

    local function normalizeOptionalString(value)
        if type(value) == "string" and value ~= "" then
            return value
        end
        return nil
    end

    local function getMailInfo()
        local state = api._state
        local info = state.mailInfo
        if type(info) ~= "table" then
            info = {}
            state.mailInfo = info
        end
        local target = info.target
        if type(target) ~= "string" then
            target = ""
        end
        return {
            target = target,
            sendMoney = normalizeMoney(info.sendMoney),
        }
    end

    api.AcceptTrade = api.AcceptTrade or function()
        local state = api._state
        state.acceptTradeCount = (tonumber(state.acceptTradeCount) or 0) + 1
        state.lastAction = "AcceptTrade"
    end

    api.Cancel = api.Cancel or function()
        local state = api._state
        state.cancelCount = (tonumber(state.cancelCount) or 0) + 1
        state.lastAction = "Cancel"
    end

    api.CompleteHousingPurchase = api.CompleteHousingPurchase or function()
        local state = api._state
        state.completeHousingPurchaseCount = (tonumber(state.completeHousingPurchaseCount) or 0) + 1
        state.lastAction = "CompleteHousingPurchase"
    end

    api.CompleteHousingVCPurchase = api.CompleteHousingVCPurchase or function()
        local state = api._state
        state.completeHousingVCPurchaseCount = (tonumber(state.completeHousingVCPurchaseCount) or 0) + 1
        state.lastAction = "CompleteHousingVCPurchase"
    end

    api.GetHousingPurchaseCost = api.GetHousingPurchaseCost or function()
        return normalizeMoney(api._state.housingPurchaseCost)
    end

    api.GetHousingPurchaseQuantity = api.GetHousingPurchaseQuantity or function()
        return normalizeQuantity(api._state.housingPurchaseQuantity)
    end

    api.GetHousingVCPurchaseProductID = api.GetHousingVCPurchaseProductID or function()
        return normalizeMoney(api._state.housingVCPurchaseProductID)
    end

    api.GetMailInfo = api.GetMailInfo or function()
        return getMailInfo()
    end

    api.GetTradePartner = api.GetTradePartner or function()
        return normalizeOptionalString(api._state.tradePartner)
    end

    api.SendMail = api.SendMail or function()
        local state = api._state
        state.sendMailCount = (tonumber(state.sendMailCount) or 0) + 1
        state.lastAction = "SendMail"
    end

    api.ShouldShowTradeOfferWarning = api.ShouldShowTradeOfferWarning or function()
        return api._state.shouldShowTradeOfferWarning == true
    end
"#;

fn register_secure_env_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(SECURE_TRANSFER_LUA).exec()?;
    g.get::<mlua::Table>("C_SecureTransfer")
        .and_then(|secure_transfer| g.set("C_SecureTransfer", secure_transfer))?;
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
    register_action_bar_state_stubs(lua, g)?;
    register_timer_query_stubs(lua, g)?;
    register_inventory_bar_stubs(lua, g)?;
    install_cooldown_frame_helpers(lua)?;
    install_communities_util_stub(lua)?;
    install_adventure_guide_util_stub(lua)?;
    Ok(())
}

fn register_action_bar_state_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
    g.set("HasBonusActionBar", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "HasTempShapeshiftActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(())
}

fn register_timer_query_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
    Ok(())
}

fn register_inventory_bar_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set("PutItemInBackpack", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("PutItemInBag", lua.create_function(|_, _bag: i32| Ok(()))?)?;
    Ok(())
}

/// Blizzard_FrameXMLUtil normally defines these global cooldown helpers in Lua.
///
/// Some startup paths reach cooldown widgets before that file has populated the
/// globals, so install a narrow fallback that mirrors Blizzard's logic.
fn install_cooldown_frame_helpers(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        if not CooldownFrame_Set then
            function CooldownFrame_Set(self, start, duration, enable, forceShowDrawEdge, modRate)
                if enable and enable ~= 0 and start > 0 and duration > 0 then
                    self:SetDrawEdge(forceShowDrawEdge)
                    self:SetCooldown(start, duration, modRate)
                else
                    CooldownFrame_Clear(self)
                end
            end

            function CooldownFrame_Clear(self)
                self:Clear()
            end

            function CooldownFrame_SetDisplayAsPercentage(self, percentage)
                local seconds = 100
                local clamped = math.max(0, math.min(1, percentage))
                self:Pause()
                self:SetCooldown(GetTime() - (seconds * clamped), seconds)
            end
        end
    "#,
    )
    .exec()
}

/// Blizzard_FrameXMLUtil normally defines the CommunitiesUtil helper table in Lua.
///
/// The micro menu only needs the unread-message predicate during startup, so we
/// install a narrow fallback that returns false until the real addon loads.
fn install_communities_util_stub(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        if not CommunitiesUtil then
            CommunitiesUtil = {}
        end

        if CommunitiesUtil.DoesAnyCommunityHaveUnreadMessages == nil then
            function CommunitiesUtil.DoesAnyCommunityHaveUnreadMessages()
                return false
            end
        end

        if CommunitiesUtil.DoesCommunityHaveUnreadMessages == nil then
            function CommunitiesUtil.DoesCommunityHaveUnreadMessages()
                return false
            end
        end

        if CommunitiesUtil.DoesOtherCommunityHaveUnreadMessages == nil then
            function CommunitiesUtil.DoesOtherCommunityHaveUnreadMessages()
                return false
            end
        end
    "#,
    )
    .exec()
}

/// Blizzard_FrameXMLUtil normally defines the AdventureGuideUtil helper table in Lua.
///
/// The micro menu only needs the availability check during startup, so the
/// fallback keeps that branch false until the real addon loads.
fn install_adventure_guide_util_stub(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        if not AdventureGuideUtil then
            AdventureGuideUtil = {}
        end

        if AdventureGuideUtil.IsAvailable == nil then
            function AdventureGuideUtil.IsAvailable()
                return false
            end
        end
    "#,
    )
    .exec()
}

/// PaperDoll, container frame, group roster, and miscellaneous stubs.
fn register_paperdoll_container_and_misc_stubs(
    lua: &Lua,
    g: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
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
    super::unit_api_extra::register_group_roster_globals(lua, state)?;
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
    g.set(
        "StoreSecureReference",
        lua.create_function(|_, (_name, _ref): (String, Value)| Ok(()))?,
    )?;
    // inParty, joined, queued, noPartialClear, achievements, lfgComment, slotCount,
    // category, leader, tank, healer, dps
    g.set(
        "GetLFGInfoServer",
        lua.create_function(|_, (_cat, _id): (Value, Value)| {
            Ok((
                false, false, false, false, false, "", 0i32, 0i32, false, false, false, false,
            ))
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
        lua.create_function(|_, _unit: Value| Ok("NONE"))?,
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
    g.set(
        "GetGuildInfo",
        lua.create_function(|_, _unit: Value| Ok(Value::Nil))?,
    )?;
    g.set(
        "UnitHonor",
        lua.create_function(|_, _unit: String| Ok(0i32))?,
    )?;
    g.set(
        "UnitHonorMax",
        lua.create_function(|_, _unit: String| Ok(100i32))?,
    )?;
    g.set(
        "UnitPowerBarTimerInfo",
        lua.create_function(|_, (_unit, _index): (String, i32)| Ok(Value::Nil))?,
    )?;
    g.set("GetSendMailPrice", lua.create_function(|_, ()| Ok(30i32))?)?;
    g.set("GetWebTicket", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    g.set(
        "GetPVPLifetimeStats",
        lua.create_function(|_, ()| Ok((0i32, 0i32)))?,
    )?;
    g.set("GetHonorLevel", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set(
        "UnitPrestige",
        lua.create_function(|_, _unit: Value| Ok(0i32))?,
    )?;
    g.set("ResetCursor", lua.create_function(|_, ()| Ok(()))?)?;
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
