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

fn register_c_tooltip_info_overrides(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let t: mlua::Table = globals
        .get::<mlua::Table>("C_TooltipInfo")
        .unwrap_or_else(|_| lua.create_table().unwrap());
    t.set(
        "GetTraitEntry",
        lua.create_function(create_trait_entry_tooltip)?,
    )?;
    t.set("GetItemByID", lua.create_function(create_item_tooltip)?)?;
    t.set(
        "GetInventoryItem",
        lua.create_function(create_inventory_item_tooltip)?,
    )?;
    t.set("GetSpellByID", lua.create_function(create_spell_tooltip)?)?;
    t.set(
        "GetUnitBuff",
        lua.create_function(create_unit_buff_tooltip)?,
    )?;
    t.set(
        "GetUnitDebuff",
        lua.create_function(create_unit_debuff_tooltip)?,
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

fn create_item_tooltip(lua: &Lua, item_id: i32) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_ITEM: i32 = 0;

    let Some(item) = crate::items::get_item(item_id as u32) else {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    };
    build_filled_item_tooltip(lua, item, TOOLTIP_DATA_TYPE_ITEM)
}

fn create_inventory_item_tooltip(lua: &Lua, (_unit, slot): (String, i32)) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_ITEM: i32 = 0;

    let Some(item_id) = super::c_item_api_globals::get_equipped_item_id(lua, slot) else {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    };
    create_item_tooltip(lua, item_id as i32)
}

fn create_spell_tooltip(lua: &Lua, spell_id: i32) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_SPELL: i32 = 1;

    let Some(spell) = crate::spells::get_spell(spell_id as u32) else {
        return build_empty_tooltip(lua, TOOLTIP_DATA_TYPE_SPELL);
    };

    let tooltip = lua.create_table()?;
    tooltip.set("type", TOOLTIP_DATA_TYPE_SPELL)?;

    let lines = lua.create_table()?;
    build_spell_tooltip_lines(lua, spell_id, spell.name, &lines)?;
    tooltip.set("lines", lines)?;
    Ok(Value::Table(tooltip))
}

fn create_unit_buff_tooltip(
    lua: &Lua,
    (unit, index, filter): (String, i32, Option<String>),
) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_UNIT_AURA: i32 = 7;

    let Some(aura) = lookup_player_aura_for_tooltip(lua, &unit, index, filter.as_deref()) else {
        return build_empty_tooltip(lua, TOOLTIP_DATA_TYPE_UNIT_AURA);
    };

    build_aura_tooltip(lua, aura, TOOLTIP_DATA_TYPE_UNIT_AURA)
}

fn create_unit_debuff_tooltip(
    lua: &Lua,
    (_unit, _index, _filter): (String, i32, Option<String>),
) -> Result<Value> {
    build_empty_tooltip(lua, 7)
}

fn build_empty_tooltip(lua: &Lua, tooltip_type: i32) -> Result<Value> {
    let tooltip = lua.create_table()?;
    tooltip.set("type", tooltip_type)?;
    tooltip.set("lines", lua.create_table()?)?;
    Ok(Value::Table(tooltip))
}

fn build_aura_tooltip(
    lua: &Lua,
    aura: crate::lua_api::game_data::AuraInfo,
    tooltip_type: i32,
) -> Result<Value> {
    const TOOLTIP_LINE_TYPE_NONE: i32 = 0;
    const TOOLTIP_LINE_TYPE_SPELL_NAME: i32 = 13;
    const TOOLTIP_LINE_TYPE_SPELL_DESCRIPTION: i32 = 34;

    let tooltip = lua.create_table()?;
    tooltip.set("type", tooltip_type)?;

    let lines = lua.create_table()?;
    append_tooltip_line(lua, &lines, 1, TOOLTIP_LINE_TYPE_SPELL_NAME, &aura.name)?;

    let mut next_index = 2;
    if aura.duration > 0.0 {
        let duration_text = format_aura_duration_text(aura.duration);
        append_tooltip_line(
            lua,
            &lines,
            next_index,
            TOOLTIP_LINE_TYPE_NONE,
            &duration_text,
        )?;
        next_index += 1;
    }

    if let Some(description_text) = aura_spell_description_text(aura.spell_id) {
        append_tooltip_line(
            lua,
            &lines,
            next_index,
            TOOLTIP_LINE_TYPE_SPELL_DESCRIPTION,
            &description_text,
        )?;
    }

    tooltip.set("lines", lines)?;
    Ok(Value::Table(tooltip))
}

