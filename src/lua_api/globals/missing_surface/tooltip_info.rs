use super::item_socket_info;
use super::{
    LINE_TYPE_EQUIP_SLOT, LINE_TYPE_ITEM_BINDING, LINE_TYPE_ITEM_LEVEL, LINE_TYPE_ITEM_NAME,
    LINE_TYPE_SPELL_DESCRIPTION, LINE_TYPE_SPELL_NAME, LINE_TYPE_UNIT_NAME, TOOLTIP_TYPE_CURRENCY,
    TOOLTIP_TYPE_ITEM, TOOLTIP_TYPE_MINIMAP_MOUSEOVER, TOOLTIP_TYPE_SPELL, TOOLTIP_TYPE_UNIT,
    TOOLTIP_TYPE_UNIT_AURA, WORLD_CURSOR_GUID, WORLD_LOOT_TOOLTIP_INVENTORY_TYPE,
    WORLD_LOOT_TOOLTIP_SPELL_ID, ensure_namespace, set_table_array,
};
use crate::items;
use crate::lua_api::game_data::CLASS_LABELS;
use crate::lua_api::globals::{currency_data, profession_data};
use crate::lua_api::globals::{spell_api, spellbook_data};
use crate::lua_api::methods::{
    borrow_state, call_function_state, create_string, create_table, table_get, table_set,
    val_to_string,
};
use crate::lua_api::state::RACE_DATA;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use crate::spell_descriptions;
use crate::spells;
use crate::traits::{TRAIT_DEFINITION_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

use super::item_spell::{current_item_upgrade_location, parse_item_guid, parse_prefixed_id};

pub(super) fn register_tooltip_surface(state: &mut LuaState) -> LuaResult<()> {
    ensure_pet_info_state(state);
    register_c_tooltip_info(state)
}

fn color_table(state: &mut LuaState, r: f64, g: f64, b: f64, a: f64) -> Val {
    let key = state.gc.intern_string(b"CreateColor");
    let create_color = state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(key, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    match call_function_state(
        state,
        create_color,
        &[Val::Num(r), Val::Num(g), Val::Num(b), Val::Num(a)],
    ) {
        Ok(color) => color,
        Err(_) => create_table(state),
    }
}

fn push_tooltip_line(
    state: &mut LuaState,
    lines: Val,
    index: i64,
    line_type: f64,
    left_text: &str,
    left_color: Option<(f64, f64, f64)>,
    wrap: bool,
) {
    let line = create_table(state);
    table_set(state, line, "type", Val::Num(line_type));
    let left_text_val = create_string(state, left_text);
    table_set(state, line, "leftText", left_text_val);
    if let Some((r, g, b)) = left_color {
        let color = color_table(state, r, g, b, 1.0);
        table_set(state, line, "leftColor", color);
    }
    if wrap {
        table_set(state, line, "wrapText", Val::Bool(true));
    }
    set_table_array(state, lines, index, line);
}

fn empty_tooltip(state: &mut LuaState, tooltip_type: f64) -> Val {
    let tooltip = create_table(state);
    let lines = create_table(state);
    table_set(state, tooltip, "type", Val::Num(tooltip_type));
    table_set(state, tooltip, "lines", lines);
    tooltip
}

fn item_quality_color(quality: u8) -> (f64, f64, f64) {
    match quality {
        0 => (0.62, 0.62, 0.62),
        1 => (1.0, 1.0, 1.0),
        2 => (0.12, 1.0, 0.0),
        3 => (0.0, 0.44, 0.87),
        4 => (0.64, 0.21, 0.93),
        5 => (1.0, 0.5, 0.0),
        _ => (1.0, 1.0, 1.0),
    }
}

fn item_equip_slot_label(inv_type: u8) -> &'static str {
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
        14 => "Shield",
        15 => "Ranged",
        16 => "Back",
        17 => "Two-Hand",
        21 => "Main Hand",
        22 => "Off Hand",
        23 => "Held In Off-hand",
        _ => "",
    }
}

fn push_item_name_line(state: &mut LuaState, lines: Val, item: &items::ItemInfo) {
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_ITEM_NAME,
        item.name,
        Some(item_quality_color(item.quality)),
        false,
    );
}

fn push_item_level_line(state: &mut LuaState, lines: Val, item: &items::ItemInfo) {
    let item_level = format!("Item Level {}", item.item_level);
    push_tooltip_line(
        state,
        lines,
        2,
        LINE_TYPE_ITEM_LEVEL,
        &item_level,
        None,
        false,
    );
}

fn item_binding_text(state: &mut LuaState, bonding: u8) -> Option<String> {
    if bonding != 1 {
        return None;
    }
    let item_bind_key = state.gc.intern_string(b"ITEM_BIND_ON_PICKUP");
    state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(item_bind_key, &state.gc.string_arena))
        .and_then(|value| val_to_string(state, value))
        .or_else(|| Some("Binds when picked up".to_string()))
}

fn push_item_equip_slot_line(
    state: &mut LuaState,
    lines: Val,
    inventory_type: u8,
    next_index: &mut i64,
) {
    let equip_slot = item_equip_slot_label(inventory_type);
    if equip_slot.is_empty() {
        return;
    }
    push_tooltip_line(
        state,
        lines,
        *next_index,
        LINE_TYPE_EQUIP_SLOT,
        equip_slot,
        None,
        false,
    );
    *next_index += 1;
}

