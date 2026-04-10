//! Social, player, and chat related C_* namespace stubs.

use mlua::{Lua, Result, Value};

#[derive(Clone, Copy)]
struct WowFriendRecord {
    name: &'static str,
    connected: bool,
    afk: bool,
    dnd: bool,
    level: i32,
    class_name: &'static str,
    area: &'static str,
    raf_link_type: i32,
    guid: &'static str,
    notes: Option<&'static str>,
}

struct PvpWorldAreaSeed {
    name: &'static str,
    can_enter: bool,
    can_queue: bool,
    is_active: bool,
    min_level: i32,
    start_time: i32,
}

struct PvpHolidayBgSeed {
    bg_id: i32,
    bg_index: i32,
    name: &'static str,
    can_queue: bool,
    min_level: i32,
}

const WOW_FRIENDS: &[WowFriendRecord] = &[
    WowFriendRecord {
        name: "Alyth",
        connected: true,
        afk: false,
        dnd: false,
        level: 80,
        class_name: "Paladin",
        area: "Stormwind City",
        raf_link_type: 0,
        guid: "Player-11-00000001",
        notes: Some("Testing the FriendsFrame list"),
    },
    WowFriendRecord {
        name: "Brom",
        connected: false,
        afk: false,
        dnd: false,
        level: 70,
        class_name: "Monk",
        area: "Orgrimmar",
        raf_link_type: 0,
        guid: "Player-11-00000002",
        notes: None,
    },
];

const WORLD_PVP_AREAS: &[PvpWorldAreaSeed] = &[
    PvpWorldAreaSeed {
        name: "Wintergrasp",
        can_enter: true,
        can_queue: true,
        is_active: true,
        min_level: 80,
        start_time: 900,
    },
    PvpWorldAreaSeed {
        name: "Tol Barad",
        can_enter: true,
        can_queue: false,
        is_active: false,
        min_level: 80,
        start_time: 1800,
    },
];

const HOLIDAY_BG_INFO: PvpHolidayBgSeed = PvpHolidayBgSeed {
    bg_id: 108,
    bg_index: 2,
    name: "Warsong Scramble",
    can_queue: true,
    min_level: 10,
};

const PVP_LOCKLIST_MAP_NAMES: &[(i32, &str)] =
    &[(566, "Eye of the Storm"), (727, "Silvershard Mines")];

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
    t.set(
        "AreOutgoingAddonChatMessagesRestricted",
        lua.create_function(|_, ()| Ok(false))?,
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
    let locklist_maps = lua.create_table()?;
    let world_pvp_areas = seeded_world_pvp_areas(lua)?;
    let holiday_bg_info = seeded_holiday_bg_info(lua)?;
    t.set("__locklistMaps", locklist_maps)?;
    t.set("__worldPVPAreas", world_pvp_areas)?;
    t.set("__holidayBGInfo", holiday_bg_info)?;
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
    let pvp_state = t.clone();
    t.set(
        "GetWorldPVPAreaInfo",
        lua.create_function(move |lua, index: i32| lookup_pvp_world_area(lua, &pvp_state, index))?,
    )?;
    let pvp_state = t.clone();
    t.set(
        "GetHolidayBGInfo",
        lua.create_function(move |_, ()| pvp_state.get::<mlua::Table>("__holidayBGInfo"))?,
    )?;
    let pvp_state = t.clone();
    t.set(
        "GetLocklistMap",
        lua.create_function(move |_, slot: i32| lookup_locklist_map_id(&pvp_state, slot))?,
    )?;
    let pvp_state = t.clone();
    t.set(
        "GetLocklistMapName",
        lua.create_function(move |lua, slot: i32| lookup_locklist_map_name(lua, &pvp_state, slot))?,
    )?;
    let pvp_state = t.clone();
    t.set(
        "SetLocklistMap",
        lua.create_function(move |_, map_id: i32| set_locklist_map(&pvp_state, map_id))?,
    )?;
    let pvp_state = t.clone();
    t.set(
        "ClearLocklistMap",
        lua.create_function(move |_, map_id: i32| clear_locklist_map(&pvp_state, map_id))?,
    )?;
    lua.globals().set("C_PvP", t)?;
    Ok(())
}

