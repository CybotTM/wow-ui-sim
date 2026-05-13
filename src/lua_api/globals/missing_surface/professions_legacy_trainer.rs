use crate::lua_api::globals::profession_data;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_api::script_helpers::fire_named_event_state;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::professions_tables::{item_icon, selected_profession};

type TrainerGlobal = (&'static str, fn(&mut LuaState) -> LuaResult<u32>);

pub(super) fn register_legacy_trainer_globals(state: &mut LuaState) -> LuaResult<()> {
    for (name, function) in LEGACY_TRAINER_GLOBALS {
        table_set_rust_fn_static(state, state.global, name, *function)?;
    }
    Ok(())
}

const LEGACY_TRAINER_GLOBALS: &[TrainerGlobal] = &[
    ("BuyTrainerService", buy_trainer_service),
    ("CloseTrainer", close_trainer),
    ("CollapseTrainerSkillLine", collapse_trainer_skill_line),
    ("ExpandTrainerSkillLine", expand_trainer_skill_line),
    ("GetNumPrimaryProfessions", get_num_primary_professions),
    ("GetNumTrainerServices", get_num_trainer_services),
    ("GetTrainerGreetingText", get_trainer_greeting_text),
    ("GetTrainerSelectionIndex", get_trainer_selection_index),
    (
        "GetTrainerServiceAbilityReq",
        get_trainer_service_ability_req,
    ),
    ("GetTrainerServiceCost", get_trainer_service_cost),
    (
        "GetTrainerServiceDescription",
        get_trainer_service_description,
    ),
    ("GetTrainerServiceIcon", get_trainer_service_icon),
    ("GetTrainerServiceInfo", get_trainer_service_info),
    ("GetTrainerServiceItemLink", get_trainer_service_item_link),
    ("GetTrainerServiceLevelReq", get_trainer_service_level_req),
    (
        "GetTrainerServiceNumAbilityReq",
        get_trainer_service_num_ability_req,
    ),
    ("GetTrainerServiceSkillLine", get_trainer_service_skill_line),
    ("GetTrainerServiceSkillReq", get_trainer_service_skill_req),
    (
        "GetTrainerServiceTypeFilter",
        get_trainer_service_type_filter,
    ),
    ("IsTradeskillTrainer", is_tradeskill_trainer),
    ("IsTrainerServiceLearnSpell", is_trainer_service_learn_spell),
    ("SelectTrainerService", select_trainer_service),
    (
        "SetTrainerServiceTypeFilter",
        set_trainer_service_type_filter,
    ),
    ("UnitCharacterPoints", unit_character_points),
];

fn get_num_trainer_services(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num((trainer_recipes().len() + 1) as f64));
    Ok(1)
}

fn get_trainer_service_info(state: &mut LuaState) -> LuaResult<u32> {
    let service_index = i32::from_stack(state, 1)?;
    if service_index == 1 {
        push_service_info(state, "Blacksmithing", "", "header", true);
        return Ok(4);
    }

    let recipe = trainer_recipe(service_index);
    let name = recipe
        .map(|recipe| create_string(state, recipe.name))
        .unwrap_or(Val::Nil);
    let sub_text = create_string(state, "Blacksmithing");
    let service_type = trainer_service_type(state, recipe);

    state.push(name);
    state.push(sub_text);
    state.push(service_type);
    state.push(Val::Bool(true));
    Ok(4)
}

fn push_service_info(
    state: &mut LuaState,
    name: &str,
    sub_text: &str,
    service_type: &str,
    expanded: bool,
) {
    let name = create_string(state, name);
    let sub_text = create_string(state, sub_text);
    let service_type = create_string(state, service_type);
    state.push(name);
    state.push(sub_text);
    state.push(service_type);
    state.push(Val::Bool(expanded));
}

fn select_trainer_service(state: &mut LuaState) -> LuaResult<u32> {
    let service_index = i32::from_stack(state, 1)?;
    if trainer_recipe(service_index).is_some() {
        borrow_state_mut(state)?
            .crafting
            .selected_trainer_service_index = Some(service_index);
    }
    Ok(0)
}

fn get_trainer_selection_index(state: &mut LuaState) -> LuaResult<u32> {
    let selected = borrow_state(state)?
        .crafting
        .selected_trainer_service_index
        .unwrap_or(0);
    state.push(Val::Num(selected as f64));
    Ok(1)
}

