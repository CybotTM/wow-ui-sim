//! C_ItemUpgrade temporary availability shim.
//!
//! Upgrade-cost and upgrade-eligibility state is not modeled yet. The selected
//! item-upgrade location is real simulator state used by tooltip probes, but
//! `CanUpgradeItem` remains an inert compatibility answer until upgrade data is
//! seeded.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_item_upgrade_availability(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ItemUpgrade")?;
    table_set_rust_fn_static(state, ns, "CanUpgradeItem", can_upgrade_item)?;
    Ok(())
}

fn can_upgrade_item(state: &mut LuaState) -> LuaResult<u32> {
    let _location = stack_val(state, 1);
    state.push(Val::Bool(false));
    Ok(1)
}
