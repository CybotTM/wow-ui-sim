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
    super::c_misc_api_core_tooltip::register_all(lua)?;
    register_profession_globals(lua)?;

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
    t.set("GetScenarioInfo", lua.create_function(stub_scenario_info)?)?;
    t.set(
        "GetScenarioStepInfo",
        lua.create_function(stub_scenario_step_info)?,
    )?;
    t.set("GetCriteriaInfo", lua.create_function(stub_criteria_info)?)?;
    t.set(
        "GetCriteriaInfoByStep",
        lua.create_function(stub_criteria_info)?,
    )?;
    t.set(
        "GetDisplayInfo",
        lua.create_function(stub_scenario_display_info)?,
    )?;
    t.set(
        "GetTieredEntranceActiveSpells",
        lua.create_function(stub_tiered_entrance_active_spells)?,
    )?;
    t.set(
        "IsTieredEntranceScenario",
        lua.create_function(stub_is_tiered_entrance_scenario)?,
    )?;
    t.set("IsInScenario", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_ScenarioInfo", t)?;
    Ok(())
}

/// Stub: returns (nil, 0, 0, 0, false, false) — no active scenario.
fn stub_scenario_info(_: &Lua, _: ()) -> Result<mlua::MultiValue> {
    Ok(mlua::MultiValue::from_vec(vec![
        Value::Nil,
        Value::Integer(0),
        Value::Integer(0),
        Value::Integer(0),
        Value::Boolean(false),
        Value::Boolean(false),
    ]))
}

/// Stub: returns (nil, nil, 0, 0) — no step info.
fn stub_scenario_step_info(_: &Lua, _step: Option<i32>) -> Result<(Value, Value, Value, Value)> {
    Ok((Value::Nil, Value::Nil, Value::Integer(0), Value::Integer(0)))
}

/// Stub: returns (nil, nil, false, 0, 0) — no criteria data.
fn stub_criteria_info(_: &Lua, _: mlua::MultiValue) -> Result<(Value, Value, Value, Value, Value)> {
    Ok((
        Value::Nil,
        Value::Nil,
        Value::Boolean(false),
        Value::Integer(0),
        Value::Integer(0),
    ))
}

/// Stub: no active scenario theme display info.
fn stub_scenario_display_info(_: &Lua, _: ()) -> Result<Value> {
    Ok(Value::Nil)
}

/// Stub: no active tiered entrance spells while outside the scenario.
fn stub_tiered_entrance_active_spells(_: &Lua, _: ()) -> Result<Value> {
    Ok(Value::Nil)
}

/// Stub: the simulator is not in a tiered entrance scenario by default.
fn stub_is_tiered_entrance_scenario(_: &Lua, _: ()) -> Result<bool> {
    Ok(false)
}

fn register_profession_globals(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("GetProfessions", lua.create_function(stub_get_professions)?)?;
    g.set(
        "GetProfessionInfo",
        lua.create_function(build_profession_info)?,
    )?;
    Ok(())
}

/// Returns (prof1, prof2, archaeology, fishing, cooking) — Blacksmithing + Mining.
fn stub_get_professions(_: &Lua, _: ()) -> Result<(Value, Value, Value, Value, Value)> {
    Ok((
        Value::Integer(1), // Blacksmithing
        Value::Integer(2), // Mining
        Value::Nil,        // archaeology
        Value::Nil,        // fishing
        Value::Nil,        // cooking
    ))
}

/// Returns (name, icon, skillLevel, maxSkillLevel, ...) for a 1-based profession index.
fn build_profession_info(lua: &Lua, index: i32) -> Result<mlua::MultiValue> {
    use super::profession_data;
    let Some(p) = profession_data::get_profession_by_index((index - 1) as usize) else {
        return Ok(mlua::MultiValue::new());
    };
    let (num_spells, spell_offset) = profession_spellbook_spell_count_and_offset(p.name);
    Ok(mlua::MultiValue::from_vec(vec![
        Value::String(lua.create_string(p.name)?),
        Value::Integer(p.icon as i64),
        Value::Integer(p.skill_level as i64),
        Value::Integer(p.max_skill_level as i64),
        Value::Integer(num_spells as i64),   // numAbilities
        Value::Integer(spell_offset as i64), // spellOffset
        Value::Integer(p.skill_line_id as i64),
        Value::Integer(p.skill_modifier as i64),
        Value::Integer(0), // specializationIndex
        Value::Integer(0), // specializationOffset
    ]))
}

