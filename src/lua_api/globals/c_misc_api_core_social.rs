//! Social, player, and chat related C_* namespace stubs.

use mlua::{Lua, Result, Value};

pub(super) fn register_all(lua: &Lua) -> Result<()> {
    register_c_nameplate(lua)?;
    register_c_player_info(lua)?;
    register_c_party_info(lua)?;
    register_c_chat_info(lua)?;
    register_c_pvp(lua)?;
    register_c_friend_list(lua)?;
    Ok(())
}

fn register_c_nameplate(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetNamePlateForUnit",
        lua.create_function(|_, _u: String| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetNamePlates",
        lua.create_function(|lua, _f: Option<bool>| lua.create_table())?,
    )?;
    t.set(
        "SetNamePlateEnemySize",
        lua.create_function(|_, (_w, _h): (f32, f32)| Ok(()))?,
    )?;
    t.set(
        "SetNamePlateFriendlySize",
        lua.create_function(|_, (_w, _h): (f32, f32)| Ok(()))?,
    )?;
    t.set(
        "SetNamePlateSelfSize",
        lua.create_function(|_, (_w, _h): (f32, f32)| Ok(()))?,
    )?;
    t.set(
        "GetNamePlateEnemySize",
        lua.create_function(|_, ()| Ok((110.0_f64, 45.0_f64)))?,
    )?;
    t.set(
        "GetNamePlateFriendlySize",
        lua.create_function(|_, ()| Ok((110.0_f64, 45.0_f64)))?,
    )?;
    t.set(
        "GetNamePlateSelfSize",
        lua.create_function(|_, ()| Ok((110.0_f64, 45.0_f64)))?,
    )?;
    t.set(
        "SetNamePlateSelfClickThrough",
        lua.create_function(|_, _c: bool| Ok(()))?,
    )?;
    t.set(
        "SetNamePlateEnemyClickThrough",
        lua.create_function(|_, _c: bool| Ok(()))?,
    )?;
    t.set(
        "SetNamePlateFriendlyClickThrough",
        lua.create_function(|_, _c: bool| Ok(()))?,
    )?;
    t.set(
        "SetTargetClampingInsets",
        lua.create_function(|_, (_top, _bottom): (f64, f64)| Ok(()))?,
    )?;
    t.set(
        "SetNamePlateSize",
        lua.create_function(|_, (_w, _h): (f64, f64)| Ok(()))?,
    )?;
    lua.globals().set("C_NamePlate", t)?;
    Ok(())
}

fn register_c_player_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    register_c_player_info_mythic(lua, &t)?;
    register_c_player_info_misc(lua, &t)?;
    lua.globals().set("C_PlayerInfo", t)?;
    Ok(())
}

fn register_c_player_info_mythic(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetPlayerMythicPlusRatingSummary",
        lua.create_function(|lua, _u: String| {
            let s = lua.create_table()?;
            s.set("currentSeasonScore", 0.0_f64)?;
            s.set("runs", lua.create_table()?)?;
            Ok(s)
        })?,
    )?;
    t.set(
        "GetContentDifficultyCreatureForPlayer",
        lua.create_function(|_, _u: String| Ok(0i32))?,
    )?;
    t.set(
        "GetContentDifficultyQualityForPlayer",
        lua.create_function(|_, _u: String| Ok(0i32))?,
    )?;
    Ok(())
}

