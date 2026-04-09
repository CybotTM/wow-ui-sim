use mlua::{Lua, Result, Value};

use crate::lua_api::tooltip::{
    parse_item_id_from_hyperlink, parse_spell_id_from_hyperlink, strip_html_tags,
};

const WORLD_LOOT_TOOLTIP_SPELL_ID: i32 = 19750;
const WORLD_LOOT_TOOLTIP_INVENTORY_TYPE: i32 = 13;
const WORLD_CURSOR_GUID: &str = "WorldLootObject-0000-0000C0DE";

pub(super) fn register_all(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let tooltip_info: mlua::Table = globals
        .get::<mlua::Table>("C_TooltipInfo")
        .unwrap_or_else(|_| lua.create_table().unwrap());
    register_item_and_spell_tooltip_overrides(lua, &tooltip_info)?;
    register_unit_tooltip_overrides(lua, &tooltip_info)?;
    globals.set("C_TooltipInfo", tooltip_info)?;
    Ok(())
}

fn register_item_and_spell_tooltip_overrides(lua: &Lua, table: &mlua::Table) -> Result<()> {
    register_item_tooltip_overrides(lua, table)?;
    register_spell_tooltip_overrides(lua, table)?;
    register_world_tooltip_overrides(lua, table)?;
    Ok(())
}

fn register_item_tooltip_overrides(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetTraitEntry",
        lua.create_function(create_trait_entry_tooltip)?,
    )?;
    table.set("GetAction", lua.create_function(create_action_tooltip)?)?;
    table.set("GetItemByID", lua.create_function(create_item_tooltip)?)?;
    table.set(
        "GetOwnedItemByID",
        lua.create_function(create_owned_item_tooltip)?,
    )?;
    table.set(
        "GetItemByGUID",
        lua.create_function(create_item_by_guid_tooltip)?,
    )?;
    table.set(
        "GetInventoryItem",
        lua.create_function(create_inventory_item_tooltip)?,
    )?;
    Ok(())
}

fn register_spell_tooltip_overrides(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetSpellBookItem",
        lua.create_function(create_spell_book_item_tooltip)?,
    )?;
    table.set("GetSpellByID", lua.create_function(create_spell_tooltip)?)?;
    table.set(
        "GetHyperlink",
        lua.create_function(create_hyperlink_tooltip)?,
    )?;
    table.set(
        "GetRecipeResultItem",
        lua.create_function(create_recipe_result_item_tooltip)?,
    )?;
    table.set(
        "GetRecipeResultItemForOrder",
        lua.create_function(create_recipe_result_item_for_order_tooltip)?,
    )?;
    Ok(())
}

fn register_world_tooltip_overrides(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetWorldCursor",
        lua.create_function(create_world_cursor_tooltip)?,
    )?;
    table.set(
        "GetWorldLootObject",
        lua.create_function(create_world_loot_object_tooltip)?,
    )?;
    Ok(())
}

fn register_unit_tooltip_overrides(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set("GetUnit", lua.create_function(create_unit_tooltip)?)?;
    table.set(
        "GetUnitBuff",
        lua.create_function(create_unit_buff_tooltip)?,
    )?;
    table.set(
        "GetUnitBuffByAuraInstanceID",
        lua.create_function(create_unit_buff_by_aura_instance_id_tooltip)?,
    )?;
    table.set(
        "GetUnitDebuff",
        lua.create_function(create_unit_debuff_tooltip)?,
    )?;
    table.set(
        "GetUnitDebuffByAuraInstanceID",
        lua.create_function(create_unit_debuff_by_aura_instance_id_tooltip)?,
    )?;
    table.set(
        "GetUnitAura",
        lua.create_function(create_unit_aura_tooltip)?,
    )?;
    table.set(
        "GetUnitAuraByAuraInstanceID",
        lua.create_function(create_unit_aura_by_aura_instance_id_tooltip)?,
    )?;
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

fn create_action_tooltip(lua: &Lua, action_id: i32) -> Result<Value> {
    let get_action_info: mlua::Function = lua.globals().get("GetActionInfo")?;
    let action_info: mlua::MultiValue = get_action_info.call(action_id)?;
    let mut action_info = action_info.into_iter();

    let action_type = action_info
        .next()
        .and_then(lua_value_to_string)
        .unwrap_or_default();
    let action_value = action_info.next().and_then(lua_value_to_i32);

    match (action_type.as_str(), action_value) {
        ("spell", Some(spell_id)) => create_spell_tooltip(lua, spell_id),
        ("item", Some(item_id)) => create_item_tooltip(lua, item_id),
        _ => Ok(Value::Nil),
    }
}

fn create_item_tooltip(lua: &Lua, item_id: i32) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_ITEM: i32 = 0;

    let Some(item) = crate::items::get_item(item_id as u32) else {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    };
    build_tooltip_with_lines(lua, TOOLTIP_DATA_TYPE_ITEM, |lines| {
        build_item_tooltip_lines(lua, item, lines)
    })
}

