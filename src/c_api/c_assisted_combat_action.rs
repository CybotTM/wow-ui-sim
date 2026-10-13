//! Assisted action identity projects its own input, never the recommendation.

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn query_action_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = borrow_state(state)?.assisted_combat.action_spell_id;
    state.push(spell_id.map_or(Val::Nil, |id| Val::Num(f64::from(id))));
    Ok(1)
}
