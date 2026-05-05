use crate::c_api::ensure_namespace;
use crate::lua_api::globals::currency_data;
use crate::lua_api::methods::{
    borrow_state, create_string, create_table, table_set, val_to_string,
};
use crate::lua_api::state::CurrencyInfo;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const CURRENCY_INFO_METHODS: &[(&str, rilua::RustFn)] = &[
    ("GetCurrencyListSize", c_currency_get_list_size),
    ("GetCurrencyListInfo", c_currency_get_list_info),
    (
        "GetBackpackCurrencyInfo",
        c_currency_get_backpack_currency_info,
    ),
    ("GetCoinTextureString", c_currency_get_coin_texture_string),
    ("GetCurrencyInfo", c_currency_get_currency_info),
    (
        "GetCurrencyInfoFromLink",
        c_currency_get_currency_info_from_link,
    ),
    (
        "GetCurrencyContainerInfo",
        c_currency_get_currency_container_info,
    ),
];

pub(super) fn register_c_currency_info(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_CurrencyInfo")?;
    for &(name, func) in CURRENCY_INFO_METHODS {
        table_set_rust_fn_static(state, table_ref, name, func)?;
    }
    Ok(())
}

fn c_currency_get_list_size(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(currency_data::currency_list_size() as f64));
    Ok(1)
}

fn c_currency_get_list_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let Some(entry) = currency_data::get_currency_list_entry(index) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    let name = create_string(state, entry.name);
    table_set(
        state,
        info,
        "currencyTypesID",
        Val::Num(entry.currency_id as f64),
    );
    table_set(state, info, "name", name);
    table_set(state, info, "quantity", Val::Num(entry.quantity as f64));
    table_set(
        state,
        info,
        "iconFileID",
        Val::Num(entry.icon_file_id as f64),
    );
    table_set(state, info, "isHeader", Val::Bool(entry.is_header));
    table_set(
        state,
        info,
        "isHeaderExpanded",
        Val::Bool(entry.is_header_expanded),
    );
    table_set(state, info, "quality", Val::Num(entry.quality as f64));
    state.push(info);
    Ok(1)
}

fn c_currency_get_coin_texture_string(state: &mut LuaState) -> LuaResult<u32> {
    let amount = i64::from_stack(state, 1)?;
    let amount = create_string(state, &format!("{amount}"));
    state.push(amount);
    Ok(1)
}

fn c_currency_get_backpack_currency_info(state: &mut LuaState) -> LuaResult<u32> {
    let _index = i32::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}

fn push_currency_info_table(state: &mut LuaState, info: &CurrencyInfo) -> Val {
    let t = create_table(state);
    write_currency_identity_fields(state, t, info);
    write_currency_quantity_fields(state, t, info);
    write_currency_flag_fields(state, t, info);
    write_currency_weekly_fields(state, t, info);
    write_currency_transfer_fields(state, t, info);
    t
}

fn write_currency_identity_fields(state: &mut LuaState, t: Val, info: &CurrencyInfo) {
    let name = create_string(state, &info.name);
    let description = create_string(state, &info.description);
    table_set(state, t, "currencyID", Val::Num(info.currency_id as f64));
    table_set(state, t, "name", name);
    table_set(state, t, "description", description);
    table_set(state, t, "iconFileID", Val::Num(info.icon_file_id as f64));
    table_set(state, t, "quality", Val::Num(info.quality as f64));
    table_set(
        state,
        t,
        "currencyListDepth",
        Val::Num(info.currency_list_depth as f64),
    );
}

fn write_currency_quantity_fields(state: &mut LuaState, t: Val, info: &CurrencyInfo) {
    table_set(state, t, "quantity", Val::Num(info.quantity as f64));
    table_set(state, t, "maxQuantity", Val::Num(info.max_quantity as f64));
    table_set(state, t, "totalEarned", Val::Num(info.total_earned as f64));
    table_set(
        state,
        t,
        "trackedQuantity",
        Val::Num(info.tracked_quantity as f64),
    );
    table_set(
        state,
        t,
        "useTotalEarnedForMaxQty",
        Val::Bool(info.use_total_earned_for_max_qty),
    );
}