fn create_item_by_guid_tooltip(lua: &Lua, guid: String) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_ITEM: i32 = 0;

    let Some(item_id) = bag_item_id_from_guid(lua, &guid) else {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    };
    add_guid_to_item_tooltip(lua, item_id, guid)
}

fn create_owned_item_tooltip(lua: &Lua, item_id: i32) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_ITEM: i32 = 0;

    let Some((owned_item_id, guid)) = first_owned_bag_item(lua, item_id as u32) else {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    };
    add_guid_to_item_tooltip(lua, owned_item_id, guid)
}

fn create_inventory_item_tooltip(lua: &Lua, (_unit, slot): (String, i32)) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_ITEM: i32 = 0;

    let Some(item_id) = super::c_item_api_globals::get_equipped_item_id(lua, slot) else {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    };
    create_item_tooltip(lua, item_id as i32)
}

fn create_spell_book_item_tooltip(
    lua: &Lua,
    (slot, _spell_bank): (i32, Option<i32>),
) -> Result<Value> {
    let Some((_, entry, _)) = super::spellbook_data::get_spell_at_slot(slot) else {
        return build_empty_tooltip(lua, 1);
    };
    create_spell_tooltip(lua, entry.spell_id as i32)
}

fn create_spell_tooltip(lua: &Lua, spell_id: i32) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_SPELL: i32 = 1;

    let Some(spell) = crate::spells::get_spell(spell_id as u32) else {
        return build_empty_tooltip(lua, TOOLTIP_DATA_TYPE_SPELL);
    };
    build_tooltip_with_lines(lua, TOOLTIP_DATA_TYPE_SPELL, |lines| {
        build_spell_tooltip_lines(lua, spell_id, spell.name, lines)
    })
}

fn create_hyperlink_tooltip(lua: &Lua, link: String) -> Result<Value> {
    if let Some(item_id) = parse_item_id_from_hyperlink(&link) {
        return create_item_tooltip(lua, item_id as i32);
    }
    if let Some(spell_id) = parse_spell_id_from_hyperlink(&link) {
        return create_spell_tooltip(lua, spell_id as i32);
    }

    build_empty_tooltip(lua, 0)
}

fn create_recipe_result_item_tooltip(
    lua: &Lua,
    (recipe_id, _reagent_infos, _recraft_item_guid, _recipe_level, _override_quality_id): (
        i32,
        Option<Value>,
        Option<String>,
        Option<i32>,
        Option<i32>,
    ),
) -> Result<Value> {
    create_recipe_output_item_tooltip(lua, recipe_id)
}

fn create_recipe_result_item_for_order_tooltip(
    lua: &Lua,
    (recipe_id, _reagent_infos, _order_id, _recipe_level, _override_quality_id): (
        i32,
        Option<Value>,
        Option<Value>,
        Option<i32>,
        Option<i32>,
    ),
) -> Result<Value> {
    create_recipe_output_item_tooltip(lua, recipe_id)
}

fn create_world_cursor_tooltip(lua: &Lua, _: ()) -> Result<Value> {
    create_world_loot_spell_tooltip(lua, WORLD_CURSOR_GUID)
}

fn create_world_loot_object_tooltip(lua: &Lua, unit: String) -> Result<Value> {
    if unit != "player" {
        return Ok(Value::Nil);
    }

    let world_loot_guid = format!("WorldLootObject-{unit}");
    create_world_loot_spell_tooltip(lua, &world_loot_guid)
}

