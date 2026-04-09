use mlua::{Lua, Result, Value};

use crate::lua_api::globals::{c_item_api, spell_api};

pub(super) fn build_empty_tooltip(lua: &Lua, tooltip_type: i32) -> Result<Value> {
    let tooltip = lua.create_table()?;
    tooltip.set("type", tooltip_type)?;
    tooltip.set("lines", lua.create_table()?)?;
    Ok(Value::Table(tooltip))
}

pub(super) fn build_tooltip_with_lines<F>(
    lua: &Lua,
    tooltip_type: i32,
    build_lines: F,
) -> Result<Value>
where
    F: FnOnce(&mlua::Table) -> Result<()>,
{
    let tooltip = lua.create_table()?;
    tooltip.set("type", tooltip_type)?;

    let lines = lua.create_table()?;
    build_lines(&lines)?;

    tooltip.set("lines", lines)?;
    Ok(Value::Table(tooltip))
}

pub(super) fn build_spell_tooltip_lines(
    lua: &Lua,
    spell_id: i32,
    spell_name: &str,
    lines: &mlua::Table,
) -> Result<()> {
    append_spell_tooltip_lines(lua, spell_id, spell_name, lines)?;
    Ok(())
}

pub(super) fn append_aura_tooltip_lines(
    lua: &Lua,
    aura: crate::lua_api::game_data::AuraInfo,
    lines: &mlua::Table,
) -> Result<()> {
    let mut body_lines = Vec::new();
    if aura.duration > 0.0 {
        body_lines.push(format_aura_duration_text(aura.duration));
    }

    append_named_description_tooltip_lines(
        lua,
        lines,
        &aura.name,
        &body_lines,
        tooltip_description_text(aura.spell_id),
    )
}

pub(super) fn append_minimap_mouseover_tooltip_lines(
    lua: &Lua,
    lines: &mlua::Table,
    zone_name: &str,
    sub_zone_name: &str,
) -> Result<()> {
    const TOOLTIP_LINE_TYPE_NONE: i32 = 0;
    const WHITE_TOOLTIP_TEXT: (f32, f32, f32) = (1.0, 1.0, 1.0);

    if zone_name.is_empty() {
        return Ok(());
    }

    append_colored_tooltip_line(
        lua,
        lines,
        1,
        TOOLTIP_LINE_TYPE_NONE,
        zone_name,
        WHITE_TOOLTIP_TEXT,
    )?;
    if !sub_zone_name.is_empty() && sub_zone_name != zone_name {
        append_tooltip_line(lua, lines, 2, TOOLTIP_LINE_TYPE_NONE, sub_zone_name)?;
    }
    Ok(())
}

pub(super) fn append_unit_tooltip_lines(
    lua: &Lua,
    lines: &mlua::Table,
    info: &crate::lua_api::frame::UnitTooltipInfo,
) -> Result<()> {
    const TOOLTIP_LINE_TYPE_NONE: i32 = 0;
    const TOOLTIP_LINE_TYPE_UNIT_NAME: i32 = 2;

    append_colored_tooltip_line(
        lua,
        lines,
        1,
        TOOLTIP_LINE_TYPE_UNIT_NAME,
        &info.name,
        info.class_color,
    )?;
    append_tooltip_line(
        lua,
        lines,
        2,
        TOOLTIP_LINE_TYPE_NONE,
        &format!("Level {}", info.level),
    )?;
    append_tooltip_line(lua, lines, 3, TOOLTIP_LINE_TYPE_NONE, &info.race)?;
    append_tooltip_line(lua, lines, 4, TOOLTIP_LINE_TYPE_NONE, &info.class_name)?;
    Ok(())
}

pub(super) fn build_empty_item_tooltip(lua: &Lua, tooltip_type: i32) -> Result<Value> {
    build_empty_tooltip(lua, tooltip_type)
}

