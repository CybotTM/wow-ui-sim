//! Supplied minimap quest-blob containment observations.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::{FromStack, TableBuilder, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

pub(crate) fn register_admin(builder: TableBuilder) -> LuaResult<TableBuilder> {
    if matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    ) {
        builder.set_function("SetMinimapQuestBlobInside", set_inside)
    } else {
        Ok(builder)
    }
}

fn set_inside(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let Val::Bool(inside) = stack_val(state, 2) else {
        return Err(runtime_error(
            "SetMinimapQuestBlobInside requires a boolean",
        ));
    };
    borrow_state_mut(state)?
        .minimap_quest_blob_inside
        .insert(quest_id, inside);
    Ok(0)
}
