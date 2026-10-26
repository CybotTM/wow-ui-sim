//! Mists corpse coordinates from explicit observations and existing death state.

use crate::c_api::{ensure_namespace, global_val};
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{borrow_state, call_function_state};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::{LuaResult, Val, vm::state::LuaState};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if ACTIVE != ClientProfile::Mists {
        return Ok(());
    }
    let namespace = ensure_namespace(state, "C_DeathInfo")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetCorpseMapPosition",
        query_corpse_position,
    )
}

fn query_corpse_position(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = i32::from_stack(state, 1)?;
    let position = {
        let sim = borrow_state(state)?;
        let has_corpse = sim.player.health <= 0 && sim.corpse_available;
        has_corpse
            .then(|| sim.corpse_map_positions.get(&map_id).copied())
            .flatten()
    };
    let Some((x, y)) = position else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let factory = global_val(state, "CreateVector2D");
    let vector = call_function_state(state, factory, &[Val::Num(x), Val::Num(y)])?;
    state.push(vector);
    Ok(1)
}