pub(super) fn build_item_tooltip_lines(
    lua: &Lua,
    item: &crate::items::ItemInfo,
    lines: &mlua::Table,
) -> Result<()> {
    const TOOLTIP_LINE_TYPE_ITEM_BINDING: i32 = 20;
    const TOOLTIP_LINE_TYPE_EQUIP_SLOT: i32 = 21;
    const TOOLTIP_LINE_TYPE_ITEM_NAME: i32 = 22;
    const TOOLTIP_LINE_TYPE_ITEM_LEVEL: i32 = 31;

    append_item_name_line(lua, lines, 1, TOOLTIP_LINE_TYPE_ITEM_NAME, item)?;
    append_item_level_line(lua, lines, 2, TOOLTIP_LINE_TYPE_ITEM_LEVEL, item.item_level)?;

    let equip_slot = c_item_api::item_equip_slot_label(item.inventory_type);
    let next_index = append_optional_item_tooltip_line(
        lua,
        lines,
        3,
        TOOLTIP_LINE_TYPE_EQUIP_SLOT,
        non_empty_tooltip_text(equip_slot),
    )?;
    append_optional_item_tooltip_line(
        lua,
        lines,
        next_index,
        TOOLTIP_LINE_TYPE_ITEM_BINDING,
        item_binding_text(item.bonding),
    )?;
    Ok(())
}

pub(super) fn lookup_player_aura_for_tooltip(
    lua: &Lua,
    unit: &str,
    index: i32,
    filter: Option<&str>,
) -> Option<crate::lua_api::game_data::AuraInfo> {
    if should_skip_player_aura_tooltip(unit, index, filter) {
        return None;
    }

    let state_rc = crate::lua_api::frame::get_sim_state(lua);
    let state = state_rc.borrow();
    state.player.buffs.get((index - 1) as usize).cloned()
}

pub(super) fn lookup_player_buff_by_aura_instance_id_for_tooltip(
    lua: &Lua,
    unit: &str,
    aura_instance_id: i32,
    filter: Option<&str>,
) -> Option<crate::lua_api::game_data::AuraInfo> {
    if should_skip_player_aura_tooltip(unit, aura_instance_id, filter) {
        return None;
    }

    lookup_player_aura_by_instance_id_for_tooltip(lua, unit, aura_instance_id)
}

pub(super) fn lookup_player_aura_by_instance_id_for_tooltip(
    lua: &Lua,
    unit: &str,
    aura_instance_id: i32,
) -> Option<crate::lua_api::game_data::AuraInfo> {
    if unit != "player" || aura_instance_id < 1 {
        return None;
    }

    let state_rc = crate::lua_api::frame::get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .player
        .buffs
        .iter()
        .find(|aura| aura.aura_instance_id == aura_instance_id)
        .cloned()
}

pub(super) fn lua_value_to_i32(value: Value) -> Option<i32> {
    match value {
        Value::Integer(n) => Some(n as i32),
        Value::Number(n) => Some(n as i32),
        _ => None,
    }
}

pub(super) fn lua_value_to_string(value: Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.to_string_lossy().to_string()),
        _ => None,
    }
}

fn append_colored_tooltip_line(
    lua: &Lua,
    lines: &mlua::Table,
    index: i32,
    line_type: i32,
    text: &str,
    color: (f32, f32, f32),
) -> Result<()> {
    let line = lua.create_table()?;
    line.set("type", line_type)?;
    line.set("leftText", text)?;
    let (r, g, b) = color;
    line.set(
        "leftColor",
        tooltip_color(lua, (r as f64, g as f64, b as f64))?,
    )?;
    lines.set(index, line)?;
    Ok(())
}