fn create_world_loot_spell_tooltip(lua: &Lua, world_loot_guid: &str) -> Result<Value> {
    let tooltip = create_spell_tooltip(lua, WORLD_LOOT_TOOLTIP_SPELL_ID)?;
    let Value::Table(tooltip) = tooltip else {
        return build_empty_tooltip(lua, 1);
    };
    tooltip.set(
        "worldLootObjectInventoryType",
        WORLD_LOOT_TOOLTIP_INVENTORY_TYPE,
    )?;
    tooltip.set("id", WORLD_LOOT_TOOLTIP_SPELL_ID)?;
    tooltip.set("worldLootObjectGUID", world_loot_guid)?;
    Ok(Value::Table(tooltip))
}

fn bag_item_id_from_guid(lua: &Lua, guid: &str) -> Option<u32> {
    let (bag, slot, item_id) = super::c_item_location_api::parse_item_guid(guid)?;
    let state_rc = crate::lua_api::frame::get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .get_bag_item(bag, slot)
        .filter(|(state_item_id, _)| *state_item_id == item_id)
        .map(|(state_item_id, _)| state_item_id)
}

fn first_owned_bag_item(lua: &Lua, item_id: u32) -> Option<(u32, String)> {
    let state_rc = crate::lua_api::frame::get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .bag_items
        .iter()
        .find(|(_, bag_item)| bag_item.item_id == item_id)
        .map(|((bag, slot), bag_item)| {
            let guid =
                super::c_item_location_api::item_guid_for_bag_slot(*bag, *slot, bag_item.item_id);
            (bag_item.item_id, guid)
        })
}

fn add_guid_to_item_tooltip(lua: &Lua, item_id: u32, guid: String) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_ITEM: i32 = 0;

    let Value::Table(tooltip) = create_item_tooltip(lua, item_id as i32)? else {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    };
    tooltip.set("guid", guid)?;
    Ok(Value::Table(tooltip))
}

fn create_recipe_output_item_tooltip(lua: &Lua, recipe_id: i32) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_ITEM: i32 = 0;

    let Some(recipe) = super::profession_data::get_recipe(recipe_id) else {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    };
    if recipe.output_item_id == 0 {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    }
    create_item_tooltip(lua, recipe.output_item_id as i32)
}

fn create_unit_tooltip(lua: &Lua, (unit, _hide_status): (String, Option<bool>)) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_UNIT: i32 = 2;

    let state_rc = crate::lua_api::frame::get_sim_state(lua);
    let state = state_rc.borrow();
    let Some(info) = crate::lua_api::frame::resolve_unit_tooltip_info(&state, &unit) else {
        return build_empty_tooltip(lua, TOOLTIP_DATA_TYPE_UNIT);
    };

    build_tooltip_with_lines(lua, TOOLTIP_DATA_TYPE_UNIT, |lines| {
        append_unit_tooltip_lines(lua, lines, &info)
    })
}

fn create_unit_buff_tooltip(
    lua: &Lua,
    (unit, index, filter): (String, i32, Option<String>),
) -> Result<Value> {
    let aura = lookup_player_aura_for_tooltip(lua, &unit, index, filter.as_deref());
    build_unit_aura_tooltip(lua, aura)
}

fn create_unit_debuff_tooltip(
    lua: &Lua,
    (_unit, _index, _filter): (String, i32, Option<String>),
) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_UNIT_AURA: i32 = 7;

    build_empty_tooltip(lua, TOOLTIP_DATA_TYPE_UNIT_AURA)
}

fn create_unit_buff_by_aura_instance_id_tooltip(
    lua: &Lua,
    (unit, aura_instance_id, filter): (String, i32, Option<String>),
) -> Result<Value> {
    let aura = lookup_player_buff_by_aura_instance_id_for_tooltip(
        lua,
        &unit,
        aura_instance_id,
        filter.as_deref(),
    );
    build_unit_aura_tooltip(lua, aura)
}

fn create_unit_debuff_by_aura_instance_id_tooltip(
    lua: &Lua,
    (_unit, _aura_instance_id, _filter): (String, i32, Option<String>),
) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_UNIT_AURA: i32 = 7;

    build_empty_tooltip(lua, TOOLTIP_DATA_TYPE_UNIT_AURA)
}

