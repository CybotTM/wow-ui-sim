use mlua::{Lua, Result, Value};

#[path = "c_misc_api_core_tooltip_lines.rs"]
mod lines;

use crate::lua_api::tooltip::{parse_item_id_from_hyperlink, parse_spell_id_from_hyperlink};
use lines::{
    append_aura_tooltip_lines, append_minimap_mouseover_tooltip_lines, append_unit_tooltip_lines,
    build_empty_item_tooltip, build_empty_tooltip, build_item_tooltip_lines,
    build_spell_tooltip_lines, build_tooltip_with_lines,
    lookup_player_aura_by_instance_id_for_tooltip, lookup_player_aura_for_tooltip,
    lookup_player_buff_by_aura_instance_id_for_tooltip, lua_value_to_i32, lua_value_to_string,
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
        "GetUpgradeItem",
        lua.create_function(create_upgrade_item_tooltip)?,
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
    table.set(
        "GetMinimapMouseover",
        lua.create_function(create_minimap_mouseover_tooltip)?,
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

fn create_upgrade_item_tooltip(lua: &Lua, _: ()) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_ITEM: i32 = 0;

    let Some(item_id) = super::c_misc_api_game::selected_item_upgrade_item_id(lua) else {
        return build_empty_item_tooltip(lua, TOOLTIP_DATA_TYPE_ITEM);
    };
    create_item_tooltip(lua, item_id as i32)
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

fn create_minimap_mouseover_tooltip(lua: &Lua, _: ()) -> Result<Value> {
    const TOOLTIP_DATA_TYPE_MINIMAP_MOUSEOVER: i32 = 21;

    let state_rc = crate::lua_api::frame::get_sim_state(lua);
    let state = state_rc.borrow();
    let zone_name = state.world.zone_name.clone();
    let sub_zone_name = state.world.sub_zone_name.clone();
    drop(state);

    build_tooltip_with_lines(lua, TOOLTIP_DATA_TYPE_MINIMAP_MOUSEOVER, |lines| {
        append_minimap_mouseover_tooltip_lines(lua, lines, &zone_name, &sub_zone_name)
    })
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