fn build_spell_tooltip_lines(
    lua: &Lua,
    spell_id: i32,
    spell_name: &str,
    lines: &mlua::Table,
) -> Result<()> {
    const TOOLTIP_LINE_TYPE_NONE: i32 = 0;
    const TOOLTIP_LINE_TYPE_SPELL_NAME: i32 = 13;
    const TOOLTIP_LINE_TYPE_SPELL_DESCRIPTION: i32 = 34;

    append_tooltip_line(lua, lines, 1, TOOLTIP_LINE_TYPE_SPELL_NAME, spell_name)?;

    let mut next_index = 2;
    if let Some(power_text) = spell_power_text(spell_id) {
        append_tooltip_line(lua, lines, next_index, TOOLTIP_LINE_TYPE_NONE, &power_text)?;
        next_index += 1;
    }

    let cast_time_text = spell_cast_time_text(spell_id);
    append_tooltip_line(
        lua,
        lines,
        next_index,
        TOOLTIP_LINE_TYPE_NONE,
        &cast_time_text,
    )?;
    next_index += 1;

    if let Some(description_text) = spell_description_text(spell_id) {
        append_tooltip_line(
            lua,
            lines,
            next_index,
            TOOLTIP_LINE_TYPE_SPELL_DESCRIPTION,
            &description_text,
        )?;
    }
    Ok(())
}

fn append_tooltip_line(
    lua: &Lua,
    lines: &mlua::Table,
    index: i32,
    line_type: i32,
    text: &str,
) -> Result<()> {
    const TOOLTIP_LINE_TYPE_SPELL_DESCRIPTION: i32 = 34;

    let line = lua.create_table()?;
    line.set("type", line_type)?;
    line.set("leftText", text)?;
    if line_type == TOOLTIP_LINE_TYPE_SPELL_DESCRIPTION {
        line.set("wrapText", true)?;
    }
    lines.set(index, line)?;
    Ok(())
}

fn lookup_player_aura_for_tooltip(
    lua: &Lua,
    unit: &str,
    index: i32,
    filter: Option<&str>,
) -> Option<crate::lua_api::game_data::AuraInfo> {
    if unit != "player" || index < 1 {
        return None;
    }
    if filter.is_some_and(|f| f.contains("HARMFUL") || f.contains("MAW")) {
        return None;
    }

    let state_rc = crate::lua_api::frame::get_sim_state(lua);
    let state = state_rc.borrow();
    state.player.buffs.get((index - 1) as usize).cloned()
}

fn aura_spell_description_text(spell_id: i32) -> Option<String> {
    let description = crate::spell_descriptions::get_spell_description(spell_id as u32)?;
    if description.is_empty() {
        None
    } else {
        Some(strip_html_tags(description))
    }
}

fn format_aura_duration_text(seconds: f64) -> String {
    let secs = seconds as u64;
    if secs >= 3600 {
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        if mins > 0 {
            format!("{hours} hr {mins} min")
        } else {
            format!("{hours} hr")
        }
    } else if secs >= 60 {
        let mins = secs / 60;
        let remaining = secs % 60;
        if remaining > 0 {
            format!("{mins} min {remaining} sec")
        } else {
            format!("{mins} min")
        }
    } else {
        format!("{secs} sec")
    }
}

fn spell_power_text(spell_id: i32) -> Option<String> {
    let costs = crate::spell_power::get_spell_power(spell_id as u32)?;
    let cost = costs.first()?;
    let type_name = crate::spell_power::power_type_name(cost.power_type);
    if cost.cost_pct > 0.0 {
        Some(format!("{}% of Base {}", cost.cost_pct, type_name))
    } else if cost.mana_cost > 0 {
        Some(format!("{} {}", cost.mana_cost, type_name))
    } else {
        None
    }
}

fn spell_cast_time_text(spell_id: i32) -> String {
    let cast_time_ms = super::spell_api::spell_cast_time(spell_id);
    if cast_time_ms > 0 {
        let secs = cast_time_ms as f64 / 1000.0;
        if (secs - secs.round()).abs() < 0.001 {
            format!("{} sec cast", secs as i32)
        } else {
            format!("{:.1} sec cast", secs)
        }
    } else {
        "Instant".to_string()
    }
}

fn spell_description_text(spell_id: i32) -> Option<String> {
    let description = crate::spell_descriptions::get_spell_description(spell_id as u32)?;
    if description.is_empty() {
        None
    } else {
        Some(strip_html_tags(description))
    }
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}

fn build_empty_item_tooltip(lua: &Lua, tooltip_type: i32) -> Result<Value> {
    build_empty_tooltip(lua, tooltip_type)
}

