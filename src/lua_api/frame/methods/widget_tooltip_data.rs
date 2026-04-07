//! Tooltip data population: items, units, auras, and associated color/label tables.

use crate::lua_api::frame::handle::get_sim_state;
use crate::lua_api::tooltip::TooltipLine;
use mlua::Value;

// --- Spell tooltips ---

pub(super) fn populate_spell_tooltip(
    lua: &mlua::Lua,
    tooltip_id: u64,
    spell_id: u32,
) -> mlua::Result<()> {
    let spell = match crate::spells::get_spell(spell_id) {
        Some(s) => s,
        None => return Ok(()),
    };

    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(td) = state.tooltips.get_mut(&tooltip_id) {
        td.lines.clear();
        td.spell_id = Some(spell_id);
        build_spell_lines(spell_id, spell.name, &mut td.lines);
    }
    state.set_frame_visible(tooltip_id, true);
    Ok(())
}

fn build_spell_lines(spell_id: u32, name: &str, lines: &mut Vec<TooltipLine>) {
    lines.push(simple_line(name.to_string()));

    if let Some(cost_text) = crate::spell_power::get_spell_power(spell_id)
        .and_then(|costs| costs.first())
        .and_then(|cost| format_power_cost(cost))
    {
        lines.push(simple_line(cost_text));
    }

    let cast_time_ms = crate::lua_api::globals::spell_api::spell_cast_time(spell_id as i32);
    let cast_text = if cast_time_ms > 0 {
        format_cast_time(cast_time_ms)
    } else {
        "Instant".to_string()
    };
    lines.push(simple_line(cast_text));

    let description = crate::spell_descriptions::get_spell_description(spell_id).unwrap_or("");
    if !description.is_empty() {
        let clean = super::widget_tooltip::strip_html_tags(description);
        lines.push(TooltipLine {
            left_text: clean,
            left_color: (1.0, 0.82, 0.0),
            wrap: true,
            ..simple_line(String::new())
        });
    }
}

/// Create a white, non-wrapping, left-aligned tooltip line.
fn simple_line(text: String) -> TooltipLine {
    TooltipLine {
        left_text: text,
        left_color: (1.0, 1.0, 1.0),
        right_text: None,
        right_color: (1.0, 1.0, 1.0),
        wrap: false,
        texture: None,
    }
}

fn format_power_cost(cost: &crate::spell_power::SpellPowerCost) -> Option<String> {
    let type_name = crate::spell_power::power_type_name(cost.power_type);
    if cost.cost_pct > 0.0 {
        Some(format!("{}% of Base {}", cost.cost_pct, type_name))
    } else if cost.mana_cost > 0 {
        Some(format!("{} {}", cost.mana_cost, type_name))
    } else {
        None
    }
}

fn format_cast_time(ms: i32) -> String {
    let secs = ms as f64 / 1000.0;
    if (secs - secs.round()).abs() < 0.001 {
        format!("{} sec cast", secs as i32)
    } else {
        format!("{:.1} sec cast", secs)
    }
}

// --- Item tooltips ---

pub(super) fn populate_item_tooltip(
    lua: &mlua::Lua,
    tooltip_id: u64,
    item_id: u32,
) -> mlua::Result<()> {
    let item = match crate::items::get_item(item_id) {
        Some(i) => i,
        None => return Ok(()),
    };
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(td) = state.tooltips.get_mut(&tooltip_id) {
        td.lines.clear();
        build_item_lines(item, &mut td.lines);
    }
    state.set_frame_visible(tooltip_id, true);
    Ok(())
}

fn build_item_lines(item: &crate::items::ItemInfo, lines: &mut Vec<TooltipLine>) {
    let (nr, ng, nb) = quality_color_rgb(item.quality);
    lines.push(TooltipLine {
        left_color: (nr, ng, nb),
        ..simple_line(item.name.to_string())
    });
    lines.push(TooltipLine {
        left_color: (1.0, 0.82, 0.0),
        ..simple_line(format!("Item Level {}", item.item_level))
    });
    let slot_label = equip_slot_label(item.inventory_type);
    if item.inventory_type > 0 && !slot_label.is_empty() {
        lines.push(simple_line(slot_label.to_string()));
    }
}

pub(super) fn parse_item_id_from_hyperlink(link: &str) -> Option<u32> {
    let start = link.find("item:")?;
    let after = &link[start + 5..];
    let end = after
        .find(|c: char| c == ':' || c == '|')
        .unwrap_or(after.len());
    after[..end].parse::<u32>().ok()
}

fn quality_color_rgb(quality: u8) -> (f32, f32, f32) {
    match quality {
        0 => (0.62, 0.62, 0.62),
        1 => (1.0, 1.0, 1.0),
        2 => (0.12, 1.0, 0.0),
        3 => (0.0, 0.44, 0.87),
        4 => (0.64, 0.21, 0.93),
        5 => (1.0, 0.5, 0.0),
        6 => (0.9, 0.8, 0.5),
        7 => (0.0, 0.8, 1.0),
        _ => (1.0, 1.0, 1.0),
    }
}

