//! Current-profile item-to-mount lookup over explicitly supplied identities.
//! No journal-record, ownership or native catalog inference.

use super::helpers::{ensure_namespace, validate_supplied_relation_id};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_MountJournal")?;
    table_set_rust_fn_static(state, namespace, "GetMountFromItem", get_mount_from_item)
}

fn get_mount_from_item(state: &mut LuaState) -> LuaResult<u32> {
    let argument = rilua::table_security::unwrap_secret(state, stack_val(state, 1))?;
    let item_id = validate_supplied_relation_id(
        argument,
        "C_MountJournal.GetMountFromItem: itemID must be a number",
    )?;
    let mount_id = {
        let sim = borrow_state(state)?;
        item_id.and_then(|id| sim.mount_item_relations.get(&id).copied())
    };
    state.push(mount_id.map_or(Val::Nil, |id| Val::Num(f64::from(id))));
    Ok(1)
}
