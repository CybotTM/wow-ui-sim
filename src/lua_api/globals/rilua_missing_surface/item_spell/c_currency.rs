use crate::lua_api::globals::{currency_data, rilua_missing_surface::ensure_namespace};
use crate::lua_api::rilua_methods::{create_string, create_table, table_set};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_c_currency_info(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_CurrencyInfo")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetCurrencyListSize",
        c_currency_get_list_size,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetCurrencyListInfo",
        c_currency_get_list_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetCoinTextureString",
        c_currency_get_coin_texture_string,
    )?;
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

pub(super) fn register_c_equipment_set(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_EquipmentSet")?;
    table_set_rust_fn(state, table_ref, "GetEquipmentSetIDs", c_equipment_set_ids)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetEquipmentSetInfo",
        c_equipment_set_info,
    )?;
    Ok(())
}

fn c_equipment_set_ids(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn c_equipment_set_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

pub(super) fn register_c_bank(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Bank")?;
    table_set_rust_fn(
        state,
        table_ref,
        "FetchDepositedMoney",
        c_bank_fetch_deposited_money,
    )?;
    Ok(())
}

fn c_bank_fetch_deposited_money(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}
