//! Supplied GUID classification intersected with the live active party.
//! Player inclusion follows the existing group policy, not native AI proof.

use super::unit_roles::represented_identity;
use crate::lua_api::methods::{borrow_state, val_to_string};
use crate::lua_bridge::stack_val;
use rilua::table_security::unwrap_secret;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

fn read_nullable_unit(state: &LuaState) -> LuaResult<Option<String>> {
    // Cached AllowedWhenUntainted: authenticate the original value before type
    // validation, without replacing its wrapper or changing caller taint.
    let value = unwrap_secret(state, stack_val(state, 1))?;
    match value {
        Val::Nil => Ok(None),
        Val::Str(_) => val_to_string(state, value)
            .map(Some)
            .ok_or_else(|| runtime_error("UnitInPartyIsAI: unit must be a UTF-8 string")),
        _ => Err(runtime_error(
            "UnitInPartyIsAI: unit must be a string or nil",
        )),
    }
}

pub(crate) fn unit_in_party_is_ai(state: &mut LuaState) -> LuaResult<u32> {
    let unit = read_nullable_unit(state)?;
    let is_ai = {
        let sim = borrow_state(state)?;
        sim.party_group_active
            && unit
                .as_deref()
                .and_then(|unit| represented_identity(&sim, unit))
                .is_some_and(|(guid, _)| sim.party_ai_guids.contains(&guid))
    };
    state.push(Val::Bool(is_ai));
    Ok(1)
}