fn profession_spellbook_spell_count_and_offset(profession_name: &str) -> (i32, i32) {
    for skill_line_index in 1..=super::spellbook_data::num_skill_lines() {
        let Some(skill_line) = super::spellbook_data::get_skill_line(skill_line_index) else {
            continue;
        };
        if skill_line.name == profession_name {
            return (
                skill_line.spells.len() as i32,
                super::spellbook_data::skill_line_offset(skill_line_index),
            );
        }
    }
    (0, 0)
}

fn register_c_trade_skill(lua: &Lua) -> Result<()> {
    use super::profession_data;

    let t = lua.create_table()?;

    t.set(
        "GetTradeSkillLine",
        lua.create_function(|_, ()| {
            let p = profession_data::get_profession_by_index(0);
            match p {
                Some(p) => Ok((
                    p.skill_line_id,
                    Value::Nil,
                    p.skill_level,
                    p.max_skill_level,
                )),
                None => Ok((0i32, Value::Nil, 0i32, 0i32)),
            }
        })?,
    )?;
    t.set("IsTradeSkillReady", lua.create_function(|_, ()| Ok(true))?)?;
    t.set(
        "IsTradeSkillLinked",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("IsNPCCrafting", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsRuneforging", lua.create_function(|_, ()| Ok(false))?)?;
    register_trade_skill_profession_info(lua, &t)?;
    register_trade_skill_recipe_funcs(lua, &t)?;
    register_trade_skill_stubs(lua, &t)?;

    lua.globals().set("C_TradeSkillUI", t)?;
    Ok(())
}

fn register_trade_skill_profession_info(lua: &Lua, t: &mlua::Table) -> Result<()> {
    let first_prof = lua.create_function(stub_first_profession_table)?;
    t.set("GetBaseProfessionInfo", first_prof.clone())?;
    t.set("GetChildProfessionInfo", first_prof)?;
    t.set(
        "GetChildProfessionInfos",
        lua.create_function(build_all_profession_tables)?,
    )?;
    t.set(
        "GetTradeSkillTexture",
        lua.create_function(stub_trade_skill_texture)?,
    )?;
    t.set(
        "GetProfessionSkillLineID",
        lua.create_function(|_, _p: Value| Ok(164i32))?,
    )?;
    t.set(
        "SetProfessionChildSkillLineID",
        lua.create_function(|_, _id: Value| Ok(()))?,
    )?;
    Ok(())
}

/// Returns profession table for the first (primary) profession.
fn stub_first_profession_table(lua: &Lua, _: ()) -> Result<Value> {
    build_profession_table(lua, super::profession_data::get_profession_by_index(0))
}

/// Returns a table of all profession info tables (1-indexed).
fn build_all_profession_tables(lua: &Lua, _: ()) -> Result<mlua::Table> {
    let tbl = lua.create_table()?;
    for (i, p) in super::profession_data::PROFESSIONS.iter().enumerate() {
        tbl.set(i + 1, build_profession_table_inner(lua, p)?)?;
    }
    Ok(tbl)
}

/// Returns the icon fileDataID of the first profession.
fn stub_trade_skill_texture(_: &Lua, _id: Value) -> Result<i32> {
    Ok(super::profession_data::get_profession_by_index(0).map_or(0, |p| p.icon))
}

fn register_trade_skill_recipe_funcs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetAllRecipeIDs",
        lua.create_function(build_all_recipe_id_table)?,
    )?;
    t.set(
        "GetFilteredRecipeIDs",
        lua.create_function(build_filtered_recipe_id_table)?,
    )?;
    t.set(
        "GetRecipeInfo",
        lua.create_function(|lua, id: i32| build_recipe_info_table(lua, id))?,
    )?;
    t.set(
        "GetRecipeSchematic",
        lua.create_function(|lua, id: i32| build_recipe_schematic_table(lua, id))?,
    )?;
    t.set(
        "GetCategoryInfo",
        lua.create_function(build_category_info_table)?,
    )?;
    t.set(
        "IsRecipeInSkillLine",
        lua.create_function(|_, (_rid, _pid): (i32, i32)| Ok(true))?,
    )?;
    Ok(())
}