fn register_c_player_info_misc(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "CanPlayerUseMountEquipment",
        lua.create_function(|_, ()| Ok((true, "")))?,
    )?;
    t.set(
        "IsPlayerNPERestricted",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetGlidingInfo",
        lua.create_function(|_, ()| Ok((false, false, 0.0_f64)))?,
    )?;
    t.set(
        "IsPlayerInChromieTime",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "IsTradingPostAvailable",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "IsTutorialsTabAvailable",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "CanPlayerUseEventScheduler",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("IsPlayerInRPE", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("GetDisplayID", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetNativeDisplayID", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set(
        "GetContentDifficultyQuestForPlayer",
        lua.create_function(|_, _id: i32| Ok(1i32))?,
    )?;
    t.set(
        "IsExpansionLandingPageUnlockedForPlayer",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetAlternateFormInfo",
        lua.create_function(|_, ()| Ok((false, false)))?,
    )?;
    t.set(
        "HasVisibleInvSlot",
        lua.create_function(|_, slot: i32| Ok(slot >= 1 && slot <= 19))?,
    )?;
    t.set(
        "IsDisplayRaceNative",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    Ok(())
}

fn register_c_party_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetActiveCategories",
        lua.create_function(|lua, ()| {
            let t = lua.create_table()?;
            t.set(1, 1i32)?;
            Ok(t)
        })?,
    )?;
    t.set(
        "CanFormCrossFactionParties",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    t.set(
        "GetInviteConfirmationInfo",
        lua.create_function(|_, _g: String| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetInviteReferralInfo",
        lua.create_function(|_, _g: String| Ok(Value::Nil))?,
    )?;
    t.set(
        "ConfirmInviteUnit",
        lua.create_function(|_, _g: String| Ok(()))?,
    )?;
    t.set(
        "DeclineInviteUnit",
        lua.create_function(|_, _g: String| Ok(()))?,
    )?;
    t.set(
        "IsPartyFull",
        lua.create_function(|_, _cat: Option<i32>| Ok(false))?,
    )?;
    t.set("CanInvite", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("InviteUnit", lua.create_function(|_, _n: String| Ok(()))?)?;
    t.set(
        "AllowedToDoPartyConversion",
        lua.create_function(|_, _r: bool| Ok(true))?,
    )?;
    t.set(
        "LeaveParty",
        lua.create_function(|_, _cat: Option<i32>| Ok(()))?,
    )?;
    t.set("ConvertToParty", lua.create_function(|_, ()| Ok(()))?)?;
    t.set("ConvertToRaid", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "GetMinLevel",
        lua.create_function(|_, _cat: Option<i32>| Ok(1i32))?,
    )?;
    t.set(
        "GetGatheringRequestInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetInstanceAbandonVoteTime",
        lua.create_function(|_, ()| Ok((0.0f64, 0.0f64)))?,
    )?;
    t.set("IsPartyWalkIn", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "IsCrossFactionParty",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_PartyInfo", t)?;
    Ok(())
}

fn register_c_chat_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "RegisterAddonMessagePrefix",
        lua.create_function(|_, _p: String| Ok(true))?,
    )?;
    t.set(
        "IsAddonMessagePrefixRegistered",
        lua.create_function(|_, _p: String| Ok(false))?,
    )?;
    t.set(
        "SendAddonMessage",
        lua.create_function(
            |_, (_p, _m, _c, _t): (String, String, Option<String>, Option<String>)| Ok(()),
        )?,
    )?;
    t.set(
        "GetRegisteredAddonMessagePrefixes",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set("SendChatMessage", lua.create_function(send_chat_message)?)?;
    t.set(
        "GetNumReservedChatWindows",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    t.set(
        "GetNumActiveChannels",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "IsChannelRegionalForChannelID",
        lua.create_function(|_, _id: Value| Ok(false))?,
    )?;
    t.set(
        "GetChannelShortcutForChannelID",
        lua.create_function(|_, _id: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "PerformEmote",
        lua.create_function(|_, (_emote, _target, _silent): (Value, Value, Value)| Ok(()))?,
    )?;
    register_c_chat_info_extras(lua, &t)?;
    lua.globals().set("C_ChatInfo", t)?;
    Ok(())
}

fn register_c_chat_info_extras(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetColorForChatType",
        lua.create_function(|lua, chat_type: String| {
            let (r, g, b) = chat_type_color(&chat_type);
            let color = lua.create_table()?;
            color.set("r", r)?;
            color.set("g", g)?;
            color.set("b", b)?;
            Ok(color)
        })?,
    )?;
    t.set(
        "ReplaceIconAndGroupExpressions",
        lua.create_function(
            |_, (input, _no_icon, _no_group): (String, Option<bool>, Option<bool>)| Ok(input),
        )?,
    )?;
    t.set(
        "GetGeneralChannelID",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    Ok(())
}

fn chat_type_color(chat_type: &str) -> (f64, f64, f64) {
    match chat_type {
        "SAY" => (1.0, 1.0, 1.0),
        "YELL" => (1.0, 0.25, 0.25),
        "WHISPER" | "WHISPER_INFORM" => (1.0, 0.5, 1.0),
        "GUILD" => (0.25, 1.0, 0.25),
        "OFFICER" => (0.25, 0.75, 0.25),
        "PARTY" | "PARTY_LEADER" => (0.67, 0.67, 1.0),
        "RAID" | "RAID_LEADER" | "RAID_WARNING" => (1.0, 0.5, 0.0),
        "INSTANCE_CHAT" | "INSTANCE_CHAT_LEADER" => (1.0, 0.5, 0.0),
        "EMOTE" => (1.0, 0.5, 0.25),
        "CHANNEL" => (1.0, 0.75, 0.75),
        "SYSTEM" => (1.0, 1.0, 0.0),
        _ => (1.0, 1.0, 1.0),
    }
}

fn send_chat_message(
    lua: &Lua,
    (msg, chat_type, _lang, _target): (String, Option<String>, Option<Value>, Option<String>),
) -> Result<()> {
    let chat_type = chat_type.unwrap_or_else(|| "SAY".to_string());
    let (r, g, b) = match chat_type.as_str() {
        "EMOTE" => ("1.0", "0.5", "0.25"),
        "YELL" => ("1.0", "0.25", "0.25"),
        "PARTY" => ("0.67", "0.67", "1.0"),
        "GUILD" => ("0.25", "1.0", "0.25"),
        "WHISPER" => ("1.0", "0.5", "1.0"),
        _ => ("1.0", "1.0", "1.0"),
    };
    lua.load(format!(
        r#"
        if ChatFrame1 and ChatFrame1.AddMessage then
            local name = UnitName("player") or "Player"
            local msg = ...
            local prefix = ""
            local fmt = GetCVar and GetCVar("showTimestamps")
            if fmt and fmt ~= "" and fmt ~= "none" then
                prefix = date(fmt, time())
            end
            ChatFrame1:AddMessage(
                prefix .. "|Hplayer:" .. name .. "|h[" .. name .. "]|h says: " .. msg,
                {r}, {g}, {b})
        end
        "#
    ))
    .call::<()>(msg)
}

fn register_c_pvp(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetZonePVPInfo",
        lua.create_function(|_, ()| Ok((Value::Nil, false, Value::Nil)))?,
    )?;
    t.set(
        "GetScoreInfo",
        lua.create_function(|_, _i: i32| Ok(Value::Nil))?,
    )?;
    t.set("IsWarModeDesired", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsWarModeActive", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsPVPMap", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsRatedMap", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsInBrawl", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "IsActiveBattlefield",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetOutdoorPvPWaitTime",
        lua.create_function(|_, _map_id: Option<i32>| Ok(Value::Nil))?,
    )?;
    t.set("IsMatchActive", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsMatchComplete", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetActiveMatchState",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetArenaCrowdControlInfo",
        lua.create_function(|_, _unit: Value| Ok((Value::Nil, Value::Nil, Value::Nil)))?,
    )?;
    t.set(
        "IsMatchConsideredArena",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "RequestCrowdControlSpell",
        lua.create_function(|_, _unit: Value| Ok(()))?,
    )?;
    t.set(
        "GetPvpTalentsUnlockedLevel",
        lua.create_function(|_, ()| Ok(10i32))?,
    )?;
    t.set(
        "GetWarModeRewardBonusDefault",
        lua.create_function(|_, ()| Ok(10i32))?,
    )?;
    t.set(
        "GetWarModeRewardBonus",
        lua.create_function(|_, ()| Ok(10i32))?,
    )?;
    t.set(
        "CanToggleWarMode",
        lua.create_function(|_, _desired: Value| Ok(false))?,
    )?;
    t.set("IsWarModeDesired", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "CanToggleWarModeInArea",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "ArePvpTalentsUnlocked",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_PvP", t)?;
    Ok(())
}

fn register_c_friend_list(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetNumFriends", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set(
        "GetNumOnlineFriends",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetFriendInfoByIndex",
        lua.create_function(|_, _i: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetFriendInfoByName",
        lua.create_function(|_, _n: String| Ok(Value::Nil))?,
    )?;
    t.set("IsFriend", lua.create_function(|_, _g: String| Ok(false))?)?;
    lua.globals().set("C_FriendList", t)?;
    Ok(())
}