fn push_item_binding_line(
    state: &mut LuaState,
    lines: Val,
    next_index: i64,
    item: &items::ItemInfo,
) {
    let Some(binding) = item_binding_text(state, item.bonding) else {
        return;
    };
    push_tooltip_line(
        state,
        lines,
        next_index,
        LINE_TYPE_ITEM_BINDING,
        &binding,
        None,
        false,
    );
}

fn tooltip_for_item_id(state: &mut LuaState, item_id: u32) -> Val {
    let Some(item) = items::get_item(item_id) else {
        return empty_tooltip(state, TOOLTIP_TYPE_ITEM);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_ITEM);
    populate_item_tooltip_lines(state, tooltip, item);
    tooltip
}

fn populate_item_tooltip_lines(state: &mut LuaState, tooltip: Val, item: &items::ItemInfo) {
    let lines = table_get(state, tooltip, "lines");
    push_item_name_line(state, lines, item);
    push_item_level_line(state, lines, item);

    let mut next_index = 3;
    push_item_equip_slot_line(state, lines, item.inventory_type, &mut next_index);
    push_item_binding_line(state, lines, next_index, item);
}

fn spell_cost_line(spell_id: u32) -> Option<&'static str> {
    match spell_id {
        19750 => Some("10% of Base MANA"),
        _ => None,
    }
}

fn spell_cast_line(spell_id: u32) -> String {
    let cast_ms = spell_api::spell_cast_time(spell_id as i32);
    if cast_ms <= 0 {
        "Instant".to_string()
    } else {
        format!("{:.1} sec cast", cast_ms as f64 / 1000.0)
    }
}

fn push_spell_tooltip_lines(state: &mut LuaState, lines: Val, spell_id: u32, spell_name: &str) {
    let mut index = 1;
    push_spell_name_line(state, lines, index, spell_name);
    index += 1;
    if push_spell_cost_line(state, lines, index, spell_id) {
        index += 1;
    }
    push_spell_cast_line(state, lines, index, spell_id);
    index += 1;
    push_spell_description_line(state, lines, index, spell_id);
}

fn push_spell_name_line(state: &mut LuaState, lines: Val, index: i64, spell_name: &str) {
    push_tooltip_line(
        state,
        lines,
        index,
        LINE_TYPE_SPELL_NAME,
        spell_name,
        None,
        false,
    );
}

/// Returns true when a cost line was written, so the caller can advance
/// the running index.
fn push_spell_cost_line(state: &mut LuaState, lines: Val, index: i64, spell_id: u32) -> bool {
    let Some(cost) = spell_cost_line(spell_id) else {
        return false;
    };
    push_tooltip_line(state, lines, index, LINE_TYPE_SPELL_NAME, cost, None, false);
    true
}

fn push_spell_cast_line(state: &mut LuaState, lines: Val, index: i64, spell_id: u32) {
    let cast_line = spell_cast_line(spell_id);
    push_tooltip_line(
        state,
        lines,
        index,
        LINE_TYPE_SPELL_NAME,
        &cast_line,
        None,
        false,
    );
}

fn push_spell_description_line(state: &mut LuaState, lines: Val, index: i64, spell_id: u32) {
    let description =
        spell_descriptions::get_spell_description(spell_id).unwrap_or("No description available.");
    push_tooltip_line(
        state,
        lines,
        index,
        LINE_TYPE_SPELL_DESCRIPTION,
        description,
        None,
        true,
    );
}

fn tooltip_for_spell_id(state: &mut LuaState, spell_id: u32) -> Val {
    let Some(spell) = spells::get_spell(spell_id) else {
        return empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    let lines = table_get(state, tooltip, "lines");
    push_spell_tooltip_lines(state, lines, spell_id, spell.name);
    table_set(state, tooltip, "id", Val::Num(spell_id as f64));
    tooltip
}

fn tooltip_for_unit_aura(
    state: &mut LuaState,
    aura: Option<crate::lua_api::game_data::AuraInfo>,
) -> Val {
    let Some(aura) = aura else {
        return empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA);
    let lines = table_get(state, tooltip, "lines");
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_SPELL_NAME,
        &aura.name,
        None,
        false,
    );
    push_tooltip_line(state, lines, 2, LINE_TYPE_SPELL_NAME, "1 hr", None, false);
    let description = spell_descriptions::get_spell_description(aura.spell_id as u32)
        .unwrap_or("No description available.");
    push_tooltip_line(
        state,
        lines,
        3,
        LINE_TYPE_SPELL_DESCRIPTION,
        description,
        None,
        true,
    );
    tooltip
}

fn class_color(class_index: i32) -> (f64, f64, f64) {
    match class_index {
        1 => (0.78, 0.61, 0.43),
        2 => (0.96, 0.55, 0.73),
        3 => (0.67, 0.83, 0.45),
        4 => (1.0, 0.96, 0.41),
        5 => (1.0, 1.0, 1.0),
        6 => (0.77, 0.12, 0.23),
        7 => (0.0, 0.44, 0.87),
        8 => (0.25, 0.78, 0.92),
        9 => (0.53, 0.53, 0.93),
        10 => (0.0, 1.0, 0.6),
        11 => (1.0, 0.49, 0.04),
        12 => (0.64, 0.19, 0.79),
        13 => (0.2, 0.58, 0.5),
        _ => (1.0, 1.0, 1.0),
    }
}

struct UnitTooltipInfo {
    name: String,
    level: i32,
    race: String,
    class_name: String,
    color: (f64, f64, f64),
}

