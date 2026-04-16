use super::ensure_namespace;
use crate::event::{Event, EventArg};
use crate::lua_api::rilua_methods::{borrow_state_mut, create_table};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_profession_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_TradeSkillUI")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetRecipesTracked",
        c_trade_skill_ui_get_recipes_tracked,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "SetRecipeTracked",
        c_trade_skill_ui_set_recipe_tracked,
    )?;
    Ok(())
}

fn c_trade_skill_ui_set_recipe_tracked(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = u32::from_stack(state, 1)?;
    let tracked = bool::from_stack(state, 2)?;
    let is_recrafting = bool::from_stack(state, 3)?;

    let mut sim = borrow_state_mut(state)?;
    let changed = sim.tracked_recipes.set(recipe_id, tracked, is_recrafting);
    if !changed {
        return Ok(0);
    }

    sim.events.push(Event {
        name: "TRACKED_RECIPE_UPDATE".to_string(),
        args: vec![
            EventArg::Number(recipe_id as f64),
            EventArg::Boolean(tracked),
        ],
    });
    Ok(0)
}

fn c_trade_skill_ui_get_recipes_tracked(state: &mut LuaState) -> LuaResult<u32> {
    let is_recrafting = bool::from_stack(state, 1)?;
    let recipe_ids = {
        let sim = borrow_state_mut(state)?;
        sim.tracked_recipes.list(is_recrafting).to_vec()
    };

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
