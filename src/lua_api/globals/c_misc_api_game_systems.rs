//! Challenge mode, club/community, and global gameplay/account stubs.

use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

const CHALLENGE_MODE_MAP_IDS: [i32; 8] = [506, 504, 370, 525, 499, 247, 500, 382];

pub(super) fn register_game_system_support(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_c_challenge_mode(lua)?;
    register_c_club(lua)?;
    register_c_club_finder(lua)?;
    register_global_game_stubs(lua, state)?;
    Ok(())
}

fn challenge_mode_map_info(map_id: i32) -> Option<(&'static str, i32)> {
    match map_id {
        506 => Some(("Cinderbrew Meadery", 1980)),
        504 => Some(("Darkflame Cleft", 1860)),
        370 => Some(("Mechagon Workshop", 1920)),
        525 => Some(("Operation: Floodgate", 1980)),
        499 => Some(("Priory of the Sacred Flame", 1950)),
        247 => Some(("The MOTHERLODE!!", 1980)),
        500 => Some(("The Rookery", 1740)),
        382 => Some(("Theater of Pain", 2040)),
        _ => None,
    }
}

fn challenge_mode_affix_info(affix_id: i32) -> Option<(&'static str, &'static str, i64)> {
    match affix_id {
        9 => Some((
            "Tyrannical",
            "Boss enemies have 20% more health and inflict up to 15% increased damage.",
            236401,
        )),
        10 => Some((
            "Fortified",
            "Non-boss enemies have 20% more health and inflict up to 30% increased damage.",
            236402,
        )),
        160 => Some((
            "Challenger's Peril",
            "Dying subtracts 15 seconds from time remaining.",
            136120,
        )),
        148 => Some((
            "Xal'atath's Bargain: Ascendant",
            "While in combat, Xal'atath rains down shadow upon players.",
            4630473,
        )),
        147 => Some((
            "Xal'atath's Bargain: Frenzied",
            "Non-boss enemies become frenzied at 30% health remaining.",
            4630474,
        )),
        149 => Some((
            "Xal'atath's Bargain: Voidbound",
            "Xal'atath opens void portals that empower nearby enemies.",
            4630471,
        )),
        158 => Some((
            "Xal'atath's Bargain: Oblivion",
            "Xal'atath tears open rifts to the void.",
            4630472,
        )),
        _ => None,
    }
}

fn add_challenge_mode_map_methods(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetMapTable",
        lua.create_function(|lua, ()| {
            let map_table = lua.create_table_with_capacity(CHALLENGE_MODE_MAP_IDS.len(), 0)?;
            for (index, map_id) in CHALLENGE_MODE_MAP_IDS.iter().enumerate() {
                map_table.set(index as i64 + 1, *map_id)?;
            }
            Ok(map_table)
        })?,
    )?;
    table.set(
        "GetMapUIInfo",
        lua.create_function(|lua, map_id: i32| match challenge_mode_map_info(map_id) {
            Some((name, time_limit)) => Ok(mlua::MultiValue::from_vec(vec![
                Value::String(lua.create_string(name)?),
                Value::Integer(map_id as i64),
                Value::Integer(time_limit as i64),
                Value::Nil,
                Value::Nil,
                Value::Integer(map_id as i64),
            ])),
            None => Ok(mlua::MultiValue::from_vec(vec![
                Value::Nil,
                Value::Nil,
                Value::Integer(0),
                Value::Nil,
                Value::Nil,
                Value::Nil,
            ])),
        })?,
    )?;
    table.set(
        "GetAffixInfo",
        lua.create_function(
            |lua, affix_id: i32| match challenge_mode_affix_info(affix_id) {
                Some((name, desc, icon)) => Ok((
                    Value::String(lua.create_string(name)?),
                    Value::String(lua.create_string(desc)?),
                    Value::Integer(icon),
                )),
                None => Ok((Value::Nil, Value::Nil, Value::Nil)),
            },
        )?,
    )?;
    Ok(())
}

