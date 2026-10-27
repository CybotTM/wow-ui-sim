//! Supplied minimap quest-blob containment observations.

use crate::c_api::helpers::{ensure_namespace, validate_supplied_relation_id};
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::{FromStack, TableBuilder, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    ) {
        let namespace = ensure_namespace(state, "C_Minimap")?;
        table_set_rust_fn_static(state, namespace, "IsInsideQuestBlob", query_inside)?;
    }
    Ok(())
}

fn query_inside(state: &mut LuaState) -> LuaResult<u32> {
    let value = rilua::table_security::unwrap_secret(state, stack_val(state, 1))?;
    let quest_id =
        validate_supplied_relation_id(value, "IsInsideQuestBlob requires a questID number")?;
    let sim = borrow_state(state)?;
    let inside = quest_id
        .and_then(|id| sim.minimap_quest_blob_inside.get(&id).copied())
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(inside));
    Ok(1)
}

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
