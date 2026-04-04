//! Core C_* namespace API stubs.
//!
//! Contains C_ namespaces for game systems:
//! - C_DateAndTime - Calendar time arithmetic (AdjustTimeByDays, AdjustTimeByMinutes, CompareCalendarTime)
//! - C_ScenarioInfo, C_TooltipInfo, C_TradeSkillUI
//! - C_MythicPlus, C_LFGInfo
//!
//! Social/player namespaces live in `c_misc_api_core_social`.
//! Currency/progression namespaces live in `c_misc_api_core_progression`.

use mlua::{Lua, Result, Value};

pub(super) fn register_all(lua: &Lua) -> Result<()> {
    register_c_date_and_time(lua)?;
    register_c_scenario_info(lua)?;
    register_c_tooltip_info_overrides(lua)?;

    register_c_trade_skill(lua)?;
    register_c_mythic_plus(lua)?;
    register_c_lfg_info(lua)?;
    super::c_misc_api_core_social::register_all(lua)?;
    super::c_misc_api_core_progression::register_all(lua)?;
    Ok(())
}

fn register_c_date_and_time(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    let t: mlua::Table = g
        .get::<mlua::Table>("C_DateAndTime")
        .unwrap_or_else(|_| lua.create_table().unwrap());
    t.set(
        "AdjustTimeByDays",
        lua.create_function(adjust_time_by_days)?,
    )?;
    t.set(
        "AdjustTimeByMinutes",
        lua.create_function(adjust_time_by_minutes)?,
    )?;
    t.set(
        "CompareCalendarTime",
        lua.create_function(compare_calendar_time)?,
    )?;
    g.set("C_DateAndTime", t)?;
    Ok(())
}

fn adjust_time_by_days(lua: &Lua, (date, days): (mlua::Table, i64)) -> Result<Value> {
    let epoch_days = calendar_time_to_epoch_days(&date)?;
    epoch_days_to_calendar_time_table(lua, epoch_days + days, &date)
}

fn adjust_time_by_minutes(lua: &Lua, (date, minutes): (mlua::Table, i64)) -> Result<Value> {
    let epoch_days = calendar_time_to_epoch_days(&date)?;
    let hour: i64 = date.get::<i64>("hour").unwrap_or(0);
    let minute: i64 = date.get::<i64>("minute").unwrap_or(0);
    let total_minutes = epoch_days * 1440 + hour * 60 + minute + minutes;
    let new_days = total_minutes.div_euclid(1440);
    let rem = total_minutes.rem_euclid(1440);
    let result = epoch_days_to_calendar_time_table(lua, new_days, &date)?;
    if let Value::Table(ref t) = result {
        t.set("hour", rem / 60)?;
        t.set("minute", rem % 60)?;
    }
    Ok(result)
}

fn compare_calendar_time(_: &Lua, (lhs, rhs): (mlua::Table, mlua::Table)) -> Result<i64> {
    let lhs_mins = calendar_time_to_total_minutes(&lhs)?;
    let rhs_mins = calendar_time_to_total_minutes(&rhs)?;
    // WoW docs: returns -1 if rhs < lhs, 0 if equal, 1 if rhs > lhs
    Ok(match lhs_mins.cmp(&rhs_mins) {
        std::cmp::Ordering::Less => 1i64,
        std::cmp::Ordering::Equal => 0i64,
        std::cmp::Ordering::Greater => -1i64,
    })
}

/// Convert a CalendarTime table to days since the Unix epoch (1970-01-01).
/// Uses Howard Hinnant's civil calendar algorithm.
fn calendar_time_to_epoch_days(date: &mlua::Table) -> mlua::Result<i64> {
    let y: i64 = date.get::<i64>("year")?;
    let m: i64 = date.get::<i64>("month")?;
    let d: i64 = date.get::<i64>("monthDay")?;
    Ok(ymd_to_epoch_days(y, m, d))
}