fn class_label(class_index: i32) -> String {
    CLASS_LABELS
        .get((class_index - 1).max(0) as usize)
        .copied()
        .unwrap_or("Unknown")
        .to_string()
}

fn unit_tooltip_info(state: &LuaState, unit: &str) -> Option<UnitTooltipInfo> {
    let sim = borrow_state(state).ok()?;
    match unit {
        "target" => sim.current_target.as_ref().map(|target| UnitTooltipInfo {
            name: target.name.clone(),
            level: target.level,
            race: target.creature_type.clone(),
            class_name: class_label(target.class_index),
            color: class_color(target.class_index),
        }),
        "player" => {
            let player = &sim.player;
            let race = RACE_DATA
                .get(player.race_index)
                .map(|(name, _, _)| (*name).to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            Some(UnitTooltipInfo {
                name: player.name.clone(),
                level: player.level,
                race,
                class_name: class_label(player.class_index),
                color: class_color(player.class_index),
            })
        }
        _ => None,
    }
}

fn push_plain_line(state: &mut LuaState, lines: Val, index: i64, text: &str) {
    push_tooltip_line(state, lines, index, LINE_TYPE_SPELL_NAME, text, None, false);
}

fn push_unit_tooltip_lines(state: &mut LuaState, lines: Val, info: &UnitTooltipInfo) {
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_UNIT_NAME,
        &info.name,
        Some(info.color),
        false,
    );
    let level_text = format!("Level {}", info.level);
    push_plain_line(state, lines, 2, &level_text);
    push_plain_line(state, lines, 3, &info.race);
    push_plain_line(state, lines, 4, &info.class_name);
}

fn tooltip_for_unit(state: &mut LuaState, unit: &str) -> Val {
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_UNIT);
    let lines = table_get(state, tooltip, "lines");
    if let Some(info) = unit_tooltip_info(state, unit) {
        push_unit_tooltip_lines(state, lines, &info);
    }
    tooltip
}

fn tooltip_for_world_loot(state: &mut LuaState) -> Val {
    let tooltip = tooltip_for_spell_id(state, WORLD_LOOT_TOOLTIP_SPELL_ID);
    table_set(
        state,
        tooltip,
        "worldLootObjectInventoryType",
        Val::Num(WORLD_LOOT_TOOLTIP_INVENTORY_TYPE),
    );
    let guid = create_string(state, WORLD_CURSOR_GUID);
    table_set(state, tooltip, "worldLootObjectGUID", guid);
    tooltip
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::env::WowLuaEnv;
    use rilua::{Lua, LuaApiMut};

    #[test]
    fn tooltip_for_item_id_populates_name_and_level_lines() {
        let mut lua = Lua::new().expect("should create rilua state");
        let tooltip = {
            let state = lua.state_mut();
            tooltip_for_item_id(state, 6948)
        };
        let mut state = lua.state_mut();
        let lines = table_get(&mut state, tooltip, "lines");

        let name_line = get_array_element(&mut state, lines, 1);
        assert_eq!(line_type(&mut state, name_line), Some(LINE_TYPE_ITEM_NAME));
        assert_eq!(
            line_text(&mut state, name_line).as_deref(),
            Some("Hearthstone")
        );

        let level_line = get_array_element(&mut state, lines, 2);
        assert_eq!(
            line_type(&mut state, level_line),
            Some(LINE_TYPE_ITEM_LEVEL)
        );
        assert_eq!(
            line_text(&mut state, level_line).as_deref(),
            Some("Item Level 1")
        );
    }

    #[test]
    fn tooltip_for_unit_player_shows_name_and_level() {
        let env = WowLuaEnv::new().expect("should create WowLuaEnv");
        {
            let mut sim = env.state().borrow_mut();
            sim.player.name = "Tester".to_string();
            sim.player.level = 99;
            sim.player.class_index = 3;
            sim.player.race_index = 1;
        }
        let tooltip = {
            let mut lua = env.rilua_mut();
            tooltip_for_unit(lua.state_mut(), "player")
        };
        let mut lua = env.rilua_mut();
        let state = lua.state_mut();
        let lines = table_get(state, tooltip, "lines");
        let name_line = get_array_element(state, lines, 1);
        assert_eq!(line_text(state, name_line).as_deref(), Some("Tester"));

        let level_line = get_array_element(state, lines, 2);
        assert_eq!(line_text(state, level_line).as_deref(), Some("Level 99"));
    }

    fn get_array_element(state: &mut LuaState, table: Val, index: i64) -> Val {
        let Val::Table(table_ref) = table else {
            return Val::Nil;
        };
        state
            .gc
            .tables
            .get(table_ref)
            .map(|table| table.get_int(index))
            .unwrap_or(Val::Nil)
    }

    fn line_type(state: &mut LuaState, line: Val) -> Option<f64> {
        match table_get(state, line, "type") {
            Val::Num(value) => Some(value),
            _ => None,
        }
    }

    fn line_text(state: &mut LuaState, line: Val) -> Option<String> {
        let text_val = table_get(state, line, "leftText");
        val_to_string(state, text_val)
    }
}

