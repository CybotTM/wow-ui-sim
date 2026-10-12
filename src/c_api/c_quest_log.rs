//! Quest eligibility projected from explicitly supplied simulator state.

use super::helpers::{ensure_namespace, validate_supplied_relation_id};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_QuestLog")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "QuestCanHaveWarModeBonus",
        quest_can_have_war_mode_bonus,
    )
}

fn quest_can_have_war_mode_bonus(state: &mut LuaState) -> LuaResult<u32> {
    let value = stack_val(state, 1);
    #[cfg(feature = "retail-12-1-0")]
    let value = rilua::table_security::unwrap_secret(state, value)?;
    let quest_id = validate_supplied_relation_id(
        value,
        "C_QuestLog.QuestCanHaveWarModeBonus: questID must be a number",
    )?;
    let eligible = {
        let sim = borrow_state(state)?;
        quest_id.is_some_and(|id| sim.quest_war_mode_eligibility.contains(&id))
    };
    state.push(Val::Bool(eligible));
    Ok(1)
}
