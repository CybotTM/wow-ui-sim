//! Wrath legacy RAF predicate over explicit represented-unit link state.
//! No links are seeded; this is simulator input policy, not native solo proof.

use crate::lua_api::globals::unit_misc::existing_guid_for_unit;
use crate::lua_api::methods::{borrow_state, val_to_string};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

fn is_refer_a_friend_linked(state: &mut LuaState) -> LuaResult<u32> {
    let value = stack_val(state, 1);
    if !matches!(value, Val::Str(_)) {
        return Err(runtime_error(
            "IsReferAFriendLinked expects a unit token string",
        ));
    }
    let unit = val_to_string(state, value)
        .ok_or_else(|| runtime_error("IsReferAFriendLinked expects a UTF-8 unit token"))?;
    let linked = {
        let sim = borrow_state(state)?;
        existing_guid_for_unit(&sim, &unit)
            .is_some_and(|guid| sim.refer_a_friend_linked_guids.contains(&guid))
    };
    state.push(Val::Bool(linked));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsReferAFriendLinked", is_refer_a_friend_linked)?;
    Ok(())
}
