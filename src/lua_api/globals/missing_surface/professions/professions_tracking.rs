use super::set_table_array;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_table};
use crate::lua_api::script_helpers::fire_named_event_state;
use crate::lua_api::state_types::CraftingState;
use crate::lua_api::tracked_recipes::TrackedRecipes;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::collections::HashSet;

pub(super) fn c_trade_skill_ui_set_recipe_tracked(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn c_trade_skill_ui_is_recipe_tracked(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = u32::from_stack(state, 1)?;
    let is_recrafting = bool::from_stack(state, 2)?;

    let tracked = {
        let sim = borrow_state(state)?;
        is_recipe_tracked(&sim.tracked_recipes, recipe_id, is_recrafting)
    };

    state.push(Val::Bool(tracked));
    Ok(1)
}

pub(super) fn c_trade_skill_ui_get_recipes_tracked(state: &mut LuaState) -> LuaResult<u32> {
    let is_recrafting = bool::from_stack(state, 1)?;
    let recipe_ids = borrow_state(state)?
        .tracked_recipes
        .list(is_recrafting)
        .to_vec();
    let table = recipe_ids_table(state, &recipe_ids);

    state.push(table);
    Ok(1)
}

fn recipe_ids_table(state: &mut LuaState, recipe_ids: &[u32]) -> Val {
    let table = create_table(state);
    for (index, recipe_id) in recipe_ids.iter().enumerate() {
        let value = Val::Num(*recipe_id as f64);
        set_table_array(state, table, (index + 1) as i64, value);
    }
    table
}

pub(super) fn c_trade_skill_ui_is_recipe_learned(state: &mut LuaState) -> LuaResult<u32> {
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
