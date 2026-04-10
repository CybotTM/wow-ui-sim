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
    super::c_stubs_api_missing_player_location::register_player_location_stub(lua, &g)?;
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
    g.set(
        "GetBonusBarOffset",
        lua.create_function(|lua, ()| {
            let action_bar: mlua::Table = lua.globals().get("C_ActionBar")?;
            match action_bar.get::<Value>("GetBonusBarOffset")? {
                Value::Function(get_bonus_bar_offset) => get_bonus_bar_offset.call::<i32>(()),
                _ => Ok(0),
            }
        })?,
    )?;
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
    register_lfg_queue_stubs(lua, g)?;
    register_lfg_role_and_group_stubs(lua, g)?;
    register_guild_info_stubs(lua, g)?;
    register_pvp_and_mail_stubs(lua, g)?;
    Ok(())
}

fn register_lfg_queue_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
    Ok(())
}

fn register_lfg_role_and_group_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
    Ok(())
}

fn register_guild_info_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
    Ok(())
}

fn register_pvp_and_mail_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
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
