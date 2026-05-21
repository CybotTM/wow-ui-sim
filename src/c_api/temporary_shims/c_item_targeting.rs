//! C_Item temporary item-targeting shims.
//!
//! Helpful/harmful item targeting depends on cursor/targeting state that the
//! simulator does not model yet. Keep the default non-targeting answers here
//! instead of mixing them into the state-backed item metadata surface.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_item_targeting_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Item")?;
    table_set_rust_fn_static(state, ns, "IsHelpfulItem", is_helpful_item)?;
    table_set_rust_fn_static(state, ns, "IsHarmfulItem", is_harmful_item)?;
    Ok(())
}

fn is_helpful_item(state: &mut LuaState) -> LuaResult<u32> {
    let _item = stack_val(state, 1);
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_harmful_item(state: &mut LuaState) -> LuaResult<u32> {
    let _item = stack_val(state, 1);
    state.push(Val::Bool(false));
    Ok(1)
}
