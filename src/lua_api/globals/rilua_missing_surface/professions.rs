use super::ensure_namespace;
use crate::event::{Event, EventArg};
use crate::lua_api::rilua_methods::borrow_state_mut;
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(super) fn register_profession_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_TradeSkillUI")?;
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
