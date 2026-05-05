use super::profession_crafting::{craft_recipe, recipe_is_craftable};
use super::{ensure_namespace, set_table_array};
use crate::items;
use crate::lua_api::globals::{profession_data, spellbook_data};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_get, table_set,
};
use crate::lua_api::script_helpers::{fire_named_event_state, protected_lua_pcall_state};
use crate::lua_api::state_types::CraftingState;
use crate::lua_api::tracked_recipes::TrackedRecipes;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::vm::{gc::arena::GcRef, table::Table};
use rilua::{LuaApiMut, LuaResult, Val};
use std::collections::HashSet;

const TRADE_SKILL_NAMESPACE: &str = "C_TradeSkillUI";
const CRAFTING_ORDERS_NAMESPACE: &str = "C_CraftingOrders";
const SELECTED_PROFESSION_KEY: &str = "_selectedProfessionID";
const BLACKSMITHING_PROFESSION: i32 = 1;
const MINING_PROFESSION: i32 = 6;
const COOKING_PROFESSION: i32 = 9;
const FISHING_PROFESSION: i32 = 10;
const PROF0_INVENTORY_SLOTS: &[i32] = &[20, 21, 22];
const PROF1_INVENTORY_SLOTS: &[i32] = &[23, 24, 25];
const COOKING_INVENTORY_SLOTS: &[i32] = &[26, 27];
const FISHING_INVENTORY_SLOTS: &[i32] = &[28];
const PROFESSION_INVENTORY_SLOTS: &[i32] = &[20, 21, 22, 23, 24, 25, 26, 27, 28];
type NamespaceMethod = (&'static str, fn(&mut LuaState) -> LuaResult<u32>);

const TRADE_SKILL_METHODS: &[NamespaceMethod] = &[
    (
        "GetAllProfessionTradeSkillLines",
        c_trade_skill_ui_get_all_profession_trade_skill_lines,
    ),
    ("GetAllRecipeIDs", c_trade_skill_ui_get_all_recipe_ids),
    (
        "GetBaseProfessionInfo",
        c_trade_skill_ui_get_base_profession_info,
    ),
    ("GetCategoryInfo", c_trade_skill_ui_get_category_info),
    (
        "GetChildProfessionInfo",
        c_trade_skill_ui_get_child_profession_info,
    ),
    (
        "GetChildProfessionInfos",
        c_trade_skill_ui_get_child_profession_infos,
    ),
    ("GetConcentrationCurrencyID", stub_zero),
    (
        "GetCraftingOrderCount",
        c_trade_skill_ui_get_crafting_order_count,
    ),
    (
        "GetDependentReagents",
        c_trade_skill_ui_get_dependent_reagents,
    ),
    ("GetCraftableCount", stub_zero),
    ("GetItemSlotModifications", stub_empty_table),
    ("GetItemSlotModificationsForOrder", stub_empty_table),
    ("GetRecraftRemovalWarnings", stub_empty_table),
    ("GetRemainingRecasts", stub_zero),
    ("IsRecraftItemEquipped", stub_false),
    ("RecraftLimitCategoryValid", stub_true),
    ("CanStoreEnchantInItem", stub_false),
    ("CanTradeSkillListLink", stub_false),
    ("GetCategories", c_trade_skill_ui_get_categories),
    (
        "GetFilteredRecipeIDs",
        c_trade_skill_ui_get_filtered_recipe_ids,
    ),
    (
        "GetProfessionInfoByRecipeID",
        c_trade_skill_ui_get_profession_info_by_recipe_id,
    ),
    (
        "GetProfessionByInventorySlot",
        c_trade_skill_ui_get_profession_by_inventory_slot,
    ),
    (
        "GetProfessionInventorySlots",
        c_trade_skill_ui_get_profession_inventory_slots,
    ),
    (
        "GetProfessionSkillLineID",
        c_trade_skill_ui_get_profession_skill_line_id,
    ),
    ("GetProfessionSlots", c_trade_skill_ui_get_profession_slots),
    ("GetProfessions", c_trade_skill_ui_get_professions),
    ("GetNumRecipes", c_trade_skill_ui_get_num_recipes),
    ("GetNumTradeSkills", c_trade_skill_ui_get_num_trade_skills),
    (
        "GetRecipeDescription",
        c_trade_skill_ui_get_recipe_description,
    ),
    ("GetRecipeInfo", c_trade_skill_ui_get_recipe_info),
    ("GetRecipeItemLink", c_trade_skill_ui_get_recipe_item_link),
    (
        "GetRecipeItemNameFilter",
        c_trade_skill_ui_get_recipe_item_name_filter,
    ),
    (
        "GetRecipeOutputItemData",
        c_trade_skill_ui_get_recipe_output_item_data,
    ),
    ("GetRecipeQualityItemIDs", stub_empty_table),
    (
        "GetRecipeNumReagents",
        c_trade_skill_ui_get_recipe_num_reagents,
    ),
    ("GetRecipeRequirements", stub_empty_table),
    (
        "GetRecipeReagentInfo",
        c_trade_skill_ui_get_recipe_reagent_info,
    ),
    (
        "GetRecipeReagentItemLink",
        c_trade_skill_ui_get_recipe_reagent_item_link,
    ),
    ("GetRecipeSchematic", c_trade_skill_ui_get_recipe_schematic),
    ("GetRecipesTracked", c_trade_skill_ui_get_recipes_tracked),
    ("GetTradeSkillLine", c_trade_skill_ui_get_trade_skill_line),
    (
        "GetTradeSkillListLink",
        c_trade_skill_ui_get_trade_skill_list_link,
    ),
    ("GetFactionSpecificOutputItem", stub_nil),
    (
        "GetTradeSkillTexture",
        c_trade_skill_ui_get_trade_skill_texture,
    ),
    (
        "IsDataSourceChanging",
        c_trade_skill_ui_is_data_source_changing,
    ),
    ("IsNPCCrafting", c_trade_skill_ui_is_npc_crafting),
    ("IsRecipeCraftable", c_trade_skill_ui_is_recipe_craftable),
    (
        "IsRecipeInSkillLine",
        c_trade_skill_ui_is_recipe_in_skill_line,
    ),
    ("IsRecipeLearned", c_trade_skill_ui_is_recipe_learned),
    ("IsRecipeTracked", c_trade_skill_ui_is_recipe_tracked),
    ("IsRuneforging", c_trade_skill_ui_is_runeforging),
    ("IsTradeSkillGuild", c_trade_skill_ui_is_trade_skill_guild),
    (
        "IsTradeSkillGuildMember",
        c_trade_skill_ui_is_trade_skill_guild_member,
    ),
    ("IsTradeSkillLinked", c_trade_skill_ui_is_trade_skill_linked),
    ("IsTradeSkillReady", c_trade_skill_ui_is_trade_skill_ready),
    ("OpenTradeSkill", c_trade_skill_ui_open_trade_skill),
    ("CloseTradeSkill", c_trade_skill_ui_close_trade_skill),
    (
        "SetProfessionChildSkillLineID",
        c_trade_skill_ui_set_profession_child_skill_line_id,
    ),
    ("SetRecipeTracked", c_trade_skill_ui_set_recipe_tracked),
    ("CraftRecipe", c_trade_skill_ui_craft_recipe),
];

const CRAFTING_ORDER_METHODS: &[NamespaceMethod] = &[
    ("AreOrderNotesDisabled", stub_false),
    ("CanOrderSkillAbility", stub_false),
    ("CloseCrafterCraftingOrders", stub_noop),
    ("GetClaimedOrder", stub_nil),
    ("GetCrafterBuckets", stub_empty_table),
    ("GetCrafterOrders", stub_empty_table),
    ("GetCraftingOrderTime", stub_zero),
    ("GetOrderClaimInfo", c_crafting_orders_get_order_claim_info),
    ("OpenCrafterCraftingOrders", stub_noop),
    ("OrderCanBeRecrafted", stub_false),
    ("ShouldShowCraftingOrderTab", stub_false),
    ("SkillLineHasOrders", stub_false),
    ("UpdateIgnoreList", stub_noop),
];

pub(super) fn register_profession_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, TRADE_SKILL_NAMESPACE)?;
    register_namespace_methods(state, table_ref, TRADE_SKILL_METHODS)?;
    let orders_ref = ensure_namespace(state, CRAFTING_ORDERS_NAMESPACE)?;
    register_namespace_methods(state, orders_ref, CRAFTING_ORDER_METHODS)?;
    table_set_rust_fn_static(
        state,
        state.global,
        "GetProfessions",
        get_professions_global,
    )?;
    table_set_rust_fn_static(
        state,
        state.global,
        "GetProfessionInfo",
        get_profession_info_global,
    )?;
    table_set_rust_fn_static(state, state.global, "AbandonSkill", abandon_skill)?;
    Ok(())
}