fn seeded_world_pvp_areas(lua: &Lua) -> Result<mlua::Table> {
    let areas = lua.create_table()?;
    for (index, area) in WORLD_PVP_AREAS.iter().enumerate() {
        let info = lua.create_table()?;
        info.set("name", area.name)?;
        info.set("canEnter", area.can_enter)?;
        info.set("canQueue", area.can_queue)?;
        info.set("isActive", area.is_active)?;
        info.set("minLevel", area.min_level)?;
        info.set("startTime", area.start_time)?;
        areas.set(index + 1, info)?;
    }
    Ok(areas)
}

fn seeded_holiday_bg_info(lua: &Lua) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("bgID", HOLIDAY_BG_INFO.bg_id)?;
    info.set("bgIndex", HOLIDAY_BG_INFO.bg_index)?;
    info.set("name", HOLIDAY_BG_INFO.name)?;
    info.set("canQueue", HOLIDAY_BG_INFO.can_queue)?;
    info.set("minLevel", HOLIDAY_BG_INFO.min_level)?;
    Ok(info)
}

fn lookup_pvp_world_area(_: &Lua, pvp_state: &mlua::Table, index: i32) -> Result<Value> {
    let areas = pvp_state.get::<mlua::Table>("__worldPVPAreas")?;
    match areas.get::<Value>(index)? {
        Value::Nil => Ok(Value::Nil),
        value => Ok(value),
    }
}

fn lookup_locklist_map_id(pvp_state: &mlua::Table, slot: i32) -> Result<i32> {
    let locklist_maps = pvp_state.get::<mlua::Table>("__locklistMaps")?;
    Ok(locklist_maps.get::<Option<i32>>(slot)?.unwrap_or(0))
}

fn lookup_locklist_map_name(lua: &Lua, pvp_state: &mlua::Table, slot: i32) -> Result<Value> {
    let map_id = lookup_locklist_map_id(pvp_state, slot)?;
    if map_id == 0 {
        return Ok(Value::Nil);
    }

    let map_name = PVP_LOCKLIST_MAP_NAMES
        .iter()
        .find_map(|(candidate_id, candidate_name)| {
            (*candidate_id == map_id).then_some(*candidate_name)
        });
    match map_name {
        Some(name) => Ok(Value::String(lua.create_string(name)?)),
        None => Ok(Value::Nil),
    }
}

fn set_locklist_map(pvp_state: &mlua::Table, map_id: i32) -> Result<()> {
    let locklist_maps = pvp_state.get::<mlua::Table>("__locklistMaps")?;
    let already_present = locklist_contains_map(&locklist_maps, map_id)?;
    if already_present {
        return Ok(());
    }

    for slot in 1..=2 {
        if locklist_maps.get::<Option<i32>>(slot)?.is_none() {
            locklist_maps.set(slot, map_id)?;
            break;
        }
    }
    Ok(())
}

fn clear_locklist_map(pvp_state: &mlua::Table, map_id: i32) -> Result<()> {
    let locklist_maps = pvp_state.get::<mlua::Table>("__locklistMaps")?;
    for slot in 1..=2 {
        if locklist_maps.get::<Option<i32>>(slot)? == Some(map_id) {
            shift_locklist_maps_left(&locklist_maps, slot)?;
            break;
        }
    }
    Ok(())
}

