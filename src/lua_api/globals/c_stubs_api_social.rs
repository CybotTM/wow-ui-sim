//! Social, trial, and feature namespace stubs.
//!
//! Split from c_stubs_api_extra.rs. Contains:
//! - C_ClassTrial, C_RecruitAFriend, C_WowTokenPublic, C_FriendList
//! - C_ReportSystem
//!
//! Shop stubs (C_CatalogShop, C_Who, C_PrivateAuras) are in c_stubs_api_shop.
//! Guild/delves stubs (C_GuildBank, C_PetBattles, C_DelvesUI, etc.) are in c_stubs_api_guild_delves.

use crate::event::{Event, EventArg};
use crate::lua_api::state::{PendingPlayerReport, SimState};
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register trial/social/feature namespaces and misc globals.
pub fn register_social_feature_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_trial_raf_token(lua, g)?;
    register_c_report_system(lua, g)?;
    super::c_stubs_api_shop::register_shop_who_auras(lua, g)?;
    super::c_stubs_api_guild_delves::register_guild_bank_pet_battles(lua, g)?;
    super::c_stubs_api_guild_delves::register_c_delves_ui(lua)?;
    super::c_stubs_api_guild_delves::register_c_zone_ability(lua)?;
    super::c_stubs_api_guild_delves::register_c_auto_complete(lua, g)?;
    super::c_stubs_api_guild_delves::register_c_photo_sharing(lua, g)?;
    super::c_stubs_api_guild_delves::register_auto_complete_globals(lua, g)?;
    super::c_stubs_api_guild_delves::register_misc_global_stubs(lua)?;
    Ok(())
}

fn register_c_report_system(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t: mlua::Table = match g.get::<Value>("C_ReportSystem")? {
        Value::Table(existing) => existing,
        _ => lua.create_table()?,
    };
    t.set(
        "CanReportPlayer",
        lua.create_function(|_, _loc: Value| Ok(true))?,
    )?;
    t.set(
        "CanReportPlayerForLanguage",
        lua.create_function(|_, _loc: Value| Ok(true))?,
    )?;
    t.set(
        "InitiateReportPlayer",
        lua.create_function(initiate_report_player)?,
    )?;
    t.set("SendReportPlayer", lua.create_function(send_report_player)?)?;
    g.set("C_ReportSystem", t)
}

fn initiate_report_player(
    lua: &Lua,
    (report_type, _player_location): (String, Option<Value>),
) -> Result<i64> {
    let Some(state) = lua.app_data_ref::<Rc<RefCell<SimState>>>() else {
        return Ok(0);
    };
    let mut state = state.borrow_mut();
    let report_token = state.next_report_token;
    state.next_report_token += 1;
    state.pending_player_reports.insert(
        report_token,
        PendingPlayerReport {
            report_type,
            comment: None,
        },
    );
    Ok(report_token)
}

fn send_report_player(lua: &Lua, (report_token, comment): (i64, Option<String>)) -> Result<()> {
    let Some(state) = lua.app_data_ref::<Rc<RefCell<SimState>>>() else {
        return Ok(());
    };
    let mut state = state.borrow_mut();
    let Some(mut report) = state.pending_player_reports.remove(&report_token) else {
        return Ok(());
    };
    report.comment = comment.filter(|text| !text.is_empty());
    state.events.push(Event {
        name: "REPORT_PLAYER_RESULT".to_string(),
        args: vec![EventArg::Number(0.0), EventArg::String(report.report_type)],
    });
    Ok(())
}

/// C_ClassTrial, C_RecruitAFriend, C_WowTokenPublic, C_FriendList stubs.
fn register_trial_raf_token(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_c_class_trial(lua, g)?;
    register_c_recruit_a_friend(lua, g)?;
    register_c_wow_token_public(lua, g)?;
    register_c_friend_list(lua, g)?;
    Ok(())
}

/// C_ClassTrial stubs.
fn register_c_class_trial(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "IsClassTrialCharacter",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetClassTrialLogoutTimeSeconds",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set("C_ClassTrial", t)
}

/// C_RecruitAFriend stubs.
fn register_c_recruit_a_friend(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetRecruitInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "IsRecruitingEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("GetRAFInfo", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set(
        "GetRAFSystemInfo",
        lua.create_function(|lua, ()| {
            let info = lua.create_table()?;
            info.set("maxRecruits", 0i32)?;
            info.set("maxRecruitMonths", 0i32)?;
            info.set("maxRewardMonths", 0i32)?;
            info.set("daysInCycle", 30i32)?;
            Ok(info)
        })?,
    )?;
    g.set("C_RecruitAFriend", t)
}

/// C_WowTokenPublic stubs.
fn register_c_wow_token_public(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetCurrentMarketPrice",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set("GetGuaranteedPrice", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("UpdateTokenCount", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "GetCommerceSystemStatus",
        lua.create_function(|_, ()| Ok((false, false, false)))?,
    )?;
    t.set("UpdateMarketPrice", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_WowTokenPublic", t)
}

/// C_FriendList stubs.
fn register_c_friend_list(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t: mlua::Table = match g.get::<Value>("C_FriendList")? {
        Value::Table(existing) => existing,
        _ => lua.create_table()?,
    };
    t.set("SetWhoToUi", lua.create_function(|_, _flag: bool| Ok(()))?)?;
    t.set("SendWho", lua.create_function(|_, _msg: String| Ok(()))?)?;
    t.set("GetNumWhoResults", lua.create_function(|_, ()| Ok(0i32))?)?;
    if t.get::<Value>("GetNumFriends")?.is_nil() {
        t.set("GetNumFriends", lua.create_function(|_, ()| Ok(0i32))?)?;
    }
    if t.get::<Value>("GetNumOnlineFriends")?.is_nil() {
        t.set(
            "GetNumOnlineFriends",
            lua.create_function(|_, ()| Ok(0i32))?,
        )?;
    }
    if t.get::<Value>("GetFriendInfoByIndex")?.is_nil() {
        t.set(
            "GetFriendInfoByIndex",
            lua.create_function(|_, _idx: i32| Ok(Value::Nil))?,
        )?;
    }
    t.set("ShowFriends", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_FriendList", t)
}
