//! `C_AccountStore` probe surface backed by `SimState` purchase / refund flags.
//!
//! `AccountStoreBaseCardMixin:SelectCard` calls `C_AccountStore.BeginPurchase`
//! and `C_AccountStore.RefundItem` from the confirmation popup `OnAccept`
//! handlers. The simulator only needs to record the requested item id and
//! report success or failure so tests can verify the UI wiring. Real purchase
//! and refund fulfillment is out of scope.

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_table, table_set_num};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_account_store_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_AccountStore")?;
    table_set_rust_fn_static(state, table_ref, "BeginPurchase", begin_purchase)?;
    table_set_rust_fn_static(state, table_ref, "RefundItem", refund_item)?;
    table_set_rust_fn_static(state, table_ref, "GetCategoryItems", get_category_items)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetCurrencyIDForStore",
        get_currency_id_for_store,
    )?;
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

fn get_category_items(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i64::from_stack(state, 1)?;
    let item_ids = borrow_state(state)?
        .account_store_category_items
        .get(&category_id)
        .cloned()
        .unwrap_or_default();

    let array = create_table(state);
    let Val::Table(array_ref) = array else {
        state.push(Val::Nil);
        return Ok(1);
    };
    for (index, item_id) in item_ids.iter().enumerate() {
        table_set_num(
            state,
            array_ref,
            (index + 1) as f64,
            Val::Num(*item_id as f64),
        );
    }
    state.push(array);
    Ok(1)
}

fn get_currency_id_for_store(state: &mut LuaState) -> LuaResult<u32> {
    let store_front_id = i64::from_stack(state, 1)?;
    let currency_id = borrow_state(state)?
        .account_store_currency_for_store
        .get(&store_front_id)
        .copied();
    match currency_id {
        Some(id) => state.push(Val::Num(id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}
