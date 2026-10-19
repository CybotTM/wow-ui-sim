//! Temporary Wrath occupied-action policy: spell effect-type data is missing.
//! Retire when generated first spell effects can be compared with effect 78.

use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn query_non_attack(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}
