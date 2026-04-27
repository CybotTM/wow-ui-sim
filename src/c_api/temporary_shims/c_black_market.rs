//! C_BlackMarket temporary shim — black market auction state is not modeled.
//!
//! The Black Market UI loads against an empty auction list: `IsViewOnly`
//! returns false, `GetNumItems` returns 0, and the item-info getters
//! return nothing. Real bidding state would replace this surface.

use crate::c_api::ensure_global_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn register_c_black_market(state: &mut LuaState) -> LuaResult<()> {
    let t = ensure_global_table(state, "C_BlackMarket");
    let Val::Table(t_ref) = t else {
        unreachable!("C_BlackMarket must be a table");
    };
    table_set_rust_fn_static(state, t_ref, "Close", noop)?;
    table_set_rust_fn_static(state, t_ref, "RequestItems", noop)?;
    table_set_rust_fn_static(state, t_ref, "ItemPlaceBid", noop)?;
    table_set_rust_fn_static(state, t_ref, "IsViewOnly", is_view_only)?;
    table_set_rust_fn_static(state, t_ref, "GetNumItems", get_num_items)?;
    table_set_rust_fn_static(state, t_ref, "GetHotItem", empty_item_info)?;
    table_set_rust_fn_static(state, t_ref, "GetItemInfoByID", empty_item_info)?;
    table_set_rust_fn_static(state, t_ref, "GetItemInfoByIndex", empty_item_info)?;
    Ok(())
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn is_view_only(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn get_num_items(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn empty_item_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
