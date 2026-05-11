use super::categories::{collect_category_achievement_ids, find_category};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn get_category_achievement_points(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i32::from_stack(state, 1)?;
    let include_subcategories = bool::from_stack(state, 2).unwrap_or(false);
    let achievement_ids = achievement_ids_for_point_total(category_id, include_subcategories);
    let total_points = earned_achievement_points(state, &achievement_ids)?;
    state.push(Val::Num(total_points as f64));
    Ok(1)
}

fn achievement_ids_for_point_total(category_id: i32, include_subcategories: bool) -> Vec<i32> {
    if include_subcategories {
        return collect_category_achievement_ids(category_id);
    }

    find_category(category_id)
        .map(|category| category.achievement_ids.to_vec())
        .unwrap_or_default()
}

fn earned_achievement_points(state: &mut LuaState, achievement_ids: &[i32]) -> LuaResult<i32> {
    let sim = borrow_state(state)?;
    Ok(achievement_ids
        .iter()
        .filter(|achievement_id| sim.world.earned_achievements.contains(achievement_id))
        .filter_map(|achievement_id| sim.achievements.get(achievement_id))
        .map(|info| info.points)
        .sum())
}
