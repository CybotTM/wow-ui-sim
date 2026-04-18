//! Crafting helpers for `professions.rs`.
//!
//! Extracted to keep `professions.rs` under 750 lines.
//! Provides `recipe_is_craftable` and `craft_recipe`, called by the
//! `C_TradeSkillUI.IsRecipeCraftable` and `C_TradeSkillUI.CraftRecipe`
//! dispatchers in `professions.rs`.

use crate::lua_api::globals::profession_data;
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use rilua::vm::state::LuaState;

/// Returns true iff the recipe exists in the catalogue AND all reagents are
/// available in `player.bag_items` for the requested `count`.
pub(super) fn recipe_is_craftable(state: &mut LuaState, recipe_id: i32, count: i32) -> bool {
    let Some(recipe) = profession_data::get_recipe(recipe_id) else {
        return false;
    };
    let Ok(sim) = borrow_state(state) else {
        return false;
    };
    for reagent in recipe.reagents {
        let needed = reagent.quantity * count;
        let have: i32 = sim
            .bag_items
            .values()
            .filter(|b| b.item_id == reagent.item_id)
            .map(|b| b.stack_count)
            .sum();
        if have < needed {
            return false;
        }
    }
    true
}

/// Consumes reagents from bags and adds output item if all reagents are available.
/// Returns false and changes nothing when reagents are insufficient.
pub(super) fn craft_recipe(state: &mut LuaState, recipe_id: i32, count: i32) -> bool {
    if !recipe_is_craftable(state, recipe_id, count) {
        return false;
    }
    let Some(recipe) = profession_data::get_recipe(recipe_id) else {
        return false;
    };

    // Collect reagent changes before mutating.
    let reagent_deltas: Vec<(u32, i32)> = recipe
        .reagents
        .iter()
        .map(|r| (r.item_id, r.quantity * count))
        .collect();
    let output_id = recipe.output_item_id;

    let Ok(mut sim) = borrow_state_mut(state) else {
        return false;
    };

    // Subtract each reagent from matching bag slots.
    for (item_id, mut needed) in reagent_deltas {
        for slot in sim.bag_items.values_mut() {
            if slot.item_id != item_id || needed == 0 {
                continue;
            }
            let take = needed.min(slot.stack_count);
            slot.stack_count -= take;
            needed -= take;
        }
    }
    // Remove depleted bag slots.
    sim.bag_items.retain(|_, b| b.stack_count > 0);

    // Add output item: increment an existing slot or pick a free bag-0 slot.
    if let Some(slot) = sim.bag_items.values_mut().find(|b| b.item_id == output_id) {
        slot.stack_count += count;
    } else {
        let key = free_bag0_slot(&sim.bag_items);
        sim.bag_items.insert(
            key,
            crate::lua_api::state::BagItem {
                item_id: output_id,
                stack_count: count,
            },
        );
    }

    true
}

/// Find a free slot in bag 0 (slots 1–16). Falls back to negative slots if
/// all 16 are occupied (tests rarely exceed that).
fn free_bag0_slot(
    bag_items: &std::collections::HashMap<(i32, i32), crate::lua_api::state::BagItem>,
) -> (i32, i32) {
    for slot in 1..=16_i32 {
        let k = (0, slot);
        if !bag_items.contains_key(&k) {
            return k;
        }
    }
    // overflow safety: negative slots are never valid WoW slots
    for slot in (-1000..=-1_i32).rev() {
        let k = (0, slot);
        if !bag_items.contains_key(&k) {
            return k;
        }
    }
    unreachable!("bag 0 exhausted")
}
