//! Wrath local minimap ping publication, independent of group membership.

use super::stack_num;
use crate::lua_api::globals::group_queries::unit_exists_in_state;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state_mut, create_string};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn publish_local_ping(state: &mut LuaState, id: u64) -> LuaResult<u32> {
    let position = read_ping_position(state);
    {
        let mut sim = borrow_state_mut(state)?;
        if !unit_exists_in_state(&sim, "player") {
            return Ok(0);
        }
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.minimap_ping_position = Some(position);
        }
    }
    let player = create_string(state, "player");
    dispatch_event_now(
        state,
        "MINIMAP_PING",
        &[
            player,
            Val::Num(position.0 as f64),
            Val::Num(position.1 as f64),
        ],
    )?;
    Ok(0)
}

fn read_ping_position(state: &LuaState) -> (f32, f32) {
    match (stack_num(state, 2), stack_num(state, 3)) {
        (Some(x), Some(y)) if x != 0.0 || y != 0.0 => {
            crate::lua_api::workarounds::temporary::wrath_minimap_ping_coordinates::retain_raw_view_coordinates(x as f32, y as f32)
        }
        _ => (0.0, 0.0),
    }
}