fn equip_slot_label(inv_type: u8) -> &'static str {
    match inv_type {
        1 => "Head",
        2 => "Neck",
        3 => "Shoulder",
        4 => "Shirt",
        5 | 20 => "Chest",
        6 => "Waist",
        7 => "Legs",
        8 => "Feet",
        9 => "Wrist",
        10 => "Hands",
        11 => "Finger",
        12 => "Trinket",
        13 => "One-Hand",
        14 => "Off Hand",
        15 => "Ranged",
        16 => "Back",
        17 => "Two-Hand",
        21 => "Main Hand",
        22 => "Off Hand",
        23 => "Held In Off-hand",
        25 => "Thrown",
        26 => "Ranged",
        _ => "",
    }
}

// --- Unit tooltips ---

/// Public entry point for SetUnit on tooltip frames (called from widget_model.rs).
pub fn set_unit_for_tooltip(
    lua: &mlua::Lua,
    tooltip_id: u64,
    args: mlua::MultiValue,
) -> mlua::Result<Value> {
    let unit = match args.into_iter().next() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        _ => return Ok(Value::Boolean(false)),
    };
    let populated = populate_unit_tooltip(lua, tooltip_id, &unit)?;
    Ok(Value::Boolean(populated))
}

fn populate_unit_tooltip(lua: &mlua::Lua, tooltip_id: u64, unit: &str) -> mlua::Result<bool> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let unit_info = resolve_unit_tooltip_info(&state, unit);
    let Some(info) = unit_info else {
        return Ok(false);
    };
    if let Some(td) = state.tooltips.get_mut(&tooltip_id) {
        td.lines.clear();
        td.lines.push(TooltipLine {
            left_text: info.name,
            left_color: info.class_color,
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap: false,
            texture: None,
        });
        td.lines.push(TooltipLine {
            left_text: format!("Level {} {}", info.level, info.race),
            left_color: (1.0, 1.0, 1.0),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap: false,
            texture: None,
        });
    }
    state.set_frame_visible(tooltip_id, true);
    Ok(true)
}

struct UnitTooltipInfo {
    name: String,
    level: i32,
    race: String,
    class_color: (f32, f32, f32),
}

fn resolve_unit_tooltip_info(
    state: &crate::lua_api::state::SimState,
    unit: &str,
) -> Option<UnitTooltipInfo> {
    match unit {
        "player" => {
            let p = &state.player;
            let class_color = class_color_rgb(p.class_index);
            let race = crate::lua_api::state::RACE_DATA
                .get(p.race_index)
                .map(|(name, _, _)| name.to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            Some(UnitTooltipInfo {
                name: p.name.clone(),
                level: p.level,
                race,
                class_color,
            })
        }
        "target" => {
            let t = state.current_target.as_ref()?;
            let class_color = class_color_rgb(t.class_index);
            Some(UnitTooltipInfo {
                name: t.name.clone(),
                level: t.level,
                race: t.creature_type.clone(),
                class_color,
            })
        }
        _ => None,
    }
}

fn class_color_rgb(class_index: i32) -> (f32, f32, f32) {
    match class_index {
        1 => (0.78, 0.61, 0.43),  // Warrior
        2 => (0.96, 0.55, 0.73),  // Paladin
        3 => (0.67, 0.83, 0.45),  // Hunter
        4 => (1.0, 0.96, 0.41),   // Rogue
        5 => (1.0, 1.0, 1.0),     // Priest
        6 => (0.77, 0.12, 0.23),  // Death Knight
        7 => (0.0, 0.44, 0.87),   // Shaman
        8 => (0.25, 0.78, 0.92),  // Mage
        9 => (0.53, 0.53, 0.93),  // Warlock
        10 => (0.0, 1.0, 0.6),    // Monk
        11 => (1.0, 0.49, 0.04),  // Druid
        12 => (0.64, 0.19, 0.79), // Demon Hunter
        13 => (0.2, 0.58, 0.5),   // Evoker
        _ => (1.0, 1.0, 1.0),
    }
}

// --- Aura tooltips ---

pub(super) fn lookup_aura_from_args(
    lua: &mlua::Lua,
    args: &mlua::MultiValue,
) -> Option<crate::lua_api::game_data::AuraInfo> {
    let mut iter = args.iter();
    let _unit = iter.next(); // unit string (e.g. "player")
    let index_or_id = match iter.next()? {
        Value::Integer(n) => *n as i32,
        Value::Number(n) => *n as i32,
        _ => return None,
    };
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    // Try as 1-based index first
    if index_or_id >= 1 {
        if let Some(aura) = state.player.buffs.get((index_or_id - 1) as usize) {
            return Some(aura.clone());
        }
    }
    // Try as aura instance ID
    state
        .player
        .buffs
        .iter()
        .find(|a| a.aura_instance_id == index_or_id)
        .cloned()
}

pub(super) fn populate_aura_tooltip(
    lua: &mlua::Lua,
    tooltip_id: u64,
    aura: &crate::lua_api::game_data::AuraInfo,
) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(td) = state.tooltips.get_mut(&tooltip_id) {
        td.lines.clear();
        td.lines.push(TooltipLine {
            left_text: aura.name.clone(),
            left_color: (1.0, 1.0, 1.0),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap: false,
            texture: None,
        });
        if aura.duration > 0.0 {
            td.lines.push(TooltipLine {
                left_text: format_aura_duration(aura.duration),
                left_color: (1.0, 1.0, 1.0),
                right_text: None,
                right_color: (1.0, 1.0, 1.0),
                wrap: false,
                texture: None,
            });
        }
    }
    state.set_frame_visible(tooltip_id, true);
    Ok(())
}

fn format_aura_duration(seconds: f64) -> String {
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
