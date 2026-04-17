//! Frame buffer and texture rotation methods.

use crate::lua_api::rilua_methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn register(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, mt, "IsFrameBuffer", is_frame_buffer)?;
    table_set_rust_fn(state, mt, "RotateTextures", rotate_textures)?;
    table_set_rust_fn(state, mt, "SetIsFrameBuffer", set_is_frame_buffer)?;
    Ok(())
}

pub fn is_frame_buffer(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = borrow_state(state)?
        .widgets
        .get(id)
        .map(|f| f.is_frame_buffer)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn rotate_textures(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let radians = match stack_val(state, 2) {
        Val::Num(n) => n as f32,
        _ => 0.0,
    };
    let mut sim = borrow_state_mut(state)?;
    rotate_descendant_textures(&mut sim, id, radians);
    Ok(0)
}

fn rotate_descendant_textures(sim: &mut crate::lua_api::SimState, frame_id: u64, radians: f32) {
    let mut pending = vec![frame_id];
    while let Some(current_id) = pending.pop() {
        let Some(frame) = sim.widgets.get(current_id) else {
            continue;
        };
        let child_ids = frame.children.clone();
        pending.extend(child_ids.iter().copied());
        for child_id in child_ids {
            if let Some(child) = sim.widgets.get_mut_visual(child_id) {
                if child.widget_type == crate::widget::WidgetType::Texture {
                    child.rotation = radians;
                }
            }
        }
    }
}

pub fn set_is_frame_buffer(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.is_frame_buffer = enabled;
    }
    Ok(0)
}