fn create_unit_aura_tooltip(
    lua: &Lua,
    (unit, index, filter): (String, i32, Option<String>),
) -> Result<Value> {
    if filter.as_deref().is_some_and(|f| f.contains("HARMFUL")) {
        return create_unit_debuff_tooltip(lua, (unit, index, filter));
    }

    create_unit_buff_tooltip(lua, (unit, index, filter))
}

fn create_unit_aura_by_aura_instance_id_tooltip(
    lua: &Lua,
    (unit, aura_instance_id, _filter): (String, i32, Option<String>),
) -> Result<Value> {
    let aura = lookup_player_aura_by_instance_id_for_tooltip(lua, &unit, aura_instance_id);
    build_unit_aura_tooltip(lua, aura)
}

fn build_unit_aura_tooltip(
    lua: &Lua,
    aura: Option<crate::lua_api::game_data::AuraInfo>,
) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_UNIT_AURA: i32 = 7;

    let Some(aura) = aura else {
        return build_empty_tooltip(lua, TOOLTIP_DATA_TYPE_UNIT_AURA);
    };
    build_tooltip_with_lines(lua, TOOLTIP_DATA_TYPE_UNIT_AURA, |lines| {
        append_aura_tooltip_lines(lua, aura, lines)
    })
}

fn build_empty_tooltip(lua: &Lua, tooltip_type: i32) -> Result<Value> {
    let tooltip = lua.create_table()?;
    tooltip.set("type", tooltip_type)?;
    tooltip.set("lines", lua.create_table()?)?;
    Ok(Value::Table(tooltip))
}

fn build_tooltip_with_lines<F>(lua: &Lua, tooltip_type: i32, build_lines: F) -> Result<Value>
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

fn build_spell_tooltip_lines(
    lua: &Lua,
    spell_id: i32,
    spell_name: &str,
    lines: &mlua::Table,
) -> Result<()> {
    append_spell_tooltip_lines(lua, spell_id, spell_name, lines)?;
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

fn lookup_player_aura_for_tooltip(
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

fn lookup_player_buff_by_aura_instance_id_for_tooltip(
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

fn lookup_player_aura_by_instance_id_for_tooltip(
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

fn append_aura_tooltip_lines(
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

fn append_unit_tooltip_lines(
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

fn should_skip_player_aura_tooltip(unit: &str, index: i32, filter: Option<&str>) -> bool {
    if unit != "player" || index < 1 {
        return true;
    }
    filter.is_some_and(|f| f.contains("HARMFUL") || f.contains("MAW"))
}

fn lua_value_to_string(value: Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.to_string_lossy().to_string()),
        _ => None,
    }
}

fn lua_value_to_i32(value: Value) -> Option<i32> {
    match value {
        Value::Integer(n) => Some(n as i32),
        Value::Number(n) => Some(n as i32),
        _ => None,
    }
}

fn tooltip_description_text(spell_id: i32) -> Option<String> {
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
            format!("{secs:.1} sec cast")
        }
    } else {
        "Instant".to_string()
    }
}

fn build_empty_item_tooltip(lua: &Lua, tooltip_type: i32) -> Result<Value> {
    build_empty_tooltip(lua, tooltip_type)
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
    append_item_level_line(lua, lines, 2, TOOLTIP_LINE_TYPE_ITEM_LEVEL, item.item_level)?;

    let equip_slot = super::c_item_api::item_equip_slot_label(item.inventory_type);
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

fn non_empty_tooltip_text(text: &str) -> Option<&str> {
    if text.is_empty() { None } else { Some(text) }
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

fn tooltip_color(lua: &Lua, (r, g, b): (f64, f64, f64)) -> Result<Value> {
    let color = lua.create_table()?;
    set_tooltip_color_channels(&color, r, g, b)?;
    register_tooltip_color_methods(lua, &color)?;
    Ok(Value::Table(color))
}

fn set_tooltip_color_channels(color: &mlua::Table, r: f64, g: f64, b: f64) -> Result<()> {
    color.set("r", r)?;
    color.set("g", g)?;
    color.set("b", b)?;
    color.set("a", 1.0)?;
    Ok(())
}

fn register_tooltip_color_methods(lua: &Lua, color: &mlua::Table) -> Result<()> {
    color.set("GetRGB", lua.create_function(tooltip_color_get_rgb)?)?;
    color.set("GetRGBA", lua.create_function(tooltip_color_get_rgba)?)?;
    Ok(())
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