fn register_item_spell_aura_methods(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
) -> LuaResult<()> {
    register_tooltip_methods(
        state,
        table_ref,
        &[
            ("GetTraitEntry", c_tooltip_get_trait_entry),
            ("GetAction", c_tooltip_get_action),
            ("GetAchievementByID", c_tooltip_get_achievement_by_id),
            ("GetAura", c_tooltip_get_aura),
            ("GetBagItem", c_tooltip_get_bag_item),
            ("GetCurrencyByID", c_tooltip_get_currency_by_id),
            ("GetCurrencyToken", c_tooltip_get_currency_token),
            ("GetGuildBankItem", c_tooltip_get_guild_bank_item),
            (
                "GetInstanceLockEncountersComplete",
                c_tooltip_get_instance_lock_encounters_complete,
            ),
            ("GetItem", c_tooltip_get_item),
            ("GetItemByID", c_tooltip_get_item_by_id),
            ("GetItemByGUID", c_tooltip_get_item_by_guid),
            ("GetLFGDungeon", c_tooltip_get_lfg_dungeon),
            ("GetOwnedItemByID", c_tooltip_get_owned_item_by_id),
            ("GetPetAction", c_tooltip_get_pet_action),
            ("GetQuestCurrency", c_tooltip_get_quest_currency),
            ("GetQuestItem", c_tooltip_get_quest_item),
            ("GetQuestLogCurrency", c_tooltip_get_quest_log_currency),
            ("GetQuestLogItem", c_tooltip_get_quest_log_item),
            ("GetRecipeReagentItem", c_tooltip_get_recipe_reagent_item),
            ("GetRecipeResultItem", c_tooltip_get_recipe_result_item),
            (
                "GetRecipeResultItemForOrder",
                c_tooltip_get_recipe_result_item_for_order,
            ),
            ("GetShapeshift", c_tooltip_get_shapeshift),
            ("GetTradeSkillItem", c_tooltip_get_trade_skill_item),
            ("GetTradePlayerItem", c_tooltip_get_trade_player_item),
            ("GetTradeTargetItem", c_tooltip_get_trade_target_item),
            ("GetTrainerService", c_tooltip_get_trainer_service),
            ("GetSocketedItem", c_tooltip_get_socketed_item),
            ("GetSocketGem", c_tooltip_get_socket_gem),
            ("GetExistingSocketGem", c_tooltip_get_existing_socket_gem),
            ("GetMountBySpellID", c_tooltip_get_mount_by_spell_id),
            ("GetTalent", c_tooltip_get_talent),
            ("GetToyByItemID", c_tooltip_get_toy_by_item_id),
            ("GetMinimapMouseover", c_tooltip_get_minimap_mouseover),
            ("GetUpgradeItem", c_tooltip_get_upgrade_item),
            ("GetInventoryItem", c_tooltip_get_inventory_item),
            ("GetMerchantItem", c_tooltip_get_merchant_item),
            ("GetTooltipDataForItem", c_tooltip_get_tooltip_data_for_item),
        ],
    )
}

fn register_spell_aura_unit_methods(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
) -> LuaResult<()> {
    register_tooltip_methods(
        state,
        table_ref,
        &[
            ("GetSpellBookItem", c_tooltip_get_spell_book_item),
            ("GetSpellByID", c_tooltip_get_spell_by_id),
            ("GetUnitBuff", c_tooltip_get_unit_buff),
            (
                "GetUnitBuffByAuraInstanceID",
                c_tooltip_get_unit_buff_by_aura_instance_id,
            ),
            ("GetUnitDebuff", c_tooltip_get_unit_debuff),
            (
                "GetUnitDebuffByAuraInstanceID",
                c_tooltip_get_unit_debuff_by_aura_instance_id,
            ),
            ("GetUnitAura", c_tooltip_get_unit_aura),
            (
                "GetUnitAuraByAuraInstanceID",
                c_tooltip_get_unit_aura_by_aura_instance_id,
            ),
            ("GetHyperlink", c_tooltip_get_hyperlink),
            ("GetInboxItem", c_tooltip_get_inbox_item),
            ("GetSendMailItem", c_tooltip_get_send_mail_item),
            ("GetSpell", c_tooltip_get_spell),
            ("GetWorldCursor", c_tooltip_get_world_cursor),
            ("GetWorldLootObject", c_tooltip_get_world_loot_object),
            ("GetUnit", c_tooltip_get_unit),
        ],
    )
}

fn register_c_tooltip_info(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_TooltipInfo")?;
    register_item_spell_aura_methods(state, table_ref)?;
    register_spell_aura_unit_methods(state, table_ref)?;
    Ok(())
}

type TooltipScriptFn = fn(&mut LuaState) -> LuaResult<u32>;

fn register_tooltip_methods(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
    entries: &[(&str, TooltipScriptFn)],
) -> LuaResult<()> {
    for &(name, func) in entries {
        table_set_rust_fn(state, table_ref, name, func)?;
    }
    Ok(())
}

