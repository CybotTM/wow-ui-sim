//! C_StringUtil: string escaping helpers used by Blizzard diagnostics.

use crate::lua_api::methods::{create_string, create_table, val_to_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::helpers::set_global_val;

pub fn register_c_string_util(state: &mut LuaState) -> LuaResult<()> {
    let c_string_util = create_table(state);
    let Val::Table(c_string_util_ref) = c_string_util else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn_static(
        state,
        c_string_util_ref,
        "EscapeQuotedCodes",
        c_string_util_escape_quoted_codes,
    )?;
    set_global_val(state, "C_StringUtil", c_string_util);
    Ok(())
}

pub fn c_string_util_escape_quoted_codes(state: &mut LuaState) -> LuaResult<u32> {
    let Some(input) = val_to_string(state, stack_val(state, 1)) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let escaped = input.replace('|', "||");
    let escaped_value = create_string(state, &escaped);
    state.push(escaped_value);
    Ok(1)
}
