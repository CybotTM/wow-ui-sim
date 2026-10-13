//! Exact host-supplied ping kits; no native catalog or inferred substitute.

use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};
use std::collections::HashMap;

const INVALID_PING_TYPE: &str =
    "C_Ping.GetTextureKitForType: ping type must be a finite i32 integer";

/// Whole host input; empty by default, with no native mapping catalog.
#[derive(Default)]
pub struct PingTextureKits {
    pub by_type: HashMap<i32, String>,
}

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = super::ensure_namespace(state, "C_Ping")?;
    table_set_rust_fn_static(state, namespace, "GetTextureKitForType", query_texture_kit)
}

fn read_ping_type(value: Val) -> LuaResult<i32> {
    let Val::Num(number) = value else {
        return Err(runtime_error(INVALID_PING_TYPE));
    };
    let in_range = (f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&number);
    let represented = number.is_finite() && number.fract() == 0.0 && in_range;
    if !represented {
        return Err(runtime_error(INVALID_PING_TYPE));
    }
    Ok(number as i32)
}

fn query_texture_kit(state: &mut LuaState) -> LuaResult<u32> {
    // AllowedWhenUntainted: authenticate before validation or any map lookup.
    let value = rilua::table_security::unwrap_secret(state, stack_val(state, 1))?;
    let kind = read_ping_type(value)?;
    let kit = borrow_state(state)?
        .ping_texture_kits
        .by_type
        .get(&kind)
        .cloned()
        .ok_or_else(|| {
            runtime_error(format!(
                "unmapped ping type {kind}: host-supplied mapping required"
            ))
        })?;
    let result = create_string(state, &kit);
    state.push(result);
    Ok(1)
}
