//! C_PerksProgram temporary shim — Trader's Tender vendor catalog is not modeled.
//!
//! The Trading Post UI loads against an empty vendor: `GetAvailableVendorItemIDs`
//! and `GetAvailableCategoryIDs` return empty arrays so the OnLoad ipairs loop
//! stays well-typed, and `GetCurrencyAmount` returns 0 so the currency-color
//! comparison in `PerksProgramCurrencyFrameMixin:UpdateCurrencyAmount` does not
//! compare nil against a number. Real catalog state would replace this surface.

use crate::c_api::ensure_global_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn register_c_perks_program(state: &mut LuaState) -> LuaResult<()> {
    let t = ensure_global_table(state, "C_PerksProgram");
    let Val::Table(t_ref) = t else {
        unreachable!("C_PerksProgram must be a table");
    };
    table_set_rust_fn_static(state, t_ref, "GetAvailableVendorItemIDs", empty_table)?;
    table_set_rust_fn_static(state, t_ref, "GetAvailableCategoryIDs", empty_table)?;
    table_set_rust_fn_static(state, t_ref, "GetCategoryInfo", return_nil)?;
    table_set_rust_fn_static(state, t_ref, "GetCurrencyAmount", return_zero)?;
    Ok(())
}

fn empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = crate::lua_api::methods::create_table(state);
    state.push(table);
    Ok(1)
}

fn return_nil(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}
