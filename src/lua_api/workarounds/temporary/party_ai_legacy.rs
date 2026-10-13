//! Earlier-profile party AI is unmodeled; preserve the existing false query.
//! Retire when those profiles receive an explicit party AI model and policy.

use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn unit_in_party_is_ai(state: &mut LuaState) -> LuaResult<u32> {
    let _ = Option::<String>::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}