fn get_trainer_service_icon(state: &mut LuaState) -> LuaResult<u32> {
    let service_index = i32::from_stack(state, 1)?;
    let icon = trainer_recipe(service_index)
        .map(|recipe| item_icon(recipe.output_item_id))
        .unwrap_or(134400.0);
    state.push(Val::Num(icon));
    Ok(1)
}

fn get_trainer_service_description(state: &mut LuaState) -> LuaResult<u32> {
    let service_index = i32::from_stack(state, 1)?;
    let description = trainer_recipe(service_index)
        .and_then(|recipe| u32::try_from(recipe.recipe_id).ok())
        .and_then(crate::spell_descriptions::get_spell_description)
        .filter(|description| !description.is_empty())
        .unwrap_or("Learn this blacksmithing recipe.");
    let description = create_string(state, description);
    state.push(description);
    Ok(1)
}

fn get_trainer_service_cost(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    Ok(2)
}

fn get_trainer_service_level_req(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(1.0));
    Ok(1)
}

fn get_trainer_service_skill_req(state: &mut LuaState) -> LuaResult<u32> {
    let service_index = i32::from_stack(state, 1)?;
    let recipe = trainer_recipe(service_index);
    if recipe.is_none() {
        state.push(Val::Nil);
        return Ok(1);
    }

    let skill = create_string(state, "Blacksmithing");
    state.push(skill);
    state.push(Val::Num(1.0));
    state.push(Val::Bool(true));
    Ok(3)
}

fn get_trainer_service_num_ability_req(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_trainer_service_ability_req(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Bool(true));
    Ok(2)
}

fn get_trainer_service_item_link(state: &mut LuaState) -> LuaResult<u32> {
    let service_index = i32::from_stack(state, 1)?;
    let link = trainer_recipe(service_index).map(|recipe| {
        format!(
            "|cff71d5ff|Henchant:{}|h[{}]|h|r",
            recipe.recipe_id, recipe.name
        )
    });
    match link {
        Some(link) => {
            let link = create_string(state, &link);
            state.push(link);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_trainer_service_skill_line(state: &mut LuaState) -> LuaResult<u32> {
    let profession =
        selected_profession(state).or_else(|| profession_data::get_profession_by_index(0));
    let name = profession
        .map(|profession| create_string(state, profession.name))
        .unwrap_or_else(|| create_string(state, "Blacksmithing"));
    state.push(name);
    Ok(1)
}

fn get_trainer_greeting_text(state: &mut LuaState) -> LuaResult<u32> {
    let greeting = create_string(state, "I can train you in blacksmithing.");
    state.push(greeting);
    Ok(1)
}

fn get_trainer_service_type_filter(state: &mut LuaState) -> LuaResult<u32> {
    let filter = String::from_stack(state, 1).unwrap_or_default();
    state.push(Val::Bool(filter != "used"));
    Ok(1)
}

fn set_trainer_service_type_filter(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn is_tradeskill_trainer(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_trainer_service_learn_spell(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    state.push(Val::Bool(false));
    Ok(2)
}

fn get_num_primary_professions(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(2.0));
    Ok(1)
}

fn unit_character_points(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(2)
}

fn buy_trainer_service(state: &mut LuaState) -> LuaResult<u32> {
    let service_index = i32::from_stack(state, 1)?;
    if let Some(recipe) = trainer_recipe(service_index) {
        borrow_state_mut(state)?
            .crafting
            .known_recipe_ids
            .insert(recipe.recipe_id);
        fire_named_event_state(state, "TRAINER_UPDATE", &[]);
    }
    Ok(0)
}

fn close_trainer(state: &mut LuaState) -> LuaResult<u32> {
    fire_named_event_state(state, "TRAINER_CLOSED", &[]);
    Ok(0)
}

fn expand_trainer_skill_line(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn collapse_trainer_skill_line(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn trainer_recipes() -> &'static [profession_data::RecipeEntry] {
    profession_data::BLACKSMITHING_RECIPES
}

fn trainer_recipe(service_index: i32) -> Option<&'static profession_data::RecipeEntry> {
    usize::try_from(service_index.saturating_sub(2))
        .ok()
        .and_then(|index| trainer_recipes().get(index))
}

fn trainer_service_type(
    state: &mut LuaState,
    recipe: Option<&profession_data::RecipeEntry>,
) -> Val {
    let Some(recipe) = recipe else {
        return Val::Nil;
    };
    let known = borrow_state(state)
        .map(|sim| sim.crafting.known_recipe_ids.contains(&recipe.recipe_id))
        .unwrap_or(false);
    let service_type = if known { "used" } else { "available" };
    create_string(state, service_type)
}