/// Converts an i32 slice to a 1-indexed Lua table.
fn ids_to_lua_table(lua: &Lua, ids: &[i32]) -> Result<mlua::Table> {
    let tbl = lua.create_table()?;
    for (i, id) in ids.iter().enumerate() {
        tbl.set(i + 1, *id)?;
    }
    Ok(tbl)
}

fn build_all_recipe_id_table(lua: &Lua, _: ()) -> Result<mlua::Table> {
    ids_to_lua_table(lua, &super::profession_data::get_all_recipe_ids())
}

fn build_filtered_recipe_id_table(lua: &Lua, _: ()) -> Result<mlua::Table> {
    ids_to_lua_table(lua, &super::profession_data::get_filtered_recipe_ids())
}

/// Returns category info table {categoryID, name, parentCategoryID, uiOrder}.
fn build_category_info_table(lua: &Lua, cat_id: i32) -> Result<Value> {
    let Some(c) = super::profession_data::get_category(cat_id) else {
        return Ok(Value::Nil);
    };
    let tbl = lua.create_table()?;
    tbl.set("categoryID", c.category_id)?;
    tbl.set("name", c.name)?;
    tbl.set("parentCategoryID", c.parent_category_id)?;
    tbl.set("uiOrder", c.ui_order)?;
    Ok(Value::Table(tbl))
}

fn register_trade_skill_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_trade_skill_nil_stubs(lua, t)?;
    t.set(
        "GetRecipesTracked",
        lua.create_function(|lua, _: Value| lua.create_table())?,
    )?;
    t.set(
        "GetRecipeRequirements",
        lua.create_function(|lua, _: i32| lua.create_table())?,
    )?;
    t.set(
        "GetQualitiesForRecipe",
        lua.create_function(|lua, _: i32| lua.create_table())?,
    )?;
    t.set(
        "IsRecipeFavorite",
        lua.create_function(|_, _: i32| Ok(false))?,
    )?;
    t.set(
        "SetRecipeFavorite",
        lua.create_function(|_, _: (i32, bool)| Ok(()))?,
    )?;
    t.set(
        "CraftRecipe",
        lua.create_function(|_, _: mlua::Variadic<Value>| Ok(()))?,
    )?;
    Ok(())
}

/// Quality/reagent info stubs that all return nil for a single Value arg.
fn register_trade_skill_nil_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    let nil_stub = lua.create_function(|_, _: Value| Ok(Value::Nil))?;
    for name in [
        "GetItemReagentQualityByItemInfo",
        "GetItemCraftedQualityByItemInfo",
        "GetItemReagentQualityInfo",
        "GetItemCraftedQualityInfo",
    ] {
        t.set(name, nil_stub.clone())?;
    }
    Ok(())
}

fn build_profession_table(
    lua: &Lua,
    prof: Option<&super::profession_data::ProfessionInfo>,
) -> mlua::Result<Value> {
    match prof {
        Some(p) => Ok(Value::Table(build_profession_table_inner(lua, p)?)),
        None => {
            let tbl = lua.create_table()?;
            tbl.set("professionID", 0)?;
            Ok(Value::Table(tbl))
        }
    }
}

fn build_profession_table_inner(
    lua: &Lua,
    p: &super::profession_data::ProfessionInfo,
) -> mlua::Result<mlua::Table> {
    let tbl = lua.create_table()?;
    tbl.set("professionID", p.profession_id)?;
    tbl.set("profession", p.profession)?;
    tbl.set("professionName", p.name)?;
    tbl.set("parentProfessionName", p.parent_profession_name)?;
    tbl.set("skillLevel", p.skill_level)?;
    tbl.set("maxSkillLevel", p.max_skill_level)?;
    tbl.set("skillModifier", p.skill_modifier)?;
    tbl.set("skillLineID", p.skill_line_id)?;
    Ok(tbl)
}