fn build_filled_item_tooltip(
    lua: &Lua,
    item: &crate::items::ItemInfo,
    tooltip_type: i32,
) -> Result<Value> {
    let tooltip = lua.create_table()?;
    tooltip.set("type", tooltip_type)?;

    let lines = lua.create_table()?;
    build_item_tooltip_lines(lua, item, &lines)?;

    tooltip.set("lines", lines)?;
    Ok(Value::Table(tooltip))
}

fn build_item_tooltip_lines(
    lua: &Lua,
    item: &crate::items::ItemInfo,
    lines: &mlua::Table,
) -> Result<()> {
    const TOOLTIP_LINE_TYPE_ITEM_BINDING: i32 = 20;
    const TOOLTIP_LINE_TYPE_EQUIP_SLOT: i32 = 21;
    const TOOLTIP_LINE_TYPE_ITEM_NAME: i32 = 22;
    const TOOLTIP_LINE_TYPE_ITEM_LEVEL: i32 = 31;

    append_item_name_line(lua, lines, 1, TOOLTIP_LINE_TYPE_ITEM_NAME, item)?;
    let item_level_text = format!("Item Level {}", item.item_level);
    append_item_tooltip_line(
        lua,
        lines,
        2,
        TOOLTIP_LINE_TYPE_ITEM_LEVEL,
        &item_level_text,
    )?;

    let equip_slot = super::c_item_api::item_equip_slot_label(item.inventory_type);
    let mut next_index = 3;
    if !equip_slot.is_empty() {
        append_item_tooltip_line(
            lua,
            lines,
            next_index,
            TOOLTIP_LINE_TYPE_EQUIP_SLOT,
            equip_slot,
        )?;
        next_index += 1;
    }

    if let Some(binding_text) = item_binding_text(item.bonding) {
        append_item_tooltip_line(
            lua,
            lines,
            next_index,
            TOOLTIP_LINE_TYPE_ITEM_BINDING,
            binding_text,
        )?;
    }
    Ok(())
}

fn append_item_name_line(
    lua: &Lua,
    lines: &mlua::Table,
    index: i32,
    line_type: i32,
    item: &crate::items::ItemInfo,
) -> Result<()> {
    let line = lua.create_table()?;
    line.set("type", line_type)?;
    line.set("leftText", item.name)?;
    line.set("leftColor", item_quality_color(lua, item.quality)?)?;
    lines.set(index, line)?;
    Ok(())
}

fn append_item_tooltip_line(
    lua: &Lua,
    lines: &mlua::Table,
    index: i32,
    line_type: i32,
    text: &str,
) -> Result<()> {
    let line = lua.create_table()?;
    line.set("type", line_type)?;
    line.set("leftText", text)?;
    lines.set(index, line)?;
    Ok(())
}

fn item_quality_color(lua: &Lua, quality: u8) -> Result<Value> {
    let (r, g, b) = item_quality_rgb(quality);
    let color = lua.create_table()?;
    color.set("r", r)?;
    color.set("g", g)?;
    color.set("b", b)?;
    color.set("a", 1.0)?;
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
    color.set(
        "GetRGBA",
        lua.create_function(|_, this: mlua::Table| {
            Ok((
                this.get::<f64>("r")?,
                this.get::<f64>("g")?,
                this.get::<f64>("b")?,
                this.get::<f64>("a")?,
            ))
        })?,
    )?;
    Ok(Value::Table(color))
}

fn item_quality_rgb(quality: u8) -> (f64, f64, f64) {
    const QUALITY_COLORS: [(f64, f64, f64); 9] = [
        (0.62, 0.62, 0.62),
        (1.00, 1.00, 1.00),
        (0.12, 1.00, 0.00),
        (0.00, 0.44, 0.87),
        (0.64, 0.21, 0.93),
        (1.00, 0.50, 0.00),
        (0.90, 0.80, 0.50),
        (0.00, 0.80, 1.00),
        (0.00, 0.80, 1.00),
    ];
    QUALITY_COLORS
        .get(quality as usize)
        .copied()
        .unwrap_or(QUALITY_COLORS[1])
}

fn item_binding_text(bonding: u8) -> Option<&'static str> {
    match bonding {
        1 => Some("Binds when picked up"),
        2 => Some("Binds when equipped"),
        3 => Some("Binds when used"),
        4 => Some("Quest Item"),
        8 => Some("Warbound"),
        9 => Some("Warbound until equipped"),
        _ => None,
    }
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