/// Convert year/month/day to days since 1970-01-01.
/// Algorithm: https://howardhinnant.github.io/date_algorithms.html
fn ymd_to_epoch_days(y: i64, m: i64, d: i64) -> i64 {
    let (y, m) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * m + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Convert days since 1970-01-01 back to (year, month, day).
/// Algorithm: https://howardhinnant.github.io/date_algorithms.html
fn epoch_days_to_ymd(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn days_to_wow_weekday(days: i64) -> i64 {
    // Verified: (days + 4).rem_euclid(7) + 1 gives WoW weekday (1=Sun..7=Sat)
    (days + 4).rem_euclid(7) + 1
}

fn epoch_days_to_calendar_time_table(
    lua: &mlua::Lua,
    days: i64,
    source: &mlua::Table,
) -> mlua::Result<Value> {
    let (y, m, d) = epoch_days_to_ymd(days);
    let weekday = days_to_wow_weekday(days);
    let result = lua.create_table()?;
    result.set("year", y)?;
    result.set("month", m)?;
    result.set("monthDay", d)?;
    result.set("weekday", weekday)?;
    // Preserve hour/minute from source (AdjustTimeByDays does not change them)
    let hour: i64 = source.get::<i64>("hour").unwrap_or(0);
    let minute: i64 = source.get::<i64>("minute").unwrap_or(0);
    result.set("hour", hour)?;
    result.set("minute", minute)?;
    // Preserve optional "day" field (present only for non-Standard gametype)
    match source.get::<Value>("day")? {
        Value::Nil => {}
        v => result.set("day", v)?,
    }
    Ok(Value::Table(result))
}

/// Convert CalendarTime to total minutes since the Unix epoch for ordering.
fn calendar_time_to_total_minutes(date: &mlua::Table) -> mlua::Result<i64> {
    let epoch_days = calendar_time_to_epoch_days(date)?;
    let hour: i64 = date.get::<i64>("hour").unwrap_or(0);
    let minute: i64 = date.get::<i64>("minute").unwrap_or(0);
    Ok(epoch_days * 1440 + hour * 60 + minute)
}

fn register_c_scenario_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetScenarioInfo",
        lua.create_function(|_, ()| {
            Ok(mlua::MultiValue::from_vec(vec![
                Value::Nil,
                Value::Integer(0),
                Value::Integer(0),
                Value::Integer(0),
                Value::Boolean(false),
                Value::Boolean(false),
            ]))
        })?,
    )?;
    t.set(
        "GetScenarioStepInfo",
        lua.create_function(|_, _step: Option<i32>| {
            Ok((Value::Nil, Value::Nil, Value::Integer(0), Value::Integer(0)))
        })?,
    )?;
    t.set(
        "GetCriteriaInfo",
        lua.create_function(|_, _: mlua::MultiValue| {
            Ok((
                Value::Nil,
                Value::Nil,
                Value::Boolean(false),
                Value::Integer(0),
                Value::Integer(0),
            ))
        })?,
    )?;
    t.set(
        "GetCriteriaInfoByStep",
        lua.create_function(|_, _: mlua::MultiValue| {
            Ok((
                Value::Nil,
                Value::Nil,
                Value::Boolean(false),
                Value::Integer(0),
                Value::Integer(0),
            ))
        })?,
    )?;
    t.set("IsInScenario", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_ScenarioInfo", t)?;
    Ok(())
}

fn register_c_tooltip_info_overrides(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let t: mlua::Table = globals
        .get::<mlua::Table>("C_TooltipInfo")
        .unwrap_or_else(|_| lua.create_table().unwrap());
    t.set(
        "GetTraitEntry",
        lua.create_function(create_trait_entry_tooltip)?,
    )?;
    globals.set("C_TooltipInfo", t)?;
    Ok(())
}

fn create_trait_entry_tooltip(lua: &Lua, (entry_id, rank): (i32, Option<i32>)) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_SPELL: i32 = 1;
    const TOOLTIP_LINE_TYPE_SPELL_NAME: i32 = 13;
    const TOOLTIP_LINE_TYPE_SPELL_DESCRIPTION: i32 = 34;

    let rank = rank.unwrap_or(1).max(1) as u32;
    let tooltip = lua.create_table()?;
    tooltip.set("type", TOOLTIP_DATA_TYPE_SPELL)?;

    let lines = lua.create_table()?;
    let mut line_index = 1;

    if let Some(name) = super::traits_api_node::trait_entry_name(entry_id as u32) {
        let line = lua.create_table()?;
        line.set("type", TOOLTIP_LINE_TYPE_SPELL_NAME)?;
        line.set("leftText", name)?;
        lines.set(line_index, line)?;
        line_index += 1;
    }

    if let Some(description) =
        super::traits_api_node::trait_entry_description(entry_id as u32, rank)
        && !description.is_empty()
    {
        let line = lua.create_table()?;
        line.set("type", TOOLTIP_LINE_TYPE_SPELL_DESCRIPTION)?;
        line.set("leftText", description)?;
        line.set("wrapText", true)?;
        lines.set(line_index, line)?;
    }

    tooltip.set("lines", lines)?;
    Ok(Value::Table(tooltip))
}

