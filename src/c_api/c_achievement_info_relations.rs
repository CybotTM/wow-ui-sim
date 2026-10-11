//! Achievement relations projected only from explicitly supplied simulator state.
use super::helpers::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_table};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_AchievementInfo")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetSupercedingAchievements",
        query_superceding_achievements,
    )
}

fn read_achievement_id(state: &LuaState) -> LuaResult<Option<i32>> {
    let value = stack_val(state, 1);
    // AllowedWhenUntainted: authenticate before validation, without changing taint.
    #[cfg(feature = "retail-12-1-0")]
    let value = rilua::table_security::unwrap_secret(state, value)?;
    let Val::Num(number) = value else {
        return Err(rilua::runtime_error(
            "C_AchievementInfo.GetSupercedingAchievements: achievementID must be a number",
        ));
    };
    let in_range = (f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&number);
    let represented = number.is_finite() && number.fract() == 0.0 && in_range;
    Ok(represented.then(|| number as i32))
}

fn query_superceding_achievements(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = read_achievement_id(state)?;
    let relations = {
        let sim = borrow_state(state)?;
        achievement_id
            .and_then(|id| sim.achievement_superceding.get(&id))
            .cloned()
            .unwrap_or_default()
    };
    let result = create_table(state);
    state.push(result);
    for (index, id) in relations.into_iter().enumerate() {
        set_table_array(state, result, index as i64 + 1, Val::Num(f64::from(id)));
    }
    Ok(1)
}
