//! `C_AccountStore` probe surface backed by `SimState` purchase / refund flags.
//!
//! `AccountStoreBaseCardMixin:SelectCard` calls `C_AccountStore.BeginPurchase`
//! and `C_AccountStore.RefundItem` from the confirmation popup `OnAccept`
//! handlers. The simulator only needs to record the requested item id and
//! report success or failure so tests can verify the UI wiring. Real purchase
//! and refund fulfillment is out of scope.

use super::ensure_namespace;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_account_store_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_AccountStore")?;
    table_set_rust_fn_static(state, table_ref, "BeginPurchase", begin_purchase)?;
    table_set_rust_fn_static(state, table_ref, "RefundItem", refund_item)?;
    Ok(())
}

fn begin_purchase(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i64::from_stack(state, 1)?;
    let succeeds = {
        let mut sim = borrow_state_mut(state)?;
        sim.last_account_store_purchase_request = Some(item_id);
        sim.account_store_begin_purchase_succeeds
    };
    state.push(Val::Bool(succeeds));
    Ok(1)
}

fn refund_item(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i64::from_stack(state, 1)?;
    let succeeds = {
        let mut sim = borrow_state_mut(state)?;
        sim.last_account_store_refund_request = Some(item_id);
        sim.account_store_refund_succeeds
    };
    state.push(Val::Bool(succeeds));
    Ok(1)
}