fn locklist_contains_map(locklist_maps: &mlua::Table, map_id: i32) -> Result<bool> {
    for slot in 1..=2 {
        if locklist_maps.get::<Option<i32>>(slot)? == Some(map_id) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn shift_locklist_maps_left(locklist_maps: &mlua::Table, start_slot: i32) -> Result<()> {
    if start_slot == 1 {
        let next_value = locklist_maps.get::<Value>(2)?;
        locklist_maps.set(1, next_value)?;
    }
    locklist_maps.set(2, Value::Nil)?;
    Ok(())
}

fn register_c_friend_list(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let t: mlua::Table = globals
        .get::<mlua::Table>("C_FriendList")
        .unwrap_or_else(|_| lua.create_table().unwrap());
    t.set(
        "GetNumFriends",
        lua.create_function(|_, ()| Ok(WOW_FRIENDS.len() as i32))?,
    )?;
    t.set(
        "GetNumOnlineFriends",
        lua.create_function(|_, ()| {
            Ok(WOW_FRIENDS.iter().filter(|friend| friend.connected).count() as i32)
        })?,
    )?;
    t.set(
        "GetFriendInfoByIndex",
        lua.create_function(get_friend_info_by_index)?,
    )?;
    t.set(
        "GetFriendInfoByName",
        lua.create_function(get_friend_info_by_name)?,
    )?;
    t.set("GetWhoInfo", lua.create_function(get_who_info_by_index)?)?;
    t.set("IsFriend", lua.create_function(is_friend)?)?;
    globals.set("C_FriendList", t)?;
    Ok(())
}

fn get_friend_info_by_index(lua: &Lua, index: i32) -> Result<Value> {
    friend_info_value(lua, lookup_friend_by_index(index))
}

fn get_friend_info_by_name(lua: &Lua, name: String) -> Result<Value> {
    friend_info_value(lua, lookup_friend_by_name(&name))
}

fn get_who_info_by_index(lua: &Lua, index: i32) -> Result<Value> {
    who_info_value(lua, lookup_friend_by_index(index))
}

fn is_friend(_: &Lua, name: String) -> Result<bool> {
    Ok(lookup_friend_by_name(&name).is_some())
}

fn lookup_friend_by_index(index: i32) -> Option<&'static WowFriendRecord> {
    index
        .checked_sub(1)
        .and_then(|zero_based| WOW_FRIENDS.get(zero_based as usize))
}

fn lookup_friend_by_name(name: &str) -> Option<&'static WowFriendRecord> {
    WOW_FRIENDS
        .iter()
        .find(|friend| friend.name.eq_ignore_ascii_case(name))
}

fn friend_info_value(lua: &Lua, friend: Option<&WowFriendRecord>) -> Result<Value> {
    match friend {
        Some(friend) => Ok(Value::Table(create_friend_info_table(lua, friend)?)),
        None => Ok(Value::Nil),
    }
}

fn who_info_value(lua: &Lua, friend: Option<&WowFriendRecord>) -> Result<Value> {
    match friend {
        Some(friend) => Ok(Value::Table(create_who_info_table(lua, friend)?)),
        None => Ok(Value::Nil),
    }
}

fn create_friend_info_table(lua: &Lua, friend: &WowFriendRecord) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("name", friend.name)?;
    info.set("connected", friend.connected)?;
    info.set("afk", friend.afk)?;
    info.set("dnd", friend.dnd)?;
    info.set("level", friend.level)?;
    info.set("className", friend.class_name)?;
    info.set("area", friend.area)?;
    info.set("rafLinkType", friend.raf_link_type)?;
    info.set("guid", friend.guid)?;
    match friend.notes {
        Some(notes) => info.set("notes", notes)?,
        None => info.set("notes", Value::Nil)?,
    }
    Ok(info)
}

fn class_filename_for_friend(friend: &WowFriendRecord) -> Option<&'static str> {
    match friend.class_name {
        "Paladin" => Some("PALADIN"),
        "Monk" => Some("MONK"),
        _ => None,
    }
}

fn who_race_and_gender(friend: &WowFriendRecord) -> (&'static str, i32) {
    match friend.name {
        "Alyth" => ("Human", 2),
        "Brom" => ("Orc", 2),
        _ => ("Unknown", 2),
    }
}

fn create_who_info_table(lua: &Lua, friend: &WowFriendRecord) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    let (race, gender) = who_race_and_gender(friend);
    info.set("fullName", friend.name)?;
    info.set("fullGuildName", "Heroes of Azeroth")?;
    info.set("level", friend.level)?;
    info.set("raceStr", race)?;
    info.set("classStr", friend.class_name)?;
    info.set("area", friend.area)?;
    info.set("filename", class_filename_for_friend(friend))?;
    info.set("gender", gender)?;
    info.set("timerunningSeasonID", Value::Nil)?;
    Ok(info)
}