fn register_c_trade_skill(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;

    t.set(
        "GetTradeSkillLine",
        lua.create_function(|_, ()| Ok((0i32, Value::Nil, 0i32, 0i32)))?,
    )?;
    t.set(
        "GetRecipeInfo",
        lua.create_function(|lua, _id: i32| {
            let info = lua.create_table()?;
            info.set("recipeID", 0)?;
            info.set("name", Value::Nil)?;
            info.set("craftable", false)?;
            Ok(info)
        })?,
    )?;
    t.set(
        "GetRecipeSchematic",
        lua.create_function(|lua, _id: i32| {
            let s = lua.create_table()?;
            s.set("recipeID", 0)?;
            Ok(s)
        })?,
    )?;
    t.set(
        "IsTradeSkillLinked",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("IsNPCCrafting", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetAllRecipeIDs",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetProfessionSkillLineID",
        lua.create_function(|_, _p: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetRecipesTracked",
        lua.create_function(|lua, _is_recraft: bool| lua.create_table())?,
    )?;
    t.set(
        "GetItemReagentQualityByItemInfo",
        lua.create_function(|_, _item: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetItemCraftedQualityByItemInfo",
        lua.create_function(|_, _item: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetItemReagentQualityInfo",
        lua.create_function(|_, _item: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetItemCraftedQualityInfo",
        lua.create_function(|_, _item: Value| Ok(Value::Nil))?,
    )?;

    lua.globals().set("C_TradeSkillUI", t)?;
    Ok(())
}

fn register_c_mythic_plus(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetRunHistory",
        lua.create_function(|lua, _args: mlua::MultiValue| lua.create_table())?,
    )?;
    t.set(
        "GetOwnedKeystoneLevel",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetOwnedKeystoneChallengeMapID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetOwnedKeystoneMapID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetCurrentAffixes",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetRewardLevelFromKeystoneLevel",
        lua.create_function(|_, _l: i32| Ok(0i32))?,
    )?;
    t.set(
        "GetWeeklyBestForMap",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetSeasonInfo",
        lua.create_function(|_, ()| Ok((1i32, 0i32, 0i32)))?,
    )?;
    t.set("GetCurrentSeason", lua.create_function(|_, ()| Ok(1i32))?)?;
    t.set(
        "GetOverallDungeonScore",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    t.set(
        "IsMythicPlusActive",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_MythicPlus", t)?;
    Ok(())
}

fn register_c_lfg_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetRoleCheckDifficultyDetails",
        lua.create_function(|_, ()| Ok((false, false, false)))?,
    )?;
    t.set(
        "GetDungeonInfo",
        lua.create_function(|lua, _id: i32| lua.create_table())?,
    )?;
    t.set(
        "GetLFDLockStates",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "CanPartyLFGBackfill",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetAllEntriesForCategory",
        lua.create_function(|lua, _cat: i32| lua.create_table())?,
    )?;
    t.set(
        "HideNameFromUI",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set(
        "CanPlayerUseLFD",
        lua.create_function(|_, ()| Ok((true, Value::Nil)))?,
    )?;
    t.set(
        "CanPlayerUseLFR",
        lua.create_function(|_, ()| Ok((true, Value::Nil)))?,
    )?;
    t.set(
        "CanPlayerUsePremadeGroup",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    t.set(
        "CanPlayerUseGroupFinder",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    lua.globals().set("C_LFGInfo", t)?;
    Ok(())
}