fn c_tooltip_get_trait_entry(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = tooltip_for_spell_id(state, 19750);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_action(state: &mut LuaState) -> LuaResult<u32> {
    let slot = u32::from_stack(state, 1)?;
    let spell_id = borrow_state(state)?.action_bars.get(&slot).copied();
    match spell_id {
        Some(spell_id) => {
            let tooltip = tooltip_for_spell_id(state, spell_id);
            state.push(tooltip);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn tooltip_for_currency(
    state: &mut LuaState,
    currency: &currency_data::CurrencyEntry,
    amount_override: Option<i32>,
) -> Val {
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_CURRENCY);
    let lines = table_get(state, tooltip, "lines");
    let amount = amount_override.unwrap_or(currency.quantity);
    let max_display = if currency.max_quantity > 0 {
        format!("{amount} / {}", currency.max_quantity)
    } else {
        amount.to_string()
    };

    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_SPELL_NAME,
        currency.name,
        Some(item_quality_color(currency.quality as u8)),
        false,
    );
    push_tooltip_line(
        state,
        lines,
        2,
        LINE_TYPE_SPELL_DESCRIPTION,
        &format!("Amount: {max_display}"),
        None,
        false,
    );

    table_set(state, tooltip, "id", Val::Num(currency.currency_id as f64));
    table_set(state, tooltip, "quantity", Val::Num(amount as f64));
    tooltip
}

fn tooltip_for_bag_item(state: &mut LuaState, bag: i32, slot: i32) -> Val {
    let item_id = borrow_state(state)
        .ok()
        .and_then(|st| st.get_bag_item(bag, slot).map(|(item_id, _)| item_id))
        .unwrap_or(0);
    tooltip_for_item_id(state, item_id)
}

fn tooltip_for_inventory_slot(state: &mut LuaState, slot: i32) -> Val {
    let item_id = borrow_state(state)
        .ok()
        .and_then(|st| st.player.equipped_items.get(&slot).map(|item| item.item_id))
        .unwrap_or(0);
    tooltip_for_item_id(state, item_id)
}

fn tooltip_for_item_source(state: &mut LuaState, value: Val) -> Val {
    match value {
        Val::Table(location) => {
            let slot = match table_get(state, Val::Table(location), "equipmentSlotIndex") {
                Val::Num(value) => Some(value as i32),
                _ => None,
            };
            if let Some(slot) = slot {
                return tooltip_for_inventory_slot(state, slot);
            }

            let bag = match table_get(state, Val::Table(location), "bagID") {
                Val::Num(value) => Some(value as i32),
                _ => None,
            };
            let slot = match table_get(state, Val::Table(location), "slotIndex") {
                Val::Num(value) => Some(value as i32),
                _ => None,
            };
            match (bag, slot) {
                (Some(bag), Some(slot)) => tooltip_for_bag_item(state, bag, slot),
                _ => empty_tooltip(state, TOOLTIP_TYPE_ITEM),
            }
        }
        other => {
            let item_id = match other {
                Val::Num(value) if value > 0.0 => Some(value as u32),
                Val::Str(_) => val_to_string(state, other).and_then(|text| {
                    parse_prefixed_id(&text, "item")
                        .or_else(|| text.strip_prefix("item:")?.split(':').next()?.parse().ok())
                        .or_else(|| text.parse().ok())
                }),
                _ => None,
            };
            item_id
                .map(|item_id| tooltip_for_item_id(state, item_id))
                .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM))
        }
    }
}

fn parse_spell_source(state: &mut LuaState, value: Val) -> Option<u32> {
    match value {
        Val::Num(value) if value > 0.0 => Some(value as u32),
        Val::Str(_) => val_to_string(state, value).and_then(|text| {
            parse_prefixed_id(&text, "spell")
                .or_else(|| text.strip_prefix("spell:")?.split(':').next()?.parse().ok())
                .or_else(|| text.parse().ok())
        }),
        _ => None,
    }
}

