//! Draw layer enable/disable methods.

use crate::lua_api::rilua_methods::{borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::LuaResult;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

pub fn register(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, mt, "DisableDrawLayer", disable_draw_layer)?;
    table_set_rust_fn(state, mt, "EnableDrawLayer", enable_draw_layer)?;
    Ok(())
}

pub fn disable_draw_layer(state: &mut LuaState) -> LuaResult<u32> {
    set_draw_layer_enabled(state, false)
}

pub fn enable_draw_layer(state: &mut LuaState) -> LuaResult<u32> {
    set_draw_layer_enabled(state, true)
}

fn set_draw_layer_enabled(state: &mut LuaState, enabled: bool) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let layer_name = String::from_stack(state, 2)?;
    let Some(layer) = crate::widget::DrawLayer::from_str(&layer_name) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.set_draw_layer_enabled(layer, enabled);
    }
    Ok(0)
}
