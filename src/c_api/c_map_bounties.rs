//! Bounty set maps projected only from explicitly supplied simulator state.
use super::helpers::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_table};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    if !cfg!(any(feature = "client-retail", feature = "client-ptr")) {
        return Ok(());
    }
    let namespace = ensure_namespace(state, "C_Map")?;
    table_set_rust_fn_static(state, namespace, "GetBountySetMaps", query_bounty_set_maps)
}

fn validate_bounty_set_id(value: Val) -> LuaResult<Option<i32>> {
    let Val::Num(number) = value else {
        return Err(rilua::runtime_error(
            "C_Map.GetBountySetMaps: bountySetID must be a number",
        ));
    };
    let in_range = (f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&number);
    let represented = number.is_finite() && number.fract() == 0.0 && in_range;
    Ok(represented.then(|| number as i32))
}

fn query_bounty_set_maps(state: &mut LuaState) -> LuaResult<u32> {
    let value = stack_val(state, 1);
    // Authenticate before validation or lookup, preserving wrappers and taint.
    #[cfg(feature = "retail-12-1-0")]
    let value = rilua::table_security::unwrap_secret(state, value)?;
    let bounty_set_id = validate_bounty_set_id(value)?;
    let maps = {
        let sim = borrow_state(state)?;
        bounty_set_id
            .and_then(|id| sim.bounty_set_maps.get(&id))
            .cloned()
            .unwrap_or_default()
    };
    let result = create_table(state);
    state.push(result);
    for (index, id) in maps.into_iter().enumerate() {
        set_table_array(state, result, index as i64 + 1, Val::Num(f64::from(id)));
    }
    Ok(1)
}