fn c_tooltip_get_bag_item(state: &mut LuaState) -> LuaResult<u32> {
    let bag = i32::from_stack(state, 1)?;
    let slot = i32::from_stack(state, 2)?;
    let tooltip = tooltip_for_bag_item(state, bag, slot);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_currency_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let currency_id = i32::from_stack(state, 1)?;
    let amount = Option::<i32>::from_stack(state, 2)?;
    let tooltip = currency_data::get_currency_by_id(currency_id)
        .map(|currency| tooltip_for_currency(state, currency, amount))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_CURRENCY));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_currency_token(state: &mut LuaState) -> LuaResult<u32> {
    let token_index = i32::from_stack(state, 1)?;
    let tooltip = currency_data::backpack_currencies()
        .nth((token_index - 1).max(0) as usize)
        .map(|currency| tooltip_for_currency(state, currency, None))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_CURRENCY));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = tooltip_for_item_source(state, stack_val(state, 1));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_item_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let tooltip = tooltip_for_item_id(state, item_id);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_item_by_guid(state: &mut LuaState) -> LuaResult<u32> {
    let guid = String::from_stack(state, 1)?;
    let tooltip = if let Some((_, _, item_id)) = parse_item_guid(&guid) {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    let guid = create_string(state, &guid);
    table_set(state, tooltip, "guid", guid);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_owned_item_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let owned = borrow_state(state)?
        .bag_items
        .values()
        .any(|item| item.item_id == item_id);
    let tooltip = if owned {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_recipe_reagent_item(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let reagent_index = i32::from_stack(state, 2)?;
    let tooltip = trade_skill_item_id(recipe_id, Some(reagent_index))
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn recipe_output_item(recipe_id: i32) -> Option<u32> {
    profession_data::get_recipe(recipe_id)
        .and_then(|recipe| (recipe.output_item_id != 0).then_some(recipe.output_item_id))
}

fn trade_skill_item_id(recipe_id: i32, reagent_index: Option<i32>) -> Option<u32> {
    let recipe = profession_data::get_recipe(recipe_id)?;
    let reagent_index = reagent_index.unwrap_or(0);
    if reagent_index <= 0 {
        return (recipe.output_item_id != 0).then_some(recipe.output_item_id);
    }

    let zero_based = usize::try_from(reagent_index.saturating_sub(1)).ok()?;
    recipe
        .reagents
        .get(zero_based)
        .map(|reagent| reagent.item_id)
}

fn c_tooltip_get_recipe_result_item(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let tooltip = if let Some(item_id) = recipe_output_item(recipe_id) {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_recipe_result_item_for_order(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let tooltip = if let Some(item_id) = recipe_output_item(recipe_id) {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_trade_skill_item(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let reagent_index = Option::<i32>::from_stack(state, 2)?;
    let tooltip = trade_skill_item_id(recipe_id, reagent_index)
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn trade_slot_item_id(state: &mut LuaState, slot: i32, player_side: bool) -> Option<u32> {
    let zero_based = usize::try_from(slot.saturating_sub(1)).ok()?;
    let st = borrow_state(state).ok()?;
    let trade = st.active_trade.as_ref()?;
    let item_id = if player_side {
        *trade.player_slots.get(zero_based)?
    } else {
        *trade.target_slots.get(zero_based)?
    };
    (item_id != 0).then_some(item_id)
}

fn merchant_item_id(state: &LuaState, slot: i32) -> Option<u32> {
    let zero_based = usize::try_from(slot.saturating_sub(1)).ok()?;
    borrow_state(state)
        .ok()?
        .merchant_items
        .get(zero_based)
        .copied()
        .filter(|item_id| *item_id != 0)
}

fn tooltip_for_toy_item_id(state: &mut LuaState, item_id: u32) -> Val {
    if items::get_item(item_id).is_some() {
        return tooltip_for_item_id(state, item_id);
    }

    let toy_name = borrow_state(state).ok().and_then(|st| {
        st.world
            .toys
            .iter()
            .find(|toy| toy.item_id == item_id)
            .map(|toy| toy.name.clone())
    });
    let Some(toy_name) = toy_name else {
        return empty_tooltip(state, TOOLTIP_TYPE_ITEM);
    };

    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_ITEM);
    let lines = table_get(state, tooltip, "lines");
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_ITEM_NAME,
        &toy_name,
        Some(item_quality_color(1)),
        false,
    );
    tooltip
}

fn tooltip_for_mount_spell_id(state: &mut LuaState, spell_id: u32) -> Val {
    if spells::get_spell(spell_id).is_some() {
        return tooltip_for_spell_id(state, spell_id);
    }

    let mount_name = borrow_state(state).ok().and_then(|st| {
        st.world
            .mounts
            .iter()
            .find(|mount| mount.spell_id == spell_id)
            .map(|mount| mount.name.clone())
    });
    let Some(mount_name) = mount_name else {
        return empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    };

    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    let lines = table_get(state, tooltip, "lines");
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_SPELL_NAME,
        &mount_name,
        None,
        false,
    );
    push_tooltip_line(
        state,
        lines,
        3,
        LINE_TYPE_SPELL_DESCRIPTION,
        "Summons this mount.",
        None,
        true,
    );
    table_set(state, tooltip, "id", Val::Num(spell_id as f64));
    tooltip
}

fn preferred_trait_spell_id(definition_id: u32) -> Option<u32> {
    let definition = TRAIT_DEFINITION_DB.get(&definition_id)?;
    [
        definition.visible_spell_id,
        definition.overrides_spell_id,
        definition.spell_id,
    ]
    .into_iter()
    .find(|spell_id| *spell_id != 0)
}

fn spell_id_for_trait_entry(entry_id: u32) -> Option<u32> {
    let mut current_id = entry_id;
    for _ in 0..8 {
        if let Some(spell_id) = preferred_trait_spell_id(current_id) {
            return Some(spell_id);
        }
        current_id = TRAIT_ENTRY_DB.get(&current_id)?.definition_id;
    }
    None
}

fn spell_id_for_talent_id(state: &LuaState, talent_id: u32) -> Option<u32> {
    if let Some(node) = TRAIT_NODE_DB.get(&talent_id) {
        let selected_entry_id = borrow_state(state)
            .ok()
            .and_then(|sim| sim.talents.node_selections.get(&talent_id).copied());
        let entry_id = selected_entry_id.or_else(|| node.entry_ids.first().copied())?;
        return spell_id_for_trait_entry(entry_id);
    }

    spell_id_for_trait_entry(talent_id).or_else(|| preferred_trait_spell_id(talent_id))
}

fn c_tooltip_get_trade_player_item(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let tooltip = trade_slot_item_id(state, slot, true)
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_trade_target_item(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let tooltip = trade_slot_item_id(state, slot, false)
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_merchant_item(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let tooltip = merchant_item_id(state, slot)
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_talent(state: &mut LuaState) -> LuaResult<u32> {
    let talent_id = u32::from_stack(state, 1)?;
    let tooltip = spell_id_for_talent_id(state, talent_id)
        .map(|spell_id| tooltip_for_spell_id(state, spell_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_SPELL));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_mount_by_spell_id(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let tooltip = tooltip_for_mount_spell_id(state, spell_id);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_toy_by_item_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let tooltip = tooltip_for_toy_item_id(state, item_id);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_socketed_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = item_socket_info::socketed_item_id(state)
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_socket_gem(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let tooltip = item_socket_info::new_socket_item_id(state, index)
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_existing_socket_gem(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let tooltip = item_socket_info::existing_socket_item_id(state, index)
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_minimap_mouseover(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_MINIMAP_MOUSEOVER);
    let lines = table_get(state, tooltip, "lines");
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_SPELL_NAME,
        "Stormwind City",
        None,
        false,
    );
    push_tooltip_line(
        state,
        lines,
        2,
        LINE_TYPE_SPELL_NAME,
        "Trade District",
        None,
        false,
    );
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_upgrade_item(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = current_item_upgrade_location(state).and_then(|(bag, slot)| {
        borrow_state(state)
            .ok()?
            .get_bag_item(bag, slot)
            .map(|(item_id, _)| item_id)
    });
    let tooltip = if let Some(item_id) = item_id {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_inventory_item(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let slot = i32::from_stack(state, 2)?;
    let tooltip = tooltip_for_inventory_slot(state, slot);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_tooltip_data_for_item(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = tooltip_for_item_source(state, stack_val(state, 1));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_spell_book_item(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    match spellbook_data::get_spell_at_slot(slot) {
        Some((_, entry, _)) => {
            let tooltip = tooltip_for_spell_id(state, entry.spell_id);
            state.push(tooltip);
        }
        None => {
            let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
            state.push(tooltip);
        }
    }
    Ok(1)
}

fn c_tooltip_get_spell_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let tooltip = tooltip_for_spell_id(state, spell_id);
    state.push(tooltip);
    Ok(1)
}

fn lookup_player_aura(state: &LuaState, index: i32) -> Option<crate::lua_api::game_data::AuraInfo> {
    let sim = borrow_state(state).ok()?;
    sim.player.buffs.get((index - 1).max(0) as usize).cloned()
}

fn lookup_player_aura_by_instance_id(
    state: &LuaState,
    aura_instance_id: i32,
) -> Option<crate::lua_api::game_data::AuraInfo> {
    let sim = borrow_state(state).ok()?;
    sim.player
        .buffs
        .iter()
        .find(|aura| aura.aura_instance_id == aura_instance_id)
        .cloned()
}

fn c_tooltip_get_unit_buff(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    let aura = lookup_player_aura(state, index);
    let tooltip = tooltip_for_unit_aura(state, aura);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_buff_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let aura_instance_id = i32::from_stack(state, 2)?;
    let aura = lookup_player_aura_by_instance_id(state, aura_instance_id);
    let tooltip = tooltip_for_unit_aura(state, aura);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_debuff(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let _index = i32::from_stack(state, 2)?;
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_debuff_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let _aura_instance_id = i32::from_stack(state, 2)?;
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_aura(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    let filter = String::from_stack(state, 3).unwrap_or_default();
    let tooltip = if filter.eq_ignore_ascii_case("HARMFUL") {
        empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA)
    } else {
        let aura = lookup_player_aura(state, index);
        tooltip_for_unit_aura(state, aura)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_aura_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let aura_instance_id = i32::from_stack(state, 2)?;
    let aura = lookup_player_aura_by_instance_id(state, aura_instance_id);
    let tooltip = tooltip_for_unit_aura(state, aura);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_hyperlink(state: &mut LuaState) -> LuaResult<u32> {
    let link = String::from_stack(state, 1)?;
    let tooltip = if let Some(item_id) = parse_prefixed_id(&link, "item") {
        tooltip_for_item_id(state, item_id)
    } else if let Some(spell_id) = parse_prefixed_id(&link, "spell") {
        tooltip_for_spell_id(state, spell_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn inbox_attachment_item_id(
    state: &mut LuaState,
    message_index: i32,
    attachment_index: Option<i32>,
) -> Option<u32> {
    let zero_based_message = usize::try_from(message_index.saturating_sub(1)).ok()?;
    let zero_based_attachment =
        usize::try_from(attachment_index.unwrap_or(1).saturating_sub(1)).ok()?;
    let st = borrow_state(state).ok()?;
    st.player
        .inbox
        .get(zero_based_message)?
        .items
        .get(zero_based_attachment)
        .map(|item| item.item_id)
}

fn send_mail_attachment_item_id(
    state: &mut LuaState,
    attachment_index: Option<i32>,
) -> Option<u32> {
    let zero_based = usize::try_from(attachment_index.unwrap_or(1).saturating_sub(1)).ok()?;
    let st = borrow_state(state).ok()?;
    st.player
        .send_mail_items
        .get(zero_based)
        .and_then(|item| item.as_ref())
        .map(|item| item.item_id)
}

fn c_tooltip_get_inbox_item(state: &mut LuaState) -> LuaResult<u32> {
    let message_index = i32::from_stack(state, 1)?;
    let attachment_index = Option::<i32>::from_stack(state, 2)?;
    let tooltip = inbox_attachment_item_id(state, message_index, attachment_index)
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_send_mail_item(state: &mut LuaState) -> LuaResult<u32> {
    let attachment_index = Option::<i32>::from_stack(state, 1)?;
    let tooltip = send_mail_attachment_item_id(state, attachment_index)
        .map(|item_id| tooltip_for_item_id(state, item_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_ITEM));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_spell(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = parse_spell_source(state, stack_val(state, 1))
        .map(|spell_id| tooltip_for_spell_id(state, spell_id))
        .unwrap_or_else(|| empty_tooltip(state, TOOLTIP_TYPE_SPELL));
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_world_cursor(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = tooltip_for_world_loot(state);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_world_loot_object(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let tooltip = tooltip_for_world_loot(state);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit(state: &mut LuaState) -> LuaResult<u32> {
    let unit = String::from_stack(state, 1)?;
    let tooltip = tooltip_for_unit(state, &unit);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_achievement_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    let name = borrow_state(state)?
        .achievements
        .get(&achievement_id)
        .map(|a| a.name.clone());
    let Some(name) = name else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    let lines = table_get(state, tooltip, "lines");
    push_plain_line(state, lines, 1, &name);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_aura(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    let filter = Option::<String>::from_stack(state, 3)?.unwrap_or_default();
    let tooltip = if filter.eq_ignore_ascii_case("HARMFUL") {
        empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA)
    } else {
        let aura = lookup_player_aura(state, index);
        tooltip_for_unit_aura(state, aura)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_guild_bank_item(state: &mut LuaState) -> LuaResult<u32> {
    let _tab = i32::from_stack(state, 1)?;
    let _slot = i32::from_stack(state, 2)?;
    state.push(Val::Nil);
    Ok(1)
}

fn c_tooltip_get_instance_lock_encounters_complete(state: &mut LuaState) -> LuaResult<u32> {
    let _difficulty_id = Option::<i32>::from_stack(state, 1)?;
    let _lock_id = Option::<i32>::from_stack(state, 2)?;
    state.push(Val::Nil);
    Ok(1)
}

fn c_tooltip_get_lfg_dungeon(state: &mut LuaState) -> LuaResult<u32> {
    let _dungeon_id = Option::<i32>::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}

/// Ensure `C_PetInfo` namespace exists, register `_state()` accessor,
/// and return the mutable state sub-table.
fn ensure_pet_info_state(state: &mut LuaState) -> Val {
    let ns_ref = super::ensure_namespace(state, "C_PetInfo").expect("C_PetInfo namespace");
    let ns = Val::Table(ns_ref);
    let pet_state = match table_get(state, ns, "_pet_state_data") {
        table @ Val::Table(_) => table,
        _ => {
            let t = create_table(state);
            // sub-tables used by tests / C_PetInfo._state()
            let spell_map = create_table(state);
            table_set(state, t, "spellByPetActionID", spell_map);
            let action_map = create_table(state);
            table_set(state, t, "petActionsByID", action_map);
            table_set(state, ns, "_pet_state_data", t);
            // register _state() as a Rust function the first time
            let _ = table_set_rust_fn(state, ns_ref, "_state", c_pet_info_get_state);
            t
        }
    };
    pet_state
}

fn c_pet_info_get_state(state: &mut LuaState) -> LuaResult<u32> {
    let pet_state = ensure_pet_info_state(state);
    state.push(pet_state);
    Ok(1)
}

fn table_get_int(state: &LuaState, table: Val, index: i32) -> Val {
    let Val::Table(table_ref) = table else {
        return Val::Nil;
    };
    state
        .gc
        .tables
        .get(table_ref)
        .map(|t| t.get_int(index as i64))
        .unwrap_or(Val::Nil)
}

fn pet_action_spell_id(state: &mut LuaState, slot: i32) -> Option<u32> {
    let pet_state = ensure_pet_info_state(state);
    // spellByPetActionID[slot] → spell id (direct integer key mapping)
    let spell_map = table_get(state, pet_state, "spellByPetActionID");
    if let Val::Num(n) = table_get_int(state, spell_map, slot) {
        return Some(n as u32);
    }
    // petActionsByID[slot].spellID fallback
    let action_map = table_get(state, pet_state, "petActionsByID");
    let entry = table_get_int(state, action_map, slot);
    if let Val::Num(n) = table_get(state, entry, "spellID") {
        return Some(n as u32);
    }
    None
}

fn c_tooltip_get_pet_action(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    match pet_action_spell_id(state, slot) {
        Some(spell_id) => {
            let tooltip = tooltip_for_spell_id(state, spell_id);
            state.push(tooltip);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_tooltip_get_quest_currency(state: &mut LuaState) -> LuaResult<u32> {
    let _quest_id = Option::<i32>::from_stack(state, 1)?;
    let _currency_type = Option::<i32>::from_stack(state, 2)?;
    let _index = Option::<i32>::from_stack(state, 3)?;
    state.push(Val::Nil);
    Ok(1)
}

fn c_tooltip_get_quest_item(state: &mut LuaState) -> LuaResult<u32> {
    let _quest_type = Option::<String>::from_stack(state, 1)?;
    let _slot = Option::<i32>::from_stack(state, 2)?;
    state.push(Val::Nil);
    Ok(1)
}

fn c_tooltip_get_quest_log_currency(state: &mut LuaState) -> LuaResult<u32> {
    let _currency_type = Option::<i32>::from_stack(state, 1)?;
    let _index = Option::<i32>::from_stack(state, 2)?;
    state.push(Val::Nil);
    Ok(1)
}

fn c_tooltip_get_quest_log_item(state: &mut LuaState) -> LuaResult<u32> {
    let _quest_type = Option::<String>::from_stack(state, 1)?;
    let _slot = Option::<i32>::from_stack(state, 2)?;
    state.push(Val::Nil);
    Ok(1)
}

fn c_tooltip_get_shapeshift(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let name = {
        let sim = borrow_state(state)?;
        let zero_based = usize::try_from(slot.saturating_sub(1)).unwrap_or(usize::MAX);
        sim.shapeshift_forms.get(zero_based).cloned()
    };
    let Some(name) = name else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    let lines = table_get(state, tooltip, "lines");
    push_plain_line(state, lines, 1, &name);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_trainer_service(state: &mut LuaState) -> LuaResult<u32> {
    let _service_index = Option::<i32>::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}
