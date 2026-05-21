//! C_EquipmentSet temporary item-lock shim.
//!
//! Equipment set contents are state-backed, but transient item lock state from
//! inventory/server operations is not modeled yet. Keep the inert lock answer
//! isolated until bag/item lock simulation exists.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_equipment_set_lock_state(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_EquipmentSet")?;
    table_set_rust_fn_static(
        state,
        ns,
        "EquipmentSetContainsLockedItems",
        equipment_set_contains_locked_items,
    )?;
    Ok(())
}

fn equipment_set_contains_locked_items(state: &mut LuaState) -> LuaResult<u32> {
    let _set_id = i32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}