fn build_recipe_info_table(lua: &Lua, recipe_id: i32) -> mlua::Result<Value> {
    use super::profession_data;
    match profession_data::get_recipe(recipe_id) {
        Some(r) => {
            let tbl = lua.create_table()?;
            tbl.set("recipeID", r.recipe_id)?;
            tbl.set("name", r.name)?;
            tbl.set("learned", r.learned)?;
            tbl.set("craftable", r.craftable)?;
            tbl.set("difficulty", r.difficulty)?;
            tbl.set("categoryID", r.category_id)?;
            tbl.set("itemLevel", r.item_level)?;
            tbl.set("favorite", false)?;
            Ok(Value::Table(tbl))
        }
        None => {
            let tbl = lua.create_table()?;
            tbl.set("recipeID", 0)?;
            tbl.set("name", Value::Nil)?;
            tbl.set("craftable", false)?;
            Ok(Value::Table(tbl))
        }
    }
}

fn build_recipe_schematic_table(lua: &Lua, recipe_id: i32) -> mlua::Result<Value> {
    use super::profession_data;
    let tbl = lua.create_table()?;
    match profession_data::get_recipe(recipe_id) {
        Some(r) => {
            tbl.set("recipeID", r.recipe_id)?;
            tbl.set("name", r.name)?;
            tbl.set("outputItemID", r.output_item_id)?;
            tbl.set("quantityMin", r.output_quantity)?;
            tbl.set("quantityMax", r.output_quantity)?;
            let reagents_tbl = lua.create_table()?;
            for (i, reagent) in r.reagents.iter().enumerate() {
                let slot = lua.create_table()?;
                let items = lua.create_table()?;
                let item = lua.create_table()?;
                item.set("itemID", reagent.item_id)?;
                item.set("quantity", reagent.quantity)?;
                items.set(1, item)?;
                slot.set("reagents", items)?;
                slot.set("quantityRequired", reagent.quantity)?;
                reagents_tbl.set(i + 1, slot)?;
            }
            tbl.set("reagentSlotSchematics", reagents_tbl)?;
        }
        None => {
            tbl.set("recipeID", 0)?;
        }
    }
    Ok(Value::Table(tbl))
}

fn register_c_mythic_plus(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    register_mythic_plus_stubs(lua, &t)?;
    t.set(
        "GetWeeklyBestForMap",
        lua.create_function(|_, _: i32| Ok(Value::Nil))?,
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

/// Bulk-register mythic+ stubs: zero-returning keystone info and empty-table getters.
fn register_mythic_plus_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    let zero = lua.create_function(|_, ()| Ok(0i32))?;
    for name in [
        "GetOwnedKeystoneLevel",
        "GetOwnedKeystoneChallengeMapID",
        "GetOwnedKeystoneMapID",
    ] {
        t.set(name, zero.clone())?;
    }
    t.set(
        "GetRewardLevelFromKeystoneLevel",
        lua.create_function(|_, _: i32| Ok(0i32))?,
    )?;
    let empty_table = lua.create_function(|lua, _: mlua::MultiValue| lua.create_table())?;
    t.set("GetRunHistory", empty_table.clone())?;
    t.set("GetCurrentAffixes", empty_table)?;
    Ok(())
}

fn register_c_lfg_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    register_lfg_info_stubs(lua, &t)?;
    t.set(
        "GetRoleCheckDifficultyDetails",
        lua.create_function(|_, ()| Ok((false, false, false)))?,
    )?;
    t.set(
        "CanPartyLFGBackfill",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "HideNameFromUI",
        lua.create_function(|_, _: i32| Ok(false))?,
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

/// LFG stubs: empty-table getters and (true, nil) capability checks.
fn register_lfg_info_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    let empty = lua.create_function(|lua, _: mlua::MultiValue| lua.create_table())?;
    for name in [
        "GetDungeonInfo",
        "GetLFDLockStates",
        "GetAllEntriesForCategory",
    ] {
        t.set(name, empty.clone())?;
    }
    let can_use = lua.create_function(|_, ()| Ok((true, Value::Nil)))?;
    for name in ["CanPlayerUseLFD", "CanPlayerUseLFR"] {
        t.set(name, can_use.clone())?;
    }
    Ok(())
}