fn write_currency_flag_fields(state: &mut LuaState, t: Val, info: &CurrencyInfo) {
    table_set(state, t, "isHeader", Val::Bool(info.is_header));
    table_set(
        state,
        t,
        "isHeaderExpanded",
        Val::Bool(info.is_header_expanded),
    );
    table_set(
        state,
        t,
        "isShowInBackpack",
        Val::Bool(info.is_show_in_backpack),
    );
    table_set(state, t, "discovered", Val::Bool(info.discovered));
    table_set(state, t, "isTradeable", Val::Bool(info.is_tradeable));
    table_set(state, t, "isTypeUnused", Val::Bool(info.is_type_unused));
}

fn write_currency_weekly_fields(state: &mut LuaState, t: Val, info: &CurrencyInfo) {
    table_set(
        state,
        t,
        "canEarnPerWeek",
        Val::Bool(info.can_earn_per_week),
    );
    table_set(
        state,
        t,
        "maxWeeklyQuantity",
        Val::Num(info.max_weekly_quantity as f64),
    );
    table_set(
        state,
        t,
        "quantityEarnedThisWeek",
        Val::Num(info.quantity_earned_this_week as f64),
    );
    table_set(
        state,
        t,
        "rechargingAmountPerCycle",
        Val::Num(info.recharging_amount_per_cycle as f64),
    );
    table_set(
        state,
        t,
        "rechargingCycleDurationMS",
        Val::Num(info.recharging_cycle_duration_ms as f64),
    );
}

fn write_currency_transfer_fields(state: &mut LuaState, t: Val, info: &CurrencyInfo) {
    table_set(
        state,
        t,
        "isAccountTransferable",
        Val::Bool(info.is_account_transferable),
    );
    table_set(state, t, "isAccountWide", Val::Bool(info.is_account_wide));
    match info.transfer_percentage {
        Some(pct) => table_set(state, t, "transferPercentage", Val::Num(pct)),
        None => table_set(state, t, "transferPercentage", Val::Nil),
    }
}

fn push_currency_info_by_id(state: &mut LuaState, currency_id: i32) -> LuaResult<u32> {
    let info = borrow_state(state)?
        .currency_info
        .get(&currency_id)
        .cloned();
    let Some(info) = info else {
        return Ok(0);
    };
    let table = push_currency_info_table(state, &info);
    state.push(table);
    Ok(1)
}

fn c_currency_get_currency_info(state: &mut LuaState) -> LuaResult<u32> {
    let currency_id = i32::from_stack(state, 1)?;
    push_currency_info_by_id(state, currency_id)
}

fn c_currency_get_currency_info_from_link(state: &mut LuaState) -> LuaResult<u32> {
    let Some(link) = val_to_string(state, stack_val(state, 1)) else {
        return Ok(0);
    };
    let Some(currency_id) = parse_currency_id_from_link(&link) else {
        return Ok(0);
    };
    push_currency_info_by_id(state, currency_id)
}

fn c_currency_get_currency_container_info(state: &mut LuaState) -> LuaResult<u32> {
    let currency_id = i32::from_stack(state, 1)?;
    let quantity = i32::from_stack(state, 2)?;
    let info = borrow_state(state)?
        .currency_info
        .get(&currency_id)
        .cloned();
    let Some(info) = info else {
        return Ok(0);
    };
    let t = create_table(state);
    let name = create_string(state, &info.name);
    let description = create_string(state, &info.description);
    table_set(state, t, "actualAmount", Val::Num(quantity as f64));
    table_set(state, t, "displayAmount", Val::Num(quantity as f64));
    table_set(state, t, "name", name);
    table_set(state, t, "description", description);
    table_set(state, t, "icon", Val::Num(info.icon_file_id as f64));
    table_set(state, t, "quality", Val::Num(info.quality as f64));
    state.push(t);
    Ok(1)
}

/// Parse the currency id from a `|Hcurrency:<id>:...|h` link.
/// Returns `None` for non-currency links or malformed input.
fn parse_currency_id_from_link(link: &str) -> Option<i32> {
    let after_prefix = link.split("|Hcurrency:").nth(1)?;
    let id_str = after_prefix.split(':').next()?;
    id_str.parse::<i32>().ok()
}

pub(super) fn register_c_bank(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Bank")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "FetchDepositedMoney",
        c_bank_fetch_deposited_money,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "FetchNumPurchasedBankTabs",
        c_bank_fetch_num_purchased_bank_tabs,
    )?;
    Ok(())
}

fn c_bank_fetch_deposited_money(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_bank_fetch_num_purchased_bank_tabs(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}
