//! `C_AccountStore` probe surface backed by `SimState` purchase / refund flags.
//!
//! `AccountStoreBaseCardMixin:SelectCard` calls `C_AccountStore.BeginPurchase`
//! and `C_AccountStore.RefundItem` from the confirmation popup `OnAccept`
//! handlers. The simulator only needs to record the requested item id and
//! report success or failure so tests can verify the UI wiring. Real purchase
//! and refund fulfillment is out of scope.

use super::ensure_namespace;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_set, table_set_num,
};
use crate::lua_api::state::{
    AccountStoreCategoryInfo, AccountStoreCurrencyInfo, AccountStoreItemInfo,
};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_account_store_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_AccountStore")?;
    table_set_rust_fn_static(state, table_ref, "BeginPurchase", begin_purchase)?;
    table_set_rust_fn_static(state, table_ref, "RefundItem", refund_item)?;
    table_set_rust_fn_static(state, table_ref, "GetCategories", get_categories)?;
    table_set_rust_fn_static(state, table_ref, "GetCategoryItems", get_category_items)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetCurrencyAvailable",
        get_currency_available,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetCurrencyIDForStore",
        get_currency_id_for_store,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetCategoryInfo", get_category_info)?;
    table_set_rust_fn_static(state, table_ref, "GetCurrencyInfo", get_currency_info)?;
    table_set_rust_fn_static(state, table_ref, "GetItemInfo", get_item_info)?;
    table_set_rust_fn_static(state, table_ref, "GetStoreFrontState", get_storefront_state)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "RequestStoreFrontInfoUpdate",
        request_storefront_info_update,
    )?;
    Ok(())
}

const ACCOUNT_STORE_STATE_AVAILABLE: f64 = 0.0;

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

fn get_categories(state: &mut LuaState) -> LuaResult<u32> {
    let mut category_ids = borrow_state(state)?
        .account_store_categories
        .keys()
        .copied()
        .collect::<Vec<_>>();
    category_ids.sort_unstable();

    let array = create_table(state);
    let Val::Table(array_ref) = array else {
        state.push(Val::Nil);
        return Ok(1);
    };
    for (index, category_id) in category_ids.iter().enumerate() {
        table_set_num(
            state,
            array_ref,
            (index + 1) as f64,
            Val::Num(*category_id as f64),
        );
    }
    state.push(array);
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

fn get_currency_available(state: &mut LuaState) -> LuaResult<u32> {
    let currency_id = i64::from_stack(state, 1)?;
    let amount = borrow_state(state)?
        .account_store_currency_info
        .get(&currency_id)
        .map(|info| info.amount)
        .unwrap_or_default();
    state.push(Val::Num(amount as f64));
    Ok(1)
}

fn get_category_info(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i64::from_stack(state, 1)?;
    let info = borrow_state(state)?
        .account_store_categories
        .get(&category_id)
        .cloned();
    let Some(info) = info else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let table = create_table(state);
    populate_category_info_table(state, table, &info);
    state.push(table);
    Ok(1)
}

fn get_currency_info(state: &mut LuaState) -> LuaResult<u32> {
    let currency_id = i64::from_stack(state, 1)?;
    let info = borrow_state(state)?
        .account_store_currency_info
        .get(&currency_id)
        .cloned();
    let Some(info) = info else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let table = create_table(state);
    populate_currency_info_table(state, table, &info);
    state.push(table);
    Ok(1)
}

fn get_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i64::from_stack(state, 1)?;
    let info = borrow_state(state)?
        .account_store_items
        .get(&item_id)
        .cloned();
    let Some(info) = info else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let table = create_table(state);
    populate_item_info_table(state, table, &info);
    state.push(table);
    Ok(1)
}

fn get_storefront_state(state: &mut LuaState) -> LuaResult<u32> {
    let store_front_id = i64::from_stack(state, 1)?;
    let raw_state = borrow_state(state)?
        .account_store_storefront_state
        .get(&store_front_id)
        .copied();
    let state_value = raw_state
        .map(|v| v as f64)
        .unwrap_or(ACCOUNT_STORE_STATE_AVAILABLE);
    state.push(Val::Num(state_value));
    Ok(1)
}

fn request_storefront_info_update(state: &mut LuaState) -> LuaResult<u32> {
    let store_front_id = i64::from_stack(state, 1)?;
    borrow_state_mut(state)?.last_account_store_storefront_info_request = Some(store_front_id);
    Ok(0)
}

fn populate_category_info_table(state: &mut LuaState, table: Val, info: &AccountStoreCategoryInfo) {
    table_set(state, table, "id", Val::Num(info.id as f64));
    let name_val = create_string(state, &info.name);
    table_set(state, table, "name", name_val);
    table_set(state, table, "type", Val::Num(info.category_type as f64));
    table_set(state, table, "icon", Val::Num(info.icon as f64));
}

fn populate_item_info_table(state: &mut LuaState, table: Val, info: &AccountStoreItemInfo) {
    table_set(state, table, "id", Val::Num(info.id as f64));
    table_set(state, table, "status", Val::Num(info.status as f64));
    table_set(state, table, "mode", Val::Num(info.mode as f64));
    table_set(
        state,
        table,
        "currencyID",
        Val::Num(info.currency_id as f64),
    );
    table_set(state, table, "flags", Val::Num(info.flags as f64));
    let name_val = create_string(state, &info.name);
    table_set(state, table, "name", name_val);
    table_set(state, table, "price", Val::Num(info.price as f64));
    table_set(state, table, "nonrefundable", Val::Bool(info.nonrefundable));
    populate_item_info_optional_fields(state, table, info);
}

fn populate_item_info_optional_fields(
    state: &mut LuaState,
    table: Val,
    info: &AccountStoreItemInfo,
) {
    if let Some(scene_id) = info.custom_ui_model_scene_id {
        table_set(
            state,
            table,
            "customUIModelSceneID",
            Val::Num(scene_id as f64),
        );
    }
    if let Some(description) = &info.description {
        let description_val = create_string(state, description);
        table_set(state, table, "description", description_val);
    }
    if let Some(creature_id) = info.creature_display_id {
        table_set(
            state,
            table,
            "creatureDisplayID",
            Val::Num(creature_id as f64),
        );
    }
    if let Some(transmog_id) = info.transmog_set_id {
        table_set(state, table, "transmogSetID", Val::Num(transmog_id as f64));
    }
    if let Some(icon) = info.display_icon {
        table_set(state, table, "displayIcon", Val::Num(icon as f64));
    }
    if let Some(seconds) = info.refund_seconds_remaining {
        table_set(
            state,
            table,
            "refundSecondsRemaining",
            Val::Num(seconds as f64),
        );
    }
}

fn populate_currency_info_table(state: &mut LuaState, table: Val, info: &AccountStoreCurrencyInfo) {
    table_set(state, table, "id", Val::Num(info.id as f64));
    table_set(state, table, "amount", Val::Num(info.amount as f64));
    if let Some(max) = info.max_quantity {
        table_set(state, table, "maxQuantity", Val::Num(max as f64));
    }
    let name_val = create_string(state, &info.name);
    table_set(state, table, "name", name_val);
    table_set(state, table, "icon", Val::Num(info.icon as f64));
}