fn append_item_level_line(
    lua: &Lua,
    lines: &mlua::Table,
    index: i32,
    line_type: i32,
    item_level: u16,
) -> Result<()> {
    let item_level_text = format!("Item Level {item_level}");
    append_item_tooltip_line(lua, lines, index, line_type, &item_level_text)
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

fn append_named_description_tooltip_lines(
    lua: &Lua,
    lines: &mlua::Table,
    name_text: &str,
    body_lines: &[String],
    description: Option<String>,
) -> Result<()> {
    const TOOLTIP_LINE_TYPE_NONE: i32 = 0;
    const TOOLTIP_LINE_TYPE_SPELL_NAME: i32 = 13;
    const TOOLTIP_LINE_TYPE_SPELL_DESCRIPTION: i32 = 34;

    append_tooltip_line(lua, lines, 1, TOOLTIP_LINE_TYPE_SPELL_NAME, name_text)?;

    let mut next_index = 2;
    for body_line in body_lines {
        append_tooltip_line(lua, lines, next_index, TOOLTIP_LINE_TYPE_NONE, body_line)?;
        next_index += 1;
    }

    if let Some(description_text) = description {
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

fn append_optional_item_tooltip_line(
    lua: &Lua,
    lines: &mlua::Table,
    index: i32,
    line_type: i32,
    text: Option<&str>,
) -> Result<i32> {
    let Some(text) = text else {
        return Ok(index);
    };
    append_item_tooltip_line(lua, lines, index, line_type, text)?;
    Ok(index + 1)
}

fn append_spell_tooltip_lines(
    lua: &Lua,
    spell_id: i32,
    spell_name: &str,
    lines: &mlua::Table,
) -> Result<()> {
    let mut body_lines = Vec::new();
    if let Some(power_text) = spell_power_text(spell_id) {
        body_lines.push(power_text);
    }

    body_lines.push(spell_cast_time_text(spell_id));

    append_named_description_tooltip_lines(
        lua,
        lines,
        spell_name,
        &body_lines,
        tooltip_description_text(spell_id),
    )
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

fn item_quality_color(lua: &Lua, quality: u8) -> Result<Value> {
    tooltip_color(lua, item_quality_rgb(quality))
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

fn non_empty_tooltip_text(text: &str) -> Option<&str> {
    if text.is_empty() { None } else { Some(text) }
}

fn register_tooltip_color_methods(lua: &Lua, color: &mlua::Table) -> Result<()> {
    color.set("GetRGB", lua.create_function(tooltip_color_get_rgb)?)?;
    color.set("GetRGBA", lua.create_function(tooltip_color_get_rgba)?)?;
    Ok(())
}

fn set_tooltip_color_channels(color: &mlua::Table, r: f64, g: f64, b: f64) -> Result<()> {
    color.set("r", r)?;
    color.set("g", g)?;
    color.set("b", b)?;
    color.set("a", 1.0)?;
    Ok(())
}

fn should_skip_player_aura_tooltip(unit: &str, index: i32, filter: Option<&str>) -> bool {
    if unit != "player" || index < 1 {
        return true;
    }
    filter.is_some_and(|f| f.contains("HARMFUL") || f.contains("MAW"))
}

fn spell_cast_time_text(spell_id: i32) -> String {
    let cast_time_ms = spell_api::spell_cast_time(spell_id);
    if cast_time_ms > 0 {
        let secs = cast_time_ms as f64 / 1000.0;
        if (secs - secs.round()).abs() < 0.001 {
            format!("{} sec cast", secs as i32)
        } else {
            format!("{secs:.1} sec cast")
        }
    } else {
        "Instant".to_string()
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

fn tooltip_color(lua: &Lua, (r, g, b): (f64, f64, f64)) -> Result<Value> {
    let color = lua.create_table()?;
    set_tooltip_color_channels(&color, r, g, b)?;
    register_tooltip_color_methods(lua, &color)?;
    Ok(Value::Table(color))
}

fn tooltip_color_get_rgb(_: &Lua, color: mlua::Table) -> Result<(f64, f64, f64)> {
    Ok((
        color.get::<f64>("r")?,
        color.get::<f64>("g")?,
        color.get::<f64>("b")?,
    ))
}

fn tooltip_color_get_rgba(_: &Lua, color: mlua::Table) -> Result<(f64, f64, f64, f64)> {
    Ok((
        color.get::<f64>("r")?,
        color.get::<f64>("g")?,
        color.get::<f64>("b")?,
        color.get::<f64>("a")?,
    ))
}

fn tooltip_description_text(spell_id: i32) -> Option<String> {
    let description = crate::spell_descriptions::get_spell_description(spell_id as u32)?;
    if description.is_empty() {
        None
    } else {
        Some(crate::lua_api::tooltip::strip_html_tags(description))
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