fn abandon_skill(state: &mut LuaState) -> LuaResult<u32> {
    let skill_line_id = i32::from_stack(state, 1)?;
    abandon_profession_impl(state, skill_line_id);
    fire_named_event_state(state, "SKILL_LINES_CHANGED", &[]);
    Ok(0)
}

pub(crate) fn abandon_profession_impl(state: &mut LuaState, skill_line_id: i32) {
    let mut sim = match borrow_state_mut(state) {
        Ok(s) => s,
        Err(_) => return,
    };
    sim.crafting.unlearned_profession_ids.insert(skill_line_id);
    if sim.crafting.selected_profession_id == Some(skill_line_id) {
        sim.crafting.selected_profession_id = None;
    }
}

pub(crate) fn relearn_profession_impl(state: &mut LuaState, skill_line_id: i32) {
    let mut sim = match borrow_state_mut(state) {
        Ok(s) => s,
        Err(_) => return,
    };
    sim.crafting.unlearned_profession_ids.remove(&skill_line_id);
}

fn c_crafting_orders_get_order_claim_info(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    set_number_field(state, table, "claimsRemaining", 0.0);
    set_number_field(state, table, "secondsToRecharge", 0.0);
    state.push(table);
    Ok(1)
}

fn stub_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn stub_nil(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn stub_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn stub_true(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn stub_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn stub_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

pub(crate) fn open_trade_skill_for_skill_line(
    state: &mut LuaState,
    skill_line_id: i32,
) -> LuaResult<bool> {
    if profession_data::get_profession(skill_line_id).is_none() {
        return Ok(false);
    }

    select_profession(state, skill_line_id)?;
    ensure_professions_frame_loaded(state);
    fire_named_event_state(state, "TRADE_SKILL_LIST_UPDATE", &[]);
    show_professions_frame(state);
    fire_named_event_state(state, "TRADE_SKILL_NAME_UPDATE", &[]);
    Ok(true)
}

fn ensure_professions_frame_loaded(state: &mut LuaState) {
    if !matches!(
        LuaApiMut::get_global_val(state, "ProfessionsFrame"),
        Val::Nil
    ) {
        return;
    }
    call_global_function(state, "ProfessionsFrame_LoadUI", &[]);
}

fn show_professions_frame(state: &mut LuaState) {
    let frame = LuaApiMut::get_global_val(state, "ProfessionsFrame");
    if matches!(frame, Val::Nil) {
        return;
    }

    let show_ui_panel = LuaApiMut::get_global_val(state, "ShowUIPanel");
    if matches!(show_ui_panel, Val::Function(_)) {
        let _ = protected_lua_pcall_state(state, show_ui_panel, &[frame]);
        return;
    }

    let show_method = table_get(state, frame, "Show");
    if matches!(show_method, Val::Function(_)) {
        let _ = protected_lua_pcall_state(state, show_method, &[frame]);
    }
}

fn call_global_function(state: &mut LuaState, name: &str, args: &[Val]) {
    let function = LuaApiMut::get_global_val(state, name);
    if !matches!(function, Val::Function(_)) {
        return;
    }
    let _ = protected_lua_pcall_state(state, function, args);
}

fn c_trade_skill_ui_get_all_profession_trade_skill_lines(state: &mut LuaState) -> LuaResult<u32> {
    let table = skill_line_id_table(state);
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_all_recipe_ids(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_ids = profession_data::get_all_recipe_ids();
    let table = recipe_id_table(state, &recipe_ids);
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_base_profession_info(state: &mut LuaState) -> LuaResult<u32> {
    let table = profession_table(state, profession_data::get_profession_by_index(0));
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_category_info(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i32::from_stack(state, 1)?;
    let table = category_table(state, profession_data::get_category(category_id));
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_child_profession_info(state: &mut LuaState) -> LuaResult<u32> {
    let profession = selected_profession(state);
    let table = profession_table(state, profession);
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_child_profession_infos(state: &mut LuaState) -> LuaResult<u32> {
    let table = all_profession_tables(state);
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_crafting_order_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// `C_TradeSkillUI.GetDependentReagents(reagent) -> CraftingReagent[]`.
///
/// Always returns an array (possibly empty). Returning nil here would crash
/// callers that wrap the result in `ipairs`, e.g.
/// `ProfessionsRecipeTransactionMixin:AreDependentReagentsAllocated`.
fn c_trade_skill_ui_get_dependent_reagents(state: &mut LuaState) -> LuaResult<u32> {
    let reagent = Val::from_stack(state, 1).unwrap_or(Val::Nil);
    let item_id = match reagent {
        Val::Table(_) => match table_get(state, reagent, "itemID") {
            Val::Num(n) if n > 0.0 => Some(n as u32),
            _ => None,
        },
        _ => None,
    };
    let dependents: &'static [profession_data::ReagentSlot] = match item_id {
        Some(id) => profession_data::find_reagent_dependents(id),
        None => &[],
    };
    let table = create_table(state);
    for (index, dep) in dependents.iter().enumerate() {
        let entry = create_table(state);
        table_set(state, entry, "itemID", Val::Num(dep.item_id as f64));
        table_set(
            state,
            entry,
            "quantityRequired",
            Val::Num(dep.quantity as f64),
        );
        set_table_array(state, table, (index + 1) as i64, entry);
    }
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_categories(state: &mut LuaState) -> LuaResult<u32> {
    let category_ids = profession_data::get_category_ids();
    for category_id in &category_ids {
        state.push(Val::Num(*category_id as f64));
    }
    Ok(category_ids.len() as u32)
}

fn c_trade_skill_ui_get_filtered_recipe_ids(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_ids = profession_data::get_filtered_recipe_ids();
    let table = recipe_id_table(state, &recipe_ids);
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_profession_info_by_recipe_id(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let profession = profession_for_recipe(recipe_id);
    let table = profession_table(state, profession);
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_profession_by_inventory_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    match profession_for_inventory_slot(slot) {
        Some(profession) => state.push(Val::Num(profession as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_trade_skill_ui_get_profession_inventory_slots(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    for (index, slot) in PROFESSION_INVENTORY_SLOTS.iter().enumerate() {
        set_table_array(state, table, (index + 1) as i64, Val::Num(*slot as f64));
    }
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_profession_skill_line_id(state: &mut LuaState) -> LuaResult<u32> {
    let profession_id = i32::from_stack(state, 1)?;
    let skill_line_id = profession_data::PROFESSIONS
        .iter()
        .find(|profession| profession.profession == profession_id)
        .map(|profession| profession.skill_line_id)
        .unwrap_or(profession_id);
    state.push(Val::Num(skill_line_id as f64));
    Ok(1)
}

fn c_trade_skill_ui_get_profession_slots(state: &mut LuaState) -> LuaResult<u32> {
    let profession = i32::from_stack(state, 1)?;
    let slots = profession_slots(profession);
    let table = create_table(state);
    for (index, slot) in slots.iter().enumerate() {
        set_table_array(state, table, (index + 1) as i64, Val::Num(*slot as f64));
    }
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_professions(state: &mut LuaState) -> LuaResult<u32> {
    let table = skill_line_id_table(state);
    state.push(table);
    Ok(1)
}

fn is_profession_unlearned(state: &LuaState, skill_line_id: i32) -> bool {
    borrow_state(state)
        .map(|s| s.crafting.unlearned_profession_ids.contains(&skill_line_id))
        .unwrap_or(false)
}

fn get_professions_global(state: &mut LuaState) -> LuaResult<u32> {
    // Slots 1-2 are the two primaries. Surviving primaries collapse to fill from slot 1.
    let mut slot1 = Val::Nil;
    let mut slot2 = Val::Nil;
    let mut primary_slot = 0usize;
    for (index, profession) in profession_data::PROFESSIONS.iter().enumerate() {
        if profession.parent_profession_name.is_empty()
            && !is_profession_unlearned(state, profession.skill_line_id)
        {
            primary_slot += 1;
            let index_val = Val::Num((index + 1) as f64);
            match primary_slot {
                1 => slot1 = index_val,
                2 => slot2 = index_val,
                _ => break,
            }
        }
    }
    state.push(slot1);
    state.push(slot2);
    state.push(Val::Nil);
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(5)
}

fn get_profession_info_global(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let Some(profession) = visible_global_profession(state, index) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    push_global_profession_info(state, profession);
    Ok(11)
}

fn visible_global_profession(
    state: &mut LuaState,
    index: i32,
) -> Option<&'static profession_data::ProfessionInfo> {
    let profession = global_profession(index)?;
    (!is_profession_unlearned(state, profession.skill_line_id)).then_some(profession)
}

fn push_global_profession_info(state: &mut LuaState, profession: &profession_data::ProfessionInfo) {
    let name = create_string(state, profession.name);
    let icon = create_string(state, profession_icon_path(profession.profession_id));
    let skill_line_name = if profession.parent_profession_name.is_empty() {
        Val::Nil
    } else {
        create_string(state, profession.parent_profession_name)
    };
    let spellbook_skill_line = profession_spellbook_skill_line(profession.name).unwrap_or(0);
    let spell_offset = if spellbook_skill_line > 0 {
        spellbook_data::skill_line_offset(spellbook_skill_line) as f64
    } else {
        0.0
    };
    let num_spells = profession_spell_count(profession.name);

    state.push(name);
    state.push(icon);
    state.push(Val::Num(profession.skill_level as f64));
    state.push(Val::Num(profession.max_skill_level as f64));
    state.push(Val::Num(num_spells as f64));
    state.push(Val::Num(spell_offset));
    state.push(Val::Num(profession.skill_line_id as f64));
    state.push(Val::Num(profession.skill_modifier as f64));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(skill_line_name);
}

fn c_trade_skill_ui_get_num_recipes(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(profession_data::BLACKSMITHING_RECIPES.len() as f64));
    Ok(1)
}

fn c_trade_skill_ui_get_num_trade_skills(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(profession_data::BLACKSMITHING_RECIPES.len() as f64));
    Ok(1)
}

fn global_profession(index: i32) -> Option<&'static profession_data::ProfessionInfo> {
    usize::try_from(index.saturating_sub(1))
        .ok()
        .and_then(profession_data::get_profession_by_index)
}

fn profession_spellbook_skill_line(name: &str) -> Option<i32> {
    (1..=spellbook_data::num_skill_lines()).find(|index| {
        spellbook_data::get_skill_line(*index)
            .map(|skill_line| skill_line.name == name)
            .unwrap_or(false)
    })
}

fn profession_spell_count(name: &str) -> i32 {
    profession_spellbook_skill_line(name)
        .and_then(spellbook_data::get_skill_line)
        .map(|skill_line| skill_line.spells.len() as i32)
        .unwrap_or(0)
}

fn profession_icon_path(profession_id: i32) -> &'static str {
    match profession_id {
        164 => "Interface\\Icons\\Trade_BlackSmithing",
        186 => "Interface\\Icons\\Trade_Mining",
        _ => "Interface\\Icons\\INV_Scroll_04",
    }
}

fn c_trade_skill_ui_get_recipe_info(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let recipe = profession_data::get_recipe(recipe_id);
    let table = recipe_info_table(state, recipe);
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_recipe_description(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let description = u32::try_from(recipe_id)
        .ok()
        .and_then(crate::spell_descriptions::get_spell_description)
        .unwrap_or("");
    let description = create_string(state, description);
    state.push(description);
    Ok(1)
}

fn c_trade_skill_ui_get_recipe_item_link(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let link = profession_data::get_recipe(recipe_id)
        .and_then(|recipe| item_link_value(state, recipe.output_item_id));
    state.push(link.unwrap_or(Val::Nil));
    Ok(1)
}

fn c_trade_skill_ui_get_recipe_item_name_filter(state: &mut LuaState) -> LuaResult<u32> {
    let filter = create_string(state, "");
    state.push(filter);
    Ok(1)
}

fn c_trade_skill_ui_get_recipe_output_item_data(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let recipe = profession_data::get_recipe(recipe_id);
    let table = create_table(state);
    if let Some(recipe) = recipe {
        let link = item_link_value(state, recipe.output_item_id).unwrap_or(Val::Nil);
        table_set(state, table, "hyperlink", link);
        set_number_field(state, table, "icon", item_icon(recipe.output_item_id));
        if recipe.output_item_id == 0 {
            table_set(state, table, "itemID", Val::Nil);
        } else {
            set_number_field(state, table, "itemID", recipe.output_item_id as f64);
        }
    }
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_recipe_num_reagents(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let count = profession_data::get_recipe(recipe_id)
        .map(|recipe| recipe.reagents.len())
        .unwrap_or(0);
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn c_trade_skill_ui_get_recipe_reagent_info(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let reagent_index = reagent_index_from_stack(state)?;
    let reagent = profession_data::get_recipe(recipe_id)
        .and_then(|recipe| recipe.reagents.get(reagent_index));
    let table = reagent_info_table(state, reagent);
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_get_recipe_reagent_item_link(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let reagent_index = reagent_index_from_stack(state)?;
    let link = profession_data::get_recipe(recipe_id)
        .and_then(|recipe| recipe.reagents.get(reagent_index))
        .and_then(|reagent| item_link_value(state, reagent.item_id));
    state.push(link.unwrap_or(Val::Nil));
    Ok(1)
}

fn c_trade_skill_ui_get_recipe_schematic(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let recipe = profession_data::get_recipe(recipe_id);
    let table = recipe_schematic_table(state, recipe);
    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_set_recipe_tracked(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = u32::from_stack(state, 1)?;
    let tracked = bool::from_stack(state, 2)?;
    let is_recrafting = Option::<bool>::from_stack(state, 3)?.unwrap_or(false);

    let changed = {
        let mut sim = borrow_state_mut(state)?;
        sim.tracked_recipes.set(recipe_id, tracked, is_recrafting)
    };
    if !changed {
        return Ok(0);
    }

    let args = [Val::Num(recipe_id as f64), Val::Bool(tracked)];
    fire_named_event_state(state, "TRACKED_RECIPE_UPDATE", &args);
    Ok(0)
}

fn c_trade_skill_ui_is_recipe_tracked(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = u32::from_stack(state, 1)?;
    let is_recrafting = bool::from_stack(state, 2)?;

    let tracked = {
        let sim = borrow_state(state)?;
        is_recipe_tracked(&sim.tracked_recipes, recipe_id, is_recrafting)
    };

    state.push(Val::Bool(tracked));
    Ok(1)
}

fn c_trade_skill_ui_get_recipes_tracked(state: &mut LuaState) -> LuaResult<u32> {
    let is_recrafting = bool::from_stack(state, 1)?;
    let recipe_ids = borrow_state(state)?
        .tracked_recipes
        .list(is_recrafting)
        .to_vec();

    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };

    if let Some(entries) = state.gc.tables.get_mut(table_ref) {
        for (index, recipe_id) in recipe_ids.into_iter().enumerate() {
            let key = Val::Num((index + 1) as f64);
            let value = Val::Num(recipe_id as f64);
            let _ = entries.raw_set(key, value, &state.gc.string_arena);
        }
    }
    state.gc.barrier_back(table_ref);

    state.push(table);
    Ok(1)
}

fn c_trade_skill_ui_is_recipe_learned(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let learned = {
        let sim = borrow_state(state)?;
        is_recipe_learned(&sim.crafting, recipe_id)
    };
    state.push(Val::Bool(learned));
    Ok(1)
}

fn is_recipe_tracked(
    tracked_recipes: &TrackedRecipes,
    recipe_id: u32,
    is_recrafting: bool,
) -> bool {
    tracked_recipes.contains(recipe_id, is_recrafting)
}

fn is_recipe_learned(crafting: &CraftingState, recipe_id: i32) -> bool {
    contains_recipe_id(&crafting.known_recipe_ids, recipe_id)
}

fn contains_recipe_id(recipe_ids: &HashSet<i32>, recipe_id: i32) -> bool {
    recipe_ids.contains(&recipe_id)
}

fn c_trade_skill_ui_is_recipe_craftable(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let count = Option::<i32>::from_stack(state, 2)?.unwrap_or(1).max(1);
    let craftable = recipe_is_craftable(state, recipe_id, count);
    state.push(Val::Bool(craftable));
    Ok(1)
}

fn c_trade_skill_ui_is_recipe_in_skill_line(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let skill_line_id = i32::from_stack(state, 2)?;
    let in_skill_line = profession_data::get_recipe(recipe_id)
        .and_then(|_| profession_data::get_profession(skill_line_id))
        .map(|profession| profession.profession == BLACKSMITHING_PROFESSION)
        .unwrap_or(false);
    state.push(Val::Bool(in_skill_line));
    Ok(1)
}

fn c_trade_skill_ui_craft_recipe(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let count = Option::<i32>::from_stack(state, 2)?.unwrap_or(1).max(1);
    let success = craft_recipe(state, recipe_id, count);
    state.push(Val::Bool(success));
    Ok(1)
}

fn c_trade_skill_ui_get_trade_skill_line(state: &mut LuaState) -> LuaResult<u32> {
    let profession =
        selected_profession(state).or_else(|| profession_data::get_profession_by_index(0));
    let (skill_line_id, skill_level, max_skill_level) = profession
        .map(|profession| {
            (
                profession.skill_line_id as f64,
                profession.skill_level as f64,
                profession.max_skill_level as f64,
            )
        })
        .unwrap_or((0.0, 0.0, 0.0));
    state.push(Val::Num(skill_line_id));
    state.push(Val::Nil);
    state.push(Val::Num(skill_level));
    state.push(Val::Num(max_skill_level));
    Ok(4)
}

fn c_trade_skill_ui_get_trade_skill_list_link(state: &mut LuaState) -> LuaResult<u32> {
    let profession =
        selected_profession(state).or_else(|| profession_data::get_profession_by_index(0));
    let link = profession
        .map(|profession| {
            format!(
                "|cff71d5ff|Htrade:{}:{}:{}|h[{}]|h|r",
                profession.profession_id,
                profession.skill_level,
                profession.max_skill_level,
                profession.name
            )
        })
        .unwrap_or_default();
    let link = create_string(state, &link);
    state.push(link);
    Ok(1)
}

fn c_trade_skill_ui_get_trade_skill_texture(state: &mut LuaState) -> LuaResult<u32> {
    let profession_id = Option::<i32>::from_stack(state, 1)?;
    let icon = profession_id
        .and_then(profession_data::get_profession)
        .or_else(|| profession_data::get_profession_by_index(0))
        .map(|profession| profession.icon as f64)
        .unwrap_or(0.0);
    state.push(Val::Num(icon));
    Ok(1)
}

fn c_trade_skill_ui_is_data_source_changing(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_trade_skill_ui_is_npc_crafting(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_trade_skill_ui_is_runeforging(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_trade_skill_ui_is_trade_skill_guild(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_trade_skill_ui_is_trade_skill_guild_member(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_trade_skill_ui_is_trade_skill_linked(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    state.push(Val::Nil);
    Ok(2)
}

fn c_trade_skill_ui_is_trade_skill_ready(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_trade_skill_ui_open_trade_skill(state: &mut LuaState) -> LuaResult<u32> {
    let skill_line_id = i32::from_stack(state, 1)?;
    let opened = open_trade_skill_for_skill_line(state, skill_line_id)?;
    state.push(Val::Bool(opened));
    Ok(1)
}

fn c_trade_skill_ui_close_trade_skill(state: &mut LuaState) -> LuaResult<u32> {
    fire_named_event_state(state, "TRADE_SKILL_CLOSE", &[]);
    Ok(0)
}

fn c_trade_skill_ui_set_profession_child_skill_line_id(state: &mut LuaState) -> LuaResult<u32> {
    let skill_line_id = i32::from_stack(state, 1)?;
    if profession_data::get_profession(skill_line_id).is_none() {
        return Ok(0);
    }

    select_profession(state, skill_line_id)?;
    Ok(0)
}

fn select_profession(state: &mut LuaState, skill_line_id: i32) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, TRADE_SKILL_NAMESPACE)?;
    borrow_state_mut(state)?.crafting.selected_profession_id = Some(skill_line_id);
    table_set(
        state,
        Val::Table(table_ref),
        SELECTED_PROFESSION_KEY,
        Val::Num(skill_line_id as f64),
    );
    Ok(())
}

fn skill_line_id_table(state: &mut LuaState) -> Val {
    let table = create_table(state);
    let mut slot = 0i64;
    for profession in profession_data::PROFESSIONS.iter() {
        if !is_profession_unlearned(state, profession.skill_line_id) {
            slot += 1;
            set_table_array(
                state,
                table,
                slot,
                Val::Num(profession.profession_id as f64),
            );
        }
    }
    table
}

fn recipe_id_table(state: &mut LuaState, recipe_ids: &[i32]) -> Val {
    let table = create_table(state);
    for (index, recipe_id) in recipe_ids.iter().enumerate() {
        set_table_array(
            state,
            table,
            (index + 1) as i64,
            Val::Num(*recipe_id as f64),
        );
    }
    table
}

fn all_profession_tables(state: &mut LuaState) -> Val {
    let table = create_table(state);
    let mut slot = 0i64;
    for profession in profession_data::PROFESSIONS.iter() {
        if !is_profession_unlearned(state, profession.skill_line_id) {
            slot += 1;
            let value = profession_table(state, Some(profession));
            set_table_array(state, table, slot, value);
        }
    }
    table
}

fn profession_table(
    state: &mut LuaState,
    profession: Option<&profession_data::ProfessionInfo>,
) -> Val {
    let table = create_table(state);
    set_number_field(
        state,
        table,
        "professionID",
        profession
            .map(|profession| profession.profession_id)
            .unwrap_or(0) as f64,
    );
    if let Some(profession) = profession {
        populate_profession_table(state, table, profession);
    }
    table
}

fn category_table(state: &mut LuaState, category: Option<&profession_data::RecipeCategory>) -> Val {
    let Some(category) = category else {
        return Val::Nil;
    };

    let table = create_table(state);
    let name = create_string(state, category.name);
    table_set(
        state,
        table,
        "categoryID",
        Val::Num(category.category_id as f64),
    );
    table_set(state, table, "name", name);
    table_set(
        state,
        table,
        "parentCategoryID",
        Val::Num(category.parent_category_id as f64),
    );
    table_set(state, table, "uiOrder", Val::Num(category.ui_order as f64));
    table
}

fn recipe_info_table(state: &mut LuaState, recipe: Option<&profession_data::RecipeEntry>) -> Val {
    let table = create_table(state);
    match recipe {
        Some(recipe) => populate_recipe_info_table(state, table, recipe),
        None => populate_missing_recipe_info_table(state, table),
    }
    table
}

fn reagent_info_table(state: &mut LuaState, reagent: Option<&profession_data::ReagentSlot>) -> Val {
    let Some(reagent) = reagent else {
        return Val::Nil;
    };

    let table = create_table(state);
    table_set(state, table, "itemID", Val::Num(reagent.item_id as f64));
    table_set(
        state,
        table,
        "numRequired",
        Val::Num(reagent.quantity as f64),
    );
    table_set(
        state,
        table,
        "quantityRequired",
        Val::Num(reagent.quantity as f64),
    );
    table_set(state, table, "reagentType", Val::Num(1.0));
    let name = items::get_item(reagent.item_id)
        .map(|item| item.name)
        .unwrap_or("Unknown");
    let name = create_string(state, name);
    table_set(state, table, "name", name);
    table
}

fn recipe_schematic_table(
    state: &mut LuaState,
    recipe: Option<&profession_data::RecipeEntry>,
) -> Val {
    let table = create_table(state);
    match recipe {
        Some(recipe) => populate_recipe_schematic_table(state, table, recipe),
        None => set_number_field(state, table, "recipeID", 0.0),
    }
    table
}

fn reagent_slot_schematic_table(
    state: &mut LuaState,
    recipe: &profession_data::RecipeEntry,
) -> Val {
    let table = create_table(state);
    for (index, reagent) in recipe.reagents.iter().enumerate() {
        let value = reagent_slot_table(state, index, reagent);
        set_table_array(state, table, (index + 1) as i64, value);
    }
    table
}

fn reagent_slot_table(
    state: &mut LuaState,
    index: usize,
    reagent: &profession_data::ReagentSlot,
) -> Val {
    let table = create_table(state);
    let reagents = reagent_entry_table(state, reagent);
    let variable_quantities = create_table(state);
    table_set(state, table, "reagents", reagents);
    table_set(state, table, "slotIndex", Val::Num((index + 1) as f64));
    table_set(state, table, "dataSlotIndex", Val::Num((index + 1) as f64));
    table_set(state, table, "reagentType", Val::Num(1.0));
    table_set(state, table, "required", Val::Bool(true));
    table_set(state, table, "hiddenInCraftingForm", Val::Bool(false));
    table_set(
        state,
        table,
        "quantityRequired",
        Val::Num(reagent.quantity as f64),
    );
    table_set(state, table, "variableQuantities", variable_quantities);
    table
}

fn reagent_entry_table(state: &mut LuaState, reagent: &profession_data::ReagentSlot) -> Val {
    let table = create_table(state);
    let reagent = reagent_info_table(state, Some(reagent));
    set_table_array(state, table, 1, reagent);
    table
}

fn profession_for_recipe(recipe_id: i32) -> Option<&'static profession_data::ProfessionInfo> {
    profession_data::get_recipe(recipe_id).and_then(|_| profession_data::get_profession_by_index(0))
}

fn register_namespace_methods(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
    methods: &[NamespaceMethod],
) -> LuaResult<()> {
    for &(name, func) in methods {
        table_set_rust_fn_static(state, table_ref, name, func)?;
    }
    Ok(())
}

fn populate_profession_table(
    state: &mut LuaState,
    table: Val,
    profession: &profession_data::ProfessionInfo,
) {
    set_number_field(state, table, "profession", profession.profession as f64);
    set_string_field(state, table, "professionName", profession.name);
    set_string_field(
        state,
        table,
        "parentProfessionName",
        profession.parent_profession_name,
    );
    set_number_field(state, table, "skillLevel", profession.skill_level as f64);
    set_number_field(
        state,
        table,
        "maxSkillLevel",
        profession.max_skill_level as f64,
    );
    set_number_field(
        state,
        table,
        "skillModifier",
        profession.skill_modifier as f64,
    );
    set_number_field(state, table, "skillLineID", profession.skill_line_id as f64);
    set_number_field(state, table, "iconFileID", profession.icon as f64);
}

fn populate_missing_recipe_info_table(state: &mut LuaState, table: Val) {
    set_number_field(state, table, "recipeID", 0.0);
    table_set(state, table, "name", Val::Nil);
    set_bool_field(state, table, "craftable", false);
}

fn populate_recipe_info_table(
    state: &mut LuaState,
    table: Val,
    recipe: &profession_data::RecipeEntry,
) {
    set_number_field(state, table, "recipeID", recipe.recipe_id as f64);
    set_string_field(state, table, "name", recipe.name);
    set_bool_field(state, table, "learned", recipe.learned);
    set_bool_field(state, table, "craftable", recipe.craftable);
    set_number_field(state, table, "difficulty", recipe.difficulty as f64);
    set_number_field(state, table, "categoryID", recipe.category_id as f64);
    set_number_field(state, table, "itemLevel", recipe.item_level as f64);
    set_number_field(state, table, "maxTrivialLevel", recipe.difficulty as f64);
    set_bool_field(state, table, "favorite", false);
}

fn populate_recipe_schematic_table(
    state: &mut LuaState,
    table: Val,
    recipe: &profession_data::RecipeEntry,
) {
    let reagent_slot_schematics = reagent_slot_schematic_table(state, recipe);
    set_number_field(state, table, "recipeID", recipe.recipe_id as f64);
    set_string_field(state, table, "name", recipe.name);
    if recipe.output_item_id == 0 {
        table_set(state, table, "outputItemID", Val::Nil);
    } else {
        set_number_field(state, table, "outputItemID", recipe.output_item_id as f64);
    }
    set_number_field(state, table, "quantityMin", recipe.output_quantity as f64);
    set_number_field(state, table, "quantityMax", recipe.output_quantity as f64);
    table_set(
        state,
        table,
        "reagentSlotSchematics",
        reagent_slot_schematics,
    );
}

fn set_number_field(state: &mut LuaState, table: Val, key: &str, value: f64) {
    table_set(state, table, key, Val::Num(value));
}

fn set_bool_field(state: &mut LuaState, table: Val, key: &str, value: bool) {
    table_set(state, table, key, Val::Bool(value));
}

fn set_string_field(state: &mut LuaState, table: Val, key: &str, value: &str) {
    let string = create_string(state, value);
    table_set(state, table, key, string);
}

fn selected_profession(state: &mut LuaState) -> Option<&'static profession_data::ProfessionInfo> {
    // SimState is the primary source; fall back to the Lua-side mirror, then to first profession.
    if let Ok(sim) = borrow_state(state) {
        if let Some(id) = sim.crafting.selected_profession_id {
            let prof = profession_data::get_profession(id)?;
            if is_profession_unlearned(state, prof.skill_line_id) {
                return None;
            }
            return Some(prof);
        }
    }
    let table_ref = ensure_namespace(state, TRADE_SKILL_NAMESPACE).ok()?;
    let selected = table_get(state, Val::Table(table_ref), SELECTED_PROFESSION_KEY);
    let Val::Num(skill_line_id) = selected else {
        return first_learned_profession(state);
    };
    let prof = profession_data::get_profession(skill_line_id as i32)?;
    if is_profession_unlearned(state, prof.skill_line_id) {
        return None;
    }
    Some(prof)
}

fn first_learned_profession(state: &LuaState) -> Option<&'static profession_data::ProfessionInfo> {
    profession_data::PROFESSIONS
        .iter()
        .find(|p| !is_profession_unlearned(state, p.skill_line_id))
}

fn profession_slots(profession: i32) -> &'static [i32] {
    match profession {
        BLACKSMITHING_PROFESSION => PROF0_INVENTORY_SLOTS,
        MINING_PROFESSION => PROF1_INVENTORY_SLOTS,
        COOKING_PROFESSION => COOKING_INVENTORY_SLOTS,
        FISHING_PROFESSION => FISHING_INVENTORY_SLOTS,
        _ => &[],
    }
}

fn profession_for_inventory_slot(slot: i32) -> Option<i32> {
    match slot {
        20..=22 => Some(BLACKSMITHING_PROFESSION),
        23..=25 => Some(MINING_PROFESSION),
        26 | 27 => Some(COOKING_PROFESSION),
        28 => Some(FISHING_PROFESSION),
        _ => None,
    }
}

fn reagent_index_from_stack(state: &mut LuaState) -> LuaResult<usize> {
    let index = i32::from_stack(state, 2)?;
    Ok(index.saturating_sub(1) as usize)
}

fn item_link_value(state: &mut LuaState, item_id: u32) -> Option<Val> {
    if item_id == 0 {
        return None;
    }

    let item = items::get_item(item_id)?;
    Some(create_string(
        state,
        &format!(
            "|cffffffff|Hitem:{item_id}::::::::80:::::|h[{}]|h|r",
            item.name
        ),
    ))
}

fn item_icon(item_id: u32) -> f64 {
    items::get_item(item_id)
        .map(|item| item.icon_file_data_id)
        .filter(|icon| *icon != 0)
        .unwrap_or(134400) as f64
}
