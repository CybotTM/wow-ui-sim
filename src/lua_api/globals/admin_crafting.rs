//! Rilua A_Admin handlers — Crafting / profession seeding.
//!
//! Provides `LearnRecipe`, `UnlearnRecipe`, `ClearKnownRecipes`, and
//! `SetSelectedProfession` for test-side setup of the crafting surface.

use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn learn_recipe(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?
        .crafting
        .known_recipe_ids
        .insert(recipe_id);
    Ok(0)
}

pub(super) fn unlearn_recipe(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?
        .crafting
        .known_recipe_ids
        .remove(&recipe_id);
    Ok(0)
}

pub(super) fn clear_known_recipes(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.crafting.known_recipe_ids.clear();
    Ok(0)
}

pub(super) fn set_selected_profession(state: &mut LuaState) -> LuaResult<u32> {
    let val = Val::from_stack(state, 1)?;
    let id = match val {
        Val::Nil => None,
        Val::Num(n) => Some(n as i32),
        _ => None,
    };
    borrow_state_mut(state)?.crafting.selected_profession_id = id;
    Ok(0)
}
