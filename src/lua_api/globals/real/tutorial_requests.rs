//! Legacy tutorial request recording, without tutorial event/flag/display policy.
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    if matches!(
        ACTIVE,
        ClientProfile::Mists
            | ClientProfile::Era
            | ClientProfile::Anniversary
            | ClientProfile::Wrath
    ) {
        LuaApiMut::register_function(lua, "TriggerTutorial", record_tutorial_request)?;
    }
    Ok(())
}

fn record_tutorial_request(state: &mut LuaState) -> LuaResult<u32> {
    let tutorial_id = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?.tutorial_requests.push(tutorial_id);
    Ok(0)
}