fn register_c_challenge_mode(lua: &Lua) -> Result<()> {
    let table = lua.create_table()?;
    table.set(
        "IsChallengeModeActive",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    table.set(
        "GetActiveChallengeMapID",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetActiveKeystoneInfo",
        lua.create_function(|_, ()| Ok((0i32, Value::Nil, false)))?,
    )?;
    table.set(
        "GetCompletionInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetDeathCount",
        lua.create_function(|_, ()| Ok((0i32, 0i32)))?,
    )?;
    table.set(
        "GetLeaverPenaltyWarningTimeLeft",
        lua.create_function(|_, ()| Ok(0.0f64))?,
    )?;
    add_challenge_mode_map_methods(lua, &table)?;
    lua.globals().set("C_ChallengeMode", table)?;
    Ok(())
}

fn register_c_club(lua: &Lua) -> Result<()> {
    let table = lua.create_table()?;
    table.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    table.set(
        "GetSubscribedClubs",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    table.set(
        "GetClubInfo",
        lua.create_function(|_, _id: i64| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetStreams",
        lua.create_function(|lua, _id: i64| lua.create_table())?,
    )?;
    table.set(
        "GetClubMembers",
        lua.create_function(|lua, _id: i64| lua.create_table())?,
    )?;
    table.set("FocusMembers", lua.create_function(|_, _id: i64| Ok(()))?)?;
    table.set("UnfocusMembers", lua.create_function(|_, _id: i64| Ok(()))?)?;
    table.set(
        "SetClubPresenceSubscription",
        lua.create_function(|_, _id: i64| Ok(()))?,
    )?;
    table.set(
        "ClearClubPresenceSubscription",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    table.set(
        "GetInvitationsForSelf",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    table.set("IsRestricted", lua.create_function(|_, ()| Ok(0i32))?)?;
    table.set(
        "ShouldAllowClubType",
        lua.create_function(|_, _ct: Value| Ok(false))?,
    )?;
    lua.globals().set("C_Club", table)?;
    Ok(())
}

fn register_c_club_finder(lua: &Lua) -> Result<()> {
    let table = lua.create_table()?;
    table.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    table.set(
        "IsCommunityFinderEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    table.set(
        "IsListingEnabledFromFlags",
        lua.create_function(|_, _f: Option<i32>| Ok(false))?,
    )?;
    table.set(
        "PlayerGetClubInvitationList",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    table.set(
        "PlayerRequestPendingClubsList",
        lua.create_function(|_, _t: Option<i32>| Ok(()))?,
    )?;
    table.set(
        "GetPlayerApplicantLocaleFlags",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    lua.globals().set("C_ClubFinder", table)?;
    Ok(())
}

fn register_global_game_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_global_combat_stubs(lua)?;
    register_global_action_stubs(lua)?;
    register_global_account_stubs(lua, state)?;
    register_actionbar_hotkey_color(lua)?;
    register_unit_stat_constants(lua)?;
    register_store_frame_functions(lua)?;
    register_communities_dialog_stubs(lua)?;
    Ok(())
}

/// Stub dialog frames checked by CommunitiesAddDialogInsecure.lua.
fn register_communities_dialog_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    for name in ["CommunitiesAddDialog", "CommunitiesCreateDialog"] {
        if globals.get::<Value>(name)?.is_nil() {
            let stub = lua.create_table()?;
            let attrs = lua.create_table()?;
            stub.set("__attrs", attrs)?;
            stub.set("IsShown", lua.create_function(|_, ()| Ok(false))?)?;
            stub.set("Hide", lua.create_function(|_, ()| Ok(()))?)?;
            stub.set("GetAttribute", lua.create_function(read_attr)?)?;
            stub.set("SetAttribute", lua.create_function(write_attr)?)?;
            globals.set(name, stub)?;
        }
    }
    globals.set(
        "CommunitiesAvatarPicker_IsShown",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set(
        "CommunitiesAvatarPicker_CloseDialog",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    Ok(())
}

fn read_attr(_: &Lua, (this, key): (mlua::Table, String)) -> Result<Value> {
    let attrs: mlua::Table = this.get("__attrs")?;
    attrs.get::<Value>(key.as_str())
}

fn write_attr(_: &Lua, (this, key, val): (mlua::Table, String, Value)) -> Result<()> {
    let attrs: mlua::Table = this.get("__attrs")?;
    attrs.set(key.as_str(), val)
}

/// LE_UNIT_STAT_* constants and SPELL_STAT*_NAME strings for PaperDollFrame.
fn register_unit_stat_constants(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set("LE_UNIT_STAT_STRENGTH", 1i32)?;
    globals.set("LE_UNIT_STAT_AGILITY", 2i32)?;
    globals.set("LE_UNIT_STAT_STAMINA", 3i32)?;
    globals.set("LE_UNIT_STAT_INTELLECT", 4i32)?;
    globals.set("SPELL_STAT1_NAME", "Strength")?;
    globals.set("SPELL_STAT2_NAME", "Agility")?;
    globals.set("SPELL_STAT3_NAME", "Stamina")?;
    globals.set("SPELL_STAT4_NAME", "Intellect")?;
    globals.set("NUM_STATS", 4i32)?;
    Ok(())
}

/// StoreFrame_IsShown function stub (used by MicroButtons).
fn register_store_frame_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "StoreFrame_IsShown",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set(
        "GetRepairAllCost",
        lua.create_function(|_, ()| Ok((0i64, false)))?,
    )?;
    globals.set(
        "GetGuildRenameRequired",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set("GetNumGuildPerks", lua.create_function(|_, ()| Ok(0i32))?)?;
    globals.set("RequestGuildRewards", lua.create_function(|_, ()| Ok(()))?)?;
    globals.set(
        "AchievementFrame_ToggleAchievementFrame",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    globals.set(
        "ToggleAchievementFrame",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    globals.set(
        "SwitchAchievementSearchTab",
        lua.create_function(|_, _tab: Value| Ok(()))?,
    )?;
    Ok(())
}

fn register_global_combat_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetTotemInfo",
        lua.create_function(|_, _s: i32| Ok((false, Value::Nil, 0.0f64, 0.0f64, Value::Nil)))?,
    )?;
    globals.set(
        "GetNegativeCorruptionEffectInfo",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    globals.set("GetCorruption", lua.create_function(|_, ()| Ok(0.0f64))?)?;
    globals.set(
        "GetCorruptionResistance",
        lua.create_function(|_, ()| Ok(0.0f64))?,
    )?;
    globals.set(
        "UnitHasVehicleUI",
        lua.create_function(|_, _u: Option<String>| Ok(false))?,
    )?;
    globals.set(
        "HasArtifactEquipped",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set(
        "IsInActiveWorldPVP",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set(
        "IsWatchingHonorAsXP",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set(
        "DoEmote",
        lua.create_function(|_, _emote: Option<String>| Ok(()))?,
    )?;
    Ok(())
}

fn register_global_action_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "IsEquippedAction",
        lua.create_function(|_, _s: Option<i32>| Ok(false))?,
    )?;
    globals.set(
        "IsConsumableAction",
        lua.create_function(|_, _s: Option<i32>| Ok(false))?,
    )?;
    globals.set(
        "IsStackableAction",
        lua.create_function(|_, _s: Option<i32>| Ok(false))?,
    )?;
    globals.set(
        "IsItemAction",
        lua.create_function(|_, _s: Option<i32>| Ok(false))?,
    )?;
    // IsCurrentAction has a stateful implementation in action_bar_api.rs; don't overwrite it.
    globals.set(
        "IsAutoRepeatAction",
        lua.create_function(|_, _s: Option<i32>| Ok(false))?,
    )?;
    globals.set(
        "IsAttackAction",
        lua.create_function(|_, _s: Option<i32>| Ok(false))?,
    )?;
    // HasAction/GetActionInfo/GetActionTexture/IsUsableAction/GetActionCooldown are stateful.
    globals.set(
        "GetActionText",
        lua.create_function(|_, _s: Option<i32>| Ok(Value::Nil))?,
    )?;
    globals.set(
        "GetActionCount",
        lua.create_function(|_, _s: Option<i32>| Ok(0i32))?,
    )?;
    globals.set(
        "GetActionCharges",
        lua.create_function(|_, _s: Option<i32>| Ok((0i32, 0i32, 0.0f64, 0.0f64)))?,
    )?;
    globals.set(
        "GetActionLossOfControlCooldown",
        lua.create_function(|_, _s: Option<i32>| Ok((0.0f64, 0.0f64)))?,
    )?;
    Ok(())
}

fn register_global_account_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetExpansionTrialInfo",
        lua.create_function(|_, ()| Ok((false, 0i32)))?,
    )?;
    globals.set(
        "UnitTrialBankedLevels",
        lua.create_function(|_, _u: Option<String>| Ok(0i32))?,
    )?;
    globals.set(
        "IsInGuild",
        lua.create_function({
            let s = Rc::clone(&state);
            move |_, ()| Ok(s.borrow().world.guild_name.is_some())
        })?,
    )?;
    globals.set(
        "GuildQuit",
        lua.create_function({
            let s = state;
            move |lua, ()| {
                let mut st = s.borrow_mut();
                st.world.guild_name = None;
                st.world.guild_rank = None;
                st.world.guild_num_members = 0;
                drop(st);
                let fire: mlua::Function = lua.globals().get("FireEvent")?;
                fire.call::<()>(mlua::MultiValue::from_vec(vec![Value::String(
                    lua.create_string("PLAYER_GUILD_UPDATE")?,
                )]))?;
                Ok(())
            }
        })?,
    )?;
    globals.set(
        "GetGuildLogoInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    globals.set(
        "HasCompletedAnyAchievement",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    globals.set(
        "CanShowAchievementUI",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    globals.set(
        "CanShowEncounterJournal",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    globals.set("SortQuestSortTypes", lua.create_function(|_, ()| Ok(()))?)?;
    globals.set("SortQuests", lua.create_function(|_, ()| Ok(()))?)?;
    globals.set(
        "QuestMapUpdateAllQuests",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    globals.set("QuestPOIUpdateIcons", lua.create_function(|_, ()| Ok(()))?)?;
    Ok(())
}

fn register_actionbar_hotkey_color(lua: &Lua) -> Result<()> {
    let color = lua.create_table()?;
    color.set("r", 0.6f64)?;
    color.set("g", 0.6f64)?;
    color.set("b", 0.6f64)?;
    color.set("a", 1.0f64)?;
    color.set(
        "GetRGB",
        lua.create_function(|_, this: mlua::Table| {
            Ok((
                this.get::<f64>("r")?,
                this.get::<f64>("g")?,
                this.get::<f64>("b")?,
            ))
        })?,
    )?;
    lua.globals().set("ACTIONBAR_HOTKEY_FONT_COLOR", color)?;
    Ok(())
}
